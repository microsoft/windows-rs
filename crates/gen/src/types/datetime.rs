use super::*;

pub fn gen_datetime(runtime_type: TokenStream) -> TokenStream {
    quote! {
        #[repr(transparent)]
        #[derive(::std::clone::Clone, ::std::marker::Copy, ::std::cmp::PartialEq, ::std::cmp::Eq, ::std::fmt::Debug)]
        pub struct DateTime(::std::time::SystemTime);

        // SAFETY: relies on [`std::time::SystemTime`](std::time::SystemTime) wrapping `FILETIME` internally on Windows.
        // That means monitoring [implementation of `std::sys::SystemTime` on Windows](https://github.com/rust-lang/rust/blob/master/library/std/src/sys/windows/time.rs) for any future change.
        unsafe impl ::windows::Abi for DateTime {
            type Abi = Self;
        }
        #runtime_type
        impl ::std::convert::From<::std::time::SystemTime> for DateTime {
            fn from(value: ::std::time::SystemTime) -> Self {
                Self(value)
            }
        }
        impl ::std::convert::From<DateTime> for ::std::time::SystemTime {
            fn from(value: DateTime) -> Self {
                value.0
            }
        }
        impl<'a> ::windows::IntoParam<'a, DateTime> for ::std::time::SystemTime {
            fn into_param(self) -> ::windows::Param<'a, DateTime> {
                ::windows::Param::Owned(self.into())
            }
        }
    }
}
