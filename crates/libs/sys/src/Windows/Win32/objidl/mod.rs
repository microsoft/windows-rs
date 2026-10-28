windows_link::link!("ole32.dll" "system" fn SNB_UserFree(param0 : *const u32, param1 : *const SNB));
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("ole32.dll" "system" fn SNB_UserFree64(param0 : *const u32, param1 : *const SNB));
windows_link::link!("ole32.dll" "system" fn SNB_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const SNB) -> *mut u8);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("ole32.dll" "system" fn SNB_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const SNB) -> *mut u8);
windows_link::link!("ole32.dll" "system" fn SNB_UserSize(param0 : *const u32, param1 : u32, param2 : *const SNB) -> u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("ole32.dll" "system" fn SNB_UserSize64(param0 : *const u32, param1 : u32, param2 : *const SNB) -> u32);
windows_link::link!("ole32.dll" "system" fn SNB_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut SNB) -> *mut u8);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("ole32.dll" "system" fn SNB_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut SNB) -> *mut u8);
pub type ADVF = i32;
pub const ADVFCACHE_FORCEBUILTIN: ADVF = 16;
pub const ADVFCACHE_NOHANDLER: ADVF = 8;
pub const ADVFCACHE_ONSAVE: ADVF = 32;
pub const ADVF_DATAONSTOP: ADVF = 64;
pub const ADVF_NODATA: ADVF = 1;
pub const ADVF_ONLYONCE: ADVF = 4;
pub const ADVF_PRIMEFIRST: ADVF = 2;
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub type ASYNC_STGMEDIUM = uSTGMEDIUM;
pub type ApplicationType = i32;
pub type BIND_FLAGS = i32;
pub const BIND_JUSTTESTEXISTENCE: BIND_FLAGS = 2;
pub const BIND_MAYBOTHERUSER: BIND_FLAGS = 1;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BIND_OPTS {
    pub cbStruct: super::DWORD,
    pub grfFlags: super::DWORD,
    pub grfMode: super::DWORD,
    pub dwTickCountDeadline: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct BIND_OPTS2 {
    pub base: BIND_OPTS,
    pub dwTrackFlags: super::DWORD,
    pub dwClassContext: super::DWORD,
    pub locale: super::LCID,
    pub pServerInfo: *mut super::COSERVERINFO,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct BIND_OPTS3 {
    pub base: BIND_OPTS2,
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
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DVTARGETDEVICE {
    pub tdSize: super::DWORD,
    pub tdDriverNameOffset: super::WORD,
    pub tdDeviceNameOffset: super::WORD,
    pub tdPortNameOffset: super::WORD,
    pub tdExtDevmodeOffset: super::WORD,
    pub tdData: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for DVTARGETDEVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct FLAG_STGMEDIUM {
    pub ContextFlags: super::LONG,
    pub fPassOwnership: super::LONG,
    pub Stgmed: STGMEDIUM,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl Default for FLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[derive(Clone, Copy, Default)]
pub struct FORMATETC {
    pub cfFormat: super::CLIPFORMAT,
    pub ptd: *mut DVTARGETDEVICE,
    pub dwAspect: super::DWORD,
    pub lindex: super::LONG,
    pub tymed: super::DWORD,
}
pub const ForcedShutdown: ShutdownType = 1;
#[repr(C, packed(8))]
#[cfg(all(feature = "rpc", feature = "shared", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct GDI_OBJECT {
    pub ObjectType: super::DWORD,
    pub u: GDI_OBJECT___MIDL_IAdviseSink_0002,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
impl Default for GDI_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "rpc", feature = "shared", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union GDI_OBJECT___MIDL_IAdviseSink_0002 {
    pub hBitmap: super::wireHBITMAP,
    pub hPalette: super::wireHPALETTE,
    pub hGeneric: super::wireHGLOBAL,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
impl Default for GDI_OBJECT___MIDL_IAdviseSink_0002 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct INTERFACEINFO {
    pub pUnk: *mut core::ffi::c_void,
    pub iid: super::IID,
    pub wMethod: super::WORD,
}
pub const IdleShutdown: ShutdownType = 0;
pub type LPADVISESINK = *mut core::ffi::c_void;
pub type LPADVISESINK2 = *mut core::ffi::c_void;
pub type LPBC = *mut core::ffi::c_void;
pub type LPBINDCTX = *mut core::ffi::c_void;
#[cfg(feature = "shared")]
pub type LPBIND_OPTS = *mut BIND_OPTS;
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "wtypesbase"))]
pub type LPBIND_OPTS2 = *mut BIND_OPTS2;
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypesbase"))]
pub type LPBIND_OPTS3 = *mut BIND_OPTS3;
pub type LPCLIPFORMAT = *mut u16;
pub type LPDATAADVISEHOLDER = *mut core::ffi::c_void;
pub type LPDATAOBJECT = *mut core::ffi::c_void;
pub type LPENUMFORMATETC = *mut core::ffi::c_void;
pub type LPENUMMONIKER = *mut core::ffi::c_void;
pub type LPENUMSTATDATA = *mut core::ffi::c_void;
pub type LPENUMSTATSTG = *mut core::ffi::c_void;
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub type LPFORMATETC = *mut FORMATETC;
pub type LPINITIALIZESPY = *mut core::ffi::c_void;
#[cfg(all(feature = "guiddef", feature = "shared"))]
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
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub type LPSTATDATA = *mut STATDATA;
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub type LPSTGMEDIUM = *mut uSTGMEDIUM;
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
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct RemSNB {
    pub ulCntStr: super::ULONG,
    pub ulCntChar: super::ULONG,
    pub rgString: [super::OLECHAR; 1],
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl Default for RemSNB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "rpc", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct RemSTGMEDIUM {
    pub tymed: super::DWORD,
    pub dwHandleType: super::DWORD,
    pub pData: super::ULONG,
    pub pUnkForRelease: super::ULONG,
    pub cbData: super::ULONG,
    pub data: [super::byte; 1],
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl Default for RemSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SERVERCALL = i32;
pub const SERVERCALL_ISHANDLED: SERVERCALL = 0;
pub const SERVERCALL_REJECTED: SERVERCALL = 1;
pub const SERVERCALL_RETRYLATER: SERVERCALL = 2;
pub type SNB = *mut *mut u16;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[derive(Clone, Copy, Default)]
pub struct STATDATA {
    pub formatetc: FORMATETC,
    pub advf: super::DWORD,
    pub pAdvSink: *mut core::ffi::c_void,
    pub dwConnection: super::DWORD,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub type STGMEDIUM = uSTGMEDIUM;
pub const ServerApplication: ApplicationType = 0;
pub type ShutdownType = i32;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct StorageLayout {
    pub LayoutType: super::DWORD,
    pub pwcsElementName: *mut super::OLECHAR,
    pub cOffset: super::LARGE_INTEGER,
    pub cBytes: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wtypesbase"))]
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
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct uSTGMEDIUM {
    pub tymed: super::DWORD,
    pub Anonymous: uSTGMEDIUM_0,
    pub pUnkForRelease: *mut core::ffi::c_void,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl Default for uSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl Default for uSTGMEDIUM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "rpc", feature = "shared", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct userFLAG_STGMEDIUM {
    pub ContextFlags: super::LONG,
    pub fPassOwnership: super::LONG,
    pub Stgmed: userSTGMEDIUM,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
impl Default for userFLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "rpc", feature = "shared", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct userSTGMEDIUM {
    pub pUnkForRelease: *mut core::ffi::c_void,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
impl Default for userSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "rpc", feature = "shared", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct userSTGMEDIUM__STGMEDIUM_UNION {
    pub tymed: super::DWORD,
    pub u: userSTGMEDIUM__STGMEDIUM_UNION___MIDL_IAdviseSink_0003,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
impl Default for userSTGMEDIUM__STGMEDIUM_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "rpc", feature = "shared", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union userSTGMEDIUM__STGMEDIUM_UNION___MIDL_IAdviseSink_0003 {
    pub hMetaFilePict: super::wireHMETAFILEPICT,
    pub hHEnhMetaFile: super::wireHENHMETAFILE,
    pub hGdiHandle: *mut GDI_OBJECT,
    pub hGlobal: super::wireHGLOBAL,
    pub lpszFileName: super::LPOLESTR,
    pub pstm: *mut super::BYTE_BLOB,
    pub pstg: *mut super::BYTE_BLOB,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
impl Default for userSTGMEDIUM__STGMEDIUM_UNION___MIDL_IAdviseSink_0003 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
pub type wireASYNC_STGMEDIUM = *mut userSTGMEDIUM;
#[cfg(all(feature = "rpc", feature = "shared", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
pub type wireFLAG_STGMEDIUM = *mut userFLAG_STGMEDIUM;
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub type wireSNB = *mut RemSNB;
#[cfg(all(feature = "rpc", feature = "shared", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
pub type wireSTGMEDIUM = *mut userSTGMEDIUM;
