use super::*;

#[derive(Debug)]
pub struct File {
    pub items: Vec<Item>,
    pub uses: Vec<syn::ItemUse>,
    pub source: String,
}

impl syn::parse::Parse for File {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut items = vec![];
        let mut uses = vec![];
        while !input.is_empty() {
            if input.peek(syn::Token![use]) {
                uses.push(input.parse::<syn::ItemUse>()?);
            } else {
                items.push(Item::Module(input.parse()?));
            }
        }

        Ok(Self {
            items,
            uses,
            source: String::new(),
        })
    }
}
