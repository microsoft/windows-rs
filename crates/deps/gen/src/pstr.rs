use super::*;

pub fn gen_pstr() -> TokenStream {
    quote! {
        #[derive(::core::clone::Clone, ::core::marker::Copy, ::core::fmt::Debug, ::core::cmp::PartialEq, ::core::cmp::Eq)]
        #[repr(transparent)]
        pub struct PSTR(pub *mut u8);
        impl PSTR {
            pub fn is_null(&self) -> bool {
                self.0.is_null()
            }
        }
        impl ::core::default::Default for PSTR {
            fn default() -> Self {
                Self(::core::ptr::null_mut())
            }
        }
        unsafe impl ::windows::runtime::Abi for PSTR {
            type Abi = Self;

            #[cfg(feature = "std")]
            unsafe fn drop_param(param: &mut ::windows::runtime::Param<'_, Self>) {
                if let ::windows::runtime::Param::Boxed(value) = param {
                    if !value.is_null() {
                        unsafe { ::std::boxed::Box::from_raw(value.0); }
                    }
                }
            }
        }
        #[cfg(feature = "std")]
        impl<'a> ::windows::runtime::IntoParam<'a, PSTR> for &str {
            fn into_param(self) -> ::windows::runtime::Param<'a, PSTR> {
                ::windows::runtime::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(self.bytes().chain(::std::iter::once(0)).collect::<std::vec::Vec<u8>>().into_boxed_slice()) as _))
            }
        }
        #[cfg(feature = "std")]
        impl<'a> ::windows::runtime::IntoParam<'a, PSTR> for String {
            fn into_param(self) -> ::windows::runtime::Param<'a, PSTR> {
                ::windows::runtime::IntoParam::into_param(self.as_str())
            }
        }
    }
}
