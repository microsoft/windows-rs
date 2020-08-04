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
            let tokens = tree.to_tokens().collect::<Vec<_>>();

            let foundation = if tree.include_foundation {
                quote! { pub use ::winrt::foundation; }
            } else {
                TokenStream::new()
            };

            quote! {
                pub mod #name {
                    #(#tokens)*
                    #foundation
                }
            }
        })
    }
}
