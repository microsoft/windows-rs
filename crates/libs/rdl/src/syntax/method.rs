#[derive(Debug)]
pub struct Method {
    pub attrs: Vec<syn::Attribute>,
    pub sig: syn::Signature,
}

impl syn::parse::Parse for Method {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let sig = input.parse()?;
        input.parse::<syn::Token![;]>()?;

        Ok(Self { attrs, sig })
    }
}
