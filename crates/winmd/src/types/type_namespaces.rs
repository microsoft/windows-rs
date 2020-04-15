use super::TypeTree;
use crate::case;
use crate::types::MethodKind;
use crate::write_ident;

use proc_macro2::TokenStream;
use quote::quote;

use std::collections::*;
use std::iter::FromIterator;

#[derive(Default)]
pub struct TypeNamespaces(pub BTreeMap<String, TypeTree>);

impl TypeNamespaces {
    pub fn to_stream(&self) -> TokenStream {
        let mut tokens = Vec::new();

        for (name, tree) in self.0.iter() {
            let name = case::to_snake(name, MethodKind::Normal);
            let name = write_ident(&name);
            let (base, abi) = tree.to_stream();

            let merged: TokenStream = quote! {
                pub mod #name {
                    #base
                    pub mod abi {
                        #abi
                    }
                }
            };

            tokens.push(merged);
        }

        TokenStream::from_iter(tokens)
    }
}
