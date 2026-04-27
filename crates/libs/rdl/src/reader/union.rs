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

impl Encoder<'_> {
    pub fn encode_union(&mut self, item: &Union) -> Result<(), Error> {
        let type_def =
            self.encode_struct_or_union(&item.name.to_string(), false, true, &item.fields)?;

        if let Some(packing_size) = self.read_packed(&item.attrs)? {
            self.output.ClassLayout(type_def, packing_size, 0);
        }

        if let Some(arch_bits) = self.read_arch(&item.attrs)? {
            self.emit_arch_attribute(metadata::writer::HasAttribute::TypeDef(type_def), arch_bits);
        }

        self.encode_attrs(
            metadata::writer::HasAttribute::TypeDef(type_def),
            &item.attrs,
            &["packed"],
        )
    }
}
