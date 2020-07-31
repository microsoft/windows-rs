use crate::case;
use crate::format_ident;
use crate::type_tree::TypeTree;
use crate::types::MethodKind;
use rayon::prelude::*;

use squote::{quote, TokenStream};

use std::collections::*;

#[derive(Default)]
pub struct TypeNamespaces(pub BTreeMap<String, TypeTree>);

impl TypeNamespaces {
    pub fn to_tokens<'a>(&'a self) -> impl ParallelIterator<Item = TokenStream> + 'a {
        self.0.par_iter().map(|(name, tree)| {
            let name = case::to_snake(name, MethodKind::Normal);
            let name = format_ident(&name);
            let tree = tree.to_tokens().collect::<Vec<_>>();

            quote! {
                pub mod #name {
                    #(#tree)*
                }
            }
        })
    }
}
