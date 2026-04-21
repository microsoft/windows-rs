use super::*;

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub is_union: bool,
}

impl Struct {
    pub fn parse(
        cursor: Cursor,
        namespace: &str,
        ref_map: &HashMap<String, String>,
        pending: &mut Vec<Cursor>,
        is_union: bool,
    ) -> Result<Self, Error> {
        let name = cursor.name();
        let mut fields = vec![];

        for child in cursor.children() {
            if child.kind() != CXCursor_FieldDecl {
                continue;
            }

            let name = child.name();
            let ty = child.ty().to_type(namespace, ref_map, pending);
            fields.push(Field { name, ty });
        }

        Ok(Self {
            name,
            fields,
            is_union,
        })
    }

    pub fn write(&self, namespace: &str) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);

        let fields = self.fields.iter().map(|field| {
            let name = write_ident(&field.name);
            let ty = write_type(namespace, &field.ty);
            quote! { #name: #ty, }
        });

        let keyword = if self.is_union {
            quote! { union }
        } else {
            quote! { struct }
        };

        Ok(quote! {
            #keyword #name {
                #(#fields)*
            }
        })
    }
}
