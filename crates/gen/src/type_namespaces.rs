use crate::case;
use crate::format_ident;
use crate::type_tree::TypeTree;
use crate::types::MethodKind;

use proc_macro2::TokenStream;
use quote::quote;

use std::collections::*;

#[derive(Default)]
pub struct TypeNamespaces(pub BTreeMap<String, TypeTree>);

impl TypeNamespaces {
    pub fn to_tokens<'a>(&'a self) -> impl Iterator<Item = TokenStream> + 'a {
        self.0.iter().map(|(name, tree)| {
            let name = case::to_snake(name, MethodKind::Normal);
            let name = format_ident(&name);
            let tree = tree.to_tokens();

            quote! {
                pub mod #name {
                    #(#tree)*
                }
            }
        })
    }
}
