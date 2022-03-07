use super::*;

pub fn gen() -> TokenStream {
    quote! {
        impl WIN32_ERROR {
            #[inline]
            pub const fn is_ok(self) -> bool {
                self.0 == 0
            }
            #[inline]
            pub const fn is_err(self) -> bool {
                !self.is_ok()
            }
            #[inline]
            pub const fn to_hresult(self) -> ::windows::core::HRESULT {
                ::windows::core::HRESULT(if self.0 == 0 { self.0 } else { (self.0 & 0x0000_FFFF) | (7 << 16) | 0x8000_0000 } as _)
            }
            #[inline]
            pub const fn ok(self) -> ::windows::core::Result<()> {
                if self.is_ok() {
                    Ok(())
                } else {
                    Err(::windows::core::Error{ code: self.to_hresult(), info: None })
                }
            }
        }
        impl ::core::convert::From<WIN32_ERROR> for ::windows::core::HRESULT {
            fn from(value: WIN32_ERROR) -> Self {
                value.to_hresult()
            }
        }
        impl ::core::convert::From<WIN32_ERROR> for ::windows::core::Error {
            fn from(value: WIN32_ERROR) -> Self {
                Self{ code: value.to_hresult(), info: None }
            }
        }
    }
}
