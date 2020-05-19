use crate::case::to_snake;
use crate::tables::*;
use crate::types::*;
use crate::{format_ident, TypeReader};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct Struct {
    pub name: TypeName,
    pub fields: Vec<(String, TypeKind)>, // TODO: might have to be a full Type to ensure we can write out nested structs for ABI layout
    pub signature: String,
}

impl Struct {
    pub fn from_type_def(reader: &TypeReader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
        let signature = name.struct_signature(reader);
        let mut fields = Vec::new();

        for field in def.fields(reader) {
            let name = to_snake(field.name(reader), MethodKind::Normal);
            let kind = TypeKind::from_field(reader, field);
            fields.push((name, kind));
        }

        Self {
            name,
            fields,
            signature,
        }
    }

    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.fields
            .iter()
            .flat_map(|i| i.1.dependencies())
            .collect()
    }

    pub fn to_tokens(&self) -> TokenStream {
        let name = &*self.name.to_tokens(&self.name.namespace);
        let signature = &self.signature;

        let fields = self.fields.iter().map(|field| {
            let name = format_ident(&field.0);
            let kind = field.1.to_tokens(&self.name.namespace);
            quote! {
                pub #name: #kind
            }
        });

        quote! {
            #[repr(C)]
            #[derive(Clone, Default, Debug, PartialEq)]
            pub struct #name {
                #(#fields),*
            }
            unsafe impl ::winrt::RuntimeType for #name {
                type Abi = Self;
                fn signature() -> String {
                    #signature.to_owned()
                }
                fn abi(&self) -> Self::Abi {
                    self.clone()
                }
                fn set_abi(&mut self) -> *mut Self::Abi {
                    self as *mut Self::Abi
                }
            }
        }
    }
}
