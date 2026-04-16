use super::*;

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub namespace: String,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn parse(cursor: Cursor, namespace: &str) -> Result<Self, Error> {
        let name = cursor.name();
        let mut fields = vec![];

        for child in cursor.children() {
            if child.kind() != CXCursor_FieldDecl {
                continue;
            }

            let name = child.name();
            let ty = child.ty().to_type(namespace);
            fields.push(Field { name, ty });
        }

        Ok(Self {
            name,
            namespace: namespace.to_string(),
            fields,
        })
    }

    pub fn write(&self) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);

        let fields = self.fields.iter().map(|field| {
            let name = write_ident(&field.name);
            let ty = write_type(&self.namespace, &field.ty);
            quote! { #name: #ty, }
        });

        Ok(quote! {
            struct #name {
                #(#fields)*
            }
        })
    }
}
