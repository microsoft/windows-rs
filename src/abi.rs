use crate::*;

#[repr(C)]
pub struct IUnknown {
    query:
        extern "system" fn(*const std::ffi::c_void, &Guid, *mut *mut std::ffi::c_void) -> ErrorCode,
    addref: extern "system" fn(*const std::ffi::c_void) -> u32,
    release: extern "system" fn(*const std::ffi::c_void) -> u32,
}

impl IUnknown {
    pub fn query(ptr: *const std::ffi::c_void, guid: &Guid) -> *const std::ffi::c_void {
        unsafe {
            let mut result = std::ptr::null_mut();
            ((*(*(ptr as *const *const Self))).query)(ptr, guid, &mut result);
            result
        }
    }

    pub fn addref(ptr: *const std::ffi::c_void) -> u32 {
        unsafe { ((*(*(ptr as *const *const Self))).addref)(ptr) }
    }

    pub fn release(ptr: *const std::ffi::c_void) -> u32 {
        unsafe { ((*(*(ptr as *const *const Self))).release)(ptr) }
    }
}

#[repr(C)]
pub struct IInspectable {
    abi_0: usize,
    abi_1: usize,
    abi_2: usize,
    abi_3: usize,
    type_name: extern "system" fn(*const std::ffi::c_void, *mut *mut std::ffi::c_void) -> ErrorCode,
}

impl IInspectable {
    pub fn type_name(ptr: *const std::ffi::c_void) -> String {
        unsafe {
            let mut hstring = std::ptr::null_mut();
            ((*(*(ptr as *const *const Self))).type_name)(ptr, &mut hstring);
            String { hstring }
        }
    }
}
