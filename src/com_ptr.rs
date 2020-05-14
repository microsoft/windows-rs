use crate::{ComInterface, Guid, IUnknown, RawComPtr};

/// A reference counted pointer to a COM interface
#[repr(transparent)]
pub struct ComPtr<T: ComInterface> {
    ptr: RawComPtr<T>,
}

impl<T: ComInterface> ComPtr<T> {
    pub fn abi(&self) -> RawComPtr<T> {
        self.ptr
    }

    pub fn set_abi(&mut self) -> *mut RawComPtr<T> {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*(self.as_iunknown()))).unknown_release)(self.as_iunknown());
            }
            self.ptr = std::ptr::null_mut();
        }
        &mut self.ptr as *mut _ as _
    }
}

unsafe impl<T: ComInterface> ComInterface for ComPtr<T> {
    type VTable = T::VTable;

    fn iid() -> Guid {
        T::iid()
    }
}

impl<T: ComInterface> Clone for ComPtr<T> {
    fn clone(&self) -> Self {
        if !self.ptr.is_null() {
            unsafe { ((*(*(self.as_iunknown()))).unknown_add_ref)(self.as_iunknown()) };
        }
        Self { ptr: self.ptr }
    }
}

impl<T: ComInterface> Drop for ComPtr<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ((*(*(self.as_iunknown()))).unknown_release)(self.as_iunknown()) };
        }
    }
}

impl<T: ComInterface> Default for ComPtr<T> {
    fn default() -> Self {
        ComPtr {
            ptr: std::ptr::null_mut(),
        }
    }
}

impl<T: ComInterface> PartialEq for ComPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.query::<IUnknown>().as_raw() == other.query::<IUnknown>().as_raw()
    }
}
