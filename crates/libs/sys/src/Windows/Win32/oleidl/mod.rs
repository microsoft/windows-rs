#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn HGLOBAL_UserFree(param0 : *mut u32, param1 : *mut super::HGLOBAL));
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn HGLOBAL_UserFree64(param0 : *mut u32, param1 : *mut super::HGLOBAL));
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn HGLOBAL_UserMarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HGLOBAL) -> *mut u8);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn HGLOBAL_UserMarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HGLOBAL) -> *mut u8);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn HGLOBAL_UserSize(param0 : *mut u32, param1 : u32, param2 : *mut super::HGLOBAL) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn HGLOBAL_UserSize64(param0 : *mut u32, param1 : u32, param2 : *mut super::HGLOBAL) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn HGLOBAL_UserUnmarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HGLOBAL) -> *mut u8);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn HGLOBAL_UserUnmarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HGLOBAL) -> *mut u8);
pub type BINDSPEED = i32;
pub const BINDSPEED_IMMEDIATE: BINDSPEED = 3;
pub const BINDSPEED_INDEFINITE: BINDSPEED = 1;
pub const BINDSPEED_MODERATE: BINDSPEED = 2;
#[cfg(feature = "windef")]
pub type BORDERWIDTHS = super::RECT;
pub const DD_DEFDRAGDELAY: i32 = 200;
pub const DD_DEFDRAGMINDIST: i32 = 2;
pub const DD_DEFSCROLLDELAY: i32 = 50;
pub const DD_DEFSCROLLINSET: i32 = 11;
pub const DD_DEFSCROLLINTERVAL: i32 = 50;
pub type DISCARDCACHE = i32;
pub const DISCARDCACHE_NOSAVE: DISCARDCACHE = 1;
pub const DISCARDCACHE_SAVEIFDIRTY: DISCARDCACHE = 0;
pub const DROPEFFECT_BACKGROUNDTARGET: i32 = 536870912;
pub const DROPEFFECT_COPY: i32 = 1;
pub const DROPEFFECT_LINK: i32 = 4;
pub const DROPEFFECT_MOVE: i32 = 2;
pub const DROPEFFECT_NEWTARGET: i32 = 1073741824;
pub const DROPEFFECT_NONE: i32 = 0;
pub const DROPEFFECT_SCROLL: u32 = 2147483648;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type HOLEMENU = super::HGLOBAL;
#[cfg(feature = "windef")]
pub type LINKSRCDESCRIPTOR = tagOBJECTDESCRIPTOR;
#[cfg(feature = "windef")]
pub type LPBORDERWIDTHS = super::LPRECT;
#[cfg(feature = "windef")]
pub type LPCBORDERWIDTHS = super::LPCRECT;
pub type LPDROPSOURCE = *mut core::ffi::c_void;
pub type LPDROPTARGET = *mut core::ffi::c_void;
pub type LPENUMOLEVERB = *mut core::ffi::c_void;
#[cfg(feature = "windef")]
pub type LPLINKSRCDESCRIPTOR = *mut tagOBJECTDESCRIPTOR;
#[cfg(feature = "windef")]
pub type LPOBJECTDESCRIPTOR = *mut tagOBJECTDESCRIPTOR;
pub type LPOLEADVISEHOLDER = *mut core::ffi::c_void;
pub type LPOLECACHE = *mut core::ffi::c_void;
pub type LPOLECACHE2 = *mut core::ffi::c_void;
pub type LPOLECACHECONTROL = *mut core::ffi::c_void;
pub type LPOLECLIENTSITE = *mut core::ffi::c_void;
pub type LPOLECONTAINER = *mut core::ffi::c_void;
pub type LPOLEINPLACEACTIVEOBJECT = *mut core::ffi::c_void;
pub type LPOLEINPLACEFRAME = *mut core::ffi::c_void;
#[cfg(feature = "windef")]
pub type LPOLEINPLACEFRAMEINFO = *mut OLEINPLACEFRAMEINFO;
pub type LPOLEINPLACEOBJECT = *mut core::ffi::c_void;
pub type LPOLEINPLACESITE = *mut core::ffi::c_void;
pub type LPOLEINPLACEUIWINDOW = *mut core::ffi::c_void;
pub type LPOLEITEMCONTAINER = *mut core::ffi::c_void;
pub type LPOLELINK = *mut core::ffi::c_void;
pub type LPOLEMENUGROUPWIDTHS = *mut OLEMENUGROUPWIDTHS;
pub type LPOLEOBJECT = *mut core::ffi::c_void;
pub type LPOLERENDER = *mut OLERENDER;
pub type LPOLEUPDATE = *mut OLEUPDATE;
#[cfg(feature = "wtypesbase")]
pub type LPOLEVERB = *mut OLEVERB;
pub type LPOLEWINDOW = *mut core::ffi::c_void;
pub type LPPARSEDISPLAYNAME = *mut core::ffi::c_void;
pub type LPVIEWOBJECT = *mut core::ffi::c_void;
pub type LPVIEWOBJECT2 = *mut core::ffi::c_void;
pub const MK_ALT: i32 = 32;
#[cfg(feature = "windef")]
pub type OBJECTDESCRIPTOR = tagOBJECTDESCRIPTOR;
pub type OLECLOSE = i32;
pub const OLECLOSE_NOSAVE: OLECLOSE = 1;
pub const OLECLOSE_PROMPTSAVE: OLECLOSE = 2;
pub const OLECLOSE_SAVEIFDIRTY: OLECLOSE = 0;
pub type OLECONTF = i32;
pub const OLECONTF_EMBEDDINGS: OLECONTF = 1;
pub const OLECONTF_LINKS: OLECONTF = 2;
pub const OLECONTF_ONLYIFRUNNING: OLECONTF = 16;
pub const OLECONTF_ONLYUSER: OLECONTF = 8;
pub const OLECONTF_OTHERS: OLECONTF = 4;
pub type OLEGETMONIKER = i32;
pub const OLEGETMONIKER_FORCEASSIGN: OLEGETMONIKER = 2;
pub const OLEGETMONIKER_ONLYIFTHERE: OLEGETMONIKER = 1;
pub const OLEGETMONIKER_TEMPFORUSER: OLEGETMONIKER = 4;
pub const OLEGETMONIKER_UNASSIGN: OLEGETMONIKER = 3;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct OLEINPLACEFRAMEINFO {
    pub cb: u32,
    pub fMDIApp: windows_sys::core::BOOL,
    pub hwndFrame: super::HWND,
    pub haccel: super::HACCEL,
    pub cAccelEntries: u32,
}
pub type OLELINKBIND = i32;
pub const OLELINKBIND_EVENIFCLASSDIFF: OLELINKBIND = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OLEMENUGROUPWIDTHS {
    pub width: [i32; 6],
}
impl Default for OLEMENUGROUPWIDTHS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OLEMISC = i32;
pub const OLEMISC_ACTIVATEWHENVISIBLE: OLEMISC = 256;
pub const OLEMISC_ACTSLIKEBUTTON: OLEMISC = 4096;
pub const OLEMISC_ACTSLIKELABEL: OLEMISC = 8192;
pub const OLEMISC_ALIGNABLE: OLEMISC = 32768;
pub const OLEMISC_ALWAYSRUN: OLEMISC = 2048;
pub const OLEMISC_CANLINKBYOLE1: OLEMISC = 32;
pub const OLEMISC_CANTLINKINSIDE: OLEMISC = 16;
pub const OLEMISC_IGNOREACTIVATEWHENVISIBLE: OLEMISC = 524288;
pub const OLEMISC_IMEMODE: OLEMISC = 262144;
pub const OLEMISC_INSERTNOTREPLACE: OLEMISC = 4;
pub const OLEMISC_INSIDEOUT: OLEMISC = 128;
pub const OLEMISC_INVISIBLEATRUNTIME: OLEMISC = 1024;
pub const OLEMISC_ISLINKOBJECT: OLEMISC = 64;
pub const OLEMISC_NOUIACTIVATE: OLEMISC = 16384;
pub const OLEMISC_ONLYICONIC: OLEMISC = 2;
pub const OLEMISC_RECOMPOSEONRESIZE: OLEMISC = 1;
pub const OLEMISC_RENDERINGISDEVICEINDEPENDENT: OLEMISC = 512;
pub const OLEMISC_SETCLIENTSITEFIRST: OLEMISC = 131072;
pub const OLEMISC_SIMPLEFRAME: OLEMISC = 65536;
pub const OLEMISC_STATIC: OLEMISC = 8;
pub const OLEMISC_SUPPORTSMULTILEVELUNDO: OLEMISC = 2097152;
pub const OLEMISC_WANTSTOMENUMERGE: OLEMISC = 1048576;
pub type OLERENDER = i32;
pub const OLERENDER_ASIS: OLERENDER = 3;
pub const OLERENDER_DRAW: OLERENDER = 1;
pub const OLERENDER_FORMAT: OLERENDER = 2;
pub const OLERENDER_NONE: OLERENDER = 0;
pub type OLEUPDATE = i32;
pub const OLEUPDATE_ALWAYS: OLEUPDATE = 1;
pub const OLEUPDATE_ONCALL: OLEUPDATE = 3;
#[repr(C)]
#[cfg(feature = "wtypesbase")]
#[derive(Clone, Copy, Default)]
pub struct OLEVERB {
    pub lVerb: i32,
    pub lpszVerbName: super::LPOLESTR,
    pub fuFlags: u32,
    pub grfAttribs: u32,
}
pub type OLEVERBATTRIB = i32;
pub const OLEVERBATTRIB_NEVERDIRTIES: OLEVERBATTRIB = 1;
pub const OLEVERBATTRIB_ONCONTAINERMENU: OLEVERBATTRIB = 2;
pub type OLEWHICHMK = i32;
pub const OLEWHICHMK_CONTAINER: OLEWHICHMK = 1;
pub const OLEWHICHMK_OBJFULL: OLEWHICHMK = 3;
pub const OLEWHICHMK_OBJREL: OLEWHICHMK = 2;
#[cfg(feature = "windef")]
pub type PLINKSRCDESCRIPTOR = *mut tagOBJECTDESCRIPTOR;
#[cfg(feature = "windef")]
pub type POBJECTDESCRIPTOR = *mut tagOBJECTDESCRIPTOR;
pub type POLEUPDATE = *mut OLEUPDATE;
pub const UPDFCACHE_ALL: u32 = 2147483647;
pub const UPDFCACHE_ALLBUTNODATACACHE: u32 = 2147483646;
pub const UPDFCACHE_IFBLANK: i32 = 16;
pub const UPDFCACHE_IFBLANKORONSAVECACHE: i32 = 18;
pub const UPDFCACHE_NODATACACHE: i32 = 1;
pub const UPDFCACHE_NORMALCACHE: i32 = 8;
pub const UPDFCACHE_ONLYIFBLANK: u32 = 2147483648;
pub const UPDFCACHE_ONSAVECACHE: i32 = 2;
pub const UPDFCACHE_ONSTOPCACHE: i32 = 4;
pub type USERCLASSTYPE = i32;
pub const USERCLASSTYPE_APPNAME: USERCLASSTYPE = 3;
pub const USERCLASSTYPE_FULL: USERCLASSTYPE = 1;
pub const USERCLASSTYPE_SHORT: USERCLASSTYPE = 2;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct tagOBJECTDESCRIPTOR {
    pub cbSize: u32,
    pub clsid: windows_sys::core::GUID,
    pub dwDrawAspect: u32,
    pub sizel: super::SIZEL,
    pub pointl: super::POINTL,
    pub dwStatus: u32,
    pub dwFullUserTypeName: u32,
    pub dwSrcOfCopy: u32,
}
