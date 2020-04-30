use crate::{ComInterface, ComInterfacePtr, Guid};

/// A reference counted pointer to a COM interface
#[repr(transparent)]
pub struct ComPtr<T: ComInterface> {
    ptr: ComInterfacePtr<T>,
}

impl<T: ComInterface> ComPtr<T> {
    pub fn set_abi(&mut self) -> *mut ComInterfacePtr<T> {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*(self.as_iunknown()))).release)(self.as_iunknown());
            }
            self.ptr = std::ptr::null_mut();
        }
        &mut self.ptr as *mut _ as _
    }
}

unsafe impl<T: ComInterface> ComInterface for ComPtr<T> {
    const GUID: Guid = T::GUID;
    type VTable = T::VTable;
}

impl<T: ComInterface> Clone for ComPtr<T> {
    fn clone(&self) -> Self {
        if !self.ptr.is_null() {
            unsafe { ((*(*(self.as_iunknown()))).addref)(self.as_iunknown()) };
        }
        Self { ptr: self.ptr }
    }
}

impl<T: ComInterface> Drop for ComPtr<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ((*(*(self.as_iunknown()))).release)(self.as_iunknown()) };
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
