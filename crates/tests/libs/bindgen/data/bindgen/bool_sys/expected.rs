pub type EnableMouseInPointer =
    unsafe extern "system" fn(fenable: windows_sys::core::BOOL) -> windows_sys::core::BOOL;
windows_link::link!("user32.dll" "system" fn EnableMouseInPointer(fenable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
