use crate::*;

#[doc(hidden)]
pub unsafe trait Abi: Sized + Clone {
    type Abi;
    type DefaultType: Sized + Clone + PartialEq;

    /// # Safety
    unsafe fn from_default(value: &Self::DefaultType) -> Result<Self> {
        let value = value as *const _ as *const Self;
        Ok((*value).clone())
    }

    /// # Safety
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        Ok(std::mem::transmute_copy(&abi))
    }

    /// # Safety
    unsafe fn drop_param(_: &mut Param<Self>) {}
}

unsafe impl<T> Abi for *mut T {
    type Abi = Self;
    type DefaultType = Self;
}

unsafe impl<T> Abi for *const T {
    type Abi = Self;
    type DefaultType = Self;
}

unsafe impl<T: Interface> Abi for T {
    type Abi = RawPtr;
    type DefaultType = Option<T>;

    unsafe fn from_default(value: &Self::DefaultType) -> Result<Self> {
        let value = value as *const _ as *const Option<Self>;

        match &*value {
            Some(value) => Ok(value.clone()),
            None => Err(Error::OK),
        }
    }

    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        let abi: RawPtr = std::mem::transmute_copy(&abi);

        if abi.is_null() {
            Err(Error::OK)
        } else {
            Ok(std::mem::transmute_copy(&abi))
        }
    }
}

unsafe impl<T: Interface> Abi for Option<T> {
    type Abi = RawPtr;
    type DefaultType = Self;
}
