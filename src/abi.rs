use crate::*;
use com::interfaces::IUnknown;

pub type RawPtr = *mut std::ffi::c_void;

#[repr(C)]
pub struct ComPtr {
    ptr: RawPtr,
}

pub unsafe fn query<I: QueryType>(ptr: RawPtr) -> RawPtr {
    if ptr.is_null() {
        return ptr;
    }
    let mut result = std::ptr::null_mut();
    let ptr = com::InterfacePtr::<dyn IUnknown>::new(ptr as *mut _);
    ptr.query_interface(I::type_guid() as *const Guid as *const _, &mut result);
    result
}

impl ComPtr {
    pub unsafe fn addref(ptr: RawPtr) -> ComPtr {
        let ptr = com::InterfacePtr::<dyn IUnknown>::new(ptr as *mut _);
        ptr.add_ref();
        ComPtr {
            ptr: ptr.as_raw() as *mut _,
        }
    }

    pub fn query<I: QueryType>(&self) -> ComPtr {
        ComPtr {
            ptr: unsafe { query::<I>(self.ptr) },
        }
    }

    pub fn get(&self) -> RawPtr {
        self.ptr
    }

    pub fn set(&mut self) -> *mut RawPtr {
        unsafe {
            if !self.ptr.is_null() {
                let ptr = com::InterfacePtr::<dyn IUnknown>::new(self.ptr as *mut _);
                ptr.release();
                self.ptr = std::ptr::null_mut();
            }
            &mut self.ptr
        }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

impl Default for ComPtr {
    fn default() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }
}

impl Clone for ComPtr {
    fn clone(&self) -> ComPtr {
        unsafe { ComPtr::addref(self.ptr) }
    }
}

impl Drop for ComPtr {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                let ptr = com::InterfacePtr::<dyn IUnknown>::new(self.ptr as *mut _);
                ptr.release();
            }
        }
    }
}
