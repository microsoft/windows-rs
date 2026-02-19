#[derive(Debug)]
pub struct Struct {
    pub attrs: Vec<syn::Attribute>,
    pub token: syn::Token![struct],
    pub name: Option<syn::Ident>,
    pub fields: Vec<StructField>,
    pub winrt: bool,
}

#[derive(Debug)]
pub enum StructField {
    Regular(syn::Field),
    Nested {
        name: syn::Ident,
        def: Struct,
    },
}

impl syn::parse::Parse for Struct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;

        let content;
        syn::braced!(content in input);

        let fields = content
            .parse_terminated(StructField::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            attrs,
            token,
            name,
            fields,
            winrt: false,
        })
    }
}

impl syn::parse::Parse for StructField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<syn::Token![:]>()?;

        if input.peek(syn::Token![struct]) {
            Ok(StructField::Nested { name, def: input.parse()? })
        } else {
            Ok(StructField::Regular(syn::Field {
                attrs: vec![],
                ident: Some(name),
                ty: input.parse()?,
                vis: syn::Visibility::Inherited,
                colon_token: Some(Default::default()),
                mutability: syn::FieldMutability::None,
            }))
        }
    }
}
