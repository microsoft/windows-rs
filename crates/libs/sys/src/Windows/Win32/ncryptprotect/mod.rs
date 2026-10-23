#[cfg(feature = "ncrypt")]
windows_link::link!("ncrypt.dll" "system" fn NCryptCloseProtectionDescriptor(hdescriptor : NCRYPT_DESCRIPTOR_HANDLE) -> super::SECURITY_STATUS);
#[cfg(all(feature = "ncrypt", feature = "shared"))]
windows_link::link!("ncrypt.dll" "system" fn NCryptCreateProtectionDescriptor(pwszdescriptorstring : windows_sys::core::PCWSTR, dwflags : super::DWORD, phdescriptor : *mut NCRYPT_DESCRIPTOR_HANDLE) -> super::SECURITY_STATUS);
#[cfg(all(feature = "minwindef", feature = "ncrypt", feature = "shared"))]
windows_link::link!("ncrypt.dll" "system" fn NCryptGetProtectionDescriptorInfo(hdescriptor : NCRYPT_DESCRIPTOR_HANDLE, pmempara : *const super::NCRYPT_ALLOC_PARA, dwinfotype : super::DWORD, ppvinfo : *mut *mut core::ffi::c_void) -> super::SECURITY_STATUS);
#[cfg(all(feature = "minwindef", feature = "ncrypt", feature = "shared", feature = "windef"))]
windows_link::link!("ncrypt.dll" "system" fn NCryptProtectSecret(hdescriptor : NCRYPT_DESCRIPTOR_HANDLE, dwflags : super::DWORD, pbdata : *const super::BYTE, cbdata : super::ULONG, pmempara : *const super::NCRYPT_ALLOC_PARA, hwnd : super::HWND, ppbprotectedblob : *mut *mut super::BYTE, pcbprotectedblob : *mut super::ULONG) -> super::SECURITY_STATUS);
#[cfg(all(feature = "ncrypt", feature = "shared"))]
windows_link::link!("ncrypt.dll" "system" fn NCryptQueryProtectionDescriptorName(pwszname : windows_sys::core::PCWSTR, pwszdescriptorstring : windows_sys::core::PWSTR, pcdescriptorstring : *mut super::SIZE_T, dwflags : super::DWORD) -> super::SECURITY_STATUS);
#[cfg(all(feature = "ncrypt", feature = "shared"))]
windows_link::link!("ncrypt.dll" "system" fn NCryptRegisterProtectionDescriptorName(pwszname : windows_sys::core::PCWSTR, pwszdescriptorstring : windows_sys::core::PCWSTR, dwflags : super::DWORD) -> super::SECURITY_STATUS);
#[cfg(feature = "ncrypt")]
windows_link::link!("ncrypt.dll" "system" fn NCryptStreamClose(hstream : NCRYPT_STREAM_HANDLE) -> super::SECURITY_STATUS);
#[cfg(all(feature = "ncrypt", feature = "shared", feature = "windef"))]
windows_link::link!("ncrypt.dll" "system" fn NCryptStreamOpenToProtect(hdescriptor : NCRYPT_DESCRIPTOR_HANDLE, dwflags : super::DWORD, hwnd : super::HWND, pstreaminfo : *const NCRYPT_PROTECT_STREAM_INFO, phstream : *mut NCRYPT_STREAM_HANDLE) -> super::SECURITY_STATUS);
#[cfg(all(feature = "ncrypt", feature = "shared", feature = "windef"))]
windows_link::link!("ncrypt.dll" "system" fn NCryptStreamOpenToUnprotect(pstreaminfo : *const NCRYPT_PROTECT_STREAM_INFO, dwflags : super::DWORD, hwnd : super::HWND, phstream : *mut NCRYPT_STREAM_HANDLE) -> super::SECURITY_STATUS);
#[cfg(all(feature = "ncrypt", feature = "shared", feature = "windef"))]
windows_link::link!("ncrypt.dll" "system" fn NCryptStreamOpenToUnprotectEx(pstreaminfo : *const NCRYPT_PROTECT_STREAM_INFO_EX, dwflags : super::DWORD, hwnd : super::HWND, phstream : *mut NCRYPT_STREAM_HANDLE) -> super::SECURITY_STATUS);
#[cfg(all(feature = "ncrypt", feature = "shared"))]
windows_link::link!("ncrypt.dll" "system" fn NCryptStreamUpdate(hstream : NCRYPT_STREAM_HANDLE, pbdata : *const super::BYTE, cbdata : super::SIZE_T, ffinal : windows_sys::core::BOOL) -> super::SECURITY_STATUS);
#[cfg(all(feature = "minwindef", feature = "ncrypt", feature = "shared", feature = "windef"))]
windows_link::link!("ncrypt.dll" "system" fn NCryptUnprotectSecret(phdescriptor : *mut NCRYPT_DESCRIPTOR_HANDLE, dwflags : super::DWORD, pbprotectedblob : *const super::BYTE, cbprotectedblob : super::ULONG, pmempara : *const super::NCRYPT_ALLOC_PARA, hwnd : super::HWND, ppbdata : *mut *mut super::BYTE, pcbdata : *mut super::ULONG) -> super::SECURITY_STATUS);
pub const MS_KEY_PROTECTION_PROVIDER: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft Key Protection Provider");
pub type NCRYPT_DESCRIPTOR_HANDLE = *mut NCRYPT_DESCRIPTOR_HANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NCRYPT_DESCRIPTOR_HANDLE__ {
    pub unused: i32,
}
pub const NCRYPT_DESCR_DELIMITER_AND: windows_sys::core::PCWSTR = windows_sys::core::w!("AND");
pub const NCRYPT_DESCR_DELIMITER_OR: windows_sys::core::PCWSTR = windows_sys::core::w!("OR");
pub const NCRYPT_DESCR_EQUAL: windows_sys::core::PCWSTR = windows_sys::core::w!("=");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_CERTIFICATE: windows_sys::core::PCWSTR = windows_sys::core::w!("CERTIFICATE");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_LOCAL: windows_sys::core::PCWSTR = windows_sys::core::w!("LOCAL");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_LOCKEDCREDENTIALS: windows_sys::core::PCWSTR = windows_sys::core::w!("LOCKEDCREDENTIALS");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_SDDL: windows_sys::core::PCWSTR = windows_sys::core::w!("SDDL");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_SID: windows_sys::core::PCWSTR = windows_sys::core::w!("SID");
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_WEBCREDENTIALS: windows_sys::core::PCWSTR = windows_sys::core::w!("WEBCREDENTIALS");
pub const NCRYPT_KEY_PROTECTION_CERT_CERTBLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("CertBlob");
pub const NCRYPT_KEY_PROTECTION_CERT_HASHID: windows_sys::core::PCWSTR = windows_sys::core::w!("HashId");
pub const NCRYPT_KEY_PROTECTION_LOCAL_LOGON: windows_sys::core::PCWSTR = windows_sys::core::w!("logon");
pub const NCRYPT_KEY_PROTECTION_LOCAL_MACHINE: windows_sys::core::PCWSTR = windows_sys::core::w!("machine");
pub const NCRYPT_KEY_PROTECTION_LOCAL_USER: windows_sys::core::PCWSTR = windows_sys::core::w!("user");
pub const NCRYPT_NAMED_DESCRIPTOR_FLAG: i32 = 1;
pub const NCRYPT_PROTECTION_INFO_TYPE_DESCRIPTOR_STRING: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "ncrypt", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct NCRYPT_PROTECT_STREAM_INFO {
    pub pfnStreamOutput: PFNCryptStreamOutputCallback,
    pub pvCallbackCtxt: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(all(feature = "ncrypt", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct NCRYPT_PROTECT_STREAM_INFO_EX {
    pub pfnStreamOutput: PFNCryptStreamOutputCallbackEx,
    pub pvCallbackCtxt: *mut core::ffi::c_void,
}
pub type NCRYPT_STREAM_HANDLE = *mut NCRYPT_STREAM_HANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NCRYPT_STREAM_HANDLE__ {
    pub unused: i32,
}
pub const NCRYPT_UNPROTECT_NO_DECRYPT: i32 = 1;
#[cfg(all(feature = "ncrypt", feature = "shared"))]
pub type PFNCryptStreamOutputCallback = Option<unsafe extern "system" fn(pvcallbackctxt: *const core::ffi::c_void, pbdata: *const super::BYTE, cbdata: super::SIZE_T, ffinal: windows_sys::core::BOOL) -> super::SECURITY_STATUS>;
#[cfg(all(feature = "ncrypt", feature = "shared"))]
pub type PFNCryptStreamOutputCallbackEx = Option<unsafe extern "system" fn(pvcallbackctxt: *const core::ffi::c_void, pbdata: *const super::BYTE, cbdata: super::SIZE_T, hdescriptor: NCRYPT_DESCRIPTOR_HANDLE, ffinal: windows_sys::core::BOOL) -> super::SECURITY_STATUS>;
