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

            unsafe fn drop_param(param: &mut ::windows::runtime::Param<'_, Self>) {
                if let ::windows::runtime::Param::Boxed(value) = param {
                    if !value.is_null() {
                        unsafe { ::windows::runtime::heap_free(value.0 as *mut _) }
                    }
                }
            }
        }
        impl<'a> ::windows::runtime::IntoParam<'a, PSTR> for &str {
            fn into_param(self) -> ::windows::runtime::Param<'a, PSTR> {
                let len = self.as_bytes().len();

                if let Ok(value) = ::windows::runtime::heap_alloc(len + 1) {
                    let value = unsafe { core::slice::from_raw_parts_mut(value as *mut u8, len + 1) };
                    value.copy_from_slice(self.as_bytes());
                    value[len] = 0;
    
                    ::windows::runtime::Param::Boxed(PSTR(value.as_mut_ptr()))
                } else {
                    ::windows::runtime::Param::None
                }
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
