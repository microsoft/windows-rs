use super::*;

pub fn gen_pwstr() -> TokenStream {
    quote! {
        #[repr(transparent)]
        #[derive(::std::clone::Clone, ::std::marker::Copy, ::std::cmp::Eq, ::std::fmt::Debug)]
        pub struct PWSTR(pub *mut u16);
        impl PWSTR {
            pub const NULL: Self = Self(::std::ptr::null_mut());
            pub fn is_null(&self) -> bool {
                self.0.is_null()
            }
        }
        impl ::std::default::Default for PWSTR {
            fn default() -> Self {
                Self(::std::ptr::null_mut())
            }
        }
        // TODO: impl Debug and Display to display value and PartialEq etc
        impl ::std::cmp::PartialEq for PWSTR {
            fn eq(&self, other: &Self) -> bool {
                // TODO: do value compare
                self.0 == other.0
            }
        }
        unsafe impl ::windows::Abi for PWSTR {
            type Abi = Self;
            type DefaultType = Self;

            fn drop_param(param: &mut ::windows::Param<'_, Self>) {
                if let ::windows::Param::Boxed(value) = param {
                    if !value.0.is_null() {
                        unsafe { ::std::boxed::Box::from_raw(value.0); }
                    }
                }
            }
        }
        impl ::windows::IntoParam<'static, PWSTR> for &str {
            fn into_param(self) -> ::windows::Param<'static, PWSTR> {
                ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(self.encode_utf16().chain(::std::iter::once(0)).collect::<std::vec::Vec<u16>>().into_boxed_slice()) as _))
            }
        }
        impl ::windows::IntoParam<'static, PWSTR> for String {
            fn into_param(self) -> ::windows::Param<'static, PWSTR> {
                // TODO: call variant above
                ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(self.encode_utf16().chain(::std::iter::once(0)).collect::<std::vec::Vec<u16>>().into_boxed_slice()) as _))
            }
        }
        #[cfg(windows)]
        impl ::windows::IntoParam<'static, PWSTR> for &::std::ffi::OsStr {
            fn into_param(self) -> ::windows::Param<'static, PWSTR> {
                use std::os::windows::ffi::OsStrExt;
                ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(self.encode_wide().chain(::std::iter::once(0)).collect::<std::vec::Vec<u16>>().into_boxed_slice()) as _))
            }
        }
        #[cfg(windows)]
        impl ::windows::IntoParam<'static, PWSTR> for ::std::ffi::OsString {
            fn into_param(self) -> ::windows::Param<'static, PWSTR> {
                use std::os::windows::ffi::OsStrExt;
                ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(self.encode_wide().chain(::std::iter::once(0)).collect::<std::vec::Vec<u16>>().into_boxed_slice()) as _))
            }
        }
    }
}
