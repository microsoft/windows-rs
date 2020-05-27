use crate::{ComInterface, Result};

/// An equivalent to `std::convert::TryInto` for converting between interfaces
pub trait TryInto<T: ComInterface> {
    fn try_into(self) -> Result<T>;
}

impl<From: ComInterface + Sized, Into: ComInterface> TryInto<Into> for &From {
    fn try_into(self) -> Result<Into> {
        let mut into = std::ptr::null_mut();
        let from = self.as_iunknown();

        if let Some(ptr) = from {
            unsafe {
                ((*(*ptr.as_ptr()).as_ptr()).unknown_query_interface)(
                    Some(ptr),
                    &Into::iid(),
                    &mut into,
                )
                .ok()?
            };

            debug_assert!(!into.is_null());
        }

        unsafe { Ok(std::mem::transmute_copy(&into)) }
    }
}
