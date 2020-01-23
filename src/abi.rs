use crate::*;

pub type RawPtr = *mut std::ffi::c_void;

#[repr(C)]
pub struct ComPtr{ ptr: RawPtr }

// pub fn query<I: TypeGuid>(ptr: RawPtr) -> RawPtr {
//     unsafe {
//         let mut result = std::ptr::null_mut();
//         if !ptr.is_null() {
//             ((*(*(ptr as *const *const IUnknown))).query)(ptr, I::type_guid(), &mut result);
//         }
//         result
//     }
// }

// pub fn addref(ptr: RawPtr) -> RawPtr {
//     unsafe {
//         if !ptr.is_null() {
//             ((*(*(ptr as *const *const IUnknown))).addref)(ptr);
//         }
//         ptr
//     }
// }

impl ComPtr {

    pub fn take_ownership(ptr: RawPtr) -> ComPtr {
        ComPtr {ptr }
    }

    pub fn addref(ptr: RawPtr) -> ComPtr {
    unsafe {
        if !ptr.is_null() {
            ((*(*(ptr as *const *const IUnknown))).addref)(ptr);
        }
        ComPtr { ptr }
    }
    }

    pub fn query<I: TypeGuid>(&self) -> ComPtr {
    unsafe {
        let mut ptr = std::ptr::null_mut();
        if !self.ptr.is_null() {
            ((*(*(self.ptr as *const *const IUnknown))).query)(self.ptr, I::type_guid(), &mut ptr);
        }
        ComPtr { ptr }
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
        Self { ptr: std::ptr::null_mut() }
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
