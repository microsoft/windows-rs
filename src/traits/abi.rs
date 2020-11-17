use crate::*;

/// Provides a generic way of referring to and converting between a Rust object
/// and its WinRT ABI equivalent.
///
/// This trait is automatically used by the generated bindings and should not be
/// used directly.
pub unsafe trait Abi: Sized {
    type Abi;

    // abi must always transmute (blit) and never Copy.
    unsafe fn abi(&self) -> Self::Abi {
        std::mem::transmute_copy(self)
    }

    // This default implemnetation is always correct. Only override if you need to assert something
    // for debugging purposes.
    unsafe fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut _ as *mut _
    }

    // This default implementation is correct for all but interfaces.
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        Ok(std::mem::transmute_copy(&abi))
    }
}

unsafe impl<T: Interface> Abi for T {
    type Abi = RawPtr;

    unsafe fn set_abi(&mut self) -> *mut Self::Abi {
        panic!("set_abi should not be used with interfaces since it implies nullable.");
    }

    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        let abi: RawPtr = std::mem::transmute_copy(&abi);

        if abi.is_null() {
            Err(Error::just_code(ErrorCode::E_POINTER))
        } else {
            Ok(std::mem::transmute_copy(&abi))
        }
    }
}

unsafe impl<T: Interface> Abi for Option<T> {
    type Abi = RawPtr;

    unsafe fn set_abi(&mut self) -> *mut Self::Abi {
        debug_assert!(self.is_none());
        self as *mut _ as *mut _
    }

    unsafe fn from_abi(_: Self::Abi) -> Result<Self> {
        panic!("Option<T> should not not be used for return types. Use Result<T> instead.");
    }
}
