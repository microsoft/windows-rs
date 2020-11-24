use crate::*;

/// Provides a generic way of referring to and converting between a Rust object
/// and its WinRT ABI equivalent.
///
/// This trait is automatically used by the generated bindings and should not be
/// used directly.
pub unsafe trait Abi: Sized {
    type Abi;

    /// Casts the Rust object to its ABI type without copying the object.
    fn abi(&self) -> Self::Abi {
        // It is always safe to interpret an `Abi` type's binary representation (without moving
        // the value) as the memory layout must be identical.
        unsafe { std::mem::transmute_copy(self) }
    }

    /// Returns a pointer for setting the object's value via an ABI call. This default implemnetation
    /// is always correct. Only override if you need to assert something for debugging purposes.
    unsafe fn set_abi(&mut self) -> *mut Self::Abi {
        // TODO: ideally we can debug_assert that the object has a zero memory layout.
        self as *mut _ as *mut _
    }

    /// Casts the ABI representation to a Rust object by taking ownership of the bits.
    /// This default implementation is correct for all but interfaces.
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
            Err(Error::fast_error(ErrorCode::E_POINTER))
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
