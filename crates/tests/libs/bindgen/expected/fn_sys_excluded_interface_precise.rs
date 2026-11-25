windows_link::link!("test.dll" "system" fn CreateController(options : ControllerOptions, controller : *mut *mut core::ffi::c_void) -> HRESULT);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ControllerOptions {
    pub controller: *mut *mut core::ffi::c_void,
    pub value: u32,
}
impl Default for ControllerOptions {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HRESULT = i32;
