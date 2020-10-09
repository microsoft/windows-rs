use super::{ComInterface, IUnknown, RawComPtr};
use crate::{AbiTransferable, Guid};

/// A reference counted pointer to a COM interface.
///
/// This is the preferred way to work with COM interfaces. If you need
/// a new handle to the COM Interface, calling `clone` on the `ComPtr`,
/// will automatically call `AddRef` and increment the reference count.
/// Additionally, dropping the `ComPtr` calls `Release` ensuring that the
/// COM interface is disposed of when no more handles to it exist.
///
/// This pointer can be null.
#[repr(transparent)]
pub struct ComPtr<T: ComInterface> {
    ptr: RawComPtr<T>,
}

unsafe impl<T: ComInterface> AbiTransferable for ComPtr<T> {
    type Abi = RawComPtr<T>;
    /// Get a raw non-reference-counted pointer to the COM interface.
    fn get_abi(&self) -> RawComPtr<T> {
        self.ptr
    }

    /// Set the COM interface pointer.
    ///
    /// This will call release on any existing interface pointer.
    fn set_abi(&mut self) -> *mut RawComPtr<T> {
        if let Some(ptr) = self.ptr {
            let ptr = ptr.as_iunknown();
            (ptr.vtable().release)(ptr);

            self.ptr = None;
        }
        &mut self.ptr
    }
}

unsafe impl<T: ComInterface> ComInterface for ComPtr<T> {
    type VTable = T::VTable;

    const IID: Guid = T::IID;
}

impl<T: ComInterface> Clone for ComPtr<T> {
    fn clone(&self) -> Self {
        if let Some(ptr) = self.ptr.map(|p| p.as_iunknown()) {
            (ptr.vtable().add_ref)(ptr);
        }
        Self { ptr: self.ptr }
    }
}

impl<T: ComInterface> Drop for ComPtr<T> {
    fn drop(&mut self) {
        if let Some(ptr) = self.ptr.map(|p| p.as_iunknown()) {
            (ptr.vtable().release)(ptr);
        }
    }
}

impl<T: ComInterface> Default for ComPtr<T> {
    fn default() -> Self {
        ComPtr { ptr: None }
    }
}

impl<T: ComInterface> PartialEq for ComPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.query::<IUnknown>().get_abi() == other.query::<IUnknown>().get_abi()
    }
}
