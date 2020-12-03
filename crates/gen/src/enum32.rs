use crate::*;
use squote::{format_ident, quote, TokenStream};

#[derive(Debug)]
pub struct Enum32 {
    pub name: TypeName,
    pub fields: Vec<(String, EnumConstant)>,
}

impl Enum32 {
    pub fn from_type_name(name: TypeName) -> Self {
        let mut fields = Vec::new();

        for field in name.def.fields() {
            for constant in field.constants() {
                let name = field.name().to_string();
                let mut value = constant.value();

                let value = match constant.value_type() {
                    winmd::ElementType::I32 => EnumConstant::I32(value.read_i32()),
                    winmd::ElementType::U32 => EnumConstant::U32(value.read_u32()),
                    _ => panic!("Enum32::from_type_def"),
                };

                fields.push((name, value));
            }
        }

        Self { name, fields }
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.name.gen();

        if self.fields.is_empty() {
            return TokenStream::new();
        }

        let repr = match self.fields[0].1 {
            EnumConstant::U32(_) => format_ident!("u32"),
            EnumConstant::I32(_) => format_ident!("i32"),
        };

        let fields = self.fields.iter().map(|(name, value)| {
            let name = format_ident(&name);
            let value = match value {
                EnumConstant::U32(value) => quote! { #value },
                EnumConstant::I32(value) => quote! { #value },
            };

            quote! {
                pub const #name: Self = Self(#value);
            }
        });

        let bitwise = bitwise_operators(&name, self.fields[0].1);

        quote! {
            #[repr(transparent)]
            #[allow(non_camel_case_types)]
            pub struct #name(#repr);
            impl ::std::clone::Clone for #name {
                fn clone(&self) -> Self {
                    Self(self.0)
                }
            }
            impl ::std::default::Default for #name {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl ::std::fmt::Debug for #name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{:?}", self.0)
                }
            }
            impl ::std::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl ::std::cmp::Eq for #name {}
            impl ::std::marker::Copy for #name {}
            impl #name {
                #![allow(non_upper_case_globals)]
                #(#fields)*
            }
            unsafe impl ::winrt::Abi for #name {
                type Abi = #repr;
            }
            #bitwise
        }
    }
}
