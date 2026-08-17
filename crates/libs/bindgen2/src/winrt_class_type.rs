use super::*;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone)]
pub(super) struct ClassInterface {
    pub(super) entity: Entity<TypeDef>,
    pub(super) namespace: String,
    pub(super) name: String,
    pub(super) arguments: Vec<ty::Type>,
    pub(super) methods: Vec<winrt_interface::NamedMethod>,
    pub(super) default: bool,
    pub(super) exclusive: bool,
    pub(super) factory: bool,
    pub(super) composable: bool,
}

impl ClassInterface {
    pub(super) fn is_async(&self) -> bool {
        self.namespace == "Windows.Foundation"
            && matches!(
                self.name.as_str(),
                "IAsyncAction"
                    | "IAsyncActionWithProgress"
                    | "IAsyncOperation"
                    | "IAsyncOperationWithProgress"
            )
    }

    pub(super) fn write_async_name(
        &self,
        namespace: &str,
        layout: Layout,
    ) -> Result<TokenStream, Error> {
        let name = tokens::ident(&self.name);
        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.write_name(namespace, layout, &[]))
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(if arguments.is_empty() {
            quote! { windows_future::#name }
        } else {
            quote! { windows_future::#name<#(#arguments),*> }
        })
    }

    pub(super) fn clone_model(&self) -> Self {
        Self {
            namespace: self.namespace.clone(),
            entity: self.entity,
            name: self.name.clone(),
            arguments: self.arguments.clone(),
            methods: Vec::new(),
            default: self.default,
            exclusive: self.exclusive,
            factory: self.factory,
            composable: self.composable,
        }
    }

    pub(super) fn write_name(&self, namespace: &str, layout: Layout) -> Result<TokenStream, Error> {
        let crate_name =
            if layout.is_package() && self.namespace == "Windows.Foundation.Collections" {
                external::package_crate_name(&self.namespace, &self.name)
            } else if namespace != self.namespace {
                external::winrt_crate(&self.namespace, &self.name)
            } else {
                None
            };
        if let Some(crate_name) = crate_name {
            let crate_name = tokens::ident(crate_name);
            let name = tokens::ident(&self.name);
            let arguments = self
                .arguments
                .iter()
                .map(|argument| argument.write_name(namespace, layout, &[]))
                .collect::<Result<Vec<_>, Error>>()?;
            return Ok(if arguments.is_empty() {
                quote! { #crate_name::#name }
            } else {
                quote! { #crate_name::#name<#(#arguments),*> }
            });
        }
        let path = tokens::namespace(namespace, &self.namespace, layout);
        let name = tokens::ident(&self.name);
        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.write_name(namespace, layout, &[]))
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(if arguments.is_empty() {
            quote! { #path #name }
        } else {
            quote! { #path #name<#(#arguments),*> }
        })
    }
}

#[derive(Clone)]
pub(super) struct ClassName {
    pub(super) namespace: String,
    pub(super) name: String,
}

impl ClassName {
    pub(super) fn write_name(&self, namespace: &str, layout: Layout) -> TokenStream {
        let path = tokens::namespace(namespace, &self.namespace, layout);
        let name = tokens::ident(&self.name);
        quote! { #path #name }
    }
}
