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
        unsafe impl ::windows::core::Abi for PWSTR {
            type Abi = Self;

            #[cfg(feature = "alloc")]
            unsafe fn drop_param(param: &mut ::windows::core::Param<'_, Self>) {
                if let ::windows::core::Param::Boxed(value) = param {
                    if !value.is_null() {
                        unsafe { ::windows::core::alloc::boxed::Box::from_raw(value.0); }
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
        #[cfg(feature = "std")]
        impl<'a> ::windows::core::IntoParam<'a, PWSTR> for &::std::ffi::OsStr {
            fn into_param(self) -> ::windows::core::Param<'a, PWSTR> {
                use ::std::os::windows::ffi::OsStrExt;
                ::windows::core::Param::Boxed(PWSTR(::windows::core::alloc::boxed::Box::<[u16]>::into_raw(self.encode_wide().chain(::core::iter::once(0)).collect::<::windows::core::alloc::vec::Vec<u16>>().into_boxed_slice()) as _))
            }
        }
        #[cfg(feature = "std")]
        impl<'a> ::windows::core::IntoParam<'a, PWSTR> for ::std::ffi::OsString {
            fn into_param(self) -> ::windows::core::Param<'a, PWSTR> {
                ::windows::core::IntoParam::into_param(self.as_os_str())
            }
        }
    }
}
