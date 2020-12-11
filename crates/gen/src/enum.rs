use crate::*;
use squote::{format_ident, quote, Literal, TokenStream};

#[derive(Debug)]
pub struct Enum {
    pub name: TypeName,
    pub fields: Vec<(&'static str, EnumConstant)>,
    pub underlying_type: winmd::ElementType,
    pub signature: String,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum EnumConstant {
    U32(u32),
    I32(i32),
}

impl Enum {
    pub fn from_type_name(name: TypeName) -> Self {
        let signature = if name.def.is_winrt() {
            name.enum_signature()
        } else {
            String::new()
        };

        let mut fields = Vec::new();
        let mut underlying_type = None;

        for field in name.def.fields() {
            if let Some(constant) = field.constants().next() {
                let mut value = constant.value();

                let value = match constant.value_type() {
                    winmd::ElementType::I32 => EnumConstant::I32(value.read_i32()),
                    winmd::ElementType::U32 => EnumConstant::U32(value.read_u32()),
                    _ => panic!("Enum::from_type_def"),
                };

                fields.push((field.name(), value));
            } else {
                let blob = &mut field.sig();
                blob.read_unsigned();
                blob.read_modifiers();

                blob.read_expected(0x1D);
                blob.read_modifiers();

                underlying_type = Some(winmd::ElementType::from_blob(blob));
            }
        }

        Self {
            name,
            fields,
            underlying_type: underlying_type.expect("Enum.from_type_name"),
            signature,
        }
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.name.gen();

        let (underlying_type, bitwise) = match self.underlying_type {
            winmd::ElementType::I32 => (format_ident!("i32"), TokenStream::new()),
            winmd::ElementType::U32 => (
                format_ident!("u32"),
                quote! {
                    impl ::std::ops::BitOr for #name {
                        type Output = Self;

                        fn bitor(self, rhs: Self) -> Self {
                            Self(self.0 | rhs.0)
                        }
                    }
                    impl ::std::ops::BitAnd for #name {
                        type Output = Self;

                        fn bitand(self, rhs: Self) -> Self {
                            Self(self.0 & rhs.0)
                        }
                    }
                },
            ),
            _ => panic!("Unexpected enum underlying type: {}", name),
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

        let runtime_type = if self.signature.is_empty() {
            TokenStream::new()
        } else {
            let signature = Literal::byte_string(&self.signature.as_bytes());

            quote! {
                unsafe impl ::winrt::RuntimeType for #name {
                    type DefaultType = Self;
                    const SIGNATURE: ::winrt::ConstBuffer = ::winrt::ConstBuffer::from_slice(#signature);
                }
            }
        };

        quote! {
            #[allow(non_camel_case_types)]
            #[repr(transparent)]
            pub struct #name(#underlying_type);
            impl ::std::convert::From<#underlying_type> for #name {
                fn from(value: #underlying_type) -> Self {
                    Self(value)
                }
            }
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
                type Abi = Self;
            }
            #runtime_type
            #bitwise
        }
    }
}
