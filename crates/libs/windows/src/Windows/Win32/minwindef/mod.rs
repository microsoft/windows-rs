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
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GLOBALHANDLE(pub super::winnt::HANDLE);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HFILE(pub i32);
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HGLOBAL(pub super::winnt::HANDLE);
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
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HLOCAL(pub super::winnt::HANDLE);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HMODULE(pub HINSTANCE);
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
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LOCALHANDLE(pub super::winnt::HANDLE);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPARAM(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPBOOL(pub *mut windows_core::BOOL);
impl LPBOOL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPBOOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPBYTE(pub *mut u8);
impl LPBYTE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPBYTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDWORD(pub *mut u32);
impl LPDWORD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPFILETIME(pub *mut FILETIME);
impl LPFILETIME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPFILETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHANDLE(pub *mut super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
impl LPHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for LPHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINT(pub *mut i32);
impl LPINT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLONG(pub *mut i32);
impl LPLONG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWORD(pub *mut u16);
impl LPWORD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LRESULT(pub isize);
pub const MAX_PATH: u32 = 260;
#[cfg(target_arch = "x86")]
pub type NEARPROC = Option<unsafe extern "system" fn() -> i32>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type NEARPROC = Option<unsafe extern "system" fn() -> isize>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBOOL(pub *mut windows_core::BOOL);
impl PBOOL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBOOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBYTE(pub *mut u8);
impl PBYTE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBYTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDWORD(pub *mut u32);
impl PDWORD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFILETIME(pub *mut FILETIME);
impl PFILETIME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFILETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFLOAT(pub *mut f32);
impl PFLOAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFLOAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHKEY(pub *mut HKEY);
impl PHKEY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PINT(pub *mut i32);
impl PINT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub type PROC = Option<unsafe extern "system" fn() -> i32>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PROC = Option<unsafe extern "system" fn() -> isize>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSZ(pub *mut i8);
impl PSZ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSZ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUCHAR(pub *mut u8);
impl PUCHAR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUCHAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUINT(pub *mut u32);
impl PUINT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PULONG(pub *mut u32);
impl PULONG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PULONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSHORT(pub *mut u16);
impl PUSHORT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSHORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWORD(pub *mut u16);
impl PWORD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SPHANDLE(pub *mut super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
impl SPHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for SPHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STRICT: u32 = 1;
pub const TRUE: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WPARAM(pub usize);
