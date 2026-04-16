use super::*;

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn parse(cursor: Cursor) -> Result<Self, Error> {
        let name = cursor.name();
        let mut fields = vec![];

        for child in cursor.children() {
            if child.kind() == CXCursor_FieldDecl {
                let name = child.name();
                let ty = match child.ty().kind {
                    CXType_Char_U | CXType_UChar => "u8",
                    CXType_UShort => "u16",
                    CXType_UInt => "u32",
                    CXType_ULong => "usize",
                    CXType_ULongLong => "u64",
                    CXType_Char_S | CXType_SChar => "i8",
                    CXType_Short => "i16",
                    CXType_Int => "i32",
                    CXType_Long => "isize",
                    CXType_LongLong => "i64",
                    CXType_Float => "f32",
                    CXType_Double => "f64",
                    _ => continue,
                };
                fields.push(Field { name, ty });
            }
        }

        Ok(Self { name, fields })
    }

    pub fn write(&self) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);

        let fields = self.fields.iter().map(|field| {
            let name = write_ident(&field.name);
            let ty = write_ident(field.ty);
            quote! { #name: #ty, }
        });

        Ok(quote! {
            struct #name {
                #(#fields)*
            }
        })
    }
}
