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

    tokens
}

fn write_types(namespace: &winmd::Namespace, scope: &std::collections::BTreeSet::<String>) -> proc_macro2::TokenStream {
    let mut enums = write_enums(namespace);

    quote! {
        #enums
    }
}

fn write_enums(namespace: &winmd::Namespace) -> proc_macro2::TokenStream {
    let mut tokens = quote! {};

    for t in namespace.enums() {
        let name = format_ident!("{}", t.name().unwrap());
        let fields = write_enum_fields(&t);
        tokens = quote! {
            #tokens
            pub enum #name { #fields }
        };
    }

    tokens
}

fn write_enum_fields(t : &winmd::TypeDef) -> proc_macro2::TokenStream {
    let mut tokens = quote! {};

    for f in t.fields().unwrap() {
        for c in f.constants().unwrap() {
            let name = format_ident!("{}", f.name().unwrap());
            let value = c.value().unwrap();

            tokens = quote! {
                #tokens
                #name,
            };
        }
    }

    tokens
}
