use super::*;

// Callback typedef chains are transparent aliases; newtyping function pointers adds no safety.
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
    // Shared bare-alias policy for typedef emission and constant constructors.
    pub fn typedef_emits_bare(&self, def: TypeDef) -> bool {
        let ty = def.underlying_type_ext(self.reader);

        // Function-pointer typedef chains stay aliases to avoid unsafe `PartialEq` on callbacks.
        let aliases_callback = resolves_to_delegate(&ty, self.reader);

        // Handle-to-handle typedefs stay aliases; base and primitive-backed handles keep newtypes.
        let aliases_handle = matches!(&ty, Type::CppStruct(inner) if inner.is_handle(self.reader));

        // Pointers to named types are pointer aliases, not handles; void pointers keep handle newtypes.
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

    pub fn write_cpp_handle(&self, def: TypeDef, cfg: &TokenStream) -> TokenStream {
        let name = to_ident(def.name());
        let ty = def.underlying_type_ext(self.reader);
        let ty_name = ty.write_name(self);

        if self.typedef_emits_bare(def) {
            // Per-arch typedef rows must keep their gates so same-name variants do not collide.
            let arches = write_arches(def);
            quote! {
                #arches
                #cfg
                pub type #name = #ty_name;
            }
        } else {
            // Arch-divergent handles are separate rows; each emitted item needs its own gate.
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

            quote! {
                #arches
                #[repr(transparent)]
                #[derive(#derive)]
                pub struct #name(pub #ty_name);
                #default
            }
        }
    }
}
