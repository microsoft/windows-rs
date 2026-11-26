use super::*;

// Callback typedef chains are transparent aliases; newtyping function pointers adds no safety.
fn resolves_to_delegate(ty: &Type, reader: &Reader) -> bool {
    match ty {
        Type::CppDelegate(_) | Type::FnPtr(_) => true,
        Type::CppStruct(inner) if inner.is_handle(reader) => {
            resolves_to_delegate(&inner.def.underlying_type_ext(reader), reader)
        }
        _ => false,
    }
}

impl Config<'_> {
    // Shared bare-alias policy for typedef emission and constant constructors.
    pub fn typedef_emits_bare(&self, def: TypeDef) -> bool {
        // Legacy handles and enums are newtypes whenever the style allows them.
        if self.reader.legacy() {
            return self.bindgen.style.emit_bare_typedef();
        }

        if def.has_attribute("NativeTypedefAttribute") {
            return true;
        }

        let ty = def.underlying_type_ext(self.reader);

        // Function-pointer typedef chains stay aliases to avoid unsafe `PartialEq` on callbacks.
        let aliases_callback = resolves_to_delegate(&ty, self.reader);

        // Handle-to-handle typedefs stay aliases; base and primitive-backed handles keep newtypes.
        let aliases_handle = matches!(&ty, Type::CppStruct(inner) if inner.is_handle(self.reader));

        // Pointers to named types are pointer aliases, not handles; void pointers keep handle
        // newtypes.
        let aliases_pointer = matches!(
            &ty,
            Type::PtrMut(inner, _) | Type::PtrConst(inner, _) if !matches!(inner.as_ref(), Type::Void)
        );

        // Unscoped C enums are bare aliases in every style.
        let is_enum = def.category() == windows_metadata::reader::TypeCategory::Enum;

        self.bindgen.style.emit_bare_typedef()
            || aliases_callback
            || aliases_handle
            || aliases_pointer
            || is_enum
    }

    /// Whether a C enum projects as a newtype rather than a bare integer alias.
    pub fn enum_is_newtype(&self, def: TypeDef) -> bool {
        def.has_attribute("ScopedEnumAttribute") || !self.typedef_emits_bare(def)
    }

    /// Re-exports the core type that replaces a legacy status definition at its metadata path.
    pub fn write_legacy_core_type(&self, ty: &Type) -> TokenStream {
        let name = ty.write_name(self);
        quote! { pub use #name; }
    }

    pub fn write_cpp_handle(&self, owner: &Type, def: TypeDef, cfg: &TokenStream) -> TokenStream {
        let name = to_ident(def.name());
        let ty = def.underlying_type_ext(self.reader);
        let ty_name = ty.write_name(self);

        if self.typedef_emits_bare(def) {
            // Per-arch typedef rows must keep their gates so same-name variants do not collide.
            let arches = self.write_reached_arches(owner, def);
            quote! {
                #arches
                #cfg
                pub type #name = #ty_name;
            }
        } else {
            // Arch-divergent handles are separate rows; each emitted item needs its own gate.
            let arches = self.write_reached_arches(owner, def);
            let arches = quote! { #arches #cfg };
            let mut derives = vec!["Clone", "Copy", "Debug", "PartialEq"];
            if ty.is_eq(self.reader) {
                derives.push("Eq");
            }
            let default = if ty.is_pointer() {
                let null = if matches!(ty, Type::PtrConst(..)) {
                    quote! { core::ptr::null() }
                } else {
                    quote! { core::ptr::null_mut() }
                };
                quote! {
                    #arches
                    impl Default for #name {
                        fn default() -> Self {
                            Self(#null)
                        }
                    }
                }
            } else {
                derives.push("Default");
                quote! {}
            };
            let derives = derives.into_iter().map(to_ident);
            let legacy = if self.reader.legacy() {
                self.write_legacy_handle(def, &ty, &arches)
            } else {
                quote! {}
            };
            quote! {
                #arches
                #[repr(transparent)]
                #[derive(#(#derives),*)]
                pub struct #name(pub #ty_name);
                #default
                #legacy
            }
        }
    }

    // Win32metadata handle attributes: `InvalidHandleValue`, `RAIIFree`, and `AlsoUsableFor`.
    fn write_legacy_handle(&self, def: TypeDef, ty: &Type, arches: &TokenStream) -> TokenStream {
        let name = to_ident(def.name());
        let invalid: Vec<i64> = def
            .attributes()
            .filter(|attribute| attribute.name() == "InvalidHandleValueAttribute")
            .filter_map(|attribute| match attribute.value().first() {
                Some((_, Value::I64(value))) => Some(*value),
                _ => None,
            })
            .collect();

        let is_invalid = if ty.is_pointer() && (invalid.is_empty() || invalid == [0]) {
            quote! { self.0.is_null() }
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
            quote! { #(#invalid)||* }
        };

        let mut tokens = quote! {};

        if !is_invalid.is_empty() {
            tokens.combine(quote! {
                #arches
                impl #name {
                    pub fn is_invalid(&self) -> bool {
                        #is_invalid
                    }
                }
            });

            if let Some(function) = self.legacy_free_function(def) {
                let link = function.write_link(self, true);
                let free = to_ident(function.method.name());
                // BCryptCloseAlgorithmProvider has an unused trailing parameter.
                let tail = if function
                    .method
                    .method_signature(&[], self.reader)
                    .params
                    .len()
                    > 1
                {
                    quote! { , 0 }
                } else {
                    quote! {}
                };
                tokens.combine(quote! {
                    #arches
                    impl windows_core::Free for #name {
                        #[inline]
                        unsafe fn free(&mut self) {
                            if !self.is_invalid() {
                                #link
                                unsafe { #free(self.0 #tail); }
                            }
                        }
                    }
                });
            }
        }

        if let Some(target) = legacy_also_usable_for(def, self.reader) {
            let target = target.write_name(self);
            tokens.combine(quote! {
                #arches
                impl windows_core::imp::CanInto<#target> for #name {}
                #arches
                impl From<#name> for #target {
                    fn from(value: #name) -> Self {
                        Self(value.0)
                    }
                }
            });
        }

        tokens
    }

    fn legacy_free_function(&self, def: TypeDef) -> Option<CppFn> {
        let attribute = def.find_attribute("RAIIFreeAttribute")?;
        let Some((_, Value::Utf8(name))) = attribute.value().into_iter().next() else {
            return None;
        };
        match self.reader.with_full_name(def.namespace(), &name).next() {
            Some(Type::CppFn(function)) => Some(function),
            _ => None,
        }
    }
}

/// The `AlsoUsableFor` conversion target of a legacy Win32metadata handle.
pub fn legacy_also_usable_for(def: TypeDef, reader: &Reader) -> Option<Type> {
    if !reader.legacy() {
        return None;
    }
    let attribute = def.find_attribute("AlsoUsableForAttribute")?;
    let Some((_, Value::Utf8(name))) = attribute.value().into_iter().next() else {
        return None;
    };
    Some(reader.unwrap_type_name(def.namespace(), &name))
}
