#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ATOM(pub u16);
pub const FALSE: u32 = 0;
#[cfg(target_arch = "x86")]
pub type FARPROC = Option<unsafe extern "system" fn() -> i32>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
#[cfg(feature = "winnt")]
pub type GLOBALHANDLE = super::winnt::HANDLE;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HFILE(pub i32);
#[cfg(feature = "winnt")]
pub type HGLOBAL = super::winnt::HANDLE;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HINSTANCE(pub *mut core::ffi::c_void);
impl HINSTANCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HINSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HKEY(pub *mut core::ffi::c_void);
impl HKEY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HKL(pub *mut core::ffi::c_void);
impl HKL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HKL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub type HLOCAL = super::winnt::HANDLE;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HLSURF(pub *mut core::ffi::c_void);
impl HLSURF {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HLSURF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMETAFILE(pub *mut core::ffi::c_void);
impl HMETAFILE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HMETAFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HMODULE = HINSTANCE;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HRGN(pub *mut core::ffi::c_void);
impl HRGN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HRGN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HRSRC(pub *mut core::ffi::c_void);
impl HRSRC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HRSRC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSPRITE(pub *mut core::ffi::c_void);
impl HSPRITE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HSPRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSTR(pub *mut core::ffi::c_void);
impl HSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HTASK(pub *mut core::ffi::c_void);
impl HTASK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HTASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HWINSTA(pub *mut core::ffi::c_void);
impl HWINSTA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HWINSTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub type LOCALHANDLE = super::winnt::HANDLE;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPARAM(pub isize);
pub type LPBOOL = *mut windows_core::BOOL;
pub type LPBYTE = *mut u8;
pub type LPDWORD = *mut u32;
pub type LPFILETIME = *mut FILETIME;
#[cfg(feature = "winnt")]
pub type LPHANDLE = *mut super::winnt::HANDLE;
pub type LPINT = *mut i32;
pub type LPLONG = *mut i32;
pub type LPWORD = *mut u16;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LRESULT(pub isize);
pub const MAX_PATH: u32 = 260;
#[cfg(target_arch = "x86")]
pub type NEARPROC = Option<unsafe extern "system" fn() -> i32>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type NEARPROC = Option<unsafe extern "system" fn() -> isize>;
pub type PBOOL = *mut windows_core::BOOL;
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
#[cfg(feature = "winnt")]
pub type SPHANDLE = *mut super::winnt::HANDLE;
pub const STRICT: u32 = 1;
pub const TRUE: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WPARAM(pub usize);
