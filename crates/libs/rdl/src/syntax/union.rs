#[derive(Debug)]
pub struct Union {
    pub attrs: Vec<syn::Attribute>,
    pub token: syn::Token![union],
    pub fields: Vec<syn::Field>,
    pub name: syn::Ident,
}

impl syn::parse::Parse for Union {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;

        let content;
        syn::braced!(content in input);

        let fields = content
            .parse_terminated(syn::Field::parse_named, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            attrs,
            token,
            name,
            fields,
        })
    }
}
