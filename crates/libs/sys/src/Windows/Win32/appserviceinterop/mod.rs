windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserFree(param0 : *mut u32, param1 : *mut windows_sys::core::HSTRING));
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserFree64(param0 : *mut u32, param1 : *mut windows_sys::core::HSTRING));
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserMarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut windows_sys::core::HSTRING) -> *mut u8);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserMarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut windows_sys::core::HSTRING) -> *mut u8);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserSize(param0 : *mut u32, param1 : u32, param2 : *mut windows_sys::core::HSTRING) -> u32);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserSize64(param0 : *mut u32, param1 : u32, param2 : *mut windows_sys::core::HSTRING) -> u32);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserUnmarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut windows_sys::core::HSTRING) -> *mut u8);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserUnmarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut windows_sys::core::HSTRING) -> *mut u8);
