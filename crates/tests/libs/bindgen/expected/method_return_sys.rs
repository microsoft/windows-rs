pub type HRESULT = i32;
#[repr(C)]
pub struct Interface_Vtbl {
    pub ResultVoid: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> HRESULT,
    pub ResultValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut Struct) -> HRESULT,
    pub ReturnStruct: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut Struct),
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Struct {
    pub x: i32,
    pub y: i32,
}
