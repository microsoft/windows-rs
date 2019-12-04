use crate::*;

pub enum Void {}

impl Void {
    pub fn null_mut() -> *mut Void {
        std::ptr::null_mut()
    }
}

#[repr(C)]
pub struct IUnknown {
    query: extern "system" fn(*const Void, &Guid, *mut *mut Void) -> ErrorCode,
    addref: extern "system" fn(*const Void) -> u32,
    release: extern "system" fn(*const Void) -> u32,
}

impl IUnknown {
    pub fn query(ptr: *const Void, guid: &Guid) -> *const Void {
        unsafe {
            let mut result = Void::null_mut();
            ((*(*(ptr as *const *const Self))).query)(ptr, guid, &mut result);
            result
        }
    }

    pub fn addref(ptr: *const Void) -> u32 {
        unsafe { ((*(*(ptr as *const *const Self))).addref)(ptr) }
    }

    pub fn release(ptr: *const Void) -> u32 {
        unsafe { ((*(*(ptr as *const *const Self))).release)(ptr) }
    }
}

#[repr(C)]
pub struct IInspectable {
    impl_0: usize,
    impl_1: usize,
    impl_2: usize,
    impl_3: usize,
    type_name: extern "system" fn(*const Void, *mut *mut Void) -> ErrorCode,
}

impl IInspectable {
    pub fn type_name(ptr: *const Void) -> String {
        unsafe {
            let mut hstring = Void::null_mut();
            ((*(*(ptr as *const *const Self))).type_name)(ptr, &mut hstring);
            String { hstring }
        }
    }
}
