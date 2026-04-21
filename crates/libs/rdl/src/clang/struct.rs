use super::*;

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn parse(
        cursor: Cursor,
        namespace: &str,
        ref_map: &HashMap<String, String>,
        tag_rename: &HashMap<String, String>,
        pending: &mut Vec<Cursor>,
    ) -> Result<Self, Error> {
        let tag_name = cursor.name();
        // Use the public typedef alias if one exists (e.g. `_TEST` → `TEST`).
        let name = tag_rename.get(&tag_name).cloned().unwrap_or(tag_name);
        let mut fields = vec![];

        for child in cursor.children() {
            if child.kind() != CXCursor_FieldDecl {
                continue;
            }

            let name = child.name();
            let ty = child.ty().to_type(namespace, ref_map, tag_rename, pending);
            fields.push(Field { name, ty });
        }

        Ok(Self { name, fields })
    }

    pub fn write(&self, namespace: &str) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);

        let fields = self.fields.iter().map(|field| {
            let name = write_ident(&field.name);
            let ty = write_type(namespace, &field.ty);
            quote! { #name: #ty, }
        });

        Ok(quote! {
            struct #name {
                #(#fields)*
            }
        })
    }
}
