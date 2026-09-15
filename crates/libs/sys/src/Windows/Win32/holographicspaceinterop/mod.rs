#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HWND_UserFree(param0 : *mut u32, param1 : *mut super::HWND));
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HWND_UserFree64(param0 : *mut u32, param1 : *mut super::HWND));
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HWND_UserMarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HWND) -> *mut u8);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HWND_UserMarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HWND) -> *mut u8);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HWND_UserSize(param0 : *mut u32, param1 : u32, param2 : *mut super::HWND) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HWND_UserSize64(param0 : *mut u32, param1 : u32, param2 : *mut super::HWND) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HWND_UserUnmarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HWND) -> *mut u8);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HWND_UserUnmarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HWND) -> *mut u8);
