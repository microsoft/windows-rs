pub struct Method {
    pub attrs: Vec<syn::Attribute>,
    pub sig: syn::Signature,
    pub semi_token: syn::Token![;],
}

impl syn::parse::Parse for Method {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let sig = input.parse()?;
        let semi_token = input.parse()?;

        Ok(Self {
            attrs,
            sig,
            semi_token,
        })
    }
}
