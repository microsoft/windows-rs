use crate::unknown::abi_IUnknown;
use crate::{ComInterface, RuntimeType};

/// A reference counted pointer to a COM interface
#[repr(transparent)]
pub struct ComPtr<T: ComInterface> {
    ptr: *mut *mut T::VTable,
}

impl<T: ComInterface> ComPtr<T> {
    #[inline]
    pub fn get_iunknown(&self) -> *const *const abi_IUnknown {
        self.ptr as *const *const abi_IUnknown
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

unsafe impl<T: ComInterface> RuntimeType for ComPtr<T> {
    type Abi = *const *const T::VTable;
    #[inline]
    fn abi(&self) -> Self::Abi {
        self.ptr as _
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*(self.get_iunknown()))).release)(self.get_iunknown());
            }
            self.ptr = std::ptr::null_mut();
        }
        &mut self.ptr as *mut _ as *mut _
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
