use super::*;

// Does `ty` resolve — directly or through a chain of native-typedef handles — to a callback
// (delegate)? A typedef whose Value field is a delegate satisfies `is_handle` (delegates are
// treated as primitive), so callback aliases like `FONTENUMPROC = FONTENUMPROCA = ... = Option<fn>`
// would otherwise be newtype-wrapped. Handle chains bottoming out in a raw pointer/scalar (`HWND`)
// return false; the `aliases_handle` check in `write_cpp_handle` handles handle-of-handle aliases.
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
    // Whether the native typedef `def` is projected as a transparent `pub type` alias rather than
    // a wrapper newtype in the current style. This is the single source of truth for the bare-alias
    // decision, shared by `write_cpp_handle` (which emits the typedef) and the constant emitter
    // (which must drop the newtype constructor for a bare-alias-typed constant).
    pub fn typedef_emits_bare(&self, def: TypeDef) -> bool {
        let ty = def.underlying_type_ext(self.reader);

        // A typedef that transitively aliases a callback (delegate) — directly
        // (`AMGETERRORTEXTPROC = AMGETERRORTEXTPROCA`) or through a chain of further typedefs
        // (`FONTENUMPROC = FONTENUMPROCA = OLDFONTENUMPROCA = Option<fn...>`) — is emitted as a
        // transparent alias rather than a wrapper newtype, matching --sys/--minimal. Newtyping a
        // callback adds no type safety and deriving `PartialEq` over the wrapped function pointer
        // triggers `unpredictable_function_pointer_comparisons`.
        let aliases_callback = resolves_to_delegate(&ty, self.reader);

        // A typedef whose underlying type is *itself* a named handle (`GLOBALHANDLE = HANDLE`,
        // `HCURSOR = HICON`, `SQLHWND = HWND`, `HOLEMENU = HGLOBAL`) is emitted as a transparent
        // alias rather than nesting one handle newtype inside another — matching --sys/--minimal.
        // Base handles, whose underlying is a raw pointer/scalar (`HANDLE = *mut void`, `HWND`),
        // keep the newtype (with its `Default` impl). Primitive-backed handles like
        // `JET_GRBIT` (underlying `u32`) are unaffected: their underlying is not a handle struct, so
        // they keep the newtype their tuple-constructed constants (`JET_GRBIT(…)`) require.
        let aliases_handle = matches!(&ty, Type::CppStruct(inner) if inner.is_handle(self.reader));

        // A typedef whose underlying type is a pointer to a *named* type (`PCOMPRESSOR_HANDLE =
        // *mut COMPRESSOR_HANDLE`, `PTRUSTEE_A = *mut TRUSTEE_A`, `PACCESS_RIGHTS = *mut u32`) is a
        // pointer alias, not a handle — emit it as a transparent alias rather than wrapping the
        // pointer in a newtype, matching --sys/--minimal. Genuine base handles point to void
        // (`HANDLE = *mut void`, `HWND = *mut void`) and keep their newtype (with its `Default`
        // impl), so pointers to void are excluded here.
        let aliases_pointer = matches!(
            &ty,
            Type::PtrMut(inner, _) | Type::PtrConst(inner, _) if !matches!(inner.as_ref(), Type::Void)
        );

        // Unscoped (C-style) enums are always projected as bare aliases — see `cpp_enum`, which is
        // the only caller that passes an enum `def` here (scoped enums keep their newtype and never
        // reach this function). So enums take the bare branch in every style, not just sys/minimal.
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

            let mut result = quote! {
                #arches
                #[repr(transparent)]
                #[derive(#derive)]
                pub struct #name(pub #ty_name);
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
