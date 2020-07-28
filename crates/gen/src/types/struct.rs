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
                let impls = ops.iter().map(|op| {
                    let impl_with_self = op.to_tokens_with_self(self);
                    let impl_with_scalar = match op {
                        Operation::Div | Operation::Mul => op.to_tokens_with_scalar(self),
                        _ => quote!{}
                    };
                    quote! { 
                        #impl_with_self
                        #impl_with_scalar
                    }
                });
                TokenStream::from_iter(impls)
            }
            ("Windows.Foundation.Numerics", "Matrix3x2")
                | ("Windows.Foundation.Numerics", "Matrix4x4")
            => {
                let ops = [Operation::Add, Operation::Sub];
                let impl_with_self = ops.iter().map(|op| {
                    op.to_tokens_with_self(self)
                });
                let rows = match &self.name.name as &str {
                    "Matrix3x2" => {
                        quote! {
                            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21,
                            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22,

                            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21,
                            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22,

                            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + rhs.m31,
                            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + rhs.m32,
                        }
                    },
                    "Matrix4x4" => {
                        quote! {
                            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31 + self.m14 * rhs.m41,
                            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32 + self.m14 * rhs.m42,
                            m13: self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33 + self.m14 * rhs.m43,
                            m14: self.m11 * rhs.m14 + self.m12 * rhs.m24 + self.m13 * rhs.m34 + self.m14 * rhs.m44,

                            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31 + self.m24 * rhs.m41,
                            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32 + self.m24 * rhs.m42,
                            m23: self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33 + self.m24 * rhs.m43,
                            m24: self.m21 * rhs.m14 + self.m22 * rhs.m24 + self.m23 * rhs.m34 + self.m24 * rhs.m44,

                            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31 + self.m34 * rhs.m41,
                            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32 + self.m34 * rhs.m42,
                            m33: self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33 + self.m34 * rhs.m43,
                            m34: self.m31 * rhs.m14 + self.m32 * rhs.m24 + self.m33 * rhs.m34 + self.m34 * rhs.m44,

                            m41: self.m41 * rhs.m11 + self.m42 * rhs.m21 + self.m43 * rhs.m31 + self.m44 * rhs.m41,
                            m42: self.m41 * rhs.m12 + self.m42 * rhs.m22 + self.m43 * rhs.m32 + self.m44 * rhs.m42,
                            m43: self.m41 * rhs.m13 + self.m42 * rhs.m23 + self.m43 * rhs.m33 + self.m44 * rhs.m43,
                            m44: self.m41 * rhs.m14 + self.m42 * rhs.m24 + self.m43 * rhs.m34 + self.m44 * rhs.m44,
                        }
                    },
                    _ => panic!(),
                };
                let op = Operation::Mul;
                let impl_custom = op.to_tokes_with_self_custom(self, &rows);
                quote!{
                    #(#impl_with_self)*
                    #impl_custom
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
    pub fn to_tokens_with_self(&self, winrt_struct: &Struct) -> TokenStream {
        let struct_name = &winrt_struct.name.tokens;
        let (op_name, op_fn, symbol) = self.get_name_fn_and_symbol();
        let fields = winrt_struct.fields.iter().map(|field| {
            let name = format_ident(&field.0);
            quote! {
                #name: self.#name #symbol rhs.#name
            }
        });
        let fields = &quote!{ #(#fields),* };
        let permutations = Self::get_self_permutations(struct_name);
        let impls = permutations.iter().map(|(lhs, rhs)| {
            quote! {
                impl std::ops::#op_name<#rhs> for #lhs {
                    type Output = #struct_name;
                    fn #op_fn(self, rhs: #rhs) -> #struct_name {
                        #struct_name {
                            #fields
                        }
                    }
                }
            }
        });
        TokenStream::from_iter(impls)
    }

    pub fn to_tokens_with_scalar(&self, winrt_struct: &Struct) -> TokenStream {
        let struct_name = &winrt_struct.name.tokens;
        let (op_name, op_fn, symbol) = self.get_name_fn_and_symbol();
        let fields = winrt_struct.fields.iter().map(|field| {
            let name = format_ident(&field.0);
            quote! {
                #name: self.#name #symbol rhs
            }
        });
        let fields = &quote!{ #(#fields),* };
        let permutations = vec![
            struct_name.clone(), quote!{ &#struct_name },
        ];
        let impls = permutations.iter().map(|lhs| {
            quote! {
                impl std::ops::#op_name<f32> for #lhs {
                    type Output = #struct_name;
                    fn #op_fn(self, rhs: f32) -> #struct_name {
                        #struct_name {
                            #fields
                        }
                    }
                }
            }
        });
        TokenStream::from_iter(impls)
    }

    pub fn to_tokes_with_self_custom(&self, winrt_struct: &Struct, custom_fields: &TokenStream) -> TokenStream {
        let struct_name = &winrt_struct.name.tokens;
        let (op_name, op_fn, _) = self.get_name_fn_and_symbol();
        let permutations = Self::get_self_permutations(struct_name);
        let impls = permutations.iter().map(|(lhs, rhs)| {
            quote! {
                impl std::ops::#op_name<#rhs> for #lhs {
                    type Output = #struct_name;
                    fn #op_fn(self, rhs: #rhs) -> #struct_name {
                        #struct_name {
                            #custom_fields
                        }
                    }
                }
            }
        });
        TokenStream::from_iter(impls)
    }

    fn get_self_permutations(struct_name: &TokenStream) -> Vec<(TokenStream, TokenStream)> {
        vec![
            ( struct_name.clone(), struct_name.clone() ),
            ( struct_name.clone(), quote!{ &#struct_name }),
            ( quote!{ &#struct_name }, struct_name.clone() ),
            ( quote!{ &#struct_name }, quote!{ &#struct_name }),
        ]
    }

    fn get_name_fn_and_symbol(&self) -> (TokenStream, TokenStream, TokenStream) {
        match self {
            Operation::Add => (quote!{Add}, quote!{add}, quote!{+}),
            Operation::Sub => (quote!{Sub}, quote!{sub}, quote!{-}),
            Operation::Div => (quote!{Div}, quote!{div}, quote!{/}),
            Operation::Mul => (quote!{Mul}, quote!{mul}, quote!{*}),
        }
    }
}