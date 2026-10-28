#[cfg(feature = "ncrypt")]
#[inline]
pub unsafe fn NCryptCloseProtectionDescriptor(hdescriptor: NCRYPT_DESCRIPTOR_HANDLE) -> super::SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptCloseProtectionDescriptor(hdescriptor : NCRYPT_DESCRIPTOR_HANDLE) -> super::SECURITY_STATUS);
    unsafe { NCryptCloseProtectionDescriptor(hdescriptor) }
}
#[cfg(all(feature = "ncrypt", feature = "shared"))]
#[inline]
pub unsafe fn NCryptCreateProtectionDescriptor<P0>(pwszdescriptorstring: P0, dwflags: super::DWORD, phdescriptor: *mut NCRYPT_DESCRIPTOR_HANDLE) -> super::SECURITY_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptCreateProtectionDescriptor(pwszdescriptorstring : windows_core::PCWSTR, dwflags : super::DWORD, phdescriptor : *mut NCRYPT_DESCRIPTOR_HANDLE) -> super::SECURITY_STATUS);
    unsafe { NCryptCreateProtectionDescriptor(pwszdescriptorstring.param().abi(), dwflags, phdescriptor as _) }
}
#[cfg(all(feature = "minwindef", feature = "ncrypt", feature = "shared"))]
#[inline]
pub unsafe fn NCryptGetProtectionDescriptorInfo(hdescriptor: NCRYPT_DESCRIPTOR_HANDLE, pmempara: Option<*const super::NCRYPT_ALLOC_PARA>, dwinfotype: super::DWORD, ppvinfo: *mut *mut core::ffi::c_void) -> super::SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptGetProtectionDescriptorInfo(hdescriptor : NCRYPT_DESCRIPTOR_HANDLE, pmempara : *const super::NCRYPT_ALLOC_PARA, dwinfotype : super::DWORD, ppvinfo : *mut *mut core::ffi::c_void) -> super::SECURITY_STATUS);
    unsafe { NCryptGetProtectionDescriptorInfo(hdescriptor, pmempara.unwrap_or(core::mem::zeroed()) as _, dwinfotype, ppvinfo as _) }
}
#[cfg(all(feature = "minwindef", feature = "ncrypt", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn NCryptProtectSecret(hdescriptor: NCRYPT_DESCRIPTOR_HANDLE, dwflags: super::DWORD, pbdata: &[u8], pmempara: Option<*const super::NCRYPT_ALLOC_PARA>, hwnd: Option<super::HWND>, ppbprotectedblob: *mut *mut super::BYTE, pcbprotectedblob: *mut super::ULONG) -> super::SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptProtectSecret(hdescriptor : NCRYPT_DESCRIPTOR_HANDLE, dwflags : super::DWORD, pbdata : *const super::BYTE, cbdata : super::ULONG, pmempara : *const super::NCRYPT_ALLOC_PARA, hwnd : super::HWND, ppbprotectedblob : *mut *mut super::BYTE, pcbprotectedblob : *mut super::ULONG) -> super::SECURITY_STATUS);
    unsafe { NCryptProtectSecret(hdescriptor, dwflags, core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap(), pmempara.unwrap_or(core::mem::zeroed()) as _, hwnd.unwrap_or(core::mem::zeroed()) as _, ppbprotectedblob as _, pcbprotectedblob as _) }
}
#[cfg(all(feature = "ncrypt", feature = "shared"))]
#[inline]
pub unsafe fn NCryptQueryProtectionDescriptorName<P0>(pwszname: P0, pwszdescriptorstring: Option<windows_core::PWSTR>, pcdescriptorstring: *mut super::SIZE_T, dwflags: super::DWORD) -> super::SECURITY_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptQueryProtectionDescriptorName(pwszname : windows_core::PCWSTR, pwszdescriptorstring : windows_core::PWSTR, pcdescriptorstring : *mut super::SIZE_T, dwflags : super::DWORD) -> super::SECURITY_STATUS);
    unsafe { NCryptQueryProtectionDescriptorName(pwszname.param().abi(), pwszdescriptorstring.unwrap_or(core::mem::zeroed()) as _, pcdescriptorstring as _, dwflags) }
}
#[cfg(all(feature = "ncrypt", feature = "shared"))]
#[inline]
pub unsafe fn NCryptRegisterProtectionDescriptorName<P0, P1>(pwszname: P0, pwszdescriptorstring: P1, dwflags: super::DWORD) -> super::SECURITY_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptRegisterProtectionDescriptorName(pwszname : windows_core::PCWSTR, pwszdescriptorstring : windows_core::PCWSTR, dwflags : super::DWORD) -> super::SECURITY_STATUS);
    unsafe { NCryptRegisterProtectionDescriptorName(pwszname.param().abi(), pwszdescriptorstring.param().abi(), dwflags) }
}
#[cfg(feature = "ncrypt")]
#[inline]
pub unsafe fn NCryptStreamClose(hstream: NCRYPT_STREAM_HANDLE) -> super::SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptStreamClose(hstream : NCRYPT_STREAM_HANDLE) -> super::SECURITY_STATUS);
    unsafe { NCryptStreamClose(hstream) }
}
#[cfg(all(feature = "ncrypt", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn NCryptStreamOpenToProtect(hdescriptor: NCRYPT_DESCRIPTOR_HANDLE, dwflags: super::DWORD, hwnd: Option<super::HWND>, pstreaminfo: *const NCRYPT_PROTECT_STREAM_INFO, phstream: *mut NCRYPT_STREAM_HANDLE) -> super::SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptStreamOpenToProtect(hdescriptor : NCRYPT_DESCRIPTOR_HANDLE, dwflags : super::DWORD, hwnd : super::HWND, pstreaminfo : *const NCRYPT_PROTECT_STREAM_INFO, phstream : *mut NCRYPT_STREAM_HANDLE) -> super::SECURITY_STATUS);
    unsafe { NCryptStreamOpenToProtect(hdescriptor, dwflags, hwnd.unwrap_or(core::mem::zeroed()) as _, pstreaminfo, phstream as _) }
}
#[cfg(all(feature = "ncrypt", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn NCryptStreamOpenToUnprotect(pstreaminfo: *const NCRYPT_PROTECT_STREAM_INFO, dwflags: super::DWORD, hwnd: Option<super::HWND>, phstream: *mut NCRYPT_STREAM_HANDLE) -> super::SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptStreamOpenToUnprotect(pstreaminfo : *const NCRYPT_PROTECT_STREAM_INFO, dwflags : super::DWORD, hwnd : super::HWND, phstream : *mut NCRYPT_STREAM_HANDLE) -> super::SECURITY_STATUS);
    unsafe { NCryptStreamOpenToUnprotect(pstreaminfo, dwflags, hwnd.unwrap_or(core::mem::zeroed()) as _, phstream as _) }
}
#[cfg(all(feature = "ncrypt", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn NCryptStreamOpenToUnprotectEx(pstreaminfo: *const NCRYPT_PROTECT_STREAM_INFO_EX, dwflags: super::DWORD, hwnd: Option<super::HWND>, phstream: *mut NCRYPT_STREAM_HANDLE) -> super::SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptStreamOpenToUnprotectEx(pstreaminfo : *const NCRYPT_PROTECT_STREAM_INFO_EX, dwflags : super::DWORD, hwnd : super::HWND, phstream : *mut NCRYPT_STREAM_HANDLE) -> super::SECURITY_STATUS);
    unsafe { NCryptStreamOpenToUnprotectEx(pstreaminfo, dwflags, hwnd.unwrap_or(core::mem::zeroed()) as _, phstream as _) }
}
#[cfg(all(feature = "ncrypt", feature = "shared"))]
#[inline]
pub unsafe fn NCryptStreamUpdate(hstream: NCRYPT_STREAM_HANDLE, pbdata: &[u8], ffinal: bool) -> super::SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptStreamUpdate(hstream : NCRYPT_STREAM_HANDLE, pbdata : *const super::BYTE, cbdata : super::SIZE_T, ffinal : windows_core::BOOL) -> super::SECURITY_STATUS);
    unsafe { NCryptStreamUpdate(hstream, core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap(), ffinal.into()) }
}
#[cfg(all(feature = "minwindef", feature = "ncrypt", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn NCryptUnprotectSecret(phdescriptor: Option<*mut NCRYPT_DESCRIPTOR_HANDLE>, dwflags: super::DWORD, pbprotectedblob: &[u8], pmempara: Option<*const super::NCRYPT_ALLOC_PARA>, hwnd: Option<super::HWND>, ppbdata: *mut *mut super::BYTE, pcbdata: *mut super::ULONG) -> super::SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptUnprotectSecret(phdescriptor : *mut NCRYPT_DESCRIPTOR_HANDLE, dwflags : super::DWORD, pbprotectedblob : *const super::BYTE, cbprotectedblob : super::ULONG, pmempara : *const super::NCRYPT_ALLOC_PARA, hwnd : super::HWND, ppbdata : *mut *mut super::BYTE, pcbdata : *mut super::ULONG) -> super::SECURITY_STATUS);
    unsafe { NCryptUnprotectSecret(phdescriptor.unwrap_or(core::mem::zeroed()) as _, dwflags, core::mem::transmute(pbprotectedblob.as_ptr()), pbprotectedblob.len().try_into().unwrap(), pmempara.unwrap_or(core::mem::zeroed()) as _, hwnd.unwrap_or(core::mem::zeroed()) as _, ppbdata as _, pcbdata as _) }
}
pub const MS_KEY_PROTECTION_PROVIDER: windows_core::PCWSTR = windows_core::w!("Microsoft Key Protection Provider");
pub type NCRYPT_DESCRIPTOR_HANDLE = *mut NCRYPT_DESCRIPTOR_HANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NCRYPT_DESCRIPTOR_HANDLE__ {
    pub unused: i32,
}
pub const NCRYPT_DESCR_DELIMITER_AND: windows_core::PCWSTR = windows_core::w!("AND");
pub const NCRYPT_DESCR_DELIMITER_OR: windows_core::PCWSTR = windows_core::w!("OR");
pub const NCRYPT_DESCR_EQUAL: windows_core::PCWSTR = windows_core::w!("=");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_CERTIFICATE: windows_core::PCWSTR = windows_core::w!("CERTIFICATE");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_LOCAL: windows_core::PCWSTR = windows_core::w!("LOCAL");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_LOCKEDCREDENTIALS: windows_core::PCWSTR = windows_core::w!("LOCKEDCREDENTIALS");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_SDDL: windows_core::PCWSTR = windows_core::w!("SDDL");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_SID: windows_core::PCWSTR = windows_core::w!("SID");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_WEBCREDENTIALS: windows_core::PCWSTR = windows_core::w!("WEBCREDENTIALS");
pub const NCRYPT_KEY_PROTECTION_CERT_CERTBLOB: windows_core::PCWSTR = windows_core::w!("CertBlob");
pub const NCRYPT_KEY_PROTECTION_CERT_HASHID: windows_core::PCWSTR = windows_core::w!("HashId");
pub const NCRYPT_KEY_PROTECTION_LOCAL_LOGON: windows_core::PCWSTR = windows_core::w!("logon");
pub const NCRYPT_KEY_PROTECTION_LOCAL_MACHINE: windows_core::PCWSTR = windows_core::w!("machine");
pub const NCRYPT_KEY_PROTECTION_LOCAL_USER: windows_core::PCWSTR = windows_core::w!("user");
pub const NCRYPT_NAMED_DESCRIPTOR_FLAG: i32 = 1;
pub const NCRYPT_PROTECTION_INFO_TYPE_DESCRIPTOR_STRING: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "ncrypt", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct NCRYPT_PROTECT_STREAM_INFO {
    pub pfnStreamOutput: PFNCryptStreamOutputCallback,
    pub pvCallbackCtxt: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(all(feature = "ncrypt", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct NCRYPT_PROTECT_STREAM_INFO_EX {
    pub pfnStreamOutput: PFNCryptStreamOutputCallbackEx,
    pub pvCallbackCtxt: *mut core::ffi::c_void,
}
pub type NCRYPT_STREAM_HANDLE = *mut NCRYPT_STREAM_HANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NCRYPT_STREAM_HANDLE__ {
    pub unused: i32,
}
pub const NCRYPT_UNPROTECT_NO_DECRYPT: i32 = 1;
#[cfg(all(feature = "ncrypt", feature = "shared"))]
pub type PFNCryptStreamOutputCallback = Option<unsafe extern "system" fn(pvcallbackctxt: *const core::ffi::c_void, pbdata: *const super::BYTE, cbdata: super::SIZE_T, ffinal: windows_core::BOOL) -> super::SECURITY_STATUS>;
#[cfg(all(feature = "ncrypt", feature = "shared"))]
pub type PFNCryptStreamOutputCallbackEx = Option<unsafe extern "system" fn(pvcallbackctxt: *const core::ffi::c_void, pbdata: *const super::BYTE, cbdata: super::SIZE_T, hdescriptor: NCRYPT_DESCRIPTOR_HANDLE, ffinal: windows_core::BOOL) -> super::SECURITY_STATUS>;
