#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct APP_LOCAL_DEVICE_ID {
    pub value: [super::BYTE; 32],
}
#[cfg(feature = "shared")]
impl Default for APP_LOCAL_DEVICE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const APP_LOCAL_DEVICE_ID_SIZE: i32 = 32;
pub type COLORREF = u32;
pub type DPI_AWARENESS = i32;
pub type DPI_AWARENESS_CONTEXT = *mut DPI_AWARENESS_CONTEXT__;
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE: DPI_AWARENESS_CONTEXT = -3 as _;
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT = -4 as _;
pub const DPI_AWARENESS_CONTEXT_SYSTEM_AWARE: DPI_AWARENESS_CONTEXT = -2 as _;
pub const DPI_AWARENESS_CONTEXT_UNAWARE: DPI_AWARENESS_CONTEXT = -1 as _;
pub const DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED: DPI_AWARENESS_CONTEXT = -5 as _;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DPI_AWARENESS_CONTEXT__ {
    pub unused: i32,
}
pub const DPI_AWARENESS_INVALID: DPI_AWARENESS = -1;
pub const DPI_AWARENESS_PER_MONITOR_AWARE: DPI_AWARENESS = 2;
pub const DPI_AWARENESS_SYSTEM_AWARE: DPI_AWARENESS = 1;
pub const DPI_AWARENESS_UNAWARE: DPI_AWARENESS = 0;
pub type DPI_HOSTING_BEHAVIOR = i32;
pub const DPI_HOSTING_BEHAVIOR_DEFAULT: DPI_HOSTING_BEHAVIOR = 0;
pub const DPI_HOSTING_BEHAVIOR_INVALID: DPI_HOSTING_BEHAVIOR = -1;
pub const DPI_HOSTING_BEHAVIOR_MIXED: DPI_HOSTING_BEHAVIOR = 1;
pub type HACCEL = *mut HACCEL__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HACCEL__ {
    pub unused: i32,
}
pub type HBITMAP = *mut HBITMAP__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HBITMAP__ {
    pub unused: i32,
}
pub type HBRUSH = *mut HBRUSH__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HBRUSH__ {
    pub unused: i32,
}
pub type HCOLORSPACE = *mut HCOLORSPACE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HCOLORSPACE__ {
    pub unused: i32,
}
pub type HCURSOR = *mut HICON__;
pub type HDC = *mut HDC__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HDC__ {
    pub unused: i32,
}
pub type HDESK = *mut HDESK__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HDESK__ {
    pub unused: i32,
}
pub type HENHMETAFILE = *mut HENHMETAFILE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HENHMETAFILE__ {
    pub unused: i32,
}
#[cfg(feature = "minwindef")]
pub const HFILE_ERROR: super::HFILE = 0xFFFFFFFF_u32 as _;
pub type HFONT = *mut HFONT__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HFONT__ {
    pub unused: i32,
}
pub type HGDIOBJ = *mut core::ffi::c_void;
pub type HGLRC = *mut HGLRC__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HGLRC__ {
    pub unused: i32,
}
pub type HHOOK = *mut HHOOK__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HHOOK__ {
    pub unused: i32,
}
pub type HICON = *mut HICON__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HICON__ {
    pub unused: i32,
}
pub type HMENU = *mut HMENU__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HMENU__ {
    pub unused: i32,
}
pub type HMONITOR = *mut HMONITOR__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HMONITOR__ {
    pub unused: i32,
}
pub type HPALETTE = *mut HPALETTE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HPALETTE__ {
    pub unused: i32,
}
pub type HPEN = *mut HPEN__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HPEN__ {
    pub unused: i32,
}
pub type HUMPD = *mut HUMPD__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HUMPD__ {
    pub unused: i32,
}
pub type HWINEVENTHOOK = *mut HWINEVENTHOOK__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HWINEVENTHOOK__ {
    pub unused: i32,
}
pub type HWND = *mut HWND__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HWND__ {
    pub unused: i32,
}
pub type LPCOLORREF = *mut u32;
#[cfg(feature = "shared")]
pub type LPCRECT = *const RECT;
#[cfg(feature = "shared")]
pub type LPCRECTL = *const RECTL;
#[cfg(feature = "shared")]
pub type LPPOINT = *mut POINT;
#[cfg(feature = "shared")]
pub type LPPOINTS = *mut POINTS;
#[cfg(feature = "shared")]
pub type LPRECT = *mut RECT;
#[cfg(feature = "shared")]
pub type LPRECTL = *mut RECTL;
#[cfg(feature = "shared")]
pub type LPSIZE = *mut SIZE;
#[cfg(feature = "shared")]
pub type LPSIZEL = *mut SIZE;
#[cfg(feature = "shared")]
pub type NPPOINT = *mut POINT;
#[cfg(feature = "shared")]
pub type NPRECT = *mut RECT;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINT {
    pub x: super::LONG,
    pub y: super::LONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINTL {
    pub x: super::LONG,
    pub y: super::LONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINTS {
    pub x: super::SHORT,
    pub y: super::SHORT,
}
#[cfg(feature = "shared")]
pub type PPOINT = *mut POINT;
#[cfg(feature = "shared")]
pub type PPOINTL = *mut POINTL;
#[cfg(feature = "shared")]
pub type PPOINTS = *mut POINTS;
#[cfg(feature = "shared")]
pub type PRECT = *mut RECT;
#[cfg(feature = "shared")]
pub type PRECTL = *mut RECTL;
#[cfg(feature = "shared")]
pub type PSIZE = *mut SIZE;
#[cfg(feature = "shared")]
pub type PSIZEL = *mut SIZE;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RECT {
    pub left: super::LONG,
    pub top: super::LONG,
    pub right: super::LONG,
    pub bottom: super::LONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RECTL {
    pub left: super::LONG,
    pub top: super::LONG,
    pub right: super::LONG,
    pub bottom: super::LONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SIZE {
    pub cx: super::LONG,
    pub cy: super::LONG,
}
#[cfg(feature = "shared")]
pub type SIZEL = SIZE;
