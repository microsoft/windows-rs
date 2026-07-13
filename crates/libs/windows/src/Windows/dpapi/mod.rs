#[cfg(all(feature = "wincrypt", feature = "windef"))]
#[inline]
pub unsafe fn CryptProtectData<P1>(pdatain: *const super::wincrypt::DATA_BLOB, szdatadescr: P1, poptionalentropy: Option<*const super::wincrypt::DATA_BLOB>, pvreserved: Option<*const core::ffi::c_void>, ppromptstruct: Option<*const CRYPTPROTECT_PROMPTSTRUCT>, dwflags: u32, pdataout: *mut super::wincrypt::DATA_BLOB) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptProtectData(pdatain : *const super::wincrypt::DATA_BLOB, szdatadescr : windows_core::PCWSTR, poptionalentropy : *const super::wincrypt::DATA_BLOB, pvreserved : *const core::ffi::c_void, ppromptstruct : *const CRYPTPROTECT_PROMPTSTRUCT, dwflags : u32, pdataout : *mut super::wincrypt::DATA_BLOB) -> windows_core::BOOL);
    unsafe { CryptProtectData(pdatain, szdatadescr.param().abi(), poptionalentropy.unwrap_or(core::mem::zeroed()) as _, pvreserved.unwrap_or(core::mem::zeroed()) as _, ppromptstruct.unwrap_or(core::mem::zeroed()) as _, dwflags, pdataout as _) }
}
#[inline]
pub unsafe fn CryptProtectMemory(pdatain: *mut core::ffi::c_void, cbdatain: u32, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptProtectMemory(pdatain : *mut core::ffi::c_void, cbdatain : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptProtectMemory(pdatain as _, cbdatain, dwflags) }
}
#[cfg(all(feature = "wincrypt", feature = "windef"))]
#[inline]
pub unsafe fn CryptUnprotectData(pdatain: *const super::wincrypt::DATA_BLOB, ppszdatadescr: *mut windows_core::PWSTR, poptionalentropy: Option<*const super::wincrypt::DATA_BLOB>, pvreserved: Option<*const core::ffi::c_void>, ppromptstruct: Option<*const CRYPTPROTECT_PROMPTSTRUCT>, dwflags: u32, pdataout: *mut super::wincrypt::DATA_BLOB) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptUnprotectData(pdatain : *const super::wincrypt::DATA_BLOB, ppszdatadescr : *mut windows_core::PWSTR, poptionalentropy : *const super::wincrypt::DATA_BLOB, pvreserved : *const core::ffi::c_void, ppromptstruct : *const CRYPTPROTECT_PROMPTSTRUCT, dwflags : u32, pdataout : *mut super::wincrypt::DATA_BLOB) -> windows_core::BOOL);
    unsafe { CryptUnprotectData(pdatain, ppszdatadescr as _, poptionalentropy.unwrap_or(core::mem::zeroed()) as _, pvreserved.unwrap_or(core::mem::zeroed()) as _, ppromptstruct.unwrap_or(core::mem::zeroed()) as _, dwflags, pdataout as _) }
}
#[inline]
pub unsafe fn CryptUnprotectMemory(pdatain: *mut core::ffi::c_void, cbdatain: u32, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptUnprotectMemory(pdatain : *mut core::ffi::c_void, cbdatain : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptUnprotectMemory(pdatain as _, cbdatain, dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptUpdateProtectedState<P1>(poldsid: Option<super::winnt::PSID>, pwszoldpassword: P1, dwflags: u32, pdwsuccesscount: Option<*mut u32>, pdwfailurecount: Option<*mut u32>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptUpdateProtectedState(poldsid : super::winnt::PSID, pwszoldpassword : windows_core::PCWSTR, dwflags : u32, pdwsuccesscount : *mut u32, pdwfailurecount : *mut u32) -> windows_core::BOOL);
    unsafe { CryptUpdateProtectedState(poldsid.unwrap_or(core::mem::zeroed()) as _, pwszoldpassword.param().abi(), dwflags, pdwsuccesscount.unwrap_or(core::mem::zeroed()) as _, pdwfailurecount.unwrap_or(core::mem::zeroed()) as _) }
}
pub const CRYPTPROTECTMEMORY_BLOCK_SIZE: u32 = 16;
pub const CRYPTPROTECTMEMORY_CROSS_PROCESS: u32 = 1;
pub const CRYPTPROTECTMEMORY_SAME_LOGON: u32 = 2;
pub const CRYPTPROTECTMEMORY_SAME_PROCESS: u32 = 0;
pub const CRYPTPROTECT_AUDIT: u32 = 16;
pub const CRYPTPROTECT_CRED_REGENERATE: u32 = 128;
pub const CRYPTPROTECT_CRED_SYNC: u32 = 8;
pub const CRYPTPROTECT_FIRST_RESERVED_FLAGVAL: u32 = 268435455;
pub const CRYPTPROTECT_LAST_RESERVED_FLAGVAL: u32 = 4294967295;
pub const CRYPTPROTECT_LOCAL_MACHINE: u32 = 4;
pub const CRYPTPROTECT_NO_RECOVERY: u32 = 32;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPTPROTECT_PROMPTSTRUCT {
    pub cbSize: u32,
    pub dwPromptFlags: u32,
    pub hwndApp: super::windef::HWND,
    pub szPrompt: windows_core::PCWSTR,
}
pub const CRYPTPROTECT_PROMPT_ON_PROTECT: u32 = 2;
pub const CRYPTPROTECT_PROMPT_ON_UNPROTECT: u32 = 1;
pub const CRYPTPROTECT_PROMPT_REQUIRE_STRONG: u32 = 16;
pub const CRYPTPROTECT_PROMPT_RESERVED: u32 = 4;
pub const CRYPTPROTECT_PROMPT_STRONG: u32 = 8;
pub const CRYPTPROTECT_UI_FORBIDDEN: u32 = 1;
pub const CRYPTPROTECT_VERIFY_PROTECTION: u32 = 64;
#[cfg(feature = "windef")]
pub type PCRYPTPROTECT_PROMPTSTRUCT = *mut CRYPTPROTECT_PROMPTSTRUCT;
pub const dwFORCE_KEY_PROTECTION_DISABLED: u32 = 0;
pub const dwFORCE_KEY_PROTECTION_HIGH: u32 = 2;
pub const dwFORCE_KEY_PROTECTION_USER_SELECT: u32 = 1;
pub const szFORCE_KEY_PROTECTION: windows_core::PCSTR = windows_core::s!("ForceKeyProtection");
