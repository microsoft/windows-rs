use super::*;

impl Config<'_> {
    pub fn write_core(&self) -> TokenStream {
        if self.bindgen.style.is_sys() {
            if self.bindgen.layout.is_package() {
                // Package mode generates the `windows-sys` crate itself.
                quote! { windows_sys::core:: }
            } else if self.bindgen.layout.is_flat() {
                // Flat sys bindings define core types inline (no prefix needed).
                quote! {}
            } else {
                // Module-based sys bindings reference root-level inline defs.
                let mut path = String::new();

                for _ in 0..self.namespace.split('.').count() {
                    path.push_str("super::");
                }

                path.parse().unwrap()
            }
        } else {
            quote! { windows_core:: }
        }
    }

    /// Returns `#[doc(hidden)]` for `--package` layout, otherwise nothing. The
    /// published `windows`/`windows-sys` crates hide their raw vtbl structs from
    /// the docs while keeping them `pub` for the macro/ABI surface.
    pub fn doc_hidden_in_package(&self) -> TokenStream {
        if self.bindgen.layout.is_package() {
            quote! { #[doc(hidden)] }
        } else {
            quote! {}
        }
    }

    pub fn write_generic_phantoms(&self, generics: &[Type]) -> TokenStream {
        if generics.is_empty() {
            quote! {}
        } else {
            let generics = generics.iter().map(|ty| ty.write_name(self));
            quote! { #(core::marker::PhantomData::<#generics>),* }
        }
    }

    pub fn write_generic_named_phantoms(&self, generics: &[Type]) -> TokenStream {
        if generics.is_empty() {
            quote! {}
        } else {
            let generics = generics.iter().map(|ty| ty.write_name(self));
            quote! { #(#generics: core::marker::PhantomData::<#generics>),* }
        }
    }

    pub fn write_generic_constraints(&self, generics: &[Type]) -> TokenStream {
        if generics.is_empty() {
            quote! {}
        } else {
            let generics = generics.iter().map(|ty| ty.write_name(self));
            quote! { #(#generics: windows_core::RuntimeType + 'static,)* }
        }
    }

    pub fn write_namespace(&self, type_name: TypeName) -> TokenStream {
        let mut path = String::new();

        if type_name.namespace().is_empty() {
            return path.parse().unwrap();
        }

        if let Some(reference) = {
            if self.types.contains_key(&type_name) {
                None
            } else {
                self.references.contains(type_name)
            }
        } {
            path.push_str(&reference.name);
            path.push_str("::");
            path.parse().unwrap()
        } else {
            if self.bindgen.layout.is_flat() || type_name.namespace() == self.namespace {
                return path.parse().unwrap();
            }

            let mut relative = self.namespace.split('.').peekable();
            let mut namespace = type_name.namespace().split('.').peekable();

            while relative.peek() == namespace.peek() {
                if relative.next().is_none() {
                    break;
                }

                namespace.next();
            }

            for _ in 0..relative.count() {
                path.push_str("super::");
            }

            for namespace in namespace {
                path.push_str(namespace);
                path.push_str("::");
            }

            path.parse().unwrap()
        }
    }
}
