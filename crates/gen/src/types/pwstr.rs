use super::*;

pub fn gen_pwstr() -> TokenStream {
    quote! {
        #[repr(transparent)]
        #[derive(::std::clone::Clone, ::std::marker::Copy, ::std::cmp::Eq)]
        /// Uses [`::windows::widestring::UCStr`] for null-terminated, checked strings.
        pub struct PWSTR(pub *mut ::windows::widestring::WideChar);
        impl PWSTR {
            pub const NULL: Self = Self(::std::ptr::null_mut());
            pub fn is_null(&self) -> bool {
                self.0.is_null()
            }
            pub fn is_empty(&self) -> bool {
                // TODO: Should possibly also check length!
                self.is_null()
            }
            pub fn len(&self) -> usize {
                self.as_wide().len()
            }
            fn as_wide(&self) -> &::windows::widestring::WideCStr {
                if self.is_null() {
                    Default::default()
                } else {
                    unsafe { ::windows::widestring::WideCStr::from_ptr_str(self.0) }
                }
            }
        }

        impl ::std::default::Default for PWSTR {
            fn default() -> Self {
                Self(::std::ptr::null_mut())
            }
        }

        impl ::std::fmt::Display for PWSTR {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(&self.as_wide().to_string().unwrap())
            }
        }
        impl ::std::fmt::Debug for PWSTR {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::write!(f, "{}", self)
            }
        }

        impl ::std::cmp::PartialEq for PWSTR {
            fn eq(&self, other: &Self) -> bool {
                self.as_wide().eq(other.as_wide())
            }
        }
        impl ::std::cmp::PartialEq<&str> for PWSTR {
            fn eq(&self, other: &&str) -> bool {
                let other = unsafe { ::windows::widestring::WideCString::from_str_unchecked(other) };
                self.as_wide().eq(&other)
            }
        }
        impl ::std::cmp::PartialEq<PWSTR> for &str {
            fn eq(&self, other: &PWSTR) -> bool {
                other.eq(self)
            }
        }

        unsafe impl ::windows::Abi for PWSTR {
            type Abi = Self;

            fn drop_param(param: &mut ::windows::Param<Self>) {
                if let ::windows::Param::Boxed(value) = param {
                    if !value.is_null() {
                        unsafe { ::windows::widestring::WideCString::from_raw(value.0) };
                    }
                }
            }
        }
        impl<'a> ::windows::IntoParam<'a, PWSTR> for &'a str {
            fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                ::windows::Param::Boxed(PWSTR(
                    ::windows::widestring::WideCString::from_str(self)
                        .unwrap()
                        .into_raw(),
                ))
            }
        }
        impl<'a> ::windows::IntoParam<'a, PWSTR> for String {
            fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                // TODO: call variant above
                ::windows::Param::Boxed(PWSTR(
                    ::windows::widestring::WideCString::from_str(self)
                        .unwrap()
                        .into_raw(),
                ))
            }
        }
    }
}
