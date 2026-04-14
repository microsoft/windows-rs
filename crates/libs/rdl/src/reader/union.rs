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
        let nested_in = self.read_nested(&item.attrs)?;
        let type_def = self.encode_struct_or_union(
            &item.name.to_string(),
            false,
            true,
            &item.fields,
            nested_in.as_ref().map(|id| id.to_string()).as_deref(),
        )?;

        if let Some(packing_size) = self.read_packed(&item.attrs)? {
            self.output.ClassLayout(type_def, packing_size, 0);
        }

        if let Some(outer_ident) = &nested_in {
            let outer_name = outer_ident.to_string();
            match self.typedef_ids.get(&outer_name).copied() {
                Some(outer_id) => self.output.NestedClass(type_def, outer_id),
                None => {
                    return self.err(
                        outer_ident,
                        &format!(
                            "`nested` outer type `{outer_name}` not found; \
                             ensure the outer type appears before the nested type"
                        ),
                    );
                }
            }
        }

        self.typedef_ids.insert(item.name.to_string(), type_def);

        self.encode_attrs(
            metadata::writer::HasAttribute::TypeDef(type_def),
            &item.attrs,
            &["packed", "nested"],
        )
    }
}
