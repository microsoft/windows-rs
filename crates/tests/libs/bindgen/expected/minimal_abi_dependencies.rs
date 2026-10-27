windows_core::link!("test.dll" "system" fn CreateValue(value : *mut *mut core::ffi::c_void));
windows_core::imp::define_interface!(IValue, IValue_Vtbl);
#[repr(C)]
pub struct IValue_Vtbl {
    Get: usize,
}
