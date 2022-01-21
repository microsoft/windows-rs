use super::*;

pub fn gen() -> TokenStream {
    quote! {
        impl ::core::convert::From<WIN32_ERROR> for ::windows::core::HRESULT {
            fn from(value: WIN32_ERROR) -> Self {
                Self::from_win32(value.0)
            }
        }
    }
}
