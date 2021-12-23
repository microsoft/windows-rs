use super::*;

pub fn gen() -> TokenStream {
    quote! {
        #[repr(transparent)]
        pub struct HANDLE(pub isize);
        impl HANDLE {
            pub fn is_invalid(&self) -> bool {
                self.0 == 0 || self.0 == -1
            }

            pub fn ok(self) -> ::windows::core::Result<Self> {
                if self.is_invalid() {
                    Err(::windows::core::Error::from_win32())
                } else {
                    Ok(self)
                }
            }
        }
        impl ::core::default::Default for HANDLE {
            fn default() -> Self {
                Self(0)
            }
        }
        impl ::core::clone::Clone for HANDLE {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl ::core::marker::Copy for HANDLE {}
        impl ::core::cmp::PartialEq for HANDLE {
            fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
            }
        }
        impl ::core::cmp::Eq for HANDLE {}
        impl ::core::fmt::Debug for HANDLE {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple("HANDLE").field(&self.0).finish()
            }
        }
        unsafe impl ::windows::core::Abi for HANDLE {
            type Abi = Self;
        }
    }
}
