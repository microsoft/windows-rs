use windows::{core::*, Win32::Foundation::*};

#[no_mangle]
unsafe extern "system" fn DllGetActivationFactory(
    _name: std::mem::ManuallyDrop<HSTRING>,
    result: *mut *mut std::ffi::c_void,
) -> HRESULT {
    *result = std::ptr::null_mut();
    CLASS_E_CLASSNOTAVAILABLE
}
