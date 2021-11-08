use super::*;

pub fn gen_handle() -> TokenStream {
    quote! {
        #[derive(::std::clone::Clone, ::core::marker::Copy, ::core::default::Default, ::core::fmt::Debug, ::core::cmp::PartialEq, ::core::cmp::Eq)]
        #[repr(transparent)]
        pub struct HANDLE(pub isize);
        unsafe impl ::windows::runtime::Handle for HANDLE {
            fn is_invalid(&self) -> bool {
                self.0 == 0 || self.0 == -1
            }

            fn ok(self) -> ::windows::runtime::Result<Self> {
                if self.is_invalid() {
                    Err(::windows::runtime::Error::from_win32())
                } else {
                    Ok(self)
                }
            }
        }
        unsafe impl ::windows::runtime::Abi for HANDLE {
            type Abi = Self;
        }
    }
}
