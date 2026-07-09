pub type BINDSPEED = i32;
pub const BINDSPEED_IMMEDIATE: BINDSPEED = 3;
pub const BINDSPEED_INDEFINITE: BINDSPEED = 1;
pub const BINDSPEED_MODERATE: BINDSPEED = 2;
#[cfg(feature = "Win32_windef")]
pub type BORDERWIDTHS = super::windef::RECT;
pub const DD_DEFDRAGDELAY: u32 = 200;
pub const DD_DEFDRAGMINDIST: u32 = 2;
pub const DD_DEFSCROLLDELAY: u32 = 50;
pub const DD_DEFSCROLLINSET: u32 = 11;
pub const DD_DEFSCROLLINTERVAL: u32 = 50;
pub type DISCARDCACHE = i32;
pub const DISCARDCACHE_NOSAVE: DISCARDCACHE = 1;
pub const DISCARDCACHE_SAVEIFDIRTY: DISCARDCACHE = 0;
pub const DROPEFFECT_BACKGROUNDTARGET: u32 = 536870912;
pub const DROPEFFECT_COPY: u32 = 1;
pub const DROPEFFECT_LINK: u32 = 4;
pub const DROPEFFECT_MOVE: u32 = 2;
pub const DROPEFFECT_NEWTARGET: u32 = 1073741824;
pub const DROPEFFECT_NONE: u32 = 0;
pub const DROPEFFECT_SCROLL: u32 = 2147483648;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type HOLEMENU = super::minwindef::HGLOBAL;
#[cfg(feature = "Win32_windef")]
pub type LINKSRCDESCRIPTOR = OBJECTDESCRIPTOR;
#[cfg(feature = "Win32_windef")]
pub type LPBORDERWIDTHS = super::windef::LPRECT;
#[cfg(feature = "Win32_windef")]
pub type LPCBORDERWIDTHS = super::windef::LPCRECT;
#[cfg(feature = "Win32_windef")]
pub type LPLINKSRCDESCRIPTOR = *mut OBJECTDESCRIPTOR;
#[cfg(feature = "Win32_windef")]
pub type LPOBJECTDESCRIPTOR = *mut OBJECTDESCRIPTOR;
#[cfg(feature = "Win32_windef")]
pub type LPOLEINPLACEFRAMEINFO = *mut OLEINPLACEFRAMEINFO;
pub type LPOLEMENUGROUPWIDTHS = *mut OLEMENUGROUPWIDTHS;
pub type LPOLERENDER = *mut OLERENDER;
pub type LPOLEUPDATE = *mut OLEUPDATE;
pub type LPOLEVERB = *mut OLEVERB;
pub const MK_ALT: u32 = 32;
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct OBJECTDESCRIPTOR {
    pub cbSize: u32,
    pub clsid: windows_sys::core::GUID,
    pub dwDrawAspect: u32,
    pub sizel: super::windef::SIZEL,
    pub pointl: super::windef::POINTL,
    pub dwStatus: u32,
    pub dwFullUserTypeName: u32,
    pub dwSrcOfCopy: u32,
}
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
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct OLEINPLACEFRAMEINFO {
    pub cb: u32,
    pub fMDIApp: windows_sys::core::BOOL,
    pub hwndFrame: super::windef::HWND,
    pub haccel: super::windef::HACCEL,
    pub cAccelEntries: u32,
}
#[cfg(feature = "Win32_windef")]
impl Default for OLEINPLACEFRAMEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[derive(Clone, Copy)]
pub struct OLEVERB {
    pub lVerb: i32,
    pub lpszVerbName: windows_sys::core::PWSTR,
    pub fuFlags: u32,
    pub grfAttribs: u32,
}
impl Default for OLEVERB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OLEVERBATTRIB = i32;
pub const OLEVERBATTRIB_NEVERDIRTIES: OLEVERBATTRIB = 1;
pub const OLEVERBATTRIB_ONCONTAINERMENU: OLEVERBATTRIB = 2;
pub type OLEWHICHMK = i32;
pub const OLEWHICHMK_CONTAINER: OLEWHICHMK = 1;
pub const OLEWHICHMK_OBJFULL: OLEWHICHMK = 3;
pub const OLEWHICHMK_OBJREL: OLEWHICHMK = 2;
#[cfg(feature = "Win32_windef")]
pub type PLINKSRCDESCRIPTOR = *mut OBJECTDESCRIPTOR;
#[cfg(feature = "Win32_windef")]
pub type POBJECTDESCRIPTOR = *mut OBJECTDESCRIPTOR;
pub type POLEUPDATE = *mut OLEUPDATE;
pub const UPDFCACHE_ALL: u32 = 2147483647;
pub const UPDFCACHE_ALLBUTNODATACACHE: u32 = 2147483646;
pub const UPDFCACHE_IFBLANK: u32 = 16;
pub const UPDFCACHE_IFBLANKORONSAVECACHE: u32 = 18;
pub const UPDFCACHE_NODATACACHE: u32 = 1;
pub const UPDFCACHE_NORMALCACHE: u32 = 8;
pub const UPDFCACHE_ONLYIFBLANK: u32 = 2147483648;
pub const UPDFCACHE_ONSAVECACHE: u32 = 2;
pub const UPDFCACHE_ONSTOPCACHE: u32 = 4;
pub type USERCLASSTYPE = i32;
pub const USERCLASSTYPE_APPNAME: USERCLASSTYPE = 3;
pub const USERCLASSTYPE_FULL: USERCLASSTYPE = 1;
pub const USERCLASSTYPE_SHORT: USERCLASSTYPE = 2;
