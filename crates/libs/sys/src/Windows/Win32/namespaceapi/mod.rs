#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn AddSIDToBoundaryDescriptor(boundarydescriptor : *mut super::HANDLE, requiredsid : super::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ClosePrivateNamespace(handle : super::HANDLE, flags : u32) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateBoundaryDescriptorW(name : windows_sys::core::PCWSTR, flags : u32) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreatePrivateNamespaceW(lpprivatenamespaceattributes : *const super::SECURITY_ATTRIBUTES, lpboundarydescriptor : *const core::ffi::c_void, lpaliasprefix : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DeleteBoundaryDescriptor(boundarydescriptor : super::HANDLE));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenPrivateNamespaceW(lpboundarydescriptor : *const core::ffi::c_void, lpaliasprefix : windows_sys::core::PCWSTR) -> super::HANDLE);
pub const PRIVATE_NAMESPACE_FLAG_DESTROY: u32 = 1;
