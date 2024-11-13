use super::*;

impl Writer {
    pub fn write_core(&self) -> TokenStream {
        if self.config.no_deps {
            if self.config.flat {
                quote! {}
            } else {
                let mut tokens = TokenStream::new();

                for _ in 0..self.namespace.split('.').count() {
                    tokens.push_str("super::");
                }

                tokens
            }
        } else if self.config.sys {
            quote! { windows_sys::core:: }
        } else {
            quote! { windows_core:: }
        }
    }

    pub fn write_generic_phantoms(&self, generics: &[Type]) -> TokenStream {
        if generics.is_empty() {
            quote! {}
        } else {
            let generics = generics.iter().map(|ty| ty.write(self));
            quote! { #(core::marker::PhantomData::<#generics>),* }
        }
    }

    pub fn write_generic_named_phantoms(&self, generics: &[Type]) -> TokenStream {
        if generics.is_empty() {
            quote! {}
        } else {
            let generics = generics.iter().map(|ty| ty.write(self));
            quote! { #(#generics: core::marker::PhantomData::<#generics>),* }
        }
    }

    pub fn write_generic_constraints(&self, generics: &[Type]) -> TokenStream {
        if generics.is_empty() {
            quote! {}
        } else {
            let generics = generics.iter().map(|ty| ty.write(self));
            quote! { #(#generics: windows_core::RuntimeType + 'static,)* }
        }
    }

    pub fn write_namespace(&self, type_name: TypeName<'_>) -> TokenStream {
        if self.config.flat || type_name.namespace().is_empty() || type_name.namespace() == self.namespace {
            return quote! {};
        }

        // TODO: here we need to check self.config.references
        if !self.config.tree.includes_namespace(type_name.namespace()) {
             todo!("deal with external references `{}`", type_name.namespace());
        }

        let mut relative = self.namespace.split('.').peekable();
        let mut namespace = type_name.namespace().split('.').peekable();

        while relative.peek() == namespace.peek() {
            if relative.next().is_none() {
                break;
            }

            namespace.next();
        }

        let mut tokens = TokenStream::new();

        for _ in 0..relative.count() {
            tokens.push_str("super::");
        }

        for namespace in namespace {
            tokens.push_str(namespace);
            tokens.push_str("::");
        }

        tokens
    }
}
