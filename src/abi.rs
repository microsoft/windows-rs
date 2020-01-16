use crate::*;

pub type handle = *mut std::ffi::c_void;

#[repr(C)]
pub struct IUnknown {
    query: extern "system" fn(handle, &Guid, *mut handle) -> ErrorCode,
    addref: extern "system" fn(handle) -> u32,
    release: extern "system" fn(handle) -> u32,
}

impl IUnknown {
    pub fn query(ptr: handle, guid: &Guid) -> handle {
        unsafe {
            let mut result = std::ptr::null_mut();
            if !ptr.is_null() {
                ((*(*(ptr as *const *const Self))).query)(ptr, guid, &mut result);
            }
            result
        }
    }

    pub fn addref(ptr: handle) {
        unsafe {
            if !ptr.is_null() {
                ((*(*(ptr as *const *const Self))).addref)(ptr);
            }
        }
    }

    pub fn release(ptr: handle) {
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
    type_name: extern "system" fn(handle, *mut handle) -> ErrorCode,
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
