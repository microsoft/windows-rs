use super::*;

pub fn gen_bool32() -> TokenStream {
    quote! {
        #[repr(transparent)]
        #[derive(::std::default::Default, ::std::clone::Clone, ::std::marker::Copy, ::std::cmp::PartialEq, ::std::cmp::Eq, ::std::fmt::Debug)]
        pub struct BOOL(pub i32);

        unsafe impl ::windows::runtime::Abi for BOOL {
            type Abi = Self;
        }
        impl BOOL {
            #[inline]
            pub fn as_bool(self) -> bool {
                !(self.0 == 0)
            }

            #[inline]
            pub fn ok(self) -> ::windows::runtime::Result<()> {
                if self.as_bool() {
                    Ok(())
                } else {
                    Err(::windows::runtime::Error::from_win32())
                }
            }

            #[inline]
            #[track_caller]
            pub fn unwrap(self) {
                self.ok().unwrap();
            }

            #[inline]
            #[track_caller]
            pub fn expect(self, msg: &str) {
                self.ok().expect(msg);
            }
        }

        impl ::std::convert::From<BOOL> for bool {
            fn from(value: BOOL) -> Self {
                value.as_bool()
            }
        }

        impl ::std::convert::From<&BOOL> for bool {
            fn from(value: &BOOL) -> Self {
                value.as_bool()
            }
        }

        impl ::std::convert::From<bool> for BOOL {
            fn from(value: bool) -> Self {
                if value {
                    BOOL(1)
                } else {
                    BOOL(0)
                }
            }
        }

        impl ::std::convert::From<&bool> for BOOL {
            fn from(value: &bool) -> Self {
                (*value).into()
            }
        }

        impl ::std::cmp::PartialEq<bool> for BOOL {
            fn eq(&self, other: &bool) -> bool {
                self.as_bool() == *other
            }
        }

        impl ::std::cmp::PartialEq<BOOL> for bool {
            fn eq(&self, other: &BOOL) -> bool {
                *self == other.as_bool()
            }
        }

        impl std::ops::Not for BOOL {
            type Output = Self;
            fn not(self) -> Self::Output {
                if self.as_bool() {
                    BOOL(0)
                } else {
                    BOOL(1)
                }
            }
        }

        impl<'a> ::windows::runtime::IntoParam<'a, BOOL> for bool {
            fn into_param(self) -> ::windows::runtime::Param<'a, BOOL> {
                ::windows::runtime::Param::Owned(self.into())
            }
        }
    }
}
