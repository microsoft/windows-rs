use super::*;

#[derive(Debug)]
pub struct Enum {
    pub attrs: Vec<syn::Attribute>,
    pub token: syn::Token![enum],
    pub name: syn::Ident,
    pub variants: Vec<syn::Variant>,
    pub winrt: bool,
}

impl syn::parse::Parse for Enum {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;

        let content;
        syn::braced!(content in input);

        let variants = content
            .parse_terminated(syn::Variant::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            attrs,
            token,
            name,
            variants,
            winrt: false,
        })
    }
}

impl Enum {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let value_type = encoder.output.TypeRef("System", "Enum");

        let mut flags = metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed;

        if self.winrt {
            flags |= metadata::TypeAttributes::WindowsRuntime;
        }

        encoder.output.TypeDef(
            encoder.namespace,
            encoder.name,
            metadata::writer::TypeDefOrRef::TypeRef(value_type),
            flags,
        );

        let Some(attribute) = self
            .attrs
            .iter()
            .find(|attribute| attribute.path().is_ident("repr"))
        else {
            return encoder.err(self.token, "`repr` attribute not found");
        };

        let Ok(ty) = attribute.parse_args::<syn::Path>() else {
            return encoder.err(attribute, "`repr` integer type attribute not found");
        };

        let ty = encode_path(encoder, &ty)?;

        encoder.output.Field(
            "value__",
            &ty,
            metadata::FieldAttributes::Private
                | metadata::FieldAttributes::SpecialName
                | metadata::FieldAttributes::RTSpecialName,
        );

        let type_name = metadata::Type::named(encoder.namespace, encoder.name);

        for variant in &self.variants {
            let name = variant.ident.to_string();

            let field = encoder.output.Field(
                &name,
                &type_name,
                metadata::FieldAttributes::Public
                    | metadata::FieldAttributes::Static
                    | metadata::FieldAttributes::Literal,
            );

            let Some((_, value)) = &variant.discriminant else {
                return encoder.err(variant, "variant value not found");
            };

            let value = encode_value(encoder, &ty, value)?;

            encoder
                .output
                .Constant(metadata::writer::HasConstant::Field(field), &value);
        }

        Ok(())
    }
}
