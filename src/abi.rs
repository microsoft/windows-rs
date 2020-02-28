use crate::*;

pub type RawPtr = *mut std::ffi::c_void;

#[repr(C)]
pub struct ComPtr {
    ptr: RawPtr,
}

pub fn query<I: QueryType>(ptr: RawPtr) -> RawPtr {
    unsafe {
        let mut result = std::ptr::null_mut();
        if !ptr.is_null() {
            ((*(*(ptr as *const *const IUnknown))).query)(ptr, I::type_guid(), &mut result);
        }
        result
    }
}

impl ComPtr {
    pub fn addref(ptr: RawPtr) -> ComPtr {
        unsafe {
            if !ptr.is_null() {
                ((*(*(ptr as *const *const IUnknown))).addref)(ptr);
            }
            ComPtr { ptr }
        }
    }

    pub fn query<I: QueryType>(&self) -> ComPtr {
        ComPtr {
            ptr: query::<I>(self.ptr),
        }
    }

    pub fn get(&self) -> RawPtr {
        self.ptr
    }

    pub fn set(&mut self) -> *mut RawPtr {
        unsafe {
            if !self.ptr.is_null() {
                ((*(*(self.ptr as *const *const IUnknown))).release)(self.ptr);
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
        ComPtr::addref(self.ptr)
    }
}

impl Drop for ComPtr {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                ((*(*(self.ptr as *const *const IUnknown))).release)(self.ptr);
            }
        }
    }
}

#[repr(C)]
struct IUnknown {
    query: extern "system" fn(RawPtr, &Guid, *mut RawPtr) -> ErrorCode,
    addref: extern "system" fn(RawPtr) -> u32,
    release: extern "system" fn(RawPtr) -> u32,
}
