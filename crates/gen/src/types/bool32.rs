use super::*;

pub fn gen_bool32() -> TokenStream {
    quote! {
        #[repr(C)]
        #[derive(::std::clone::Clone, ::std::marker::Copy, ::std::cmp::PartialEq, ::std::cmp::Eq, ::std::default::Default)]
        pub struct BOOL(pub i32);
        impl BOOL {
            #[inline]
            pub fn as_bool(self) -> bool {
                !(self.0 == 0)
            }
            #[inline]
            pub fn ok(self) -> ::windows::Result<()> {
                if self.as_bool() {
                    Ok(())
                } else {
                    Err(::windows::ErrorCode::from_thread().into())
                }
            }
            #[inline]
            pub fn unwrap(self) {
                self.ok().unwrap();
            }
            #[inline]
            pub fn expect(self, msg: &str) {
                self.ok().expect(msg);
            }
        }
        impl ::std::fmt::Debug for BOOL {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let msg = if self.as_bool() { "true" } else { "false" };
                fmt.write_str(msg)
            }
        }
        unsafe impl ::windows::Abi for BOOL {
            type Abi = Self;
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
        impl<'a> ::windows::IntoParam<'a, BOOL> for bool {
            fn into_param(self) -> ::windows::Param<'a, BOOL> {
                ::windows::Param::Owned(self.into())
            }
        }
    }
}
