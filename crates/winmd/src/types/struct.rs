use crate::tables::*;
use crate::types::*;
use crate::{write_ident, Reader};

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct Struct {
    pub name: TypeName,
    pub fields: Vec<(String, TypeKind)>, // TODO: might have to be a full Type to ensure we can write out nested structs for ABI layout
}

impl Struct {
    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.fields
            .iter()
            .flat_map(|i| i.1.dependencies())
            .collect()
    }

    pub fn from_type_def(reader: &Reader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
        let mut fields = Vec::new();

        for field in def.fields(reader) {
            let name = field.name(reader).to_string();
            let kind = TypeKind::from_field(reader, field);
            fields.push((name, kind));
        }

        Self { name, fields }
    }

    pub fn to_stream(&self) -> TokenStream {
        let name = self.name.ident();

        let fields = self.fields.iter().map(|field| {
            let name = write_ident(&field.0);
            let kind = field.1.ident();
            quote! {
                pub #name: #kind
            }
        });

        quote! {
            #[repr(C)]
            #[derive(Copy, Clone, Default, Debug, PartialEq)]
            pub struct #name {
                #(#fields),*
            }
        }
    }
}
