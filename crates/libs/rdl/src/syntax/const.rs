// const Name: Type = Type { value };
// const Name: Type = value

#[derive(Debug)]
pub struct Const {
    pub attrs: Vec<syn::Attribute>,
    pub token: syn::Token![const],
    pub name: syn::Ident,
    pub ty: syn::Type,
    pub expr: syn::Expr,
}

impl syn::parse::Parse for Const {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;
        input.parse::<syn::Token![:]>()?;
        let ty = input.parse()?;
        input.parse::<syn::Token![=]>()?;
        let expr = input.parse()?;
        input.parse::<syn::Token![;]>()?;

        Ok(Self {
            attrs,
            token,
            name,
            ty,
            expr,
        })
    }
}
