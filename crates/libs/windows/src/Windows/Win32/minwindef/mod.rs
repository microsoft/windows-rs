pub type ATOM = u16;
pub const FALSE: i32 = 0;
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
#[cfg(feature = "winnt")]
pub type GLOBALHANDLE = super::HANDLE;
pub type HFILE = i32;
#[cfg(feature = "winnt")]
pub type HGLOBAL = super::HANDLE;
pub type HINSTANCE = *mut HINSTANCE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HINSTANCE__ {
    pub unused: i32,
}
pub type HKEY = *mut HKEY__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HKEY__ {
    pub unused: i32,
}
pub type HKL = *mut HKL__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HKL__ {
    pub unused: i32,
}
#[cfg(feature = "winnt")]
pub type HLOCAL = super::HANDLE;
pub type HLSURF = *mut HLSURF__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HLSURF__ {
    pub unused: i32,
}
pub type HMETAFILE = *mut HMETAFILE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HMETAFILE__ {
    pub unused: i32,
}
pub type HMODULE = HINSTANCE;
pub type HRGN = *mut HRGN__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HRGN__ {
    pub unused: i32,
}
pub type HRSRC = *mut HRSRC__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HRSRC__ {
    pub unused: i32,
}
pub type HSPRITE = *mut HSPRITE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HSPRITE__ {
    pub unused: i32,
}
pub type HSTR = *mut HSTR__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HSTR__ {
    pub unused: i32,
}
pub type HTASK = *mut HTASK__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTASK__ {
    pub unused: i32,
}
pub type HWINSTA = *mut HWINSTA__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HWINSTA__ {
    pub unused: i32,
}
#[cfg(feature = "winnt")]
pub type LOCALHANDLE = super::HANDLE;
pub type LPARAM = isize;
pub type LPBOOL = *mut windows_core::BOOL;
pub type LPBYTE = *mut u8;
pub type LPCVOID = *const core::ffi::c_void;
pub type LPDWORD = *mut u32;
pub type LPFILETIME = *mut FILETIME;
#[cfg(feature = "winnt")]
pub type LPHANDLE = *mut super::HANDLE;
pub type LPINT = *mut i32;
pub type LPLONG = *mut i32;
pub type LPWORD = *mut u16;
pub type LRESULT = isize;
pub const MAX_PATH: i32 = 260;
pub type NEARPROC = Option<unsafe extern "system" fn() -> isize>;
pub type PBOOL = *mut windows_core::BOOL;
pub type PBYTE = *mut u8;
pub type PDWORD = *mut u32;
pub type PFILETIME = *mut FILETIME;
pub type PFLOAT = *mut f32;
pub type PHKEY = *mut HKEY;
pub type PINT = *mut i32;
pub type PROC = Option<unsafe extern "system" fn() -> isize>;
pub type PSZ = *mut i8;
pub type PUCHAR = *mut u8;
pub type PUINT = *mut u32;
pub type PULONG = *mut u32;
pub type PUSHORT = *mut u16;
pub type PWORD = *mut u16;
#[cfg(feature = "winnt")]
pub type SPHANDLE = *mut super::HANDLE;
pub const STRICT: i32 = 1;
pub const TRUE: i32 = 1;
pub type WPARAM = usize;
