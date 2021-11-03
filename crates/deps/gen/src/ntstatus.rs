use super::*;

pub fn gen_ntstatus() -> TokenStream {
    quote! {
        #[repr(transparent)]
        #[derive(::std::default::Default, ::std::clone::Clone, ::std::marker::Copy, ::std::cmp::PartialEq, ::std::cmp::Eq, ::std::fmt::Debug)]
        pub struct NTSTATUS(pub u32);

        impl NTSTATUS {
            #[inline]
            pub const fn is_ok(self) -> bool {
                self.0 & 0x8000_0000 == 0
            }

            #[inline]
            pub const fn is_err(self) -> bool {
                !self.is_ok()
            }

            #[inline]
            pub const fn to_hresult(self) -> ::windows::runtime::HRESULT {
                ::windows::runtime::HRESULT(self.0 | 0x1000_0000)
            }

            #[inline]
            pub fn ok(self) -> ::windows::runtime::Result<()> {
                if self.is_ok() {
                    Ok(())
                } else {
                    Err(::windows::runtime::Error::fast_error(self.to_hresult()))
                }
            }
        }

        unsafe impl ::windows::runtime::Abi for NTSTATUS {
            type Abi = Self;
        }
    }
}
