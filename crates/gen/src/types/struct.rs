use crate::case::to_snake;
use crate::tables::*;
use crate::types::*;
use crate::{format_ident, TypeReader};
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Struct {
    pub name: TypeName,
    pub fields: Vec<(String, TypeKind)>, // TODO: might have to be a full Type to ensure we can write out nested structs for ABI layout
    pub signature: String,
}

impl Struct {
    pub fn from_type_name(reader: &TypeReader, name: TypeName) -> Self {
        let signature = name.struct_signature(reader);
        let mut fields = Vec::new();

        for field in name.def.fields(reader) {
            let field_name = to_snake(field.name(reader), MethodKind::Normal);
            let kind = TypeKind::from_field(reader, field, &name.namespace);
            fields.push((field_name, kind));
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
        let name = &self.name.tokens;
        let signature = &self.signature;

        let fields = self.fields.iter().map(|field| {
            let name = format_ident(&field.0);
            let kind = field.1.to_tokens();
            quote! {
                pub #name: #kind
            }
        });

        let extensions = self.add_extensions();

        quote! {
            #[repr(C)]
            #[derive(Clone, Default, Debug, PartialEq)]
            pub struct #name {
                #(#fields),*
            }
            unsafe impl ::winrt::RuntimeType for #name {
                fn signature() -> String {
                    #signature.to_owned()
                }
            }
            unsafe impl ::winrt::AbiTransferable for #name {
                type Abi = Self;
                fn get_abi(&self) -> Self::Abi {
                    self.clone()
                }
                fn set_abi(&mut self) -> *mut Self::Abi {
                    self as *mut Self::Abi
                }
            }
            #extensions
        }
    }

    fn add_extensions(&self) -> TokenStream {
        match (&self.name.namespace as &str, &self.name.name as &str) {
            ("Windows.Foundation.Numerics", "Vector2")
                | ("Windows.Foundation.Numerics", "Vector3")
                | ("Windows.Foundation.Numerics", "Vector4") 
            => {
                let ops = [Operation::Add, Operation::Sub, Operation::Div, Operation::Mul];
                let impls = {
                    let mut impls = Vec::new();
                    for op in &ops {
                        let tokens = op.to_tokens(self);
                        impls.push(quote!{ #tokens });
                    }
                    TokenStream::from_iter(impls)
                };
                quote! {
                    #impls
                }
            }
            _ => { quote!{} }
        }
    }
}

// TODO: Where to put this?
enum Operation {
    Add,
    Sub,
    Div,
    Mul,
}

impl Operation {
    pub fn to_tokens(&self, winrt_struct: &Struct) -> TokenStream {
        let struct_name = &winrt_struct.name.tokens;
        let (op_name, op_fn, symbol) = self.get_name_and_symbol();
        let fields = winrt_struct.fields.iter().map(|field| {
            let name = format_ident(&field.0);
            quote! {
                #name: self.#name #symbol rhs.#name
            }
        });
        let permutations = vec![
            ( struct_name.clone(), struct_name.clone() ),
            ( struct_name.clone(), quote!{ &#struct_name }),
            ( quote!{ &#struct_name }, struct_name.clone() ),
            ( quote!{ &#struct_name }, quote!{ &#struct_name }),
        ];
        let versions = {
            let mut versions = Vec::new();
            for (lhs, rhs) in permutations {
                let fields = fields.clone();
                versions.push(quote! {
                    impl std::ops::#op_name<#rhs> for #lhs {
                        type Output = #struct_name;
                        fn #op_fn(self, rhs: #rhs) -> #struct_name {
                            #struct_name {
                                #(#fields),*
                            }
                        }
                    }
                });
            }
            match self {
                Operation::Div | Operation::Mul => {
                    let fields = winrt_struct.fields.iter().map(|field| {
                        let name = format_ident(&field.0);
                        quote! {
                            #name: self.#name #symbol rhs
                        }
                    });
                    let permutations = vec![struct_name.clone(), quote!{ &#struct_name }];
                    for impl_type in permutations {
                        let fields = fields.clone();
                        versions.push(quote! {
                            impl std::ops::#op_name<f32> for #impl_type {
                                type Output = #struct_name;
                                fn #op_fn(self, rhs: f32) -> #struct_name {
                                    #struct_name {
                                        #(#fields),*
                                    }
                                }
                            }
                        });
                    }
                },
                _ => (),
            }
            TokenStream::from_iter(versions)
        };
        versions
    }

    fn get_name_and_symbol(&self) -> (TokenStream, TokenStream, TokenStream) {
        match self {
            Operation::Add => (quote!{Add}, quote!{add}, quote!{+}),
            Operation::Sub => (quote!{Sub}, quote!{sub}, quote!{-}),
            Operation::Div => (quote!{Div}, quote!{div}, quote!{/}),
            Operation::Mul => (quote!{Mul}, quote!{mul}, quote!{*}),
        }
    }
}