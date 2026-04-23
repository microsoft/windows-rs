use super::*;

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub is_union: bool,
}

impl Struct {
    pub fn parse(cursor: Cursor, parser: &mut Parser<'_>, is_union: bool) -> Result<Self, Error> {
        let tag_name = cursor.name();
        // Use the public typedef alias if one exists (e.g. `_TEST` → `TEST`).
        // For anonymous types the spelling is empty; fall back to location_id.
        let name = if is_anonymous_name(&tag_name) {
            parser
                .tag_rename
                .get(&cursor.location_id())
                .cloned()
                .unwrap_or(tag_name)
        } else {
            parser
                .tag_rename
                .get(&tag_name)
                .cloned()
                .unwrap_or(tag_name)
        };
        let mut fields = vec![];

        for child in cursor.children() {
            if child.kind() != CXCursor_FieldDecl {
                continue;
            }

            let name = child.name();
            let ty = child.ty().to_type(parser);
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
