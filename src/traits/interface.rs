use crate::*;
use bindings::Windows::Win32::System::WinRT::IWeakReferenceSource;

/// Provides low-level access to a COM interface.
///
/// This trait is automatically used by the generated bindings and should not be
/// used directly.
pub unsafe trait Interface: Sized + Abi + PartialEq {
    type Vtable;
    const IID: Guid;

    /// # Safety
    /// Returns the vtable for the current interface.
    unsafe fn vtable(&self) -> &Self::Vtable {
        self.assume_vtable::<Self>()
    }

    /// # Safety
    /// Returns the vtable for an assumed interface. The name comes from [`Box::assume_init()`] as
    /// it assumes the vtable is implemented by the current interface's vtable (e.g. a parent interface).
    unsafe fn assume_vtable<T: Interface>(&self) -> &T::Vtable {
        let this: RawPtr = std::mem::transmute_copy(self);
        &(*(*(this as *mut *mut _) as *mut _))
    }

    /// # Safety
    unsafe fn query(&self, iid: *const Guid, interface: *mut RawPtr) -> HRESULT {
        (self.assume_vtable::<IUnknown>().0)(std::mem::transmute_copy(self), iid, interface)
    }

    /// Attempts to cast the current interface to another interface using `QueryInterface`.
    /// The name `cast` is preferred to `query` because there is a WinRT method named query but not one
    /// named cast.
    fn cast<T: Interface>(&self) -> Result<T> {
        unsafe {
            let mut result = None;

            (self.assume_vtable::<IUnknown>().0)(
                std::mem::transmute_copy(self),
                &T::IID,
                &mut result as *mut _ as _,
            )
            .and_some(result)
        }
    }

    /// Attempts to create a [`Weak`] reference to this object.
    fn downgrade(&self) -> Result<Weak<Self>> {
        self.cast::<IWeakReferenceSource>()
            .and_then(|source| Weak::downgrade(&source))
    }
}
