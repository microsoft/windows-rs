#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn AddSIDToBoundaryDescriptor(boundarydescriptor : *mut super::winnt::HANDLE, requiredsid : super::winnt::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn ClosePrivateNamespace(handle : super::winnt::HANDLE, flags : u32) -> bool);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateBoundaryDescriptorW(name : windows_sys::core::PCWSTR, flags : u32) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreatePrivateNamespaceW(lpprivatenamespaceattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lpboundarydescriptor : *const core::ffi::c_void, lpaliasprefix : windows_sys::core::PCWSTR) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn DeleteBoundaryDescriptor(boundarydescriptor : super::winnt::HANDLE));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenPrivateNamespaceW(lpboundarydescriptor : *const core::ffi::c_void, lpaliasprefix : windows_sys::core::PCWSTR) -> super::winnt::HANDLE);
pub const PRIVATE_NAMESPACE_FLAG_DESTROY: u32 = 1;
