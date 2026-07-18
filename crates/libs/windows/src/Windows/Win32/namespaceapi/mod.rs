#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddSIDToBoundaryDescriptor(boundarydescriptor: *mut super::HANDLE, requiredsid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AddSIDToBoundaryDescriptor(boundarydescriptor : *mut super::HANDLE, requiredsid : super::PSID) -> windows_core::BOOL);
    unsafe { AddSIDToBoundaryDescriptor(boundarydescriptor as _, requiredsid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ClosePrivateNamespace(handle: super::HANDLE, flags: u32) -> bool {
    windows_core::link!("kernel32.dll" "system" fn ClosePrivateNamespace(handle : super::HANDLE, flags : u32) -> bool);
    unsafe { ClosePrivateNamespace(handle, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateBoundaryDescriptorW<P0>(name: P0, flags: u32) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateBoundaryDescriptorW(name : windows_core::PCWSTR, flags : u32) -> super::HANDLE);
    unsafe { CreateBoundaryDescriptorW(name.param().abi(), flags) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreatePrivateNamespaceW<P2>(lpprivatenamespaceattributes: Option<*const super::SECURITY_ATTRIBUTES>, lpboundarydescriptor: *const core::ffi::c_void, lpaliasprefix: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreatePrivateNamespaceW(lpprivatenamespaceattributes : *const super::SECURITY_ATTRIBUTES, lpboundarydescriptor : *const core::ffi::c_void, lpaliasprefix : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { CreatePrivateNamespaceW(lpprivatenamespaceattributes.unwrap_or(core::mem::zeroed()) as _, lpboundarydescriptor, lpaliasprefix.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeleteBoundaryDescriptor(boundarydescriptor: super::HANDLE) {
    windows_core::link!("kernel32.dll" "system" fn DeleteBoundaryDescriptor(boundarydescriptor : super::HANDLE));
    unsafe { DeleteBoundaryDescriptor(boundarydescriptor) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn OpenPrivateNamespaceW<P1>(lpboundarydescriptor: *const core::ffi::c_void, lpaliasprefix: P1) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenPrivateNamespaceW(lpboundarydescriptor : *const core::ffi::c_void, lpaliasprefix : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { OpenPrivateNamespaceW(lpboundarydescriptor, lpaliasprefix.param().abi()) }
}
pub const PRIVATE_NAMESPACE_FLAG_DESTROY: u32 = 1;
