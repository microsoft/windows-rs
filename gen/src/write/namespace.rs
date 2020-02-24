use std::collections::BTreeMap;
use std::iter::FromIterator;

use proc_macro2::TokenStream;
use quote::quote;

use crate::helpers::to_snake;

#[derive(Default)]
pub struct Namespace {
    types: TokenStream,
    namespaces: Namespaces,
}

impl Namespace {
    pub fn write(&self) -> TokenStream {
        let types = &self.types;
        let namespaces = self.namespaces.write_namespaces();

        quote! {
            #types
            #namespaces
        }
    }
}

#[derive(Default)]
pub struct Namespaces(BTreeMap<String, Namespace>);

impl Namespaces {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert_namespace(&mut self, namespace: &str, types: TokenStream) {
        if let Some(pos) = namespace.find('.') {
            self.0
                .entry(namespace[..pos].to_string())
                .or_default()
                .namespaces
                .insert_namespace(&namespace[pos + 1..], types);
        } else {
            self.0.entry(namespace.to_string()).or_default().types = types;
        }
    }

    pub fn write_namespaces(&self) -> TokenStream {
        let mut tokens = Vec::new();

        for (name, namespace) in self.0.iter() {
            let name = super::write_ident(&to_snake(name));
            let namespace = namespace.write();

            tokens.push(quote! {
                pub mod #name {
                    #namespace
                }
            });
        }

        TokenStream::from_iter(tokens)
    }
}
