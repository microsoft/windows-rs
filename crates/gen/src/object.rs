use super::*;

pub fn gen_object(name: &TokenStream, constraints: &TokenStream) -> TokenStream {
    quote! {
        impl<#constraints> ::std::convert::From<#name> for ::windows::IInspectable {
            fn from(value: #name) -> Self {
                value.0
            }
        }
        impl<#constraints> ::std::convert::From<&#name> for ::windows::IInspectable {
            fn from(value: &#name) -> Self {
                value.0.clone()
            }
        }

        impl<'a, #constraints> ::windows::IntoParam<'a, ::windows::IInspectable> for #name {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a, #constraints> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a #name {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
    }
}
