pub const RECO_COPY: i32 = 2;
pub const RECO_CUT: i32 = 3;
pub const RECO_DRAG: i32 = 4;
pub const RECO_DROP: i32 = 1;
pub const RECO_PASTE: i32 = 0;
#[repr(C)]
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct REOBJECT {
    pub cbStruct: u32,
    pub cp: i32,
    pub clsid: windows_sys::core::GUID,
    pub poleobj: *mut core::ffi::c_void,
    pub pstg: *mut core::ffi::c_void,
    pub polesite: *mut core::ffi::c_void,
    pub sizel: super::SIZEL,
    pub dvaspect: u32,
    pub dwFlags: u32,
    pub dwUser: u32,
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "windef"))]
impl Default for REOBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REO_ALIGNTORIGHT: i32 = 256;
pub const REO_BELOWBASELINE: i32 = 2;
pub const REO_BLANK: i32 = 16;
pub const REO_CANROTATE: i32 = 128;
pub const REO_CP_SELECTION: u32 = 4294967295;
pub const REO_DONTNEEDPALETTE: i32 = 32;
pub const REO_DYNAMICSIZE: i32 = 8;
pub const REO_GETMETAFILE: i32 = 4194304;
pub const REO_GETOBJ_ALL_INTERFACES: i32 = 7;
pub const REO_GETOBJ_NO_INTERFACES: i32 = 0;
pub const REO_GETOBJ_POLEOBJ: i32 = 1;
pub const REO_GETOBJ_POLESITE: i32 = 4;
pub const REO_GETOBJ_PSTG: i32 = 2;
pub const REO_HILITED: i32 = 16777216;
pub const REO_INPLACEACTIVE: i32 = 33554432;
pub const REO_INVERTEDSELECT: i32 = 4;
pub const REO_IOB_SELECTION: u32 = 4294967295;
pub const REO_IOB_USE_CP: u32 = 4294967294;
pub const REO_LINK: u32 = 2147483648;
pub const REO_LINKAVAILABLE: i32 = 8388608;
pub const REO_NULL: i32 = 0;
pub const REO_OPEN: i32 = 67108864;
pub const REO_OWNERDRAWSELECT: i32 = 64;
pub const REO_READWRITEMASK: i32 = 2047;
pub const REO_RESIZABLE: i32 = 1;
pub const REO_SELECTED: i32 = 134217728;
pub const REO_STATIC: i32 = 1073741824;
pub const REO_USEASBACKGROUND: i32 = 1024;
pub const REO_WRAPTEXTAROUND: i32 = 512;
