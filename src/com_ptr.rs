use crate::*;

#[repr(transparent)]
pub struct ComPtr<T: ComInterface> {
    ptr: *mut *mut T::VTable,
}

impl<T: ComInterface> ComPtr<T> {
    #[inline]
    pub fn get(&self) -> *const *const T::VTable {
        self.ptr as *const *const _
    }

    pub fn set(&mut self) -> *mut *const *const T::VTable {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*(self.ptr as *const *const abi_IUnknown))).release)(self.get_iunknown());
                self.ptr = std::ptr::null_mut();
            }
        }
        &mut self.ptr as *mut _ as *mut _
    }

    #[inline]
    pub fn get_iunknown(&self) -> *const *const abi_IUnknown {
        self.ptr as *const *const abi_IUnknown
    }
}

impl<T: ComInterface> Clone for ComPtr<T> {
    fn clone(&self) -> Self {
        if !self.ptr.is_null() {
            unsafe { ((*(*(self.get_iunknown()))).addref)(self.get_iunknown()) };
        }
        Self { ptr: self.ptr }
    }
}

impl<T: ComInterface> Drop for ComPtr<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ((*(*(self.get_iunknown()))).release)(self.get_iunknown()) };
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
