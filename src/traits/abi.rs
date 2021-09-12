use crate::*;

/// Provides a generic way of referring to and converting between a Rust object
/// and its ABI equivalent.
///
/// This trait is automatically used by the generated bindings and should not be
/// used directly.
pub unsafe trait Abi: Sized + Clone {
    /// The abi representation of the implementing type.
    ///
    /// # Safety
    /// `Self` and `Abi` *must* have the same exact in-memory representation.

    // TODO: this type should not exist - the Windows crate should just use ManuallyDrop<Self> in all cases
    type Abi;

    type DefaultType: Sized + Clone + PartialEq;

    /// Converts from `Self::DefaultType` to `Result<T>`.
    fn ok(value: &Self::DefaultType) -> Result<Self> {
        unsafe {
            let value = value as *const _ as *const Self;
            Ok((*value).clone())
        }
    }

    /// Casts the Rust object to its ABI type without copying the object.
    fn abi(&self) -> Self::Abi {
        // It is always safe to interpret an `Abi` type's binary representation (without moving
        // the value) as the memory layout must be identical.
        unsafe { std::mem::transmute_copy(self) }
    }

    /// Returns a pointer for setting the object's value via an ABI call.
    // Note: This default implementation is always correct. Only override if you need to assert something for debugging purposes.
    fn set_abi(&mut self) -> *mut Self::Abi {
        // TODO: ideally we can debug_assert that the object has a zero memory layout.
        self as *mut _ as *mut _
    }

    /// # Safety
    /// Casts the ABI representation to a Rust object by taking ownership of the bits.
    /// This default implementation is correct for all but interfaces.
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        Ok(std::mem::transmute_copy(&abi))
    }

    fn drop_param(_: &mut Param<Self>) {}
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

    fn ok(value: &Self::DefaultType) -> Result<Self> {
        unsafe {
            // For some reason, Rust can't figure out that DefaultType is an Option here.
            let value = value as *const _ as *const Option<Self>;

            match &*value {
                Some(value) => Ok(value.clone()),
                None => Err(Error::OK),
            }
        }
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        panic!("set_abi should not be used with interfaces since it implies nullable.");
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

    fn set_abi(&mut self) -> *mut Self::Abi {
        debug_assert!(self.is_none());
        self as *mut _ as *mut _
    }

    unsafe fn from_abi(_: Self::Abi) -> Result<Self> {
        panic!("Option<T> should not not be used for return types. Use Result<T> instead.");
    }
}
