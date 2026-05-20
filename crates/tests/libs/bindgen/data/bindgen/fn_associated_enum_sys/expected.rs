pub type CoInitializeEx = unsafe extern "system" fn(
    pvreserved: *const core::ffi::c_void,
    dwcoinit: u32,
) -> windows_sys::core::HRESULT;
windows_link::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : u32) -> windows_sys::core::HRESULT);
pub type COINIT = i32;
