use super::*;

pub fn gen_object(name: &TokenStream, constraints: &TokenStream) -> TokenStream {
    quote! {
        impl<#constraints> ::std::convert::From<#name> for ::windows::Object {
            fn from(value: #name) -> Self {
                value.0
            }
        }
        impl<#constraints> ::std::convert::From<&#name> for ::windows::Object {
            fn from(value: &#name) -> Self {
                ::std::convert::From::from(::std::clone::Clone::clone(value))
            }
        }

        impl<'a, #constraints> ::std::convert::Into<::windows::Param<'a, ::windows::Object>> for #name {
            fn into(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::Object>::into(self))
            }
        }
        impl<'a, #constraints> ::std::convert::Into<::windows::Param<'a, ::windows::Object>> for &'a #name {
            fn into(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::Object>::into(::std::clone::Clone::clone(self)))
            }
        }
    }
}
