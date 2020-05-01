use crate::tables::*;
use crate::types::*;
use crate::{format_ident, TypeReader};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug)]
pub struct Enum {
    pub name: TypeName,
    pub fields: Vec<(String, EnumConstant)>,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum EnumConstant {
    U32(u32),
    I32(i32),
}

impl Enum {
    pub fn from_type_def(reader: &TypeReader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
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

        Self { name, fields }
    }

    // TODO: need to model WinRT enums as structs rather than Rust enums as that would
    // avoid hte issue of duplicates below and also allow bit flags WinRT enums.
    pub fn to_tokens(&self) -> TokenStream {
        let name = self.name.to_tokens(&self.name.namespace);

        let repr = match self.fields[0].1 {
            EnumConstant::U32(_) => format_ident!("u32"),
            EnumConstant::I32(_) => format_ident!("i32"),
        };

        let fields = self.fields.iter().map(|field| {
            let name = format_ident(&field.0);

            let value = match field.1 {
                EnumConstant::U32(value) => quote! { #value },
                EnumConstant::I32(value) => quote! { #value },
            };

            quote! {
                pub const #name: Self = Self { value: #value };
            }
        });

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
                fn abi(&self) -> Self::Abi {
                    self.value
                }
                fn set_abi(&mut self) -> *mut Self::Abi {
                    &mut self.value
                }
            }
        }
    }
}
