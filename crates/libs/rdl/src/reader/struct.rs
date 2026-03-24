use super::*;

#[derive(Debug, Clone)]
pub struct Struct {
    pub attrs: Vec<syn::Attribute>,
    pub span: proc_macro2::Span,
    pub name: syn::Ident,
    pub fields: Vec<Field>,
    pub winrt: bool,
}

impl syn::parse::Parse for Struct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token: syn::Token![struct] = input.parse()?;
        let name = input.parse()?;

        let content;
        syn::braced!(content in input);

        let fields = content
            .parse_terminated(Field::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            attrs,
            span: token.span,
            name,
            fields,
            winrt: false,
        })
    }
}

impl Encoder<'_> {
    pub fn encode_struct(&mut self, item: &Struct) -> Result<(), Error> {
        let type_def =
            self.encode_struct_or_union(&item.name.to_string(), item.winrt, false, &item.fields)?;

        if let Some(packing_size) = self.read_packed(&item.attrs)? {
            self.output.ClassLayout(type_def, packing_size, 0);
        }

        self.encode_attrs(
            metadata::writer::HasAttribute::TypeDef(type_def),
            &item.attrs,
            &["packed"],
        )
    }

    pub fn encode_struct_or_union(
        &mut self,
        item_name: &str,
        winrt: bool,
        is_union: bool,
        fields: &[Field],
    ) -> Result<metadata::writer::TypeDef, Error> {
        let value_type = self.output.TypeRef("System", "ValueType");

        let layout_flag = if is_union {
            metadata::TypeAttributes::ExplicitLayout
        } else {
            metadata::TypeAttributes::SequentialLayout
        };

        let flags = layout_flag
            | metadata::TypeAttributes::Sealed
            | metadata::TypeAttributes::Public
            | if winrt {
                metadata::TypeAttributes::WindowsRuntime
            } else {
                metadata::TypeAttributes::default()
            };

        let type_def = self.output.TypeDef(
            self.namespace,
            item_name,
            metadata::writer::TypeDefOrRef::TypeRef(value_type),
            flags,
        );

        for field in fields {
            let field_name = field.name.to_string();
            let mt = self.encode_type(&field.ty)?;
            let field_id = self
                .output
                .Field(&field_name, &mt, metadata::FieldAttributes::Public);
            if is_union {
                self.output.FieldLayout(field_id, 0);
            }
            self.encode_attrs(
                metadata::writer::HasAttribute::Field(field_id),
                &field.attrs,
                &[],
            )?;
        }

        Ok(type_def)
    }
}
