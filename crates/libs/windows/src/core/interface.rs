use super::*;
use bindings::IWeakReferenceSource;

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
pub unsafe trait Interface: Sized {
    /// A unique identifier representing this interface.
    const IID: GUID;

    /// The interface's vtable
    #[doc(hidden)]
    type Vtable;

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

    /// Turn this interface into a raw pointer
    #[inline(always)]
    fn as_raw(&self) -> *mut core::ffi::c_void {
        // SAFETY: implementors of this trait must guarantee that the implementing type has a pointer in-memory representation
        unsafe { core::mem::transmute_copy(self) }
    }

    /// Attempts to create a [`Weak`] reference to this object.
    fn downgrade(&self) -> Result<Weak<Self>> {
        self.cast::<IWeakReferenceSource>().and_then(|source| Weak::downgrade(&source))
    }

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
    unsafe fn assume_vtable<T: Interface>(&self) -> &T::Vtable {
        &**(self.as_raw() as *mut *mut T::Vtable)
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
