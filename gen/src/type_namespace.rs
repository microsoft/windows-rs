use crate::*;

#[derive(Default)]
pub struct TypeNamespace(pub BTreeMap<String, TypeTree>);

impl TypeNamespace {
    pub fn into_stream(&self) -> TokenStream {
        let mut tokens = Vec::new();

        for (name, tree) in self.0.iter() {
            let name = write_ident(name);
            let tree = tree.into_stream();

            tokens.push(quote! {
                pub mod #name {
                    #tree
                }
            });
        }

        TokenStream::from_iter(tokens)
    }
}