use super::*;

/// Describes how a type can be transfered across FFI boundaries
///
/// # Safety
///
/// * It must be safe to alias `Self`
///     * For example, aliasing `IUnknown` is safe while aliasing `Box<T>` is not.
/// * Values of type `Abi` must not outlive the associated values of type `Self`
/// * The associated type `Abi` must be safe to transfer over FFI boundaries (e.g., it must have a stable layout)
///     * The `Abi` type must also be trivially droppable. For non-trivially droppable types consider wrapping in `ManuallyDrop`.
///       This ensures that when the type is passed over an FFI boundary, the `Drop` impl is not called.
/// * It must be legal for `Abi` to be all zeros
///     * This allows for params across the FFI boundary to be initialized as zero.
#[doc(hidden)]
pub unsafe trait Abi: Sized {
    /// The type used to represent `Self` across FFI boundaries
    type Abi;

    /// Converts `self` to an Abi value
    fn abi(&self) -> Self::Abi;

    /// Converts an abi value to `Self` or fails.
    ///
    /// Requirements:
    /// * `from_abi` must check for illegal representations and return an error if they are found.
    /// * For example, since `Abi` can be all zeros, if `Self` cannot be, then `from_abi` must check for all zeros and return an error if found.
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self>;
}

unsafe impl<T: Interface> Abi for T {
    type Abi = *mut core::ffi::c_void;

    fn abi(&self) -> Self::Abi {
        self.as_raw()
    }

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
            // there's no need to call `AddRef` after converting since `abi` is required to
            // be an "owning" pointer and already have an incremented ref count
            Ok(core::mem::transmute_copy(&abi))
        }
    }
}

// SAFETY: optional interfaces are FFI safe, optional interfaces and raw pointers have the same
// in-memory representation, and all representations of `Abi` are valid representations of `Self`.
unsafe impl<T: Interface> Abi for Option<T> {
    type Abi = *mut core::ffi::c_void;

    fn abi(&self) -> Self::Abi {
        self.as_ref().map(|i| i.as_raw()).unwrap_or_else(std::ptr::null_mut)
    }

    /// Converts an abi value to `Self` or fails
    ///
    /// # Safety
    ///
    /// This function is safe as long as `abi` is a valid interface pointer or null
    /// The interface's ref count is assumed to already have been incremented for
    /// this handle meaning that if the interface pointer gets dropped it is
    /// safe to call `Release`.
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        // there's no need to call `AddRef` after converting since `abi` is required to
        // be an "owning" pointer and already have an incremented ref count
        Ok(core::mem::transmute_copy(&abi))
    }
}

macro_rules! primitive_types {
    ($($t:ty),+) => {
        $(
            unsafe impl Abi for $t {
                type Abi = Self;

                fn abi(&self) -> Self::Abi {
                    *self
                }

                unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
                    Ok(abi)
                }
            }
        )*
    };
}

primitive_types! {
    bool,
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    f32,
    f64,
    usize,
    isize
}

// SAFETY: raw pointers are FFI safe, `Abi` & `Self` are the same so thus they have the same in-memory representation, and
// all representations of `Abi` are valid representations of `Self`.
unsafe impl<T> Abi for *mut T {
    type Abi = Self;

    fn abi(&self) -> Self::Abi {
        *self
    }

    /// Converts an abi value to `Self` or fails.
    ///
    /// # SAFETY
    ///
    /// This is safe to call as long as `abi` is valid pointer to `T`
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        Ok(abi)
    }
}

// SAFETY: see the justification for `*mut T`
unsafe impl<T> Abi for *const T {
    type Abi = Self;

    fn abi(&self) -> Self::Abi {
        *self
    }

    /// Converts an abi value to `Self` or fails.
    ///
    /// # SAFETY
    ///
    /// This is safe to call as long as `abi` is valid pointer to `T`
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        Ok(abi)
    }
}
