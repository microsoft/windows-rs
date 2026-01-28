use super::*;

#[derive(Debug)]
pub struct File {
    pub items: Vec<Item>,
}

impl syn::parse::Parse for File {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            items: {
                let mut items = Vec::new();
                while !input.is_empty() {
                    items.push(input.parse()?);
                }
                items
            },
        })
    }
}
