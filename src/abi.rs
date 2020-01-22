use crate::*;

pub type RawPtr = *mut std::ffi::c_void;

pub struct ComPtr(RawPtr);

impl ComPtr {
    pub fn query(&self, guid: &Guid) -> ComPtr {
        let mut result: ComPtr = Default::default();
        if !self.0.is_null() {
            (self.deref::<IUnknown>().query)(self.0, guid, &mut result.0);
        }
        result
    }

    pub fn set(&mut self) -> *mut ComPtr {
        if !self.0.is_null() {
            (self.deref::<IUnknown>().release)(self.0);
            self.0 = std::ptr::null_mut();
        }
        self as *mut ComPtr
    }

    pub fn deref<T>(&self) -> &T {
        unsafe { &(*(*(self.0 as *const *const T))) }
    }
}

impl Default for ComPtr {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

impl Clone for ComPtr {
    fn clone(&self) -> ComPtr {
        if !self.0.is_null() {
            (self.deref::<IUnknown>().addref)(self.0);
        }
        ComPtr(self.0)
    }
}

impl Drop for ComPtr {
    fn drop(&mut self) {
        if !self.0.is_null() {
            (self.deref::<IUnknown>().release)(self.0);
        }
    }
}

#[repr(C)]
pub struct IUnknown {
    query: extern "system" fn(RawPtr, &Guid, *mut RawPtr) -> ErrorCode,
    addref: extern "system" fn(RawPtr) -> u32,
    release: extern "system" fn(RawPtr) -> u32,
}

impl IUnknown {
    pub fn query(ptr: RawPtr, guid: &Guid) -> RawPtr {
        unsafe {
            let mut result = std::ptr::null_mut();
            if !ptr.is_null() {
                ((*(*(ptr as *const *const Self))).query)(ptr, guid, &mut result);
            }
            result
        }
    }

    pub fn addref(ptr: RawPtr) -> RawPtr {
        unsafe {
            if !ptr.is_null() {
                ((*(*(ptr as *const *const Self))).addref)(ptr);
            }
            ptr
        }
    }

    pub fn release(ptr: RawPtr) {
        unsafe {
            if !ptr.is_null() {
                ((*(*(ptr as *const *const Self))).release)(ptr);
            }
        }
    }

    pub fn release_mut(ptr: *mut RawPtr) -> *mut RawPtr {
        unsafe {
            if !(*ptr).is_null() {
                ((*(*(*ptr as *const *const Self))).release)(*ptr);
                *ptr = std::ptr::null_mut();
            }

            ptr
        }
    }
}

#[repr(C)]
pub struct IInspectable {
    __0: usize,
    __1: usize,
    __2: usize,
    __3: usize,
    type_name: extern "system" fn(RawPtr, *mut RawPtr) -> ErrorCode,
}

// impl IInspectable {
//     pub fn type_name() -> String {
//         unsafe {
//             let mut ptr = std::ptr::null_mut();
//             ((*(*(ptr as *const *const Self))).type_name)(ptr, &mut ptr);
//             String { ptr }
//         }
//     }
// }
