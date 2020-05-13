use crate::unknown::abi_IUnknown;
use crate::{ComInterface, RawPtr, Result};

/// An equivalent to `std::convert::TryInto` for converting between interfaces
pub trait TryInto<T: ComInterface> {
    fn try_into(self) -> Result<T>;
}

impl<From: ComInterface + Sized, Into: ComInterface> TryInto<Into> for &From {
    fn try_into(self) -> Result<Into> {
        unsafe {
            let mut into = std::ptr::null_mut();
            let from: RawPtr = std::mem::transmute_copy(self);

            if from.is_null() {
                return Ok(std::mem::transmute_copy(&into));
            }

            ((*(*(from as *const *const abi_IUnknown))).unknown_query_interface)(
                from as *const *const abi_IUnknown,
                &Into::iid(),
                &mut into,
            )
            .ok()?;

            debug_assert!(!into.is_null());

            Ok(std::mem::transmute_copy(&into))
        }
    }
}
