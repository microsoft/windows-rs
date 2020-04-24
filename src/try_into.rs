use crate::*;

pub trait TryInto<T: ComInterface> {
    fn try_into(self) -> Option<T>;
}

impl<From: ComInterface + Sized, Into: ComInterface> TryInto<Into> for &From {
    fn try_into(self) -> Option<Into> {
        unsafe {
            let mut into = std::ptr::null_mut();
            let from: RawPtr = std::mem::transmute_copy(self);

            if !from.is_null() {
                ((*(*(from as *const *const abi_IUnknown))).query)(from, &Into::GUID, &mut into);
            }

            match into.is_null() {
                false => Some(std::mem::transmute_copy(&into)),
                true => None,
            }
        }
    }
}
