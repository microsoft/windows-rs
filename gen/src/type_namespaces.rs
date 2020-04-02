use crate::*;

#[derive(Default)]
pub struct TypeNamespaces(pub BTreeMap<String, TypeTree>);

impl TypeNamespaces {
    pub fn to_stream(&self) -> TokenStream {
        let mut tokens = Vec::new();

        for (name, tree) in self.0.iter() {
            let name = write_ident(name);
            let tree = tree.to_stream();

            tokens.push(quote! {
                pub mod #name {
                    #tree
                }
            });
        }

        TokenStream::from_iter(tokens)
    }
}
