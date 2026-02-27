syn::custom_keyword!(attribute);

#[derive(Debug)]
pub struct Attribute {
    pub attrs: Vec<syn::Attribute>,
    pub token: attribute,
    pub name: syn::Ident,
    pub methods: Vec<syn::TypeBareFn>,
    pub winrt: bool,
}

impl syn::parse::Parse for Attribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;

        let content;
        syn::braced!(content in input);
        let mut methods = vec![];

        while !content.is_empty() {
            methods.push(content.parse()?);
            content.parse::<syn::Token![;]>()?;
        }

        Ok(Self {
            attrs,
            token,
            name,
            methods,
            winrt: false,
        })
    }
}
