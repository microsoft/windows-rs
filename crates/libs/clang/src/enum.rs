use super::*;

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub repr: &'static str,
    pub variants: Vec<(String, i64)>,
    pub flags: bool,
    /// C++ `enum class` / `enum struct` -> `ScopedEnum`.
    pub scoped: bool,
}

impl Enum {
    pub fn parse(cursor: Cursor) -> Result<Self, Error> {
        let repr = match cursor.enum_repr().kind() {
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
        let scoped = cursor.is_scoped_enum();

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
            flags: false,
            scoped,
        })
    }

    pub fn write(&self) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);
        // Flag enums are bit masks; promote signed backing to same-width unsigned.
        let repr_str = if self.flags {
            match self.repr {
                "i8" => "u8",
                "i16" => "u16",
                "i32" => "u32",
                "i64" => "u64",
                other => other,
            }
        } else {
            self.repr
        };
        let repr = write_ident(repr_str);

        let variants = self.variants.iter().map(|(name, value)| {
            let name = write_ident(name);
            // clang returns a signed bit pattern; reinterpret it for unsigned reprs.
            let value = match repr_str {
                "u8" => Literal::u8_unsuffixed(*value as u8),
                "u16" => Literal::u16_unsuffixed(*value as u16),
                "u32" => Literal::u32_unsuffixed(*value as u32),
                "u64" => Literal::u64_unsuffixed(*value as u64),
                "i8" => Literal::i8_unsuffixed(*value as i8),
                "i16" => Literal::i16_unsuffixed(*value as i16),
                "i64" => Literal::i64_unsuffixed(*value),
                _ => Literal::i32_unsuffixed(*value as i32),
            };
            quote! { #name = #value, }
        });

        let flags_attr = if self.flags {
            quote! { #[flags] }
        } else {
            quote! {}
        };

        let scoped_attr = if self.scoped {
            quote! { #[scoped] }
        } else {
            quote! {}
        };

        Ok(quote! {
            #[repr(#repr)]
            #flags_attr
            #scoped_attr
            enum #name {
                #(#variants)*
            }
        })
    }
}
