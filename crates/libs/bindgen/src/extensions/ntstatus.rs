use super::*;

pub fn gen() -> TokenStream {
    quote! {
        impl NTSTATUS {
            #[inline]
            pub const fn is_ok(self) -> bool {
                self.0 >= 0
            }
            #[inline]
            pub const fn is_err(self) -> bool {
                !self.is_ok()
            }
            #[inline]
            pub const fn to_hresult(self) -> ::windows::core::HRESULT {
                ::windows::core::HRESULT(self.0 | 0x1000_0000)
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
        impl ::core::convert::From<NTSTATUS> for ::windows::core::HRESULT {
            fn from(value: NTSTATUS) -> Self {
                value.to_hresult()
            }
        }
        impl ::core::convert::From<NTSTATUS> for ::windows::core::Error {
            fn from(value: NTSTATUS) -> Self {
                Self{ code: value.to_hresult(), info: None }
            }
        }
    }
}
