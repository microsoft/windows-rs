use crate::tables::*;
use crate::types::*;
use crate::{write_ident, TypeReader};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug)]
pub struct Enum {
    pub name: TypeName,
    pub fields: Vec<(String, EnumConstant)>,
}

#[derive(Debug)]
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

    pub fn to_stream(&self) -> TokenStream {
        let name = self.name.ident();
        let default = write_ident(&self.fields[0].0);

        let repr = match self.fields[0].1 {
            EnumConstant::U32(_) => format_ident!("u32"),
            EnumConstant::I32(_) => format_ident!("i32"),
        };

        let fields = self.fields.iter().map(|field| {
            let name = write_ident(&field.0);

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
        }
    }
}
