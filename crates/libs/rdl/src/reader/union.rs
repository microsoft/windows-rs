use super::*;

#[derive(Debug, Clone)]
pub struct Union {
    pub attrs: Vec<syn::Attribute>,
    pub name: syn::Ident,
    pub fields: Vec<Field>,
}

impl syn::parse::Parse for Union {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let _: syn::Token![union] = input.parse()?;
        let name = input.parse()?;

        let content;
        syn::braced!(content in input);

        let fields = content
            .parse_terminated(Field::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            attrs,
            name,
            fields,
        })
    }
}

impl Union {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let mut breadcrumbs = vec![];
        let type_def = encode_body(
            encoder,
            &self.name.to_string(),
            None,
            false,
            true,
            &self.fields,
            &mut breadcrumbs,
        )?;
        encode_attrs(
            encoder,
            metadata::writer::HasAttribute::TypeDef(type_def),
            &self.attrs,
            &[],
        )
    }
}
