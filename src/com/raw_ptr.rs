use super::{ComInterface, IUnknown};
use crate::{AbiTransferable, Guid};

/// A non-reference-counted pointer to a COM interface.
pub type RawComPtr<T> = Option<NonNullRawComPtr<T>>;

/// A non-reference-counted pointer to a COM interface that is guaranteed not null.
#[repr(transparent)]
pub struct NonNullRawComPtr<T: ComInterface> {
    inner: std::ptr::NonNull<std::ptr::NonNull<<T as ComInterface>::VTable>>,
}

impl<T: ComInterface> NonNullRawComPtr<T> {
    /// Create from a non-null pointer.
    pub fn new(inner: std::ptr::NonNull<std::ptr::NonNull<<T as ComInterface>::VTable>>) -> Self {
        Self { inner }
    }

    /// Access a reference to the ComInterface's vtable.
    pub fn vtable(&self) -> &<T as ComInterface>::VTable {
        unsafe { &(*(*self.inner.as_ptr()).as_ptr()) }
    }

    /// Cast a ComInterface to another ComInterface.
    ///
    /// # Safety
    /// The ComInterface being cast to must be an interface that the
    /// current interface inherits from.
    pub unsafe fn cast<U: ComInterface>(self) -> NonNullRawComPtr<U> {
        NonNullRawComPtr {
            inner: self.inner.cast(),
        }
    }

    /// Access as a raw pointer.
    pub fn as_raw(self) -> *mut std::ptr::NonNull<<T as ComInterface>::VTable> {
        self.inner.as_ptr()
    }

    pub fn as_iunknown(&self) -> NonNullRawComPtr<IUnknown> {
        // Safe because all ComInterfaces must be castable to IUnknown
        unsafe { self.cast::<IUnknown>() }
    }
}

unsafe impl<T: ComInterface> AbiTransferable for NonNullRawComPtr<T> {
    type Abi = Self;

    fn get_abi(&self) -> Self::Abi {
        *self
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self
    }
}

unsafe impl<T: ComInterface> ComInterface for NonNullRawComPtr<T> {
    type VTable = <T as ComInterface>::VTable;

    fn iid() -> Guid {
        T::iid()
    }
}

impl<T: ComInterface> PartialEq for NonNullRawComPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: ComInterface> Clone for NonNullRawComPtr<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: ComInterface> Copy for NonNullRawComPtr<T> {}

impl<T: ComInterface> std::fmt::Debug for NonNullRawComPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.inner))
    }
}
