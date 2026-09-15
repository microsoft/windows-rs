#[repr(C)]
#[derive(Clone, Copy)]
pub struct APP_LOCAL_DEVICE_ID {
    pub value: [u8; 32],
}
impl Default for APP_LOCAL_DEVICE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const APP_LOCAL_DEVICE_ID_SIZE: i32 = 32;
pub type COLORREF = u32;
pub const DC_BINNAMES: i32 = 12;
pub const DC_BINS: i32 = 6;
pub const DC_COPIES: i32 = 18;
pub const DC_DRIVER: i32 = 11;
pub const DC_DUPLEX: i32 = 7;
pub const DC_ENUMRESOLUTIONS: i32 = 13;
pub const DC_EXTRA: i32 = 9;
pub const DC_FIELDS: i32 = 1;
pub const DC_FILEDEPENDENCIES: i32 = 14;
pub const DC_MAXEXTENT: i32 = 5;
pub const DC_MINEXTENT: i32 = 4;
pub const DC_ORIENTATION: i32 = 17;
pub const DC_PAPERNAMES: i32 = 16;
pub const DC_PAPERS: i32 = 2;
pub const DC_PAPERSIZE: i32 = 3;
pub const DC_SIZE: i32 = 8;
pub const DC_TRUETYPE: i32 = 15;
pub const DC_VERSION: i32 = 10;
pub const DM_COPY: i32 = 2;
pub const DM_IN_BUFFER: i32 = 8;
pub const DM_IN_PROMPT: i32 = 4;
pub const DM_MODIFY: i32 = 8;
pub const DM_OUT_BUFFER: i32 = 2;
pub const DM_OUT_DEFAULT: i32 = 1;
pub const DM_PROMPT: i32 = 4;
pub const DM_UPDATE: i32 = 1;
pub type DPI_AWARENESS = i32;
pub type DPI_AWARENESS_CONTEXT = *mut DPI_AWARENESS_CONTEXT__;
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE: DPI_AWARENESS_CONTEXT = -3 as _;
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT = -4 as _;
pub const DPI_AWARENESS_CONTEXT_SYSTEM_AWARE: DPI_AWARENESS_CONTEXT = -2 as _;
pub const DPI_AWARENESS_CONTEXT_UNAWARE: DPI_AWARENESS_CONTEXT = -1 as _;
pub const DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED: DPI_AWARENESS_CONTEXT = -5 as _;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct HACCEL__ {
    pub unused: i32,
}
pub type HBITMAP = *mut HBITMAP__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HBITMAP__ {
    pub unused: i32,
}
pub type HBRUSH = *mut HBRUSH__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HBRUSH__ {
    pub unused: i32,
}
pub type HCOLORSPACE = *mut HCOLORSPACE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HCOLORSPACE__ {
    pub unused: i32,
}
pub type HCURSOR = HICON;
pub type HDC = *mut HDC__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HDC__ {
    pub unused: i32,
}
pub type HDESK = *mut HDESK__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HDESK__ {
    pub unused: i32,
}
pub type HENHMETAFILE = *mut HENHMETAFILE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HENHMETAFILE__ {
    pub unused: i32,
}
#[cfg(feature = "minwindef")]
pub const HFILE_ERROR: super::HFILE = 0xFFFFFFFF_u32 as _;
pub type HFONT = *mut HFONT__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HFONT__ {
    pub unused: i32,
}
pub type HGDIOBJ = *mut core::ffi::c_void;
pub type HGLRC = *mut HGLRC__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HGLRC__ {
    pub unused: i32,
}
pub type HHOOK = *mut HHOOK__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HHOOK__ {
    pub unused: i32,
}
pub type HICON = *mut HICON__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HICON__ {
    pub unused: i32,
}
pub type HMENU = *mut HMENU__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HMENU__ {
    pub unused: i32,
}
pub type HMONITOR = *mut HMONITOR__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HMONITOR__ {
    pub unused: i32,
}
pub type HPALETTE = *mut HPALETTE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HPALETTE__ {
    pub unused: i32,
}
pub type HPEN = *mut HPEN__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HPEN__ {
    pub unused: i32,
}
pub type HUMPD = *mut HUMPD__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HUMPD__ {
    pub unused: i32,
}
pub type HWINEVENTHOOK = *mut HWINEVENTHOOK__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HWINEVENTHOOK__ {
    pub unused: i32,
}
pub type HWND = *mut HWND__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HWND__ {
    pub unused: i32,
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
#[derive(Clone, Copy, Default)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POINTL {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RECTL {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SIZE {
    pub cx: i32,
    pub cy: i32,
}
pub type SIZEL = SIZE;
