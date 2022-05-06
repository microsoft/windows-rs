use super::*;

/// Describes how a type can be transfered across FFI boundaries
///
/// # Safety
///
/// * The associated type `Abi` must be safe to transfer over FFI boundaries (e.g., it must have a stable layout)
/// * `from_abi` must be implemented if there are some in-memory representations of `Abi` that are not valid representations of `Self`
#[doc(hidden)]
pub unsafe trait Abi: Sized {
    /// The type used to represent `Self` across FFI boundaries
    type Abi;

    /// Converts an abi value to `Self` or fails
    ///
    /// # Safety
    ///
    /// This function is safe to call if `abi` can safely be trivial transmuted to `Self`
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
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        if abi.is_null() {
            Err(Error::OK)
        } else {
            // TODO: shouldn't `AddRef` be called here?
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
