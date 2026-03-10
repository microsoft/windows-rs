use super::*;

#[derive(Debug, Clone)]
pub struct Union {
    pub attrs: Vec<syn::Attribute>,
    pub token: syn::Token![union],
    pub fields: Vec<StructField>,
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
            .parse_terminated(StructField::parse, syn::Token![,])?
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

impl Union {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let item = Struct {
            attrs: self.attrs.clone(),
            span: self.token.span,
            name: Some(self.name.clone()),
            fields: self.fields.clone(),
            winrt: false,
            is_union: true,
        };
        let mut breadcrumbs = vec![];
        encode_struct_inner(
            encoder,
            &item,
            &self.name.to_string(),
            None,
            &mut breadcrumbs,
        )
    }
}
