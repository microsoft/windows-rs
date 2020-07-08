use proc_macro2::TokenStream;
use quote::quote;

pub fn to_object_tokens(from: &TokenStream, constraints: &TokenStream) -> TokenStream {
    quote! {
        impl<#constraints> ::std::convert::From<#from> for ::winrt::Object {
            fn from(value: #from) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl<#constraints> ::std::convert::From<&#from> for ::winrt::Object {
            fn from(value: &#from) -> Self {
                ::std::convert::From::from(::std::clone::Clone::clone(value))
            }
        }
    }
}
