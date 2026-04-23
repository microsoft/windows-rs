use super::*;

#[derive(Debug, Clone)]
pub struct Typedef {
    pub attrs: Vec<syn::Attribute>,
    pub name: syn::Ident,
    pub ty: syn::Type,
}

impl syn::parse::Parse for Typedef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        input.parse::<syn::Token![type]>()?;
        let name = input.parse()?;
        input.parse::<syn::Token![=]>()?;
        let ty = input.parse()?;
        input.parse::<syn::Token![;]>()?;
        Ok(Self { attrs, name, ty })
    }
}

impl Encoder<'_> {
    pub fn encode_typedef(&mut self, item: &Typedef) -> Result<(), Error> {
        let value_type = self.output.TypeRef("System", "ValueType");

        let flags = metadata::TypeAttributes::SequentialLayout
            | metadata::TypeAttributes::Sealed
            | metadata::TypeAttributes::Public;

        let type_def = self.output.TypeDef(
            self.namespace,
            &item.name.to_string(),
            metadata::writer::TypeDefOrRef::TypeRef(value_type),
            flags,
        );

        // Encode the underlying type as a single field named `value`.
        let mt = self.encode_type(&item.ty)?;
        self.output
            .Field("Value", &mt, metadata::FieldAttributes::Public);

        // Add the NativeTypedefAttribute.
        self.encode_native_typedef_attribute(metadata::writer::HasAttribute::TypeDef(type_def));

        Ok(())
    }
}
