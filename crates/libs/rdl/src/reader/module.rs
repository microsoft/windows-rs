use super::*;

#[derive(Debug)]
pub struct Module {
    pub attrs: Vec<syn::Attribute>,
    pub token: syn::Token![mod],
    pub name: syn::Ident,
    pub items: Vec<Item>,
}

impl syn::parse::Parse for Module {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        if let Some(attr) = attrs.iter().find(|attr| is_opaque_attribute(attr)) {
            return Err(syn::Error::new_spanned(
                attr,
                "`opaque` is only valid on native records",
            ));
        }
        let token = input.parse()?;
        let name = input.parse()?;

        let content;
        syn::braced!(content in input);
        let mut items = vec![];

        while !content.is_empty() {
            items.push(content.parse()?);
        }

        Ok(Self {
            attrs,
            token,
            name,
            items,
        })
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.name.fmt(f)
    }
}
