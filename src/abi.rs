use crate::*;

pub type RawPtr = *mut std::ffi::c_void;

#[repr(C)]
pub struct ComPtr(pub RawPtr);

pub fn query<I: TypeGuid>(ptr: RawPtr) -> RawPtr {
    unsafe {
        let mut result = std::ptr::null_mut();
        if !ptr.is_null() {
            ((*(*(ptr as *const *const IUnknown))).query)(ptr, I::type_guid(), &mut result);
        }
        result
    }
}

impl ComPtr {
    pub fn query<I: TypeGuid>(&self) -> RawPtr {
        query::<I>(self.0)
    }

    pub fn addref(&self) -> RawPtr {
        unsafe {
            if !self.0.is_null() {
                ((*(*(self.0 as *const *const IUnknown))).addref)(self.0);
            }
            self.0
        }
    }

    pub fn get(&self) -> RawPtr {
        self.0
    }

    pub fn set(&mut self) -> *mut RawPtr {
        unsafe {
            if !self.0.is_null() {
                ((*(*(self.0 as *const *const IUnknown))).release)(self.0);
                self.0 = std::ptr::null_mut();
            }
            &mut self.0
        }
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

impl Default for ComPtr {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

impl Clone for ComPtr {
    fn clone(&self) -> ComPtr {
        unsafe {
            if !self.0.is_null() {
                ((*(*(self.0 as *const *const IUnknown))).addref)(self.0);
            }
            ComPtr(self.0)
        }
    }
}

impl Drop for ComPtr {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() {
                ((*(*(self.0 as *const *const IUnknown))).release)(self.0);
            }
        }
    }
}

impl From<RawPtr> for ComPtr {
    fn from(ptr: RawPtr) -> Self {
        unsafe { std::mem::transmute(ptr) }
    }
}

#[repr(C)]
struct IUnknown {
    query: extern "system" fn(RawPtr, &Guid, *mut RawPtr) -> ErrorCode,
    addref: extern "system" fn(RawPtr) -> u32,
    release: extern "system" fn(RawPtr) -> u32,
}
