use super::*;

pub struct ItemInterface {
    pub attrs: Vec<syn::Attribute>,
    pub interface_token: interface,
    pub ident: syn::Ident,
    // pub colon_token: Option<Token![:]>,
    // pub requires: Punctuated<TypeParamBound, Token![+]>,
    pub brace_token: syn::token::Brace,
    pub methods: Vec<Method>,
}

impl syn::parse::Parse for ItemInterface {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let interface_token = input.parse()?;
        let ident = input.parse()?;

        let content;
        let brace_token = syn::braced!(content in input);
        let mut methods = Vec::new();

        while !content.is_empty() {
            methods.push(content.parse()?);
        }

        Ok(Self {
            attrs,
            interface_token,
            ident,
            brace_token,
            methods,
        })
    }
}
