use super::ComInterface;
use crate::Result;

/// An equivalent to `std::convert::TryInto` for converting between interfaces.
pub trait TryInto<T: ComInterface> {
    fn try_into(self) -> Result<T>;
}

impl<From: ComInterface + Sized, Into: ComInterface> TryInto<Into> for &From {
    fn try_into(self) -> Result<Into> {
        let mut into = std::ptr::null_mut();
        let from = self.as_iunknown();

        if let Some(ptr) = from {
            unsafe { (ptr.vtable().unknown_query_interface)(ptr, &Into::iid(), &mut into).ok()? };

            debug_assert!(
                !into.is_null(),
                "Null pointer found after successful QueryInterface call"
            );
        }

        unsafe { Ok(std::mem::transmute_copy(&into)) }
    }
}
