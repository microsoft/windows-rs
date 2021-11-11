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
        unsafe impl ::windows::core::Abi for PSTR {
            type Abi = Self;

            #[cfg(feature = "alloc")]
            unsafe fn drop_param(param: &mut ::windows::core::Param<'_, Self>) {
                if let ::windows::core::Param::Boxed(value) = param {
                    if !value.is_null() {
                        unsafe { ::windows::core::alloc::boxed::Box::from_raw(value.0); }
                    }
                }
            }
        }
        #[cfg(feature = "alloc")]
        impl<'a> ::windows::core::IntoParam<'a, PSTR> for &str {
            fn into_param(self) -> ::windows::core::Param<'a, PSTR> {
                ::windows::core::Param::Boxed(PSTR(::windows::core::alloc::boxed::Box::<[u8]>::into_raw(self.bytes().chain(::core::iter::once(0)).collect::<::windows::core::alloc::vec::Vec<u8>>().into_boxed_slice()) as _))
            }
        }
        #[cfg(feature = "alloc")]
        impl<'a> ::windows::core::IntoParam<'a, PSTR> for ::windows::core::alloc::string::String {
            fn into_param(self) -> ::windows::core::Param<'a, PSTR> {
                ::windows::core::IntoParam::into_param(self.as_str())
            }
        }
    }
}
