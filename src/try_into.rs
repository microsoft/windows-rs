use crate::*;

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

            ((*(*(from as *const *const abi_IUnknown))).query)(from, &Into::GUID, &mut into)
                .ok()?;

            debug_assert!(!into.is_null());

            Ok(std::mem::transmute_copy(&into))
        }
    }
}
