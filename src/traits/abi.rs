use crate::*;

pub unsafe trait Abi: Sized {
    type Abi;
    unsafe fn get_abi(&self) -> Self::Abi;
    unsafe fn set_abi(&mut self) -> *mut Self::Abi;
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self>;
}

unsafe impl<T: Interface> Abi for T {
    type Abi = RawPtr;

    unsafe fn get_abi(&self) -> Self::Abi {
        std::mem::transmute_copy(self)
    }

    unsafe fn set_abi(&mut self) -> *mut Self::Abi {
        debug_assert!(self.is_null());
        self as *mut _ as *mut _
    }

    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        let abi: RawPtr = std::mem::transmute_copy(&abi);

        if abi.is_null() {
            Err(ErrorCode::E_POINTER.into())
        } else {
            Ok(std::mem::transmute_copy(&abi))
        }
    }
}

unsafe impl<T: Interface> Abi for Option<T> {
    type Abi = RawPtr;

    unsafe fn get_abi(&self) -> Self::Abi {
        if let Some(interface) = self {
            interface.as_raw_ptr()
        } else {
            std::ptr::null_mut()
        }
    }

    unsafe fn set_abi(&mut self) -> *mut Self::Abi {
        debug_assert!(self.is_none());
        self as *mut _ as *mut _
    }

    unsafe fn from_abi(_: Self::Abi) -> Result<Self> {
        panic!("Option<T> should not not be used for return types");
    }
}
