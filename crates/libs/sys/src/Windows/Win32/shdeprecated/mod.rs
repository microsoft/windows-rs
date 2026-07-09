#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub type BASEBROWSERDATA = BASEBROWSERDATALH;
#[repr(C)]
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct BASEBROWSERDATALH {
    pub _hwnd: super::windef::HWND,
    pub _ptl: *mut core::ffi::c_void,
    pub _phlf: *mut core::ffi::c_void,
    pub _pautoWB2: *mut core::ffi::c_void,
    pub _pautoEDS: *mut core::ffi::c_void,
    pub _pautoSS: *mut core::ffi::c_void,
    pub _eSecureLockIcon: i32,
    pub _bitfield: u32,
    pub _uActivateState: u32,
    pub _pidlViewState: super::shtypes::LPCITEMIDLIST,
    pub _pctView: *mut core::ffi::c_void,
    pub _pidlCur: super::shtypes::LPITEMIDLIST,
    pub _psv: *mut core::ffi::c_void,
    pub _psf: *mut core::ffi::c_void,
    pub _hwndView: super::windef::HWND,
    pub _pszTitleCur: windows_sys::core::PWSTR,
    pub _pidlPending: super::shtypes::LPITEMIDLIST,
    pub _psvPending: *mut core::ffi::c_void,
    pub _psfPending: *mut core::ffi::c_void,
    pub _hwndViewPending: super::windef::HWND,
    pub _pszTitlePending: windows_sys::core::PWSTR,
    pub _fIsViewMSHTML: windows_sys::core::BOOL,
    pub _fPrivacyImpacted: windows_sys::core::BOOL,
    pub _clsidView: windows_sys::core::GUID,
    pub _clsidViewPending: windows_sys::core::GUID,
    pub _hwndFrame: super::windef::HWND,
    pub _lPhishingFilterStatus: i32,
}
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
impl Default for BASEBROWSERDATALH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct BASEBROWSERDATAXP {
    pub _hwnd: super::windef::HWND,
    pub _ptl: *mut core::ffi::c_void,
    pub _phlf: *mut core::ffi::c_void,
    pub _pautoWB2: *mut core::ffi::c_void,
    pub _pautoEDS: *mut core::ffi::c_void,
    pub _pautoSS: *mut core::ffi::c_void,
    pub _eSecureLockIcon: i32,
    pub _bitfield: u32,
    pub _uActivateState: u32,
    pub _pidlViewState: super::shtypes::LPCITEMIDLIST,
    pub _pctView: *mut core::ffi::c_void,
    pub _pidlCur: super::shtypes::LPITEMIDLIST,
    pub _psv: *mut core::ffi::c_void,
    pub _psf: *mut core::ffi::c_void,
    pub _hwndView: super::windef::HWND,
    pub _pszTitleCur: windows_sys::core::PWSTR,
    pub _pidlPending: super::shtypes::LPITEMIDLIST,
    pub _psvPending: *mut core::ffi::c_void,
    pub _psfPending: *mut core::ffi::c_void,
    pub _hwndViewPending: super::windef::HWND,
    pub _pszTitlePending: windows_sys::core::PWSTR,
    pub _fIsViewMSHTML: windows_sys::core::BOOL,
    pub _fPrivacyImpacted: windows_sys::core::BOOL,
    pub _clsidView: windows_sys::core::GUID,
    pub _clsidViewPending: windows_sys::core::GUID,
    pub _hwndFrame: super::windef::HWND,
}
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
impl Default for BASEBROWSERDATAXP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BNSTATE = i32;
pub const BNS_BEGIN_NAVIGATE: BNSTATE = 1;
pub const BNS_NAVIGATE: BNSTATE = 2;
pub const BNS_NORMAL: BNSTATE = 0;
pub const BSF_CANMAXIMIZE: u32 = 1024;
pub const BSF_DELEGATEDNAVIGATION: u32 = 65536;
pub const BSF_DONTSHOWNAVCANCELPAGE: u32 = 16384;
pub const BSF_FEEDNAVIGATION: u32 = 524288;
pub const BSF_FEEDSUBSCRIBED: u32 = 1048576;
pub const BSF_HTMLNAVCANCELED: u32 = 8192;
pub const BSF_MERGEDMENUS: u32 = 262144;
pub const BSF_NAVNOHISTORY: u32 = 4096;
pub const BSF_NOLOCALFILEWARNING: u32 = 16;
pub const BSF_REGISTERASDROPTARGET: u32 = 1;
pub const BSF_RESIZABLE: u32 = 512;
pub const BSF_SETNAVIGATABLECODEPAGE: u32 = 32768;
pub const BSF_THEATERMODE: u32 = 2;
pub const BSF_TOPBROWSER: u32 = 2048;
pub const BSF_TRUSTEDFORACTIVEX: u32 = 131072;
pub const BSF_UISETBYAUTOMATION: u32 = 256;
#[repr(C)]
#[cfg(feature = "shobjidl_core")]
#[derive(Clone, Copy, Default)]
pub struct FOLDERSETDATA {
    pub _fs: super::shobjidl_core::FOLDERSETTINGS,
    pub _vidRestore: super::shobjidl_core::SHELLVIEWID,
    pub _dwViewPriority: u32,
}
pub const HLNF_ALLOW_AUTONAVIGATE: u32 = 536870912;
pub const HLNF_CALLERUNTRUSTED: u32 = 2097152;
pub const HLNF_DISABLEWINDOWRESTRICTIONS: u32 = 8388608;
pub const HLNF_EXTERNALNAVIGATE: u32 = 268435456;
pub const HLNF_NEWWINDOWSMANAGED: u32 = 2147483648;
pub const HLNF_TRUSTEDFORACTIVEX: u32 = 4194304;
pub const HLNF_TRUSTFIRSTDOWNLOAD: u32 = 16777216;
pub const HLNF_UNTRUSTEDFORDOWNLOAD: u32 = 33554432;
pub type IEPDNFLAGS = i32;
pub const IEPDN_BINDINGUI: IEPDNFLAGS = 1;
pub const ITB_VIEW: u32 = 4294967295;
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub type LPBASEBROWSERDATA = *mut BASEBROWSERDATA;
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub type LPBASEBROWSERDATALH = *mut BASEBROWSERDATALH;
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub type LPBASEBROWSERDATAXP = *mut BASEBROWSERDATAXP;
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub type LPCBASEBROWSERDATA = *const BASEBROWSERDATA;
#[cfg(feature = "shobjidl_core")]
pub type LPFOLDERSETDATA = *mut FOLDERSETDATA;
#[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "windef"))]
pub type LPTOOLBARITEM = *mut TOOLBARITEM;
pub const SBSC_HIDE: SHELLBROWSERSHOWCONTROL = 0;
pub const SBSC_QUERY: SHELLBROWSERSHOWCONTROL = 3;
pub const SBSC_SHOW: SHELLBROWSERSHOWCONTROL = 1;
pub const SBSC_TOGGLE: SHELLBROWSERSHOWCONTROL = 2;
pub type SECURELOCKCODE = i32;
pub const SECURELOCK_FIRSTSUGGEST: SECURELOCKCODE = 7;
pub const SECURELOCK_NOCHANGE: SECURELOCKCODE = -1;
pub const SECURELOCK_SET_FORTEZZA: SECURELOCKCODE = 5;
pub const SECURELOCK_SET_MIXED: SECURELOCKCODE = 1;
pub const SECURELOCK_SET_SECURE128BIT: SECURELOCKCODE = 6;
pub const SECURELOCK_SET_SECURE40BIT: SECURELOCKCODE = 3;
pub const SECURELOCK_SET_SECURE56BIT: SECURELOCKCODE = 4;
pub const SECURELOCK_SET_SECUREUNKNOWNBIT: SECURELOCKCODE = 2;
pub const SECURELOCK_SET_UNSECURE: SECURELOCKCODE = 0;
pub const SECURELOCK_SUGGEST_FORTEZZA: SECURELOCKCODE = 12;
pub const SECURELOCK_SUGGEST_MIXED: SECURELOCKCODE = 8;
pub const SECURELOCK_SUGGEST_SECURE128BIT: SECURELOCKCODE = 13;
pub const SECURELOCK_SUGGEST_SECURE40BIT: SECURELOCKCODE = 10;
pub const SECURELOCK_SUGGEST_SECURE56BIT: SECURELOCKCODE = 11;
pub const SECURELOCK_SUGGEST_SECUREUNKNOWNBIT: SECURELOCKCODE = 9;
pub const SECURELOCK_SUGGEST_UNSECURE: SECURELOCKCODE = 7;
pub type SHELLBROWSERSHOWCONTROL = i32;
pub const SHHLNF_NOAUTOSELECT: u32 = 67108864;
pub const SHHLNF_WRITENOHISTORY: u32 = 134217728;
pub const TLMENUF_BACK: u32 = 16;
pub const TLMENUF_BACKANDFORTH: u32 = 49;
pub const TLMENUF_CHECKCURRENT: u32 = 3;
pub const TLMENUF_FORE: u32 = 32;
pub const TLMENUF_INCLUDECURRENT: u32 = 1;
pub const TLOG_BACK: i32 = -1;
pub const TLOG_CURRENT: u32 = 0;
pub const TLOG_FORE: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct TOOLBARITEM {
    pub ptbar: *mut core::ffi::c_void,
    pub rcBorderTool: super::oleidl::BORDERWIDTHS,
    pub pwszItem: windows_sys::core::PWSTR,
    pub fShow: windows_sys::core::BOOL,
    pub hMon: super::windef::HMONITOR,
}
#[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "windef"))]
impl Default for TOOLBARITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TrackShellMenu: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8278f931_2a3e_11d2_838f_00c04fd918d0);
pub const VIEW_PRIORITY_CACHEHIT: u32 = 80;
pub const VIEW_PRIORITY_CACHEMISS: u32 = 48;
pub const VIEW_PRIORITY_DESPERATE: u32 = 16;
pub const VIEW_PRIORITY_INHERIT: u32 = 32;
pub const VIEW_PRIORITY_NONE: u32 = 0;
pub const VIEW_PRIORITY_RESTRICTED: u32 = 112;
pub const VIEW_PRIORITY_SHELLEXT: u32 = 64;
pub const VIEW_PRIORITY_SHELLEXT_ASBACKUP: u32 = 21;
pub const VIEW_PRIORITY_STALECACHEHIT: u32 = 69;
pub const VIEW_PRIORITY_USEASDEFAULT: u32 = 67;
