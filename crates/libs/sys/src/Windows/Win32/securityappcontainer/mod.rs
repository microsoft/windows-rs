#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetAppContainerNamedObjectPath(token : super::HANDLE, appcontainersid : super::PSID, objectpathlength : super::ULONG, objectpath : windows_sys::core::PWSTR, returnlength : super::PULONG) -> windows_sys::core::BOOL);
