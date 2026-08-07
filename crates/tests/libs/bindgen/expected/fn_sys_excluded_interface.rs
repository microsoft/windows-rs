windows_link::link!("test.dll" "system" fn CreateController(options : ControllerOptions, controller : *mut *mut core::ffi::c_void) -> HRESULT);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ControllerOptions {
    pub controller: *mut *mut core::ffi::c_void,
    pub value: u32,
}
pub type HRESULT = i32;
