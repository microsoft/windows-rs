syn::custom_keyword!(delegate);

#[derive(Debug)]
pub struct Delegate {
    pub attrs: Vec<syn::Attribute>,
    pub token: delegate,
    pub sig: syn::Signature,
    pub winrt: bool,
}

impl syn::parse::Parse for Delegate {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let sig = input.parse()?;
        input.parse::<syn::Token![;]>()?;

        Ok(Self {
            attrs,
            token,
            sig,
            winrt: false,
        })
    }
}
