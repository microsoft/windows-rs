#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HDC_UserFree(param0 : *mut u32, param1 : *mut super::HDC));
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HDC_UserFree64(param0 : *mut u32, param1 : *mut super::HDC));
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HDC_UserMarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HDC) -> *mut u8);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HDC_UserMarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HDC) -> *mut u8);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HDC_UserSize(param0 : *mut u32, param1 : u32, param2 : *mut super::HDC) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HDC_UserSize64(param0 : *mut u32, param1 : u32, param2 : *mut super::HDC) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HDC_UserUnmarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HDC) -> *mut u8);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HDC_UserUnmarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HDC) -> *mut u8);
#[cfg(feature = "wtypesbase")]
windows_link::link!("ole32.dll" "system" fn SNB_UserFree(param0 : *mut u32, param1 : *mut SNB));
#[cfg(feature = "wtypesbase")]
windows_link::link!("ole32.dll" "system" fn SNB_UserFree64(param0 : *mut u32, param1 : *mut SNB));
#[cfg(feature = "wtypesbase")]
windows_link::link!("ole32.dll" "system" fn SNB_UserMarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut SNB) -> *mut u8);
#[cfg(feature = "wtypesbase")]
windows_link::link!("ole32.dll" "system" fn SNB_UserMarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut SNB) -> *mut u8);
#[cfg(feature = "wtypesbase")]
windows_link::link!("ole32.dll" "system" fn SNB_UserSize(param0 : *mut u32, param1 : u32, param2 : *mut SNB) -> u32);
#[cfg(feature = "wtypesbase")]
windows_link::link!("ole32.dll" "system" fn SNB_UserSize64(param0 : *mut u32, param1 : u32, param2 : *mut SNB) -> u32);
#[cfg(feature = "wtypesbase")]
windows_link::link!("ole32.dll" "system" fn SNB_UserUnmarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut SNB) -> *mut u8);
#[cfg(feature = "wtypesbase")]
windows_link::link!("ole32.dll" "system" fn SNB_UserUnmarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut SNB) -> *mut u8);
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn STGMEDIUM_UserFree(param0 : *mut u32, param1 : *mut STGMEDIUM));
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn STGMEDIUM_UserFree64(param0 : *mut u32, param1 : *mut STGMEDIUM));
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn STGMEDIUM_UserMarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut STGMEDIUM) -> *mut u8);
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn STGMEDIUM_UserMarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut STGMEDIUM) -> *mut u8);
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn STGMEDIUM_UserSize(param0 : *mut u32, param1 : u32, param2 : *mut STGMEDIUM) -> u32);
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn STGMEDIUM_UserSize64(param0 : *mut u32, param1 : u32, param2 : *mut STGMEDIUM) -> u32);
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn STGMEDIUM_UserUnmarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut STGMEDIUM) -> *mut u8);
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn STGMEDIUM_UserUnmarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut STGMEDIUM) -> *mut u8);
pub type ADVF = i32;
pub const ADVFCACHE_FORCEBUILTIN: ADVF = 16;
pub const ADVFCACHE_NOHANDLER: ADVF = 8;
pub const ADVFCACHE_ONSAVE: ADVF = 32;
pub const ADVF_DATAONSTOP: ADVF = 64;
pub const ADVF_NODATA: ADVF = 1;
pub const ADVF_ONLYONCE: ADVF = 4;
pub const ADVF_PRIMEFIRST: ADVF = 2;
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "objidlbase", feature = "winnt", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct BIND_OPTS2 {
    pub Base: BIND_OPTS,
    pub dwTrackFlags: u32,
    pub dwClassContext: u32,
    pub locale: super::LCID,
    pub pServerInfo: *mut super::COSERVERINFO,
}
#[repr(C)]
#[cfg(all(feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct BIND_OPTS3 {
    pub Base: BIND_OPTS2,
    pub hwnd: super::HWND,
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
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct FLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: STGMEDIUM,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for FLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Default)]
pub struct FORMATETC {
    pub cfFormat: super::CLIPFORMAT,
    pub ptd: *mut DVTARGETDEVICE,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
pub const ForcedShutdown: ShutdownType = 1;
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct GDI_OBJECT {
    pub ObjectType: u32,
    pub u: __MIDL_IAdviseSink_0002,
}
#[cfg(all(feature = "rpc", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
impl Default for GDI_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INTERFACEINFO {
    pub pUnk: *mut core::ffi::c_void,
    pub iid: windows_sys::core::GUID,
    pub wMethod: u16,
}
pub const IdleShutdown: ShutdownType = 0;
pub type LPADVISESINK = *mut core::ffi::c_void;
pub type LPADVISESINK2 = *mut core::ffi::c_void;
pub type LPBC = *mut core::ffi::c_void;
pub type LPBINDCTX = *mut core::ffi::c_void;
pub type LPBIND_OPTS = *mut BIND_OPTS;
#[cfg(all(feature = "objidlbase", feature = "winnt", feature = "wtypesbase"))]
pub type LPBIND_OPTS2 = *mut BIND_OPTS2;
#[cfg(all(feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypesbase"))]
pub type LPBIND_OPTS3 = *mut BIND_OPTS3;
#[cfg(feature = "wtypes")]
pub type LPCLIPFORMAT = *mut super::CLIPFORMAT;
pub type LPDATAADVISEHOLDER = *mut core::ffi::c_void;
pub type LPDATAOBJECT = *mut core::ffi::c_void;
pub type LPENUMFORMATETC = *mut core::ffi::c_void;
pub type LPENUMMONIKER = *mut core::ffi::c_void;
pub type LPENUMSTATDATA = *mut core::ffi::c_void;
pub type LPENUMSTATSTG = *mut core::ffi::c_void;
#[cfg(feature = "wtypes")]
pub type LPFORMATETC = *mut FORMATETC;
pub type LPINITIALIZESPY = *mut core::ffi::c_void;
pub type LPINTERFACEINFO = *mut INTERFACEINFO;
pub type LPLOCKBYTES = *mut core::ffi::c_void;
pub type LPMALLOCSPY = *mut core::ffi::c_void;
pub type LPMESSAGEFILTER = *mut core::ffi::c_void;
pub type LPMONIKER = *mut core::ffi::c_void;
pub type LPPERSIST = *mut core::ffi::c_void;
pub type LPPERSISTFILE = *mut core::ffi::c_void;
pub type LPPERSISTSTORAGE = *mut core::ffi::c_void;
pub type LPPERSISTSTREAM = *mut core::ffi::c_void;
pub type LPROOTSTORAGE = *mut core::ffi::c_void;
pub type LPRUNNABLEOBJECT = *mut core::ffi::c_void;
pub type LPRUNNINGOBJECTTABLE = *mut core::ffi::c_void;
#[cfg(feature = "wtypes")]
pub type LPSTATDATA = *mut STATDATA;
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type LPSTGMEDIUM = *mut STGMEDIUM;
pub type LPSTORAGE = *mut core::ffi::c_void;
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
#[cfg(feature = "wtypesbase")]
#[derive(Clone, Copy)]
pub struct RemSNB {
    pub ulCntStr: u32,
    pub ulCntChar: u32,
    pub rgString: [super::OLECHAR; 1],
}
#[cfg(feature = "wtypesbase")]
impl Default for RemSNB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "rpc")]
#[derive(Clone, Copy)]
pub struct RemSTGMEDIUM {
    pub tymed: u32,
    pub dwHandleType: u32,
    pub pData: u32,
    pub pUnkForRelease: u32,
    pub cbData: u32,
    pub data: [super::byte; 1],
}
#[cfg(feature = "rpc")]
impl Default for RemSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SERVERCALL = i32;
pub const SERVERCALL_ISHANDLED: SERVERCALL = 0;
pub const SERVERCALL_REJECTED: SERVERCALL = 1;
pub const SERVERCALL_RETRYLATER: SERVERCALL = 2;
#[cfg(feature = "wtypesbase")]
pub type SNB = *mut super::LPOLESTR;
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Default)]
pub struct STATDATA {
    pub formatetc: FORMATETC,
    pub advf: u32,
    pub pAdvSink: *mut core::ffi::c_void,
    pub dwConnection: u32,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type STGMEDIUM = uSTGMEDIUM;
pub const ServerApplication: ApplicationType = 0;
pub type ShutdownType = i32;
#[repr(C)]
#[cfg(all(feature = "winnt", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct StorageLayout {
    pub LayoutType: u32,
    pub pwcsElementName: *mut super::OLECHAR,
    pub cOffset: super::LARGE_INTEGER,
    pub cBytes: super::LARGE_INTEGER,
}
#[cfg(all(feature = "winnt", feature = "wtypesbase"))]
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
#[cfg(all(feature = "rpc", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union __MIDL_IAdviseSink_0002 {
    pub hBitmap: super::wireHBITMAP,
    pub hPalette: super::wireHPALETTE,
    pub hGeneric: super::wireHGLOBAL,
}
#[cfg(all(feature = "rpc", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
impl Default for __MIDL_IAdviseSink_0002 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct uSTGMEDIUM {
    pub tymed: u32,
    pub Anonymous: uSTGMEDIUM_0,
    pub pUnkForRelease: *mut core::ffi::c_void,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for uSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union uSTGMEDIUM_0 {
    pub hBitmap: super::HBITMAP,
    pub hMetaFilePict: super::HMETAFILEPICT,
    pub hEnhMetaFile: super::HENHMETAFILE,
    pub hGlobal: super::HGLOBAL,
    pub lpszFileName: super::LPOLESTR,
    pub pstm: *mut core::ffi::c_void,
    pub pstg: *mut core::ffi::c_void,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[derive(Clone, Copy, Default)]
pub struct userSTGMEDIUM {
    pub pUnkForRelease: *mut core::ffi::c_void,
}
pub type wireASYNC_STGMEDIUM = *mut userSTGMEDIUM;
pub type wireFLAG_STGMEDIUM = *mut userFLAG_STGMEDIUM;
#[cfg(feature = "wtypesbase")]
pub type wireSNB = *mut RemSNB;
pub type wireSTGMEDIUM = *mut userSTGMEDIUM;
