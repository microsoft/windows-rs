use crate::tables::*;
use crate::types::*;
use crate::{format_ident, TypeReader};
use std::collections::*;

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
        let default = format_ident(&self.fields[0].0);

        let repr = match self.fields[0].1 {
            EnumConstant::U32(_) => format_ident!("u32"),
            EnumConstant::I32(_) => format_ident!("i32"),
        };

        // Rust enum variants must be unique, but WinRT enums may contain duplicates
        // so we remove any duplicates ensuring there is at least one of each value.
        let mut values = BTreeSet::new();

        let fields = self.fields.iter().filter(|field| {
            if values.contains(&field.1) {
                false
            } else {
                values.insert(field.1);
                true
            }
        });

        let fields = fields.map(|field| {
            let name = format_ident(&field.0);

            let value = match field.1 {
                EnumConstant::U32(value) => quote! { #value },
                EnumConstant::I32(value) => quote! { #value },
            };

            quote! {
                #name = #value
            }
        });

        quote! {
            #[repr(#repr)]
            #[derive(Copy, Clone, Debug, PartialEq)]
            pub enum #name {
                #(#fields),*
            }
            impl Default for #name {
                fn default() -> Self {
                    Self::#default
                }
            }
            impl ::winrt::RuntimeType for #name {
                type Abi = Self;
                fn abi(&self) -> Self::Abi {
                    *self
                }
                fn set_abi(&mut self) -> *mut Self::Abi {
                    self as *mut Self::Abi
                }
            }
        }
    }
}
