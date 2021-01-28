use squote::{quote, TokenStream};
use std::collections::*;

#[derive(Default)]
pub struct TypeNamespaces(pub BTreeMap<&'static str, crate::type_tree::TypeTree>);

impl TypeNamespaces {
    pub fn gen<'a>(&'a self) -> impl Iterator<Item = TokenStream> + 'a {
        self.0.iter().map(|(name, tree)| {
            let name = crate::to_snake(name);
            let name = crate::format_ident(&name);
            let tokens = tree.gen().collect::<Vec<_>>();

            let foundation = if tree.include_foundation {
                quote! { pub use ::windows::*; }
            } else {
                TokenStream::new()
            };

            quote! {
                // TODO: remove this allowance when https://github.com/microsoft/windows-rs/issues/212 is fixed
                #[allow(unused_variables)]
                #[allow(non_upper_case_globals)]
                pub mod #name {
                    #(#tokens)*
                    #foundation
                }
            }
        })
    }
}
