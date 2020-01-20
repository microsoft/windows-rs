use crate::*;

pub type RawPtr = *mut std::ffi::c_void;

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

    pub fn addref(ptr: RawPtr) {
        unsafe {
            if !ptr.is_null() {
                ((*(*(ptr as *const *const Self))).addref)(ptr);
            }
        }
    }

    pub fn release(ptr: RawPtr) {
        unsafe {
            if !ptr.is_null() {
                ((*(*(ptr as *const *const Self))).release)(ptr);
            }
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
