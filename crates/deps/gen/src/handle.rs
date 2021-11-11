use super::*;

pub fn gen_handle() -> TokenStream {
    quote! {
        #[derive(::core::clone::Clone, ::core::marker::Copy, ::core::default::Default, ::core::fmt::Debug, ::core::cmp::PartialEq, ::core::cmp::Eq)]
        #[repr(transparent)]
        pub struct HANDLE(pub isize);
        unsafe impl ::windows::core::Handle for HANDLE {
            fn is_invalid(&self) -> bool {
                self.0 == 0 || self.0 == -1
            }

            fn ok(self) -> ::windows::core::Result<Self> {
                if self.is_invalid() {
                    Err(::windows::core::Error::from_win32())
                } else {
                    Ok(self)
                }
            }
        }
        unsafe impl ::windows::core::Abi for HANDLE {
            type Abi = Self;
        }
    }
}
