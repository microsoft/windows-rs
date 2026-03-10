use super::*;

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

impl Union {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let value_type = encoder.output.TypeRef("System", "ValueType");

        let type_def = encoder.output.TypeDef(
            encoder.namespace,
            encoder.name,
            metadata::writer::TypeDefOrRef::TypeRef(value_type),
            metadata::TypeAttributes::Public
                | metadata::TypeAttributes::ExplicitLayout
                | metadata::TypeAttributes::Sealed,
        );

        // Emit any Named attributes attached to this union.
        encode_attrs(
            encoder,
            metadata::writer::HasAttribute::TypeDef(type_def),
            &self.attrs,
            &[],
        )?;

        for field in &self.fields {
            let name = field.ident.as_ref().unwrap().to_string();
            let ty = encode_type(encoder, &field.ty)?;
            let field_id = encoder
                .output
                .Field(&name, &ty, metadata::FieldAttributes::Public);

            encoder.output.FieldLayout(field_id, 0);

            encode_attrs(
                encoder,
                metadata::writer::HasAttribute::Field(field_id),
                &field.attrs,
                &[],
            )?;
        }

        Ok(())
    }
}
