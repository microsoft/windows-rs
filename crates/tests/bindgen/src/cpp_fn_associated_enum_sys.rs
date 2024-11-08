windows_targets::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : COINIT) -> HRESULT);
pub type COINIT = i32;
pub type HRESULT = i32;
