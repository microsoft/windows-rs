pub type EnableMouseInPointer = unsafe extern "system" fn(fenable: BOOL) -> BOOL;
windows_link::link!("user32.dll" "system" fn EnableMouseInPointer(fenable : BOOL) -> BOOL);
pub type BOOL = i32;
