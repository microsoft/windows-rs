use crate::format_ident;
use crate::types::*;
use squote::{quote, TokenStream};
use std::iter::FromIterator;

pub fn generate_struct_extensions(winrt_struct: &Struct) -> TokenStream {
    match (
        &winrt_struct.name.namespace as &str,
        &winrt_struct.name.name as &str,
    ) {
        ("Windows.Foundation.Numerics", "Vector2")
        | ("Windows.Foundation.Numerics", "Vector3")
        | ("Windows.Foundation.Numerics", "Vector4") => {
            let ops = [
                Operation::Add,
                Operation::Sub,
                Operation::Div,
                Operation::Mul,
            ];
            let impls = ops.iter().map(|op| {
                let impl_with_same_type = op.to_tokens_with_same_type(winrt_struct);
                let impl_with_scalar = match op {
                    Operation::Div | Operation::Mul => op.to_tokens_with_scalar(winrt_struct),
                    _ => quote! {},
                };
                quote! {
                    #impl_with_same_type
                    #impl_with_scalar
                }
            });
            //TokenStream::from_iter(impls)
            let vector_functions = to_tokens_vector_functions(winrt_struct);
            println!("{}", vector_functions.to_string());
            quote! {
                #(#impls)*
                #vector_functions
            }
        }
        ("Windows.Foundation.Numerics", "Matrix3x2")
        | ("Windows.Foundation.Numerics", "Matrix4x4") => {
            let ops = [Operation::Add, Operation::Sub];
            let impl_with_same_type = ops.iter().map(|op| op.to_tokens_with_same_type(winrt_struct));
            let impl_with_scalar = Operation::Mul.to_tokens_with_scalar(winrt_struct);
            let rows = match &winrt_struct.name.name as &str {
                "Matrix3x2" => {
                    quote! {
                        m11: self.m11 * rhs.m11 + self.m12 * rhs.m21,
                        m12: self.m11 * rhs.m12 + self.m12 * rhs.m22,

                        m21: self.m21 * rhs.m11 + self.m22 * rhs.m21,
                        m22: self.m21 * rhs.m12 + self.m22 * rhs.m22,

                        m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + rhs.m31,
                        m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + rhs.m32,
                    }
                }
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
                }
                _ => panic!(),
            };
            let impl_custom = Operation::Mul.to_tokens_with_same_type_custom(winrt_struct, &rows);
            quote! {
                #(#impl_with_same_type)*
                #impl_with_scalar
                #impl_custom
            }
        }
        _ => {
            quote! {}
        }
    }
}

enum Operation {
    Add,
    Sub,
    Div,
    Mul,
}

impl Operation {
    pub fn to_tokens_with_same_type(&self, winrt_struct: &Struct) -> TokenStream {
        let struct_name = &winrt_struct.name.tokens;
        let (op_name, op_fn, symbol) = self.get_name_fn_and_symbol();
        let fields = winrt_struct.fields.iter().map(|field| {
            let name = format_ident(&field.0);
            quote! {
                #name: self.#name #symbol rhs.#name
            }
        });
        let fields = &quote! { #(#fields),* };
        let permutations = Self::get_same_type_permutations(struct_name);
        let impls = permutations.iter().map(|(lhs, rhs)| {
            quote! {
                impl ::std::ops::#op_name<#rhs> for #lhs {
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
        let fields = &quote! { #(#fields),* };
        let permutations = vec![struct_name.clone(), quote! { &#struct_name }];
        let impls = permutations.iter().map(|lhs| {
            quote! {
                impl ::std::ops::#op_name<f32> for #lhs {
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

    pub fn to_tokens_with_same_type_custom(
        &self,
        winrt_struct: &Struct,
        custom_fields: &TokenStream,
    ) -> TokenStream {
        let struct_name = &winrt_struct.name.tokens;
        let (op_name, op_fn, _) = self.get_name_fn_and_symbol();
        let permutations = Self::get_same_type_permutations(struct_name);
        let impls = permutations.iter().map(|(lhs, rhs)| {
            quote! {
                impl ::std::ops::#op_name<#rhs> for #lhs {
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

    fn get_same_type_permutations(struct_name: &TokenStream) -> Vec<(TokenStream, TokenStream)> {
        vec![
            (struct_name.clone(), struct_name.clone()),
            (struct_name.clone(), quote! { &#struct_name }),
            (quote! { &#struct_name }, struct_name.clone()),
            (quote! { &#struct_name }, quote! { &#struct_name }),
        ]
    }

    fn get_name_fn_and_symbol(&self) -> (TokenStream, TokenStream, TokenStream) {
        match self {
            Operation::Add => (quote! {Add}, quote! {add}, quote! {+}),
            Operation::Sub => (quote! {Sub}, quote! {sub}, quote! {-}),
            Operation::Div => (quote! {Div}, quote! {div}, quote! {/}),
            Operation::Mul => (quote! {Mul}, quote! {mul}, quote! {*}),
        }
    }
}

fn to_tokens_vector_functions(winrt_struct: &Struct) -> TokenStream {
    let struct_name = &winrt_struct.name.tokens;
    let fields_zero = field_assignment_to_tokens(winrt_struct, 0.0);
    let fields_one = field_assignment_to_tokens(winrt_struct, 1.0);

    let unit_functions = {
        let unit_functions = winrt_struct.fields.iter().map(|field| {
            let name_string = &field.0;
            let unit_fn_name = format_ident(&format!("unit_{}", field.0));

            let fields = winrt_struct.fields.iter().map(|field| {
                let name = format_ident(&field.0);
                let value = if &field.0 == name_string {
                    quote!{ 1.0 }
                } else {
                    quote!{ 0.0 }
                };
                quote! {
                    #name: #value
                }
            });

            quote! {
                pub fn #unit_fn_name() -> Self {
                    Self {
                        #(#fields),*
                    }
                }
            }
        });
        TokenStream::from_iter(unit_functions)
    };

    let dot = {
        let fields = winrt_struct.fields.iter().map(|field| {
            let name = format_ident(&field.0);
            quote! {
                self.#name * rhs.#name
            }
        });
        quote! {
            pub fn dot(&self, rhs: &#struct_name) -> f32 {
                #(#fields)+*
            }
        }
    };

    quote! {
        impl #struct_name {
            pub fn zero() -> Self {
                Self {
                    #fields_zero
                }
            }
            pub fn one() -> Self {
                Self {
                    #fields_one
                }
            }
            #unit_functions
            #dot
            pub fn length_squared(&self) -> f32 {
                self.dot(self)
            }
            pub fn length(&self) -> f32 {
                self.length_squared().sqrt()
            }
            pub fn distance(&self, value: &#struct_name) -> f32 {
                (self - value).length()
            }
            pub fn distance_squared(&self, value: &#struct_name) -> f32 {
                (self - value).length_squared()
            }
            pub fn normalize(&self) -> Self {
                self / self.length()
            }
        }
    }
}

fn field_assignment_to_tokens(winrt_struct: &Struct, value: f32) -> TokenStream {
    let value = quote! { #value };
    let fields = winrt_struct.fields.iter().map(|field| {
        let name = format_ident(&field.0);
        quote! {
            #name: #value,
        }
    });
    TokenStream::from_iter(fields)
}