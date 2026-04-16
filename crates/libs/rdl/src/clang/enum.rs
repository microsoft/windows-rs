use super::*;

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub repr: &'static str,
    pub variants: Vec<(String, i64)>,
}

impl Enum {
    pub fn parse(cursor: Cursor) -> Result<Self, Error> {
        let repr = match cursor.enum_repr().kind {
            CXType_Int | CXType_Long => "i32",
            CXType_UInt | CXType_ULong => "u32",
            CXType_Short => "i16",
            CXType_UShort => "u16",
            CXType_Char_S | CXType_SChar => "i8",
            CXType_Char_U | CXType_UChar => "u8",
            CXType_LongLong => "i64",
            CXType_ULongLong => "u64",
            _ => "i32",
        };

        let name = cursor.name();

        let mut variants = vec![];

        for child in cursor.children() {
            if child.kind() == CXCursor_EnumConstantDecl {
                let name = child.name();
                let value = child.enum_value();
                variants.push((name, value));
            }
        }

        Ok(Self {
            name,
            repr,
            variants,
        })
    }

    pub fn write(&self) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);
        let repr = write_ident(self.repr);

        let variants = self.variants.iter().map(|(name, value)| {
            let name = write_ident(name);
            let value = Literal::i64_unsuffixed(*value);
            quote! { #name = #value, }
        });

        Ok(quote! {
            #[repr(#repr)]
            enum #name {
                #(#variants)*
            }
        })
    }
}
