#[cfg(feature = "ncrypt")]
windows_link::link!("ncrypt.dll" "system" fn NCryptCloseProtectionDescriptor(hdescriptor : NCRYPT_DESCRIPTOR_HANDLE) -> super::SECURITY_STATUS);
#[cfg(feature = "ncrypt")]
windows_link::link!("ncrypt.dll" "system" fn NCryptCreateProtectionDescriptor(pwszdescriptorstring : windows_sys::core::PCWSTR, dwflags : u32, phdescriptor : *mut NCRYPT_DESCRIPTOR_HANDLE) -> super::SECURITY_STATUS);
#[cfg(feature = "ncrypt")]
windows_link::link!("ncrypt.dll" "system" fn NCryptGetProtectionDescriptorInfo(hdescriptor : NCRYPT_DESCRIPTOR_HANDLE, pmempara : *const super::NCRYPT_ALLOC_PARA, dwinfotype : u32, ppvinfo : *mut *mut core::ffi::c_void) -> super::SECURITY_STATUS);
#[cfg(all(feature = "ncrypt", feature = "windef"))]
windows_link::link!("ncrypt.dll" "system" fn NCryptProtectSecret(hdescriptor : NCRYPT_DESCRIPTOR_HANDLE, dwflags : u32, pbdata : *const u8, cbdata : u32, pmempara : *const super::NCRYPT_ALLOC_PARA, hwnd : super::HWND, ppbprotectedblob : *mut *mut u8, pcbprotectedblob : *mut u32) -> super::SECURITY_STATUS);
#[cfg(feature = "ncrypt")]
windows_link::link!("ncrypt.dll" "system" fn NCryptQueryProtectionDescriptorName(pwszname : windows_sys::core::PCWSTR, pwszdescriptorstring : windows_sys::core::PWSTR, pcdescriptorstring : *mut usize, dwflags : u32) -> super::SECURITY_STATUS);
#[cfg(feature = "ncrypt")]
windows_link::link!("ncrypt.dll" "system" fn NCryptRegisterProtectionDescriptorName(pwszname : windows_sys::core::PCWSTR, pwszdescriptorstring : windows_sys::core::PCWSTR, dwflags : u32) -> super::SECURITY_STATUS);
#[cfg(feature = "ncrypt")]
windows_link::link!("ncrypt.dll" "system" fn NCryptStreamClose(hstream : NCRYPT_STREAM_HANDLE) -> super::SECURITY_STATUS);
#[cfg(all(feature = "ncrypt", feature = "windef"))]
windows_link::link!("ncrypt.dll" "system" fn NCryptStreamOpenToProtect(hdescriptor : NCRYPT_DESCRIPTOR_HANDLE, dwflags : u32, hwnd : super::HWND, pstreaminfo : *const NCRYPT_PROTECT_STREAM_INFO, phstream : *mut NCRYPT_STREAM_HANDLE) -> super::SECURITY_STATUS);
#[cfg(all(feature = "ncrypt", feature = "windef"))]
windows_link::link!("ncrypt.dll" "system" fn NCryptStreamOpenToUnprotect(pstreaminfo : *const NCRYPT_PROTECT_STREAM_INFO, dwflags : u32, hwnd : super::HWND, phstream : *mut NCRYPT_STREAM_HANDLE) -> super::SECURITY_STATUS);
#[cfg(all(feature = "ncrypt", feature = "windef"))]
windows_link::link!("ncrypt.dll" "system" fn NCryptStreamOpenToUnprotectEx(pstreaminfo : *const NCRYPT_PROTECT_STREAM_INFO_EX, dwflags : u32, hwnd : super::HWND, phstream : *mut NCRYPT_STREAM_HANDLE) -> super::SECURITY_STATUS);
#[cfg(feature = "ncrypt")]
windows_link::link!("ncrypt.dll" "system" fn NCryptStreamUpdate(hstream : NCRYPT_STREAM_HANDLE, pbdata : *const u8, cbdata : usize, ffinal : windows_sys::core::BOOL) -> super::SECURITY_STATUS);
#[cfg(all(feature = "ncrypt", feature = "windef"))]
windows_link::link!("ncrypt.dll" "system" fn NCryptUnprotectSecret(phdescriptor : *mut NCRYPT_DESCRIPTOR_HANDLE, dwflags : u32, pbprotectedblob : *const u8, cbprotectedblob : u32, pmempara : *const super::NCRYPT_ALLOC_PARA, hwnd : super::HWND, ppbdata : *mut *mut u8, pcbdata : *mut u32) -> super::SECURITY_STATUS);
pub const MS_KEY_PROTECTION_PROVIDER: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft Key Protection Provider");
pub type NCRYPT_DESCRIPTOR_HANDLE = *mut core::ffi::c_void;
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
pub const NCRYPT_NAMED_DESCRIPTOR_FLAG: u32 = 1;
pub const NCRYPT_PROTECTION_INFO_TYPE_DESCRIPTOR_STRING: u32 = 1;
#[repr(C)]
#[cfg(feature = "ncrypt")]
#[derive(Clone, Copy)]
pub struct NCRYPT_PROTECT_STREAM_INFO {
    pub pfnStreamOutput: PFNCryptStreamOutputCallback,
    pub pvCallbackCtxt: *mut core::ffi::c_void,
}
#[cfg(feature = "ncrypt")]
impl Default for NCRYPT_PROTECT_STREAM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ncrypt")]
#[derive(Clone, Copy)]
pub struct NCRYPT_PROTECT_STREAM_INFO_EX {
    pub pfnStreamOutput: PFNCryptStreamOutputCallbackEx,
    pub pvCallbackCtxt: *mut core::ffi::c_void,
}
#[cfg(feature = "ncrypt")]
impl Default for NCRYPT_PROTECT_STREAM_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NCRYPT_STREAM_HANDLE = *mut core::ffi::c_void;
pub const NCRYPT_UNPROTECT_NO_DECRYPT: u32 = 1;
#[cfg(feature = "ncrypt")]
pub type PFNCryptStreamOutputCallback = Option<unsafe extern "system" fn(pvcallbackctxt: *const core::ffi::c_void, pbdata: *const u8, cbdata: usize, ffinal: windows_sys::core::BOOL) -> super::SECURITY_STATUS>;
#[cfg(feature = "ncrypt")]
pub type PFNCryptStreamOutputCallbackEx = Option<unsafe extern "system" fn(pvcallbackctxt: *const core::ffi::c_void, pbdata: *const u8, cbdata: usize, hdescriptor: NCRYPT_DESCRIPTOR_HANDLE, ffinal: windows_sys::core::BOOL) -> super::SECURITY_STATUS>;
