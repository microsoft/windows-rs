use quote::*;
use syn::*;

#[proc_macro_derive(DeriveInterface)]
pub fn derive_interface(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = format_token!("{}", input.ident.to_string());

    let mut base = None;

    if let Data::Struct(data) = input.data {
        if let Fields::Unnamed(fields) = data.fields {
            if let Some(field) = fields.unnamed.first() {
                if let Type::Path(path) = &field.ty {
                    if let Some(segment) = path.path.segments.last() {
                        base = Some(segment.ident.to_string());
                    }
                }
            }
        }
    }

    let mut tokens = quote! {
        impl ::std::convert::From<#name> for ::windows::runtime::IUnknown {
            fn from(value: #name) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl ::std::convert::From<&#name> for ::windows::runtime::IUnknown {
            fn from(value: &#name) -> Self {
                ::std::convert::From::from(::std::clone::Clone::clone(value))
            }
        }
        impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for #name {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
            }
        }
        impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &#name {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
            }
        }
    };

    if let Some(base) = base {
        if base == "IInspectable" {
            tokens.combine(&quote! {
                impl ::std::convert::From<#name> for ::windows::runtime::IInspectable {
                    fn from(value: #name) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&#name> for ::windows::runtime::IInspectable {
                    fn from(value: &#name) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for #name {
                    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
                        ::windows::runtime::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a #name {
                    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
                        ::windows::runtime::Param::Borrowed(&self.0)
                    }
                }
            })
        }
    }

    tokens.as_str().parse().unwrap()
}
