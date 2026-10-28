#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn AddSIDToBoundaryDescriptor(boundarydescriptor : *mut super::HANDLE, requiredsid : super::PSID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ClosePrivateNamespace(handle : super::HANDLE, flags : super::ULONG) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateBoundaryDescriptorW(name : windows_sys::core::PCWSTR, flags : super::ULONG) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreatePrivateNamespaceW(lpprivatenamespaceattributes : super::LPSECURITY_ATTRIBUTES, lpboundarydescriptor : super::LPVOID, lpaliasprefix : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DeleteBoundaryDescriptor(boundarydescriptor : super::HANDLE));
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn OpenPrivateNamespaceW(lpboundarydescriptor : super::LPVOID, lpaliasprefix : windows_sys::core::PCWSTR) -> super::HANDLE);
pub const PRIVATE_NAMESPACE_FLAG_DESTROY: i32 = 1;
