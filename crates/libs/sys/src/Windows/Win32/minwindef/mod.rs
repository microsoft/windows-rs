pub type ATOM = u16;
pub const FALSE: u32 = 0;
#[cfg(target_arch = "x86")]
pub type FARPROC = Option<unsafe extern "system" fn() -> i32>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
#[cfg(feature = "Win32_winnt")]
pub type GLOBALHANDLE = super::winnt::HANDLE;
pub type HFILE = i32;
#[cfg(feature = "Win32_winnt")]
pub type HGLOBAL = super::winnt::HANDLE;
pub type HINSTANCE = *mut core::ffi::c_void;
pub type HKEY = *mut core::ffi::c_void;
pub type HKL = *mut core::ffi::c_void;
#[cfg(feature = "Win32_winnt")]
pub type HLOCAL = super::winnt::HANDLE;
pub type HLSURF = *mut core::ffi::c_void;
pub type HMETAFILE = *mut core::ffi::c_void;
pub type HMODULE = HINSTANCE;
pub type HRGN = *mut core::ffi::c_void;
pub type HRSRC = *mut core::ffi::c_void;
pub type HSPRITE = *mut core::ffi::c_void;
pub type HSTR = *mut core::ffi::c_void;
pub type HTASK = *mut core::ffi::c_void;
pub type HWINSTA = *mut core::ffi::c_void;
#[cfg(feature = "Win32_winnt")]
pub type LOCALHANDLE = super::winnt::HANDLE;
pub type LPARAM = isize;
pub type LPBOOL = *mut windows_sys::core::BOOL;
pub type LPBYTE = *mut u8;
pub type LPDWORD = *mut u32;
pub type LPFILETIME = *mut FILETIME;
#[cfg(feature = "Win32_winnt")]
pub type LPHANDLE = *mut super::winnt::HANDLE;
pub type LPINT = *mut i32;
pub type LPLONG = *mut i32;
pub type LPWORD = *mut u16;
pub type LRESULT = isize;
pub const MAX_PATH: u32 = 260;
#[cfg(target_arch = "x86")]
pub type NEARPROC = Option<unsafe extern "system" fn() -> i32>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type NEARPROC = Option<unsafe extern "system" fn() -> isize>;
pub type PBOOL = *mut windows_sys::core::BOOL;
pub type PBYTE = *mut u8;
pub type PDWORD = *mut u32;
pub type PFILETIME = *mut FILETIME;
pub type PFLOAT = *mut f32;
pub type PHKEY = *mut HKEY;
pub type PINT = *mut i32;
#[cfg(target_arch = "x86")]
pub type PROC = Option<unsafe extern "system" fn() -> i32>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PROC = Option<unsafe extern "system" fn() -> isize>;
pub type PSZ = *mut i8;
pub type PUCHAR = *mut u8;
pub type PUINT = *mut u32;
pub type PULONG = *mut u32;
pub type PUSHORT = *mut u16;
pub type PWORD = *mut u16;
#[cfg(feature = "Win32_winnt")]
pub type SPHANDLE = *mut super::winnt::HANDLE;
pub const STRICT: u32 = 1;
pub const TRUE: u32 = 1;
pub type WPARAM = usize;
