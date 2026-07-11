use super::*;

// Does `ty` resolve — directly or through a chain of native-typedef handles — to a callback
// (delegate)? A typedef whose Value field is a delegate satisfies `is_handle` (delegates are
// treated as primitive), so callback aliases like `FONTENUMPROC = FONTENUMPROCA = ... = Option<fn>`
// would otherwise be newtype-wrapped. Handle chains bottoming out in a scalar/pointer (`HWND`,
// `GLOBALHANDLE`) return false and keep their newtype.
fn resolves_to_delegate(ty: &Type, reader: &Reader) -> bool {
    match ty {
        Type::CppDelegate(_) => true,
        Type::CppStruct(inner) if inner.is_handle(reader) => {
            resolves_to_delegate(&inner.def.underlying_type_ext(reader), reader)
        }
        _ => false,
    }
}

impl Config<'_> {
    pub fn write_cpp_handle(&self, def: TypeDef, cfg: &TokenStream) -> TokenStream {
        let name = to_ident(def.name());
        let ty = def.underlying_type_ext(self.reader);
        let ty_name = ty.write_name(self);

        // A typedef that transitively aliases a callback (delegate) — directly
        // (`AMGETERRORTEXTPROC = AMGETERRORTEXTPROCA`) or through a chain of further typedefs
        // (`FONTENUMPROC = FONTENUMPROCA = OLDFONTENUMPROCA = Option<fn...>`) — is emitted as a
        // transparent alias rather than a wrapper newtype, matching --sys/--minimal. Newtyping a
        // callback adds no type safety and deriving `PartialEq` over the wrapped function pointer
        // triggers `unpredictable_function_pointer_comparisons`. (Handle-of-handle typedefs like
        // `GLOBALHANDLE = HGLOBAL` stay newtypes here: their constants are constructed as tuple
        // structs elsewhere in the generated code.)
        let aliases_callback = resolves_to_delegate(&ty, self.reader);

        // Unscoped (C-style) enums are always projected as bare aliases — see `cpp_enum`, which is
        // the only caller that passes an enum `def` here (scoped enums keep their newtype and never
        // reach this function). So enums take the bare branch in every style, not just sys/minimal.
        let is_enum = def.category() == windows_metadata::reader::TypeCategory::Enum;

        if self.bindgen.style.emit_bare_typedef() || aliases_callback || is_enum {
            // Emit the def's arch cfg. An arch-divergent alias (`HALF_PTR = i16` on x86, `= i32`
            // on x64) or an unscoped enum that exists only on some arches (`KSPIN_LOCK_QUEUE_NUMBER`
            // is an enum on x86/arm64 but a pointer typedef on x64) has a distinct per-arch
            // definition that must stay gated so the arch-specific copies don't collide. Same-name
            // enum copies are kept distinct in the codegen set by their arch bits (see `sort_key`),
            // so each surviving copy carries its own gate here.
            let arches = write_arches(def);
            quote! {
                #arches
                #cfg
                pub type #name = #ty_name;
            }
        } else {
            // Called once per metadata row, so an arch-divergent handle (e.g.
            // `HALF_PTR` or `PCONTEXT`) reaches here twice. Every emitted item must
            // carry the def's arch cfg or the variants collide.
            let arches = write_arches(def);
            let arches = quote! { #arches #cfg };
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

            let is_invalid = if ty.is_pointer() {
                quote! {
                    #arches
                    impl #name {
                        pub fn is_invalid(&self) -> bool {
                            self.0.is_null()
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
