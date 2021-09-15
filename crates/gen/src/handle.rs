use super::*;

pub fn gen_handle() -> TokenStream {
    quote! {
        #[derive(::std::clone::Clone, ::std::marker::Copy, ::std::default::Default, ::std::fmt::Debug, ::std::cmp::PartialEq, ::std::cmp::Eq)]
        #[repr(transparent)]
        pub struct HANDLE(pub isize);
        impl HANDLE {
            pub const NULL: Self = Self(0);
            pub const INVALID: Self = Self(-1);
            pub fn is_invalid(&self) -> bool {
                self.0 == -1
            }
        }
        unsafe impl ::windows::Handle for HANDLE {
            fn is_null(&self) -> bool {
                *self == unsafe { ::std::mem::zeroed() }
            }

            fn ok(self) -> ::windows::Result<Self> {
                if self != Self::NULL && self != Self::INVALID {
                    Ok(self)
                } else {
                    Err(::windows::HRESULT::from_thread().into())
                }
            }
        }
        unsafe impl ::windows::Abi for HANDLE {
            type Abi = Self;
            type DefaultType = Self;
        }
    }
}
