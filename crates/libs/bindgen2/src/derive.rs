use super::*;

pub struct Derive(BTreeSet<String>);

impl Derive {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn from<I, S>(names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut derive = Self::new();
        derive.add(names);
        derive
    }

    pub fn add<I, S>(&mut self, names: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for name in names {
            let inserted = self.0.insert(name.as_ref().to_string());
            debug_assert!(inserted);
        }
    }
}

impl ToTokens for Derive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if !self.0.is_empty() {
            let derive = self.0.iter().map(|derive| to_ident(derive));
            tokens.combine(quote! {
                #[derive(#(#derive),*)]
            })
        }
    }
}
