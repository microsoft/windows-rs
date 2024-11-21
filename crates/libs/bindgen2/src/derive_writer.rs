use super::*;

pub struct DeriveWriter(BTreeSet<String>);

// impl std::ops::Deref for DeriveWriter {
//     type Target = BTreeSet<String>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl core::ops::DerefMut for DeriveWriter {
//     fn deref_mut(&mut self) -> &mut BTreeSet<String> {
//         &mut self.0
//     }
// }

impl DeriveWriter {
    pub fn new(writer: &Writer, type_name: TypeName) -> Self {
        let mut derive = BTreeSet::new();
        derive.extend(writer.config.derive.get(type_name));
        Self(derive)
    }

    pub fn extend<I, S>(&mut self, iter: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + ToString,
    {
        self.0.extend(iter.into_iter().map(|s| s.to_string()));
    }
}

impl ToTokens for DeriveWriter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if !self.0.is_empty() {
            let derive = self.0.iter().map(|derive| to_ident(derive));
            tokens.combine(quote! {
                #[derive(#(#derive),*)]
            })
        }
    }
}
