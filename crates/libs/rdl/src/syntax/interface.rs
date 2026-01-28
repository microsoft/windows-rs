use super::*;

syn::custom_keyword!(interface);

#[derive(Debug)]
pub struct Interface {
    pub attrs: Vec<syn::Attribute>,
    pub token: interface,
    pub name: syn::Ident,
    // pub requires: Vec<TypeParamBound>,
    pub methods: Vec<Method>,
}

impl syn::parse::Parse for Interface {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;

        let content;
        syn::braced!(content in input);
        let mut methods = Vec::new();

        while !content.is_empty() {
            methods.push(content.parse()?);
        }

        Ok(Self {
            attrs,
            token,
            name,
            methods,
        })
    }
}
