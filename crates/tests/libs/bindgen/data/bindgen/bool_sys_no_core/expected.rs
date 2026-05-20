windows_link::link!("user32.dll" "system" fn EnableMouseInPointer(fenable : BOOL) -> BOOL);
pub type BOOL = i32;
