use super::*;

#[doc(hidden)]
pub unsafe trait Abi: Sized {
    type Abi;

    /// # Safety
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        Ok(core::mem::transmute_copy(&abi))
    }

    /// # Safety
    unsafe fn drop_param(_: &mut Param<Self>) {}
}

unsafe impl<T> Abi for *mut T {
    type Abi = Self;
}

unsafe impl<T> Abi for *const T {
    type Abi = Self;
}

unsafe impl<T: Interface> Abi for T {
    type Abi = RawPtr;

    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        let abi: RawPtr = core::mem::transmute_copy(&abi);

        if abi.is_null() {
            Err(Error::OK)
        } else {
            Ok(core::mem::transmute_copy(&abi))
        }
    }
}

unsafe impl<T: Interface> Abi for Option<T> {
    type Abi = RawPtr;
}

unsafe impl Abi for usize {
    type Abi = Self;
}

unsafe impl Abi for isize {
    type Abi = Self;
}
