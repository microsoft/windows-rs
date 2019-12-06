use crate::*;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

pub(crate) fn write_namespace(namespace: &winmd::Namespace, scope: &std::collections::BTreeSet<String>) -> TokenStream {
    let mut tokens = write_types(namespace, scope);

    for name in namespace.name().rsplit('.') {
        let name = format_ident!("{}", name.to_lowercase());
        tokens = quote! {
            pub mod #name { #tokens }
        };
    }

    tokens
}

fn write_types(namespace: &winmd::Namespace, scope: &std::collections::BTreeSet<String>) -> TokenStream {
    let enums = write_enums(namespace);
    let structs = write_structs(namespace);

    quote! {
        #enums
        #structs
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

fn write_enum_fields(t: &winmd::TypeDef) -> TokenStream {
    let mut tokens = quote! {};

    for f in t.fields().unwrap() {
        for c in f.constants().unwrap() {
            let name = format_ident!("{}", f.name().unwrap());
            let value = c.value().unwrap();

            tokens = quote! {
                #tokens
                #name,
                // TODO: write out the enum value
            };
        }
    }

    tokens
}

fn write_structs(namespace: &winmd::Namespace) -> proc_macro2::TokenStream {
    let mut tokens = quote! {};

    for t in namespace.structs() {
        let name = format_ident!("{}", t.name().unwrap());
        let fields = write_struct_fields(&t);
        tokens = quote! {
            #tokens
            #[repr(C)]
            #[derive(Default, Debug)]
            pub struct #name { #fields }
        };
    }

    tokens
}

fn write_struct_fields(t: &winmd::TypeDef) -> TokenStream {
    let mut tokens = quote! {};

    for f in t.fields().unwrap() {
        let name = format_ident!("{}", f.name().unwrap().to_lowercase());

        tokens = quote! {
            #tokens
            #name: u32,
            // TODO: write out field type
        };
    }

    tokens
}