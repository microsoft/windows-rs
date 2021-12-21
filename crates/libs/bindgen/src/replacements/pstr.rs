use super::*;

// TODO: move to windows::core
pub fn gen() -> TokenStream {
    quote! {
        #[repr(transparent)]
        pub struct PSTR(pub *mut u8);
        impl PSTR {
            pub fn is_null(&self) -> bool {
                self.0.is_null()
            }
        }
        impl ::core::default::Default for PSTR {
            fn default() -> Self {
                Self(::core::ptr::null_mut())
            }
        }
        impl ::core::clone::Clone for PSTR {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl ::core::marker::Copy for PSTR {}
        impl ::core::cmp::PartialEq for PSTR {
            fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
            }
        }
        impl ::core::cmp::Eq for PSTR {}
        impl ::core::fmt::Debug for PSTR {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple("PSTR").field(&self.0).finish()
            }
        }
        unsafe impl ::windows::core::Abi for PSTR {
            type Abi = Self;

            #[cfg(feature = "alloc")]
            unsafe fn drop_param(param: &mut ::windows::core::Param<'_, Self>) {
                if let ::windows::core::Param::Boxed(value) = param {
                    if !value.is_null() {
                        ::windows::core::alloc::boxed::Box::from_raw(value.0);
                    }
                }
            }
        }
        #[cfg(feature = "alloc")]
        impl<'a> ::windows::core::IntoParam<'a, PSTR> for &str {
            fn into_param(self) -> ::windows::core::Param<'a, PSTR> {
                ::windows::core::Param::Boxed(PSTR(::windows::core::alloc::boxed::Box::<[u8]>::into_raw(self.bytes().chain(::core::iter::once(0)).collect::<::windows::core::alloc::vec::Vec<u8>>().into_boxed_slice()) as _))
            }
        }
        #[cfg(feature = "alloc")]
        impl<'a> ::windows::core::IntoParam<'a, PSTR> for ::windows::core::alloc::string::String {
            fn into_param(self) -> ::windows::core::Param<'a, PSTR> {
                ::windows::core::IntoParam::into_param(self.as_str())
            }
        }
    }
}
