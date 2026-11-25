#[repr(C)]
pub struct IInner_Vtbl {
    Value: usize,
}
#[repr(C)]
pub struct IOuter_Vtbl {
    pub GetInner: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
}
