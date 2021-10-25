use super::*;

pub fn gen_timespan() -> TokenStream {
    quote! {
        impl ::std::convert::From<::std::time::Duration> for TimeSpan {
            fn from(value: ::std::time::Duration) -> Self {
                Self {
                    Duration: (value.as_nanos() / 100) as i64,
                }
            }
        }
        impl ::std::convert::From<TimeSpan> for ::std::time::Duration {
            fn from(value: TimeSpan) -> Self {
                ::std::time::Duration::from_nanos((value.Duration * 100) as u64)
            }
        }
        impl<'a> ::windows::runtime::IntoParam<'a, TimeSpan> for ::std::time::Duration {
            fn into_param(self) -> ::windows::runtime::Param<'a, TimeSpan> {
                ::windows::runtime::Param::Owned(self.into())
            }
        }
    }
}
