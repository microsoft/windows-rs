use super::*;

#[derive(Debug)]
pub struct File {
    pub items: Vec<Item>,
}

impl syn::parse::Parse for File {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            items: {
                let mut items = vec![];
                while !input.is_empty() {
                    items.push(Item::Module(input.parse()?));

                    // TODO: possibly support file-level items other than module for nested-file module declarations e.g. `mod Nested;`
                    //items.push(input.parse()?);
                }
                items
            },
        })
    }
}
