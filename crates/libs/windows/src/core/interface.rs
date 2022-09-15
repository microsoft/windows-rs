use super::*;
use bindings::IWeakReferenceSource;

pub unsafe trait Vtable: Sized {
    // TOOD: Just call this `Type`?
    type Vtable;

    /// A reference to the interface's vtable
    #[doc(hidden)]
    fn vtable(&self) -> &Self::Vtable {
        // SAFETY: the implementor of the trait guarantees that `Self` is castable to its vtable
        unsafe { self.assume_vtable::<Self>() }
    }

    /// Cast this interface as a reference to the supplied interfaces `Vtable`
    ///
    /// # SAFETY
    ///
    /// This is safe if `T` is an equivalent interface to `Self` or a super interface.
    /// In other words, `T::Vtable` must be equivalent to the beginning of `Self::Vtable`.
    #[doc(hidden)]
    unsafe fn assume_vtable<T: Vtable>(&self) -> &T::Vtable {
        &**(self.as_raw() as *mut *mut T::Vtable)
    }

        /// Returns the raw COM interface pointer. The resulting pointer continues to be owned by the `Interface` implementation.
        #[inline(always)]
        fn as_raw(&self) -> *mut core::ffi::c_void {
            // SAFETY: implementors of this trait must guarantee that the implementing type has a pointer in-memory representation
            unsafe { core::mem::transmute_copy(self) }
        }
    
        /// Returns the raw COM interface pointer and releases ownership. It the caller's responsibility to release the COM interface pointer.
        fn into_raw(self) -> *mut core::ffi::c_void {
            // SAFETY: implementors of this trait must guarantee that the implementing type has a pointer in-memory representation
            let raw = self.as_raw();
            std::mem::forget(self);
            raw
        }
    
        /// Creates an `Interface` by taking ownership of the `raw` COM interface pointer.
        ///
        /// # Safety
        ///
        /// The `raw` pointer must be owned by the caller and represent a valid COM interface pointer. In other words,
        /// it must point to a vtable beginning with the `IUnknown` function pointers and match the vtable of `Interface`.
        unsafe fn from_raw(raw: *mut core::ffi::c_void) -> Self {
            std::mem::transmute_copy(&raw)
        }
    
        /// Creates an `Interface` that is valid so long as the `raw` COM interface pointer is valid.
        ///
        /// # Safety
        ///
        /// The `raw` pointer must be a valid COM interface pointer. In other words, it must point to a vtable
        /// beginning with the `IUnknown` function pointers and match the vtable of `Interface`.
        unsafe fn from_raw_borrowed<'a>(raw: &'a *mut core::ffi::c_void) -> &'a Self {
            std::mem::transmute_copy(&raw)
        }
}

/// Provides low-level access to a COM interface.
///
/// This trait is automatically implemented by the generated bindings and should not be
/// implemented manually.
///
/// # Safety
///
/// It is only safe to implement this trait if the implementing type is a valid COM interface pointer meaning
/// the following criteria are met:
/// * its in-memory representation is equal to `NonNull<NonNull<Self::VTable>>`
/// * the vtable is correctly structured beginning with the `IUnknown` function pointers
/// * the vtable must be the correct vtable for the supplied IID
pub unsafe trait Interface: Vtable {
    /// A unique identifier representing this interface.
    const IID: GUID;

    /// Attempts to cast the current interface to another interface using `QueryInterface`.
    ///
    /// The name `cast` is preferred to `query` because there is a WinRT method named query but not one
    /// named cast.
    fn cast<T: Interface>(&self) -> Result<T> {
        let mut result = None;
        // SAFETY: `result` is valid for writing an interface pointer and it is safe
        // to cast the `result` pointer as `T` on success because we are using the `IID` tied
        // to `T` which the implementor of `Interface` has guaranteed is correct
        unsafe { self.query(&T::IID, &mut result as *mut _ as _).and_some(result) }
    }

    /// Attempts to create a [`Weak`] reference to this object.
    fn downgrade(&self) -> Result<Weak<Self>> {
        self.cast::<IWeakReferenceSource>().and_then(|source| Weak::downgrade(&source))
    }

    /// Call `QueryInterface` on this interface
    ///
    /// # SAFETY
    ///
    /// `interface` must be a non-null, valid pointer for writing an interface pointer
    #[doc(hidden)]
    unsafe fn query(&self, iid: &GUID, interface: *mut *const core::ffi::c_void) -> HRESULT {
        (self.assume_vtable::<IUnknown>().QueryInterface)(self.as_raw(), iid, interface)
    }
}
