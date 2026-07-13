windows_link::link!("shell32.dll" "system" fn GetCurrentProcessExplicitAppUserModelID(appid : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHAssocEnumHandlers(pszextra : windows_sys::core::PCWSTR, affilter : ASSOC_FILTER, ppenumhandler : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHAssocEnumHandlersForProtocolByApplication(protocol : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, enumhandlers : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHCreateAssociationRegistration(riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHCreateDefaultExtractIcon(riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SHCreateItemFromIDList(pidl : *const super::shtypes::ITEMIDLIST, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("shell32.dll" "system" fn SHCreateItemFromParsingName(pszpath : windows_sys::core::PCWSTR, pbc : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("shell32.dll" "system" fn SHCreateItemFromRelativeName(psiparent : *mut core::ffi::c_void, pszname : windows_sys::core::PCWSTR, pbc : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SHCreateItemInKnownFolder(kfid : *const super::shtypes::KNOWNFOLDERID, dwkfflags : u32, pszitem : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SHCreateItemWithParent(pidlparent : *const super::shtypes::ITEMIDLIST, psfparent : *mut core::ffi::c_void, pidl : *const super::shtypes::ITEMIDLIST, riid : *const windows_sys::core::GUID, ppvitem : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SHCreateShellItemArray(pidlparent : *const super::shtypes::ITEMIDLIST, psf : *mut core::ffi::c_void, cidl : u32, ppidl : *const super::shtypes::LPCITEMIDLIST, ppsiitemarray : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("shell32.dll" "system" fn SHCreateShellItemArrayFromDataObject(pdo : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SHCreateShellItemArrayFromIDLists(cidl : u32, rgpidl : *const super::shtypes::LPCITEMIDLIST, ppsiitemarray : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHCreateShellItemArrayFromShellItem(psi : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SHGetIDListFromObject(punk : *mut core::ffi::c_void, ppidl : *mut super::shtypes::LPITEMIDLIST) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("shell32.dll" "system" fn SHGetItemFromDataObject(pdtobj : *mut core::ffi::c_void, dwflags : DATAOBJ_GET_ITEM_FLAGS, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHGetItemFromObject(punk : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SHGetNameFromIDList(pidl : *const super::shtypes::ITEMIDLIST, sigdnname : SIGDN, ppszname : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "propsys", feature = "shtypes"))]
windows_link::link!("shell32.dll" "system" fn SHGetPropertyStoreFromIDList(pidl : *const super::shtypes::ITEMIDLIST, flags : super::propsys::GETPROPERTYSTOREFLAGS, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "propsys"))]
windows_link::link!("shell32.dll" "system" fn SHGetPropertyStoreFromParsingName(pszpath : windows_sys::core::PCWSTR, pbc : *mut core::ffi::c_void, flags : super::propsys::GETPROPERTYSTOREFLAGS, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("shell32.dll" "system" fn SHGetTemporaryPropertyForItem(psi : *mut core::ffi::c_void, propkey : *const super::wtypes::PROPERTYKEY, ppropvar : *mut super::propidlbase::PROPVARIANT) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHResolveLibrary(psilibrary : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("shell32.dll" "system" fn SHSetTemporaryPropertyForItem(psi : *mut core::ffi::c_void, propkey : *const super::wtypes::PROPERTYKEY, propvar : *const super::propidlbase::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("shell32.dll" "system" fn SHShowManageLibraryUI(psilibrary : *mut core::ffi::c_void, hwndowner : super::windef::HWND, psztitle : windows_sys::core::PCWSTR, pszinstruction : windows_sys::core::PCWSTR, lmdoptions : LIBRARYMANAGEDIALOGOPTIONS) -> windows_sys::core::HRESULT);
#[cfg(feature = "shtypes")]
windows_link::link!("shell32.dll" "system" fn SHSimpleIDListFromPath(pszpath : windows_sys::core::PCWSTR) -> super::shtypes::LPITEMIDLIST);
windows_link::link!("shell32.dll" "system" fn SetCurrentProcessExplicitAppUserModelID(appid : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
pub type ACTIVATEOPTIONS = u32;
pub const ADE_LEFT: ADJACENT_DISPLAY_EDGES = 1;
pub const ADE_NONE: ADJACENT_DISPLAY_EDGES = 0;
pub const ADE_RIGHT: ADJACENT_DISPLAY_EDGES = 2;
pub type ADJACENT_DISPLAY_EDGES = u32;
pub const ADLT_FREQUENT: APPDOCLISTTYPE = 1;
pub const ADLT_RECENT: APPDOCLISTTYPE = 0;
pub const AHE_DESKTOP: AHE_TYPE = 0;
pub const AHE_IMMERSIVE: AHE_TYPE = 1;
pub type AHE_TYPE = i32;
pub type AHTYPE = u32;
pub const AHTYPE_ANY_APPLICATION: AHTYPE = 16;
pub const AHTYPE_ANY_PROGID: AHTYPE = 512;
pub const AHTYPE_APPLICATION: AHTYPE = 128;
pub const AHTYPE_CLASS_APPLICATION: AHTYPE = 256;
pub const AHTYPE_MACHINEDEFAULT: AHTYPE = 32;
pub const AHTYPE_PROGID: AHTYPE = 64;
pub const AHTYPE_UNDEFINED: AHTYPE = 0;
pub const AHTYPE_USER_APPLICATION: AHTYPE = 8;
pub const AL_EFFECTIVE: ASSOCIATIONLEVEL = 1;
pub const AL_MACHINE: ASSOCIATIONLEVEL = 0;
pub const AL_USER: ASSOCIATIONLEVEL = 2;
pub const ANCESTORDEFAULT: u32 = 4294967295;
pub const AO_DESIGNMODE: ACTIVATEOPTIONS = 1;
pub const AO_NOERRORUI: ACTIVATEOPTIONS = 2;
pub const AO_NONE: ACTIVATEOPTIONS = 0;
pub const AO_NOSPLASHSCREEN: ACTIVATEOPTIONS = 4;
pub const AO_PRELAUNCH: ACTIVATEOPTIONS = 33554432;
pub type APPDOCLISTTYPE = i32;
pub type APPLICATION_VIEW_MIN_WIDTH = i32;
pub type APPLICATION_VIEW_ORIENTATION = i32;
pub type APPLICATION_VIEW_SIZE_PREFERENCE = i32;
pub type APPLICATION_VIEW_STATE = i32;
pub const ARCONTENT_AUDIOCD: u32 = 4;
pub const ARCONTENT_AUTOPLAYMUSIC: u32 = 256;
pub const ARCONTENT_AUTOPLAYPIX: u32 = 128;
pub const ARCONTENT_AUTOPLAYVIDEO: u32 = 512;
pub const ARCONTENT_AUTORUNINF: u32 = 2;
pub const ARCONTENT_BLANKBD: u32 = 8192;
pub const ARCONTENT_BLANKCD: u32 = 16;
pub const ARCONTENT_BLANKDVD: u32 = 32;
pub const ARCONTENT_BLURAY: u32 = 16384;
pub const ARCONTENT_CAMERASTORAGE: u32 = 32768;
pub const ARCONTENT_CUSTOMEVENT: u32 = 65536;
pub const ARCONTENT_DVDAUDIO: u32 = 4096;
pub const ARCONTENT_DVDMOVIE: u32 = 8;
pub const ARCONTENT_MASK: u32 = 131070;
pub const ARCONTENT_NONE: u32 = 0;
pub const ARCONTENT_PHASE_FINAL: u32 = 1073741824;
pub const ARCONTENT_PHASE_MASK: u32 = 1879048192;
pub const ARCONTENT_PHASE_PRESNIFF: u32 = 268435456;
pub const ARCONTENT_PHASE_SNIFFING: u32 = 536870912;
pub const ARCONTENT_PHASE_UNKNOWN: u32 = 0;
pub const ARCONTENT_SVCD: u32 = 2048;
pub const ARCONTENT_UNKNOWNCONTENT: u32 = 64;
pub const ARCONTENT_VCD: u32 = 1024;
pub type ASSOCIATIONLEVEL = i32;
pub type ASSOCIATIONTYPE = i32;
pub type ASSOC_FILTER = u32;
pub const ASSOC_FILTER_NONE: ASSOC_FILTER = 0;
pub const ASSOC_FILTER_RECOMMENDED: ASSOC_FILTER = 1;
pub type ATTACHMENT_ACTION = i32;
pub const ATTACHMENT_ACTION_CANCEL: ATTACHMENT_ACTION = 0;
pub const ATTACHMENT_ACTION_EXEC: ATTACHMENT_ACTION = 2;
pub const ATTACHMENT_ACTION_SAVE: ATTACHMENT_ACTION = 1;
pub type ATTACHMENT_PROMPT = i32;
pub const ATTACHMENT_PROMPT_EXEC: ATTACHMENT_PROMPT = 2;
pub const ATTACHMENT_PROMPT_EXEC_OR_SAVE: ATTACHMENT_PROMPT = 3;
pub const ATTACHMENT_PROMPT_NONE: ATTACHMENT_PROMPT = 0;
pub const ATTACHMENT_PROMPT_SAVE: ATTACHMENT_PROMPT = 1;
pub const AT_FILEEXTENSION: ASSOCIATIONTYPE = 0;
pub const AT_MIMETYPE: ASSOCIATIONTYPE = 3;
pub const AT_STARTMENUCLIENT: ASSOCIATIONTYPE = 2;
pub const AT_URLPROTOCOL: ASSOCIATIONTYPE = 1;
pub const AVMW_320: APPLICATION_VIEW_MIN_WIDTH = 1;
pub const AVMW_500: APPLICATION_VIEW_MIN_WIDTH = 2;
pub const AVMW_DEFAULT: APPLICATION_VIEW_MIN_WIDTH = 0;
pub const AVO_LANDSCAPE: APPLICATION_VIEW_ORIENTATION = 0;
pub const AVO_PORTRAIT: APPLICATION_VIEW_ORIENTATION = 1;
pub const AVSP_CUSTOM: APPLICATION_VIEW_SIZE_PREFERENCE = 6;
pub const AVSP_DEFAULT: APPLICATION_VIEW_SIZE_PREFERENCE = 0;
pub const AVSP_USE_HALF: APPLICATION_VIEW_SIZE_PREFERENCE = 2;
pub const AVSP_USE_LESS: APPLICATION_VIEW_SIZE_PREFERENCE = 1;
pub const AVSP_USE_MINIMUM: APPLICATION_VIEW_SIZE_PREFERENCE = 4;
pub const AVSP_USE_MORE: APPLICATION_VIEW_SIZE_PREFERENCE = 3;
pub const AVSP_USE_NONE: APPLICATION_VIEW_SIZE_PREFERENCE = 5;
pub const AVS_FILLED: APPLICATION_VIEW_STATE = 1;
pub const AVS_FULLSCREEN_LANDSCAPE: APPLICATION_VIEW_STATE = 0;
pub const AVS_FULLSCREEN_PORTRAIT: APPLICATION_VIEW_STATE = 3;
pub const AVS_SNAPPED: APPLICATION_VIEW_STATE = 2;
pub const AppShellVerbHandler: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4ed3a719_cea8_4bd9_910d_e252f997afc2);
pub const AppStartupLink: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x273eb5e7_88b0_4843_bfef_e2c81d43aae5);
pub const AppVisibility: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7e5fe3d9_985f_4908_91f9_ee19f9fd1514);
pub const ApplicationActivationManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x45ba127d_10a8_46ea_8ab7_56ea9078943c);
pub const ApplicationAssociationRegistration: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x591209c7_767b_42b2_9fba_44ee4615f2c7);
pub const ApplicationDesignModeSettings: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x958a6fb5_dcb2_4faf_aafd_7fb054ad1a3b);
pub const ApplicationDestinations: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x86c14003_4d6b_4ef3_a7b4_0506663b2e68);
pub const ApplicationDocumentLists: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x86bec222_30f2_47e0_9f25_60d11cd75c28);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BANDSITEINFO {
    pub dwMask: u32,
    pub dwState: u32,
    pub dwStyle: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BANNER_NOTIFICATION {
    pub event: BANNER_NOTIFICATION_EVENT,
    pub providerIdentity: windows_sys::core::PCWSTR,
    pub contentId: windows_sys::core::PCWSTR,
}
impl Default for BANNER_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BANNER_NOTIFICATION_EVENT = i32;
pub const BFO_ADD_IE_TOCAPTIONBAR: BROWSERFRAMEOPTIONS = 512;
pub const BFO_BOTH_OPTIONS: BROWSERFRAMEOPTIONS = 4;
pub const BFO_BROWSER_PERSIST_SETTINGS: BROWSERFRAMEOPTIONS = 1;
pub const BFO_BROWSE_NO_IN_NEW_PROCESS: BROWSERFRAMEOPTIONS = 16;
pub const BFO_ENABLE_HYPERLINK_TRACKING: BROWSERFRAMEOPTIONS = 32;
pub const BFO_GO_HOME_PAGE: BROWSERFRAMEOPTIONS = 16384;
pub const BFO_NONE: BROWSERFRAMEOPTIONS = 0;
pub const BFO_NO_PARENT_FOLDER_SUPPORT: BROWSERFRAMEOPTIONS = 4096;
pub const BFO_NO_REOPEN_NEXT_RESTART: BROWSERFRAMEOPTIONS = 8192;
pub const BFO_PREFER_IEPROCESS: BROWSERFRAMEOPTIONS = 32768;
pub const BFO_QUERY_ALL: BROWSERFRAMEOPTIONS = 4294967295;
pub const BFO_RENAME_FOLDER_OPTIONS_TOINTERNET: BROWSERFRAMEOPTIONS = 2;
pub const BFO_SHOW_NAVIGATION_CANCELLED: BROWSERFRAMEOPTIONS = 65536;
pub const BFO_SUBSTITUE_INTERNET_START_PAGE: BROWSERFRAMEOPTIONS = 128;
pub const BFO_USE_DIALUP_REF: BROWSERFRAMEOPTIONS = 1024;
pub const BFO_USE_IE_LOGOBANDING: BROWSERFRAMEOPTIONS = 256;
pub const BFO_USE_IE_OFFLINE_SUPPORT: BROWSERFRAMEOPTIONS = 64;
pub const BFO_USE_IE_STATUSBAR: BROWSERFRAMEOPTIONS = 131072;
pub const BFO_USE_IE_TOOLBAR: BROWSERFRAMEOPTIONS = 2048;
pub const BIF_PREFER_INTERNET_SHORTCUT: BROWSERFRAMEOPTIONS = 8;
pub const BIND_INTERRUPTABLE: u32 = 4294967295;
pub const BNE_Button1Clicked: BANNER_NOTIFICATION_EVENT = 4;
pub const BNE_Button2Clicked: BANNER_NOTIFICATION_EVENT = 5;
pub const BNE_Closed: BANNER_NOTIFICATION_EVENT = 2;
pub const BNE_Dismissed: BANNER_NOTIFICATION_EVENT = 3;
pub const BNE_Hovered: BANNER_NOTIFICATION_EVENT = 1;
pub const BNE_Rendered: BANNER_NOTIFICATION_EVENT = 0;
pub type BROWSERFRAMEOPTIONS = u32;
pub const BSID_BANDADDED: tagBANDSITECID = 0;
pub const BSID_BANDREMOVED: tagBANDSITECID = 1;
pub const BSIM_STATE: u32 = 1;
pub const BSIM_STYLE: u32 = 2;
pub const BSIS_ALWAYSGRIPPER: u32 = 2;
pub const BSIS_AUTOGRIPPER: u32 = 0;
pub const BSIS_FIXEDORDER: u32 = 1024;
pub const BSIS_LEFTALIGN: u32 = 4;
pub const BSIS_LOCKED: u32 = 256;
pub const BSIS_NOCAPTION: u32 = 64;
pub const BSIS_NOCONTEXTMENU: u32 = 16;
pub const BSIS_NODROPTARGET: u32 = 32;
pub const BSIS_NOGRIPPER: u32 = 1;
pub const BSIS_PREFERNOLINEBREAK: u32 = 128;
pub const BSIS_PRESERVEORDERDURINGLAYOUT: u32 = 512;
pub const BSIS_SINGLECLICK: u32 = 8;
pub const BSSF_NOTITLE: u32 = 2;
pub const BSSF_UNDELETEABLE: u32 = 4096;
pub const BSSF_VISIBLE: u32 = 1;
pub type CATEGORYINFO_FLAGS = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CATEGORY_INFO {
    pub cif: CATEGORYINFO_FLAGS,
    pub wszName: [u16; 260],
}
impl Default for CATEGORY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CATINFO_COLLAPSED: CATEGORYINFO_FLAGS = 1;
pub const CATINFO_EXPANDED: CATEGORYINFO_FLAGS = 4;
pub const CATINFO_HIDDEN: CATEGORYINFO_FLAGS = 2;
pub const CATINFO_NOHEADER: CATEGORYINFO_FLAGS = 8;
pub const CATINFO_NOHEADERCOUNT: CATEGORYINFO_FLAGS = 32;
pub const CATINFO_NORMAL: CATEGORYINFO_FLAGS = 0;
pub const CATINFO_NOTCOLLAPSIBLE: CATEGORYINFO_FLAGS = 16;
pub const CATINFO_SEPARATE_IMAGES: CATEGORYINFO_FLAGS = 128;
pub const CATINFO_SHOWEMPTY: CATEGORYINFO_FLAGS = 256;
pub const CATINFO_SUBSETTED: CATEGORYINFO_FLAGS = 64;
pub const CATSORT_DEFAULT: CATSORT_FLAGS = 0;
pub type CATSORT_FLAGS = u32;
pub const CATSORT_NAME: CATSORT_FLAGS = 1;
pub const CDB2GVF_ADDSHIELD: u32 = 64;
pub const CDB2GVF_ALLOWPREVIEWPANE: u32 = 4;
pub const CDB2GVF_ISFILESAVE: u32 = 2;
pub const CDB2GVF_ISFOLDERPICKER: u32 = 32;
pub const CDB2GVF_NOINCLUDEITEM: u32 = 16;
pub const CDB2GVF_NOSELECTVERB: u32 = 8;
pub const CDB2GVF_SHOWALLFILES: u32 = 1;
pub const CDB2N_CONTEXTMENU_DONE: u32 = 1;
pub const CDB2N_CONTEXTMENU_START: u32 = 2;
pub const CDBOSC_KILLFOCUS: u32 = 1;
pub const CDBOSC_RENAME: u32 = 3;
pub const CDBOSC_SELCHANGE: u32 = 2;
pub const CDBOSC_SETFOCUS: u32 = 0;
pub const CDBOSC_STATECHANGE: u32 = 4;
pub type CDCONTROLSTATEF = u32;
pub const CDCS_ENABLED: CDCONTROLSTATEF = 1;
pub const CDCS_ENABLEDVISIBLE: CDCONTROLSTATEF = 3;
pub const CDCS_INACTIVE: CDCONTROLSTATEF = 0;
pub const CDCS_VISIBLE: CDCONTROLSTATEF = 2;
pub const CMDSTR_NEWFOLDERA: windows_sys::core::PCSTR = windows_sys::core::s!("NewFolder");
pub const CMDSTR_NEWFOLDERW: windows_sys::core::PCWSTR = windows_sys::core::w!("NewFolder");
pub const CMDSTR_VIEWDETAILSA: windows_sys::core::PCSTR = windows_sys::core::s!("ViewDetails");
pub const CMDSTR_VIEWDETAILSW: windows_sys::core::PCWSTR = windows_sys::core::w!("ViewDetails");
pub const CMDSTR_VIEWLISTA: windows_sys::core::PCSTR = windows_sys::core::s!("ViewList");
pub const CMDSTR_VIEWLISTW: windows_sys::core::PCWSTR = windows_sys::core::w!("ViewList");
pub const CMF_ASYNCVERBSTATE: u32 = 1024;
pub const CMF_CANRENAME: u32 = 16;
pub const CMF_DEFAULTONLY: u32 = 1;
pub const CMF_DISABLEDVERBS: u32 = 512;
pub const CMF_DONOTPICKDEFAULT: u32 = 8192;
pub const CMF_EXPLORE: u32 = 4;
pub const CMF_EXTENDEDVERBS: u32 = 256;
pub const CMF_ITEMMENU: u32 = 128;
pub const CMF_NODEFAULT: u32 = 32;
pub const CMF_NORMAL: u32 = 0;
pub const CMF_NOVERBS: u32 = 8;
pub const CMF_OPTIMIZEFORINVOKE: u32 = 2048;
pub const CMF_RESERVED: u32 = 4294901760;
pub const CMF_SYNCCASCADEMENU: u32 = 4096;
pub const CMF_VERBSONLY: u32 = 2;
pub const CMIC_MASK_ASYNCOK: u32 = 1048576;
pub const CMIC_MASK_CONTROL_DOWN: u32 = 1073741824;
pub const CMIC_MASK_FLAG_LOG_USAGE: u32 = 67108864;
pub const CMIC_MASK_FLAG_NO_UI: u32 = 1024;
pub const CMIC_MASK_HOTKEY: u32 = 32;
pub const CMIC_MASK_NOASYNC: u32 = 256;
pub const CMIC_MASK_NOZONECHECKS: u32 = 8388608;
pub const CMIC_MASK_NO_CONSOLE: u32 = 32768;
pub const CMIC_MASK_PTINVOKE: u32 = 536870912;
pub const CMIC_MASK_SHIFT_DOWN: u32 = 268435456;
pub const CMIC_MASK_UNICODE: u32 = 16384;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CMINVOKECOMMANDINFO {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::windef::HWND,
    pub lpVerb: windows_sys::core::PCSTR,
    pub lpParameters: windows_sys::core::PCSTR,
    pub lpDirectory: windows_sys::core::PCSTR,
    pub nShow: i32,
    pub dwHotKey: u32,
    pub hIcon: super::winnt::HANDLE,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for CMINVOKECOMMANDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CMINVOKECOMMANDINFOEX {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::windef::HWND,
    pub lpVerb: windows_sys::core::PCSTR,
    pub lpParameters: windows_sys::core::PCSTR,
    pub lpDirectory: windows_sys::core::PCSTR,
    pub nShow: i32,
    pub dwHotKey: u32,
    pub hIcon: super::winnt::HANDLE,
    pub lpTitle: windows_sys::core::PCSTR,
    pub lpVerbW: windows_sys::core::PCWSTR,
    pub lpParametersW: windows_sys::core::PCWSTR,
    pub lpDirectoryW: windows_sys::core::PCWSTR,
    pub lpTitleW: windows_sys::core::PCWSTR,
    pub ptInvoke: super::windef::POINT,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for CMINVOKECOMMANDINFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct CMINVOKECOMMANDINFOEX_REMOTE {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::windef::HWND,
    pub lpVerbString: windows_sys::core::PCSTR,
    pub lpParameters: windows_sys::core::PCSTR,
    pub lpDirectory: windows_sys::core::PCSTR,
    pub nShow: i32,
    pub dwHotKey: u32,
    pub lpTitle: windows_sys::core::PCSTR,
    pub lpVerbWString: windows_sys::core::PCWSTR,
    pub lpParametersW: windows_sys::core::PCWSTR,
    pub lpDirectoryW: windows_sys::core::PCWSTR,
    pub lpTitleW: windows_sys::core::PCWSTR,
    pub ptInvoke: super::windef::POINT,
    pub lpVerbInt: u32,
    pub lpVerbWInt: u32,
}
#[cfg(feature = "windef")]
impl Default for CMINVOKECOMMANDINFOEX_REMOTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_COLUMNINFO {
    pub cbSize: u32,
    pub dwMask: u32,
    pub dwState: u32,
    pub uWidth: u32,
    pub uDefaultWidth: u32,
    pub uIdealWidth: u32,
    pub wszName: [u16; 80],
}
impl Default for CM_COLUMNINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CM_ENUM_ALL: CM_ENUM_FLAGS = 1;
pub type CM_ENUM_FLAGS = u32;
pub const CM_ENUM_VISIBLE: CM_ENUM_FLAGS = 2;
pub type CM_MASK = u32;
pub const CM_MASK_DEFAULTWIDTH: CM_MASK = 2;
pub const CM_MASK_IDEALWIDTH: CM_MASK = 4;
pub const CM_MASK_NAME: CM_MASK = 8;
pub const CM_MASK_STATE: CM_MASK = 16;
pub const CM_MASK_WIDTH: CM_MASK = 1;
pub type CM_SET_WIDTH_VALUE = i32;
pub type CM_STATE = u32;
pub const CM_STATE_ALWAYSVISIBLE: CM_STATE = 8;
pub const CM_STATE_FIXEDWIDTH: CM_STATE = 2;
pub const CM_STATE_NONE: CM_STATE = 0;
pub const CM_STATE_NOSORTBYFOLDERNESS: CM_STATE = 4;
pub const CM_STATE_VISIBLE: CM_STATE = 1;
pub const CM_WIDTH_AUTOSIZE: CM_SET_WIDTH_VALUE = -2;
pub const CM_WIDTH_USEDEFAULT: CM_SET_WIDTH_VALUE = -1;
pub const CONFLICT_RESOLUTION_CLSID_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("ConflictResolutionCLSID");
pub type CPVIEW = i32;
pub const CPVIEW_ALLITEMS: CPVIEW = 0;
pub const CPVIEW_CATEGORY: CPVIEW = 1;
pub const CPVIEW_CLASSIC: CPVIEW = 0;
pub const CPVIEW_HOME: CPVIEW = 1;
pub const CSIDL_FLAG_PFTI_TRACKTARGET: u32 = 16384;
pub type DATAOBJ_GET_ITEM_FLAGS = u32;
pub const DBID_BANDINFOCHANGED: tagDESKBANDCID = 0;
pub const DBID_DELAYINIT: tagDESKBANDCID = 4;
pub const DBID_FINISHINIT: tagDESKBANDCID = 5;
pub const DBID_MAXIMIZEBAND: tagDESKBANDCID = 2;
pub const DBID_PERMITAUTOHIDE: tagDESKBANDCID = 7;
pub const DBID_PUSHCHEVRON: tagDESKBANDCID = 3;
pub const DBID_SETWINDOWTHEME: tagDESKBANDCID = 6;
pub const DBID_SHOWONLY: tagDESKBANDCID = 1;
pub const DBIF_VIEWMODE_FLOATING: u32 = 2;
pub const DBIF_VIEWMODE_NORMAL: u32 = 0;
pub const DBIF_VIEWMODE_TRANSPARENT: u32 = 4;
pub const DBIF_VIEWMODE_VERTICAL: u32 = 1;
pub const DBIMF_ADDTOFRONT: u32 = 512;
pub const DBIMF_ALWAYSGRIPPER: u32 = 4096;
pub const DBIMF_BKCOLOR: u32 = 64;
pub const DBIMF_BREAK: u32 = 256;
pub const DBIMF_DEBOSSED: u32 = 32;
pub const DBIMF_FIXED: u32 = 1;
pub const DBIMF_FIXEDBMP: u32 = 4;
pub const DBIMF_NOGRIPPER: u32 = 2048;
pub const DBIMF_NOMARGINS: u32 = 8192;
pub const DBIMF_NORMAL: u32 = 0;
pub const DBIMF_TOPALIGN: u32 = 1024;
pub const DBIMF_UNDELETEABLE: u32 = 16;
pub const DBIMF_USECHEVRON: u32 = 128;
pub const DBIMF_VARIABLEHEIGHT: u32 = 8;
pub const DBIM_ACTUAL: u32 = 8;
pub const DBIM_BKCOLOR: u32 = 64;
pub const DBIM_INTEGRAL: u32 = 4;
pub const DBIM_MAXSIZE: u32 = 2;
pub const DBIM_MINSIZE: u32 = 1;
pub const DBIM_MODEFLAGS: u32 = 32;
pub const DBIM_TITLE: u32 = 16;
pub const DBPC_SELECTFIRST: u32 = 4294967295;
pub const DBPC_SELECTLAST: u32 = 4294967294;
pub type DEFAULTSAVEFOLDERTYPE = i32;
pub type DEFAULT_FOLDER_MENU_RESTRICTIONS = u32;
pub const DEFSHAREID_PUBLIC: DEF_SHARE_ID = 2;
pub const DEFSHAREID_USERS: DEF_SHARE_ID = 1;
pub type DEF_SHARE_ID = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DELEGATEITEMID {
    pub cbSize: u16,
    pub wOuter: u16,
    pub cbInner: u16,
    pub rgb: [u8; 1],
}
impl Default for DELEGATEITEMID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DEPRECATED_HRESULT = windows_sys::core::HRESULT;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct DESKBANDINFO {
    pub dwMask: u32,
    pub ptMinSize: super::windef::POINTL,
    pub ptMaxSize: super::windef::POINTL,
    pub ptIntegral: super::windef::POINTL,
    pub ptActual: super::windef::POINTL,
    pub wszTitle: [u16; 256],
    pub dwModeFlags: u32,
    pub crBkgnd: super::windef::COLORREF,
}
#[cfg(feature = "windef")]
impl Default for DESKBANDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DESKTOP_SLIDESHOW_DIRECTION = i32;
pub type DESKTOP_SLIDESHOW_OPTIONS = u32;
pub type DESKTOP_SLIDESHOW_STATE = u32;
pub type DESKTOP_WALLPAPER_POSITION = i32;
pub const DFMR_DEFAULT: DEFAULT_FOLDER_MENU_RESTRICTIONS = 0;
pub const DFMR_NO_ASYNC_VERBS: DEFAULT_FOLDER_MENU_RESTRICTIONS = 1024;
pub const DFMR_NO_NATIVECPU_VERBS: DEFAULT_FOLDER_MENU_RESTRICTIONS = 2048;
pub const DFMR_NO_NONWOW_VERBS: DEFAULT_FOLDER_MENU_RESTRICTIONS = 4096;
pub const DFMR_NO_RESOURCE_VERBS: DEFAULT_FOLDER_MENU_RESTRICTIONS = 32;
pub const DFMR_NO_STATIC_VERBS: DEFAULT_FOLDER_MENU_RESTRICTIONS = 8;
pub const DFMR_OPTIN_HANDLERS_ONLY: DEFAULT_FOLDER_MENU_RESTRICTIONS = 64;
pub const DFMR_RESOURCE_AND_FOLDER_VERBS_ONLY: DEFAULT_FOLDER_MENU_RESTRICTIONS = 128;
pub const DFMR_STATIC_VERBS_ONLY: DEFAULT_FOLDER_MENU_RESTRICTIONS = 16;
pub const DFMR_USE_SPECIFIED_HANDLERS: DEFAULT_FOLDER_MENU_RESTRICTIONS = 256;
pub const DFMR_USE_SPECIFIED_VERBS: DEFAULT_FOLDER_MENU_RESTRICTIONS = 512;
pub const DOGIF_DEFAULT: DATAOBJ_GET_ITEM_FLAGS = 0;
pub const DOGIF_NO_HDROP: DATAOBJ_GET_ITEM_FLAGS = 2;
pub const DOGIF_NO_URL: DATAOBJ_GET_ITEM_FLAGS = 4;
pub const DOGIF_ONLY_IF_ONE: DATAOBJ_GET_ITEM_FLAGS = 8;
pub const DOGIF_TRAVERSE_LINK: DATAOBJ_GET_ITEM_FLAGS = 1;
pub const DSD_BACKWARD: DESKTOP_SLIDESHOW_DIRECTION = 1;
pub const DSD_FORWARD: DESKTOP_SLIDESHOW_DIRECTION = 0;
pub const DSFT_DETECT: DEFAULTSAVEFOLDERTYPE = 1;
pub const DSFT_PRIVATE: DEFAULTSAVEFOLDERTYPE = 2;
pub const DSFT_PUBLIC: DEFAULTSAVEFOLDERTYPE = 3;
pub const DSO_SHUFFLEIMAGES: DESKTOP_SLIDESHOW_OPTIONS = 1;
pub const DSS_DISABLED_BY_REMOTE_SESSION: DESKTOP_SLIDESHOW_STATE = 4;
pub const DSS_ENABLED: DESKTOP_SLIDESHOW_STATE = 1;
pub const DSS_SLIDESHOW: DESKTOP_SLIDESHOW_STATE = 2;
pub const DWPOS_CENTER: DESKTOP_WALLPAPER_POSITION = 0;
pub const DWPOS_FILL: DESKTOP_WALLPAPER_POSITION = 4;
pub const DWPOS_FIT: DESKTOP_WALLPAPER_POSITION = 3;
pub const DWPOS_SPAN: DESKTOP_WALLPAPER_POSITION = 5;
pub const DWPOS_STRETCH: DESKTOP_WALLPAPER_POSITION = 2;
pub const DWPOS_TILE: DESKTOP_WALLPAPER_POSITION = 1;
pub const DefFolderMenu: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc63382be_7933_48d0_9ac8_85fb46be2fdd);
pub const DesktopWallpaper: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc2cf3110_460e_4fc1_b9d0_8a1c0c9cc4bd);
pub const DestinationList: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x77f10cf0_3db5_4966_b520_b7c54fd35ed6);
pub const DestinationListBoth: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x38fe0cf4_6a59_4729_8e4a_2d580059ede4);
pub const DriveSizeCategorizer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x94357b53_ca29_4b78_83ae_e8fe7409134f);
pub const DriveTypeCategorizer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb0a8f3cf_4333_4bab_8873_1ccb1cada48b);
pub const EBF_NODROPTARGET: EXPLORER_BROWSER_FILL_FLAGS = 512;
pub const EBF_NONE: EXPLORER_BROWSER_FILL_FLAGS = 0;
pub const EBF_SELECTFROMDATAOBJECT: EXPLORER_BROWSER_FILL_FLAGS = 256;
pub const EBO_ALWAYSNAVIGATE: EXPLORER_BROWSER_OPTIONS = 4;
pub const EBO_HTMLSHAREPOINTVIEW: EXPLORER_BROWSER_OPTIONS = 32;
pub const EBO_NAVIGATEONCE: EXPLORER_BROWSER_OPTIONS = 1;
pub const EBO_NOBORDER: EXPLORER_BROWSER_OPTIONS = 64;
pub const EBO_NONE: EXPLORER_BROWSER_OPTIONS = 0;
pub const EBO_NOPERSISTVIEWSTATE: EXPLORER_BROWSER_OPTIONS = 128;
pub const EBO_NOTRAVELLOG: EXPLORER_BROWSER_OPTIONS = 8;
pub const EBO_NOWRAPPERWINDOW: EXPLORER_BROWSER_OPTIONS = 16;
pub const EBO_SHOWFRAMES: EXPLORER_BROWSER_OPTIONS = 2;
pub const ECF_AUTOMENUICONS: EXPCMDFLAGS = 512;
pub const ECF_DEFAULT: EXPCMDFLAGS = 0;
pub const ECF_HASLUASHIELD: EXPCMDFLAGS = 16;
pub const ECF_HASSPLITBUTTON: EXPCMDFLAGS = 2;
pub const ECF_HASSUBCOMMANDS: EXPCMDFLAGS = 1;
pub const ECF_HIDELABEL: EXPCMDFLAGS = 4;
pub const ECF_ISDROPDOWN: EXPCMDFLAGS = 128;
pub const ECF_ISSEPARATOR: EXPCMDFLAGS = 8;
pub const ECF_SEPARATORAFTER: EXPCMDFLAGS = 64;
pub const ECF_SEPARATORBEFORE: EXPCMDFLAGS = 32;
pub const ECF_TOGGLEABLE: EXPCMDFLAGS = 256;
pub const ECHUIM_DESKTOP: EC_HOST_UI_MODE = 0;
pub const ECHUIM_IMMERSIVE: EC_HOST_UI_MODE = 1;
pub const ECHUIM_SYSTEM_LAUNCHER: EC_HOST_UI_MODE = 2;
pub const ECS_CHECKBOX: EXPCMDSTATE = 4;
pub const ECS_CHECKED: EXPCMDSTATE = 8;
pub const ECS_DISABLED: EXPCMDSTATE = 1;
pub const ECS_ENABLED: EXPCMDSTATE = 0;
pub const ECS_HIDDEN: EXPCMDSTATE = 2;
pub const ECS_RADIOCHECK: EXPCMDSTATE = 16;
pub type EC_HOST_UI_MODE = i32;
pub type EDGE_GESTURE_KIND = i32;
pub const EGK_KEYBOARD: EDGE_GESTURE_KIND = 1;
pub const EGK_MOUSE: EDGE_GESTURE_KIND = 2;
pub const EGK_TOUCH: EDGE_GESTURE_KIND = 0;
pub const EPS_DEFAULT_OFF: EXPLORERPANESTATE = 2;
pub const EPS_DEFAULT_ON: EXPLORERPANESTATE = 1;
pub const EPS_DONTCARE: EXPLORERPANESTATE = 0;
pub const EPS_FORCE: EXPLORERPANESTATE = 131072;
pub const EPS_INITIALSTATE: EXPLORERPANESTATE = 65536;
pub const EPS_STATEMASK: EXPLORERPANESTATE = 65535;
pub type EXPCMDFLAGS = u32;
pub type EXPCMDSTATE = u32;
pub type EXPLORERPANE = windows_sys::core::GUID;
pub type EXPLORERPANESTATE = u32;
pub type EXPLORER_BROWSER_FILL_FLAGS = u32;
pub type EXPLORER_BROWSER_OPTIONS = u32;
pub type EXPPS = u32;
pub const EXPPS_FILETYPES: EXPPS = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXTRASEARCH {
    pub guidSearch: windows_sys::core::GUID,
    pub wszFriendlyName: [u16; 80],
    pub wszUrl: [u16; 2084],
}
impl Default for EXTRASEARCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const E_PREVIEWHANDLER_CORRUPT: windows_sys::core::HRESULT = 0x86420004_u32 as _;
pub const E_PREVIEWHANDLER_DRM_FAIL: windows_sys::core::HRESULT = 0x86420001_u32 as _;
pub const E_PREVIEWHANDLER_NOAUTH: windows_sys::core::HRESULT = 0x86420002_u32 as _;
pub const E_PREVIEWHANDLER_NOTFOUND: windows_sys::core::HRESULT = 0x86420003_u32 as _;
pub const EnumerableObjectCollection: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2d3468c1_36a7_43b6_ac24_d3f02fd9607a);
pub const ExecuteUnknown: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe44e9428_bdbc_4987_a099_40dc8fd255e7);
pub const FCT_ADDTOEND: u32 = 4;
pub const FCT_CONFIGABLE: u32 = 2;
pub const FCT_MERGE: u32 = 1;
pub const FCW_INTERNETBAR: u32 = 6;
pub const FCW_PROGRESS: u32 = 8;
pub const FCW_STATUS: u32 = 1;
pub const FCW_TOOLBAR: u32 = 2;
pub const FCW_TREE: u32 = 3;
pub type FDAP = i32;
pub const FDAP_BOTTOM: FDAP = 0;
pub const FDAP_TOP: FDAP = 1;
pub const FDEOR_ACCEPT: FDE_OVERWRITE_RESPONSE = 1;
pub const FDEOR_DEFAULT: FDE_OVERWRITE_RESPONSE = 0;
pub const FDEOR_REFUSE: FDE_OVERWRITE_RESPONSE = 2;
pub const FDESVR_ACCEPT: FDE_SHAREVIOLATION_RESPONSE = 1;
pub const FDESVR_DEFAULT: FDE_SHAREVIOLATION_RESPONSE = 0;
pub const FDESVR_REFUSE: FDE_SHAREVIOLATION_RESPONSE = 2;
pub type FDE_OVERWRITE_RESPONSE = i32;
pub type FDE_SHAREVIOLATION_RESPONSE = i32;
pub const FEM_NAVIGATION: FOLDER_ENUM_MODE = 1;
pub const FEM_VIEWRESULT: FOLDER_ENUM_MODE = 0;
pub const FFFP_EXACTMATCH: FFFP_MODE = 0;
pub type FFFP_MODE = i32;
pub const FFFP_NEARESTPARENTMATCH: FFFP_MODE = 1;
pub type FILEOPENDIALOGOPTIONS = u32;
pub type FILE_OPERATION_FLAGS2 = u32;
pub type FILE_USAGE_TYPE = i32;
pub const FLVM_CONTENT: FOLDERLOGICALVIEWMODE = 5;
pub const FLVM_DETAILS: FOLDERLOGICALVIEWMODE = 1;
pub const FLVM_FIRST: FOLDERLOGICALVIEWMODE = 1;
pub const FLVM_ICONS: FOLDERLOGICALVIEWMODE = 3;
pub const FLVM_LAST: FOLDERLOGICALVIEWMODE = 5;
pub const FLVM_LIST: FOLDERLOGICALVIEWMODE = 4;
pub const FLVM_TILES: FOLDERLOGICALVIEWMODE = 2;
pub const FLVM_UNSPECIFIED: FOLDERLOGICALVIEWMODE = -1;
pub type FLYOUT_PLACEMENT = i32;
pub const FOF2_MERGEFOLDERSONCOLLISION: FILE_OPERATION_FLAGS2 = 1;
pub const FOF2_NONE: FILE_OPERATION_FLAGS2 = 0;
pub const FOFX_ADDUNDORECORD: u32 = 536870912;
pub const FOFX_COPYASDOWNLOAD: u32 = 1073741824;
pub const FOFX_DONTDISPLAYDESTPATH: u32 = 134217728;
pub const FOFX_DONTDISPLAYLOCATIONS: u32 = 2147483648;
pub const FOFX_DONTDISPLAYSOURCEPATH: u32 = 67108864;
pub const FOFX_EARLYFAILURE: u32 = 1048576;
pub const FOFX_KEEPNEWERFILE: u32 = 4194304;
pub const FOFX_MOVEACLSACROSSVOLUMES: u32 = 33554432;
pub const FOFX_NOCOPYHOOKS: u32 = 8388608;
pub const FOFX_NOMINIMIZEBOX: u32 = 16777216;
pub const FOFX_NOSKIPJUNCTIONS: u32 = 65536;
pub const FOFX_PREFERHARDLINK: u32 = 131072;
pub const FOFX_PRESERVEFILEEXTENSIONS: u32 = 2097152;
pub const FOFX_RECYCLEONDELETE: u32 = 524288;
pub const FOFX_REQUIREELEVATION: u32 = 268435456;
pub const FOFX_SHOWELEVATIONPROMPT: u32 = 262144;
pub type FOLDERFLAGS = u32;
pub type FOLDERLOGICALVIEWMODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FOLDERSETTINGS {
    pub ViewMode: u32,
    pub fFlags: u32,
}
pub type FOLDERVIEWMODE = i32;
pub type FOLDER_ENUM_MODE = i32;
pub const FOS_ALLNONSTORAGEITEMS: FILEOPENDIALOGOPTIONS = 128;
pub const FOS_ALLOWMULTISELECT: FILEOPENDIALOGOPTIONS = 512;
pub const FOS_CREATEPROMPT: FILEOPENDIALOGOPTIONS = 8192;
pub const FOS_DEFAULTNOMINIMODE: FILEOPENDIALOGOPTIONS = 536870912;
pub const FOS_DONTADDTORECENT: FILEOPENDIALOGOPTIONS = 33554432;
pub const FOS_FILEMUSTEXIST: FILEOPENDIALOGOPTIONS = 4096;
pub const FOS_FORCEFILESYSTEM: FILEOPENDIALOGOPTIONS = 64;
pub const FOS_FORCEPREVIEWPANEON: FILEOPENDIALOGOPTIONS = 1073741824;
pub const FOS_FORCESHOWHIDDEN: FILEOPENDIALOGOPTIONS = 268435456;
pub const FOS_HIDEMRUPLACES: FILEOPENDIALOGOPTIONS = 131072;
pub const FOS_HIDEPINNEDPLACES: FILEOPENDIALOGOPTIONS = 262144;
pub const FOS_NOCHANGEDIR: FILEOPENDIALOGOPTIONS = 8;
pub const FOS_NODEREFERENCELINKS: FILEOPENDIALOGOPTIONS = 1048576;
pub const FOS_NOREADONLYRETURN: FILEOPENDIALOGOPTIONS = 32768;
pub const FOS_NOTESTFILECREATE: FILEOPENDIALOGOPTIONS = 65536;
pub const FOS_NOVALIDATE: FILEOPENDIALOGOPTIONS = 256;
pub const FOS_OKBUTTONNEEDSINTERACTION: FILEOPENDIALOGOPTIONS = 2097152;
pub const FOS_OVERWRITEPROMPT: FILEOPENDIALOGOPTIONS = 2;
pub const FOS_PATHMUSTEXIST: FILEOPENDIALOGOPTIONS = 2048;
pub const FOS_PICKFOLDERS: FILEOPENDIALOGOPTIONS = 32;
pub const FOS_SHAREAWARE: FILEOPENDIALOGOPTIONS = 16384;
pub const FOS_STRICTFILETYPES: FILEOPENDIALOGOPTIONS = 4;
pub const FOS_SUPPORTSTREAMABLEITEMS: FILEOPENDIALOGOPTIONS = 2147483648;
pub const FP_ABOVE: FLYOUT_PLACEMENT = 1;
pub const FP_BELOW: FLYOUT_PLACEMENT = 2;
pub const FP_DEFAULT: FLYOUT_PLACEMENT = 0;
pub const FP_LEFT: FLYOUT_PLACEMENT = 3;
pub const FP_RIGHT: FLYOUT_PLACEMENT = 4;
pub const FUT_EDITING: FILE_USAGE_TYPE = 1;
pub const FUT_GENERIC: FILE_USAGE_TYPE = 2;
pub const FUT_PLAYING: FILE_USAGE_TYPE = 0;
pub const FVM_AUTO: FOLDERVIEWMODE = -1;
pub const FVM_CONTENT: FOLDERVIEWMODE = 8;
pub const FVM_DETAILS: FOLDERVIEWMODE = 4;
pub const FVM_FIRST: FOLDERVIEWMODE = 1;
pub const FVM_ICON: FOLDERVIEWMODE = 1;
pub const FVM_LAST: FOLDERVIEWMODE = 8;
pub const FVM_LIST: FOLDERVIEWMODE = 3;
pub const FVM_SMALLICON: FOLDERVIEWMODE = 2;
pub const FVM_THUMBNAIL: FOLDERVIEWMODE = 5;
pub const FVM_THUMBSTRIP: FOLDERVIEWMODE = 7;
pub const FVM_TILE: FOLDERVIEWMODE = 6;
pub const FVST_EMPTYTEXT: FVTEXTTYPE = 0;
pub type FVTEXTTYPE = i32;
pub const FWF_ABBREVIATEDNAMES: FOLDERFLAGS = 2;
pub const FWF_ALIGNLEFT: FOLDERFLAGS = 2048;
pub const FWF_ALLOWRTLREADING: FOLDERFLAGS = 2147483648;
pub const FWF_AUTOARRANGE: FOLDERFLAGS = 1;
pub const FWF_AUTOCHECKSELECT: FOLDERFLAGS = 134217728;
pub const FWF_BESTFITWINDOW: FOLDERFLAGS = 16;
pub const FWF_CHECKSELECT: FOLDERFLAGS = 262144;
pub const FWF_DESKTOP: FOLDERFLAGS = 32;
pub const FWF_EXTENDEDTILES: FOLDERFLAGS = 33554432;
pub const FWF_FULLROWSELECT: FOLDERFLAGS = 2097152;
pub const FWF_HIDEFILENAMES: FOLDERFLAGS = 131072;
pub const FWF_NOBROWSERVIEWSTATE: FOLDERFLAGS = 268435456;
pub const FWF_NOCLIENTEDGE: FOLDERFLAGS = 512;
pub const FWF_NOCOLUMNHEADER: FOLDERFLAGS = 8388608;
pub const FWF_NOENUMREFRESH: FOLDERFLAGS = 524288;
pub const FWF_NOFILTERS: FOLDERFLAGS = 4194304;
pub const FWF_NOGROUPING: FOLDERFLAGS = 1048576;
pub const FWF_NOHEADERINALLVIEWS: FOLDERFLAGS = 16777216;
pub const FWF_NOICONS: FOLDERFLAGS = 4096;
pub const FWF_NONE: FOLDERFLAGS = 0;
pub const FWF_NOSCROLL: FOLDERFLAGS = 1024;
pub const FWF_NOSUBFOLDERS: FOLDERFLAGS = 128;
pub const FWF_NOVISIBLE: FOLDERFLAGS = 16384;
pub const FWF_NOWEBVIEW: FOLDERFLAGS = 65536;
pub const FWF_OWNERDATA: FOLDERFLAGS = 8;
pub const FWF_SHOWSELALWAYS: FOLDERFLAGS = 8192;
pub const FWF_SINGLECLICKACTIVATE: FOLDERFLAGS = 32768;
pub const FWF_SINGLESEL: FOLDERFLAGS = 64;
pub const FWF_SNAPTOGRID: FOLDERFLAGS = 4;
pub const FWF_SUBSETGROUPS: FOLDERFLAGS = 536870912;
pub const FWF_TRANSPARENT: FOLDERFLAGS = 256;
pub const FWF_TRICHECKSELECT: FOLDERFLAGS = 67108864;
pub const FWF_USESEARCHFOLDER: FOLDERFLAGS = 1073741824;
pub const FileOpenDialog: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdc1c5a9c_e88a_4dde_a5a1_60f82a20aef7);
pub const FileOperation: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3ad05575_8857_4850_9277_11b85bdb8e09);
pub const FileSaveDialog: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc0b4e2f3_ba21_4773_8dba_335ec946eb8b);
pub const FrameworkInputPane: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd5120aa3_46ba_44c5_822d_ca8092c1fc72);
pub const FreeSpaceCategorizer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb5607793_24ac_44c7_82e2_831726aa6cb7);
pub const GCS_HELPTEXT: u32 = 1;
pub const GCS_HELPTEXTA: u32 = 1;
pub const GCS_HELPTEXTW: u32 = 5;
pub const GCS_UNICODE: u32 = 4;
pub const GCS_VALIDATE: u32 = 2;
pub const GCS_VALIDATEA: u32 = 2;
pub const GCS_VALIDATEW: u32 = 6;
pub const GCS_VERB: u32 = 0;
pub const GCS_VERBA: u32 = 0;
pub const GCS_VERBICONW: u32 = 20;
pub const GCS_VERBW: u32 = 4;
pub const HGSC_DOCUMENTSLIBRARY: HOMEGROUPSHARINGCHOICES = 8;
pub const HGSC_MUSICLIBRARY: HOMEGROUPSHARINGCHOICES = 1;
pub const HGSC_NONE: HOMEGROUPSHARINGCHOICES = 0;
pub const HGSC_PICTURESLIBRARY: HOMEGROUPSHARINGCHOICES = 2;
pub const HGSC_PRINTERS: HOMEGROUPSHARINGCHOICES = 16;
pub const HGSC_VIDEOSLIBRARY: HOMEGROUPSHARINGCHOICES = 4;
pub type HOMEGROUPSHARINGCHOICES = u32;
pub const HOMEGROUP_SECURITY_GROUP: windows_sys::core::PCWSTR = windows_sys::core::w!("HomeUsers");
pub const HOMEGROUP_SECURITY_GROUP_MULTI: windows_sys::core::PCWSTR = windows_sys::core::w!("HUG");
#[cfg(feature = "winnt")]
pub type HTHEME = super::winnt::HANDLE;
pub const HomeGroup: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xde77ba04_3c92_4d11_a1a5_42352a53e0e3);
pub const IEIFLAG_ASPECT: u32 = 4;
pub const IEIFLAG_ASYNC: u32 = 1;
pub const IEIFLAG_CACHE: u32 = 2;
pub const IEIFLAG_GLEAM: u32 = 16;
pub const IEIFLAG_NOBORDER: u32 = 256;
pub const IEIFLAG_NOSTAMP: u32 = 128;
pub const IEIFLAG_OFFLINE: u32 = 8;
pub const IEIFLAG_ORIGSIZE: u32 = 64;
pub const IEIFLAG_QUALITY: u32 = 512;
pub const IEIFLAG_REFRESH: u32 = 1024;
pub const IEIFLAG_SCREEN: u32 = 32;
pub const IEIT_PRIORITY_NORMAL: u32 = 268435456;
pub const IEI_PRIORITY_MAX: u32 = 2147483647;
pub const IEI_PRIORITY_MIN: u32 = 0;
pub const IRTIR_TASK_FINISHED: u32 = 4;
pub const IRTIR_TASK_NOT_RUNNING: u32 = 0;
pub const IRTIR_TASK_PENDING: u32 = 3;
pub const IRTIR_TASK_RUNNING: u32 = 1;
pub const IRTIR_TASK_SUSPENDED: u32 = 2;
pub const ISIOI_ICONFILE: u32 = 1;
pub const ISIOI_ICONINDEX: u32 = 2;
pub const ITSAT_DEFAULT_LPARAM: usize = 18446744073709551615u64 as usize;
pub const ITSAT_DEFAULT_PRIORITY: u32 = 268435456;
pub const ITSAT_MAX_PRIORITY: u32 = 2147483647;
pub const ITSAT_MIN_PRIORITY: u32 = 0;
pub const ITSSFLAG_COMPLETE_ON_DESTROY: u32 = 0;
pub const ITSSFLAG_FLAGS_MASK: u32 = 3;
pub const ITSSFLAG_KILL_ON_DESTROY: u32 = 1;
pub const ITSS_THREAD_DESTROY_DEFAULT_TIMEOUT: u32 = 10000;
pub const ITSS_THREAD_TERMINATE_TIMEOUT: i32 = -1;
pub const ITSS_THREAD_TIMEOUT_NO_CHANGE: i32 = -2;
pub const KDC_FREQUENT: KNOWNDESTCATEGORY = 1;
pub const KDC_RECENT: KNOWNDESTCATEGORY = 2;
pub const KFDF_LOCAL_REDIRECT_ONLY: KF_DEFINITION_FLAGS = 2;
pub const KFDF_NO_REDIRECT_UI: KF_DEFINITION_FLAGS = 64;
pub const KFDF_PRECREATE: KF_DEFINITION_FLAGS = 8;
pub const KFDF_PUBLISHEXPANDEDPATH: KF_DEFINITION_FLAGS = 32;
pub const KFDF_ROAMABLE: KF_DEFINITION_FLAGS = 4;
pub const KFDF_STREAM: KF_DEFINITION_FLAGS = 16;
pub type KF_CATEGORY = i32;
pub const KF_CATEGORY_COMMON: KF_CATEGORY = 3;
pub const KF_CATEGORY_FIXED: KF_CATEGORY = 2;
pub const KF_CATEGORY_PERUSER: KF_CATEGORY = 4;
pub const KF_CATEGORY_VIRTUAL: KF_CATEGORY = 1;
pub type KF_DEFINITION_FLAGS = u32;
pub type KF_REDIRECTION_CAPABILITIES = u32;
pub const KF_REDIRECTION_CAPABILITIES_ALLOW_ALL: KF_REDIRECTION_CAPABILITIES = 255;
pub const KF_REDIRECTION_CAPABILITIES_DENY_ALL: KF_REDIRECTION_CAPABILITIES = 1048320;
pub const KF_REDIRECTION_CAPABILITIES_DENY_PERMISSIONS: KF_REDIRECTION_CAPABILITIES = 1024;
pub const KF_REDIRECTION_CAPABILITIES_DENY_POLICY: KF_REDIRECTION_CAPABILITIES = 512;
pub const KF_REDIRECTION_CAPABILITIES_DENY_POLICY_REDIRECTED: KF_REDIRECTION_CAPABILITIES = 256;
pub const KF_REDIRECTION_CAPABILITIES_REDIRECTABLE: KF_REDIRECTION_CAPABILITIES = 1;
pub const KF_REDIRECT_CHECK_ONLY: KF_REDIRECT_FLAGS = 16;
pub const KF_REDIRECT_COPY_CONTENTS: KF_REDIRECT_FLAGS = 512;
pub const KF_REDIRECT_COPY_SOURCE_DACL: KF_REDIRECT_FLAGS = 2;
pub const KF_REDIRECT_DEL_SOURCE_CONTENTS: KF_REDIRECT_FLAGS = 1024;
pub const KF_REDIRECT_EXCLUDE_ALL_KNOWN_SUBFOLDERS: KF_REDIRECT_FLAGS = 2048;
pub type KF_REDIRECT_FLAGS = u32;
pub const KF_REDIRECT_OWNER_USER: KF_REDIRECT_FLAGS = 4;
pub const KF_REDIRECT_PIN: KF_REDIRECT_FLAGS = 128;
pub const KF_REDIRECT_SET_OWNER_EXPLICIT: KF_REDIRECT_FLAGS = 8;
pub const KF_REDIRECT_UNPIN: KF_REDIRECT_FLAGS = 64;
pub const KF_REDIRECT_USER_EXCLUSIVE: KF_REDIRECT_FLAGS = 1;
pub const KF_REDIRECT_WITH_UI: KF_REDIRECT_FLAGS = 32;
pub type KNOWNDESTCATEGORY = i32;
#[repr(C)]
#[cfg(feature = "shtypes")]
#[derive(Clone, Copy)]
pub struct KNOWNFOLDER_DEFINITION {
    pub category: KF_CATEGORY,
    pub pszName: windows_sys::core::PWSTR,
    pub pszDescription: windows_sys::core::PWSTR,
    pub fidParent: super::shtypes::KNOWNFOLDERID,
    pub pszRelativePath: windows_sys::core::PWSTR,
    pub pszParsingName: windows_sys::core::PWSTR,
    pub pszTooltip: windows_sys::core::PWSTR,
    pub pszLocalizedName: windows_sys::core::PWSTR,
    pub pszIcon: windows_sys::core::PWSTR,
    pub pszSecurity: windows_sys::core::PWSTR,
    pub dwAttributes: u32,
    pub kfdFlags: KF_DEFINITION_FLAGS,
    pub ftidType: super::shtypes::FOLDERTYPEID,
}
#[cfg(feature = "shtypes")]
impl Default for KNOWNFOLDER_DEFINITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KnownFolderManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4df0c730_df9d_4ae3_9153_aa6b82e9795a);
pub const LFF_ALLITEMS: LIBRARYFOLDERFILTER = 3;
pub const LFF_FORCEFILESYSTEM: LIBRARYFOLDERFILTER = 1;
pub const LFF_STORAGEITEMS: LIBRARYFOLDERFILTER = 2;
pub type LIBRARYFOLDERFILTER = i32;
pub type LIBRARYMANAGEDIALOGOPTIONS = u32;
pub type LIBRARYOPTIONFLAGS = u32;
pub type LIBRARYSAVEFLAGS = u32;
pub const LMD_ALLOWUNINDEXABLENETWORKLOCATIONS: LIBRARYMANAGEDIALOGOPTIONS = 1;
pub const LMD_DEFAULT: LIBRARYMANAGEDIALOGOPTIONS = 0;
pub const LOF_DEFAULT: LIBRARYOPTIONFLAGS = 0;
pub const LOF_MASK_ALL: LIBRARYOPTIONFLAGS = 1;
pub const LOF_PINNEDTONAVPANE: LIBRARYOPTIONFLAGS = 1;
pub type LPCFOLDERSETTINGS = *const FOLDERSETTINGS;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type LPCMINVOKECOMMANDINFO = *mut CMINVOKECOMMANDINFO;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type LPCMINVOKECOMMANDINFOEX = *mut CMINVOKECOMMANDINFOEX;
pub type LPEXTRASEARCH = *mut EXTRASEARCH;
#[cfg(all(feature = "minwindef", feature = "prsht"))]
pub type LPFNSVADDPROPSHEETPAGE = super::prsht::LPFNADDPROPSHEETPAGE;
pub type LPFOLDERSETTINGS = *mut FOLDERSETTINGS;
#[cfg(feature = "windef")]
pub type LPSHDRAGIMAGE = *mut SHDRAGIMAGE;
#[cfg(all(feature = "shtypes", feature = "windef"))]
pub type LPSMDATA = *mut SMDATA;
pub type LPSTGTRANSCONFIRMATION = *mut windows_sys::core::GUID;
#[cfg(all(feature = "oleidl", feature = "windef"))]
pub type LPSV2CVW2_PARAMS = *mut SV2CVW2_PARAMS;
#[cfg(feature = "commctrl")]
pub type LPTBBUTTONSB = super::commctrl::LPTBBUTTON;
#[cfg(feature = "windef")]
pub type LPTHUMBBUTTON = *mut THUMBBUTTON;
pub const LSF_FAILIFTHERE: LIBRARYSAVEFLAGS = 0;
pub const LSF_MAKEUNIQUENAME: LIBRARYSAVEFLAGS = 2;
pub const LSF_OVERRIDEEXISTING: LIBRARYSAVEFLAGS = 1;
pub const MAV_APP_VISIBLE: MONITOR_APP_VISIBILITY = 2;
pub const MAV_NO_APP_VISIBLE: MONITOR_APP_VISIBILITY = 1;
pub const MAV_UNKNOWN: MONITOR_APP_VISIBILITY = 0;
pub const MBHANDCID_PIDLSELECT: tagMENUBANDHANDLERCID = 0;
pub type MERGE_UPDATE_STATUS = i32;
pub type MONITOR_APP_VISIBILITY = i32;
pub const MPOS_CANCELLEVEL: tagMENUPOPUPSELECT = 2;
pub const MPOS_CHILDTRACKING: tagMENUPOPUPSELECT = 5;
pub const MPOS_EXECUTE: tagMENUPOPUPSELECT = 0;
pub const MPOS_FULLCANCEL: tagMENUPOPUPSELECT = 1;
pub const MPOS_SELECTLEFT: tagMENUPOPUPSELECT = 3;
pub const MPOS_SELECTRIGHT: tagMENUPOPUPSELECT = 4;
pub const MPPF_ALIGN_LEFT: tagMENUPOPUPPOPUPFLAGS = 33554432;
pub const MPPF_ALIGN_RIGHT: tagMENUPOPUPPOPUPFLAGS = 67108864;
pub const MPPF_BOTTOM: tagMENUPOPUPPOPUPFLAGS = -2147483648;
pub const MPPF_FINALSELECT: tagMENUPOPUPPOPUPFLAGS = 128;
pub const MPPF_FORCEZORDER: tagMENUPOPUPPOPUPFLAGS = 64;
pub const MPPF_INITIALSELECT: tagMENUPOPUPPOPUPFLAGS = 2;
pub const MPPF_KEYBOARD: tagMENUPOPUPPOPUPFLAGS = 16;
pub const MPPF_LEFT: tagMENUPOPUPPOPUPFLAGS = 1073741824;
pub const MPPF_NOANIMATE: tagMENUPOPUPPOPUPFLAGS = 4;
pub const MPPF_POS_MASK: tagMENUPOPUPPOPUPFLAGS = -536870912;
pub const MPPF_REPOSITION: tagMENUPOPUPPOPUPFLAGS = 32;
pub const MPPF_RIGHT: tagMENUPOPUPPOPUPFLAGS = 1610612736;
pub const MPPF_SETFOCUS: tagMENUPOPUPPOPUPFLAGS = 1;
pub const MPPF_TOP: tagMENUPOPUPPOPUPFLAGS = 536870912;
pub type MP_POPUPFLAGS = i32;
pub const MUS_COMPLETE: MERGE_UPDATE_STATUS = 0;
pub const MUS_FAILED: MERGE_UPDATE_STATUS = 2;
pub const MUS_USERINPUTNEEDED: MERGE_UPDATE_STATUS = 1;
pub const MailRecipient: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9e56be60_c50f_11cf_9a2c_00a0c90a90ce);
pub type NAMESPACEWALKFLAG = u32;
pub type NATIVE_DISPLAY_ORIENTATION = i32;
pub const NDO_LANDSCAPE: NATIVE_DISPLAY_ORIENTATION = 0;
pub const NDO_PORTRAIT: NATIVE_DISPLAY_ORIENTATION = 1;
pub type NMCII_FLAGS = i32;
pub const NMCII_FOLDERS: NMCII_FLAGS = 2;
pub const NMCII_ITEMS: NMCII_FLAGS = 1;
pub const NMCII_NONE: NMCII_FLAGS = 0;
pub const NMCSAEI_EDIT: NMCSAEI_FLAGS = 1;
pub type NMCSAEI_FLAGS = i32;
pub const NMCSAEI_SELECT: NMCSAEI_FLAGS = 0;
pub const NSTCFC_DELAY_REGISTER_NOTIFY: NSTCFOLDERCAPABILITIES = 2;
pub const NSTCFC_NONE: NSTCFOLDERCAPABILITIES = 0;
pub const NSTCFC_PINNEDITEMFILTERING: NSTCFOLDERCAPABILITIES = 1;
pub type NSTCFOLDERCAPABILITIES = u32;
pub type NSTCGNI = i32;
pub const NSTCGNI_CHILD: NSTCGNI = 5;
pub const NSTCGNI_FIRSTVISIBLE: NSTCGNI = 6;
pub const NSTCGNI_LASTVISIBLE: NSTCGNI = 7;
pub const NSTCGNI_NEXT: NSTCGNI = 0;
pub const NSTCGNI_NEXTVISIBLE: NSTCGNI = 1;
pub const NSTCGNI_PARENT: NSTCGNI = 4;
pub const NSTCGNI_PREV: NSTCGNI = 2;
pub const NSTCGNI_PREVVISIBLE: NSTCGNI = 3;
pub const NSTCIS_BOLD: NSTCITEMSTATE = 4;
pub const NSTCIS_DISABLED: NSTCITEMSTATE = 8;
pub const NSTCIS_EXPANDED: NSTCITEMSTATE = 2;
pub const NSTCIS_NONE: NSTCITEMSTATE = 0;
pub const NSTCIS_SELECTED: NSTCITEMSTATE = 1;
pub const NSTCIS_SELECTEDNOEXPAND: NSTCITEMSTATE = 16;
pub type NSTCITEMSTATE = u32;
pub type NSTCROOTSTYLE = u32;
pub const NSTCRS_EXPANDED: NSTCROOTSTYLE = 2;
pub const NSTCRS_HIDDEN: NSTCROOTSTYLE = 1;
pub const NSTCRS_VISIBLE: NSTCROOTSTYLE = 0;
pub type NSTCSTYLE = u32;
pub const NSTCS_ALLOWJUNCTIONS: NSTCSTYLE = 268435456;
pub const NSTCS_AUTOHSCROLL: NSTCSTYLE = 1048576;
pub const NSTCS_BORDER: NSTCSTYLE = 32768;
pub const NSTCS_CHECKBOXES: NSTCSTYLE = 8388608;
pub const NSTCS_DIMMEDCHECKBOXES: NSTCSTYLE = 67108864;
pub const NSTCS_DISABLEDRAGDROP: NSTCSTYLE = 4096;
pub const NSTCS_EMPTYTEXT: NSTCSTYLE = 4194304;
pub const NSTCS_EVENHEIGHT: NSTCSTYLE = 1024;
pub const NSTCS_EXCLUSIONCHECKBOXES: NSTCSTYLE = 33554432;
pub const NSTCS_FADEINOUTEXPANDOS: NSTCSTYLE = 2097152;
pub const NSTCS_FAVORITESMODE: NSTCSTYLE = 524288;
pub const NSTCS_FULLROWSELECT: NSTCSTYLE = 8;
pub const NSTCS_HASEXPANDOS: NSTCSTYLE = 1;
pub const NSTCS_HASLINES: NSTCSTYLE = 2;
pub const NSTCS_HORIZONTALSCROLL: NSTCSTYLE = 32;
pub const NSTCS_NOEDITLABELS: NSTCSTYLE = 65536;
pub const NSTCS_NOINDENTCHECKS: NSTCSTYLE = 134217728;
pub const NSTCS_NOINFOTIP: NSTCSTYLE = 512;
pub const NSTCS_NOORDERSTREAM: NSTCSTYLE = 8192;
pub const NSTCS_NOREPLACEOPEN: NSTCSTYLE = 2048;
pub const NSTCS_PARTIALCHECKBOXES: NSTCSTYLE = 16777216;
pub const NSTCS_RICHTOOLTIP: NSTCSTYLE = 16384;
pub const NSTCS_ROOTHASEXPANDO: NSTCSTYLE = 64;
pub const NSTCS_SHOWDELETEBUTTON: NSTCSTYLE = 1073741824;
pub const NSTCS_SHOWREFRESHBUTTON: NSTCSTYLE = 2147483648;
pub const NSTCS_SHOWSELECTIONALWAYS: NSTCSTYLE = 128;
pub const NSTCS_SHOWTABSBUTTON: NSTCSTYLE = 536870912;
pub const NSTCS_SINGLECLICKEXPAND: NSTCSTYLE = 4;
pub const NSTCS_SPRINGEXPAND: NSTCSTYLE = 16;
pub const NSTCS_TABSTOP: NSTCSTYLE = 131072;
pub const NSWF_ACCUMULATE_FOLDERS: NAMESPACEWALKFLAG = 2048;
pub const NSWF_ANY_IMPLIES_ALL: NAMESPACEWALKFLAG = 32768;
pub const NSWF_ASYNC: NAMESPACEWALKFLAG = 512;
pub const NSWF_DEFAULT: NAMESPACEWALKFLAG = 0;
pub const NSWF_DONT_ACCUMULATE_RESULT: NAMESPACEWALKFLAG = 8;
pub const NSWF_DONT_RESOLVE_LINKS: NAMESPACEWALKFLAG = 1024;
pub const NSWF_DONT_SORT: NAMESPACEWALKFLAG = 4096;
pub const NSWF_DONT_TRAVERSE_LINKS: NAMESPACEWALKFLAG = 4;
pub const NSWF_DONT_TRAVERSE_STREAM_JUNCTIONS: NAMESPACEWALKFLAG = 16384;
pub const NSWF_FILESYSTEM_ONLY: NAMESPACEWALKFLAG = 32;
pub const NSWF_FLAG_VIEWORDER: NAMESPACEWALKFLAG = 128;
pub const NSWF_IGNORE_AUTOPLAY_HIDA: NAMESPACEWALKFLAG = 256;
pub const NSWF_NONE_IMPLIES_ALL: NAMESPACEWALKFLAG = 1;
pub const NSWF_ONE_IMPLIES_ALL: NAMESPACEWALKFLAG = 2;
pub const NSWF_SHOW_PROGRESS: NAMESPACEWALKFLAG = 64;
pub const NSWF_TRAVERSE_STREAM_JUNCTIONS: NAMESPACEWALKFLAG = 16;
pub const NSWF_USE_TRANSFER_MEDIUM: NAMESPACEWALKFLAG = 8192;
pub type NWMF = u32;
pub const NWMF_FIRST: NWMF = 4;
pub const NWMF_FORCETAB: NWMF = 131072;
pub const NWMF_FORCEWINDOW: NWMF = 65536;
pub const NWMF_FROMDIALOGCHILD: NWMF = 64;
pub const NWMF_HTMLDIALOG: NWMF = 32;
pub const NWMF_INACTIVETAB: NWMF = 1048576;
pub const NWMF_OVERRIDEKEY: NWMF = 8;
pub const NWMF_SHOWHELP: NWMF = 16;
pub const NWMF_SUGGESTTAB: NWMF = 524288;
pub const NWMF_SUGGESTWINDOW: NWMF = 262144;
pub const NWMF_UNLOADING: NWMF = 1;
pub const NWMF_USERALLOWED: NWMF = 256;
pub const NWMF_USERINITED: NWMF = 2;
pub const NWMF_USERREQUESTED: NWMF = 128;
pub const NamespaceWalker: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x72eb61e0_8672_4303_9175_f2e4c68b2e7c);
pub const NetworkConnections: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7007acc7_3202_11d1_aad2_00805fc1270e);
pub const NetworkExplorerFolder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf02c1a0d_be21_4350_88b0_7367fc96ef3c);
pub const NetworkPlaces: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x208d2c60_3aea_1069_a2d7_08002b30309d);
pub const OF_CAP_CANCLOSE: u32 = 2;
pub const OF_CAP_CANSWITCHTO: u32 = 1;
pub type OPPROGDLGF = u32;
pub const OPPROGDLG_ALLOWUNDO: OPPROGDLGF = 256;
pub const OPPROGDLG_DEFAULT: OPPROGDLGF = 0;
pub const OPPROGDLG_DONTDISPLAYDESTPATH: OPPROGDLGF = 1024;
pub const OPPROGDLG_DONTDISPLAYLOCATIONS: OPPROGDLGF = 4096;
pub const OPPROGDLG_DONTDISPLAYSOURCEPATH: OPPROGDLGF = 512;
pub const OPPROGDLG_ENABLEPAUSE: OPPROGDLGF = 128;
pub const OPPROGDLG_NOMULTIDAYESTIMATES: OPPROGDLGF = 2048;
pub const OpenControlPanel: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x06622d85_6856_4460_8de1_a81921b41c4b);
pub type PACKAGE_EXECUTION_STATE = i32;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PCCMINVOKECOMMANDINFO = *const CMINVOKECOMMANDINFO;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PCCMINVOKECOMMANDINFOEX = *const CMINVOKECOMMANDINFOEX;
pub type PCDELEGATEITEMID = *const DELEGATEITEMID;
pub type PDELEGATEITEMID = *mut DELEGATEITEMID;
pub type PDMODE = u32;
pub const PDM_DEFAULT: PDMODE = 0;
pub const PDM_ERRORSBLOCKING: PDMODE = 8;
pub const PDM_INDETERMINATE: PDMODE = 16;
pub const PDM_PREFLIGHT: PDMODE = 2;
pub const PDM_RUN: PDMODE = 1;
pub const PDM_UNDOING: PDMODE = 4;
pub type PDOPSTATUS = i32;
pub const PDOPS_CANCELLED: PDOPSTATUS = 3;
pub const PDOPS_ERRORS: PDOPSTATUS = 5;
pub const PDOPS_PAUSED: PDOPSTATUS = 2;
pub const PDOPS_RUNNING: PDOPSTATUS = 1;
pub const PDOPS_STOPPED: PDOPSTATUS = 4;
#[repr(C)]
#[cfg(feature = "shtypes")]
#[derive(Clone, Copy)]
pub struct PERSIST_FOLDER_TARGET_INFO {
    pub pidlTargetFolder: super::shtypes::LPITEMIDLIST,
    pub szTargetParsingName: [u16; 260],
    pub szNetworkProvider: [u16; 260],
    pub dwAttributes: u32,
    pub csidl: i32,
}
#[cfg(feature = "shtypes")]
impl Default for PERSIST_FOLDER_TARGET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PES_RUNNING: PACKAGE_EXECUTION_STATE = 1;
pub const PES_SUSPENDED: PACKAGE_EXECUTION_STATE = 3;
pub const PES_SUSPENDING: PACKAGE_EXECUTION_STATE = 2;
pub const PES_TERMINATED: PACKAGE_EXECUTION_STATE = 4;
pub const PES_UNKNOWN: PACKAGE_EXECUTION_STATE = 0;
pub type PFOLDERSETTINGS = *mut FOLDERSETTINGS;
pub type PLACEHOLDER_STATES = u32;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct PREVIEWHANDLERFRAMEINFO {
    pub haccel: super::windef::HACCEL,
    pub cAccelEntries: u32,
}
#[cfg(feature = "windef")]
impl Default for PREVIEWHANDLERFRAMEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PROPERTYUI_FLAGS = u32;
pub type PROPERTYUI_FORMAT_FLAGS = u32;
pub type PROPERTYUI_NAME_FLAGS = u32;
pub const PROP_CONTRACT_DELEGATE: windows_sys::core::PCWSTR = windows_sys::core::w!("ContractDelegate");
#[cfg(feature = "shtypes")]
pub type PSMCSHCHANGENOTIFYSTRUCT = *mut SMCSHCHANGENOTIFYSTRUCT;
pub type PSMINFO = *mut SMINFO;
pub const PS_ALL: PLACEHOLDER_STATES = 15;
pub const PS_CLOUDFILE_PLACEHOLDER: PLACEHOLDER_STATES = 8;
pub const PS_CREATE_FILE_ACCESSIBLE: PLACEHOLDER_STATES = 4;
pub const PS_DEFAULT: PLACEHOLDER_STATES = 7;
pub const PS_FULL_PRIMARY_STREAM_AVAILABLE: PLACEHOLDER_STATES = 2;
pub const PS_MARKED_FOR_OFFLINE_AVAILABILITY: PLACEHOLDER_STATES = 1;
pub const PS_NONE: PLACEHOLDER_STATES = 0;
pub const PUIFFDF_DEFAULT: PROPERTYUI_FORMAT_FLAGS = 0;
pub const PUIFFDF_FRIENDLYDATE: PROPERTYUI_FORMAT_FLAGS = 8;
pub const PUIFFDF_NOTIME: PROPERTYUI_FORMAT_FLAGS = 4;
pub const PUIFFDF_RIGHTTOLEFT: PROPERTYUI_FORMAT_FLAGS = 1;
pub const PUIFFDF_SHORTFORMAT: PROPERTYUI_FORMAT_FLAGS = 2;
pub const PUIFNF_DEFAULT: PROPERTYUI_NAME_FLAGS = 0;
pub const PUIFNF_MNEMONIC: PROPERTYUI_NAME_FLAGS = 1;
pub const PUIF_DEFAULT: PROPERTYUI_FLAGS = 0;
pub const PUIF_NOLABELININFOTIP: PROPERTYUI_FLAGS = 2;
pub const PUIF_RIGHTALIGN: PROPERTYUI_FLAGS = 1;
pub const PackageDebugSettings: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb1aec16f_2383_4852_b0e9_8f0b1dc66b4d);
pub const PropertiesUI: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd912f8cf_0396_4915_884e_fb425d32943b);
pub const SBSP_ABSOLUTE: u32 = 0;
pub const SBSP_ACTIVATE_NOFOCUS: u32 = 524288;
pub const SBSP_ALLOW_AUTONAVIGATE: u32 = 65536;
pub const SBSP_CALLERUNTRUSTED: u32 = 8388608;
pub const SBSP_CREATENOHISTORY: u32 = 1048576;
pub const SBSP_DEFBROWSER: u32 = 0;
pub const SBSP_DEFMODE: u32 = 0;
pub const SBSP_EXPLOREMODE: u32 = 32;
pub const SBSP_FEEDNAVIGATION: u32 = 536870912;
pub const SBSP_HELPMODE: u32 = 64;
pub const SBSP_INITIATEDBYHLINKFRAME: u32 = 2147483648;
pub const SBSP_KEEPSAMETEMPLATE: u32 = 131072;
pub const SBSP_KEEPWORDWHEELTEXT: u32 = 262144;
pub const SBSP_NAVIGATEBACK: u32 = 16384;
pub const SBSP_NAVIGATEFORWARD: u32 = 32768;
pub const SBSP_NEWBROWSER: u32 = 2;
pub const SBSP_NOAUTOSELECT: u32 = 67108864;
pub const SBSP_NOTRANSFERHIST: u32 = 128;
pub const SBSP_OPENMODE: u32 = 16;
pub const SBSP_PARENT: u32 = 8192;
pub const SBSP_PLAYNOSOUND: u32 = 2097152;
pub const SBSP_REDIRECT: u32 = 1073741824;
pub const SBSP_RELATIVE: u32 = 4096;
pub const SBSP_SAMEBROWSER: u32 = 1;
pub const SBSP_TRUSTEDFORACTIVEX: u32 = 268435456;
pub const SBSP_TRUSTFIRSTDOWNLOAD: u32 = 16777216;
pub const SBSP_UNTRUSTEDFORDOWNLOAD: u32 = 33554432;
pub const SBSP_WRITENOHISTORY: u32 = 134217728;
pub type SFGAOF = u32;
pub const SFGAO_BROWSABLE: u32 = 134217728;
pub const SFGAO_CANCOPY: u32 = 1;
pub const SFGAO_CANDELETE: u32 = 32;
pub const SFGAO_CANLINK: u32 = 4;
pub const SFGAO_CANMONIKER: u32 = 4194304;
pub const SFGAO_CANMOVE: u32 = 2;
pub const SFGAO_CANRENAME: u32 = 16;
pub const SFGAO_CAPABILITYMASK: u32 = 375;
pub const SFGAO_COMPRESSED: u32 = 67108864;
pub const SFGAO_CONTENTSMASK: u32 = 2147483648;
pub const SFGAO_DISPLAYATTRMASK: u32 = 1032192;
pub const SFGAO_DROPTARGET: u32 = 256;
pub const SFGAO_ENCRYPTED: u32 = 8192;
pub const SFGAO_FILESYSANCESTOR: u32 = 268435456;
pub const SFGAO_FILESYSTEM: u32 = 1073741824;
pub const SFGAO_FOLDER: u32 = 536870912;
pub const SFGAO_GHOSTED: u32 = 32768;
pub const SFGAO_HASPROPSHEET: u32 = 64;
pub const SFGAO_HASSTORAGE: u32 = 4194304;
pub const SFGAO_HASSUBFOLDER: u32 = 2147483648;
pub const SFGAO_HIDDEN: u32 = 524288;
pub const SFGAO_ISSLOW: u32 = 16384;
pub const SFGAO_LINK: u32 = 65536;
pub const SFGAO_NEWCONTENT: u32 = 2097152;
pub const SFGAO_NONENUMERATED: u32 = 1048576;
pub const SFGAO_PKEYSFGAOMASK: u32 = 2164539392;
pub const SFGAO_PLACEHOLDER: u32 = 2048;
pub const SFGAO_READONLY: u32 = 262144;
pub const SFGAO_REMOVABLE: u32 = 33554432;
pub const SFGAO_SHARE: u32 = 131072;
pub const SFGAO_STORAGE: u32 = 8;
pub const SFGAO_STORAGEANCESTOR: u32 = 8388608;
pub const SFGAO_STORAGECAPMASK: u32 = 1891958792;
pub const SFGAO_STREAM: u32 = 4194304;
pub const SFGAO_SYSTEM: u32 = 4096;
pub const SFGAO_VALIDATE: u32 = 16777216;
pub type SHARE_ROLE = i32;
pub const SHARE_ROLE_CONTRIBUTOR: SHARE_ROLE = 1;
pub const SHARE_ROLE_CO_OWNER: SHARE_ROLE = 2;
pub const SHARE_ROLE_CUSTOM: SHARE_ROLE = 4;
pub const SHARE_ROLE_INVALID: SHARE_ROLE = -1;
pub const SHARE_ROLE_MIXED: SHARE_ROLE = 5;
pub const SHARE_ROLE_OWNER: SHARE_ROLE = 3;
pub const SHARE_ROLE_READER: SHARE_ROLE = 0;
pub const SHCIDS_ALLFIELDS: u32 = 2147483648;
pub const SHCIDS_BITMASK: u32 = 4294901760;
pub const SHCIDS_CANONICALONLY: u32 = 268435456;
pub const SHCIDS_COLUMNMASK: u32 = 65535;
pub type SHCONTF = u32;
pub const SHCONTF_CHECKING_FOR_CHILDREN: SHCONTF = 16;
pub const SHCONTF_ENABLE_ASYNC: SHCONTF = 32768;
pub const SHCONTF_FASTITEMS: SHCONTF = 8192;
pub const SHCONTF_FLATLIST: SHCONTF = 16384;
pub const SHCONTF_FOLDERS: SHCONTF = 32;
pub const SHCONTF_INCLUDEHIDDEN: SHCONTF = 128;
pub const SHCONTF_INCLUDESUPERHIDDEN: SHCONTF = 65536;
pub const SHCONTF_INIT_ON_FIRST_NEXT: SHCONTF = 256;
pub const SHCONTF_NAVIGATION_ENUM: SHCONTF = 4096;
pub const SHCONTF_NETPRINTERSRCH: SHCONTF = 512;
pub const SHCONTF_NONFOLDERS: SHCONTF = 64;
pub const SHCONTF_SHAREABLE: SHCONTF = 1024;
pub const SHCONTF_STORAGE: SHCONTF = 2048;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct SHDRAGIMAGE {
    pub sizeDragImage: super::windef::SIZE,
    pub ptOffset: super::windef::POINT,
    pub hbmpDragImage: super::windef::HBITMAP,
    pub crColorKey: super::windef::COLORREF,
}
#[cfg(feature = "windef")]
impl Default for SHDRAGIMAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SHELLVIEWID = windows_sys::core::GUID;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SHELL_ITEM_RESOURCE {
    pub guidType: windows_sys::core::GUID,
    pub szName: [u16; 260],
}
impl Default for SHELL_ITEM_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SHGDNF = u32;
pub const SHGDN_FORADDRESSBAR: SHGDNF = 16384;
pub const SHGDN_FOREDITING: SHGDNF = 4096;
pub const SHGDN_FORPARSING: SHGDNF = 32768;
pub const SHGDN_INFOLDER: SHGDNF = 1;
pub const SHGDN_NORMAL: SHGDNF = 0;
pub type SIATTRIBFLAGS = u32;
pub const SIATTRIBFLAGS_ALLITEMS: SIATTRIBFLAGS = 16384;
pub const SIATTRIBFLAGS_AND: SIATTRIBFLAGS = 1;
pub const SIATTRIBFLAGS_APPCOMPAT: SIATTRIBFLAGS = 3;
pub const SIATTRIBFLAGS_MASK: SIATTRIBFLAGS = 3;
pub const SIATTRIBFLAGS_OR: SIATTRIBFLAGS = 2;
pub type SICHINTF = u32;
pub const SICHINT_ALLFIELDS: SICHINTF = 2147483648;
pub const SICHINT_CANONICAL: SICHINTF = 268435456;
pub const SICHINT_DISPLAY: SICHINTF = 0;
pub const SICHINT_TEST_FILESYSPATH_IF_NOT_EQUAL: SICHINTF = 536870912;
pub const SID_LaunchSourceAppUserModelId: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2ce78010_74db_48bc_9c6a_10f372495723);
pub const SID_LaunchSourceViewSizePreference: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x80605492_67d9_414f_af89_a1cdf1242bc1);
pub const SID_LaunchTargetViewSizePreference: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x26db2472_b7b7_406b_9702_730a4e20d3bf);
pub const SID_ShellExecuteNamedPropertyStore: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeb84ada2_00ff_4992_8324_ed5ce061cb29);
pub const SID_URLExecutionContext: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfb5f8ebc_bbb6_4d10_a461_777291a09030);
pub type SIGDN = i32;
pub const SIGDN_DESKTOPABSOLUTEEDITING: SIGDN = -2147172352;
pub const SIGDN_DESKTOPABSOLUTEPARSING: SIGDN = -2147319808;
pub const SIGDN_FILESYSPATH: SIGDN = -2147123200;
pub const SIGDN_NORMALDISPLAY: SIGDN = 0;
pub const SIGDN_PARENTRELATIVE: SIGDN = -2146959359;
pub const SIGDN_PARENTRELATIVEEDITING: SIGDN = -2147282943;
pub const SIGDN_PARENTRELATIVEFORADDRESSBAR: SIGDN = -2146975743;
pub const SIGDN_PARENTRELATIVEFORUI: SIGDN = -2146877439;
pub const SIGDN_PARENTRELATIVEPARSING: SIGDN = -2147385343;
pub const SIGDN_URL: SIGDN = -2147057664;
pub type SIIGBF = i32;
pub const SIIGBF_BIGGERSIZEOK: SIIGBF = 1;
pub const SIIGBF_CROPTOSQUARE: SIIGBF = 32;
pub const SIIGBF_ICONBACKGROUND: SIIGBF = 128;
pub const SIIGBF_ICONONLY: SIIGBF = 4;
pub const SIIGBF_INCACHEONLY: SIIGBF = 16;
pub const SIIGBF_MEMORYONLY: SIIGBF = 2;
pub const SIIGBF_RESIZETOFIT: SIIGBF = 0;
pub const SIIGBF_SCALEUP: SIIGBF = 256;
pub const SIIGBF_THUMBNAILONLY: SIIGBF = 8;
pub const SIIGBF_WIDETHUMBNAILS: SIIGBF = 64;
pub type SLGP_FLAGS = u32;
pub const SLGP_RAWPATH: SLGP_FLAGS = 4;
pub const SLGP_RELATIVEPRIORITY: SLGP_FLAGS = 8;
pub const SLGP_SHORTPATH: SLGP_FLAGS = 1;
pub const SLGP_UNCPRIORITY: SLGP_FLAGS = 2;
pub const SLR_ANY_MATCH: SLR_FLAGS = 2;
pub type SLR_FLAGS = u32;
pub const SLR_INVOKE_MSI: SLR_FLAGS = 128;
pub const SLR_KNOWNFOLDER: SLR_FLAGS = 1024;
pub const SLR_MACHINE_IN_LOCAL_TARGET: SLR_FLAGS = 2048;
pub const SLR_NOLINKINFO: SLR_FLAGS = 64;
pub const SLR_NONE: SLR_FLAGS = 0;
pub const SLR_NOSEARCH: SLR_FLAGS = 16;
pub const SLR_NOTRACK: SLR_FLAGS = 32;
pub const SLR_NOUPDATE: SLR_FLAGS = 8;
pub const SLR_NO_OBJECT_ID: SLR_FLAGS = 8192;
pub const SLR_NO_UI: SLR_FLAGS = 1;
pub const SLR_NO_UI_WITH_MSG_PUMP: SLR_FLAGS = 257;
pub const SLR_OFFER_DELETE_WITHOUT_FILE: SLR_FLAGS = 512;
pub const SLR_UPDATE: SLR_FLAGS = 4;
pub const SLR_UPDATE_MACHINE_AND_SID: SLR_FLAGS = 4096;
pub const SMAE_CONTRACTED: u32 = 2;
pub const SMAE_EXPANDED: u32 = 1;
pub const SMAE_USER: u32 = 4;
pub const SMAE_VALID: u32 = 7;
#[repr(C)]
#[cfg(feature = "shtypes")]
#[derive(Clone, Copy)]
pub struct SMCSHCHANGENOTIFYSTRUCT {
    pub lEvent: i32,
    pub pidl1: super::shtypes::LPCITEMIDLIST,
    pub pidl2: super::shtypes::LPCITEMIDLIST,
}
#[cfg(feature = "shtypes")]
impl Default for SMCSHCHANGENOTIFYSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SMC_AUTOEXPANDCHANGE: u32 = 66;
pub const SMC_CHEVRONEXPAND: u32 = 25;
pub const SMC_CHEVRONGETTIP: u32 = 47;
pub const SMC_CREATE: u32 = 2;
pub const SMC_DEFAULTICON: u32 = 22;
pub const SMC_DEMOTE: u32 = 17;
pub const SMC_DISPLAYCHEVRONTIP: u32 = 42;
pub const SMC_EXITMENU: u32 = 3;
pub const SMC_GETAUTOEXPANDSTATE: u32 = 65;
pub const SMC_GETBKCONTEXTMENU: u32 = 68;
pub const SMC_GETCONTEXTMENUMODIFIER: u32 = 67;
pub const SMC_GETINFO: u32 = 5;
pub const SMC_GETOBJECT: u32 = 7;
pub const SMC_GETSFINFO: u32 = 6;
pub const SMC_GETSFOBJECT: u32 = 8;
pub const SMC_INITMENU: u32 = 1;
pub const SMC_NEWITEM: u32 = 23;
pub const SMC_OPEN: u32 = 69;
pub const SMC_PROMOTE: u32 = 18;
pub const SMC_REFRESH: u32 = 16;
pub const SMC_SETSFOBJECT: u32 = 45;
pub const SMC_SFDDRESTRICTED: u32 = 48;
pub const SMC_SFEXEC: u32 = 9;
pub const SMC_SFEXEC_MIDDLE: u32 = 49;
pub const SMC_SFSELECTITEM: u32 = 10;
pub const SMC_SHCHANGENOTIFY: u32 = 46;
#[repr(C)]
#[cfg(all(feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct SMDATA {
    pub dwMask: u32,
    pub dwFlags: u32,
    pub hmenu: super::windef::HMENU,
    pub hwnd: super::windef::HWND,
    pub uId: u32,
    pub uIdParent: u32,
    pub uIdAncestor: u32,
    pub punk: *mut core::ffi::c_void,
    pub pidlFolder: super::shtypes::LPITEMIDLIST,
    pub pidlItem: super::shtypes::LPITEMIDLIST,
    pub psf: *mut core::ffi::c_void,
    pub pvUserData: *mut core::ffi::c_void,
}
#[cfg(all(feature = "shtypes", feature = "windef"))]
impl Default for SMDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SMDM_HMENU: u32 = 2;
pub const SMDM_SHELLFOLDER: u32 = 1;
pub const SMDM_TOOLBAR: u32 = 4;
pub const SMIF_ACCELERATOR: tagSMINFOFLAGS = 2;
pub const SMIF_ALTSTATE: tagSMINFOFLAGS = 2048;
pub const SMIF_CHECKED: tagSMINFOFLAGS = 32;
pub const SMIF_DEMOTED: tagSMINFOFLAGS = 1024;
pub const SMIF_DISABLED: tagSMINFOFLAGS = 256;
pub const SMIF_DRAGNDROP: tagSMINFOFLAGS = 4096;
pub const SMIF_DROPCASCADE: tagSMINFOFLAGS = 64;
pub const SMIF_DROPTARGET: tagSMINFOFLAGS = 4;
pub const SMIF_HIDDEN: tagSMINFOFLAGS = 128;
pub const SMIF_ICON: tagSMINFOFLAGS = 1;
pub const SMIF_NEW: tagSMINFOFLAGS = 8192;
pub const SMIF_SUBMENU: tagSMINFOFLAGS = 8;
pub const SMIF_TRACKPOPUP: tagSMINFOFLAGS = 512;
pub const SMIM_FLAGS: tagSMINFOMASK = 2;
pub const SMIM_ICON: tagSMINFOMASK = 4;
pub const SMIM_TYPE: tagSMINFOMASK = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SMINFO {
    pub dwMask: u32,
    pub dwType: u32,
    pub dwFlags: u32,
    pub iIcon: i32,
}
pub const SMINIT_AUTOEXPAND: u32 = 256;
pub const SMINIT_AUTOTOOLTIP: u32 = 512;
pub const SMINIT_CACHED: u32 = 16;
pub const SMINIT_DEFAULT: u32 = 0;
pub const SMINIT_DROPONCONTAINER: u32 = 1024;
pub const SMINIT_HORIZONTAL: u32 = 536870912;
pub const SMINIT_RESTRICT_DRAGDROP: u32 = 2;
pub const SMINIT_TOPLEVEL: u32 = 4;
pub const SMINIT_VERTICAL: u32 = 268435456;
pub const SMINV_ID: u32 = 8;
pub const SMINV_REFRESH: u32 = 1;
pub const SMIT_SEPARATOR: tagSMINFOTYPE = 1;
pub const SMIT_STRING: tagSMINFOTYPE = 2;
pub const SMSET_BOTTOM: u32 = 536870912;
pub const SMSET_DONTOWN: u32 = 1;
pub const SMSET_TOP: u32 = 268435456;
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Default)]
pub struct SORTCOLUMN {
    pub propkey: super::wtypes::PROPERTYKEY,
    pub direction: SORTDIRECTION,
}
pub type SORTDIRECTION = i32;
pub const SORT_ASCENDING: tagSORTDIRECTION = 1;
pub const SORT_DESCENDING: tagSORTDIRECTION = -1;
pub type SORT_ORDER_TYPE = i32;
pub const SOT_DEFAULT: SORT_ORDER_TYPE = 0;
pub const SOT_IGNORE_FOLDERNESS: SORT_ORDER_TYPE = 1;
pub type SPACTION = i32;
pub const SPACTION_APPLYINGATTRIBS: SPACTION = 4;
pub const SPACTION_CALCULATING: SPACTION = 7;
pub const SPACTION_COPYING: SPACTION = 2;
pub const SPACTION_COPY_MOVING: SPACTION = 13;
pub const SPACTION_DELETING: SPACTION = 10;
pub const SPACTION_DOWNLOADING: SPACTION = 5;
pub const SPACTION_FORMATTING: SPACTION = 12;
pub const SPACTION_MOVING: SPACTION = 1;
pub const SPACTION_NONE: SPACTION = 0;
pub const SPACTION_RECYCLING: SPACTION = 3;
pub const SPACTION_RENAMING: SPACTION = 11;
pub const SPACTION_SEARCHING_FILES: SPACTION = 9;
pub const SPACTION_SEARCHING_INTERNET: SPACTION = 6;
pub const SPACTION_UPLOADING: SPACTION = 8;
pub type SPBEGINF = u32;
pub const SPBEGINF_AUTOTIME: SPBEGINF = 2;
pub const SPBEGINF_MARQUEEPROGRESS: SPBEGINF = 32;
pub const SPBEGINF_NOCANCELBUTTON: SPBEGINF = 64;
pub const SPBEGINF_NOPROGRESSBAR: SPBEGINF = 16;
pub const SPBEGINF_NORMAL: SPBEGINF = 0;
pub const SPFF_CREATED_ON_THIS_DEVICE: STORAGE_PROVIDER_FILE_FLAGS = 2;
pub const SPFF_DOWNLOAD_BY_DEFAULT: STORAGE_PROVIDER_FILE_FLAGS = 1;
pub const SPFF_NONE: STORAGE_PROVIDER_FILE_FLAGS = 0;
pub type SPINITF = u32;
pub const SPINITF_MODAL: SPINITF = 1;
pub const SPINITF_NOMINIMIZE: SPINITF = 8;
pub const SPINITF_NORMAL: SPINITF = 0;
pub type SPTEXT = i32;
pub const SPTEXT_ACTIONDESCRIPTION: SPTEXT = 1;
pub const SPTEXT_ACTIONDETAIL: SPTEXT = 2;
pub type STGOP = i32;
pub const STGOP_APPLYPROPERTIES: STGOP = 8;
pub const STGOP_COPY: STGOP = 2;
pub const STGOP_MOVE: STGOP = 1;
pub const STGOP_NEW: STGOP = 10;
pub const STGOP_REMOVE: STGOP = 5;
pub const STGOP_RENAME: STGOP = 6;
pub const STGOP_SYNC: STGOP = 3;
pub type STGTRANSCONFIRMATION = windows_sys::core::GUID;
pub type STORAGE_PROVIDER_FILE_FLAGS = u32;
pub type STPFLAG = u32;
pub const STPF_NONE: STPFLAG = 0;
pub const STPF_USEAPPPEEKALWAYS: STPFLAG = 4;
pub const STPF_USEAPPPEEKWHENACTIVE: STPFLAG = 8;
pub const STPF_USEAPPTHUMBNAILALWAYS: STPFLAG = 1;
pub const STPF_USEAPPTHUMBNAILWHENACTIVE: STPFLAG = 2;
pub const STR_AVOID_DRIVE_RESTRICTION_POLICY: windows_sys::core::PCWSTR = windows_sys::core::w!("Avoid Drive Restriction Policy");
pub const STR_BIND_DELEGATE_CREATE_OBJECT: windows_sys::core::PCWSTR = windows_sys::core::w!("Delegate Object Creation");
pub const STR_BIND_FOLDERS_READ_ONLY: windows_sys::core::PCWSTR = windows_sys::core::w!("Folders As Read Only");
pub const STR_BIND_FOLDER_ENUM_MODE: windows_sys::core::PCWSTR = windows_sys::core::w!("Folder Enum Mode");
pub const STR_BIND_FORCE_FOLDER_SHORTCUT_RESOLVE: windows_sys::core::PCWSTR = windows_sys::core::w!("Force Folder Shortcut Resolve");
pub const STR_DONT_PARSE_RELATIVE: windows_sys::core::PCWSTR = windows_sys::core::w!("Don\'t Parse Relative");
pub const STR_DONT_RESOLVE_LINK: windows_sys::core::PCWSTR = windows_sys::core::w!("Don\'t Resolve Link");
pub const STR_ENUM_ITEMS_FLAGS: windows_sys::core::PCWSTR = windows_sys::core::w!("SHCONTF");
pub const STR_FILE_SYS_BIND_DATA: windows_sys::core::PCWSTR = windows_sys::core::w!("File System Bind Data");
pub const STR_FILE_SYS_BIND_DATA_WIN7_FORMAT: windows_sys::core::PCWSTR = windows_sys::core::w!("Win7FileSystemIdList");
pub const STR_GET_ASYNC_HANDLER: windows_sys::core::PCWSTR = windows_sys::core::w!("GetAsyncHandler");
pub const STR_GPS_BESTEFFORT: windows_sys::core::PCWSTR = windows_sys::core::w!("GPS_BESTEFFORT");
pub const STR_GPS_DELAYCREATION: windows_sys::core::PCWSTR = windows_sys::core::w!("GPS_DELAYCREATION");
pub const STR_GPS_FASTPROPERTIESONLY: windows_sys::core::PCWSTR = windows_sys::core::w!("GPS_FASTPROPERTIESONLY");
pub const STR_GPS_HANDLERPROPERTIESONLY: windows_sys::core::PCWSTR = windows_sys::core::w!("GPS_HANDLERPROPERTIESONLY");
pub const STR_GPS_NO_OPLOCK: windows_sys::core::PCWSTR = windows_sys::core::w!("GPS_NO_OPLOCK");
pub const STR_GPS_OPENSLOWITEM: windows_sys::core::PCWSTR = windows_sys::core::w!("GPS_OPENSLOWITEM");
pub const STR_INTERNAL_NAVIGATE: windows_sys::core::PCWSTR = windows_sys::core::w!("Internal Navigation");
pub const STR_INTERNETFOLDER_PARSE_ONLY_URLMON_BINDABLE: windows_sys::core::PCWSTR = windows_sys::core::w!("Validate URL");
pub const STR_ITEM_CACHE_CONTEXT: windows_sys::core::PCWSTR = windows_sys::core::w!("ItemCacheContext");
pub const STR_NO_VALIDATE_FILENAME_CHARS: windows_sys::core::PCWSTR = windows_sys::core::w!("NoValidateFilenameChars");
pub const STR_PARSE_ALLOW_INTERNET_SHELL_FOLDERS: windows_sys::core::PCWSTR = windows_sys::core::w!("Allow binding to Internet shell folder handlers and negate STR_PARSE_PREFER_WEB_BROWSING");
pub const STR_PARSE_AND_CREATE_ITEM: windows_sys::core::PCWSTR = windows_sys::core::w!("ParseAndCreateItem");
pub const STR_PARSE_DONT_REQUIRE_VALIDATED_URLS: windows_sys::core::PCWSTR = windows_sys::core::w!("Do not require validated URLs");
pub const STR_PARSE_EXPLICIT_ASSOCIATION_SUCCESSFUL: windows_sys::core::PCWSTR = windows_sys::core::w!("ExplicitAssociationSuccessful");
pub const STR_PARSE_PREFER_FOLDER_BROWSING: windows_sys::core::PCWSTR = windows_sys::core::w!("Parse Prefer Folder Browsing");
pub const STR_PARSE_PREFER_WEB_BROWSING: windows_sys::core::PCWSTR = windows_sys::core::w!("Do not bind to Internet shell folder handlers");
pub const STR_PARSE_PROPERTYSTORE: windows_sys::core::PCWSTR = windows_sys::core::w!("DelegateNamedProperties");
pub const STR_PARSE_SHELL_PROTOCOL_TO_FILE_OBJECTS: windows_sys::core::PCWSTR = windows_sys::core::w!("Parse Shell Protocol To File Objects");
pub const STR_PARSE_SHOW_NET_DIAGNOSTICS_UI: windows_sys::core::PCWSTR = windows_sys::core::w!("Show network diagnostics UI");
pub const STR_PARSE_SKIP_NET_CACHE: windows_sys::core::PCWSTR = windows_sys::core::w!("Skip Net Resource Cache");
pub const STR_PARSE_TRANSLATE_ALIASES: windows_sys::core::PCWSTR = windows_sys::core::w!("Parse Translate Aliases");
pub const STR_PARSE_WITH_EXPLICIT_ASSOCAPP: windows_sys::core::PCWSTR = windows_sys::core::w!("ExplicitAssociationApp");
pub const STR_PARSE_WITH_EXPLICIT_PROGID: windows_sys::core::PCWSTR = windows_sys::core::w!("ExplicitProgid");
pub const STR_PROPERTYBAG_PARAM: windows_sys::core::PCWSTR = windows_sys::core::w!("SHBindCtxPropertyBag");
pub const STR_REFERRER_IDENTIFIER: windows_sys::core::PCWSTR = windows_sys::core::w!("Referrer Identifier");
pub const STR_SKIP_BINDING_CLSID: windows_sys::core::PCWSTR = windows_sys::core::w!("Skip Binding CLSID");
pub const STR_STORAGEITEM_CREATION_FLAGS: windows_sys::core::PCWSTR = windows_sys::core::w!("SHGETSTORAGEITEM");
pub const STR_TAB_REUSE_IDENTIFIER: windows_sys::core::PCWSTR = windows_sys::core::w!("Tab Reuse Identifier");
pub const STR_TRACK_CLSID: windows_sys::core::PCWSTR = windows_sys::core::w!("Track the CLSID");
pub const STS_EXCLUDED: SYNC_TRANSFER_STATUS = 256;
pub const STS_FETCHING_METADATA: SYNC_TRANSFER_STATUS = 32;
pub const STS_HASERROR: SYNC_TRANSFER_STATUS = 16;
pub const STS_HASWARNING: SYNC_TRANSFER_STATUS = 128;
pub const STS_INCOMPLETE: SYNC_TRANSFER_STATUS = 512;
pub const STS_NEEDSDOWNLOAD: SYNC_TRANSFER_STATUS = 2;
pub const STS_NEEDSUPLOAD: SYNC_TRANSFER_STATUS = 1;
pub const STS_NONE: SYNC_TRANSFER_STATUS = 0;
pub const STS_PAUSED: SYNC_TRANSFER_STATUS = 8;
pub const STS_PLACEHOLDER_IFEMPTY: SYNC_TRANSFER_STATUS = 1024;
pub const STS_TRANSFERRING: SYNC_TRANSFER_STATUS = 4;
pub const STS_USER_REQUESTED_REFRESH: SYNC_TRANSFER_STATUS = 64;
#[repr(C)]
#[cfg(all(feature = "oleidl", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct SV2CVW2_PARAMS {
    pub cbSize: u32,
    pub psvPrev: *mut core::ffi::c_void,
    pub pfs: LPCFOLDERSETTINGS,
    pub psbOwner: *mut core::ffi::c_void,
    pub prcView: *mut super::windef::RECT,
    pub pvid: *const SHELLVIEWID,
    pub hwndView: super::windef::HWND,
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
impl Default for SV2CVW2_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SV2GV_CURRENTVIEW: u32 = 4294967295;
pub const SV2GV_DEFAULTVIEW: u32 = 4294967294;
pub type SVGIO = u32;
pub const SVGIO_ALLVIEW: SVGIO = 2;
pub const SVGIO_BACKGROUND: SVGIO = 0;
pub const SVGIO_CHECKED: SVGIO = 3;
pub const SVGIO_FLAG_VIEWORDER: SVGIO = 2147483648;
pub const SVGIO_SELECTION: SVGIO = 1;
pub const SVGIO_TYPE_MASK: SVGIO = 15;
pub type SVSIF = u32;
pub const SVSI_CHECK: SVSIF = 256;
pub const SVSI_CHECK2: SVSIF = 512;
pub const SVSI_DESELECT: SVSIF = 0;
pub const SVSI_DESELECTOTHERS: SVSIF = 4;
pub const SVSI_EDIT: SVSIF = 3;
pub const SVSI_ENSUREVISIBLE: SVSIF = 8;
pub const SVSI_FOCUSED: SVSIF = 16;
pub const SVSI_KEYBOARDSELECT: SVSIF = 1025;
pub const SVSI_NOSTATECHANGE: u32 = 2147483648;
pub const SVSI_NOTAKEFOCUS: SVSIF = 1073741824;
pub const SVSI_POSITIONITEM: SVSIF = 128;
pub const SVSI_SELECT: SVSIF = 1;
pub const SVSI_SELECTIONMARK: SVSIF = 64;
pub const SVSI_TRANSLATEPT: SVSIF = 32;
pub const SVUIA_ACTIVATE_FOCUS: SVUIA_STATUS = 2;
pub const SVUIA_ACTIVATE_NOFOCUS: SVUIA_STATUS = 1;
pub const SVUIA_DEACTIVATE: SVUIA_STATUS = 0;
pub const SVUIA_INPLACEACTIVATE: SVUIA_STATUS = 3;
pub type SVUIA_STATUS = i32;
pub type SYNC_TRANSFER_STATUS = u32;
pub const ScheduledTasks: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd6277990_4c6a_11cf_8d87_00aa0060f5bf);
pub const SearchFolderItemFactory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x14010e02_bbbd_41f0_88e3_eda371216584);
pub const SharingConfigurationManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x49f371e1_8c5c_4d9c_9a3b_54a6827f513c);
pub const ShellDesktop: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00021400_0000_0000_c000_000000000046);
pub const ShellFSFolder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf3364ba0_65b9_11ce_a9ba_00aa004ae837);
pub const ShellItem: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9ac9fbe1_e0a2_4ad6_b4ee_e212013ea917);
pub const ShellLibrary: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd9b3211d_e57f_4426_aaef_30a806add397);
pub const ShellLink: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00021401_0000_0000_c000_000000000046);
pub const SizeCategorizer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x55d7b852_f6d1_42f2_aa75_8728a1b2d264);
pub const SuspensionDependencyManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6b273fc5_61fd_4918_95a2_c3b5e9d7f581);
pub type TBPFLAG = u32;
pub const TBPF_ERROR: TBPFLAG = 4;
pub const TBPF_INDETERMINATE: TBPFLAG = 1;
pub const TBPF_NOPROGRESS: TBPFLAG = 0;
pub const TBPF_NORMAL: TBPFLAG = 2;
pub const TBPF_PAUSED: TBPFLAG = 8;
pub const THBF_DISABLED: THUMBBUTTONFLAGS = 1;
pub const THBF_DISMISSONCLICK: THUMBBUTTONFLAGS = 2;
pub const THBF_ENABLED: THUMBBUTTONFLAGS = 0;
pub const THBF_HIDDEN: THUMBBUTTONFLAGS = 8;
pub const THBF_NOBACKGROUND: THUMBBUTTONFLAGS = 4;
pub const THBF_NONINTERACTIVE: THUMBBUTTONFLAGS = 16;
pub const THBN_CLICKED: u32 = 6144;
pub const THB_BITMAP: THUMBBUTTONMASK = 1;
pub const THB_FLAGS: THUMBBUTTONMASK = 8;
pub const THB_ICON: THUMBBUTTONMASK = 2;
pub const THB_TOOLTIP: THUMBBUTTONMASK = 4;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct THUMBBUTTON {
    pub dwMask: THUMBBUTTONMASK,
    pub iId: u32,
    pub iBitmap: u32,
    pub hIcon: super::windef::HICON,
    pub szTip: [u16; 260],
    pub dwFlags: THUMBBUTTONFLAGS,
}
#[cfg(feature = "windef")]
impl Default for THUMBBUTTON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type THUMBBUTTONFLAGS = u32;
pub type THUMBBUTTONMASK = u32;
pub type TRANSFER_ADVISE_STATE = u32;
pub type TRANSFER_SOURCE_FLAGS = u32;
pub const TSF_ALLOW_DECRYPTION: TRANSFER_SOURCE_FLAGS = 4;
pub const TSF_COPY_CREATION_TIME: TRANSFER_SOURCE_FLAGS = 16;
pub const TSF_COPY_HARD_LINK: TRANSFER_SOURCE_FLAGS = 256;
pub const TSF_COPY_LOCALIZED_NAME: TRANSFER_SOURCE_FLAGS = 512;
pub const TSF_COPY_WRITE_TIME: TRANSFER_SOURCE_FLAGS = 32;
pub const TSF_DELETE_RECYCLE_IF_POSSIBLE: TRANSFER_SOURCE_FLAGS = 128;
pub const TSF_FAIL_EXIST: TRANSFER_SOURCE_FLAGS = 0;
pub const TSF_MOVE_AS_COPY_DELETE: TRANSFER_SOURCE_FLAGS = 1024;
pub const TSF_NORMAL: TRANSFER_SOURCE_FLAGS = 0;
pub const TSF_NO_SECURITY: TRANSFER_SOURCE_FLAGS = 8;
pub const TSF_OVERWRITE_EXIST: TRANSFER_SOURCE_FLAGS = 2;
pub const TSF_RENAME_EXIST: TRANSFER_SOURCE_FLAGS = 1;
pub const TSF_SUSPEND_SHELLEVENTS: TRANSFER_SOURCE_FLAGS = 2048;
pub const TSF_USE_FULL_ACCESS: TRANSFER_SOURCE_FLAGS = 64;
pub const TS_INDETERMINATE: TRANSFER_ADVISE_STATE = 4;
pub const TS_NONE: TRANSFER_ADVISE_STATE = 0;
pub const TS_PERFORMING: TRANSFER_ADVISE_STATE = 1;
pub const TS_PREPARING: TRANSFER_ADVISE_STATE = 2;
pub const TaskbarList: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56fdf344_fd6d_11d0_958a_006097c9a090);
pub const UserNotification: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0010890e_8789_413c_adbc_48f5b511b3af);
pub type tagBANDSITECID = i32;
pub type tagDESKBANDCID = i32;
pub type tagMENUBANDHANDLERCID = i32;
pub type tagMENUPOPUPPOPUPFLAGS = i32;
pub type tagMENUPOPUPSELECT = i32;
pub type tagSMINFOFLAGS = i32;
pub type tagSMINFOMASK = i32;
pub type tagSMINFOTYPE = i32;
pub type tagSORTDIRECTION = i32;
