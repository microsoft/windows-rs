use crate::tables::*;
use crate::types::*;
use crate::{format_ident, TypeReader};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug)]
pub struct Enum {
    pub name: TypeName,
    pub fields: Vec<(String, EnumConstant)>,
    pub signature: String,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum EnumConstant {
    U32(u32),
    I32(i32),
}

impl Enum {
    pub fn from_type_def(reader: &TypeReader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
        let signature = name.enum_signature(reader);
        let mut fields = Vec::new();

        for field in def.fields(reader) {
            for constant in field.constants(reader) {
                let name = field.name(reader).to_string();
                let mut value = constant.value(reader);

                let value = match constant.value_type(reader) {
                    0x08 => EnumConstant::I32(value.read_i32()),
                    0x09 => EnumConstant::U32(value.read_u32()),
                    _ => panic!("Enum::from_type_def"),
                };

                fields.push((name, value));
            }
        }

        Self {
            name,
            fields,
            signature,
        }
    }

    pub fn to_tokens(&self) -> TokenStream {
        let name = &*self.name.to_tokens(&self.name.namespace);
        let signature = &self.signature;

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
                pub const #name: Self = Self { value: #value };
            }
        });
        let bitwise = bitwise_operators(&name, &self.fields[0].1);

        quote! {
            #[repr(transparent)]
            #[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
            pub struct #name {
                value: #repr
            }
            impl #name {
                #![allow(non_upper_case_globals)]
                #(#fields)*
            }
            unsafe impl ::winrt::RuntimeType for #name {
                type Abi = #repr;
                fn signature() -> String {
                    #signature.to_owned()
                }
                fn abi(&self) -> Self::Abi {
                    self.value
                }
                fn set_abi(&mut self) -> *mut Self::Abi {
                    &mut self.value
                }
            }
            #bitwise
        }
    }
}

fn bitwise_operators(name: &TokenStream, value_type: &EnumConstant) -> TokenStream {
    match value_type {
        EnumConstant::I32(_) => return quote! {},
        _ => {}
    }

    quote! {
        impl ::std::ops::BitOr for #name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self {
                Self { value: self.value | rhs.value }
            }
        }
        impl ::std::ops::BitAnd for #name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self {
                Self { value: self.value & rhs.value }
            }
        }
    }
}
