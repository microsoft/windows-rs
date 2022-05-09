use super::*;

/// Describes how a type can be transfered across FFI boundaries
///
/// # Safety
///
/// * The associated type `Abi` must be safe to transfer over FFI boundaries (e.g., it must have a stable layout)
/// * It must be legal for `Abi` to be all zeros
/// * `from_abi` must be implemented if there are any in-memory representations of `Abi` that are not valid representations of `Self`.
///   `from_abi` must then check for this illegal representations and return an error if they are found.
///   * For example, size `Abi` can be all zeros, if `Self` cannot be, then `from_abi` must check for all zeros and return an error if found.
#[doc(hidden)]
pub unsafe trait Abi: Sized {
    /// The type used to represent `Self` across FFI boundaries
    type Abi;

    /// Converts an abi value to `Self` or fails
    ///
    /// # Safety
    ///
    /// This function is safe to call if `abi` can safely be trivial transmuted to `Self`.
    /// Note that if `abi`'s drop implementation will be run. If this is incorrect for `Self`,
    /// consider a custom implementation of `from_abi`.
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        Ok(core::mem::transmute_copy(&abi))
    }

    /// # Safety
    ///
    /// This function is only safe to call inside of `_param`'s `Drop` implementation
    /// as `_param` may contain freed memory inside of it after this function is run
    unsafe fn drop_param(_param: &mut Param<Self>) {}
}

// SAFETY: raw pointers are FFI safe, `Abi` & `Self` are the same so thus they have the same in-memory representation, and
// all representations of `Abi` are valid representations of `Self`.
unsafe impl<T> Abi for *mut T {
    type Abi = Self;
}

// SAFETY: see the justification for `*mut T`
unsafe impl<T> Abi for *const T {
    type Abi = Self;
}

unsafe impl<T: Interface> Abi for T {
    type Abi = RawPtr;

    /// Converts an abi value to `Self` or fails
    ///
    /// # Safety
    ///
    /// This function is safe as long as `abi` is a valid interface pointer or null
    /// The interface's ref count is assumed to already have been incremented for
    /// this handle meaning that if the interface pointer gets dropped it is
    /// safe to call `Release`.
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        if abi.is_null() {
            Err(Error::OK)
        } else {
            Ok(core::mem::transmute_copy(&abi))
        }
    }
}

// SAFETY: optional interfaces are FFI safe, optional interfaces and raw pointers have the same
// in-memory representation, and all representations of `Abi` are valid representations of `Self`.
unsafe impl<T: Interface> Abi for Option<T> {
    type Abi = RawPtr;
}

// SAFETY: see the justification for `*mut T`
unsafe impl Abi for usize {
    type Abi = Self;
}

// SAFETY: see the justification for `*mut T`
unsafe impl Abi for isize {
    type Abi = Self;
}
