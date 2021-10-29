use super::*;

pub fn gen_object(name: &TokenStream, constraints: &TokenStream, features: &TokenStream) -> TokenStream {
    quote! {
        #features
        impl<#constraints> ::std::convert::From<#name> for ::windows::runtime::IUnknown {
            fn from(value: #name) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        #features
        impl<#constraints> ::std::convert::From<&#name> for ::windows::runtime::IUnknown {
            fn from(value: &#name) -> Self {
                ::std::convert::From::from(::std::clone::Clone::clone(value))
            }
        }
        #features
        impl<'a, #constraints> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for #name {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
            }
        }
        #features
        impl<'a, #constraints> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &#name {
            fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
                ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
