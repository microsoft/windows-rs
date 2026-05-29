use super::*;

impl Config<'_> {
    pub fn write_core(&self) -> TokenStream {
        self.write_specific("windows_core")
    }

    pub fn write_result(&self) -> TokenStream {
        self.write_specific("windows_result")
    }

    pub fn write_strings(&self) -> TokenStream {
        self.write_specific("windows_strings")
    }

    fn write_specific(&self, specific: &str) -> TokenStream {
        if self.bindgen.style.is_sys() {
            if self.bindgen.layout.is_package() || self.bindgen.deps != DepMode::None {
                quote! { windows_sys::core:: }
            } else if self.bindgen.layout.is_flat() {
                quote! {}
            } else {
                let mut path = String::new();

                for _ in 0..self.namespace.split('.').count() {
                    path.push_str("super::");
                }

                path.parse().unwrap()
            }
        } else if self.bindgen.deps != DepMode::Specific {
            quote! { windows_core:: }
        } else {
            format!("{specific}::").parse().unwrap()
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

            if reference.style == ReferenceStyle::Flat {
                return path.parse().unwrap();
            }

            let mut namespace = type_name.namespace().split('.').peekable();

            if reference.style == ReferenceStyle::SkipRoot {
                namespace.next();
            }

            for namespace in namespace {
                path.push_str(namespace);
                path.push_str("::");
            }

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
