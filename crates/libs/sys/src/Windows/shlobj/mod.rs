#[cfg(feature = "windef")]
windows_link::link!("shdocvw.dll" "system" fn DoPrivacyDlg(hwndowner : super::windef::HWND, pszurl : windows_sys::core::PCWSTR, pprivacyenum : *const IEnumPrivacyRecords, freportallsites : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("shdocvw.dll" "system" fn ImportPrivacySettings(pszfilename : windows_sys::core::PCWSTR, pfparseprivacypreferences : *mut windows_sys::core::BOOL, pfparsepersiterules : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn PathIsSlowA(pszfile : windows_sys::core::PCSTR, dwattr : u32) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn PathIsSlowW(pszfile : windows_sys::core::PCWSTR, dwattr : u32) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn PathQualify(psz : windows_sys::core::PWSTR));
#[cfg(feature = "shlobj_core")]
windows_link::link!("shell32.dll" "system" fn SHChangeNotifyRegisterThread(status : super::shlobj_core::SCNRT_STATUS));
#[cfg(all(feature = "minwindef", feature = "shlobj_core"))]
windows_link::link!("shell32.dll" "system" fn SHCreatePropSheetExtArray(hkey : super::minwindef::HKEY, pszsubkey : windows_sys::core::PCWSTR, max_iface : u32) -> super::shlobj_core::HPSXA);
#[cfg(feature = "objidl")]
windows_link::link!("shell32.dll" "system" fn SHCreateQueryCancelAutoPlayMoniker(ppmoniker : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("shell32.dll" "system" fn SHMultiFileProperties(pdtobj : *mut core::ffi::c_void, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shobjidl_core"))]
windows_link::link!("shell32.dll" "system" fn SHOpenPropSheetW(pszcaption : windows_sys::core::PCWSTR, ahkeys : *const super::minwindef::HKEY, ckeys : u32, pclsiddefault : *const windows_sys::core::GUID, pdtobj : *mut core::ffi::c_void, psb : *mut core::ffi::c_void, pstartpage : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "urlmon", feature = "windef"))]
windows_link::link!("shdocvw.dll" "system" fn SoftwareUpdateMessageBox(hwnd : super::windef::HWND, pszdistunit : windows_sys::core::PCWSTR, dwflags : u32, psdi : *mut super::urlmon::SOFTDISTINFO) -> u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AASHELLMENUFILENAME {
    pub cbTotal: i16,
    pub rgbReserved: [u8; 12],
    pub szFileName: [u16; 1],
}
impl Default for AASHELLMENUFILENAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AASHELLMENUITEM {
    pub lpReserved1: *mut core::ffi::c_void,
    pub iReserved: i32,
    pub uiReserved: u32,
    pub lpName: LPAASHELLMENUFILENAME,
    pub psz: windows_sys::core::PWSTR,
}
impl Default for AASHELLMENUITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct BANDINFOSFB {
    pub dwMask: u32,
    pub dwStateMask: u32,
    pub dwState: u32,
    pub crBkgnd: super::windef::COLORREF,
    pub crBtnLt: super::windef::COLORREF,
    pub crBtnDk: super::windef::COLORREF,
    pub wViewMode: u16,
    pub wAlign: u16,
    pub psf: *mut core::ffi::c_void,
    pub pidl: super::shtypes::LPITEMIDLIST,
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
impl Default for BANDINFOSFB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BMICON_LARGE: i32 = 0;
pub const BMICON_SMALL: i32 = 1;
pub const CLOSEPROPS_DISCARD: u32 = 1;
pub const CLOSEPROPS_NONE: u32 = 0;
pub const DBCID_CLSIDOFBAR: i32 = 2;
pub const DBCID_EMPTY: i32 = 0;
pub const DBCID_GETBAR: i32 = 4;
pub const DBCID_ONDRAG: i32 = 1;
pub const DBCID_RESIZE: i32 = 3;
pub const DBCID_UPDATESIZE: i32 = 5;
pub const DBC_GS_IDEAL: u32 = 0;
pub const DBC_GS_SIZEDOWN: u32 = 1;
pub const DBC_HIDE: u32 = 0;
pub const DBC_SHOW: u32 = 1;
pub const DBC_SHOWOBSCURE: u32 = 2;
pub const DWFAF_AUTOHIDE: u32 = 16;
pub const DWFAF_GROUP1: u32 = 2;
pub const DWFAF_GROUP2: u32 = 4;
pub const DWFAF_HIDDEN: u32 = 1;
pub const DWFRF_DELETECONFIGDATA: u32 = 1;
pub const DWFRF_NORMAL: u32 = 0;
pub const FCIDM_STATUS: u32 = 40961;
pub const FCIDM_TOOLBAR: u32 = 40960;
pub const GADOF_DIRTY: u32 = 1;
pub const GETPROPS_NONE: u32 = 0;
pub const IDC_OFFLINE_HAND: u32 = 103;
pub const IDC_PANTOOL_HAND_CLOSED: u32 = 105;
pub const IDC_PANTOOL_HAND_OPEN: u32 = 104;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IEnumPrivacyRecords(pub u8);
pub const ISFBVIEWMODE_LARGEICONS: u32 = 2;
pub const ISFBVIEWMODE_SMALLICONS: u32 = 1;
pub const ISFB_MASK_BKCOLOR: u32 = 2;
pub const ISFB_MASK_COLORS: u32 = 32;
pub const ISFB_MASK_IDLIST: u32 = 16;
pub const ISFB_MASK_SHELLFOLDER: u32 = 8;
pub const ISFB_MASK_STATE: u32 = 1;
pub const ISFB_MASK_VIEWMODE: u32 = 4;
pub const ISFB_STATE_ALLOWRENAME: u32 = 2;
pub const ISFB_STATE_BTNMINSIZE: u32 = 256;
pub const ISFB_STATE_CHANNELBAR: u32 = 16;
pub const ISFB_STATE_DEBOSSED: u32 = 1;
pub const ISFB_STATE_DEFAULT: u32 = 0;
pub const ISFB_STATE_FULLOPEN: u32 = 64;
pub const ISFB_STATE_NONAMESORT: u32 = 128;
pub const ISFB_STATE_NOSHOWTEXT: u32 = 4;
pub const ISFB_STATE_QLINKSMODE: u32 = 32;
pub type LPAASHELLMENUFILENAME = *mut AASHELLMENUFILENAME;
pub type LPAASHELLMENUITEM = *mut AASHELLMENUITEM;
pub type LPCSHCOLUMNDATA = *const SHCOLUMNDATA;
#[cfg(all(feature = "shtypes", feature = "wtypes"))]
pub type LPCSHCOLUMNINFO = *const SHCOLUMNINFO;
pub type LPCSHCOLUMNINIT = *const SHCOLUMNINIT;
#[cfg(all(feature = "shtypes", feature = "windef"))]
pub type LPSFV_SETITEMPOS = *mut SFV_SETITEMPOS;
pub type LPSHCOLUMNDATA = *mut SHCOLUMNDATA;
#[cfg(all(feature = "shtypes", feature = "wtypes"))]
pub type LPSHCOLUMNINFO = *mut SHCOLUMNINFO;
pub type LPSHCOLUMNINIT = *mut SHCOLUMNINIT;
pub type LPSHChangeProductKeyAsIDList = *mut SHChangeProductKeyAsIDList;
pub type LPTBINFO = *mut TBINFO;
pub const OPENPROPS_INHIBITPIF: u32 = 32768;
pub const OPENPROPS_NONE: u32 = 0;
pub const PANE_NAVIGATION: u32 = 5;
pub const PANE_NONE: u32 = 4294967295;
pub const PANE_OFFLINE: u32 = 2;
pub const PANE_PRINTER: u32 = 3;
pub const PANE_PRIVACY: u32 = 7;
pub const PANE_PROGRESS: u32 = 6;
pub const PANE_SSL: u32 = 4;
pub const PANE_ZONE: u32 = 1;
#[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub type PBANDINFOSFB = *mut BANDINFOSFB;
#[cfg(all(feature = "shtypes", feature = "windef"))]
pub type PCSFV_SETITEMPOS = *const SFV_SETITEMPOS;
pub const SCHEME_CREATE: u32 = 128;
pub const SCHEME_DISPLAY: u32 = 1;
pub const SCHEME_DONOTUSE: u32 = 64;
pub const SCHEME_EDIT: u32 = 2;
pub const SCHEME_GLOBAL: u32 = 8;
pub const SCHEME_LOCAL: u32 = 4;
pub const SCHEME_REFRESH: u32 = 16;
pub const SCHEME_UPDATE: u32 = 32;
pub const SETPROPS_NONE: u32 = 0;
pub const SFBID_PIDLCHANGED: i32 = 0;
pub const SFVM_ADDOBJECT: u32 = 3;
pub const SFVM_GETSELECTEDOBJECTS: u32 = 9;
pub const SFVM_REARRANGE: u32 = 1;
pub const SFVM_REMOVEOBJECT: u32 = 6;
pub const SFVM_SETCLIPBOARD: u32 = 16;
pub const SFVM_SETITEMPOS: u32 = 14;
pub const SFVM_SETPOINTS: u32 = 23;
pub const SFVM_UPDATEOBJECT: u32 = 7;
#[repr(C)]
#[cfg(all(feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct SFV_SETITEMPOS {
    pub pidl: super::shtypes::LPCITEMIDLIST,
    pub pt: super::windef::POINT,
}
#[cfg(all(feature = "shtypes", feature = "windef"))]
impl Default for SFV_SETITEMPOS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SHCDF_UPDATEITEM: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SHCOLUMNDATA {
    pub dwFlags: u32,
    pub dwFileAttributes: u32,
    pub dwReserved: u32,
    pub pwszExt: *mut u16,
    pub wszFile: [u16; 260],
}
impl Default for SHCOLUMNDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shtypes", feature = "wtypes"))]
#[derive(Clone, Copy)]
pub struct SHCOLUMNINFO {
    pub scid: super::shtypes::SHCOLUMNID,
    pub vt: super::wtypes::VARTYPE,
    pub fmt: u32,
    pub cChars: u32,
    pub csFlags: u32,
    pub wszTitle: [u16; 80],
    pub wszDescription: [u16; 128],
}
#[cfg(all(feature = "shtypes", feature = "wtypes"))]
impl Default for SHCOLUMNINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SHCOLUMNINIT {
    pub dwFlags: u32,
    pub dwReserved: u32,
    pub wszFolder: [u16; 260],
}
impl Default for SHCOLUMNINIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SHChangeProductKeyAsIDList {
    pub cb: u16,
    pub wszProductKey: [u16; 39],
    pub cbZero: u16,
}
impl Default for SHChangeProductKeyAsIDList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SSM_CLEAR: u32 = 0;
pub const SSM_REFRESH: u32 = 2;
pub const SSM_SET: u32 = 1;
pub const SSM_UPDATE: u32 = 4;
pub const TBIF_APPEND: u32 = 0;
pub const TBIF_DEFAULT: u32 = 0;
pub const TBIF_INTERNETBAR: u32 = 65536;
pub const TBIF_NOTOOLBAR: u32 = 196608;
pub const TBIF_PREPEND: u32 = 1;
pub const TBIF_REPLACE: u32 = 2;
pub const TBIF_STANDARDTOOLBAR: u32 = 131072;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TBINFO {
    pub cbuttons: u32,
    pub uFlags: u32,
}
