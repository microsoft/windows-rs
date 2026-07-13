#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct APP_LOCAL_DEVICE_ID {
    pub value: [u8; 32],
}
impl Default for APP_LOCAL_DEVICE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const APP_LOCAL_DEVICE_ID_SIZE: u32 = 32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct COLORREF(pub u32);
pub const DC_BINNAMES: u32 = 12;
pub const DC_BINS: u32 = 6;
pub const DC_COPIES: u32 = 18;
pub const DC_DRIVER: u32 = 11;
pub const DC_DUPLEX: u32 = 7;
pub const DC_ENUMRESOLUTIONS: u32 = 13;
pub const DC_EXTRA: u32 = 9;
pub const DC_FIELDS: u32 = 1;
pub const DC_FILEDEPENDENCIES: u32 = 14;
pub const DC_MAXEXTENT: u32 = 5;
pub const DC_MINEXTENT: u32 = 4;
pub const DC_ORIENTATION: u32 = 17;
pub const DC_PAPERNAMES: u32 = 16;
pub const DC_PAPERS: u32 = 2;
pub const DC_PAPERSIZE: u32 = 3;
pub const DC_SIZE: u32 = 8;
pub const DC_TRUETYPE: u32 = 15;
pub const DC_VERSION: u32 = 10;
pub const DM_COPY: u32 = 2;
pub const DM_IN_BUFFER: u32 = 8;
pub const DM_IN_PROMPT: u32 = 4;
pub const DM_MODIFY: u32 = 8;
pub const DM_OUT_BUFFER: u32 = 2;
pub const DM_OUT_DEFAULT: u32 = 1;
pub const DM_PROMPT: u32 = 4;
pub const DM_UPDATE: u32 = 1;
pub type DPI_AWARENESS = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DPI_AWARENESS_CONTEXT(pub *mut core::ffi::c_void);
impl Default for DPI_AWARENESS_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-3 as _);
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-4 as _);
pub const DPI_AWARENESS_CONTEXT_SYSTEM_AWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-2 as _);
pub const DPI_AWARENESS_CONTEXT_UNAWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-1 as _);
pub const DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-5 as _);
pub const DPI_AWARENESS_INVALID: DPI_AWARENESS = -1;
pub const DPI_AWARENESS_PER_MONITOR_AWARE: DPI_AWARENESS = 2;
pub const DPI_AWARENESS_SYSTEM_AWARE: DPI_AWARENESS = 1;
pub const DPI_AWARENESS_UNAWARE: DPI_AWARENESS = 0;
pub type DPI_HOSTING_BEHAVIOR = i32;
pub const DPI_HOSTING_BEHAVIOR_DEFAULT: DPI_HOSTING_BEHAVIOR = 0;
pub const DPI_HOSTING_BEHAVIOR_INVALID: DPI_HOSTING_BEHAVIOR = -1;
pub const DPI_HOSTING_BEHAVIOR_MIXED: DPI_HOSTING_BEHAVIOR = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HACCEL(pub *mut core::ffi::c_void);
impl Default for HACCEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HBITMAP(pub *mut core::ffi::c_void);
impl Default for HBITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HBRUSH(pub *mut core::ffi::c_void);
impl Default for HBRUSH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCOLORSPACE(pub *mut core::ffi::c_void);
impl Default for HCOLORSPACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HCURSOR = HICON;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDC(pub *mut core::ffi::c_void);
impl Default for HDC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDESK(pub *mut core::ffi::c_void);
impl Default for HDESK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HENHMETAFILE(pub *mut core::ffi::c_void);
impl Default for HENHMETAFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
pub const HFILE_ERROR: super::minwindef::HFILE = super::minwindef::HFILE(0xFFFFFFFF_u32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HFONT(pub *mut core::ffi::c_void);
impl Default for HFONT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HGDIOBJ(pub *mut core::ffi::c_void);
impl Default for HGDIOBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HGLRC(pub *mut core::ffi::c_void);
impl Default for HGLRC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HHOOK(pub *mut core::ffi::c_void);
impl Default for HHOOK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HICON(pub *mut core::ffi::c_void);
impl Default for HICON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMENU(pub *mut core::ffi::c_void);
impl Default for HMENU {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMONITOR(pub *mut core::ffi::c_void);
impl Default for HMONITOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HPALETTE(pub *mut core::ffi::c_void);
impl Default for HPALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HPEN(pub *mut core::ffi::c_void);
impl Default for HPEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HUMPD(pub *mut core::ffi::c_void);
impl Default for HUMPD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HWINEVENTHOOK(pub *mut core::ffi::c_void);
impl Default for HWINEVENTHOOK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HWND(pub *mut core::ffi::c_void);
impl Default for HWND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPCOLORREF = *mut u32;
pub type LPCRECT = *const RECT;
pub type LPCRECTL = *const RECTL;
pub type LPPOINT = *mut POINT;
pub type LPPOINTS = *mut POINTS;
pub type LPRECT = *mut RECT;
pub type LPRECTL = *mut RECTL;
pub type LPSIZE = *mut SIZE;
pub type LPSIZEL = *mut SIZE;
pub type NPPOINT = *mut POINT;
pub type NPRECT = *mut RECT;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POINTL {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POINTS {
    pub x: i16,
    pub y: i16,
}
pub type PPOINT = *mut POINT;
pub type PPOINTL = *mut POINTL;
pub type PPOINTS = *mut POINTS;
pub type PRECT = *mut RECT;
pub type PRECTL = *mut RECTL;
pub type PSIZE = *mut SIZE;
pub type PSIZEL = *mut SIZE;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RECTL {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SIZE {
    pub cx: i32,
    pub cy: i32,
}
pub type SIZEL = SIZE;
