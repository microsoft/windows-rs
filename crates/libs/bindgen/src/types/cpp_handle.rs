use super::*;

impl Config<'_> {
    pub fn write_cpp_handle(&self, def: TypeDef) -> TokenStream {
        let name = to_ident(def.name());
        let ty = def.underlying_type_ext(self.reader);
        let ty_name = ty.write_name(self);

        if self.bindgen.style.emit_bare_typedef() {
            // An arch-split enum collapses to a single name-keyed entry whose underlying
            // alias (`-> i32`) is the same on every arch — only its constants, which are
            // separate items, are arch-specific. Emitting that lone entry's cfg would hide
            // the type on the arches where its constants live, so drop it for enums. A plain
            // typedef keeps the def's cfg: an arch-divergent alias (`HALF_PTR = i16` on x86,
            // `= i32` on x64) has a distinct per-arch definition that must stay gated, and a
            // pointer alias (`P* = *mut Struct`) may reference an arch-gated pointee.
            let is_enum = def.category() == windows_metadata::reader::TypeCategory::Enum;
            let arches = if is_enum {
                quote! {}
            } else {
                write_arches(def)
            };
            quote! {
                #arches
                pub type #name = #ty_name;
            }
        } else {
            // Called once per metadata row, so an arch-divergent handle (e.g.
            // `HALF_PTR` or `PCONTEXT`) reaches here twice. Every emitted item must
            // carry the def's arch cfg or the variants collide.
            let arches = write_arches(def);
            let mut derive = quote! { Clone, Copy, Debug, PartialEq, Eq, };

            let default = if ty.is_pointer() {
                quote! {
                    #arches
                    impl Default for #name {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                }
            } else {
                derive.combine(quote! { Default, });
                quote! {}
            };

            let invalid = def.invalid_values();

            let is_invalid = if ty.is_pointer() && (invalid.is_empty() || invalid == [0]) {
                quote! {
                    #arches
                    impl #name {
                        pub fn is_invalid(&self) -> bool {
                            self.0.is_null()
                        }
                    }
                }
            } else if invalid.is_empty() {
                quote! {}
            } else {
                let invalid = invalid.iter().map(|value| {
                    let literal = Literal::i64_unsuffixed(*value);

                    if ty.is_pointer() || (*value < 0 && ty.is_unsigned()) {
                        quote! { self.0 == #literal as _ }
                    } else {
                        quote! { self.0 == #literal }
                    }
                });
                quote! {
                    #arches
                    impl #name {
                        pub fn is_invalid(&self) -> bool {
                            #(#invalid)||*
                        }
                    }
                }
            };

            let free = if let Some(function) = def.free_function(self.reader) {
                if is_invalid.is_empty() {
                    // TODO: https://github.com/microsoft/win32metadata/issues/1891
                    quote! {}
                } else {
                    let link = function.write_link(self, true);
                    let free = to_ident(function.method.name());

                    quote! {
                        #arches
                        impl windows_core::Free for #name {
                            #[inline]
                            unsafe fn free(&mut self) {
                                if !self.is_invalid() {
                                    #link
                                    unsafe { #free(self.0); }
                                }
                            }
                        }
                    }
                }
            } else {
                quote! {}
            };

            let mut result = quote! {
                #arches
                #[repr(transparent)]
                #[derive(#derive)]
                pub struct #name(pub #ty_name);
                #is_invalid
                #free
                #default
            };

            if let Some(attribute) = def.find_attribute("AlsoUsableForAttribute") {
                if let Some((_, Value::Utf8(type_name))) = attribute.value().first() {
                    let ty = self.reader.unwrap_full_name(def.namespace(), type_name);

                    let ty = ty.write_name(self);

                    result.combine(quote! {
                        #arches
                        impl windows_core::imp::CanInto<#ty> for #name {}
                        #arches
                        impl From<#name> for #ty {
                            fn from(value: #name) -> Self {
                                Self(value.0)
                            }
                        }
                    });
                }
            }

            result
        }
    }
}
