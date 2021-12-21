use super::*;

pub fn gen() -> TokenStream {
    quote! {
        #[repr(transparent)]
        pub struct NTSTATUS(pub i32);
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
            pub fn ok(self) -> ::windows::core::Result<()> {
                if self.is_ok() {
                    Ok(())
                } else {
                    Err(::windows::core::Error::fast_error(self.to_hresult()))
                }
            }
        }
        impl ::core::default::Default for NTSTATUS {
            fn default() -> Self {
                Self(0)
            }
        }
        impl ::core::clone::Clone for NTSTATUS {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl ::core::marker::Copy for NTSTATUS {}
        impl ::core::cmp::PartialEq for NTSTATUS {
            fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
            }
        }
        impl ::core::cmp::Eq for NTSTATUS {}
        impl ::core::fmt::Debug for NTSTATUS {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple("NTSTATUS").field(&self.0).finish()
            }
        }
        unsafe impl ::windows::core::Abi for NTSTATUS {
            type Abi = Self;
        }
    }
}
