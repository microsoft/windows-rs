use super::*;

pub fn gen_unknown(name: &TokenStream) -> TokenStream {
    quote! {
        impl ::core::convert::From<#name> for ::windows::core::IUnknown {
            fn from(value: #name) -> Self {
                value.0
            }
        }
        impl ::core::convert::From<&#name> for ::windows::core::IUnknown {
            fn from(value: &#name) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for #name {
            fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
                ::windows::core::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a #name {
            fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
                ::windows::core::Param::Borrowed(&self.0)
            }
        }
    }
}

pub fn gen_inspectable(name: &TokenStream, constraints: &TokenStream, features: &TokenStream) -> TokenStream {
    quote! {
        #features
        impl<#constraints> ::core::convert::From<#name> for ::windows::core::IUnknown {
            fn from(value: #name) -> Self {
                value.0.0
            }
        }
        #features
        impl<#constraints> ::core::convert::From<&#name> for ::windows::core::IUnknown {
            fn from(value: &#name) -> Self {
                value.0.0.clone()
            }
        }
        #features
        impl<'a, #constraints> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for #name {
            fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
                ::windows::core::Param::Owned(self.0.0)
            }
        }
        #features
        impl<'a, #constraints> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a #name {
            fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
                ::windows::core::Param::Borrowed(&self.0.0)
            }
        }

        #features
        impl<#constraints> ::core::convert::From<#name> for ::windows::core::IInspectable {
            fn from(value: #name) -> Self {
                value.0
            }
        }
        #features
        impl<#constraints> ::core::convert::From<&#name> for ::windows::core::IInspectable {
            fn from(value: &#name) -> Self {
                value.0.clone()
            }
        }
        #features
        impl<'a, #constraints> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for #name {
            fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
                ::windows::core::Param::Owned(self.0)
            }
        }
        #features
        impl<'a, #constraints> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a #name {
            fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
                ::windows::core::Param::Borrowed(&self.0)
            }
        }
    }
}
