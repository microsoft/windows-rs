pub type ATOM = u16;
pub const FALSE: i32 = 0;
#[cfg(target_arch = "x86")]
pub type FARPROC = Option<unsafe extern "system" fn() -> i32>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type FARPROC = Option<unsafe extern "system" fn() -> super::INT_PTR>;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct FILETIME {
    pub dwLowDateTime: super::DWORD,
    pub dwHighDateTime: super::DWORD,
}
pub type FLOAT = f32;
pub type GLOBALHANDLE = *mut core::ffi::c_void;
pub type HFILE = i32;
pub type HGLOBAL = *mut core::ffi::c_void;
pub type HINSTANCE = *mut HINSTANCE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HINSTANCE__ {
    pub unused: i32,
}
pub type HKEY = *mut HKEY__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HKEY__ {
    pub unused: i32,
}
pub type HKL = *mut HKL__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HKL__ {
    pub unused: i32,
}
pub type HLOCAL = *mut core::ffi::c_void;
pub type HLSURF = *mut HLSURF__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HLSURF__ {
    pub unused: i32,
}
pub type HMETAFILE = *mut HMETAFILE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HMETAFILE__ {
    pub unused: i32,
}
pub type HMODULE = *mut HINSTANCE__;
pub type HRGN = *mut HRGN__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HRGN__ {
    pub unused: i32,
}
pub type HRSRC = *mut HRSRC__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HRSRC__ {
    pub unused: i32,
}
pub type HSPRITE = *mut HSPRITE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HSPRITE__ {
    pub unused: i32,
}
pub type HSTR = *mut HSTR__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HSTR__ {
    pub unused: i32,
}
pub type HTASK = *mut HTASK__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTASK__ {
    pub unused: i32,
}
pub type HWINSTA = *mut HWINSTA__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HWINSTA__ {
    pub unused: i32,
}
pub type LOCALHANDLE = *mut core::ffi::c_void;
#[cfg(target_arch = "x86")]
pub type LPARAM = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type LPARAM = i64;
pub type LPBOOL = *mut i32;
pub type LPDWORD = *mut u32;
#[cfg(feature = "shared")]
pub type LPFILETIME = *mut FILETIME;
pub type LPHANDLE = *mut *mut core::ffi::c_void;
pub type LPINT = *mut i32;
pub type LPLONG = *mut i32;
pub type LPVOID = *mut core::ffi::c_void;
pub type LPWORD = *mut u16;
#[cfg(target_arch = "x86")]
pub type LRESULT = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type LRESULT = i64;
pub const MAX_PATH: i32 = 260;
#[cfg(target_arch = "x86")]
pub type NEARPROC = Option<unsafe extern "system" fn() -> i32>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type NEARPROC = Option<unsafe extern "system" fn() -> super::INT_PTR>;
pub type PBOOL = *mut i32;
pub type PDWORD = *mut u32;
#[cfg(feature = "shared")]
pub type PFILETIME = *mut FILETIME;
pub type PFLOAT = *mut f32;
pub type PHKEY = *mut *mut HKEY__;
pub type PINT = *mut i32;
#[cfg(target_arch = "x86")]
pub type PROC = Option<unsafe extern "system" fn() -> i32>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type PROC = Option<unsafe extern "system" fn() -> super::INT_PTR>;
pub type PSZ = *mut i8;
pub type PUCHAR = *mut u8;
pub type PUINT = *mut u32;
pub type PULONG = *mut u32;
pub type PUSHORT = *mut u16;
pub type PWORD = *mut u16;
pub type SPHANDLE = *mut *mut core::ffi::c_void;
pub const STRICT: i32 = 1;
pub const TRUE: i32 = 1;
#[cfg(target_arch = "x86")]
pub type WPARAM = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type WPARAM = u64;
