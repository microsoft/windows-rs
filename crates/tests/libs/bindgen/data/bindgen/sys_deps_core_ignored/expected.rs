windows_link::link!("user32.dll" "system" fn EnableMouseInPointer(fenable : BOOL) -> BOOL);
windows_link::link!("user32.dll" "system" fn LoadCursorW(hinstance : HINSTANCE, lpcursorname : PCWSTR) -> HCURSOR);
pub type BOOL = i32;
pub type HCURSOR = *mut core::ffi::c_void;
pub type HICON = *mut core::ffi::c_void;
pub type HINSTANCE = *mut core::ffi::c_void;
pub type HMODULE = *mut core::ffi::c_void;
pub type PCWSTR = *const u16;
