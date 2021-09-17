use super::*;

pub fn gen_handle() -> TokenStream {
    quote! {
        #[derive(::std::clone::Clone, ::std::marker::Copy, ::std::default::Default, ::std::fmt::Debug, ::std::cmp::PartialEq, ::std::cmp::Eq)]
        #[repr(transparent)]
        pub struct HANDLE(pub isize);
        unsafe impl ::windows::Handle for HANDLE {
            fn is_invalid(&self) -> bool {
                self.0 == 0 || self.0 == -1
            }

            fn ok(self) -> ::windows::Result<Self> {
                if self.is_invalid() {
                    Err(::windows::HRESULT::from_thread().into())
                } else {
                    Ok(self)
                }
            }
        }
        unsafe impl ::windows::Abi for HANDLE {
            type Abi = Self;
            type DefaultType = Self;
        }
    }
}
