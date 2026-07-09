pub type ADVF = i32;
pub const ADVFCACHE_FORCEBUILTIN: ADVF = 16;
pub const ADVFCACHE_NOHANDLER: ADVF = 8;
pub const ADVFCACHE_ONSAVE: ADVF = 32;
pub const ADVF_DATAONSTOP: ADVF = 64;
pub const ADVF_NODATA: ADVF = 1;
pub const ADVF_ONLYONCE: ADVF = 4;
pub const ADVF_PRIMEFIRST: ADVF = 2;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
pub type ASYNC_STGMEDIUM = STGMEDIUM;
pub type ApplicationType = i32;
pub type BIND_FLAGS = i32;
pub const BIND_JUSTTESTEXISTENCE: BIND_FLAGS = 2;
pub const BIND_MAYBOTHERUSER: BIND_FLAGS = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BIND_OPTS {
    pub cbStruct: u32,
    pub grfFlags: u32,
    pub grfMode: u32,
    pub dwTickCountDeadline: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct BIND_OPTS2 {
    pub Base: BIND_OPTS,
    pub dwTrackFlags: u32,
    pub dwClassContext: u32,
    pub locale: super::winnt::LCID,
    pub pServerInfo: *mut super::objidlbase::COSERVERINFO,
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypesbase"))]
impl Default for BIND_OPTS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct BIND_OPTS3 {
    pub Base: BIND_OPTS2,
    pub hwnd: super::windef::HWND,
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypesbase"))]
impl Default for BIND_OPTS3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CALLTYPE = i32;
pub const CALLTYPE_ASYNC: CALLTYPE = 3;
pub const CALLTYPE_ASYNC_CALLPENDING: CALLTYPE = 5;
pub const CALLTYPE_NESTED: CALLTYPE = 2;
pub const CALLTYPE_TOPLEVEL: CALLTYPE = 1;
pub const CALLTYPE_TOPLEVEL_CALLPENDING: CALLTYPE = 4;
pub type DATADIR = i32;
pub const DATADIR_GET: DATADIR = 1;
pub const DATADIR_SET: DATADIR = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DVTARGETDEVICE {
    pub tdSize: u32,
    pub tdDriverNameOffset: u16,
    pub tdDeviceNameOffset: u16,
    pub tdPortNameOffset: u16,
    pub tdExtDevmodeOffset: u16,
    pub tdData: [u8; 1],
}
impl Default for DVTARGETDEVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
#[derive(Clone, Copy)]
pub struct FLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: STGMEDIUM,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
impl Default for FLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wtypes")]
#[derive(Clone, Copy)]
pub struct FORMATETC {
    pub cfFormat: super::wtypes::CLIPFORMAT,
    pub ptd: *mut DVTARGETDEVICE,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
#[cfg(feature = "Win32_wtypes")]
impl Default for FORMATETC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ForcedShutdown: ShutdownType = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wingdi", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct GDI_OBJECT {
    pub ObjectType: u32,
    pub u: GDI_OBJECT_0,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wingdi", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for GDI_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wingdi", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub union GDI_OBJECT_0 {
    pub hBitmap: super::wtypes::wireHBITMAP,
    pub hPalette: super::wtypes::wireHPALETTE,
    pub hGeneric: super::wtypes::wireHGLOBAL,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wingdi", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for GDI_OBJECT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INTERFACEINFO {
    pub pUnk: *mut core::ffi::c_void,
    pub iid: windows_sys::core::GUID,
    pub wMethod: u16,
}
impl Default for INTERFACEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IdleShutdown: ShutdownType = 0;
pub type LPBIND_OPTS = *mut BIND_OPTS;
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypesbase"))]
pub type LPBIND_OPTS2 = *mut BIND_OPTS2;
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypesbase"))]
pub type LPBIND_OPTS3 = *mut BIND_OPTS3;
#[cfg(feature = "Win32_wtypes")]
pub type LPCLIPFORMAT = *mut super::wtypes::CLIPFORMAT;
#[cfg(feature = "Win32_wtypes")]
pub type LPFORMATETC = *mut FORMATETC;
pub type LPINTERFACEINFO = *mut INTERFACEINFO;
#[cfg(feature = "Win32_wtypes")]
pub type LPSTATDATA = *mut STATDATA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
pub type LPSTGMEDIUM = *mut STGMEDIUM;
pub const LibraryApplication: ApplicationType = 1;
pub type MKRREDUCE = i32;
pub const MKRREDUCE_ALL: MKRREDUCE = 0;
pub const MKRREDUCE_ONE: MKRREDUCE = 196608;
pub const MKRREDUCE_THROUGHUSER: MKRREDUCE = 65536;
pub const MKRREDUCE_TOUSER: MKRREDUCE = 131072;
pub type MKSYS = i32;
pub const MKSYS_ANTIMONIKER: MKSYS = 3;
pub const MKSYS_CLASSMONIKER: MKSYS = 7;
pub const MKSYS_FILEMONIKER: MKSYS = 2;
pub const MKSYS_GENERICCOMPOSITE: MKSYS = 1;
pub const MKSYS_ITEMMONIKER: MKSYS = 4;
pub const MKSYS_LUAMONIKER: MKSYS = 10;
pub const MKSYS_NONE: MKSYS = 0;
pub const MKSYS_OBJREFMONIKER: MKSYS = 8;
pub const MKSYS_POINTERMONIKER: MKSYS = 5;
pub const MKSYS_SESSIONMONIKER: MKSYS = 9;
pub type PENDINGMSG = i32;
pub const PENDINGMSG_CANCELCALL: PENDINGMSG = 0;
pub const PENDINGMSG_WAITDEFPROCESS: PENDINGMSG = 2;
pub const PENDINGMSG_WAITNOPROCESS: PENDINGMSG = 1;
pub type PENDINGTYPE = i32;
pub const PENDINGTYPE_NESTED: PENDINGTYPE = 2;
pub const PENDINGTYPE_TOPLEVEL: PENDINGTYPE = 1;
#[repr(C)]
#[cfg(feature = "Win32_wtypesbase")]
#[derive(Clone, Copy)]
pub struct RemSNB {
    pub ulCntStr: u32,
    pub ulCntChar: u32,
    pub rgString: [super::wtypesbase::OLECHAR; 1],
}
#[cfg(feature = "Win32_wtypesbase")]
impl Default for RemSNB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_rpc")]
#[derive(Clone, Copy)]
pub struct RemSTGMEDIUM {
    pub tymed: u32,
    pub dwHandleType: u32,
    pub pData: u32,
    pub pUnkForRelease: u32,
    pub cbData: u32,
    pub data: [super::rpc::byte; 1],
}
#[cfg(feature = "Win32_rpc")]
impl Default for RemSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SERVERCALL = i32;
pub const SERVERCALL_ISHANDLED: SERVERCALL = 0;
pub const SERVERCALL_REJECTED: SERVERCALL = 1;
pub const SERVERCALL_RETRYLATER: SERVERCALL = 2;
pub type SNB = *mut windows_sys::core::PWSTR;
#[repr(C)]
#[cfg(feature = "Win32_wtypes")]
#[derive(Clone, Copy)]
pub struct STATDATA {
    pub formatetc: FORMATETC,
    pub advf: u32,
    pub pAdvSink: *mut core::ffi::c_void,
    pub dwConnection: u32,
}
#[cfg(feature = "Win32_wtypes")]
impl Default for STATDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
pub type STGMEDIUM = uSTGMEDIUM;
pub const ServerApplication: ApplicationType = 0;
pub type ShutdownType = i32;
#[repr(C)]
#[cfg(feature = "Win32_wtypesbase")]
#[derive(Clone, Copy)]
pub struct StorageLayout {
    pub LayoutType: u32,
    pub pwcsElementName: *mut super::wtypesbase::OLECHAR,
    pub cOffset: i64,
    pub cBytes: i64,
}
#[cfg(feature = "Win32_wtypesbase")]
impl Default for StorageLayout {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TYMED = i32;
pub const TYMED_ENHMF: TYMED = 64;
pub const TYMED_FILE: TYMED = 2;
pub const TYMED_GDI: TYMED = 16;
pub const TYMED_HGLOBAL: TYMED = 1;
pub const TYMED_ISTORAGE: TYMED = 8;
pub const TYMED_ISTREAM: TYMED = 4;
pub const TYMED_MFPICT: TYMED = 32;
pub const TYMED_NULL: TYMED = 0;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
#[derive(Clone, Copy)]
pub struct uSTGMEDIUM {
    pub tymed: u32,
    pub Anonymous: uSTGMEDIUM_0,
    pub pUnkForRelease: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
impl Default for uSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
#[derive(Clone, Copy)]
pub union uSTGMEDIUM_0 {
    pub hBitmap: super::windef::HBITMAP,
    pub hMetaFilePict: super::wtypes::HMETAFILEPICT,
    pub hEnhMetaFile: super::windef::HENHMETAFILE,
    pub hGlobal: super::minwindef::HGLOBAL,
    pub lpszFileName: windows_sys::core::PWSTR,
    pub pstm: *mut core::ffi::c_void,
    pub pstg: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
impl Default for uSTGMEDIUM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct userFLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: userSTGMEDIUM,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct userSTGMEDIUM {
    pub pUnkForRelease: *mut core::ffi::c_void,
}
impl Default for userSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wingdi", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct userSTGMEDIUM_0 {
    pub tymed: u32,
    pub u: userSTGMEDIUM_0_0,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wingdi", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for userSTGMEDIUM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wingdi", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub union userSTGMEDIUM_0_0 {
    pub hMetaFilePict: super::wtypes::wireHMETAFILEPICT,
    pub hHEnhMetaFile: super::wtypes::wireHENHMETAFILE,
    pub hGdiHandle: *mut GDI_OBJECT,
    pub hGlobal: super::wtypes::wireHGLOBAL,
    pub lpszFileName: windows_sys::core::PWSTR,
    pub pstm: *mut super::wtypesbase::BYTE_BLOB,
    pub pstg: *mut super::wtypesbase::BYTE_BLOB,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_wingdi", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for userSTGMEDIUM_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type wireASYNC_STGMEDIUM = *mut userSTGMEDIUM;
pub type wireFLAG_STGMEDIUM = *mut userFLAG_STGMEDIUM;
#[cfg(feature = "Win32_wtypesbase")]
pub type wireSNB = *mut RemSNB;
pub type wireSTGMEDIUM = *mut userSTGMEDIUM;
