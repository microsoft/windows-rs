#[cfg(all(feature = "wincrypt", feature = "windef"))]
windows_link::link!("crypt32.dll" "system" fn CryptProtectData(pdatain : *const super::DATA_BLOB, szdatadescr : windows_sys::core::PCWSTR, poptionalentropy : *const super::DATA_BLOB, pvreserved : *const core::ffi::c_void, ppromptstruct : *const CRYPTPROTECT_PROMPTSTRUCT, dwflags : u32, pdataout : *mut super::DATA_BLOB) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptProtectMemory(pdatain : *mut core::ffi::c_void, cbdatain : u32, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincrypt", feature = "windef"))]
windows_link::link!("crypt32.dll" "system" fn CryptUnprotectData(pdatain : *const super::DATA_BLOB, ppszdatadescr : *mut windows_sys::core::PWSTR, poptionalentropy : *const super::DATA_BLOB, pvreserved : *const core::ffi::c_void, ppromptstruct : *const CRYPTPROTECT_PROMPTSTRUCT, dwflags : u32, pdataout : *mut super::DATA_BLOB) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptUnprotectMemory(pdatain : *mut core::ffi::c_void, cbdatain : u32, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("crypt32.dll" "system" fn CryptUpdateProtectedState(poldsid : super::PSID, pwszoldpassword : windows_sys::core::PCWSTR, dwflags : u32, pdwsuccesscount : *mut u32, pdwfailurecount : *mut u32) -> windows_sys::core::BOOL);
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
#[derive(Clone, Copy)]
pub struct CRYPTPROTECT_PROMPTSTRUCT {
    pub cbSize: u32,
    pub dwPromptFlags: u32,
    pub hwndApp: super::HWND,
    pub szPrompt: windows_sys::core::PCWSTR,
}
#[cfg(feature = "windef")]
impl Default for CRYPTPROTECT_PROMPTSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
pub const szFORCE_KEY_PROTECTION: windows_sys::core::PCSTR = windows_sys::core::s!("ForceKeyProtection");
