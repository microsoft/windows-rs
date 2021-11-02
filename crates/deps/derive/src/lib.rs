use quote::*;

#[proc_macro_derive(DeriveInterface)]
pub fn derive_interface(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let name = format_token!("{}", syn::parse_macro_input!(input as syn::DeriveInput).ident.to_string());
    // TODO: grab the struct's field and see whether its IUnknown, IInspectable, or something else

    let tokens = quote! {
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

    tokens.as_str().parse().unwrap()
}
