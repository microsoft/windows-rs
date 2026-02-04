#[derive(Debug)]
pub struct Fn {
    pub attrs: Vec<syn::Attribute>,
    pub sig: syn::Signature,
}

impl syn::parse::Parse for Fn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let sig = input.parse()?;
        input.parse::<syn::Token![;]>()?;

        Ok(Self { attrs, sig })
    }
}
