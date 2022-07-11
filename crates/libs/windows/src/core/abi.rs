use super::*;

/// Describes how a type can be transfered across FFI boundaries
///
/// # Safety
///
/// * It must always be safe to memcopy `Self` into `Self::Abi`.
/// * It must always be safe to memcopy `Self::Abi`.
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
    ///
    /// Note: `Self::Abi` is only valid for however long `&self` lives for
    fn abi(&self) -> Self::Abi {
        // SAFETY: the `Abi` trait ensures that `Self` can be memcopied into `Self::Abi`
        unsafe { core::mem::transmute_copy(self) }
    }

    /// Converts an abi value to `Self` or fails if we can determine `abi` is not valid.
    ///
    /// # Safety
    ///
    /// `abi` must be in a state where it can be trivially transmuted into a valid value of type `Self`
    /// if `abi_is_possibly_valid` returns true.
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        if Self::abi_is_possibly_valid(&abi) {
            Ok(core::mem::transmute_copy(&abi))
        } else {
            Err(Error::OK)
        }
    }

    /// Converts a reference to an abi value to a reference to `Self` or fails if we can determine `abi` is not valid.
    ///
    /// # Safety
    ///
    /// `abi` must be in a state where it can be trivially transmuted into a valid value of type `&Self`
    /// if `abi_is_possibly_valid` returns true.
    unsafe fn from_abi_ref(abi: &Self::Abi) -> Result<&Self> {
        if Self::abi_is_possibly_valid(abi) {
            Ok(core::mem::transmute(abi))
        } else {
            Err(Error::OK)
        }
    }

    /// Whether `abi` is known to be invalid
    ///
    /// Note: this does not guarantee that it definitely *is* valid.
    fn abi_is_possibly_valid(abi: &Self::Abi) -> bool {
        true
    }
}

unsafe impl<T: Interface> Abi for Option<T> {
    type Abi = *mut core::ffi::c_void;
}

unsafe impl<T: Interface> Abi for T {
    type Abi = *mut core::ffi::c_void;

    fn abi_is_possibly_valid(abi: &Self::Abi) -> bool {
        !abi.is_null()
    }
}

unsafe impl<T> Abi for *mut T {
    type Abi = Self;
}

unsafe impl<T> Abi for *const T {
    type Abi = Self;
}

macro_rules! primitive_types {
    ($($t:ty),+) => {
        $(
            unsafe impl Abi for $t {
                type Abi = Self;
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
