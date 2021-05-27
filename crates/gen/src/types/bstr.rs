use super::*;

pub fn gen_bstr() -> TokenStream {
    quote! {
        #[repr(transparent)]
        #[derive(::std::cmp::Eq)]
        pub struct BSTR(*mut u16);
        impl BSTR {
            /// Create an empty `BSTR`.
            ///
            /// This function does not allocate memory.
            pub fn new() -> Self {
                Self(std::ptr::null_mut())
            }

            /// Returns `true` if the string is empty.
            pub fn is_empty(&self) -> bool {
                self.0.is_null()
            }

            /// Returns the length of the string.
            pub fn len(&self) -> usize {
                if self.is_empty() {
                    return 0;
                }

                unsafe { super::System::OleAutomation::SysStringLen(self) as usize }
            }

            /// Create a `BSTR` from a slice of 16-bit characters.
            pub fn from_wide(value: &[u16]) -> Self {
                if value.len() == 0 {
                    return Self(::std::ptr::null_mut());
                }

                unsafe {
                    super::System::OleAutomation::SysAllocStringLen(
                        PWSTR(value.as_ptr() as _),
                        value.len() as u32,
                    )
                }
            }

            /// Get the string as 16-bit characters.
            pub fn as_wide(&self) -> &[u16] {
                if self.0.is_null() {
                    return &[];
                }

                unsafe { ::std::slice::from_raw_parts(self.0 as *const u16, self.len()) }
            }
        }
        impl ::std::clone::Clone for BSTR {
            fn clone(&self) -> Self {
                Self::from_wide(self.as_wide())
            }
        }
        impl ::std::convert::From<&str> for BSTR {
            fn from(value: &str) -> Self {
                let value: ::std::vec::Vec<u16> = value.encode_utf16().collect();
                Self::from_wide(&value)
            }
        }

        impl ::std::convert::From<::std::string::String> for BSTR {
            fn from(value: ::std::string::String) -> Self {
                value.as_str().into()
            }
        }

        impl  ::std::convert::From<&::std::string::String> for BSTR {
            fn from(value: &::std::string::String) -> Self {
                value.as_str().into()
            }
        }
        impl<'a> ::std::convert::TryFrom<&'a BSTR> for ::std::string::String {
            type Error = ::std::string::FromUtf16Error;

            fn try_from(value: &BSTR) -> ::std::result::Result<Self, Self::Error> {
                ::std::string::String::from_utf16(value.as_wide())
            }
        }

        impl ::std::convert::TryFrom<BSTR> for ::std::string::String {
            type Error = ::std::string::FromUtf16Error;

            fn try_from(value: BSTR) -> ::std::result::Result<Self, Self::Error> {
                ::std::string::String::try_from(&value)
            }
        }
        impl ::std::default::Default for BSTR {
            fn default() -> Self {
                Self(::std::ptr::null_mut())
            }
        }
        impl ::std::fmt::Display for BSTR {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use ::std::fmt::Write;
                for c in ::std::char::decode_utf16(self.as_wide().iter().cloned()) {
                    f.write_char(c.map_err(|_| ::std::fmt::Error)?)?
                }
                Ok(())
            }
        }
        impl ::std::fmt::Debug for BSTR {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::write!(f, "{}", self)
            }
        }
        impl ::std::cmp::PartialEq for BSTR {
            fn eq(&self, other: &Self) -> bool {
                self.as_wide() == other.as_wide()
            }
        }
        impl ::std::cmp::PartialEq<::std::string::String> for BSTR {
            fn eq(&self, other: &::std::string::String) -> bool {
                self == other.as_str()
            }
        }
        impl ::std::cmp::PartialEq<str> for BSTR {
            fn eq(&self, other: &str) -> bool {
                self == other
            }
        }
        impl ::std::cmp::PartialEq<&str> for BSTR {
            fn eq(&self, other: &&str) -> bool {
                self.as_wide().iter().copied().eq(other.encode_utf16())
            }
        }

        impl ::std::cmp::PartialEq<BSTR> for &str {
            fn eq(&self, other: &BSTR) -> bool {
                other == self
            }
        }
        impl ::std::ops::Drop for BSTR {
            fn drop(&mut self) {
                if !self.0.is_null() {
                    unsafe { super::System::OleAutomation::SysFreeString(self as &Self) }
                }
            }
        }
        unsafe impl ::windows::Abi for BSTR {
            type Abi = *mut u16;

            fn set_abi(&mut self) -> *mut *mut u16 {
                debug_assert!(self.0.is_null());
                &mut self.0 as *mut _ as _
            }
        }
        pub type BSTR_abi = *mut u16;
    }
}
