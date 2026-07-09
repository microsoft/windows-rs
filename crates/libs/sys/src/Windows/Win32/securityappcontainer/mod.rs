#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn GetAppContainerNamedObjectPath(token : super::winnt::HANDLE, appcontainersid : super::winnt::PSID, objectpathlength : u32, objectpath : windows_sys::core::PWSTR, returnlength : *mut u32) -> windows_sys::core::BOOL);
