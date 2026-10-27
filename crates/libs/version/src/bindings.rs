windows_link::link!("advapi32.dll" "system" fn RegGetValueA(hkey : HKEY, lpsubkey : PCSTR, lpvalue : PCSTR, dwflags : DWORD, pdwtype : LPDWORD, pvdata : PVOID, pcbdata : LPDWORD) -> LSTATUS);
windows_link::link!("ntdll.dll" "system" fn RtlGetVersion(lpversioninformation : PRTL_OSVERSIONINFOW) -> NTSTATUS);
pub type BYTE = u8;
pub type DWORD = u32;
pub type HKEY = *mut HKEY__;
pub const HKEY_LOCAL_MACHINE: HKEY = -2147483646 as _;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HKEY__ {
    pub unused: i32,
}
pub type LPDWORD = *mut u32;
pub type LSTATUS = i32;
pub type NTSTATUS = i32;
pub type OSVERSIONINFOEXW = _OSVERSIONINFOEXW;
pub type PCSTR = *const u8;
pub type PRTL_OSVERSIONINFOW = *mut _OSVERSIONINFOW;
pub type PVOID = *mut core::ffi::c_void;
pub const RRF_RT_REG_DWORD: i32 = 16;
pub const VER_NT_WORKSTATION: i32 = 1;
pub type WCHAR = u16;
pub type WORD = u16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _OSVERSIONINFOEXW {
    pub dwOSVersionInfoSize: DWORD,
    pub dwMajorVersion: DWORD,
    pub dwMinorVersion: DWORD,
    pub dwBuildNumber: DWORD,
    pub dwPlatformId: DWORD,
    pub szCSDVersion: [WCHAR; 128],
    pub wServicePackMajor: WORD,
    pub wServicePackMinor: WORD,
    pub wSuiteMask: WORD,
    pub wProductType: BYTE,
    pub wReserved: BYTE,
}
impl Default for _OSVERSIONINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _OSVERSIONINFOW {
    pub dwOSVersionInfoSize: DWORD,
    pub dwMajorVersion: DWORD,
    pub dwMinorVersion: DWORD,
    pub dwBuildNumber: DWORD,
    pub dwPlatformId: DWORD,
    pub szCSDVersion: [WCHAR; 128],
}
impl Default for _OSVERSIONINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
