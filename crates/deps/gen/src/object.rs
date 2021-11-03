use super::*;

pub fn gen_unknown(name: &TokenStream) -> TokenStream {
    quote! {
        impl ::std::convert::From<#name> for ::windows::runtime::IUnknown {
            fn from(value: #name) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&#name> for ::windows::runtime::IUnknown {
            fn from(value: &#name) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for #name {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                ::windows::runtime::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a #name {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                ::windows::runtime::Param::Borrowed(&self.0)
            }
        }
    }
}

pub fn gen_inspectable(name: &TokenStream, constraints: &TokenStream, features: &TokenStream) -> TokenStream {
    quote! {
        #features
        impl<#constraints> ::std::convert::From<#name> for ::windows::runtime::IUnknown {
            fn from(value: #name) -> Self {
                value.0.0
            }
        }
        #features
        impl<#constraints> ::std::convert::From<&#name> for ::windows::runtime::IUnknown {
            fn from(value: &#name) -> Self {
                value.0.0.clone()
            }
        }
        #features
        impl<'a, #constraints> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for #name {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                ::windows::runtime::Param::Owned(self.0.0)
            }
        }
        #features
        impl<'a, #constraints> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a #name {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                ::windows::runtime::Param::Borrowed(&self.0.0)
            }
        }

        #features
        impl<#constraints> ::std::convert::From<#name> for ::windows::runtime::IInspectable {
            fn from(value: #name) -> Self {
                value.0
            }
        }
        #features
        impl<#constraints> ::std::convert::From<&#name> for ::windows::runtime::IInspectable {
            fn from(value: &#name) -> Self {
                value.0.clone()
            }
        }
        #features
        impl<'a, #constraints> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for #name {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
                ::windows::runtime::Param::Owned(self.0)
            }
        }
        #features
        impl<'a, #constraints> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a #name {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
                ::windows::runtime::Param::Borrowed(&self.0)
            }
        }
    }
}
