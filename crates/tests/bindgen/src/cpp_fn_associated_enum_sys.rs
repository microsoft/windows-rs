windows_targets::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : u32) -> HRESULT);
pub type HRESULT = i32;
