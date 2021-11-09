use super::*;

pub fn gen_pwstr() -> TokenStream {
    quote! {
        #[derive(::core::clone::Clone, ::core::marker::Copy, ::core::fmt::Debug, ::core::cmp::PartialEq, ::core::cmp::Eq)]
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
        unsafe impl ::windows::runtime::Abi for PWSTR {
            type Abi = Self;

            #[cfg(feature = "std")]
            unsafe fn drop_param(param: &mut ::windows::runtime::Param<'_, Self>) {
                if let ::windows::runtime::Param::Boxed(value) = param {
                    if !value.is_null() {
                        unsafe { ::std::boxed::Box::from_raw(value.0); }
                    }
                }
            }
        }
        #[cfg(feature = "std")]
        impl<'a> ::windows::runtime::IntoParam<'a, PWSTR> for &str {
            fn into_param(self) -> ::windows::runtime::Param<'a, PWSTR> {
                ::windows::runtime::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(self.encode_utf16().chain(::core::iter::once(0)).collect::<std::vec::Vec<u16>>().into_boxed_slice()) as _))
            }
        }
        #[cfg(feature = "std")]
        impl<'a> ::windows::runtime::IntoParam<'a, PWSTR> for String {
            fn into_param(self) -> ::windows::runtime::Param<'a, PWSTR> {
                ::windows::runtime::IntoParam::into_param(self.as_str())
            }
        }
        #[cfg(all(windows, feature = "std"))]
        impl<'a> ::windows::runtime::IntoParam<'a, PWSTR> for &::std::ffi::OsStr {
            fn into_param(self) -> ::windows::runtime::Param<'a, PWSTR> {
                use ::std::os::windows::ffi::OsStrExt;
                ::windows::runtime::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(self.encode_wide().chain(::core::iter::once(0)).collect::<std::vec::Vec<u16>>().into_boxed_slice()) as _))
            }
        }
        #[cfg(all(windows, feature = "std"))]
        impl<'a> ::windows::runtime::IntoParam<'a, PWSTR> for ::std::ffi::OsString {
            fn into_param(self) -> ::windows::runtime::Param<'a, PWSTR> {
                ::windows::runtime::IntoParam::into_param(self.as_os_str())
            }
        }
    }
}
