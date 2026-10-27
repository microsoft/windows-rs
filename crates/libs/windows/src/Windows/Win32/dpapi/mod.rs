#[cfg(all(feature = "shared", feature = "wincrypt", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn CryptProtectData<P1>(pdatain: *const super::DATA_BLOB, szdatadescr: P1, poptionalentropy: Option<*const super::DATA_BLOB>, pvreserved: Option<super::PVOID>, ppromptstruct: Option<*const CRYPTPROTECT_PROMPTSTRUCT>, dwflags: super::DWORD, pdataout: *mut super::DATA_BLOB) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptProtectData(pdatain : *const super::DATA_BLOB, szdatadescr : windows_core::PCWSTR, poptionalentropy : *const super::DATA_BLOB, pvreserved : super::PVOID, ppromptstruct : *const CRYPTPROTECT_PROMPTSTRUCT, dwflags : super::DWORD, pdataout : *mut super::DATA_BLOB) -> windows_core::BOOL);
    unsafe { CryptProtectData(pdatain, szdatadescr.param().abi(), poptionalentropy.unwrap_or(core::mem::zeroed()) as _, pvreserved.unwrap_or(core::mem::zeroed()) as _, ppromptstruct.unwrap_or(core::mem::zeroed()) as _, dwflags, pdataout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CryptProtectMemory(pdatain: super::LPVOID, cbdatain: super::DWORD, dwflags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptProtectMemory(pdatain : super::LPVOID, cbdatain : super::DWORD, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { CryptProtectMemory(pdatain as _, cbdatain, dwflags) }
}
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn CryptUnprotectData(pdatain: *const super::DATA_BLOB, ppszdatadescr: Option<*mut windows_core::PWSTR>, poptionalentropy: Option<*const super::DATA_BLOB>, pvreserved: Option<super::PVOID>, ppromptstruct: Option<*const CRYPTPROTECT_PROMPTSTRUCT>, dwflags: super::DWORD, pdataout: *mut super::DATA_BLOB) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptUnprotectData(pdatain : *const super::DATA_BLOB, ppszdatadescr : *mut windows_core::PWSTR, poptionalentropy : *const super::DATA_BLOB, pvreserved : super::PVOID, ppromptstruct : *const CRYPTPROTECT_PROMPTSTRUCT, dwflags : super::DWORD, pdataout : *mut super::DATA_BLOB) -> windows_core::BOOL);
    unsafe { CryptUnprotectData(pdatain, ppszdatadescr.unwrap_or(core::mem::zeroed()) as _, poptionalentropy.unwrap_or(core::mem::zeroed()) as _, pvreserved.unwrap_or(core::mem::zeroed()) as _, ppromptstruct.unwrap_or(core::mem::zeroed()) as _, dwflags, pdataout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CryptUnprotectMemory(pdatain: super::LPVOID, cbdatain: super::DWORD, dwflags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptUnprotectMemory(pdatain : super::LPVOID, cbdatain : super::DWORD, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { CryptUnprotectMemory(pdatain as _, cbdatain, dwflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CryptUpdateProtectedState<P1>(poldsid: Option<super::PSID>, pwszoldpassword: P1, dwflags: super::DWORD, pdwsuccesscount: Option<*mut super::DWORD>, pdwfailurecount: Option<*mut super::DWORD>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptUpdateProtectedState(poldsid : super::PSID, pwszoldpassword : windows_core::PCWSTR, dwflags : super::DWORD, pdwsuccesscount : *mut super::DWORD, pdwfailurecount : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { CryptUpdateProtectedState(poldsid.unwrap_or(core::mem::zeroed()) as _, pwszoldpassword.param().abi(), dwflags, pdwsuccesscount.unwrap_or(core::mem::zeroed()) as _, pdwfailurecount.unwrap_or(core::mem::zeroed()) as _) }
}
pub const CRYPTPROTECTMEMORY_BLOCK_SIZE: i32 = 16;
pub const CRYPTPROTECTMEMORY_CROSS_PROCESS: i32 = 1;
pub const CRYPTPROTECTMEMORY_SAME_LOGON: i32 = 2;
pub const CRYPTPROTECTMEMORY_SAME_PROCESS: i32 = 0;
pub const CRYPTPROTECT_AUDIT: i32 = 16;
pub const CRYPTPROTECT_CRED_REGENERATE: i32 = 128;
pub const CRYPTPROTECT_CRED_SYNC: i32 = 8;
pub const CRYPTPROTECT_FIRST_RESERVED_FLAGVAL: i32 = 268435455;
pub const CRYPTPROTECT_LAST_RESERVED_FLAGVAL: u32 = 4294967295;
pub const CRYPTPROTECT_LOCAL_MACHINE: i32 = 4;
pub const CRYPTPROTECT_NO_RECOVERY: i32 = 32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPTPROTECT_PROMPTSTRUCT {
    pub cbSize: super::DWORD,
    pub dwPromptFlags: super::DWORD,
    pub hwndApp: super::HWND,
    pub szPrompt: windows_core::PCWSTR,
}
pub const CRYPTPROTECT_PROMPT_ON_PROTECT: i32 = 2;
pub const CRYPTPROTECT_PROMPT_ON_UNPROTECT: i32 = 1;
pub const CRYPTPROTECT_PROMPT_REQUIRE_STRONG: i32 = 16;
pub const CRYPTPROTECT_PROMPT_RESERVED: i32 = 4;
pub const CRYPTPROTECT_PROMPT_STRONG: i32 = 8;
pub const CRYPTPROTECT_UI_FORBIDDEN: i32 = 1;
pub const CRYPTPROTECT_VERIFY_PROTECTION: i32 = 64;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PCRYPTPROTECT_PROMPTSTRUCT = *mut CRYPTPROTECT_PROMPTSTRUCT;
pub const dwFORCE_KEY_PROTECTION_DISABLED: i32 = 0;
pub const dwFORCE_KEY_PROTECTION_HIGH: i32 = 2;
pub const dwFORCE_KEY_PROTECTION_USER_SELECT: i32 = 1;
pub const szFORCE_KEY_PROTECTION: windows_core::PCSTR = windows_core::s!("ForceKeyProtection");
