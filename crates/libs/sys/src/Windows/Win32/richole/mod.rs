pub const RECO_COPY: u32 = 2;
pub const RECO_CUT: u32 = 3;
pub const RECO_DRAG: u32 = 4;
pub const RECO_DROP: u32 = 1;
pub const RECO_PASTE: u32 = 0;
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
pub const REO_ALIGNTORIGHT: u32 = 256;
pub const REO_BELOWBASELINE: u32 = 2;
pub const REO_BLANK: u32 = 16;
pub const REO_CANROTATE: u32 = 128;
pub const REO_CP_SELECTION: u32 = 4294967295;
pub const REO_DONTNEEDPALETTE: u32 = 32;
pub const REO_DYNAMICSIZE: u32 = 8;
pub const REO_GETMETAFILE: u32 = 4194304;
pub const REO_GETOBJ_ALL_INTERFACES: u32 = 7;
pub const REO_GETOBJ_NO_INTERFACES: u32 = 0;
pub const REO_GETOBJ_POLEOBJ: u32 = 1;
pub const REO_GETOBJ_POLESITE: u32 = 4;
pub const REO_GETOBJ_PSTG: u32 = 2;
pub const REO_HILITED: u32 = 16777216;
pub const REO_INPLACEACTIVE: u32 = 33554432;
pub const REO_INVERTEDSELECT: u32 = 4;
pub const REO_IOB_SELECTION: u32 = 4294967295;
pub const REO_IOB_USE_CP: u32 = 4294967294;
pub const REO_LINK: u32 = 2147483648;
pub const REO_LINKAVAILABLE: u32 = 8388608;
pub const REO_NULL: u32 = 0;
pub const REO_OPEN: u32 = 67108864;
pub const REO_OWNERDRAWSELECT: u32 = 64;
pub const REO_READWRITEMASK: u32 = 2047;
pub const REO_RESIZABLE: u32 = 1;
pub const REO_SELECTED: u32 = 134217728;
pub const REO_STATIC: u32 = 1073741824;
pub const REO_USEASBACKGROUND: u32 = 1024;
pub const REO_WRAPTEXTAROUND: u32 = 512;
