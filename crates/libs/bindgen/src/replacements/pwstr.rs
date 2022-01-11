use super::*;

// TODO: move to windows::core
pub fn gen() -> TokenStream {
    quote! {
        #[repr(transparent)]
        pub struct PWSTR(pub *mut u16);
        impl PWSTR {
            pub fn is_null(&self) -> bool {
                self.0.is_null()
            }
        }
        impl ::core::default::Default for PWSTR {
            fn default() -> Self {
                Self(::core::ptr::null_mut())
            }
        }
        impl ::core::clone::Clone for PWSTR {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl ::core::marker::Copy for PWSTR {}
        impl ::core::cmp::PartialEq for PWSTR {
            fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
            }
        }
        impl ::core::cmp::Eq for PWSTR {}
        impl ::core::fmt::Debug for PWSTR {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple("PWSTR").field(&self.0).finish()
            }
        }
        unsafe impl ::windows::core::Abi for PWSTR {
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
        impl<'a> ::windows::core::IntoParam<'a, PWSTR> for &str {
            fn into_param(self) -> ::windows::core::Param<'a, PWSTR> {
                ::windows::core::Param::Boxed(PWSTR(::windows::core::alloc::boxed::Box::<[u16]>::into_raw(self.encode_utf16().chain(::core::iter::once(0)).collect::<::windows::core::alloc::vec::Vec<u16>>().into_boxed_slice()) as _))
            }
        }
        #[cfg(feature = "alloc")]
        impl<'a> ::windows::core::IntoParam<'a, PWSTR> for ::windows::core::alloc::string::String {
            fn into_param(self) -> ::windows::core::Param<'a, PWSTR> {
                ::windows::core::IntoParam::into_param(self.as_str())
            }
        }
        impl<'a> ::windows::core::IntoParam<'a, PWSTR> for &::std::ffi::OsStr {
            fn into_param(self) -> ::windows::core::Param<'a, PWSTR> {
                use ::std::os::windows::ffi::OsStrExt;
                ::windows::core::Param::Boxed(PWSTR(::windows::core::alloc::boxed::Box::<[u16]>::into_raw(self.encode_wide().chain(::core::iter::once(0)).collect::<::windows::core::alloc::vec::Vec<u16>>().into_boxed_slice()) as _))
            }
        }
        impl<'a> ::windows::core::IntoParam<'a, PWSTR> for ::std::ffi::OsString {
            fn into_param(self) -> ::windows::core::Param<'a, PWSTR> {
                ::windows::core::IntoParam::into_param(self.as_os_str())
            }
        }
    }
}
