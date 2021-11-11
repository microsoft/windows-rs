use super::*;

pub fn gen_win32_error() -> TokenStream {
    quote! {
        impl ::core::convert::From<WIN32_ERROR> for ::windows::core::HRESULT {
            fn from(value: WIN32_ERROR) -> Self {
                Self(if value.0 as i32 <= 0 {
                    value.0
                } else {
                    (value.0 & 0x0000_FFFF) | (7 << 16) | 0x8000_0000
                })
            }
        }
    }
}
