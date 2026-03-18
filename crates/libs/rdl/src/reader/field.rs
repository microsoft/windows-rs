/// A field in a struct or union.
#[derive(Debug, Clone)]
pub struct Field {
    pub attrs: Vec<syn::Attribute>,
    pub name: syn::Ident,
    pub ty: syn::Type,
}

impl syn::parse::Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let name: syn::Ident = input.parse()?;
        input.parse::<syn::Token![:]>()?;
        let ty = input.parse()?;
        Ok(Field { attrs, name, ty })
    }
}
