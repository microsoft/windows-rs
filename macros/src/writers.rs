use crate::*;
use quote::quote;
use quote::format_ident;

pub(crate) fn write_namespace(namespace: &winmd::Namespace, scope: &std::collections::BTreeSet::<String>) -> proc_macro2::TokenStream {
    let mut tokens = write_types(namespace, scope);

    for name in namespace.name().rsplit('.') {
        let name = format_ident!("{}", name.to_lowercase());
        tokens = quote! {
            pub mod #name { #tokens }
        };
    }

    tokens.into()
}

fn write_types(namespace: &winmd::Namespace, scope: &std::collections::BTreeSet::<String>) -> proc_macro2::TokenStream {
    quote! {

    }
}
