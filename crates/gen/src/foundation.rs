use super::*;

// Injects code into the Windows.Foundation namespace.
pub fn gen_foundation(namespace: &str) -> TokenStream {
    if namespace != "Windows.Foundation" {
        return TokenStream::new();
    }

    quote! {
        impl ::std::convert::From<::std::time::Duration> for TimeSpan {
            fn from(value: ::std::time::Duration) -> Self {
                Self {
                    duration: (value.as_nanos() / 100) as i64,
                }
            }
        }

        impl ::std::convert::From<TimeSpan> for ::std::time::Duration {
            fn from(value: TimeSpan) -> Self {
                ::std::time::Duration::from_nanos((value.duration * 100) as u64)
            }
        }

        impl<'a> ::windows::IntoParam<'a, TimeSpan> for ::std::time::Duration {
            fn into_param(self) -> ::windows::Param<'a, TimeSpan> {
                ::windows::Param::Owned(self.into())
            }
        }
    }
}
