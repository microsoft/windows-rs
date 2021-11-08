use super::*;

pub fn gen_bstr() -> TokenStream {
    quote! {
        #[repr(transparent)]
        #[derive(::core::cmp::Eq)]
        pub struct BSTR(pub *mut u16);
        impl BSTR {
            pub fn new() -> Self {
                Self(core::ptr::null_mut())
            }

            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }

            pub fn len(&self) -> usize {
                if self.0.is_null() {
                    0
                } else {
                    unsafe { SysStringLen(self) as usize }
                }
            }

            pub fn from_wide(value: &[u16]) -> Self {
                if value.len() == 0 {
                    return Self(::core::ptr::null_mut());
                }

                unsafe {
                    SysAllocStringLen(
                        PWSTR(value.as_ptr() as *mut _),
                        value.len() as u32,
                    )
                }
            }

            pub fn as_wide(&self) -> &[u16] {
                if self.0.is_null() {
                    return &[];
                }

                unsafe { ::core::slice::from_raw_parts(self.0 as *const u16, self.len()) }
            }
        }
        impl ::core::clone::Clone for BSTR {
            fn clone(&self) -> Self {
                Self::from_wide(self.as_wide())
            }
        }
        #[cfg(not(feature = "no_std"))]
        impl ::core::convert::From<&str> for BSTR {
            fn from(value: &str) -> Self {
                let value: ::std::vec::Vec<u16> = value.encode_utf16().collect();
                Self::from_wide(&value)
            }
        }
        #[cfg(not(feature = "no_std"))]
        impl ::core::convert::From<::std::string::String> for BSTR {
            fn from(value: ::std::string::String) -> Self {
                value.as_str().into()
            }
        }
        #[cfg(not(feature = "no_std"))]
        impl  ::core::convert::From<&::std::string::String> for BSTR {
            fn from(value: &::std::string::String) -> Self {
                value.as_str().into()
            }
        }
        #[cfg(not(feature = "no_std"))]
        impl<'a> ::core::convert::TryFrom<&'a BSTR> for ::std::string::String {
            type Error = ::std::string::FromUtf16Error;

            fn try_from(value: &BSTR) -> ::core::result::Result<Self, Self::Error> {
                ::std::string::String::from_utf16(value.as_wide())
            }
        }
        #[cfg(not(feature = "no_std"))]
        impl ::core::convert::TryFrom<BSTR> for ::std::string::String {
            type Error = ::std::string::FromUtf16Error;

            fn try_from(value: BSTR) -> ::core::result::Result<Self, Self::Error> {
                ::std::string::String::try_from(&value)
            }
        }
        impl ::core::default::Default for BSTR {
            fn default() -> Self {
                Self(::core::ptr::null_mut())
            }
        }
        #[cfg(not(feature = "no_std"))]
        impl ::core::fmt::Display for BSTR {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                use ::core::fmt::Write;
                for c in ::std::char::decode_utf16(self.as_wide().iter().cloned()) {
                    f.write_char(c.map_err(|_| ::core::fmt::Error)?)?
                }
                Ok(())
            }
        }
        impl ::core::fmt::Debug for BSTR {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::write!(f, "{}", self)
            }
        }
        impl ::core::cmp::PartialEq for BSTR {
            fn eq(&self, other: &Self) -> bool {
                self.as_wide() == other.as_wide()
            }
        }
        #[cfg(not(feature = "no_std"))]
        impl ::core::cmp::PartialEq<::std::string::String> for BSTR {
            fn eq(&self, other: &::std::string::String) -> bool {
                self == other.as_str()
            }
        }
        impl ::core::cmp::PartialEq<str> for BSTR {
            fn eq(&self, other: &str) -> bool {
                self == other
            }
        }
        impl ::core::cmp::PartialEq<&str> for BSTR {
            fn eq(&self, other: &&str) -> bool {
                self.as_wide().iter().copied().eq(other.encode_utf16())
            }
        }

        impl ::core::cmp::PartialEq<BSTR> for &str {
            fn eq(&self, other: &BSTR) -> bool {
                other == self
            }
        }
        impl ::core::ops::Drop for BSTR {
            fn drop(&mut self) {
                if !self.0.is_null() {
                    unsafe { SysFreeString(self as &Self) }
                }
            }
        }
        unsafe impl ::windows::runtime::Abi for BSTR {
            type Abi = ::core::mem::ManuallyDrop<Self>;
        }
        pub type BSTR_abi = *mut u16;
        impl<'a> ::windows::runtime::IntoParam<'a, BSTR> for &str {
            fn into_param(self) -> ::windows::runtime::Param<'a, BSTR> {
                ::windows::runtime::Param::Owned(self.into())
            }
        }
        impl<'a> ::windows::runtime::IntoParam<'a, BSTR> for String {
            fn into_param(self) -> ::windows::runtime::Param<'a, BSTR> {
                ::windows::runtime::Param::Owned(self.into())
            }
        }
    }
}
