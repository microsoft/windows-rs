pub struct ItemEnum {
    pub attrs: Vec<syn::Attribute>,
    pub enum_token: syn::Token![enum],
    pub ident: syn::Ident,
    pub brace_token: syn::token::Brace,
    pub variants: syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
}

impl syn::parse::Parse for ItemEnum {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let enum_token = input.parse()?;
        let ident = input.parse()?;

        let content;
        let brace_token = syn::braced!(content in input);
        let variants = content.parse_terminated(syn::Variant::parse, syn::Token![,])?;

        Ok(Self {
            attrs,
            enum_token,
            ident,
            brace_token,
            variants,
        })
    }
}
