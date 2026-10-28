#[cfg(feature = "windef")]
windows_link::link!("shdocvw.dll" "system" fn DoPrivacyDlg(hwndowner : super::HWND, pszurl : windows_sys::core::PCWSTR, pprivacyenum : *const IEnumPrivacyRecords, freportallsites : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("shdocvw.dll" "system" fn ImportPrivacySettings(pszfilename : windows_sys::core::PCWSTR, pfparseprivacypreferences : *mut windows_sys::core::BOOL, pfparsepersiterules : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn PathIsSlowA(pszfile : windows_sys::core::PCSTR, dwattr : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("shell32.dll" "system" fn PathIsSlowW(pszfile : windows_sys::core::PCWSTR, dwattr : super::DWORD) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn PathQualify(psz : windows_sys::core::PWSTR));
#[cfg(feature = "shlobj_core")]
windows_link::link!("shell32.dll" "system" fn SHChangeNotifyRegisterThread(status : super::SCNRT_STATUS));
#[cfg(all(feature = "minwindef", feature = "shared", feature = "shlobj_core"))]
windows_link::link!("shell32.dll" "system" fn SHCreatePropSheetExtArray(hkey : super::HKEY, pszsubkey : windows_sys::core::PCWSTR, max_iface : super::UINT) -> super::HPSXA);
#[cfg(feature = "objidl")]
windows_link::link!("shell32.dll" "system" fn SHCreateQueryCancelAutoPlayMoniker(ppmoniker : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared"))]
windows_link::link!("shell32.dll" "system" fn SHMultiFileProperties(pdtobj : *mut core::ffi::c_void, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "shobjidl_core"))]
windows_link::link!("shell32.dll" "system" fn SHOpenPropSheetW(pszcaption : windows_sys::core::PCWSTR, ahkeys : *const super::HKEY, ckeys : super::UINT, pclsiddefault : *const super::CLSID, pdtobj : *mut core::ffi::c_void, psb : *mut core::ffi::c_void, pstartpage : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "urlmon", feature = "windef"))]
windows_link::link!("shdocvw.dll" "system" fn SoftwareUpdateMessageBox(hwnd : super::HWND, pszdistunit : windows_sys::core::PCWSTR, dwflags : super::DWORD, psdi : super::LPSOFTDISTINFO) -> super::DWORD);
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct AASHELLMENUFILENAME {
    pub cbTotal: super::SHORT,
    pub rgbReserved: [super::BYTE; 12],
    pub szFileName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for AASHELLMENUFILENAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct AASHELLMENUITEM {
    pub lpReserved1: *mut core::ffi::c_void,
    pub iReserved: i32,
    pub uiReserved: super::UINT,
    pub lpName: LPAASHELLMENUFILENAME,
    pub psz: windows_sys::core::PWSTR,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct BANDINFOSFB {
    pub dwMask: super::DWORD,
    pub dwStateMask: super::DWORD,
    pub dwState: super::DWORD,
    pub crBkgnd: super::COLORREF,
    pub crBtnLt: super::COLORREF,
    pub crBtnDk: super::COLORREF,
    pub wViewMode: super::WORD,
    pub wAlign: super::WORD,
    pub psf: *mut core::ffi::c_void,
    pub pidl: super::LPITEMIDLIST,
}
pub const BMICON_LARGE: i32 = 0;
pub const BMICON_SMALL: i32 = 1;
pub const CLOSEPROPS_DISCARD: i32 = 1;
pub const CLOSEPROPS_NONE: i32 = 0;
pub const DBCID_CLSIDOFBAR: i32 = 2;
pub const DBCID_EMPTY: i32 = 0;
pub const DBCID_GETBAR: i32 = 4;
pub const DBCID_ONDRAG: i32 = 1;
pub const DBCID_RESIZE: i32 = 3;
pub const DBCID_UPDATESIZE: i32 = 5;
pub const DBC_GS_IDEAL: i32 = 0;
pub const DBC_GS_SIZEDOWN: i32 = 1;
pub const DBC_HIDE: i32 = 0;
pub const DBC_SHOW: i32 = 1;
pub const DBC_SHOWOBSCURE: i32 = 2;
pub const DWFAF_AUTOHIDE: i32 = 16;
pub const DWFAF_GROUP1: i32 = 2;
pub const DWFAF_GROUP2: i32 = 4;
pub const DWFAF_HIDDEN: i32 = 1;
pub const DWFRF_DELETECONFIGDATA: i32 = 1;
pub const DWFRF_NORMAL: i32 = 0;
pub const FCIDM_STATUS: i32 = 40961;
pub const FCIDM_TOOLBAR: i32 = 40960;
pub const GADOF_DIRTY: i32 = 1;
pub const GETPROPS_NONE: i32 = 0;
pub const IDC_OFFLINE_HAND: i32 = 103;
pub const IDC_PANTOOL_HAND_CLOSED: i32 = 105;
pub const IDC_PANTOOL_HAND_OPEN: i32 = 104;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IEnumPrivacyRecords(pub u8);
pub const ISFBVIEWMODE_LARGEICONS: i32 = 2;
pub const ISFBVIEWMODE_SMALLICONS: i32 = 1;
pub const ISFB_MASK_BKCOLOR: i32 = 2;
pub const ISFB_MASK_COLORS: i32 = 32;
pub const ISFB_MASK_IDLIST: i32 = 16;
pub const ISFB_MASK_SHELLFOLDER: i32 = 8;
pub const ISFB_MASK_STATE: i32 = 1;
pub const ISFB_MASK_VIEWMODE: i32 = 4;
pub const ISFB_STATE_ALLOWRENAME: i32 = 2;
pub const ISFB_STATE_BTNMINSIZE: i32 = 256;
pub const ISFB_STATE_CHANNELBAR: i32 = 16;
pub const ISFB_STATE_DEBOSSED: i32 = 1;
pub const ISFB_STATE_DEFAULT: i32 = 0;
pub const ISFB_STATE_FULLOPEN: i32 = 64;
pub const ISFB_STATE_NONAMESORT: i32 = 128;
pub const ISFB_STATE_NOSHOWTEXT: i32 = 4;
pub const ISFB_STATE_QLINKSMODE: i32 = 32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPAASHELLMENUFILENAME = *mut AASHELLMENUFILENAME;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPAASHELLMENUITEM = *mut AASHELLMENUITEM;
pub type LPACTIVEDESKTOPP = *mut core::ffi::c_void;
pub type LPADESKTOPP2 = *mut core::ffi::c_void;
pub type LPCOPYHOOKA = *mut core::ffi::c_void;
pub type LPCOPYHOOKW = *mut core::ffi::c_void;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPCSHCOLUMNDATA = *const SHCOLUMNDATA;
#[cfg(all(feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes"))]
pub type LPCSHCOLUMNINFO = *const SHCOLUMNINFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPCSHCOLUMNINIT = *const SHCOLUMNINIT;
#[cfg(all(feature = "shared", feature = "shtypes", feature = "windef"))]
pub type LPSFV_SETITEMPOS = *mut SFV_SETITEMPOS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSHCOLUMNDATA = *mut SHCOLUMNDATA;
#[cfg(all(feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes"))]
pub type LPSHCOLUMNINFO = *mut SHCOLUMNINFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSHCOLUMNINIT = *mut SHCOLUMNINIT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSHChangeProductKeyAsIDList = *mut SHChangeProductKeyAsIDList;
#[cfg(feature = "shared")]
pub type LPTBINFO = *mut TBINFO;
pub type LPTHUMBNAILCAPTURE = *mut core::ffi::c_void;
pub const OPENPROPS_INHIBITPIF: i32 = 32768;
pub const OPENPROPS_NONE: i32 = 0;
pub const PANE_NAVIGATION: i32 = 5;
#[cfg(feature = "shared")]
pub const PANE_NONE: super::DWORD = 4294967295;
pub const PANE_OFFLINE: i32 = 2;
pub const PANE_PRINTER: i32 = 3;
pub const PANE_PRIVACY: i32 = 7;
pub const PANE_PROGRESS: i32 = 6;
pub const PANE_SSL: i32 = 4;
pub const PANE_ZONE: i32 = 1;
#[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub type PBANDINFOSFB = *mut BANDINFOSFB;
#[cfg(all(feature = "shared", feature = "shtypes", feature = "windef"))]
pub type PCSFV_SETITEMPOS = *const SFV_SETITEMPOS;
pub const SCHEME_CREATE: i32 = 128;
pub const SCHEME_DISPLAY: i32 = 1;
pub const SCHEME_DONOTUSE: i32 = 64;
pub const SCHEME_EDIT: i32 = 2;
pub const SCHEME_GLOBAL: i32 = 8;
pub const SCHEME_LOCAL: i32 = 4;
pub const SCHEME_REFRESH: i32 = 16;
pub const SCHEME_UPDATE: i32 = 32;
pub const SETPROPS_NONE: i32 = 0;
pub const SFBID_PIDLCHANGED: i32 = 0;
pub const SFVM_ADDOBJECT: i32 = 3;
pub const SFVM_GETSELECTEDOBJECTS: i32 = 9;
pub const SFVM_REARRANGE: i32 = 1;
pub const SFVM_REMOVEOBJECT: i32 = 6;
pub const SFVM_SETCLIPBOARD: i32 = 16;
pub const SFVM_SETITEMPOS: i32 = 14;
pub const SFVM_SETPOINTS: i32 = 23;
pub const SFVM_UPDATEOBJECT: i32 = 7;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct SFV_SETITEMPOS {
    pub pidl: super::LPCITEMIDLIST,
    pub pt: super::POINT,
}
pub const SHCDF_UPDATEITEM: i32 = 1;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SHCOLUMNDATA {
    pub dwFlags: super::ULONG,
    pub dwFileAttributes: super::DWORD,
    pub dwReserved: super::ULONG,
    pub pwszExt: *mut super::WCHAR,
    pub wszFile: [super::WCHAR; 260],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SHCOLUMNDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes"))]
#[derive(Clone, Copy)]
pub struct SHCOLUMNINFO {
    pub scid: super::SHCOLUMNID,
    pub vt: super::VARTYPE,
    pub fmt: super::DWORD,
    pub cChars: super::UINT,
    pub csFlags: super::DWORD,
    pub wszTitle: [super::WCHAR; 80],
    pub wszDescription: [super::WCHAR; 128],
}
#[cfg(all(feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes"))]
impl Default for SHCOLUMNINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SHCOLUMNINIT {
    pub dwFlags: super::ULONG,
    pub dwReserved: super::ULONG,
    pub wszFolder: [super::WCHAR; 260],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SHCOLUMNINIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SHChangeProductKeyAsIDList {
    pub cb: super::USHORT,
    pub wszProductKey: [super::WCHAR; 39],
    pub cbZero: super::USHORT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SHChangeProductKeyAsIDList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SSM_CLEAR: i32 = 0;
pub const SSM_REFRESH: i32 = 2;
pub const SSM_SET: i32 = 1;
pub const SSM_UPDATE: i32 = 4;
pub const TBIF_APPEND: i32 = 0;
pub const TBIF_DEFAULT: i32 = 0;
pub const TBIF_INTERNETBAR: i32 = 65536;
pub const TBIF_NOTOOLBAR: i32 = 196608;
pub const TBIF_PREPEND: i32 = 1;
pub const TBIF_REPLACE: i32 = 2;
pub const TBIF_STANDARDTOOLBAR: i32 = 131072;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TBINFO {
    pub cbuttons: super::UINT,
    pub uFlags: super::UINT,
}
