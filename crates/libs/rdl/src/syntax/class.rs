syn::custom_keyword!(class);

#[derive(Debug)]
pub struct Class {
    pub attrs: Vec<syn::Attribute>,
    pub token: class,
    pub name: syn::Ident,
    pub extends: Option<syn::Path>,
    pub interfaces: Vec<ClassInterface>,
}

impl syn::parse::Parse for Class {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;

        let extends = if input.parse::<syn::Token![:]>().is_ok() {
            Some(input.parse()?)
        } else {
            None
        };

        let content;
        syn::braced!(content in input);

        let interfaces = content
            .parse_terminated(ClassInterface::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            attrs,
            token,
            name,
            extends,
            interfaces,
        })
    }
}

#[derive(Debug)]
pub struct ClassInterface {
    pub attrs: Vec<syn::Attribute>,
    pub ty: syn::Path,
}

impl syn::parse::Parse for ClassInterface {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let ty = input.parse()?;

        Ok(Self { attrs, ty })
    }
}
