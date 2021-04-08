use super::*;

pub fn gen_handle() -> TokenStream {
    quote! {
        impl HANDLE {
            pub const NULL: Self = Self(0);
            pub const INVALID: Self = Self(-1);
            pub fn is_null(&self) -> bool {
                self.0 == 0
            }
            pub fn is_invalid(&self) -> bool {
                self.0 == -1
            }
        }
    }
}
