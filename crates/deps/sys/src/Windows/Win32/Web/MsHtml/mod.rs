#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ComputeInvCMAP(prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD, ncolors: u32, pinvtable: *mut u8, cbtable: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateDDrawSurfaceOnDIB(hbmdib: super::super::Graphics::Gdi::HBITMAP, ppsurface: *mut super::super::Graphics::DirectDraw::IDirectDrawSurface) -> ::windows_sys::core::HRESULT;
    pub fn CreateMIMEMap(ppmap: *mut IMapMIMEToCLSID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn DecodeImage(pstream: super::super::System::Com::IStream, pmap: IMapMIMEToCLSID, peventsink: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DecodeImageEx(pstream: super::super::System::Com::IStream, pmap: IMapMIMEToCLSID, peventsink: ::windows_sys::core::IUnknown, pszmimetypeparam: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DitherTo8(pdestbits: *mut u8, ndestpitch: i32, psrcbits: *mut u8, nsrcpitch: i32, bfidsrc: *const ::windows_sys::core::GUID, prgbdestcolors: *mut super::super::Graphics::Gdi::RGBQUAD, prgbsrccolors: *mut super::super::Graphics::Gdi::RGBQUAD, pbdestinvmap: *mut u8, x: i32, y: i32, cx: i32, cy: i32, ldesttrans: i32, lsrctrans: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DoPrivacyDlg(hwndowner: super::super::Foundation::HWND, pszurl: super::super::Foundation::PWSTR, pprivacyenum: IEnumPrivacyRecords, freportallsites: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn GetMaxMIMEIDBytes(pnmaxbytes: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn IdentifyMIMEType(pbbytes: *const u8, nbytes: u32, pnformat: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAccessDeniedDialog(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PSTR, pszcontentdescription: super::super::Foundation::PSTR, pratingdetails: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAccessDeniedDialog2(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PSTR, pratingdetails: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAccessDeniedDialog2W(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PWSTR, pratingdetails: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAccessDeniedDialogW(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PWSTR, pszcontentdescription: super::super::Foundation::PWSTR, pratingdetails: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAddToApprovedSites(hdlg: super::super::Foundation::HWND, cbpasswordblob: u32, pbpasswordblob: *mut u8, lpszurl: super::super::Foundation::PWSTR, falwaysnever: super::super::Foundation::BOOL, fsitepage: super::super::Foundation::BOOL, fapprovedsitesenforced: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingCheckUserAccess(pszusername: super::super::Foundation::PSTR, pszurl: super::super::Foundation::PSTR, pszratinginfo: super::super::Foundation::PSTR, pdata: *const u8, cbdata: u32, ppratingdetails: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingCheckUserAccessW(pszusername: super::super::Foundation::PWSTR, pszurl: super::super::Foundation::PWSTR, pszratinginfo: super::super::Foundation::PWSTR, pdata: *const u8, cbdata: u32, ppratingdetails: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingClickedOnPRFInternal(hwndowner: super::super::Foundation::HWND, param1: super::super::Foundation::HINSTANCE, lpszfilename: super::super::Foundation::PSTR, nshow: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingClickedOnRATInternal(hwndowner: super::super::Foundation::HWND, param1: super::super::Foundation::HINSTANCE, lpszfilename: super::super::Foundation::PSTR, nshow: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingEnable(hwndparent: super::super::Foundation::HWND, pszusername: super::super::Foundation::PSTR, fenable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingEnableW(hwndparent: super::super::Foundation::HWND, pszusername: super::super::Foundation::PWSTR, fenable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn RatingEnabledQuery() -> ::windows_sys::core::HRESULT;
    pub fn RatingFreeDetails(pratingdetails: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn RatingInit() -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingObtainCancel(hratingobtainquery: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingObtainQuery(psztargeturl: super::super::Foundation::PSTR, dwuserdata: u32, fcallback: isize, phratingobtainquery: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingObtainQueryW(psztargeturl: super::super::Foundation::PWSTR, dwuserdata: u32, fcallback: isize, phratingobtainquery: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingSetupUI(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingSetupUIW(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SniffStream(pinstream: super::super::System::Com::IStream, pnformat: *mut u32, ppoutstream: *mut super::super::System::Com::IStream) -> ::windows_sys::core::HRESULT;
}
pub const ADDRESSBAND: u32 = 2u32;
#[repr(transparent)]
pub struct ADDURL_FLAG(pub i32);
pub const ADDURL_FIRST: ADDURL_FLAG = ADDURL_FLAG(0i32);
pub const ADDURL_ADDTOHISTORYANDCACHE: ADDURL_FLAG = ADDURL_FLAG(0i32);
pub const ADDURL_ADDTOCACHE: ADDURL_FLAG = ADDURL_FLAG(1i32);
pub const ADDURL_Max: ADDURL_FLAG = ADDURL_FLAG(2147483647i32);
impl ::core::marker::Copy for ADDURL_FLAG {}
impl ::core::clone::Clone for ADDURL_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
pub const AnchorClick: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 332742972, data2: 13241, data3: 4562, data4: [149, 167, 0, 192, 79, 142, 203, 2] };
pub const ApplicationCache: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616873, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct BEHAVIOR_EVENT(pub i32);
pub const BEHAVIOREVENT_FIRST: BEHAVIOR_EVENT = BEHAVIOR_EVENT(0i32);
pub const BEHAVIOREVENT_CONTENTREADY: BEHAVIOR_EVENT = BEHAVIOR_EVENT(0i32);
pub const BEHAVIOREVENT_DOCUMENTREADY: BEHAVIOR_EVENT = BEHAVIOR_EVENT(1i32);
pub const BEHAVIOREVENT_APPLYSTYLE: BEHAVIOR_EVENT = BEHAVIOR_EVENT(2i32);
pub const BEHAVIOREVENT_DOCUMENTCONTEXTCHANGE: BEHAVIOR_EVENT = BEHAVIOR_EVENT(3i32);
pub const BEHAVIOREVENT_CONTENTSAVE: BEHAVIOR_EVENT = BEHAVIOR_EVENT(4i32);
pub const BEHAVIOREVENT_LAST: BEHAVIOR_EVENT = BEHAVIOR_EVENT(4i32);
pub const BEHAVIOR_EVENT_Max: BEHAVIOR_EVENT = BEHAVIOR_EVENT(2147483647i32);
impl ::core::marker::Copy for BEHAVIOR_EVENT {}
impl ::core::clone::Clone for BEHAVIOR_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BEHAVIOR_EVENT_FLAGS(pub i32);
pub const BEHAVIOREVENTFLAGS_BUBBLE: BEHAVIOR_EVENT_FLAGS = BEHAVIOR_EVENT_FLAGS(1i32);
pub const BEHAVIOREVENTFLAGS_STANDARDADDITIVE: BEHAVIOR_EVENT_FLAGS = BEHAVIOR_EVENT_FLAGS(2i32);
pub const BEHAVIOR_EVENT_FLAGS_Max: BEHAVIOR_EVENT_FLAGS = BEHAVIOR_EVENT_FLAGS(2147483647i32);
impl ::core::marker::Copy for BEHAVIOR_EVENT_FLAGS {}
impl ::core::clone::Clone for BEHAVIOR_EVENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BEHAVIOR_LAYOUT_INFO(pub i32);
pub const BEHAVIORLAYOUTINFO_FULLDELEGATION: BEHAVIOR_LAYOUT_INFO = BEHAVIOR_LAYOUT_INFO(1i32);
pub const BEHAVIORLAYOUTINFO_MODIFYNATURAL: BEHAVIOR_LAYOUT_INFO = BEHAVIOR_LAYOUT_INFO(2i32);
pub const BEHAVIORLAYOUTINFO_MAPSIZE: BEHAVIOR_LAYOUT_INFO = BEHAVIOR_LAYOUT_INFO(4i32);
pub const BEHAVIOR_LAYOUT_INFO_Max: BEHAVIOR_LAYOUT_INFO = BEHAVIOR_LAYOUT_INFO(2147483647i32);
impl ::core::marker::Copy for BEHAVIOR_LAYOUT_INFO {}
impl ::core::clone::Clone for BEHAVIOR_LAYOUT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BEHAVIOR_LAYOUT_MODE(pub i32);
pub const BEHAVIORLAYOUTMODE_NATURAL: BEHAVIOR_LAYOUT_MODE = BEHAVIOR_LAYOUT_MODE(1i32);
pub const BEHAVIORLAYOUTMODE_MINWIDTH: BEHAVIOR_LAYOUT_MODE = BEHAVIOR_LAYOUT_MODE(2i32);
pub const BEHAVIORLAYOUTMODE_MAXWIDTH: BEHAVIOR_LAYOUT_MODE = BEHAVIOR_LAYOUT_MODE(4i32);
pub const BEHAVIORLAYOUTMODE_MEDIA_RESOLUTION: BEHAVIOR_LAYOUT_MODE = BEHAVIOR_LAYOUT_MODE(16384i32);
pub const BEHAVIORLAYOUTMODE_FINAL_PERCENT: BEHAVIOR_LAYOUT_MODE = BEHAVIOR_LAYOUT_MODE(32768i32);
pub const BEHAVIOR_LAYOUT_MODE_Max: BEHAVIOR_LAYOUT_MODE = BEHAVIOR_LAYOUT_MODE(2147483647i32);
impl ::core::marker::Copy for BEHAVIOR_LAYOUT_MODE {}
impl ::core::clone::Clone for BEHAVIOR_LAYOUT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BEHAVIOR_RELATION(pub i32);
pub const BEHAVIOR_FIRSTRELATION: BEHAVIOR_RELATION = BEHAVIOR_RELATION(0i32);
pub const BEHAVIOR_SAMEELEMENT: BEHAVIOR_RELATION = BEHAVIOR_RELATION(0i32);
pub const BEHAVIOR_PARENT: BEHAVIOR_RELATION = BEHAVIOR_RELATION(1i32);
pub const BEHAVIOR_CHILD: BEHAVIOR_RELATION = BEHAVIOR_RELATION(2i32);
pub const BEHAVIOR_SIBLING: BEHAVIOR_RELATION = BEHAVIOR_RELATION(3i32);
pub const BEHAVIOR_LASTRELATION: BEHAVIOR_RELATION = BEHAVIOR_RELATION(3i32);
pub const BEHAVIOR_RELATION_Max: BEHAVIOR_RELATION = BEHAVIOR_RELATION(2147483647i32);
impl ::core::marker::Copy for BEHAVIOR_RELATION {}
impl ::core::clone::Clone for BEHAVIOR_RELATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BEHAVIOR_RENDER_INFO(pub i32);
pub const BEHAVIORRENDERINFO_BEFOREBACKGROUND: BEHAVIOR_RENDER_INFO = BEHAVIOR_RENDER_INFO(1i32);
pub const BEHAVIORRENDERINFO_AFTERBACKGROUND: BEHAVIOR_RENDER_INFO = BEHAVIOR_RENDER_INFO(2i32);
pub const BEHAVIORRENDERINFO_BEFORECONTENT: BEHAVIOR_RENDER_INFO = BEHAVIOR_RENDER_INFO(4i32);
pub const BEHAVIORRENDERINFO_AFTERCONTENT: BEHAVIOR_RENDER_INFO = BEHAVIOR_RENDER_INFO(8i32);
pub const BEHAVIORRENDERINFO_AFTERFOREGROUND: BEHAVIOR_RENDER_INFO = BEHAVIOR_RENDER_INFO(32i32);
pub const BEHAVIORRENDERINFO_ABOVECONTENT: BEHAVIOR_RENDER_INFO = BEHAVIOR_RENDER_INFO(40i32);
pub const BEHAVIORRENDERINFO_ALLLAYERS: BEHAVIOR_RENDER_INFO = BEHAVIOR_RENDER_INFO(255i32);
pub const BEHAVIORRENDERINFO_DISABLEBACKGROUND: BEHAVIOR_RENDER_INFO = BEHAVIOR_RENDER_INFO(256i32);
pub const BEHAVIORRENDERINFO_DISABLENEGATIVEZ: BEHAVIOR_RENDER_INFO = BEHAVIOR_RENDER_INFO(512i32);
pub const BEHAVIORRENDERINFO_DISABLECONTENT: BEHAVIOR_RENDER_INFO = BEHAVIOR_RENDER_INFO(1024i32);
pub const BEHAVIORRENDERINFO_DISABLEPOSITIVEZ: BEHAVIOR_RENDER_INFO = BEHAVIOR_RENDER_INFO(2048i32);
pub const BEHAVIORRENDERINFO_DISABLEALLLAYERS: BEHAVIOR_RENDER_INFO = BEHAVIOR_RENDER_INFO(3840i32);
pub const BEHAVIORRENDERINFO_HITTESTING: BEHAVIOR_RENDER_INFO = BEHAVIOR_RENDER_INFO(4096i32);
pub const BEHAVIORRENDERINFO_SURFACE: BEHAVIOR_RENDER_INFO = BEHAVIOR_RENDER_INFO(1048576i32);
pub const BEHAVIORRENDERINFO_3DSURFACE: BEHAVIOR_RENDER_INFO = BEHAVIOR_RENDER_INFO(2097152i32);
pub const BEHAVIOR_RENDER_INFO_Max: BEHAVIOR_RENDER_INFO = BEHAVIOR_RENDER_INFO(2147483647i32);
impl ::core::marker::Copy for BEHAVIOR_RENDER_INFO {}
impl ::core::clone::Clone for BEHAVIOR_RENDER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const BlockFormats: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612785, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct BoolValue(pub i32);
pub const True: BoolValue = BoolValue(1i32);
pub const False: BoolValue = BoolValue(0i32);
pub const BoolValue_Max: BoolValue = BoolValue(2147483647i32);
impl ::core::marker::Copy for BoolValue {}
impl ::core::clone::Clone for BoolValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CARET_DIRECTION(pub i32);
pub const CARET_DIRECTION_INDETERMINATE: CARET_DIRECTION = CARET_DIRECTION(0i32);
pub const CARET_DIRECTION_SAME: CARET_DIRECTION = CARET_DIRECTION(1i32);
pub const CARET_DIRECTION_BACKWARD: CARET_DIRECTION = CARET_DIRECTION(2i32);
pub const CARET_DIRECTION_FORWARD: CARET_DIRECTION = CARET_DIRECTION(3i32);
pub const CARET_DIRECTION_Max: CARET_DIRECTION = CARET_DIRECTION(2147483647i32);
impl ::core::marker::Copy for CARET_DIRECTION {}
impl ::core::clone::Clone for CARET_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CATID_MSOfficeAntiVirus: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1459604528, data2: 54168, data3: 4560, data4: [178, 174, 0, 160, 201, 8, 250, 73] };
pub const CClientCaps: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2123088974, data2: 44799, data3: 4561, data4: [137, 194, 0, 192, 79, 182, 191, 196] };
pub const CDeviceRect: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612436, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CDownloadBehavior: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612158, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CEventObj: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611850, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CGID_DocHostCommandHandler: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4086022722, data2: 47440, data3: 4561, data4: [137, 24, 0, 192, 79, 194, 200, 54] };
pub const CGID_EditStateCommands: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611894, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CHeaderFooter: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612429, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CLayoutRect: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612324, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CMDID_HOSTCONTEXT_URL: u32 = 8u32;
pub const CMDID_SCRIPTSITE_ALLOWRECOVERY: u32 = 9u32;
pub const CMDID_SCRIPTSITE_BASEIURI: u32 = 10u32;
pub const CMDID_SCRIPTSITE_HTMLDLGTRUST: u32 = 1u32;
pub const CMDID_SCRIPTSITE_IURI: u32 = 7u32;
pub const CMDID_SCRIPTSITE_NAMESPACE: u32 = 6u32;
pub const CMDID_SCRIPTSITE_SECSTATE: u32 = 2u32;
pub const CMDID_SCRIPTSITE_SECURITY_WINDOW: u32 = 5u32;
pub const CMDID_SCRIPTSITE_SID: u32 = 3u32;
pub const CMDID_SCRIPTSITE_TRUSTEDDOC: u32 = 4u32;
pub const CMDID_SCRIPTSITE_URL: u32 = 0u32;
pub const CMD_ZOOM_FIT: i32 = -5i32;
pub const CMD_ZOOM_ONEPAGE: i32 = -2i32;
pub const CMD_ZOOM_PAGEWIDTH: i32 = -1i32;
pub const CMD_ZOOM_SELECTION: i32 = -4i32;
pub const CMD_ZOOM_TWOPAGES: i32 = -3i32;
pub const CMimeTypes: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611710, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const COLOR_NO_TRANSPARENT: u32 = 4294967295u32;
pub const CONTEXT_MENU_ANCHOR: u32 = 5u32;
pub const CONTEXT_MENU_CONTROL: u32 = 2u32;
pub const CONTEXT_MENU_DEBUG: u32 = 8u32;
pub const CONTEXT_MENU_DEFAULT: u32 = 0u32;
pub const CONTEXT_MENU_DISABLEDFLASH: u32 = 14u32;
pub const CONTEXT_MENU_ENTITY: u32 = 12u32;
pub const CONTEXT_MENU_HSCROLL: u32 = 10u32;
pub const CONTEXT_MENU_IMAGE: u32 = 1u32;
pub const CONTEXT_MENU_IMGDYNSRC: u32 = 7u32;
pub const CONTEXT_MENU_MEDIA: u32 = 11u32;
pub const CONTEXT_MENU_PDF: u32 = 13u32;
pub const CONTEXT_MENU_TABLE: u32 = 3u32;
pub const CONTEXT_MENU_TEXTSELECT: u32 = 4u32;
pub const CONTEXT_MENU_UNKNOWN: u32 = 6u32;
pub const CONTEXT_MENU_VSCROLL: u32 = 9u32;
pub const COOKIEACTION_ACCEPT: u32 = 1u32;
pub const COOKIEACTION_DOWNGRADE: u32 = 4u32;
pub const COOKIEACTION_LEASH: u32 = 8u32;
pub const COOKIEACTION_NONE: u32 = 0u32;
pub const COOKIEACTION_READ: u32 = 32u32;
pub const COOKIEACTION_REJECT: u32 = 2u32;
pub const COOKIEACTION_SUPPRESS: u32 = 16u32;
#[repr(transparent)]
pub struct COORD_SYSTEM(pub i32);
pub const COORD_SYSTEM_GLOBAL: COORD_SYSTEM = COORD_SYSTEM(0i32);
pub const COORD_SYSTEM_PARENT: COORD_SYSTEM = COORD_SYSTEM(1i32);
pub const COORD_SYSTEM_CONTAINER: COORD_SYSTEM = COORD_SYSTEM(2i32);
pub const COORD_SYSTEM_CONTENT: COORD_SYSTEM = COORD_SYSTEM(3i32);
pub const COORD_SYSTEM_FRAME: COORD_SYSTEM = COORD_SYSTEM(4i32);
pub const COORD_SYSTEM_CLIENT: COORD_SYSTEM = COORD_SYSTEM(5i32);
pub const COORD_SYSTEM_Max: COORD_SYSTEM = COORD_SYSTEM(2147483647i32);
impl ::core::marker::Copy for COORD_SYSTEM {}
impl ::core::clone::Clone for COORD_SYSTEM {
    fn clone(&self) -> Self {
        *self
    }
}
pub const COpsProfile: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611714, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CPersistDataPeer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611847, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CPersistHistory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611912, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CPersistShortcut: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611910, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CPersistSnapshot: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611913, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CPersistUserData: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611854, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CPlugins: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611711, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CPrintManagerTemplatePrinter: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1667342164,
    data2: 40305,
    data3: 19491,
    data4: [160, 141, 80, 215, 241, 141, 178, 233],
};
pub const CTemplatePrinter: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612403, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CanvasGradient: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616597, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CanvasImageData: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616603, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CanvasPattern: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616599, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CanvasRenderingContext2D: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616576, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CanvasTextMetrics: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616601, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CoDitherToRGB8: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2824916560, data2: 14608, data3: 4560, data4: [134, 252, 0, 160, 201, 19, 247, 80] };
pub const CoMapMIMEToCLSID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 818131072, data2: 12539, data3: 4560, data4: [183, 36, 0, 170, 0, 108, 26, 1] };
pub const CoSniffStream: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1778515360, data2: 12511, data3: 4560, data4: [183, 36, 0, 170, 0, 108, 26, 1] };
pub const DEBUGCALLBACKNOTIFICATION_ANIMATIONFRAME: u32 = 8u32;
pub const DEBUGCALLBACKNOTIFICATION_DOMEVENT: u32 = 16u32;
pub const DEBUGCALLBACKNOTIFICATION_IMMEDIATE: u32 = 4u32;
pub const DEBUGCALLBACKNOTIFICATION_INTERVAL: u32 = 2u32;
pub const DEBUGCALLBACKNOTIFICATION_TIMEOUT: u32 = 1u32;
pub const DEBUGDOMEVENTPROPAGATIONSTATUS_DEFAULTCANCELED: u32 = 1u32;
pub const DEBUGDOMEVENTPROPAGATIONSTATUS_STOPIMMEDIATEPROPAGATION: u32 = 2u32;
pub const DEBUGDOMEVENTPROPAGATIONSTATUS_STOPPROPAGATION: u32 = 4u32;
#[repr(transparent)]
pub struct DEV_CONSOLE_MESSAGE_LEVEL(pub i32);
pub const DCML_INFORMATIONAL: DEV_CONSOLE_MESSAGE_LEVEL = DEV_CONSOLE_MESSAGE_LEVEL(0i32);
pub const DCML_WARNING: DEV_CONSOLE_MESSAGE_LEVEL = DEV_CONSOLE_MESSAGE_LEVEL(1i32);
pub const DCML_ERROR: DEV_CONSOLE_MESSAGE_LEVEL = DEV_CONSOLE_MESSAGE_LEVEL(2i32);
pub const DEV_CONSOLE_MESSAGE_LEVEL_Max: DEV_CONSOLE_MESSAGE_LEVEL = DEV_CONSOLE_MESSAGE_LEVEL(2147483647i32);
impl ::core::marker::Copy for DEV_CONSOLE_MESSAGE_LEVEL {}
impl ::core::clone::Clone for DEV_CONSOLE_MESSAGE_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DISP10_IE10_XMSARIAFLOWFROM: u32 = 66835u32;
pub const DISPID_1D: u32 = 2000u32;
pub const DISPID_2D: u32 = 1000u32;
pub const DISPID_AAHEADER: u32 = 70793u32;
pub const DISPID_ACTIVEXFILTERINGENABLED: u32 = 61u32;
pub const DISPID_ACTIVEX_EXPANDO_BASE: u32 = 72536u32;
pub const DISPID_ACTIVEX_EXPANDO_MAX: u32 = 73535u32;
pub const DISPID_ADDCHANNEL: u32 = 5u32;
pub const DISPID_ADDDESKTOPCOMPONENT: u32 = 6u32;
pub const DISPID_ADDFAVORITE: u32 = 4u32;
pub const DISPID_ADDSEARCHPROVIDER: u32 = 14u32;
pub const DISPID_ADDSERVICE: u32 = 30u32;
pub const DISPID_ADDSITEMODE: u32 = 49u32;
pub const DISPID_ADDTHUMBNAILBUTTONS: u32 = 48u32;
pub const DISPID_ADDTOFAVORITESBAR: u32 = 32u32;
pub const DISPID_ADDTRACKINGPROTECTIONLIST: u32 = 57u32;
pub const DISPID_ADVANCEERROR: u32 = 10u32;
pub const DISPID_ADVISEDATASRCCHANGEEVENT: i32 = -3901i32;
pub const DISPID_AMBIENT_DLCONTROL: i32 = -5512i32;
pub const DISPID_AMBIENT_OFFLINEIFNOTCONNECTED: i32 = -5501i32;
pub const DISPID_AMBIENT_SILENT: i32 = -5502i32;
pub const DISPID_AMBIENT_USERAGENT: i32 = -5513i32;
pub const DISPID_ANCHOR: u32 = 1000u32;
pub const DISPID_APPLICATIONCACHE: u32 = 1000u32;
pub const DISPID_AREA: u32 = 1000u32;
pub const DISPID_ATTRS: u32 = 70536u32;
pub const DISPID_AUDIO: u32 = 1050u32;
pub const DISPID_AUTOCOMPLETEATTACH: u32 = 12u32;
pub const DISPID_AUTOCOMPLETESAVEFORM: u32 = 10u32;
pub const DISPID_AUTOSCAN: u32 = 11u32;
pub const DISPID_A_ACCELERATOR: u32 = 70683u32;
pub const DISPID_A_ACCEVENTRECORDID_END: u32 = 71209u32;
pub const DISPID_A_ACCEVENTRECORDID_START: u32 = 71190u32;
pub const DISPID_A_ALIGNCONTENT: u32 = 71009u32;
pub const DISPID_A_ALIGNITEMS: u32 = 71007u32;
pub const DISPID_A_ALIGNMENTBASELINE: u32 = 70814u32;
pub const DISPID_A_ALIGNSELF: u32 = 71008u32;
pub const DISPID_A_ALLOWTRANSPARENCY: u32 = 70742u32;
pub const DISPID_A_ANIMATION: u32 = 70985u32;
pub const DISPID_A_ANIMATIONDELAY: u32 = 70981u32;
pub const DISPID_A_ANIMATIONDIRECTION: u32 = 70982u32;
pub const DISPID_A_ANIMATIONDURATION: u32 = 70979u32;
pub const DISPID_A_ANIMATIONFILLMODE: u32 = 70986u32;
pub const DISPID_A_ANIMATIONITERATIONCOUNT: u32 = 70984u32;
pub const DISPID_A_ANIMATIONNAME: u32 = 70978u32;
pub const DISPID_A_ANIMATIONPLAYSTATE: u32 = 70983u32;
pub const DISPID_A_ANIMATIONTIMINGFUNCTION: u32 = 70980u32;
pub const DISPID_A_BACKFACEVISIBILITY: u32 = 70977u32;
pub const DISPID_A_BACKGROUND: u32 = 70568u32;
pub const DISPID_A_BACKGROUNDATTACHMENT: u32 = 70581u32;
pub const DISPID_A_BACKGROUNDIMAGE: u32 = 70537u32;
pub const DISPID_A_BACKGROUNDPOSITION: u32 = 70582u32;
pub const DISPID_A_BACKGROUNDPOSX: u32 = 70569u32;
pub const DISPID_A_BACKGROUNDPOSY: u32 = 70570u32;
pub const DISPID_A_BACKGROUNDREPEAT: u32 = 70580u32;
pub const DISPID_A_BASEFONT: u32 = 70562u32;
pub const DISPID_A_BASELINESHIFT: u32 = 70815u32;
pub const DISPID_A_BDURLIMGCTXCACHEINDEX: u32 = 71214u32;
pub const DISPID_A_BEHAVIOR: u32 = 70651u32;
pub const DISPID_A_BGURLIMGCTXCACHEINDEX: u32 = 70646u32;
pub const DISPID_A_BGURLIMGCTXCACHEINDEX_FLETTER: u32 = 70738u32;
pub const DISPID_A_BGURLIMGCTXCACHEINDEX_FLINE: u32 = 70737u32;
pub const DISPID_A_BORDER: u32 = 70585u32;
pub const DISPID_A_BORDERBOTTOM: u32 = 70588u32;
pub const DISPID_A_BORDERBOTTOMCOLOR: u32 = 70593u32;
pub const DISPID_A_BORDERBOTTOMLEFTRADIUS: u32 = 70850u32;
pub const DISPID_A_BORDERBOTTOMRIGHTRADIUS: u32 = 70849u32;
pub const DISPID_A_BORDERBOTTOMSTYLE: u32 = 70603u32;
pub const DISPID_A_BORDERBOTTOMWIDTH: u32 = 70598u32;
pub const DISPID_A_BORDERCOLLAPSE: u32 = 70620u32;
pub const DISPID_A_BORDERCOLOR: u32 = 70590u32;
pub const DISPID_A_BORDERIMAGE: u32 = 71010u32;
pub const DISPID_A_BORDERIMAGEOUTSET: u32 = 71014u32;
pub const DISPID_A_BORDERIMAGEREPEAT: u32 = 71015u32;
pub const DISPID_A_BORDERIMAGESLICE: u32 = 71012u32;
pub const DISPID_A_BORDERIMAGESOURCE: u32 = 71011u32;
pub const DISPID_A_BORDERIMAGEWIDTH: u32 = 71013u32;
pub const DISPID_A_BORDERLEFT: u32 = 70589u32;
pub const DISPID_A_BORDERLEFTCOLOR: u32 = 70594u32;
pub const DISPID_A_BORDERLEFTSTYLE: u32 = 70604u32;
pub const DISPID_A_BORDERLEFTWIDTH: u32 = 70599u32;
pub const DISPID_A_BORDERRADIUS: u32 = 70846u32;
pub const DISPID_A_BORDERRIGHT: u32 = 70587u32;
pub const DISPID_A_BORDERRIGHTCOLOR: u32 = 70592u32;
pub const DISPID_A_BORDERRIGHTSTYLE: u32 = 70602u32;
pub const DISPID_A_BORDERRIGHTWIDTH: u32 = 70597u32;
pub const DISPID_A_BORDERSPACING: u32 = 70763u32;
pub const DISPID_A_BORDERSTYLE: u32 = 70600u32;
pub const DISPID_A_BORDERTOP: u32 = 70586u32;
pub const DISPID_A_BORDERTOPCOLOR: u32 = 70591u32;
pub const DISPID_A_BORDERTOPLEFTRADIUS: u32 = 70847u32;
pub const DISPID_A_BORDERTOPRIGHTRADIUS: u32 = 70848u32;
pub const DISPID_A_BORDERTOPSTYLE: u32 = 70601u32;
pub const DISPID_A_BORDERTOPWIDTH: u32 = 70596u32;
pub const DISPID_A_BORDERWIDTH: u32 = 70595u32;
pub const DISPID_A_BOXSIZING: u32 = 70762u32;
pub const DISPID_A_BREAKAFTER: u32 = 70882u32;
pub const DISPID_A_BREAKBEFORE: u32 = 70881u32;
pub const DISPID_A_BREAKINSIDE: u32 = 70883u32;
pub const DISPID_A_CAPTIONSIDE: u32 = 70755u32;
pub const DISPID_A_CLASSLIST: u32 = 70953u32;
pub const DISPID_A_CLEAR: u32 = 70552u32;
pub const DISPID_A_CLIP: u32 = 70628u32;
pub const DISPID_A_CLIPPATH: u32 = 70820u32;
pub const DISPID_A_CLIPRECTBOTTOM: u32 = 70631u32;
pub const DISPID_A_CLIPRECTLEFT: u32 = 70632u32;
pub const DISPID_A_CLIPRECTRIGHT: u32 = 70630u32;
pub const DISPID_A_CLIPRECTTOP: u32 = 70629u32;
pub const DISPID_A_CLIPRULE: u32 = 70821u32;
pub const DISPID_A_COLOR: u32 = 70538u32;
pub const DISPID_A_COLORINTERPOLATIONFILTERS: u32 = 70928u32;
pub const DISPID_A_COLUMNCOUNT: u32 = 70872u32;
pub const DISPID_A_COLUMNFILL: u32 = 70875u32;
pub const DISPID_A_COLUMNGAP: u32 = 70874u32;
pub const DISPID_A_COLUMNRULE: u32 = 70877u32;
pub const DISPID_A_COLUMNRULECOLOR: u32 = 70880u32;
pub const DISPID_A_COLUMNRULESTYLE: u32 = 70878u32;
pub const DISPID_A_COLUMNRULEWIDTH: u32 = 70879u32;
pub const DISPID_A_COLUMNS: u32 = 70871u32;
pub const DISPID_A_COLUMNSPAN: u32 = 70876u32;
pub const DISPID_A_COLUMNWIDTH: u32 = 70873u32;
pub const DISPID_A_CONTENT: u32 = 70754u32;
pub const DISPID_A_COUNTERINCREMENT: u32 = 70756u32;
pub const DISPID_A_COUNTERRESET: u32 = 70757u32;
pub const DISPID_A_CSSFLOAT: u32 = 70845u32;
pub const DISPID_A_CURSOR: u32 = 70638u32;
pub const DISPID_A_DATASET: u32 = 71016u32;
pub const DISPID_A_DEFAULTTEXTSELECTION: u32 = 70724u32;
pub const DISPID_A_DIR: u32 = 70653u32;
pub const DISPID_A_DIRECTION: u32 = 70655u32;
pub const DISPID_A_DISPLAY: u32 = 70607u32;
pub const DISPID_A_DOCFRAGMENT: u32 = 70678u32;
pub const DISPID_A_DOMINANTBASELINE: u32 = 70816u32;
pub const DISPID_A_DRAGGABLE: u32 = 70944u32;
pub const DISPID_A_EDITABLE: u32 = 70698u32;
pub const DISPID_A_EMPTYCELLS: u32 = 70786u32;
pub const DISPID_A_ENABLEBACKGROUND: u32 = 70946u32;
pub const DISPID_A_EVENTSINK: u32 = 70639u32;
pub const DISPID_A_EXTENDEDTAGDESC: u32 = 70687u32;
pub const DISPID_A_FILL: u32 = 70822u32;
pub const DISPID_A_FILLOPACITY: u32 = 70823u32;
pub const DISPID_A_FILLRULE: u32 = 70824u32;
pub const DISPID_A_FILTER: u32 = 70618u32;
pub const DISPID_A_FIRST: u32 = 70536u32;
pub const DISPID_A_FLEX: u32 = 71002u32;
pub const DISPID_A_FLEXBASIS: u32 = 71005u32;
pub const DISPID_A_FLEXDIRECTION: u32 = 70998u32;
pub const DISPID_A_FLEXFLOW: u32 = 71000u32;
pub const DISPID_A_FLEXGROW: u32 = 71003u32;
pub const DISPID_A_FLEXSHRINK: u32 = 71004u32;
pub const DISPID_A_FLEXWRAP: u32 = 70999u32;
pub const DISPID_A_FLOAT: u32 = 70606u32;
pub const DISPID_A_FLOODCOLOR: u32 = 70926u32;
pub const DISPID_A_FLOODOPACITY: u32 = 70927u32;
pub const DISPID_A_FONT: u32 = 70577u32;
pub const DISPID_A_FONTFACE: u32 = 70554u32;
pub const DISPID_A_FONTFACESRC: u32 = 70633u32;
pub const DISPID_A_FONTFEATURESETTINGS: u32 = 70987u32;
pub const DISPID_A_FONTSIZE: u32 = 70555u32;
pub const DISPID_A_FONTSIZEADJUST: u32 = 70817u32;
pub const DISPID_A_FONTSIZECOMBINE: u32 = 70579u32;
pub const DISPID_A_FONTSIZEKEYWORD: u32 = 70578u32;
pub const DISPID_A_FONTSTRETCH: u32 = 70818u32;
pub const DISPID_A_FONTSTYLE: u32 = 70560u32;
pub const DISPID_A_FONTVARIANT: u32 = 70561u32;
pub const DISPID_A_FONTWEIGHT: u32 = 70563u32;
pub const DISPID_A_FROZEN: u32 = 70734u32;
pub const DISPID_A_GLYPHORIENTATIONHORIZONTAL: u32 = 70843u32;
pub const DISPID_A_GLYPHORIENTATIONVERTICAL: u32 = 70844u32;
pub const DISPID_A_HASLAYOUT: u32 = 70696u32;
pub const DISPID_A_HIDDEN: u32 = 70617u32;
pub const DISPID_A_HIDEFOCUS: u32 = 70699u32;
pub const DISPID_A_HTCDD_CREATEEVENTOBJECT: u32 = 70680u32;
pub const DISPID_A_HTCDD_DEFAULTS: u32 = 70701u32;
pub const DISPID_A_HTCDD_ELEMENT: u32 = 70679u32;
pub const DISPID_A_HTCDD_ISMARKUPSHARED: u32 = 70693u32;
pub const DISPID_A_HTCDD_PROTECTEDELEMENT: u32 = 70690u32;
pub const DISPID_A_HTCDISPATCHITEM_VALUE: u32 = 70677u32;
pub const DISPID_A_HTCDISPATCHITEM_VALUE_SCRIPTSONLY: u32 = 70686u32;
pub const DISPID_A_IE9_BACKGROUNDCLIP: u32 = 70852u32;
pub const DISPID_A_IE9_BACKGROUNDORIGIN: u32 = 70853u32;
pub const DISPID_A_IE9_BACKGROUNDSIZE: u32 = 70854u32;
pub const DISPID_A_IE9_BOXSHADOW: u32 = 70855u32;
pub const DISPID_A_IMEMODE: u32 = 70656u32;
pub const DISPID_A_INTERPOLATION: u32 = 70749u32;
pub const DISPID_A_ISBLOCK: u32 = 70744u32;
pub const DISPID_A_JUSTIFYCONTENT: u32 = 71006u32;
pub const DISPID_A_KERNING: u32 = 70825u32;
pub const DISPID_A_LANG: u32 = 70545u32;
pub const DISPID_A_LANGUAGE: u32 = 70636u32;
pub const DISPID_A_LAYOUTFLOW: u32 = 70691u32;
pub const DISPID_A_LAYOUTGRID: u32 = 70667u32;
pub const DISPID_A_LAYOUTGRIDCHAR: u32 = 70663u32;
pub const DISPID_A_LAYOUTGRIDLINE: u32 = 70664u32;
pub const DISPID_A_LAYOUTGRIDMODE: u32 = 70665u32;
pub const DISPID_A_LAYOUTGRIDTYPE: u32 = 70666u32;
pub const DISPID_A_LETTERSPACING: u32 = 70544u32;
pub const DISPID_A_LIGHTINGCOLOR: u32 = 70929u32;
pub const DISPID_A_LINEBREAK: u32 = 70669u32;
pub const DISPID_A_LINEHEIGHT: u32 = 70542u32;
pub const DISPID_A_LISTSTYLE: u32 = 70611u32;
pub const DISPID_A_LISTSTYLEIMAGE: u32 = 70610u32;
pub const DISPID_A_LISTSTYLEPOSITION: u32 = 70609u32;
pub const DISPID_A_LISTSTYLETYPE: u32 = 70608u32;
pub const DISPID_A_LISTTYPE: u32 = 70553u32;
pub const DISPID_A_LIURLIMGCTXCACHEINDEX: u32 = 70647u32;
pub const DISPID_A_MARGIN: u32 = 70572u32;
pub const DISPID_A_MARGINBOTTOM: u32 = 70575u32;
pub const DISPID_A_MARGINLEFT: u32 = 70576u32;
pub const DISPID_A_MARGINRIGHT: u32 = 70574u32;
pub const DISPID_A_MARGINTOP: u32 = 70573u32;
pub const DISPID_A_MARKER: u32 = 70826u32;
pub const DISPID_A_MARKEREND: u32 = 70827u32;
pub const DISPID_A_MARKERMID: u32 = 70828u32;
pub const DISPID_A_MARKERSTART: u32 = 70829u32;
pub const DISPID_A_MASK: u32 = 70830u32;
pub const DISPID_A_MAX: u32 = 71535u32;
pub const DISPID_A_MAXHEIGHT: u32 = 70750u32;
pub const DISPID_A_MAXWIDTH: u32 = 70752u32;
pub const DISPID_A_MEDIA: u32 = 70697u32;
pub const DISPID_A_MEDIAASPECTRATIO: u32 = 71153u32;
pub const DISPID_A_MEDIACOLOR: u32 = 71159u32;
pub const DISPID_A_MEDIACOLORINDEX: u32 = 71162u32;
pub const DISPID_A_MEDIADEVICEASPECTRATIO: u32 = 71156u32;
pub const DISPID_A_MEDIADEVICEHEIGHT: u32 = 71150u32;
pub const DISPID_A_MEDIADEVICEWIDTH: u32 = 71147u32;
pub const DISPID_A_MEDIAGRID: u32 = 71188u32;
pub const DISPID_A_MEDIAHEIGHT: u32 = 71144u32;
pub const DISPID_A_MEDIAMAXASPECTRATIO: u32 = 71151u32;
pub const DISPID_A_MEDIAMAXCOLOR: u32 = 71157u32;
pub const DISPID_A_MEDIAMAXCOLORINDEX: u32 = 71160u32;
pub const DISPID_A_MEDIAMAXDEVICEASPECTRATIO: u32 = 71154u32;
pub const DISPID_A_MEDIAMAXDEVICEHEIGHT: u32 = 71148u32;
pub const DISPID_A_MEDIAMAXDEVICEWIDTH: u32 = 71145u32;
pub const DISPID_A_MEDIAMAXHEIGHT: u32 = 71142u32;
pub const DISPID_A_MEDIAMAXMONOCHROME: u32 = 71163u32;
pub const DISPID_A_MEDIAMAXRESOLUTION: u32 = 71166u32;
pub const DISPID_A_MEDIAMAXWIDTH: u32 = 71139u32;
pub const DISPID_A_MEDIAMINASPECTRATIO: u32 = 71152u32;
pub const DISPID_A_MEDIAMINCOLOR: u32 = 71158u32;
pub const DISPID_A_MEDIAMINCOLORINDEX: u32 = 71161u32;
pub const DISPID_A_MEDIAMINDEVICEASPECTRATIO: u32 = 71155u32;
pub const DISPID_A_MEDIAMINDEVICEHEIGHT: u32 = 71149u32;
pub const DISPID_A_MEDIAMINDEVICEWIDTH: u32 = 71146u32;
pub const DISPID_A_MEDIAMINHEIGHT: u32 = 71143u32;
pub const DISPID_A_MEDIAMINMONOCHROME: u32 = 71164u32;
pub const DISPID_A_MEDIAMINRESOLUTION: u32 = 71167u32;
pub const DISPID_A_MEDIAMINWIDTH: u32 = 71140u32;
pub const DISPID_A_MEDIAMONOCHROME: u32 = 71165u32;
pub const DISPID_A_MEDIAMSHIGHCONTRAST: u32 = 71174u32;
pub const DISPID_A_MEDIAMSVIEWSTATE: u32 = 71178u32;
pub const DISPID_A_MEDIAORIENTATION: u32 = 71138u32;
pub const DISPID_A_MEDIARESOLUTION: u32 = 71168u32;
pub const DISPID_A_MEDIASCAN: u32 = 71189u32;
pub const DISPID_A_MEDIAWEBKITDEVICEPIXELRATIO: u32 = 71215u32;
pub const DISPID_A_MEDIAWEBKITMAXDEVICEPIXELRATIO: u32 = 71216u32;
pub const DISPID_A_MEDIAWEBKITMINDEVICEPIXELRATIO: u32 = 71217u32;
pub const DISPID_A_MEDIAWIDTH: u32 = 71141u32;
pub const DISPID_A_MIN: u32 = 70536u32;
pub const DISPID_A_MINHEIGHT: u32 = 70747u32;
pub const DISPID_A_MINWIDTH: u32 = 70751u32;
pub const DISPID_A_MS_ACCELERATOR: u32 = 70783u32;
pub const DISPID_A_MS_ANIMATION: u32 = 70924u32;
pub const DISPID_A_MS_ANIMATIONDELAY: u32 = 70920u32;
pub const DISPID_A_MS_ANIMATIONDIRECTION: u32 = 70921u32;
pub const DISPID_A_MS_ANIMATIONDURATION: u32 = 70918u32;
pub const DISPID_A_MS_ANIMATIONFILLMODE: u32 = 70925u32;
pub const DISPID_A_MS_ANIMATIONITERATIONCOUNT: u32 = 70923u32;
pub const DISPID_A_MS_ANIMATIONNAME: u32 = 70917u32;
pub const DISPID_A_MS_ANIMATIONPLAYSTATE: u32 = 70922u32;
pub const DISPID_A_MS_ANIMATIONTIMINGFUNCTION: u32 = 70919u32;
pub const DISPID_A_MS_BACKFACEVISIBILITY: u32 = 70890u32;
pub const DISPID_A_MS_BACKGROUNDPOSX: u32 = 70781u32;
pub const DISPID_A_MS_BACKGROUNDPOSY: u32 = 70782u32;
pub const DISPID_A_MS_BEHAVIOR: u32 = 70767u32;
pub const DISPID_A_MS_BLOCKPROGRESSION: u32 = 70787u32;
pub const DISPID_A_MS_CONTENTZOOMCHAINING: u32 = 70895u32;
pub const DISPID_A_MS_CONTENTZOOMFACTOR: u32 = 70900u32;
pub const DISPID_A_MS_CONTENTZOOMING: u32 = 70892u32;
pub const DISPID_A_MS_CONTENTZOOMLIMIT: u32 = 70897u32;
pub const DISPID_A_MS_CONTENTZOOMLIMITMAX: u32 = 70902u32;
pub const DISPID_A_MS_CONTENTZOOMLIMITMIN: u32 = 70901u32;
pub const DISPID_A_MS_CONTENTZOOMSNAP: u32 = 70898u32;
pub const DISPID_A_MS_CONTENTZOOMSNAPPOINTS: u32 = 70899u32;
pub const DISPID_A_MS_CONTENTZOOMSNAPTYPE: u32 = 70893u32;
pub const DISPID_A_MS_FILTER: u32 = 70801u32;
pub const DISPID_A_MS_FLEX: u32 = 70955u32;
pub const DISPID_A_MS_FLEXALIGN: u32 = 70962u32;
pub const DISPID_A_MS_FLEXDIRECTION: u32 = 70960u32;
pub const DISPID_A_MS_FLEXFLOW: u32 = 70959u32;
pub const DISPID_A_MS_FLEXITEMALIGN: u32 = 70963u32;
pub const DISPID_A_MS_FLEXLINEPACK: u32 = 70965u32;
pub const DISPID_A_MS_FLEXNEGATIVE: u32 = 70957u32;
pub const DISPID_A_MS_FLEXORDER: u32 = 70966u32;
pub const DISPID_A_MS_FLEXPACK: u32 = 70964u32;
pub const DISPID_A_MS_FLEXPOSITIVE: u32 = 70956u32;
pub const DISPID_A_MS_FLEXPREFERREDSIZE: u32 = 70958u32;
pub const DISPID_A_MS_FLEXWRAP: u32 = 70961u32;
pub const DISPID_A_MS_FLOWFROM: u32 = 70938u32;
pub const DISPID_A_MS_FLOWINTO: u32 = 70939u32;
pub const DISPID_A_MS_FONTFEATURESETTINGS: u32 = 70950u32;
pub const DISPID_A_MS_GRIDCOLUMN: u32 = 70908u32;
pub const DISPID_A_MS_GRIDCOLUMNALIGN: u32 = 70909u32;
pub const DISPID_A_MS_GRIDCOLUMNS: u32 = 70910u32;
pub const DISPID_A_MS_GRIDCOLUMNSPAN: u32 = 70911u32;
pub const DISPID_A_MS_GRIDROW: u32 = 70913u32;
pub const DISPID_A_MS_GRIDROWALIGN: u32 = 70914u32;
pub const DISPID_A_MS_GRIDROWS: u32 = 70915u32;
pub const DISPID_A_MS_GRIDROWSPAN: u32 = 70916u32;
pub const DISPID_A_MS_HIGHCONTRASTADJUST: u32 = 70945u32;
pub const DISPID_A_MS_HYPHENATE_LIMIT_CHARS: u32 = 70942u32;
pub const DISPID_A_MS_HYPHENATE_LIMIT_LINES: u32 = 70943u32;
pub const DISPID_A_MS_HYPHENATE_LIMIT_ZONE: u32 = 70941u32;
pub const DISPID_A_MS_HYPHENS: u32 = 70940u32;
pub const DISPID_A_MS_IMEALIGN: u32 = 71017u32;
pub const DISPID_A_MS_IMEMODE: u32 = 70780u32;
pub const DISPID_A_MS_LAYOUTFLOW: u32 = 70784u32;
pub const DISPID_A_MS_LAYOUTGRID: u32 = 70799u32;
pub const DISPID_A_MS_LAYOUTGRIDCHAR: u32 = 70795u32;
pub const DISPID_A_MS_LAYOUTGRIDLINE: u32 = 70796u32;
pub const DISPID_A_MS_LAYOUTGRIDMODE: u32 = 70797u32;
pub const DISPID_A_MS_LAYOUTGRIDTYPE: u32 = 70798u32;
pub const DISPID_A_MS_LINEBREAK: u32 = 70800u32;
pub const DISPID_A_MS_OVERFLOWSTYLE: u32 = 70935u32;
pub const DISPID_A_MS_OVERFLOWX: u32 = 70802u32;
pub const DISPID_A_MS_OVERFLOWY: u32 = 70803u32;
pub const DISPID_A_MS_PERSPECTIVE: u32 = 70885u32;
pub const DISPID_A_MS_PERSPECTIVEORIGIN: u32 = 70886u32;
pub const DISPID_A_MS_PERSPECTIVEORIGINX: u32 = 70887u32;
pub const DISPID_A_MS_PERSPECTIVEORIGINY: u32 = 70888u32;
pub const DISPID_A_MS_SCROLLBAR3DLIGHTCOLOR: u32 = 70770u32;
pub const DISPID_A_MS_SCROLLBARARROWCOLOR: u32 = 70774u32;
pub const DISPID_A_MS_SCROLLBARBASECOLOR: u32 = 70768u32;
pub const DISPID_A_MS_SCROLLBARDARKSHADOWCOLOR: u32 = 70773u32;
pub const DISPID_A_MS_SCROLLBARFACECOLOR: u32 = 70769u32;
pub const DISPID_A_MS_SCROLLBARHIGHLIGHTCOLOR: u32 = 70772u32;
pub const DISPID_A_MS_SCROLLBARSHADOWCOLOR: u32 = 70771u32;
pub const DISPID_A_MS_SCROLLBARTRACKCOLOR: u32 = 70775u32;
pub const DISPID_A_MS_SCROLLCHAINING: u32 = 70891u32;
pub const DISPID_A_MS_SCROLLLIMIT: u32 = 70934u32;
pub const DISPID_A_MS_SCROLLLIMITXMAX: u32 = 70932u32;
pub const DISPID_A_MS_SCROLLLIMITXMIN: u32 = 70930u32;
pub const DISPID_A_MS_SCROLLLIMITYMAX: u32 = 70933u32;
pub const DISPID_A_MS_SCROLLLIMITYMIN: u32 = 70931u32;
pub const DISPID_A_MS_SCROLLRAILS: u32 = 70894u32;
pub const DISPID_A_MS_SCROLLSNAPPOINTSX: u32 = 70905u32;
pub const DISPID_A_MS_SCROLLSNAPPOINTSY: u32 = 70906u32;
pub const DISPID_A_MS_SCROLLSNAPTYPE: u32 = 70896u32;
pub const DISPID_A_MS_SCROLLSNAPX: u32 = 70903u32;
pub const DISPID_A_MS_SCROLLSNAPY: u32 = 70904u32;
pub const DISPID_A_MS_SCROLLTRANSLATION: u32 = 70954u32;
pub const DISPID_A_MS_TEXTALIGNLAST: u32 = 70776u32;
pub const DISPID_A_MS_TEXTAUTOSPACE: u32 = 70804u32;
pub const DISPID_A_MS_TEXTCOMBINEHORIZONTAL: u32 = 71018u32;
pub const DISPID_A_MS_TEXTJUSTIFY: u32 = 70805u32;
pub const DISPID_A_MS_TEXTKASHIDASPACE: u32 = 70806u32;
pub const DISPID_A_MS_TEXTOVERFLOW: u32 = 70777u32;
pub const DISPID_A_MS_TEXTSIZEADJUST: u32 = 70864u32;
pub const DISPID_A_MS_TEXTUNDERLINEPOSITION: u32 = 70778u32;
pub const DISPID_A_MS_TOUCHACTION: u32 = 70952u32;
pub const DISPID_A_MS_TOUCHSELECT: u32 = 70994u32;
pub const DISPID_A_MS_TRANSFORM: u32 = 70851u32;
pub const DISPID_A_MS_TRANSFORMORIGIN: u32 = 70861u32;
pub const DISPID_A_MS_TRANSFORMORIGINX: u32 = 70862u32;
pub const DISPID_A_MS_TRANSFORMORIGINY: u32 = 70863u32;
pub const DISPID_A_MS_TRANSFORMORIGINZ: u32 = 70884u32;
pub const DISPID_A_MS_TRANSFORMSTYLE: u32 = 70889u32;
pub const DISPID_A_MS_TRANSITION: u32 = 70870u32;
pub const DISPID_A_MS_TRANSITIONDELAY: u32 = 70869u32;
pub const DISPID_A_MS_TRANSITIONDURATION: u32 = 70867u32;
pub const DISPID_A_MS_TRANSITIONPROPERTY: u32 = 70866u32;
pub const DISPID_A_MS_TRANSITIONTIMINGFUNCTION: u32 = 70868u32;
pub const DISPID_A_MS_USERSELECT: u32 = 70951u32;
pub const DISPID_A_MS_WORDBREAK: u32 = 70807u32;
pub const DISPID_A_MS_WORDWRAP: u32 = 70808u32;
pub const DISPID_A_MS_WRAPFLOW: u32 = 70949u32;
pub const DISPID_A_MS_WRAPMARGIN: u32 = 70947u32;
pub const DISPID_A_MS_WRAPTHROUGH: u32 = 70937u32;
pub const DISPID_A_MS_WRITINGMODE: u32 = 70779u32;
pub const DISPID_A_MS_ZOOM: u32 = 70785u32;
pub const DISPID_A_NOWRAP: u32 = 70541u32;
pub const DISPID_A_OPACITY: u32 = 70819u32;
pub const DISPID_A_ORDER: u32 = 71001u32;
pub const DISPID_A_ORPHANS: u32 = 70764u32;
pub const DISPID_A_OUTLINE: u32 = 70758u32;
pub const DISPID_A_OUTLINECOLOR: u32 = 70761u32;
pub const DISPID_A_OUTLINESTYLE: u32 = 70760u32;
pub const DISPID_A_OUTLINEWIDTH: u32 = 70759u32;
pub const DISPID_A_OVERFLOW: u32 = 70546u32;
pub const DISPID_A_OVERFLOWX: u32 = 70675u32;
pub const DISPID_A_OVERFLOWY: u32 = 70676u32;
pub const DISPID_A_PADDING: u32 = 70547u32;
pub const DISPID_A_PADDINGBOTTOM: u32 = 70550u32;
pub const DISPID_A_PADDINGLEFT: u32 = 70551u32;
pub const DISPID_A_PADDINGRIGHT: u32 = 70549u32;
pub const DISPID_A_PADDINGTOP: u32 = 70548u32;
pub const DISPID_A_PAGEBREAKAFTER: u32 = 70614u32;
pub const DISPID_A_PAGEBREAKBEFORE: u32 = 70613u32;
pub const DISPID_A_PAGEBREAKINSIDE: u32 = 70766u32;
pub const DISPID_A_PERSPECTIVE: u32 = 70974u32;
pub const DISPID_A_PERSPECTIVEORIGIN: u32 = 70975u32;
pub const DISPID_A_PERSPECTIVEORIGINX: u32 = 70992u32;
pub const DISPID_A_PERSPECTIVEORIGINY: u32 = 70993u32;
pub const DISPID_A_POINTEREVENTS: u32 = 70831u32;
pub const DISPID_A_POSITION: u32 = 70626u32;
pub const DISPID_A_PROPNOTIFYSINK: u32 = 70640u32;
pub const DISPID_A_QUOTES: u32 = 70788u32;
pub const DISPID_A_READYSTATE: u32 = 70652u32;
pub const DISPID_A_RENDERINGPRIORITY: u32 = 70706u32;
pub const DISPID_A_ROTATE: u32 = 70688u32;
pub const DISPID_A_ROWPOSITIONCHANGESINK: u32 = 70650u32;
pub const DISPID_A_ROWSETASYNCHNOTIFYSINK: u32 = 70648u32;
pub const DISPID_A_ROWSETNOTIFYSINK: u32 = 70641u32;
pub const DISPID_A_RUBYALIGN: u32 = 70657u32;
pub const DISPID_A_RUBYOVERHANG: u32 = 70659u32;
pub const DISPID_A_RUBYPOSITION: u32 = 70658u32;
pub const DISPID_A_SCROLL: u32 = 70615u32;
pub const DISPID_A_SCROLLBAR3DLIGHTCOLOR: u32 = 70718u32;
pub const DISPID_A_SCROLLBARARROWCOLOR: u32 = 70722u32;
pub const DISPID_A_SCROLLBARBASECOLOR: u32 = 70716u32;
pub const DISPID_A_SCROLLBARDARKSHADOWCOLOR: u32 = 70721u32;
pub const DISPID_A_SCROLLBARFACECOLOR: u32 = 70717u32;
pub const DISPID_A_SCROLLBARHIGHLIGHTCOLOR: u32 = 70720u32;
pub const DISPID_A_SCROLLBARSHADOWCOLOR: u32 = 70719u32;
pub const DISPID_A_SCROLLBARTRACKCOLOR: u32 = 70732u32;
pub const DISPID_A_SPELLCHECK: u32 = 70907u32;
pub const DISPID_A_STOPCOLOR: u32 = 70832u32;
pub const DISPID_A_STOPOPACITY: u32 = 70833u32;
pub const DISPID_A_STROKE: u32 = 70834u32;
pub const DISPID_A_STROKEDASHARRAY: u32 = 70835u32;
pub const DISPID_A_STROKEDASHOFFSET: u32 = 70836u32;
pub const DISPID_A_STROKELINECAP: u32 = 70837u32;
pub const DISPID_A_STROKELINEJOIN: u32 = 70838u32;
pub const DISPID_A_STROKEMITERLIMIT: u32 = 70839u32;
pub const DISPID_A_STROKEOPACITY: u32 = 70840u32;
pub const DISPID_A_STROKEWIDTH: u32 = 70841u32;
pub const DISPID_A_STYLETEXT: u32 = 70635u32;
pub const DISPID_A_STYLETEXTDECORATION: u32 = 70727u32;
pub const DISPID_A_TABLEBORDERCOLOR: u32 = 70564u32;
pub const DISPID_A_TABLEBORDERCOLORDARK: u32 = 70566u32;
pub const DISPID_A_TABLEBORDERCOLORLIGHT: u32 = 70565u32;
pub const DISPID_A_TABLELAYOUT: u32 = 70634u32;
pub const DISPID_A_TABLEVALIGN: u32 = 70567u32;
pub const DISPID_A_TEXTALIGNLAST: u32 = 70739u32;
pub const DISPID_A_TEXTANCHOR: u32 = 70842u32;
pub const DISPID_A_TEXTAUTOSPACE: u32 = 70668u32;
pub const DISPID_A_TEXTBACKGROUNDCOLOR: u32 = 70705u32;
pub const DISPID_A_TEXTCOLOR: u32 = 70726u32;
pub const DISPID_A_TEXTDECORATION: u32 = 70571u32;
pub const DISPID_A_TEXTDECORATIONBLINK: u32 = 70558u32;
pub const DISPID_A_TEXTDECORATIONCOLOR: u32 = 70725u32;
pub const DISPID_A_TEXTDECORATIONLINETHROUGH: u32 = 70556u32;
pub const DISPID_A_TEXTDECORATIONNONE: u32 = 70559u32;
pub const DISPID_A_TEXTDECORATIONOVERLINE: u32 = 70605u32;
pub const DISPID_A_TEXTDECORATIONUNDERLINE: u32 = 70557u32;
pub const DISPID_A_TEXTEFFECT: u32 = 70704u32;
pub const DISPID_A_TEXTINDENT: u32 = 70543u32;
pub const DISPID_A_TEXTJUSTIFY: u32 = 70671u32;
pub const DISPID_A_TEXTJUSTIFYTRIM: u32 = 70672u32;
pub const DISPID_A_TEXTKASHIDA: u32 = 70673u32;
pub const DISPID_A_TEXTKASHIDASPACE: u32 = 70740u32;
pub const DISPID_A_TEXTLINETHROUGHSTYLE: u32 = 70702u32;
pub const DISPID_A_TEXTOVERFLOW: u32 = 70745u32;
pub const DISPID_A_TEXTSHADOW: u32 = 70936u32;
pub const DISPID_A_TEXTTRANSFORM: u32 = 70540u32;
pub const DISPID_A_TEXTUNDERLINEPOSITION: u32 = 70695u32;
pub const DISPID_A_TEXTUNDERLINESTYLE: u32 = 70703u32;
pub const DISPID_A_TOUCHACTION: u32 = 71019u32;
pub const DISPID_A_TRANSFORM: u32 = 70967u32;
pub const DISPID_A_TRANSFORMORIGIN: u32 = 70968u32;
pub const DISPID_A_TRANSFORMORIGINX: u32 = 70988u32;
pub const DISPID_A_TRANSFORMORIGINY: u32 = 70989u32;
pub const DISPID_A_TRANSFORMORIGINZ: u32 = 70990u32;
pub const DISPID_A_TRANSFORMSTYLE: u32 = 70976u32;
pub const DISPID_A_TRANSITION: u32 = 70973u32;
pub const DISPID_A_TRANSITIONDELAY: u32 = 70972u32;
pub const DISPID_A_TRANSITIONDURATION: u32 = 70970u32;
pub const DISPID_A_TRANSITIONPROPERTY: u32 = 70969u32;
pub const DISPID_A_TRANSITIONTIMINGFUNCTION: u32 = 70971u32;
pub const DISPID_A_UNICODEBIDI: u32 = 70654u32;
pub const DISPID_A_UNIQUEPEERNUMBER: u32 = 70682u32;
pub const DISPID_A_URNATOM: u32 = 70681u32;
pub const DISPID_A_VALUE: u32 = 70637u32;
pub const DISPID_A_VERTICALALIGN: u32 = 70584u32;
pub const DISPID_A_VIEWINHERITSTYLE: u32 = 70735u32;
pub const DISPID_A_VISIBILITY: u32 = 70616u32;
pub const DISPID_A_WEBKIT_ANIMATION: u32 = 71033u32;
pub const DISPID_A_WEBKIT_ANIMATIONDELAY: u32 = 71038u32;
pub const DISPID_A_WEBKIT_ANIMATIONDIRECTION: u32 = 71040u32;
pub const DISPID_A_WEBKIT_ANIMATIONDURATION: u32 = 71036u32;
pub const DISPID_A_WEBKIT_ANIMATIONFILLMODE: u32 = 71027u32;
pub const DISPID_A_WEBKIT_ANIMATIONITERATIONCOUNT: u32 = 71039u32;
pub const DISPID_A_WEBKIT_ANIMATIONNAME: u32 = 71035u32;
pub const DISPID_A_WEBKIT_ANIMATIONPLAYSTATE: u32 = 71041u32;
pub const DISPID_A_WEBKIT_ANIMATIONTIMINGFUNCTION: u32 = 71037u32;
pub const DISPID_A_WEBKIT_APPEARANCE: u32 = 71020u32;
pub const DISPID_A_WEBKIT_BACKFACEVISIBILITY: u32 = 71030u32;
pub const DISPID_A_WEBKIT_BACKGROUND: u32 = 71055u32;
pub const DISPID_A_WEBKIT_BACKGROUNDATTACHMENT: u32 = 71046u32;
pub const DISPID_A_WEBKIT_BACKGROUNDCLIP: u32 = 71048u32;
pub const DISPID_A_WEBKIT_BACKGROUNDCOLOR: u32 = 71047u32;
pub const DISPID_A_WEBKIT_BACKGROUNDIMAGE: u32 = 71049u32;
pub const DISPID_A_WEBKIT_BACKGROUNDORIGIN: u32 = 71051u32;
pub const DISPID_A_WEBKIT_BACKGROUNDPOSITION: u32 = 71052u32;
pub const DISPID_A_WEBKIT_BACKGROUNDPOSITIONX: u32 = 71053u32;
pub const DISPID_A_WEBKIT_BACKGROUNDPOSITIONY: u32 = 71054u32;
pub const DISPID_A_WEBKIT_BACKGROUNDREPEAT: u32 = 71050u32;
pub const DISPID_A_WEBKIT_BACKGROUNDSIZE: u32 = 71029u32;
pub const DISPID_A_WEBKIT_BORDERIMAGE: u32 = 71061u32;
pub const DISPID_A_WEBKIT_BORDERIMAGEOUTSET: u32 = 71065u32;
pub const DISPID_A_WEBKIT_BORDERIMAGEREPEAT: u32 = 71066u32;
pub const DISPID_A_WEBKIT_BORDERIMAGESLICE: u32 = 71063u32;
pub const DISPID_A_WEBKIT_BORDERIMAGESOURCE: u32 = 71062u32;
pub const DISPID_A_WEBKIT_BORDERIMAGEWIDTH: u32 = 71064u32;
pub const DISPID_A_WEBKIT_BOXALIGN: u32 = 71021u32;
pub const DISPID_A_WEBKIT_BOXDIRECTION: u32 = 71026u32;
pub const DISPID_A_WEBKIT_BOXFLEX: u32 = 71024u32;
pub const DISPID_A_WEBKIT_BOXORDINALGROUP: u32 = 71022u32;
pub const DISPID_A_WEBKIT_BOXORIENT: u32 = 71025u32;
pub const DISPID_A_WEBKIT_BOXPACK: u32 = 71023u32;
pub const DISPID_A_WEBKIT_BOXSIZING: u32 = 71031u32;
pub const DISPID_A_WEBKIT_TEXTSIZEADJUST: u32 = 71060u32;
pub const DISPID_A_WEBKIT_TRANSFORM: u32 = 71028u32;
pub const DISPID_A_WEBKIT_TRANSFORMORIGIN: u32 = 71056u32;
pub const DISPID_A_WEBKIT_TRANSFORMORIGINX: u32 = 71057u32;
pub const DISPID_A_WEBKIT_TRANSFORMORIGINY: u32 = 71058u32;
pub const DISPID_A_WEBKIT_TRANSFORMORIGINZ: u32 = 71059u32;
pub const DISPID_A_WEBKIT_TRANSITION: u32 = 71034u32;
pub const DISPID_A_WEBKIT_TRANSITIONDELAY: u32 = 71045u32;
pub const DISPID_A_WEBKIT_TRANSITIONDURATION: u32 = 71043u32;
pub const DISPID_A_WEBKIT_TRANSITIONPROPERTY: u32 = 71042u32;
pub const DISPID_A_WEBKIT_TRANSITIONTIMINGFUNCTION: u32 = 71044u32;
pub const DISPID_A_WEBKIT_USERSELECT: u32 = 71032u32;
pub const DISPID_A_WHITESPACE: u32 = 70612u32;
pub const DISPID_A_WIDOWS: u32 = 70765u32;
pub const DISPID_A_WORDBREAK: u32 = 70670u32;
pub const DISPID_A_WORDSPACING: u32 = 70583u32;
pub const DISPID_A_WORDWRAP: u32 = 70694u32;
pub const DISPID_A_WRITINGMODE: u32 = 70728u32;
pub const DISPID_A_ZINDEX: u32 = 70627u32;
pub const DISPID_A_ZOOM: u32 = 70689u32;
pub const DISPID_BASE_STYLE: u32 = 70036u32;
pub const DISPID_BASE_STYLERULE: u32 = 1100u32;
pub const DISPID_BEFORENAVIGATE: u32 = 100u32;
pub const DISPID_BEFORENAVIGATE2: u32 = 250u32;
pub const DISPID_BEFORESCRIPTEXECUTE: u32 = 290u32;
pub const DISPID_BGSOUND: u32 = 1000u32;
pub const DISPID_BLOCK: u32 = 1000u32;
pub const DISPID_BODY: u32 = 2000u32;
pub const DISPID_BR: u32 = 1000u32;
pub const DISPID_BRANDIMAGEURI: u32 = 20u32;
pub const DISPID_BUILDNEWTABPAGE: u32 = 33u32;
pub const DISPID_BUTTON: u32 = 8000u32;
pub const DISPID_CANADVANCEERROR: u32 = 12u32;
pub const DISPID_CANRETREATERROR: u32 = 13u32;
pub const DISPID_CANVASELEMENT: u32 = 1000u32;
pub const DISPID_CANVASGRADIENT: u32 = 1000u32;
pub const DISPID_CANVASIMAGEDATA: u32 = 1000u32;
pub const DISPID_CANVASPIXELARRAY: u32 = 1000u32;
pub const DISPID_CANVASPIXELARRAY_BASE: u32 = 5000000u32;
pub const DISPID_CANVASPIXELARRAY_MAX: u32 = 2000000000u32;
pub const DISPID_CANVASRENDERCONTEXT2D: u32 = 1000u32;
pub const DISPID_CANVASTEXTMETRICS: u32 = 1000u32;
pub const DISPID_CHANGEDEFAULTBROWSER: u32 = 68u32;
pub const DISPID_CHECKBOX: u32 = 1000u32;
pub const DISPID_CLEARNOTIFICATION: u32 = 71u32;
pub const DISPID_CLEARSITEMODEICONOVERLAY: u32 = 45u32;
pub const DISPID_CLIENTCAPS: u32 = 1u32;
pub const DISPID_CLIENTTOHOSTWINDOW: u32 = 268u32;
pub const DISPID_COLLECTION: u32 = 1500u32;
pub const DISPID_COLLECTION_MAX: u32 = 2999999u32;
pub const DISPID_COLLECTION_MIN: u32 = 1000000u32;
pub const DISPID_COMMANDSTATECHANGE: u32 = 105u32;
pub const DISPID_COMMENTPDL: u32 = 1000u32;
pub const DISPID_CONTENTDISCOVERYRESET: u32 = 36u32;
pub const DISPID_COUNTVIEWTYPES: u32 = 22u32;
pub const DISPID_CREATESUBSCRIPTION: u32 = 11u32;
pub const DISPID_CUSTOMIZECLEARTYPE: u32 = 23u32;
pub const DISPID_CUSTOMIZESETTINGS: u32 = 17u32;
pub const DISPID_CommonCtrl_FONTBOLD: u32 = 3u32;
pub const DISPID_CommonCtrl_FONTCHARSET: u32 = 8u32;
pub const DISPID_CommonCtrl_FONTITAL: u32 = 4u32;
pub const DISPID_CommonCtrl_FONTNAME: u32 = 1u32;
pub const DISPID_CommonCtrl_FONTSIZE: u32 = 2u32;
pub const DISPID_CommonCtrl_FONTSTRIKE: u32 = 6u32;
pub const DISPID_CommonCtrl_FONTSUBSCRIPT: u32 = 10u32;
pub const DISPID_CommonCtrl_FONTSUPERSCRIPT: u32 = 9u32;
pub const DISPID_CommonCtrl_FONTUNDER: u32 = 5u32;
pub const DISPID_CommonCtrl_FONTWEIGHT: u32 = 7u32;
pub const DISPID_DATALIST: u32 = 1000u32;
pub const DISPID_DATATRANSFER: u32 = 1000u32;
pub const DISPID_DD: u32 = 1000u32;
pub const DISPID_DEBUG_ENABLESECUREPROXYASSERTS: i32 = -5518i32;
pub const DISPID_DEBUG_INTERNALWINDOW: i32 = -5517i32;
pub const DISPID_DEBUG_ISSECUREPROXY: i32 = -5515i32;
pub const DISPID_DEBUG_TRUSTEDPROXY: i32 = -5516i32;
pub const DISPID_DEFAULTS: u32 = 1000u32;
pub const DISPID_DEFAULTSEARCHPROVIDER: u32 = 26u32;
pub const DISPID_DEFAULTVALUE: u32 = 70619u32;
pub const DISPID_DELETESUBSCRIPTION: u32 = 12u32;
pub const DISPID_DEPTH: u32 = 17u32;
pub const DISPID_DIAGNOSECONNECTION: u32 = 22u32;
pub const DISPID_DIAGNOSECONNECTIONUILESS: u32 = 66u32;
pub const DISPID_DIR: u32 = 1000u32;
pub const DISPID_DIV: u32 = 1000u32;
pub const DISPID_DL: u32 = 1000u32;
pub const DISPID_DOCFRAG: u32 = 1000u32;
pub const DISPID_DOCUMENTCOMPATIBLEINFO: u32 = 1000u32;
pub const DISPID_DOCUMENTCOMPATIBLEINFO_COLLECTION: u32 = 1000u32;
pub const DISPID_DOCUMENTCOMPLETE: u32 = 259u32;
pub const DISPID_DOCUMENTTYPE: u32 = 1000u32;
pub const DISPID_DOMATTRIBUTE: u32 = 1000u32;
pub const DISPID_DOMBEFOREUNLOADEVENT: u32 = 1375u32;
pub const DISPID_DOMCLOSEEVENT: u32 = 1525u32;
pub const DISPID_DOMCOMPOSITIONEVENT: u32 = 1175u32;
pub const DISPID_DOMCUSTOMEVENT: u32 = 1200u32;
pub const DISPID_DOMDRAGEVENT: u32 = 1400u32;
pub const DISPID_DOMEVENT: u32 = 1000u32;
pub const DISPID_DOMEXCEPTION: u32 = 1000u32;
pub const DISPID_DOMFOCUSEVENT: u32 = 1250u32;
pub const DISPID_DOMIMPLEMENTATION: u32 = 1000u32;
pub const DISPID_DOMKEYBOARDEVENT: u32 = 1150u32;
pub const DISPID_DOMMESSAGEEVENT: u32 = 1325u32;
pub const DISPID_DOMMOUSEEVENT: u32 = 1050u32;
pub const DISPID_DOMMOUSEWHEELEVENT: u32 = 1075u32;
pub const DISPID_DOMMSANIMATIONEVENT: u32 = 1500u32;
pub const DISPID_DOMMSGESTUREEVENT: u32 = 1450u32;
pub const DISPID_DOMMSMANIPULATIONEVENT: u32 = 1525u32;
pub const DISPID_DOMMSPOINTEREVENT: u32 = 1425u32;
pub const DISPID_DOMMSTRANSITIONEVENT: u32 = 1475u32;
pub const DISPID_DOMMUTATIONEVENT: u32 = 1225u32;
pub const DISPID_DOMPARSER: u32 = 1000u32;
pub const DISPID_DOMPROGRESSEVENT: u32 = 1550u32;
pub const DISPID_DOMRANGE: u32 = 1000u32;
pub const DISPID_DOMSITEMODEEVENT: u32 = 1300u32;
pub const DISPID_DOMSTORAGE: u32 = 1000u32;
pub const DISPID_DOMSTORAGEEVENT: u32 = 1350u32;
pub const DISPID_DOMSTORAGEITEM: u32 = 1000u32;
pub const DISPID_DOMSTORAGELIST: u32 = 1000u32;
pub const DISPID_DOMTEXTEVENT: u32 = 1125u32;
pub const DISPID_DOMTEXTNODE: u32 = 1000u32;
pub const DISPID_DOMTRAVERSAL: u32 = 1000u32;
pub const DISPID_DOMUIEVENT: u32 = 1025u32;
pub const DISPID_DOMWHEELEVENT: u32 = 1100u32;
pub const DISPID_DOUBLECLICK: u32 = 3u32;
pub const DISPID_DOWNLOADBEGIN: u32 = 106u32;
pub const DISPID_DOWNLOADCOMPLETE: u32 = 104u32;
pub const DISPID_DT: u32 = 1000u32;
pub const DISPID_DWEBBRIDGEEVENTS_ONCLICK: i32 = -600i32;
pub const DISPID_DWEBBRIDGEEVENTS_ONDBLCLICK: i32 = -601i32;
pub const DISPID_DWEBBRIDGEEVENTS_ONKEYDOWN: i32 = -602i32;
pub const DISPID_DWEBBRIDGEEVENTS_ONKEYPRESS: i32 = -603i32;
pub const DISPID_DWEBBRIDGEEVENTS_ONKEYUP: i32 = -604i32;
pub const DISPID_DWEBBRIDGEEVENTS_ONMOUSEDOWN: i32 = -605i32;
pub const DISPID_DWEBBRIDGEEVENTS_ONMOUSEMOVE: i32 = -606i32;
pub const DISPID_DWEBBRIDGEEVENTS_ONMOUSEUP: i32 = -607i32;
pub const DISPID_DWEBBRIDGEEVENTS_ONREADYSTATECHANGE: i32 = -609i32;
pub const DISPID_DWEBBRIDGEEVENTS_ONSCRIPTLETEVENT: u32 = 1u32;
pub const DISPID_EFONT: u32 = 1000u32;
pub const DISPID_ELEMENT: u32 = 66536u32;
pub const DISPID_ENABLENOTIFICATIONQUEUE: u32 = 72u32;
pub const DISPID_ENABLENOTIFICATIONQUEUELARGE: u32 = 78u32;
pub const DISPID_ENABLENOTIFICATIONQUEUESQUARE: u32 = 76u32;
pub const DISPID_ENABLENOTIFICATIONQUEUEWIDE: u32 = 77u32;
pub const DISPID_ENABLESUGGESTEDSITES: u32 = 39u32;
pub const DISPID_ENUMOPTIONS: u32 = 14u32;
pub const DISPID_EVENTEXCEPTION: u32 = 1000u32;
pub const DISPID_EVENTHOOK_INSENSITIVE_BASE: u32 = 4500000u32;
pub const DISPID_EVENTHOOK_INSENSITIVE_MAX: u32 = 4999999u32;
pub const DISPID_EVENTHOOK_SENSITIVE_BASE: u32 = 4000000u32;
pub const DISPID_EVENTHOOK_SENSITIVE_MAX: u32 = 4499999u32;
pub const DISPID_EVENTOBJ: u32 = 1000u32;
pub const DISPID_EVENTS: u32 = 71536u32;
pub const DISPID_EVMETH_ONABORT: u32 = 1000u32;
pub const DISPID_EVMETH_ONACTIVATE: u32 = 1044u32;
pub const DISPID_EVMETH_ONAFTERPRINT: u32 = 1025u32;
pub const DISPID_EVMETH_ONAFTERUPDATE: u32 = 65541u32;
pub const DISPID_EVMETH_ONALERT: u32 = 1061u32;
pub const DISPID_EVMETH_ONBEFOREACTIVATE: u32 = 1047u32;
pub const DISPID_EVMETH_ONBEFORECOPY: u32 = 65566u32;
pub const DISPID_EVMETH_ONBEFORECUT: u32 = 65565u32;
pub const DISPID_EVMETH_ONBEFOREDEACTIVATE: u32 = 1034u32;
pub const DISPID_EVMETH_ONBEFOREEDITFOCUS: u32 = 1027u32;
pub const DISPID_EVMETH_ONBEFOREPASTE: u32 = 65567u32;
pub const DISPID_EVMETH_ONBEFOREPRINT: u32 = 1024u32;
pub const DISPID_EVMETH_ONBEFOREUNLOAD: u32 = 1017u32;
pub const DISPID_EVMETH_ONBEFOREUPDATE: u32 = 65540u32;
pub const DISPID_EVMETH_ONBOUNCE: u32 = 1009u32;
pub const DISPID_EVMETH_ONCELLCHANGE: u32 = 65570u32;
pub const DISPID_EVMETH_ONCHANGE: u32 = 1001u32;
pub const DISPID_EVMETH_ONCHANGEBLUR: u32 = 1019u32;
pub const DISPID_EVMETH_ONCHANGEFOCUS: u32 = 1018u32;
pub const DISPID_EVMETH_ONCLICK: i32 = -600i32;
pub const DISPID_EVMETH_ONCONTENTREADY: u32 = 1029u32;
pub const DISPID_EVMETH_ONCONTEXTMENU: u32 = 1023u32;
pub const DISPID_EVMETH_ONCONTROLSELECT: u32 = 1036u32;
pub const DISPID_EVMETH_ONCOPY: u32 = 65563u32;
pub const DISPID_EVMETH_ONCUT: u32 = 65562u32;
pub const DISPID_EVMETH_ONDATAAVAILABLE: u32 = 65551u32;
pub const DISPID_EVMETH_ONDATASETCHANGED: u32 = 65550u32;
pub const DISPID_EVMETH_ONDATASETCOMPLETE: u32 = 65552u32;
pub const DISPID_EVMETH_ONDBLCLICK: i32 = -601i32;
pub const DISPID_EVMETH_ONDEACTIVATE: u32 = 1045u32;
pub const DISPID_EVMETH_ONDOMMUTATION: u32 = 1068u32;
pub const DISPID_EVMETH_ONDRAG: u32 = 65556u32;
pub const DISPID_EVMETH_ONDRAGEND: u32 = 65557u32;
pub const DISPID_EVMETH_ONDRAGENTER: u32 = 65558u32;
pub const DISPID_EVMETH_ONDRAGLEAVE: u32 = 65560u32;
pub const DISPID_EVMETH_ONDRAGOVER: u32 = 65559u32;
pub const DISPID_EVMETH_ONDRAGSTART: u32 = 65547u32;
pub const DISPID_EVMETH_ONDROP: u32 = 65561u32;
pub const DISPID_EVMETH_ONERROR: u32 = 1002u32;
pub const DISPID_EVMETH_ONERRORUPDATE: u32 = 65549u32;
pub const DISPID_EVMETH_ONFILTER: u32 = 65553u32;
pub const DISPID_EVMETH_ONFINISH: u32 = 1010u32;
pub const DISPID_EVMETH_ONFOCUS: u32 = 65537u32;
pub const DISPID_EVMETH_ONFOCUSIN: u32 = 1048u32;
pub const DISPID_EVMETH_ONFOCUSOUT: u32 = 1049u32;
pub const DISPID_EVMETH_ONHASHCHANGE: u32 = 1066u32;
pub const DISPID_EVMETH_ONHELP: u32 = 65546u32;
pub const DISPID_EVMETH_ONHIDE: u32 = 1060u32;
pub const DISPID_EVMETH_ONKEYDOWN: i32 = -602i32;
pub const DISPID_EVMETH_ONKEYPRESS: i32 = -603i32;
pub const DISPID_EVMETH_ONKEYUP: i32 = -604i32;
pub const DISPID_EVMETH_ONLAYOUT: u32 = 1013u32;
pub const DISPID_EVMETH_ONLAYOUTCOMPLETE: u32 = 1030u32;
pub const DISPID_EVMETH_ONLINKEDOVERFLOW: u32 = 1032u32;
pub const DISPID_EVMETH_ONLOAD: u32 = 1003u32;
pub const DISPID_EVMETH_ONLOSECAPTURE: u32 = 65554u32;
pub const DISPID_EVMETH_ONMESSAGE: u32 = 1067u32;
pub const DISPID_EVMETH_ONMOUSEDOWN: i32 = -605i32;
pub const DISPID_EVMETH_ONMOUSEENTER: u32 = 1042u32;
pub const DISPID_EVMETH_ONMOUSEHOVER: u32 = 1028u32;
pub const DISPID_EVMETH_ONMOUSELEAVE: u32 = 1043u32;
pub const DISPID_EVMETH_ONMOUSEMOVE: i32 = -606i32;
pub const DISPID_EVMETH_ONMOUSEOUT: u32 = 65545u32;
pub const DISPID_EVMETH_ONMOUSEOVER: u32 = 65544u32;
pub const DISPID_EVMETH_ONMOUSEUP: i32 = -607i32;
pub const DISPID_EVMETH_ONMOUSEWHEEL: u32 = 1033u32;
pub const DISPID_EVMETH_ONMOVE: u32 = 1035u32;
pub const DISPID_EVMETH_ONMOVEEND: u32 = 1039u32;
pub const DISPID_EVMETH_ONMOVESTART: u32 = 1038u32;
pub const DISPID_EVMETH_ONMULTILAYOUTCLEANUP: u32 = 1046u32;
pub const DISPID_EVMETH_ONOBJECTCONTENTSCROLLED: u32 = 1056u32;
pub const DISPID_EVMETH_ONOFFLINE: u32 = 1065u32;
pub const DISPID_EVMETH_ONONLINE: u32 = 1064u32;
pub const DISPID_EVMETH_ONPAGE: u32 = 1031u32;
pub const DISPID_EVMETH_ONPASTE: u32 = 65564u32;
pub const DISPID_EVMETH_ONPERSISTLOAD: u32 = 1022u32;
pub const DISPID_EVMETH_ONPERSISTSAVE: u32 = 1021u32;
pub const DISPID_EVMETH_ONPOPUPMENUEND: u32 = 1063u32;
pub const DISPID_EVMETH_ONPOPUPMENUSTART: u32 = 1062u32;
pub const DISPID_EVMETH_ONPROPERTYCHANGE: u32 = 65555u32;
pub const DISPID_EVMETH_ONREADYSTATECHANGE: i32 = -609i32;
pub const DISPID_EVMETH_ONRESET: u32 = 1015u32;
pub const DISPID_EVMETH_ONRESIZE: u32 = 1016u32;
pub const DISPID_EVMETH_ONRESIZEEND: u32 = 1041u32;
pub const DISPID_EVMETH_ONRESIZESTART: u32 = 1040u32;
pub const DISPID_EVMETH_ONROWENTER: u32 = 65543u32;
pub const DISPID_EVMETH_ONROWEXIT: u32 = 65542u32;
pub const DISPID_EVMETH_ONROWSDELETE: u32 = 65568u32;
pub const DISPID_EVMETH_ONROWSINSERTED: u32 = 65569u32;
pub const DISPID_EVMETH_ONSCROLL: u32 = 1014u32;
pub const DISPID_EVMETH_ONSELECT: u32 = 1006u32;
pub const DISPID_EVMETH_ONSELECTADD: u32 = 1051u32;
pub const DISPID_EVMETH_ONSELECTIONCHANGE: u32 = 1037u32;
pub const DISPID_EVMETH_ONSELECTREMOVE: u32 = 1052u32;
pub const DISPID_EVMETH_ONSELECTSTART: u32 = 65548u32;
pub const DISPID_EVMETH_ONSELECTWITHIN: u32 = 1053u32;
pub const DISPID_EVMETH_ONSHOW: u32 = 1059u32;
pub const DISPID_EVMETH_ONSTART: u32 = 1011u32;
pub const DISPID_EVMETH_ONSTOP: u32 = 1026u32;
pub const DISPID_EVMETH_ONSTORAGE: u32 = 1057u32;
pub const DISPID_EVMETH_ONSTORAGECOMMIT: u32 = 1058u32;
pub const DISPID_EVMETH_ONSUBMIT: u32 = 1007u32;
pub const DISPID_EVMETH_ONSYSTEMSCROLLINGEND: u32 = 1055u32;
pub const DISPID_EVMETH_ONSYSTEMSCROLLINGSTART: u32 = 1054u32;
pub const DISPID_EVMETH_ONUNLOAD: u32 = 1008u32;
pub const DISPID_EVMETH_ONVALUECHANGE: u32 = 1050u32;
pub const DISPID_EVPROPS_COUNT: u32 = 260u32;
pub const DISPID_EVPROP_ADDTRACK: u32 = 71736u32;
pub const DISPID_EVPROP_CACHED: u32 = 71721u32;
pub const DISPID_EVPROP_CANPLAY: u32 = 71670u32;
pub const DISPID_EVPROP_CANPLAYTHROUGH: u32 = 71671u32;
pub const DISPID_EVPROP_CHECKING: u32 = 71717u32;
pub const DISPID_EVPROP_COMPOSITIONEND: u32 = 71660u32;
pub const DISPID_EVPROP_COMPOSITIONSTART: u32 = 71658u32;
pub const DISPID_EVPROP_COMPOSITIONUPDATE: u32 = 71659u32;
pub const DISPID_EVPROP_DOMATTRMODIFIED: u32 = 71661u32;
pub const DISPID_EVPROP_DOMCHARDATAMODIFIED: u32 = 71664u32;
pub const DISPID_EVPROP_DOMCONTENTLOADED: u32 = 71662u32;
pub const DISPID_EVPROP_DOMNODEINSERTED: u32 = 71667u32;
pub const DISPID_EVPROP_DOMNODEREMOVED: u32 = 71668u32;
pub const DISPID_EVPROP_DOMSUBTREEMODIFIED: u32 = 71669u32;
pub const DISPID_EVPROP_DOWNLOADING: u32 = 71719u32;
pub const DISPID_EVPROP_DURATIONCHANGE: u32 = 71672u32;
pub const DISPID_EVPROP_EMPTIED: u32 = 71673u32;
pub const DISPID_EVPROP_ENDED: u32 = 71674u32;
pub const DISPID_EVPROP_HTML5ONREADYSTATECHANGE: u32 = 71780u32;
pub const DISPID_EVPROP_INPUT: u32 = 71663u32;
pub const DISPID_EVPROP_INVALID: u32 = 71724u32;
pub const DISPID_EVPROP_LOADEDDATA: u32 = 71675u32;
pub const DISPID_EVPROP_LOADEDMETADATA: u32 = 71676u32;
pub const DISPID_EVPROP_LOADEND: u32 = 71723u32;
pub const DISPID_EVPROP_LOADSTART: u32 = 71677u32;
pub const DISPID_EVPROP_MSCONNECT: u32 = 71697u32;
pub const DISPID_EVPROP_MSDISCONNECT: u32 = 71698u32;
pub const DISPID_EVPROP_MSELEMENTRESIZE: u32 = 71742u32;
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONCONTAINSFULLSCREENELEMENTCHANGED: u32 = 71783u32;
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONCONTENTLOADING: u32 = 71753u32;
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONDOMCONTENTLOADED: u32 = 71752u32;
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONFRAMECONTENTLOADING: u32 = 71757u32;
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONFRAMEDOMCONTENTLOADED: u32 = 71756u32;
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONFRAMENAVIGATIONCOMPLETED: u32 = 71759u32;
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONFRAMENAVIGATIONSTARTING: u32 = 71758u32;
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONLONGRUNNINGSCRIPTDETECTED: u32 = 71763u32;
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONNAVIGATIONCOMPLETED: u32 = 71755u32;
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONNAVIGATIONSTARTING: u32 = 71754u32;
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONSCRIPTNOTIFY: u32 = 71760u32;
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONUNSAFECONTENTWARNINGDISPLAYING: u32 = 71762u32;
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONUNVIEWABLECONTENT: u32 = 71761u32;
pub const DISPID_EVPROP_MSORIENTATIONCHANGE: u32 = 71772u32;
pub const DISPID_EVPROP_NOUPDATE: u32 = 71718u32;
pub const DISPID_EVPROP_OBSOLETE: u32 = 71722u32;
pub const DISPID_EVPROP_ONABORT: u32 = 71564u32;
pub const DISPID_EVPROP_ONACTIVATE: u32 = 71623u32;
pub const DISPID_EVPROP_ONADDSOURCEBUFFER: u32 = 71746u32;
pub const DISPID_EVPROP_ONAFTERPRINT: u32 = 71603u32;
pub const DISPID_EVPROP_ONAFTERUPDATE: u32 = 71558u32;
pub const DISPID_EVPROP_ONALERT: u32 = 71640u32;
pub const DISPID_EVPROP_ONANIMATIONEND: u32 = 71712u32;
pub const DISPID_EVPROP_ONANIMATIONITERATION: u32 = 71713u32;
pub const DISPID_EVPROP_ONANIMATIONSTART: u32 = 71711u32;
pub const DISPID_EVPROP_ONATTACHEVENT: u32 = 71606u32;
pub const DISPID_EVPROP_ONBEFOREACTIVATE: u32 = 71626u32;
pub const DISPID_EVPROP_ONBEFORECOPY: u32 = 71595u32;
pub const DISPID_EVPROP_ONBEFORECUT: u32 = 71594u32;
pub const DISPID_EVPROP_ONBEFOREDEACTIVATE: u32 = 71613u32;
pub const DISPID_EVPROP_ONBEFOREDRAGOVER: u32 = 71559u32;
pub const DISPID_EVPROP_ONBEFOREDROPORPASTE: u32 = 71560u32;
pub const DISPID_EVPROP_ONBEFOREEDITFOCUS: u32 = 71605u32;
pub const DISPID_EVPROP_ONBEFOREPASTE: u32 = 71596u32;
pub const DISPID_EVPROP_ONBEFOREPRINT: u32 = 71602u32;
pub const DISPID_EVPROP_ONBEFOREUNLOAD: u32 = 71575u32;
pub const DISPID_EVPROP_ONBEFOREUPDATE: u32 = 71557u32;
pub const DISPID_EVPROP_ONBLOCKED: u32 = 71726u32;
pub const DISPID_EVPROP_ONBLUR: u32 = 71551u32;
pub const DISPID_EVPROP_ONBOUNCE: u32 = 71556u32;
pub const DISPID_EVPROP_ONCELLCHANGE: u32 = 71600u32;
pub const DISPID_EVPROP_ONCHANGE: u32 = 71566u32;
pub const DISPID_EVPROP_ONCHANGEBLUR: u32 = 71581u32;
pub const DISPID_EVPROP_ONCHANGEFOCUS: u32 = 71580u32;
pub const DISPID_EVPROP_ONCLICK: u32 = 71544u32;
pub const DISPID_EVPROP_ONCLOSE: u32 = 71716u32;
pub const DISPID_EVPROP_ONCOMPASSNEEDSCALIBRATION: u32 = 71782u32;
pub const DISPID_EVPROP_ONCOMPLETE: u32 = 71727u32;
pub const DISPID_EVPROP_ONCONTENTREADY: u32 = 71608u32;
pub const DISPID_EVPROP_ONCONTEXTMENU: u32 = 71601u32;
pub const DISPID_EVPROP_ONCONTROLSELECT: u32 = 71615u32;
pub const DISPID_EVPROP_ONCOPY: u32 = 71592u32;
pub const DISPID_EVPROP_ONCUECHANGE: u32 = 71729u32;
pub const DISPID_EVPROP_ONCUT: u32 = 71591u32;
pub const DISPID_EVPROP_ONDATAAVAILABLE: u32 = 71577u32;
pub const DISPID_EVPROP_ONDATASETCHANGED: u32 = 71576u32;
pub const DISPID_EVPROP_ONDATASETCOMPLETE: u32 = 71578u32;
pub const DISPID_EVPROP_ONDBLCLICK: u32 = 71545u32;
pub const DISPID_EVPROP_ONDEACTIVATE: u32 = 71624u32;
pub const DISPID_EVPROP_ONDEVICEMOTION: u32 = 71774u32;
pub const DISPID_EVPROP_ONDEVICEORIENTATION: u32 = 71773u32;
pub const DISPID_EVPROP_ONDOMFOCUSIN: u32 = 71793u32;
pub const DISPID_EVPROP_ONDOMFOCUSOUT: u32 = 71794u32;
pub const DISPID_EVPROP_ONDOMMUTATION: u32 = 71647u32;
pub const DISPID_EVPROP_ONDRAG: u32 = 71585u32;
pub const DISPID_EVPROP_ONDRAGEND: u32 = 71586u32;
pub const DISPID_EVPROP_ONDRAGENTER: u32 = 71587u32;
pub const DISPID_EVPROP_ONDRAGLEAVE: u32 = 71589u32;
pub const DISPID_EVPROP_ONDRAGOVER: u32 = 71588u32;
pub const DISPID_EVPROP_ONDRAGSTART: u32 = 71571u32;
pub const DISPID_EVPROP_ONDROP: u32 = 71590u32;
pub const DISPID_EVPROP_ONENTER: u32 = 71730u32;
pub const DISPID_EVPROP_ONERROR: u32 = 71565u32;
pub const DISPID_EVPROP_ONERRORUPDATE: u32 = 71574u32;
pub const DISPID_EVPROP_ONEXIT: u32 = 71731u32;
pub const DISPID_EVPROP_ONFILTER: u32 = 71579u32;
pub const DISPID_EVPROP_ONFINISH: u32 = 71562u32;
pub const DISPID_EVPROP_ONFOCUS: u32 = 71550u32;
pub const DISPID_EVPROP_ONFOCUSIN: u32 = 71627u32;
pub const DISPID_EVPROP_ONFOCUSOUT: u32 = 71628u32;
pub const DISPID_EVPROP_ONHASHCHANGE: u32 = 71645u32;
pub const DISPID_EVPROP_ONHELP: u32 = 71549u32;
pub const DISPID_EVPROP_ONHIDE: u32 = 71639u32;
pub const DISPID_EVPROP_ONKEYDOWN: u32 = 71541u32;
pub const DISPID_EVPROP_ONKEYPRESS: u32 = 71543u32;
pub const DISPID_EVPROP_ONKEYUP: u32 = 71542u32;
pub const DISPID_EVPROP_ONLAYOUT: u32 = 71570u32;
pub const DISPID_EVPROP_ONLAYOUTCOMPLETE: u32 = 71609u32;
pub const DISPID_EVPROP_ONLINKEDOVERFLOW: u32 = 71611u32;
pub const DISPID_EVPROP_ONLOAD: u32 = 71568u32;
pub const DISPID_EVPROP_ONLOSECAPTURE: u32 = 71582u32;
pub const DISPID_EVPROP_ONMESSAGE: u32 = 71646u32;
pub const DISPID_EVPROP_ONMOUSEDOWN: u32 = 71538u32;
pub const DISPID_EVPROP_ONMOUSEENTER: u32 = 71621u32;
pub const DISPID_EVPROP_ONMOUSEHOVER: u32 = 71607u32;
pub const DISPID_EVPROP_ONMOUSELEAVE: u32 = 71622u32;
pub const DISPID_EVPROP_ONMOUSEMOVE: u32 = 71540u32;
pub const DISPID_EVPROP_ONMOUSEOUT: u32 = 71537u32;
pub const DISPID_EVPROP_ONMOUSEOVER: u32 = 71536u32;
pub const DISPID_EVPROP_ONMOUSEUP: u32 = 71539u32;
pub const DISPID_EVPROP_ONMOUSEWHEEL: u32 = 71612u32;
pub const DISPID_EVPROP_ONMOVE: u32 = 71614u32;
pub const DISPID_EVPROP_ONMOVEEND: u32 = 71618u32;
pub const DISPID_EVPROP_ONMOVESTART: u32 = 71617u32;
pub const DISPID_EVPROP_ONMSCANDIDATEWINDOWHIDE: u32 = 71779u32;
pub const DISPID_EVPROP_ONMSCANDIDATEWINDOWSHOW: u32 = 71777u32;
pub const DISPID_EVPROP_ONMSCANDIDATEWINDOWUPDATE: u32 = 71778u32;
pub const DISPID_EVPROP_ONMSCONTENTZOOM: u32 = 71708u32;
pub const DISPID_EVPROP_ONMSFULLSCREENCHANGE: u32 = 71740u32;
pub const DISPID_EVPROP_ONMSFULLSCREENERROR: u32 = 71741u32;
pub const DISPID_EVPROP_ONMSGESTURECHANGE: u32 = 71700u32;
pub const DISPID_EVPROP_ONMSGESTUREDOUBLETAP: u32 = 71704u32;
pub const DISPID_EVPROP_ONMSGESTUREEND: u32 = 71701u32;
pub const DISPID_EVPROP_ONMSGESTUREHOLD: u32 = 71702u32;
pub const DISPID_EVPROP_ONMSGESTURESTART: u32 = 71699u32;
pub const DISPID_EVPROP_ONMSGESTURETAP: u32 = 71703u32;
pub const DISPID_EVPROP_ONMSGOTPOINTERCAPTURE: u32 = 71707u32;
pub const DISPID_EVPROP_ONMSHOLDVISUAL: u32 = 71738u32;
pub const DISPID_EVPROP_ONMSINERTIASTART: u32 = 71705u32;
pub const DISPID_EVPROP_ONMSKEYADDED: u32 = 71751u32;
pub const DISPID_EVPROP_ONMSKEYERROR: u32 = 71750u32;
pub const DISPID_EVPROP_ONMSKEYMESSAGE: u32 = 71749u32;
pub const DISPID_EVPROP_ONMSLOSTPOINTERCAPTURE: u32 = 71706u32;
pub const DISPID_EVPROP_ONMSMANIPULATIONSTATECHANGED: u32 = 71714u32;
pub const DISPID_EVPROP_ONMSNEEDKEY: u32 = 71748u32;
pub const DISPID_EVPROP_ONMSPOINTERCANCEL: u32 = 71695u32;
pub const DISPID_EVPROP_ONMSPOINTERDOWN: u32 = 71690u32;
pub const DISPID_EVPROP_ONMSPOINTERENTER: u32 = 71769u32;
pub const DISPID_EVPROP_ONMSPOINTERHOVER: u32 = 71696u32;
pub const DISPID_EVPROP_ONMSPOINTERLEAVE: u32 = 71770u32;
pub const DISPID_EVPROP_ONMSPOINTERMOVE: u32 = 71691u32;
pub const DISPID_EVPROP_ONMSPOINTEROUT: u32 = 71694u32;
pub const DISPID_EVPROP_ONMSPOINTEROVER: u32 = 71693u32;
pub const DISPID_EVPROP_ONMSPOINTERUP: u32 = 71692u32;
pub const DISPID_EVPROP_ONMSREGIONUPDATE: u32 = 71733u32;
pub const DISPID_EVPROP_ONMSSITEMODEJUMPLISTITEMREMOVED: u32 = 71666u32;
pub const DISPID_EVPROP_ONMSSITEPINNED: u32 = 71771u32;
pub const DISPID_EVPROP_ONMSTHUMBNAILCLICK: u32 = 71657u32;
pub const DISPID_EVPROP_ONMSVIDEOFORMATCHANGED: u32 = 71735u32;
pub const DISPID_EVPROP_ONMSVIDEOFRAMESTEPCOMPLETED: u32 = 71737u32;
pub const DISPID_EVPROP_ONMSVIDEOOPTIMALLAYOUTCHANGED: u32 = 71739u32;
pub const DISPID_EVPROP_ONMULTILAYOUTCLEANUP: u32 = 71625u32;
pub const DISPID_EVPROP_ONOBJECTCONTENTSCROLLED: u32 = 71635u32;
pub const DISPID_EVPROP_ONOFFLINE: u32 = 71644u32;
pub const DISPID_EVPROP_ONONLINE: u32 = 71643u32;
pub const DISPID_EVPROP_ONOPEN: u32 = 71715u32;
pub const DISPID_EVPROP_ONPAGE: u32 = 71610u32;
pub const DISPID_EVPROP_ONPAGEHIDE: u32 = 71776u32;
pub const DISPID_EVPROP_ONPAGESHOW: u32 = 71775u32;
pub const DISPID_EVPROP_ONPASTE: u32 = 71593u32;
pub const DISPID_EVPROP_ONPERSISTLOAD: u32 = 71597u32;
pub const DISPID_EVPROP_ONPERSISTSAVE: u32 = 71584u32;
pub const DISPID_EVPROP_ONPOPSTATE: u32 = 71728u32;
pub const DISPID_EVPROP_ONPOPUPMENUEND: u32 = 71642u32;
pub const DISPID_EVPROP_ONPOPUPMENUSTART: u32 = 71641u32;
pub const DISPID_EVPROP_ONPROPERTYCHANGE: u32 = 71583u32;
pub const DISPID_EVPROP_ONREADYSTATECHANGE: u32 = 71561u32;
pub const DISPID_EVPROP_ONREMOVESOURCEBUFFER: u32 = 71747u32;
pub const DISPID_EVPROP_ONRESET: u32 = 71548u32;
pub const DISPID_EVPROP_ONRESIZE: u32 = 71572u32;
pub const DISPID_EVPROP_ONRESIZEEND: u32 = 71620u32;
pub const DISPID_EVPROP_ONRESIZESTART: u32 = 71619u32;
pub const DISPID_EVPROP_ONROWENTER: u32 = 71555u32;
pub const DISPID_EVPROP_ONROWEXIT: u32 = 71554u32;
pub const DISPID_EVPROP_ONROWSDELETE: u32 = 71598u32;
pub const DISPID_EVPROP_ONROWSINSERTED: u32 = 71599u32;
pub const DISPID_EVPROP_ONSCROLL: u32 = 71567u32;
pub const DISPID_EVPROP_ONSELECT: u32 = 71546u32;
pub const DISPID_EVPROP_ONSELECTADD: u32 = 71630u32;
pub const DISPID_EVPROP_ONSELECTIONCHANGE: u32 = 71616u32;
pub const DISPID_EVPROP_ONSELECTREMOVE: u32 = 71631u32;
pub const DISPID_EVPROP_ONSELECTSTART: u32 = 71573u32;
pub const DISPID_EVPROP_ONSELECTWITHIN: u32 = 71632u32;
pub const DISPID_EVPROP_ONSHOW: u32 = 71638u32;
pub const DISPID_EVPROP_ONSOURCECLOSE: u32 = 71744u32;
pub const DISPID_EVPROP_ONSOURCEENDED: u32 = 71745u32;
pub const DISPID_EVPROP_ONSOURCEOPEN: u32 = 71743u32;
pub const DISPID_EVPROP_ONSTART: u32 = 71563u32;
pub const DISPID_EVPROP_ONSTOP: u32 = 71604u32;
pub const DISPID_EVPROP_ONSTORAGE: u32 = 71636u32;
pub const DISPID_EVPROP_ONSTORAGECOMMIT: u32 = 71637u32;
pub const DISPID_EVPROP_ONSUBMIT: u32 = 71547u32;
pub const DISPID_EVPROP_ONSUCCESS: u32 = 71725u32;
pub const DISPID_EVPROP_ONSYSTEMSCROLLINGEND: u32 = 71634u32;
pub const DISPID_EVPROP_ONSYSTEMSCROLLINGSTART: u32 = 71633u32;
pub const DISPID_EVPROP_ONTOUCHCANCEL: u32 = 71787u32;
pub const DISPID_EVPROP_ONTOUCHEND: u32 = 71785u32;
pub const DISPID_EVPROP_ONTOUCHMOVE: u32 = 71786u32;
pub const DISPID_EVPROP_ONTOUCHSTART: u32 = 71784u32;
pub const DISPID_EVPROP_ONTRANSITIONEND: u32 = 71710u32;
pub const DISPID_EVPROP_ONTRANSITIONSTART: u32 = 71709u32;
pub const DISPID_EVPROP_ONUNLOAD: u32 = 71569u32;
pub const DISPID_EVPROP_ONUPDATE: u32 = 71767u32;
pub const DISPID_EVPROP_ONUPDATEEND: u32 = 71768u32;
pub const DISPID_EVPROP_ONUPDATESTART: u32 = 71766u32;
pub const DISPID_EVPROP_ONUPGRADENEEDED: u32 = 71734u32;
pub const DISPID_EVPROP_ONVALUECHANGE: u32 = 71629u32;
pub const DISPID_EVPROP_ONWEBKITANIMATIONEND: u32 = 71790u32;
pub const DISPID_EVPROP_ONWEBKITANIMATIONITERATION: u32 = 71791u32;
pub const DISPID_EVPROP_ONWEBKITANIMATIONSTART: u32 = 71789u32;
pub const DISPID_EVPROP_ONWEBKITTRANSITIONEND: u32 = 71788u32;
pub const DISPID_EVPROP_ORIENTATIONCHANGE: u32 = 71795u32;
pub const DISPID_EVPROP_PAUSE: u32 = 71678u32;
pub const DISPID_EVPROP_PLAY: u32 = 71679u32;
pub const DISPID_EVPROP_PLAYING: u32 = 71680u32;
pub const DISPID_EVPROP_PROGRESS: u32 = 71681u32;
pub const DISPID_EVPROP_RATECHANGE: u32 = 71682u32;
pub const DISPID_EVPROP_REMOVETRACK: u32 = 71781u32;
pub const DISPID_EVPROP_SEEKED: u32 = 71683u32;
pub const DISPID_EVPROP_SEEKING: u32 = 71684u32;
pub const DISPID_EVPROP_SINKLIMIT: u32 = 71647u32;
pub const DISPID_EVPROP_STALLED: u32 = 71685u32;
pub const DISPID_EVPROP_SUSPEND: u32 = 71686u32;
pub const DISPID_EVPROP_SVGABORT: u32 = 71652u32;
pub const DISPID_EVPROP_SVGERROR: u32 = 71653u32;
pub const DISPID_EVPROP_SVGLOAD: u32 = 71650u32;
pub const DISPID_EVPROP_SVGRESIZE: u32 = 71654u32;
pub const DISPID_EVPROP_SVGSCROLL: u32 = 71655u32;
pub const DISPID_EVPROP_SVGUNLOAD: u32 = 71651u32;
pub const DISPID_EVPROP_SVGZOOM: u32 = 71656u32;
pub const DISPID_EVPROP_TEXTINPUT: u32 = 71665u32;
pub const DISPID_EVPROP_TIMEOUT: u32 = 71648u32;
pub const DISPID_EVPROP_TIMEUPDATE: u32 = 71687u32;
pub const DISPID_EVPROP_UPDATEREADY: u32 = 71720u32;
pub const DISPID_EVPROP_VISIBILITYCHANGE: u32 = 71732u32;
pub const DISPID_EVPROP_VOLUMECHANGE: u32 = 71688u32;
pub const DISPID_EVPROP_WAITING: u32 = 71689u32;
pub const DISPID_EVPROP_WEBGLCONTEXTCREATIONERROR: u32 = 71792u32;
pub const DISPID_EVPROP_WEBGLCONTEXTLOST: u32 = 71764u32;
pub const DISPID_EVPROP_WEBGLCONTEXTRESTORED: u32 = 71765u32;
pub const DISPID_EVPROP_WHEEL: u32 = 71649u32;
pub const DISPID_EXPAND: u32 = 25u32;
pub const DISPID_EXPANDO_BASE: u32 = 3000000u32;
pub const DISPID_EXPANDO_MAX: u32 = 3999999u32;
pub const DISPID_EXPORT: u32 = 7u32;
pub const DISPID_FAVSELECTIONCHANGE: u32 = 1u32;
pub const DISPID_FILEDOWNLOAD: u32 = 270u32;
pub const DISPID_FILTERS: u32 = 1000u32;
pub const DISPID_FLAGS: u32 = 19u32;
pub const DISPID_FORM: u32 = 1000u32;
pub const DISPID_FRAME: u32 = 69536u32;
pub const DISPID_FRAMEBEFORENAVIGATE: u32 = 200u32;
pub const DISPID_FRAMENAVIGATECOMPLETE: u32 = 201u32;
pub const DISPID_FRAMENEWWINDOW: u32 = 204u32;
pub const DISPID_FRAMESCOLLECTION: u32 = 1000u32;
pub const DISPID_FRAMESET: u32 = 1000u32;
pub const DISPID_FRAMESITE: u32 = 68536u32;
pub const DISPID_GENERIC: u32 = 1000u32;
pub const DISPID_GETALWAYSSHOWLOCKSTATE: u32 = 23u32;
pub const DISPID_GETCVLISTDATA: u32 = 93u32;
pub const DISPID_GETCVLISTLOCALDATA: u32 = 94u32;
pub const DISPID_GETDETAILSSTATE: u32 = 19u32;
pub const DISPID_GETEMIELISTDATA: u32 = 95u32;
pub const DISPID_GETEMIELISTLOCALDATA: u32 = 96u32;
pub const DISPID_GETERRORCHAR: u32 = 15u32;
pub const DISPID_GETERRORCODE: u32 = 16u32;
pub const DISPID_GETERRORLINE: u32 = 14u32;
pub const DISPID_GETERRORMSG: u32 = 17u32;
pub const DISPID_GETERRORURL: u32 = 18u32;
pub const DISPID_GETEXPERIMENTALFLAG: u32 = 85u32;
pub const DISPID_GETEXPERIMENTALVALUE: u32 = 87u32;
pub const DISPID_GETNEEDHVSIAUTOLAUNCHFLAG: u32 = 100u32;
pub const DISPID_GETNEEDIEAUTOLAUNCHFLAG: u32 = 89u32;
pub const DISPID_GETOSSKU: u32 = 103u32;
pub const DISPID_GETPERERRSTATE: u32 = 21u32;
pub const DISPID_HASNEEDHVSIAUTOLAUNCHFLAG: u32 = 102u32;
pub const DISPID_HASNEEDIEAUTOLAUNCHFLAG: u32 = 88u32;
pub const DISPID_HEADER: u32 = 1000u32;
pub const DISPID_HEDELEMS: u32 = 1000u32;
pub const DISPID_HISTORY: u32 = 1u32;
pub const DISPID_HISTORYOBJECT: i32 = -5507i32;
pub const DISPID_HR: u32 = 1000u32;
pub const DISPID_HTML5ATTRIBUTESELECTORCI: u32 = 1000u32;
pub const DISPID_HTMLAPP: u32 = 5000u32;
pub const DISPID_HTMLDLG: u32 = 25000u32;
pub const DISPID_HTMLDLGMODEL: u32 = 26000u32;
pub const DISPID_HTMLDOCUMENT: u32 = 1000u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONACTIVATE: u32 = 1044u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONAFTERUPDATE: u32 = 65541u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONBEFOREACTIVATE: u32 = 1047u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONBEFOREDEACTIVATE: u32 = 1034u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONBEFOREEDITFOCUS: u32 = 1027u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONBEFOREUPDATE: u32 = 65540u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONCELLCHANGE: u32 = 65570u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONCLICK: i32 = -600i32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONCONTEXTMENU: u32 = 1023u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONCONTROLSELECT: u32 = 1036u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONDATAAVAILABLE: u32 = 65551u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONDATASETCHANGED: u32 = 65550u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONDATASETCOMPLETE: u32 = 65552u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONDBLCLICK: i32 = -601i32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONDEACTIVATE: u32 = 1045u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONDRAGSTART: u32 = 65547u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONERRORUPDATE: u32 = 65549u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONFOCUSIN: u32 = 1048u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONFOCUSOUT: u32 = 1049u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONHELP: u32 = 65546u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONKEYDOWN: i32 = -602i32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONKEYPRESS: i32 = -603i32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONKEYUP: i32 = -604i32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONMOUSEDOWN: i32 = -605i32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONMOUSEMOVE: i32 = -606i32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONMOUSEOUT: u32 = 65545u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONMOUSEOVER: u32 = 65544u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONMOUSEUP: i32 = -607i32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONMOUSEWHEEL: u32 = 1033u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONPROPERTYCHANGE: u32 = 65555u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONREADYSTATECHANGE: i32 = -609i32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONROWENTER: u32 = 65543u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONROWEXIT: u32 = 65542u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONROWSDELETE: u32 = 65568u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONROWSINSERTED: u32 = 65569u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONSELECTIONCHANGE: u32 = 1037u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONSELECTSTART: u32 = 65548u32;
pub const DISPID_HTMLDOCUMENTEVENTS2_ONSTOP: u32 = 1026u32;
pub const DISPID_HTMLDOCUMENTEVENTS3_ONSTORAGE: u32 = 1057u32;
pub const DISPID_HTMLDOCUMENTEVENTS3_ONSTORAGECOMMIT: u32 = 1058u32;
pub const DISPID_HTMLDOCUMENTEVENTS4_ONMSSITEMODEJUMPLISTITEMREMOVED: u32 = 71666u32;
pub const DISPID_HTMLDOCUMENTEVENTS4_ONMSTHUMBNAILCLICK: u32 = 71657u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONACTIVATE: u32 = 1044u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONAFTERUPDATE: u32 = 65541u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONBEFOREACTIVATE: u32 = 1047u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONBEFOREDEACTIVATE: u32 = 1034u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONBEFOREEDITFOCUS: u32 = 1027u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONBEFOREUPDATE: u32 = 65540u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONCELLCHANGE: u32 = 65570u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONCLICK: i32 = -600i32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONCONTEXTMENU: u32 = 1023u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONCONTROLSELECT: u32 = 1036u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONDATAAVAILABLE: u32 = 65551u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONDATASETCHANGED: u32 = 65550u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONDATASETCOMPLETE: u32 = 65552u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONDBLCLICK: i32 = -601i32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONDEACTIVATE: u32 = 1045u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONDRAGSTART: u32 = 65547u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONERRORUPDATE: u32 = 65549u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONFOCUSIN: u32 = 1048u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONFOCUSOUT: u32 = 1049u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONHELP: u32 = 65546u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONKEYDOWN: i32 = -602i32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONKEYPRESS: i32 = -603i32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONKEYUP: i32 = -604i32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONMOUSEDOWN: i32 = -605i32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONMOUSEMOVE: i32 = -606i32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONMOUSEOUT: u32 = 65545u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONMOUSEOVER: u32 = 65544u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONMOUSEUP: i32 = -607i32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONMOUSEWHEEL: u32 = 1033u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONPROPERTYCHANGE: u32 = 65555u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONREADYSTATECHANGE: i32 = -609i32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONROWENTER: u32 = 65543u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONROWEXIT: u32 = 65542u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONROWSDELETE: u32 = 65568u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONROWSINSERTED: u32 = 65569u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONSELECTIONCHANGE: u32 = 1037u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONSELECTSTART: u32 = 65548u32;
pub const DISPID_HTMLDOCUMENTEVENTS_ONSTOP: u32 = 1026u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONACTIVATE: u32 = 1044u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONAFTERUPDATE: u32 = 65541u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONBEFOREACTIVATE: u32 = 1047u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONBEFORECOPY: u32 = 65566u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONBEFORECUT: u32 = 65565u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONBEFOREDEACTIVATE: u32 = 1034u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONBEFOREPASTE: u32 = 65567u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONBEFOREUPDATE: u32 = 65540u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONCELLCHANGE: u32 = 65570u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONCLICK: i32 = -600i32;
pub const DISPID_HTMLELEMENTEVENTS2_ONCONTEXTMENU: u32 = 1023u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONCONTROLSELECT: u32 = 1036u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONCOPY: u32 = 65563u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONCUT: u32 = 65562u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONDATAAVAILABLE: u32 = 65551u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONDATASETCHANGED: u32 = 65550u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONDATASETCOMPLETE: u32 = 65552u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONDBLCLICK: i32 = -601i32;
pub const DISPID_HTMLELEMENTEVENTS2_ONDEACTIVATE: u32 = 1045u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONDRAG: u32 = 65556u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONDRAGEND: u32 = 65557u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONDRAGENTER: u32 = 65558u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONDRAGLEAVE: u32 = 65560u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONDRAGOVER: u32 = 65559u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONDRAGSTART: u32 = 65547u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONDROP: u32 = 65561u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONERRORUPDATE: u32 = 65549u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONFILTERCHANGE: u32 = 65553u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONFOCUS: u32 = 65537u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONFOCUSIN: u32 = 1048u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONFOCUSOUT: u32 = 1049u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONHELP: u32 = 65546u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONKEYDOWN: i32 = -602i32;
pub const DISPID_HTMLELEMENTEVENTS2_ONKEYPRESS: i32 = -603i32;
pub const DISPID_HTMLELEMENTEVENTS2_ONKEYUP: i32 = -604i32;
pub const DISPID_HTMLELEMENTEVENTS2_ONLAYOUTCOMPLETE: u32 = 1030u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONLOSECAPTURE: u32 = 65554u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONMOUSEDOWN: i32 = -605i32;
pub const DISPID_HTMLELEMENTEVENTS2_ONMOUSEENTER: u32 = 1042u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONMOUSELEAVE: u32 = 1043u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONMOUSEMOVE: i32 = -606i32;
pub const DISPID_HTMLELEMENTEVENTS2_ONMOUSEOUT: u32 = 65545u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONMOUSEOVER: u32 = 65544u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONMOUSEUP: i32 = -607i32;
pub const DISPID_HTMLELEMENTEVENTS2_ONMOUSEWHEEL: u32 = 1033u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONMOVE: u32 = 1035u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONMOVEEND: u32 = 1039u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONMOVESTART: u32 = 1038u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONPAGE: u32 = 1031u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONPASTE: u32 = 65564u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONPROPERTYCHANGE: u32 = 65555u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONREADYSTATECHANGE: i32 = -609i32;
pub const DISPID_HTMLELEMENTEVENTS2_ONRESIZE: u32 = 1016u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONRESIZEEND: u32 = 1041u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONRESIZESTART: u32 = 1040u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONROWENTER: u32 = 65543u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONROWEXIT: u32 = 65542u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONROWSDELETE: u32 = 65568u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONROWSINSERTED: u32 = 65569u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONSCROLL: u32 = 1014u32;
pub const DISPID_HTMLELEMENTEVENTS2_ONSELECTSTART: u32 = 65548u32;
pub const DISPID_HTMLELEMENTEVENTS3_ONOFFLINE: u32 = 1065u32;
pub const DISPID_HTMLELEMENTEVENTS3_ONONLINE: u32 = 1064u32;
pub const DISPID_HTMLELEMENTEVENTS4_ONABORT: u32 = 1000u32;
pub const DISPID_HTMLELEMENTEVENTS4_ONCHANGE: u32 = 1001u32;
pub const DISPID_HTMLELEMENTEVENTS4_ONERROR: u32 = 1002u32;
pub const DISPID_HTMLELEMENTEVENTS4_ONLOAD: u32 = 1003u32;
pub const DISPID_HTMLELEMENTEVENTS4_ONMSCONTENTZOOM: u32 = 71708u32;
pub const DISPID_HTMLELEMENTEVENTS4_ONRESET: u32 = 1015u32;
pub const DISPID_HTMLELEMENTEVENTS4_ONSELECT: u32 = 1006u32;
pub const DISPID_HTMLELEMENTEVENTS4_ONSUBMIT: u32 = 1007u32;
pub const DISPID_HTMLELEMENTEVENTS_ONACTIVATE: u32 = 1044u32;
pub const DISPID_HTMLELEMENTEVENTS_ONAFTERUPDATE: u32 = 65541u32;
pub const DISPID_HTMLELEMENTEVENTS_ONBEFOREACTIVATE: u32 = 1047u32;
pub const DISPID_HTMLELEMENTEVENTS_ONBEFORECOPY: u32 = 65566u32;
pub const DISPID_HTMLELEMENTEVENTS_ONBEFORECUT: u32 = 65565u32;
pub const DISPID_HTMLELEMENTEVENTS_ONBEFOREDEACTIVATE: u32 = 1034u32;
pub const DISPID_HTMLELEMENTEVENTS_ONBEFOREEDITFOCUS: u32 = 1027u32;
pub const DISPID_HTMLELEMENTEVENTS_ONBEFOREPASTE: u32 = 65567u32;
pub const DISPID_HTMLELEMENTEVENTS_ONBEFOREUPDATE: u32 = 65540u32;
pub const DISPID_HTMLELEMENTEVENTS_ONCELLCHANGE: u32 = 65570u32;
pub const DISPID_HTMLELEMENTEVENTS_ONCLICK: i32 = -600i32;
pub const DISPID_HTMLELEMENTEVENTS_ONCONTEXTMENU: u32 = 1023u32;
pub const DISPID_HTMLELEMENTEVENTS_ONCONTROLSELECT: u32 = 1036u32;
pub const DISPID_HTMLELEMENTEVENTS_ONCOPY: u32 = 65563u32;
pub const DISPID_HTMLELEMENTEVENTS_ONCUT: u32 = 65562u32;
pub const DISPID_HTMLELEMENTEVENTS_ONDATAAVAILABLE: u32 = 65551u32;
pub const DISPID_HTMLELEMENTEVENTS_ONDATASETCHANGED: u32 = 65550u32;
pub const DISPID_HTMLELEMENTEVENTS_ONDATASETCOMPLETE: u32 = 65552u32;
pub const DISPID_HTMLELEMENTEVENTS_ONDBLCLICK: i32 = -601i32;
pub const DISPID_HTMLELEMENTEVENTS_ONDEACTIVATE: u32 = 1045u32;
pub const DISPID_HTMLELEMENTEVENTS_ONDRAG: u32 = 65556u32;
pub const DISPID_HTMLELEMENTEVENTS_ONDRAGEND: u32 = 65557u32;
pub const DISPID_HTMLELEMENTEVENTS_ONDRAGENTER: u32 = 65558u32;
pub const DISPID_HTMLELEMENTEVENTS_ONDRAGLEAVE: u32 = 65560u32;
pub const DISPID_HTMLELEMENTEVENTS_ONDRAGOVER: u32 = 65559u32;
pub const DISPID_HTMLELEMENTEVENTS_ONDRAGSTART: u32 = 65547u32;
pub const DISPID_HTMLELEMENTEVENTS_ONDROP: u32 = 65561u32;
pub const DISPID_HTMLELEMENTEVENTS_ONERRORUPDATE: u32 = 65549u32;
pub const DISPID_HTMLELEMENTEVENTS_ONFILTERCHANGE: u32 = 65553u32;
pub const DISPID_HTMLELEMENTEVENTS_ONFOCUS: u32 = 65537u32;
pub const DISPID_HTMLELEMENTEVENTS_ONFOCUSIN: u32 = 1048u32;
pub const DISPID_HTMLELEMENTEVENTS_ONFOCUSOUT: u32 = 1049u32;
pub const DISPID_HTMLELEMENTEVENTS_ONHELP: u32 = 65546u32;
pub const DISPID_HTMLELEMENTEVENTS_ONKEYDOWN: i32 = -602i32;
pub const DISPID_HTMLELEMENTEVENTS_ONKEYPRESS: i32 = -603i32;
pub const DISPID_HTMLELEMENTEVENTS_ONKEYUP: i32 = -604i32;
pub const DISPID_HTMLELEMENTEVENTS_ONLAYOUTCOMPLETE: u32 = 1030u32;
pub const DISPID_HTMLELEMENTEVENTS_ONLOSECAPTURE: u32 = 65554u32;
pub const DISPID_HTMLELEMENTEVENTS_ONMOUSEDOWN: i32 = -605i32;
pub const DISPID_HTMLELEMENTEVENTS_ONMOUSEENTER: u32 = 1042u32;
pub const DISPID_HTMLELEMENTEVENTS_ONMOUSELEAVE: u32 = 1043u32;
pub const DISPID_HTMLELEMENTEVENTS_ONMOUSEMOVE: i32 = -606i32;
pub const DISPID_HTMLELEMENTEVENTS_ONMOUSEOUT: u32 = 65545u32;
pub const DISPID_HTMLELEMENTEVENTS_ONMOUSEOVER: u32 = 65544u32;
pub const DISPID_HTMLELEMENTEVENTS_ONMOUSEUP: i32 = -607i32;
pub const DISPID_HTMLELEMENTEVENTS_ONMOUSEWHEEL: u32 = 1033u32;
pub const DISPID_HTMLELEMENTEVENTS_ONMOVE: u32 = 1035u32;
pub const DISPID_HTMLELEMENTEVENTS_ONMOVEEND: u32 = 1039u32;
pub const DISPID_HTMLELEMENTEVENTS_ONMOVESTART: u32 = 1038u32;
pub const DISPID_HTMLELEMENTEVENTS_ONPAGE: u32 = 1031u32;
pub const DISPID_HTMLELEMENTEVENTS_ONPASTE: u32 = 65564u32;
pub const DISPID_HTMLELEMENTEVENTS_ONPROPERTYCHANGE: u32 = 65555u32;
pub const DISPID_HTMLELEMENTEVENTS_ONREADYSTATECHANGE: i32 = -609i32;
pub const DISPID_HTMLELEMENTEVENTS_ONRESIZE: u32 = 1016u32;
pub const DISPID_HTMLELEMENTEVENTS_ONRESIZEEND: u32 = 1041u32;
pub const DISPID_HTMLELEMENTEVENTS_ONRESIZESTART: u32 = 1040u32;
pub const DISPID_HTMLELEMENTEVENTS_ONROWENTER: u32 = 65543u32;
pub const DISPID_HTMLELEMENTEVENTS_ONROWEXIT: u32 = 65542u32;
pub const DISPID_HTMLELEMENTEVENTS_ONROWSDELETE: u32 = 65568u32;
pub const DISPID_HTMLELEMENTEVENTS_ONROWSINSERTED: u32 = 65569u32;
pub const DISPID_HTMLELEMENTEVENTS_ONSCROLL: u32 = 1014u32;
pub const DISPID_HTMLELEMENTEVENTS_ONSELECTSTART: u32 = 65548u32;
pub const DISPID_HTMLFORMELEMENTEVENTS2_ONRESET: u32 = 1015u32;
pub const DISPID_HTMLFORMELEMENTEVENTS2_ONSUBMIT: u32 = 1007u32;
pub const DISPID_HTMLFORMELEMENTEVENTS_ONRESET: u32 = 1015u32;
pub const DISPID_HTMLFORMELEMENTEVENTS_ONSUBMIT: u32 = 1007u32;
pub const DISPID_HTMLFRAMESITEEVENTS2_ONLOAD: u32 = 1003u32;
pub const DISPID_HTMLFRAMESITEEVENTS_ONLOAD: u32 = 1003u32;
pub const DISPID_HTMLIMGEVENTS2_ONABORT: u32 = 1000u32;
pub const DISPID_HTMLIMGEVENTS2_ONERROR: u32 = 1002u32;
pub const DISPID_HTMLIMGEVENTS2_ONLOAD: u32 = 1003u32;
pub const DISPID_HTMLIMGEVENTS_ONABORT: u32 = 1000u32;
pub const DISPID_HTMLIMGEVENTS_ONERROR: u32 = 1002u32;
pub const DISPID_HTMLIMGEVENTS_ONLOAD: u32 = 1003u32;
pub const DISPID_HTMLINPUTIMAGEEVENTS2_ONABORT: u32 = 1000u32;
pub const DISPID_HTMLINPUTIMAGEEVENTS2_ONERROR: u32 = 1002u32;
pub const DISPID_HTMLINPUTIMAGEEVENTS2_ONLOAD: u32 = 1003u32;
pub const DISPID_HTMLINPUTIMAGEEVENTS_ONABORT: u32 = 1000u32;
pub const DISPID_HTMLINPUTIMAGEEVENTS_ONERROR: u32 = 1002u32;
pub const DISPID_HTMLINPUTIMAGEEVENTS_ONLOAD: u32 = 1003u32;
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS2_ONABORT: u32 = 1000u32;
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS2_ONCHANGE: u32 = 1001u32;
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS2_ONERROR: u32 = 1002u32;
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS2_ONLOAD: u32 = 1003u32;
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS2_ONSELECT: u32 = 1006u32;
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS_ONABORT: u32 = 1000u32;
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS_ONCHANGE: u32 = 1001u32;
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS_ONERROR: u32 = 1002u32;
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS_ONLOAD: u32 = 1003u32;
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS_ONSELECT: u32 = 1006u32;
pub const DISPID_HTMLLINKELEMENTEVENTS2_ONERROR: u32 = 1002u32;
pub const DISPID_HTMLLINKELEMENTEVENTS2_ONLOAD: u32 = 1003u32;
pub const DISPID_HTMLLINKELEMENTEVENTS_ONERROR: u32 = 1002u32;
pub const DISPID_HTMLLINKELEMENTEVENTS_ONLOAD: u32 = 1003u32;
pub const DISPID_HTMLMARQUEEELEMENTEVENTS2_ONBOUNCE: u32 = 1009u32;
pub const DISPID_HTMLMARQUEEELEMENTEVENTS2_ONFINISH: u32 = 1010u32;
pub const DISPID_HTMLMARQUEEELEMENTEVENTS2_ONSTART: u32 = 1011u32;
pub const DISPID_HTMLMARQUEEELEMENTEVENTS_ONBOUNCE: u32 = 1009u32;
pub const DISPID_HTMLMARQUEEELEMENTEVENTS_ONFINISH: u32 = 1010u32;
pub const DISPID_HTMLMARQUEEELEMENTEVENTS_ONSTART: u32 = 1011u32;
pub const DISPID_HTMLNAMESPACEEVENTS_ONREADYSTATECHANGE: i32 = -609i32;
pub const DISPID_HTMLOBJECT: u32 = 66036u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONAFTERUPDATE: u32 = 65541u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONBEFOREUPDATE: u32 = 65540u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONCELLCHANGE: u32 = 65570u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONDATAAVAILABLE: u32 = 65551u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONDATASETCHANGED: u32 = 65550u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONDATASETCOMPLETE: u32 = 65552u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONERROR: u32 = 65555u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONERRORUPDATE: u32 = 65549u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONREADYSTATECHANGE: u32 = 65556u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONROWENTER: u32 = 65543u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONROWEXIT: u32 = 65542u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONROWSDELETE: u32 = 65568u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONROWSINSERTED: u32 = 65569u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONAFTERUPDATE: u32 = 65541u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONBEFOREUPDATE: u32 = 65540u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONCELLCHANGE: u32 = 65570u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONDATAAVAILABLE: u32 = 65551u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONDATASETCHANGED: u32 = 65550u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONDATASETCOMPLETE: u32 = 65552u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONERROR: u32 = 65555u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONERRORUPDATE: u32 = 65549u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONREADYSTATECHANGE: u32 = 65556u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONROWENTER: u32 = 65543u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONROWEXIT: u32 = 65542u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONROWSDELETE: u32 = 65568u32;
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONROWSINSERTED: u32 = 65569u32;
pub const DISPID_HTMLPOPUP: u32 = 27000u32;
pub const DISPID_HTMLSCRIPTEVENTS2_ONERROR: u32 = 1002u32;
pub const DISPID_HTMLSCRIPTEVENTS_ONERROR: u32 = 1002u32;
pub const DISPID_HTMLSELECTELEMENTEVENTS2_ONCHANGE: u32 = 1001u32;
pub const DISPID_HTMLSELECTELEMENTEVENTS_ONCHANGE: u32 = 1001u32;
pub const DISPID_HTMLSELECTION: u32 = 1000u32;
pub const DISPID_HTMLSTYLEELEMENTEVENTS2_ONERROR: u32 = 1002u32;
pub const DISPID_HTMLSTYLEELEMENTEVENTS2_ONLOAD: u32 = 1003u32;
pub const DISPID_HTMLSTYLEELEMENTEVENTS_ONERROR: u32 = 1002u32;
pub const DISPID_HTMLSTYLEELEMENTEVENTS_ONLOAD: u32 = 1003u32;
pub const DISPID_HTMLTEXTCONTAINEREVENTS2_ONCHANGE: u32 = 1001u32;
pub const DISPID_HTMLTEXTCONTAINEREVENTS2_ONSELECT: u32 = 1006u32;
pub const DISPID_HTMLTEXTCONTAINEREVENTS_ONCHANGE: u32 = 1001u32;
pub const DISPID_HTMLTEXTCONTAINEREVENTS_ONSELECT: u32 = 1006u32;
pub const DISPID_HTMLWINDOWEVENTS2_ONAFTERPRINT: u32 = 1025u32;
pub const DISPID_HTMLWINDOWEVENTS2_ONBEFOREPRINT: u32 = 1024u32;
pub const DISPID_HTMLWINDOWEVENTS2_ONBEFOREUNLOAD: u32 = 1017u32;
pub const DISPID_HTMLWINDOWEVENTS2_ONERROR: u32 = 1002u32;
pub const DISPID_HTMLWINDOWEVENTS2_ONFOCUS: u32 = 65537u32;
pub const DISPID_HTMLWINDOWEVENTS2_ONHELP: u32 = 65546u32;
pub const DISPID_HTMLWINDOWEVENTS2_ONLOAD: u32 = 1003u32;
pub const DISPID_HTMLWINDOWEVENTS2_ONRESIZE: u32 = 1016u32;
pub const DISPID_HTMLWINDOWEVENTS2_ONSCROLL: u32 = 1014u32;
pub const DISPID_HTMLWINDOWEVENTS2_ONUNLOAD: u32 = 1008u32;
pub const DISPID_HTMLWINDOWEVENTS3_ONHASHCHANGE: u32 = 1066u32;
pub const DISPID_HTMLWINDOWEVENTS3_ONMESSAGE: u32 = 1067u32;
pub const DISPID_HTMLWINDOWEVENTS_ONAFTERPRINT: u32 = 1025u32;
pub const DISPID_HTMLWINDOWEVENTS_ONBEFOREPRINT: u32 = 1024u32;
pub const DISPID_HTMLWINDOWEVENTS_ONBEFOREUNLOAD: u32 = 1017u32;
pub const DISPID_HTMLWINDOWEVENTS_ONERROR: u32 = 1002u32;
pub const DISPID_HTMLWINDOWEVENTS_ONFOCUS: u32 = 65537u32;
pub const DISPID_HTMLWINDOWEVENTS_ONHELP: u32 = 65546u32;
pub const DISPID_HTMLWINDOWEVENTS_ONLOAD: u32 = 1003u32;
pub const DISPID_HTMLWINDOWEVENTS_ONRESIZE: u32 = 1016u32;
pub const DISPID_HTMLWINDOWEVENTS_ONSCROLL: u32 = 1014u32;
pub const DISPID_HTMLWINDOWEVENTS_ONUNLOAD: u32 = 1008u32;
pub const DISPID_HTMLXMLHTTPREQUESTEVENTS_ONREADYSTATECHANGE: u32 = 1008u32;
pub const DISPID_HTMLXMLHTTPREQUESTEVENTS_ONTIMEOUT: u32 = 1016u32;
pub const DISPID_IBLOCKFORMATS_COUNT: u32 = 1u32;
pub const DISPID_IBLOCKFORMATS_ITEM: u32 = 0u32;
pub const DISPID_IBLOCKFORMATS__NEWENUM: i32 = -4i32;
pub const DISPID_ICANVASGRADIENT_ADDCOLORSTOP: u32 = 1000u32;
pub const DISPID_ICANVASIMAGEDATA_DATA: u32 = 1002u32;
pub const DISPID_ICANVASIMAGEDATA_HEIGHT: u32 = 1001u32;
pub const DISPID_ICANVASIMAGEDATA_WIDTH: u32 = 1000u32;
pub const DISPID_ICANVASPIXELARRAY_LENGTH: u32 = 1000u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_ARC: u32 = 1026u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_ARCTO: u32 = 1027u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_BEGINPATH: u32 = 1028u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_BEZIERCURVETO: u32 = 1029u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_CANVAS: u32 = 1000u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_CLEARRECT: u32 = 1023u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_CLIP: u32 = 1030u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_CLOSEPATH: u32 = 1031u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_CREATEIMAGEDATA: u32 = 1046u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_CREATELINEARGRADIENT: u32 = 1012u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_CREATEPATTERN: u32 = 1014u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_CREATERADIALGRADIENT: u32 = 1013u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_DRAWIMAGE: u32 = 1045u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_FILL: u32 = 1032u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_FILLRECT: u32 = 1024u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_FILLSTYLE: u32 = 1010u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_FILLTEXT: u32 = 1042u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_FONT: u32 = 1039u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_GETIMAGEDATA: u32 = 1047u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_GLOBALALPHA: u32 = 1008u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_GLOBALCOMPOSITEOPERATION: u32 = 1009u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_ISPOINTINPATH: u32 = 1038u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_LINECAP: u32 = 1015u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_LINEJOIN: u32 = 1016u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_LINETO: u32 = 1033u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_LINEWIDTH: u32 = 1017u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_MEASURETEXT: u32 = 1043u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_MITERLIMIT: u32 = 1018u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_MOVETO: u32 = 1034u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_PUTIMAGEDATA: u32 = 1048u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_QUADRATICCURVETO: u32 = 1035u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_RECT: u32 = 1036u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_RESTORE: u32 = 1001u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_ROTATE: u32 = 1003u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_SAVE: u32 = 1002u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_SCALE: u32 = 1004u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_SETTRANSFORM: u32 = 1005u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_SHADOWBLUR: u32 = 1019u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_SHADOWCOLOR: u32 = 1020u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_SHADOWOFFSETX: u32 = 1021u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_SHADOWOFFSETY: u32 = 1022u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_STROKE: u32 = 1037u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_STROKERECT: u32 = 1025u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_STROKESTYLE: u32 = 1011u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_STROKETEXT: u32 = 1044u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_TEXTALIGN: u32 = 1040u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_TEXTBASELINE: u32 = 1041u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_TRANSFORM: u32 = 1006u32;
pub const DISPID_ICANVASRENDERINGCONTEXT2D_TRANSLATE: u32 = 1007u32;
pub const DISPID_ICANVASTEXTMETRICS_WIDTH: u32 = 1000u32;
pub const DISPID_ICLIENTCAPS_ADDCOMPONENTREQUEST: u32 = 19u32;
pub const DISPID_ICLIENTCAPS_AVAILHEIGHT: u32 = 13u32;
pub const DISPID_ICLIENTCAPS_AVAILWIDTH: u32 = 14u32;
pub const DISPID_ICLIENTCAPS_BUFFERDEPTH: u32 = 10u32;
pub const DISPID_ICLIENTCAPS_CLEARCOMPONENTREQUEST: u32 = 21u32;
pub const DISPID_ICLIENTCAPS_COLORDEPTH: u32 = 9u32;
pub const DISPID_ICLIENTCAPS_COMPAREVERSIONS: u32 = 18u32;
pub const DISPID_ICLIENTCAPS_CONNECTIONSPEED: u32 = 7u32;
pub const DISPID_ICLIENTCAPS_CONNECTIONTYPE: u32 = 15u32;
pub const DISPID_ICLIENTCAPS_COOKIEENABLED: u32 = 2u32;
pub const DISPID_ICLIENTCAPS_CPUCLASS: u32 = 3u32;
pub const DISPID_ICLIENTCAPS_DOCOMPONENTREQUEST: u32 = 20u32;
pub const DISPID_ICLIENTCAPS_GETCOMPONENTVERSION: u32 = 17u32;
pub const DISPID_ICLIENTCAPS_HEIGHT: u32 = 12u32;
pub const DISPID_ICLIENTCAPS_ISCOMPONENTINSTALLED: u32 = 16u32;
pub const DISPID_ICLIENTCAPS_JAVAENABLED: u32 = 1u32;
pub const DISPID_ICLIENTCAPS_ONLINE: u32 = 8u32;
pub const DISPID_ICLIENTCAPS_PLATFORM: u32 = 6u32;
pub const DISPID_ICLIENTCAPS_SYSTEMLANGUAGE: u32 = 4u32;
pub const DISPID_ICLIENTCAPS_USERLANGUAGE: u32 = 5u32;
pub const DISPID_ICLIENTCAPS_WIDTH: u32 = 11u32;
pub const DISPID_IDOCUMENTEVENT_CREATEEVENT: u32 = 1108u32;
pub const DISPID_IDOCUMENTRANGE_CREATERANGE: u32 = 1111u32;
pub const DISPID_IDOCUMENTSELECTOR_QUERYSELECTOR: u32 = 1105u32;
pub const DISPID_IDOCUMENTSELECTOR_QUERYSELECTORALL: u32 = 1106u32;
pub const DISPID_IDOCUMENTTRAVERSAL_CREATENODEITERATOR: u32 = 1121u32;
pub const DISPID_IDOCUMENTTRAVERSAL_CREATETREEWALKER: u32 = 1122u32;
pub const DISPID_IDOMBEFOREUNLOADEVENT_RETURNVALUE: u32 = 1376u32;
pub const DISPID_IDOMCLOSEEVENT_INITCLOSEEVENT: u32 = 1529u32;
pub const DISPID_IDOMCLOSEEVENT_WASCLEAN: u32 = 1526u32;
pub const DISPID_IDOMCOMPOSITIONEVENT_DATA: u32 = 1176u32;
pub const DISPID_IDOMCOMPOSITIONEVENT_INITCOMPOSITIONEVENT: u32 = 1177u32;
pub const DISPID_IDOMCOMPOSITIONEVENT_LOCALE: u32 = 1178u32;
pub const DISPID_IDOMCUSTOMEVENT_DETAIL: u32 = 1201u32;
pub const DISPID_IDOMCUSTOMEVENT_INITCUSTOMEVENT: u32 = 1202u32;
pub const DISPID_IDOMDOCUMENTTYPE_ENTITIES: u32 = 1001u32;
pub const DISPID_IDOMDOCUMENTTYPE_INTERNALSUBSET: u32 = 1005u32;
pub const DISPID_IDOMDOCUMENTTYPE_NAME: u32 = 1000u32;
pub const DISPID_IDOMDOCUMENTTYPE_NOTATIONS: u32 = 1002u32;
pub const DISPID_IDOMDOCUMENTTYPE_PUBLICID: u32 = 1003u32;
pub const DISPID_IDOMDOCUMENTTYPE_SYSTEMID: u32 = 1004u32;
pub const DISPID_IDOMDRAGEVENT_DATATRANSFER: u32 = 1401u32;
pub const DISPID_IDOMDRAGEVENT_INITDRAGEVENT: u32 = 1402u32;
pub const DISPID_IDOMEVENT_BUBBLES: u32 = 1001u32;
pub const DISPID_IDOMEVENT_CANCELABLE: u32 = 1002u32;
pub const DISPID_IDOMEVENT_CANCELBUBBLE: u32 = 1014u32;
pub const DISPID_IDOMEVENT_CURRENTTARGET: u32 = 1003u32;
pub const DISPID_IDOMEVENT_DEFAULTPREVENTED: u32 = 1004u32;
pub const DISPID_IDOMEVENT_EVENTPHASE: u32 = 1005u32;
pub const DISPID_IDOMEVENT_INITEVENT: u32 = 1009u32;
pub const DISPID_IDOMEVENT_ISTRUSTED: u32 = 1013u32;
pub const DISPID_IDOMEVENT_PREVENTDEFAULT: u32 = 1010u32;
pub const DISPID_IDOMEVENT_SRCELEMENT: u32 = 1015u32;
pub const DISPID_IDOMEVENT_STOPIMMEDIATEPROPAGATION: u32 = 1012u32;
pub const DISPID_IDOMEVENT_STOPPROPAGATION: u32 = 1011u32;
pub const DISPID_IDOMEVENT_TARGET: u32 = 1006u32;
pub const DISPID_IDOMEVENT_TIMESTAMP: u32 = 1007u32;
pub const DISPID_IDOMEVENT_TYPE: u32 = 1008u32;
pub const DISPID_IDOMEXCEPTION_CODE: u32 = 1000u32;
pub const DISPID_IDOMEXCEPTION_MESSAGE: u32 = 1001u32;
pub const DISPID_IDOMFOCUSEVENT_INITFOCUSEVENT: u32 = 1252u32;
pub const DISPID_IDOMFOCUSEVENT_RELATEDTARGET: u32 = 1251u32;
pub const DISPID_IDOMKEYBOARDEVENT_ALTKEY: u32 = 1155u32;
pub const DISPID_IDOMKEYBOARDEVENT_CHARCODE: u32 = 1161u32;
pub const DISPID_IDOMKEYBOARDEVENT_CTRLKEY: u32 = 1153u32;
pub const DISPID_IDOMKEYBOARDEVENT_GETMODIFIERSTATE: u32 = 1158u32;
pub const DISPID_IDOMKEYBOARDEVENT_IE9_CHAR: u32 = 1163u32;
pub const DISPID_IDOMKEYBOARDEVENT_INITKEYBOARDEVENT: u32 = 1159u32;
pub const DISPID_IDOMKEYBOARDEVENT_KEY: u32 = 1151u32;
pub const DISPID_IDOMKEYBOARDEVENT_KEYCODE: u32 = 1160u32;
pub const DISPID_IDOMKEYBOARDEVENT_LOCALE: u32 = 1164u32;
pub const DISPID_IDOMKEYBOARDEVENT_LOCATION: u32 = 1152u32;
pub const DISPID_IDOMKEYBOARDEVENT_METAKEY: u32 = 1156u32;
pub const DISPID_IDOMKEYBOARDEVENT_REPEAT: u32 = 1157u32;
pub const DISPID_IDOMKEYBOARDEVENT_SHIFTKEY: u32 = 1154u32;
pub const DISPID_IDOMKEYBOARDEVENT_WHICH: u32 = 1162u32;
pub const DISPID_IDOMMESSAGEEVENT_DATA: u32 = 1326u32;
pub const DISPID_IDOMMESSAGEEVENT_INITMESSAGEEVENT: u32 = 1329u32;
pub const DISPID_IDOMMESSAGEEVENT_ORIGIN: u32 = 1327u32;
pub const DISPID_IDOMMESSAGEEVENT_SOURCE: u32 = 1328u32;
pub const DISPID_IDOMMOUSEEVENT_ALTKEY: u32 = 1057u32;
pub const DISPID_IDOMMOUSEEVENT_BUTTON: u32 = 1059u32;
pub const DISPID_IDOMMOUSEEVENT_BUTTONS: u32 = 1063u32;
pub const DISPID_IDOMMOUSEEVENT_CLIENTX: u32 = 1053u32;
pub const DISPID_IDOMMOUSEEVENT_CLIENTY: u32 = 1054u32;
pub const DISPID_IDOMMOUSEEVENT_CTRLKEY: u32 = 1055u32;
pub const DISPID_IDOMMOUSEEVENT_FROMELEMENT: u32 = 1064u32;
pub const DISPID_IDOMMOUSEEVENT_GETMODIFIERSTATE: u32 = 1062u32;
pub const DISPID_IDOMMOUSEEVENT_INITMOUSEEVENT: u32 = 1061u32;
pub const DISPID_IDOMMOUSEEVENT_LAYERX: u32 = 1072u32;
pub const DISPID_IDOMMOUSEEVENT_LAYERY: u32 = 1073u32;
pub const DISPID_IDOMMOUSEEVENT_METAKEY: u32 = 1058u32;
pub const DISPID_IDOMMOUSEEVENT_OFFSETX: u32 = 1068u32;
pub const DISPID_IDOMMOUSEEVENT_OFFSETY: u32 = 1069u32;
pub const DISPID_IDOMMOUSEEVENT_PAGEX: u32 = 1070u32;
pub const DISPID_IDOMMOUSEEVENT_PAGEY: u32 = 1071u32;
pub const DISPID_IDOMMOUSEEVENT_RELATEDTARGET: u32 = 1060u32;
pub const DISPID_IDOMMOUSEEVENT_SCREENX: u32 = 1051u32;
pub const DISPID_IDOMMOUSEEVENT_SCREENY: u32 = 1052u32;
pub const DISPID_IDOMMOUSEEVENT_SHIFTKEY: u32 = 1056u32;
pub const DISPID_IDOMMOUSEEVENT_TOELEMENT: u32 = 1065u32;
pub const DISPID_IDOMMOUSEEVENT_WHICH: u32 = 1074u32;
pub const DISPID_IDOMMOUSEEVENT_X: u32 = 1066u32;
pub const DISPID_IDOMMOUSEEVENT_Y: u32 = 1067u32;
pub const DISPID_IDOMMOUSEWHEELEVENT_INITMOUSEWHEELEVENT: u32 = 1077u32;
pub const DISPID_IDOMMOUSEWHEELEVENT_WHEELDELTA: u32 = 1076u32;
pub const DISPID_IDOMMSANIMATIONEVENT_ANIMATIONNAME: u32 = 1501u32;
pub const DISPID_IDOMMSANIMATIONEVENT_ELAPSEDTIME: u32 = 1502u32;
pub const DISPID_IDOMMSANIMATIONEVENT_INITMSANIMATIONEVENT: u32 = 1503u32;
pub const DISPID_IDOMMSMANIPULATIONEVENT_CURRENTSTATE: u32 = 1527u32;
pub const DISPID_IDOMMSMANIPULATIONEVENT_INITMSMANIPULATIONEVENT: u32 = 1528u32;
pub const DISPID_IDOMMSMANIPULATIONEVENT_LASTSTATE: u32 = 1526u32;
pub const DISPID_IDOMMSTRANSITIONEVENT_ELAPSEDTIME: u32 = 1477u32;
pub const DISPID_IDOMMSTRANSITIONEVENT_INITMSTRANSITIONEVENT: u32 = 1478u32;
pub const DISPID_IDOMMSTRANSITIONEVENT_PROPERTYNAME: u32 = 1476u32;
pub const DISPID_IDOMMUTATIONEVENT_ATTRCHANGE: u32 = 1230u32;
pub const DISPID_IDOMMUTATIONEVENT_ATTRNAME: u32 = 1229u32;
pub const DISPID_IDOMMUTATIONEVENT_INITMUTATIONEVENT: u32 = 1231u32;
pub const DISPID_IDOMMUTATIONEVENT_NEWVALUE: u32 = 1228u32;
pub const DISPID_IDOMMUTATIONEVENT_PREVVALUE: u32 = 1227u32;
pub const DISPID_IDOMMUTATIONEVENT_RELATEDNODE: u32 = 1226u32;
pub const DISPID_IDOMNODEITERATOR_DETACH: u32 = 1012u32;
pub const DISPID_IDOMNODEITERATOR_EXPANDENTITYREFERENCES: u32 = 1003u32;
pub const DISPID_IDOMNODEITERATOR_FILTER: u32 = 1002u32;
pub const DISPID_IDOMNODEITERATOR_NEXTNODE: u32 = 1010u32;
pub const DISPID_IDOMNODEITERATOR_PREVIOUSNODE: u32 = 1011u32;
pub const DISPID_IDOMNODEITERATOR_ROOT: u32 = 1000u32;
pub const DISPID_IDOMNODEITERATOR_WHATTOSHOW: u32 = 1001u32;
pub const DISPID_IDOMPARSERFACTORY_CREATE: u32 = 0u32;
pub const DISPID_IDOMPARSER_PARSEFROMSTRING: u32 = 1000u32;
pub const DISPID_IDOMPROCESSINGINSTRUCTION_DATA: u32 = 1001u32;
pub const DISPID_IDOMPROCESSINGINSTRUCTION_TARGET: u32 = 1000u32;
pub const DISPID_IDOMPROGRESSEVENT_INITPROGRESSEVENT: u32 = 1554u32;
pub const DISPID_IDOMPROGRESSEVENT_LENGTHCOMPUTABLE: u32 = 1551u32;
pub const DISPID_IDOMPROGRESSEVENT_LOADED: u32 = 1552u32;
pub const DISPID_IDOMPROGRESSEVENT_TOTAL: u32 = 1553u32;
pub const DISPID_IDOMSITEMODEEVENT_ACTIONURL: u32 = 1302u32;
pub const DISPID_IDOMSITEMODEEVENT_BUTTONID: u32 = 1301u32;
pub const DISPID_IDOMSTORAGEEVENT_INITSTORAGEEVENT: u32 = 1356u32;
pub const DISPID_IDOMSTORAGEEVENT_KEY: u32 = 1351u32;
pub const DISPID_IDOMSTORAGEEVENT_NEWVALUE: u32 = 1353u32;
pub const DISPID_IDOMSTORAGEEVENT_OLDVALUE: u32 = 1352u32;
pub const DISPID_IDOMSTORAGEEVENT_STORAGEAREA: u32 = 1355u32;
pub const DISPID_IDOMSTORAGEEVENT_URL: u32 = 1354u32;
pub const DISPID_IDOMTEXTEVENT_DATA: u32 = 1126u32;
pub const DISPID_IDOMTEXTEVENT_INITTEXTEVENT: u32 = 1128u32;
pub const DISPID_IDOMTEXTEVENT_INPUTMETHOD: u32 = 1127u32;
pub const DISPID_IDOMTEXTEVENT_LOCALE: u32 = 1129u32;
pub const DISPID_IDOMTREEWALKER_CURRENTNODE: u32 = 1020u32;
pub const DISPID_IDOMTREEWALKER_EXPANDENTITYREFERENCES: u32 = 1003u32;
pub const DISPID_IDOMTREEWALKER_FILTER: u32 = 1002u32;
pub const DISPID_IDOMTREEWALKER_FIRSTCHILD: u32 = 1022u32;
pub const DISPID_IDOMTREEWALKER_LASTCHILD: u32 = 1023u32;
pub const DISPID_IDOMTREEWALKER_NEXTNODE: u32 = 1027u32;
pub const DISPID_IDOMTREEWALKER_NEXTSIBLING: u32 = 1025u32;
pub const DISPID_IDOMTREEWALKER_PARENTNODE: u32 = 1021u32;
pub const DISPID_IDOMTREEWALKER_PREVIOUSNODE: u32 = 1026u32;
pub const DISPID_IDOMTREEWALKER_PREVIOUSSIBLING: u32 = 1024u32;
pub const DISPID_IDOMTREEWALKER_ROOT: u32 = 1000u32;
pub const DISPID_IDOMTREEWALKER_WHATTOSHOW: u32 = 1001u32;
pub const DISPID_IDOMUIEVENT_DETAIL: u32 = 1027u32;
pub const DISPID_IDOMUIEVENT_INITUIEVENT: u32 = 1028u32;
pub const DISPID_IDOMUIEVENT_VIEW: u32 = 1026u32;
pub const DISPID_IDOMWHEELEVENT_DELTAMODE: u32 = 1104u32;
pub const DISPID_IDOMWHEELEVENT_DELTAX: u32 = 1101u32;
pub const DISPID_IDOMWHEELEVENT_DELTAY: u32 = 1102u32;
pub const DISPID_IDOMWHEELEVENT_DELTAZ: u32 = 1103u32;
pub const DISPID_IDOMWHEELEVENT_INITWHEELEVENT: u32 = 1105u32;
pub const DISPID_IDOMXMLSERIALIZERFACTORY_CREATE: u32 = 0u32;
pub const DISPID_IDOMXMLSERIALIZER_SERIALIZETOSTRING: u32 = 1000u32;
pub const DISPID_IE10_ELEMENT: u32 = 66822u32;
pub const DISPID_IE10_ELEMENTBASE: u32 = 66822u32;
pub const DISPID_IE8_ANCHOR: u32 = 1150u32;
pub const DISPID_IE8_AREA: u32 = 1150u32;
pub const DISPID_IE8_ATTR: u32 = 1150u32;
pub const DISPID_IE8_BASE: u32 = 1150u32;
pub const DISPID_IE8_BLOCK: u32 = 1150u32;
pub const DISPID_IE8_BODY: u32 = 1150u32;
pub const DISPID_IE8_COLLECTION: u32 = 1150u32;
pub const DISPID_IE8_ELEMENT: u32 = 66736u32;
pub const DISPID_IE8_ELEMENTBASE: u32 = 66736u32;
pub const DISPID_IE8_ELEMENTMAX: u32 = 66776u32;
pub const DISPID_IE8_EMBED: u32 = 1150u32;
pub const DISPID_IE8_FORM: u32 = 1150u32;
pub const DISPID_IE8_FRAME: u32 = 69656u32;
pub const DISPID_IE8_FRAMESITEBASE: u32 = 69656u32;
pub const DISPID_IE8_HEAD: u32 = 1150u32;
pub const DISPID_IE8_IFRAME: u32 = 69656u32;
pub const DISPID_IE8_IMG: u32 = 1150u32;
pub const DISPID_IE8_INPUT: u32 = 1150u32;
pub const DISPID_IE8_LINK: u32 = 1150u32;
pub const DISPID_IE8_META: u32 = 1150u32;
pub const DISPID_IE8_MOD: u32 = 1150u32;
pub const DISPID_IE8_NAMEDNODEMAP: u32 = 1150u32;
pub const DISPID_IE8_NORMAL_FIRST: u32 = 1150u32;
pub const DISPID_IE8_OBJECT: u32 = 68566u32;
pub const DISPID_IE8_OBJECTBASE: u32 = 68566u32;
pub const DISPID_IE8_PARAM: u32 = 1150u32;
pub const DISPID_IE8_SCRIPT: u32 = 1150u32;
pub const DISPID_IE8_SELECT: u32 = 1150u32;
pub const DISPID_IE8_STYLE: u32 = 1150u32;
pub const DISPID_IE9EVENTS_ABORT: u32 = 1000u32;
pub const DISPID_IE9EVENTS_ACTIVATE: u32 = 1044u32;
pub const DISPID_IE9EVENTS_ADDSOURCEBUFFER: u32 = 71746u32;
pub const DISPID_IE9EVENTS_ADDTRACK: u32 = 71736u32;
pub const DISPID_IE9EVENTS_AFTERPRINT: u32 = 1025u32;
pub const DISPID_IE9EVENTS_ANIMATIONEND: u32 = 71712u32;
pub const DISPID_IE9EVENTS_ANIMATIONITERATION: u32 = 71713u32;
pub const DISPID_IE9EVENTS_ANIMATIONSTART: u32 = 71711u32;
pub const DISPID_IE9EVENTS_BEFOREACTIVATE: u32 = 1047u32;
pub const DISPID_IE9EVENTS_BEFORECOPY: u32 = 65566u32;
pub const DISPID_IE9EVENTS_BEFORECUT: u32 = 65565u32;
pub const DISPID_IE9EVENTS_BEFOREDEACTIVATE: u32 = 1034u32;
pub const DISPID_IE9EVENTS_BEFOREPASTE: u32 = 65567u32;
pub const DISPID_IE9EVENTS_BEFOREPRINT: u32 = 1024u32;
pub const DISPID_IE9EVENTS_BEFOREUNLOAD: u32 = 1017u32;
pub const DISPID_IE9EVENTS_BLOCKED: u32 = 71726u32;
pub const DISPID_IE9EVENTS_BOUNCE: u32 = 1009u32;
pub const DISPID_IE9EVENTS_CACHED: u32 = 71721u32;
pub const DISPID_IE9EVENTS_CANPLAY: u32 = 71670u32;
pub const DISPID_IE9EVENTS_CANPLAYTHROUGH: u32 = 71671u32;
pub const DISPID_IE9EVENTS_CHANGE: u32 = 1001u32;
pub const DISPID_IE9EVENTS_CHECKING: u32 = 71717u32;
pub const DISPID_IE9EVENTS_CLICK: i32 = -600i32;
pub const DISPID_IE9EVENTS_CLOSE: u32 = 71716u32;
pub const DISPID_IE9EVENTS_COMPASSNEEDSCALIBRATION: u32 = 71782u32;
pub const DISPID_IE9EVENTS_COMPLETE: u32 = 71727u32;
pub const DISPID_IE9EVENTS_COMPOSITIONEND: u32 = 71660u32;
pub const DISPID_IE9EVENTS_COMPOSITIONSTART: u32 = 71658u32;
pub const DISPID_IE9EVENTS_COMPOSITIONUPDATE: u32 = 71659u32;
pub const DISPID_IE9EVENTS_CONTEXTMENU: u32 = 1023u32;
pub const DISPID_IE9EVENTS_COPY: u32 = 65563u32;
pub const DISPID_IE9EVENTS_CUECHANGE: u32 = 71729u32;
pub const DISPID_IE9EVENTS_CUT: u32 = 65562u32;
pub const DISPID_IE9EVENTS_DBLCLICK: i32 = -601i32;
pub const DISPID_IE9EVENTS_DEACTIVATE: u32 = 1045u32;
pub const DISPID_IE9EVENTS_DEVICEMOTION: u32 = 71774u32;
pub const DISPID_IE9EVENTS_DEVICEORIENTATION: u32 = 71773u32;
pub const DISPID_IE9EVENTS_DOMATTRMODIFIED: u32 = 71661u32;
pub const DISPID_IE9EVENTS_DOMCHARACTERDATAMODIFIED: u32 = 71664u32;
pub const DISPID_IE9EVENTS_DOMCONTENTLOADED: u32 = 71662u32;
pub const DISPID_IE9EVENTS_DOMNODEINSERTED: u32 = 71667u32;
pub const DISPID_IE9EVENTS_DOMNODEREMOVED: u32 = 71668u32;
pub const DISPID_IE9EVENTS_DOMSUBTREEMODIFIED: u32 = 71669u32;
pub const DISPID_IE9EVENTS_DOWNLOADING: u32 = 71719u32;
pub const DISPID_IE9EVENTS_DRAG: u32 = 65556u32;
pub const DISPID_IE9EVENTS_DRAGEND: u32 = 65557u32;
pub const DISPID_IE9EVENTS_DRAGENTER: u32 = 65558u32;
pub const DISPID_IE9EVENTS_DRAGLEAVE: u32 = 65560u32;
pub const DISPID_IE9EVENTS_DRAGOVER: u32 = 65559u32;
pub const DISPID_IE9EVENTS_DRAGSTART: u32 = 65547u32;
pub const DISPID_IE9EVENTS_DROP: u32 = 65561u32;
pub const DISPID_IE9EVENTS_DURATIONCHANGE: u32 = 71672u32;
pub const DISPID_IE9EVENTS_EMPTIED: u32 = 71673u32;
pub const DISPID_IE9EVENTS_ENDED: u32 = 71674u32;
pub const DISPID_IE9EVENTS_ENTER: u32 = 71730u32;
pub const DISPID_IE9EVENTS_ERROR: u32 = 1002u32;
pub const DISPID_IE9EVENTS_EXIT: u32 = 71731u32;
pub const DISPID_IE9EVENTS_FINISH: u32 = 1010u32;
pub const DISPID_IE9EVENTS_FOCUS: u32 = 65537u32;
pub const DISPID_IE9EVENTS_FOCUSIN: u32 = 1048u32;
pub const DISPID_IE9EVENTS_FOCUSOUT: u32 = 1049u32;
pub const DISPID_IE9EVENTS_HASHCHANGE: u32 = 1066u32;
pub const DISPID_IE9EVENTS_HELP: u32 = 65546u32;
pub const DISPID_IE9EVENTS_INPUT: u32 = 71663u32;
pub const DISPID_IE9EVENTS_INVALID: u32 = 71724u32;
pub const DISPID_IE9EVENTS_KEYDOWN: i32 = -602i32;
pub const DISPID_IE9EVENTS_KEYPRESS: i32 = -603i32;
pub const DISPID_IE9EVENTS_KEYUP: i32 = -604i32;
pub const DISPID_IE9EVENTS_LOAD: u32 = 1003u32;
pub const DISPID_IE9EVENTS_LOADEDDATA: u32 = 71675u32;
pub const DISPID_IE9EVENTS_LOADEDMETADATA: u32 = 71676u32;
pub const DISPID_IE9EVENTS_LOADEND: u32 = 71723u32;
pub const DISPID_IE9EVENTS_LOADSTART: u32 = 71677u32;
pub const DISPID_IE9EVENTS_MESSAGE: u32 = 1067u32;
pub const DISPID_IE9EVENTS_MOUSEDOWN: i32 = -605i32;
pub const DISPID_IE9EVENTS_MOUSEENTER: u32 = 1042u32;
pub const DISPID_IE9EVENTS_MOUSELEAVE: u32 = 1043u32;
pub const DISPID_IE9EVENTS_MOUSEMOVE: i32 = -606i32;
pub const DISPID_IE9EVENTS_MOUSEOUT: u32 = 65545u32;
pub const DISPID_IE9EVENTS_MOUSEOVER: u32 = 65544u32;
pub const DISPID_IE9EVENTS_MOUSEUP: i32 = -607i32;
pub const DISPID_IE9EVENTS_MOUSEWHEEL: u32 = 1033u32;
pub const DISPID_IE9EVENTS_MSBEFOREEDITFOCUS: u32 = 1027u32;
pub const DISPID_IE9EVENTS_MSCANDIDATEWINDOWHIDE: u32 = 71779u32;
pub const DISPID_IE9EVENTS_MSCANDIDATEWINDOWSHOW: u32 = 71777u32;
pub const DISPID_IE9EVENTS_MSCANDIDATEWINDOWUPDATE: u32 = 71778u32;
pub const DISPID_IE9EVENTS_MSCONTENTZOOM: u32 = 71708u32;
pub const DISPID_IE9EVENTS_MSCONTROLRESIZEEND: u32 = 1041u32;
pub const DISPID_IE9EVENTS_MSCONTROLRESIZESTART: u32 = 1040u32;
pub const DISPID_IE9EVENTS_MSCONTROLSELECT: u32 = 1036u32;
pub const DISPID_IE9EVENTS_MSELEMENTRESIZE: u32 = 71742u32;
pub const DISPID_IE9EVENTS_MSFULLSCREENCHANGE: u32 = 71740u32;
pub const DISPID_IE9EVENTS_MSFULLSCREENERROR: u32 = 71741u32;
pub const DISPID_IE9EVENTS_MSGESTURECHANGE: u32 = 71700u32;
pub const DISPID_IE9EVENTS_MSGESTUREDOUBLETAP: u32 = 71704u32;
pub const DISPID_IE9EVENTS_MSGESTUREEND: u32 = 71701u32;
pub const DISPID_IE9EVENTS_MSGESTUREHOLD: u32 = 71702u32;
pub const DISPID_IE9EVENTS_MSGESTURESTART: u32 = 71699u32;
pub const DISPID_IE9EVENTS_MSGESTURETAP: u32 = 71703u32;
pub const DISPID_IE9EVENTS_MSGOTPOINTERCAPTURE: u32 = 71707u32;
pub const DISPID_IE9EVENTS_MSHOLDVISUAL: u32 = 71738u32;
pub const DISPID_IE9EVENTS_MSINERTIASTART: u32 = 71705u32;
pub const DISPID_IE9EVENTS_MSKEYADDED: u32 = 71751u32;
pub const DISPID_IE9EVENTS_MSKEYERROR: u32 = 71750u32;
pub const DISPID_IE9EVENTS_MSKEYMESSAGE: u32 = 71749u32;
pub const DISPID_IE9EVENTS_MSLOSTPOINTERCAPTURE: u32 = 71706u32;
pub const DISPID_IE9EVENTS_MSMANIPULATIONSTATECHANGED: u32 = 71714u32;
pub const DISPID_IE9EVENTS_MSNEEDKEY: u32 = 71748u32;
pub const DISPID_IE9EVENTS_MSORIENTATIONCHANGE: u32 = 71772u32;
pub const DISPID_IE9EVENTS_MSPOINTERCANCEL: u32 = 71695u32;
pub const DISPID_IE9EVENTS_MSPOINTERDOWN: u32 = 71690u32;
pub const DISPID_IE9EVENTS_MSPOINTERENTER: u32 = 71769u32;
pub const DISPID_IE9EVENTS_MSPOINTERHOVER: u32 = 71696u32;
pub const DISPID_IE9EVENTS_MSPOINTERLEAVE: u32 = 71770u32;
pub const DISPID_IE9EVENTS_MSPOINTERMOVE: u32 = 71691u32;
pub const DISPID_IE9EVENTS_MSPOINTEROUT: u32 = 71694u32;
pub const DISPID_IE9EVENTS_MSPOINTEROVER: u32 = 71693u32;
pub const DISPID_IE9EVENTS_MSPOINTERUP: u32 = 71692u32;
pub const DISPID_IE9EVENTS_MSREGIONUPDATE: u32 = 71733u32;
pub const DISPID_IE9EVENTS_MSSITEMODEJUMPLISTITEMREMOVED: u32 = 71666u32;
pub const DISPID_IE9EVENTS_MSSITEPINNED: u32 = 71771u32;
pub const DISPID_IE9EVENTS_MSTHUMBNAILCLICK: u32 = 71657u32;
pub const DISPID_IE9EVENTS_MSVIDEOFORMATCHANGED: u32 = 71735u32;
pub const DISPID_IE9EVENTS_MSVIDEOFRAMESTEPCOMPLETED: u32 = 71737u32;
pub const DISPID_IE9EVENTS_MSVIDEOOPTIMALLAYOUTCHANGED: u32 = 71739u32;
pub const DISPID_IE9EVENTS_MSWEBVIEWCONTAINSFULLSCREENELEMENTCHANGED: u32 = 71783u32;
pub const DISPID_IE9EVENTS_MSWEBVIEWCONTENTLOADING: u32 = 71753u32;
pub const DISPID_IE9EVENTS_MSWEBVIEWDOMCONTENTLOADED: u32 = 71752u32;
pub const DISPID_IE9EVENTS_MSWEBVIEWFRAMECONTENTLOADING: u32 = 71757u32;
pub const DISPID_IE9EVENTS_MSWEBVIEWFRAMEDOMCONTENTLOADED: u32 = 71756u32;
pub const DISPID_IE9EVENTS_MSWEBVIEWFRAMENAVIGATIONCOMPLETED: u32 = 71759u32;
pub const DISPID_IE9EVENTS_MSWEBVIEWFRAMENAVIGATIONSTARTING: u32 = 71758u32;
pub const DISPID_IE9EVENTS_MSWEBVIEWLONGRUNNINGSCRIPTDETECTED: u32 = 71763u32;
pub const DISPID_IE9EVENTS_MSWEBVIEWNAVIGATIONCOMPLETED: u32 = 71755u32;
pub const DISPID_IE9EVENTS_MSWEBVIEWNAVIGATIONSTARTING: u32 = 71754u32;
pub const DISPID_IE9EVENTS_MSWEBVIEWSCRIPTNOTIFY: u32 = 71760u32;
pub const DISPID_IE9EVENTS_MSWEBVIEWUNSAFECONTENTWARNINGDISPLAYING: u32 = 71762u32;
pub const DISPID_IE9EVENTS_MSWEBVIEWUNVIEWABLECONTENTIDENTIFIED: u32 = 71761u32;
pub const DISPID_IE9EVENTS_NOUPDATE: u32 = 71718u32;
pub const DISPID_IE9EVENTS_OBSOLETE: u32 = 71722u32;
pub const DISPID_IE9EVENTS_OFFLINE: u32 = 1065u32;
pub const DISPID_IE9EVENTS_ONLINE: u32 = 1064u32;
pub const DISPID_IE9EVENTS_OPEN: u32 = 71715u32;
pub const DISPID_IE9EVENTS_ORIENTATIONCHANGE: u32 = 71795u32;
pub const DISPID_IE9EVENTS_PAGEHIDE: u32 = 71776u32;
pub const DISPID_IE9EVENTS_PAGESHOW: u32 = 71775u32;
pub const DISPID_IE9EVENTS_PASTE: u32 = 65564u32;
pub const DISPID_IE9EVENTS_PAUSE: u32 = 71678u32;
pub const DISPID_IE9EVENTS_PLAY: u32 = 71679u32;
pub const DISPID_IE9EVENTS_PLAYING: u32 = 71680u32;
pub const DISPID_IE9EVENTS_POPSTATE: u32 = 71728u32;
pub const DISPID_IE9EVENTS_PROGRESS: u32 = 71681u32;
pub const DISPID_IE9EVENTS_RATECHANGE: u32 = 71682u32;
pub const DISPID_IE9EVENTS_READYSTATECHANGE: i32 = -609i32;
pub const DISPID_IE9EVENTS_REMOVESOURCEBUFFER: u32 = 71747u32;
pub const DISPID_IE9EVENTS_REMOVETRACK: u32 = 71781u32;
pub const DISPID_IE9EVENTS_RESET: u32 = 1015u32;
pub const DISPID_IE9EVENTS_RESIZE: u32 = 1016u32;
pub const DISPID_IE9EVENTS_SCROLL: u32 = 1014u32;
pub const DISPID_IE9EVENTS_SEEKED: u32 = 71683u32;
pub const DISPID_IE9EVENTS_SEEKING: u32 = 71684u32;
pub const DISPID_IE9EVENTS_SELECT: u32 = 1006u32;
pub const DISPID_IE9EVENTS_SELECTIONCHANGE: u32 = 1037u32;
pub const DISPID_IE9EVENTS_SELECTSTART: u32 = 65548u32;
pub const DISPID_IE9EVENTS_SOURCECLOSE: u32 = 71744u32;
pub const DISPID_IE9EVENTS_SOURCEENDED: u32 = 71745u32;
pub const DISPID_IE9EVENTS_SOURCEOPEN: u32 = 71743u32;
pub const DISPID_IE9EVENTS_STALLED: u32 = 71685u32;
pub const DISPID_IE9EVENTS_START: u32 = 1011u32;
pub const DISPID_IE9EVENTS_STOP: u32 = 1026u32;
pub const DISPID_IE9EVENTS_STORAGE: u32 = 1057u32;
pub const DISPID_IE9EVENTS_STORAGECOMMIT: u32 = 1058u32;
pub const DISPID_IE9EVENTS_SUBMIT: u32 = 1007u32;
pub const DISPID_IE9EVENTS_SUCCESS: u32 = 71725u32;
pub const DISPID_IE9EVENTS_SUSPEND: u32 = 71686u32;
pub const DISPID_IE9EVENTS_SVGABORT: u32 = 71652u32;
pub const DISPID_IE9EVENTS_SVGERROR: u32 = 71653u32;
pub const DISPID_IE9EVENTS_SVGLOAD: u32 = 71650u32;
pub const DISPID_IE9EVENTS_SVGRESIZE: u32 = 71654u32;
pub const DISPID_IE9EVENTS_SVGSCROLL: u32 = 71655u32;
pub const DISPID_IE9EVENTS_SVGUNLOAD: u32 = 71651u32;
pub const DISPID_IE9EVENTS_SVGZOOM: u32 = 71656u32;
pub const DISPID_IE9EVENTS_TEXTINPUT: u32 = 71665u32;
pub const DISPID_IE9EVENTS_TIMEOUT: u32 = 0u32;
pub const DISPID_IE9EVENTS_TIMEUPDATE: u32 = 71687u32;
pub const DISPID_IE9EVENTS_TOUCHCANCEL: u32 = 71787u32;
pub const DISPID_IE9EVENTS_TOUCHEND: u32 = 71785u32;
pub const DISPID_IE9EVENTS_TOUCHMOVE: u32 = 71786u32;
pub const DISPID_IE9EVENTS_TOUCHSTART: u32 = 71784u32;
pub const DISPID_IE9EVENTS_TRANSITIONEND: u32 = 71710u32;
pub const DISPID_IE9EVENTS_TRANSITIONSTART: u32 = 71709u32;
pub const DISPID_IE9EVENTS_UNLOAD: u32 = 1008u32;
pub const DISPID_IE9EVENTS_UPDATE: u32 = 71767u32;
pub const DISPID_IE9EVENTS_UPDATEEND: u32 = 71768u32;
pub const DISPID_IE9EVENTS_UPDATEREADY: u32 = 71720u32;
pub const DISPID_IE9EVENTS_UPDATESTART: u32 = 71766u32;
pub const DISPID_IE9EVENTS_UPGRADENEEDED: u32 = 71734u32;
pub const DISPID_IE9EVENTS_VISIBILITYCHANGE: u32 = 71732u32;
pub const DISPID_IE9EVENTS_VOLUMECHANGE: u32 = 71688u32;
pub const DISPID_IE9EVENTS_WAITING: u32 = 71689u32;
pub const DISPID_IE9EVENTS_WEBGLCONTEXTCREATIONERROR: u32 = 71792u32;
pub const DISPID_IE9EVENTS_WEBGLCONTEXTLOST: u32 = 71764u32;
pub const DISPID_IE9EVENTS_WEBGLCONTEXTRESTORED: u32 = 71765u32;
pub const DISPID_IE9EVENTS_WHEEL: u32 = 71649u32;
pub const DISPID_IE9_ELEMENT: u32 = 66786u32;
pub const DISPID_IE9_ELEMENTBASE: u32 = 66786u32;
pub const DISPID_IE9_ELEMENTMAX: u32 = 66821u32;
pub const DISPID_IELEMENTSELECTOR_QUERYSELECTOR: u32 = 66650u32;
pub const DISPID_IELEMENTSELECTOR_QUERYSELECTORALL: u32 = 66651u32;
pub const DISPID_IELEMENTTRAVERSAL_CHILDELEMENTCOUNT: u32 = 66812u32;
pub const DISPID_IELEMENTTRAVERSAL_FIRSTELEMENTCHILD: u32 = 66808u32;
pub const DISPID_IELEMENTTRAVERSAL_LASTELEMENTCHILD: u32 = 66809u32;
pub const DISPID_IELEMENTTRAVERSAL_NEXTELEMENTSIBLING: u32 = 66811u32;
pub const DISPID_IELEMENTTRAVERSAL_PREVIOUSELEMENTSIBLING: u32 = 66810u32;
pub const DISPID_IEVENTEXCEPTION_CODE: u32 = 1000u32;
pub const DISPID_IEVENTEXCEPTION_MESSAGE: u32 = 1001u32;
pub const DISPID_IEVENTTARGET_ADDEVENTLISTENER: u32 = 66046u32;
pub const DISPID_IEVENTTARGET_DISPATCHEVENT: u32 = 66048u32;
pub const DISPID_IEVENTTARGET_REMOVEEVENTLISTENER: u32 = 66047u32;
pub const DISPID_IFONTNAMES_COUNT: u32 = 1u32;
pub const DISPID_IFONTNAMES_ITEM: u32 = 0u32;
pub const DISPID_IFONTNAMES__NEWENUM: i32 = -4i32;
pub const DISPID_IFRAME: u32 = 69536u32;
pub const DISPID_IGETSVGDOCUMENT_GETSVGDOCUMENT: u32 = 65615u32;
pub const DISPID_IHTCATTACHBEHAVIOR2_FIREEVENT: u32 = 0u32;
pub const DISPID_IHTCATTACHBEHAVIOR_DETACHEVENT: u32 = 66036u32;
pub const DISPID_IHTCATTACHBEHAVIOR_FIREEVENT: u32 = 0u32;
pub const DISPID_IHTCDEFAULTDISPATCH_CREATEEVENTOBJECT: u32 = 70680u32;
pub const DISPID_IHTCDEFAULTDISPATCH_DEFAULTS: u32 = 70701u32;
pub const DISPID_IHTCDEFAULTDISPATCH_DOCUMENT: u32 = 70678u32;
pub const DISPID_IHTCDEFAULTDISPATCH_ELEMENT: u32 = 70679u32;
pub const DISPID_IHTCDESCBEHAVIOR_NAME: u32 = 66037u32;
pub const DISPID_IHTCDESCBEHAVIOR_URN: u32 = 66036u32;
pub const DISPID_IHTCEVENTBEHAVIOR_FIRE: u32 = 66036u32;
pub const DISPID_IHTCPROPERTYBEHAVIOR_FIRECHANGE: u32 = 66036u32;
pub const DISPID_IHTCPROPERTYBEHAVIOR_VALUE: u32 = 70677u32;
pub const DISPID_IHTMLANCHORELEMENT2_CHARSET: u32 = 1023u32;
pub const DISPID_IHTMLANCHORELEMENT2_COORDS: u32 = 1024u32;
pub const DISPID_IHTMLANCHORELEMENT2_HREFLANG: u32 = 1025u32;
pub const DISPID_IHTMLANCHORELEMENT2_SHAPE: u32 = 1026u32;
pub const DISPID_IHTMLANCHORELEMENT2_TYPE: u32 = 1027u32;
pub const DISPID_IHTMLANCHORELEMENT3_IE8_COORDS: u32 = 1152u32;
pub const DISPID_IHTMLANCHORELEMENT3_IE8_HREF: u32 = 1153u32;
pub const DISPID_IHTMLANCHORELEMENT3_IE8_SHAPE: u32 = 1151u32;
pub const DISPID_IHTMLANCHORELEMENT_ACCESSKEY: u32 = 67541u32;
pub const DISPID_IHTMLANCHORELEMENT_BLUR: u32 = 67538u32;
pub const DISPID_IHTMLANCHORELEMENT_FOCUS: u32 = 67536u32;
pub const DISPID_IHTMLANCHORELEMENT_HASH: u32 = 1018u32;
pub const DISPID_IHTMLANCHORELEMENT_HOST: u32 = 1012u32;
pub const DISPID_IHTMLANCHORELEMENT_HOSTNAME: u32 = 1013u32;
pub const DISPID_IHTMLANCHORELEMENT_HREF: u32 = 0u32;
pub const DISPID_IHTMLANCHORELEMENT_METHODS: u32 = 1008u32;
pub const DISPID_IHTMLANCHORELEMENT_MIMETYPE: u32 = 1030u32;
pub const DISPID_IHTMLANCHORELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLANCHORELEMENT_NAMEPROP: u32 = 1032u32;
pub const DISPID_IHTMLANCHORELEMENT_ONBLUR: u32 = 71551u32;
pub const DISPID_IHTMLANCHORELEMENT_ONFOCUS: u32 = 71550u32;
pub const DISPID_IHTMLANCHORELEMENT_PATHNAME: u32 = 1014u32;
pub const DISPID_IHTMLANCHORELEMENT_PORT: u32 = 1015u32;
pub const DISPID_IHTMLANCHORELEMENT_PROTOCOL: u32 = 1016u32;
pub const DISPID_IHTMLANCHORELEMENT_PROTOCOLLONG: u32 = 1031u32;
pub const DISPID_IHTMLANCHORELEMENT_REL: u32 = 1005u32;
pub const DISPID_IHTMLANCHORELEMENT_REV: u32 = 1006u32;
pub const DISPID_IHTMLANCHORELEMENT_SEARCH: u32 = 1017u32;
pub const DISPID_IHTMLANCHORELEMENT_TABINDEX: u32 = 65551u32;
pub const DISPID_IHTMLANCHORELEMENT_TARGET: u32 = 1003u32;
pub const DISPID_IHTMLANCHORELEMENT_URN: u32 = 1007u32;
pub const DISPID_IHTMLAPPBEHAVIOR2_CONTEXTMENU: u32 = 5014u32;
pub const DISPID_IHTMLAPPBEHAVIOR2_INNERBORDER: u32 = 5015u32;
pub const DISPID_IHTMLAPPBEHAVIOR2_SCROLL: u32 = 5016u32;
pub const DISPID_IHTMLAPPBEHAVIOR2_SCROLLFLAT: u32 = 5017u32;
pub const DISPID_IHTMLAPPBEHAVIOR2_SELECTION: u32 = 5018u32;
pub const DISPID_IHTMLAPPBEHAVIOR3_NAVIGABLE: u32 = 5019u32;
pub const DISPID_IHTMLAPPBEHAVIOR_APPLICATIONNAME: u32 = 5000u32;
pub const DISPID_IHTMLAPPBEHAVIOR_BORDER: u32 = 5007u32;
pub const DISPID_IHTMLAPPBEHAVIOR_BORDERSTYLE: u32 = 5008u32;
pub const DISPID_IHTMLAPPBEHAVIOR_CAPTION: u32 = 5010u32;
pub const DISPID_IHTMLAPPBEHAVIOR_COMMANDLINE: u32 = 5013u32;
pub const DISPID_IHTMLAPPBEHAVIOR_ICON: u32 = 5002u32;
pub const DISPID_IHTMLAPPBEHAVIOR_MAXIMIZEBUTTON: u32 = 5006u32;
pub const DISPID_IHTMLAPPBEHAVIOR_MINIMIZEBUTTON: u32 = 5005u32;
pub const DISPID_IHTMLAPPBEHAVIOR_SHOWINTASKBAR: u32 = 5012u32;
pub const DISPID_IHTMLAPPBEHAVIOR_SINGLEINSTANCE: u32 = 5003u32;
pub const DISPID_IHTMLAPPBEHAVIOR_SYSMENU: u32 = 5009u32;
pub const DISPID_IHTMLAPPBEHAVIOR_VERSION: u32 = 5001u32;
pub const DISPID_IHTMLAPPBEHAVIOR_WINDOWSTATE: u32 = 5011u32;
pub const DISPID_IHTMLAPPLICATIONCACHE_ABORT: u32 = 1004u32;
pub const DISPID_IHTMLAPPLICATIONCACHE_ONCACHED: u32 = 71721u32;
pub const DISPID_IHTMLAPPLICATIONCACHE_ONCHECKING: u32 = 71717u32;
pub const DISPID_IHTMLAPPLICATIONCACHE_ONDOWNLOADING: u32 = 71719u32;
pub const DISPID_IHTMLAPPLICATIONCACHE_ONERROR: u32 = 71565u32;
pub const DISPID_IHTMLAPPLICATIONCACHE_ONNOUPDATE: u32 = 71718u32;
pub const DISPID_IHTMLAPPLICATIONCACHE_ONOBSOLETE: u32 = 71722u32;
pub const DISPID_IHTMLAPPLICATIONCACHE_ONPROGRESS: u32 = 71681u32;
pub const DISPID_IHTMLAPPLICATIONCACHE_ONUPDATEREADY: u32 = 71720u32;
pub const DISPID_IHTMLAPPLICATIONCACHE_STATUS: u32 = 1001u32;
pub const DISPID_IHTMLAPPLICATIONCACHE_SWAPCACHE: u32 = 1003u32;
pub const DISPID_IHTMLAPPLICATIONCACHE_UPDATE: u32 = 1002u32;
pub const DISPID_IHTMLAREAELEMENT2_IE8_COORDS: u32 = 1152u32;
pub const DISPID_IHTMLAREAELEMENT2_IE8_HREF: u32 = 1153u32;
pub const DISPID_IHTMLAREAELEMENT2_IE8_SHAPE: u32 = 1151u32;
pub const DISPID_IHTMLAREAELEMENT_ALT: u32 = 1005u32;
pub const DISPID_IHTMLAREAELEMENT_BLUR: u32 = 67538u32;
pub const DISPID_IHTMLAREAELEMENT_COORDS: u32 = 1002u32;
pub const DISPID_IHTMLAREAELEMENT_FOCUS: u32 = 67536u32;
pub const DISPID_IHTMLAREAELEMENT_HASH: u32 = 1013u32;
pub const DISPID_IHTMLAREAELEMENT_HOST: u32 = 1007u32;
pub const DISPID_IHTMLAREAELEMENT_HOSTNAME: u32 = 1008u32;
pub const DISPID_IHTMLAREAELEMENT_HREF: u32 = 0u32;
pub const DISPID_IHTMLAREAELEMENT_NOHREF: u32 = 1006u32;
pub const DISPID_IHTMLAREAELEMENT_ONBLUR: u32 = 71551u32;
pub const DISPID_IHTMLAREAELEMENT_ONFOCUS: u32 = 71550u32;
pub const DISPID_IHTMLAREAELEMENT_PATHNAME: u32 = 1009u32;
pub const DISPID_IHTMLAREAELEMENT_PORT: u32 = 1010u32;
pub const DISPID_IHTMLAREAELEMENT_PROTOCOL: u32 = 1011u32;
pub const DISPID_IHTMLAREAELEMENT_SEARCH: u32 = 1012u32;
pub const DISPID_IHTMLAREAELEMENT_SHAPE: u32 = 1001u32;
pub const DISPID_IHTMLAREAELEMENT_TABINDEX: u32 = 65551u32;
pub const DISPID_IHTMLAREAELEMENT_TARGET: u32 = 1004u32;
pub const DISPID_IHTMLAREASCOLLECTION2_URNS: u32 = 1505u32;
pub const DISPID_IHTMLAREASCOLLECTION3_NAMEDITEM: u32 = 1506u32;
pub const DISPID_IHTMLAREASCOLLECTION4_IE8_ITEM: u32 = 1152u32;
pub const DISPID_IHTMLAREASCOLLECTION4_IE8_LENGTH: u32 = 1150u32;
pub const DISPID_IHTMLAREASCOLLECTION4_IE8_NAMEDITEM: u32 = 1153u32;
pub const DISPID_IHTMLAREASCOLLECTION_ADD: u32 = 1503u32;
pub const DISPID_IHTMLAREASCOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLAREASCOLLECTION_LENGTH: u32 = 1500u32;
pub const DISPID_IHTMLAREASCOLLECTION_REMOVE: u32 = 1504u32;
pub const DISPID_IHTMLAREASCOLLECTION_TAGS: u32 = 1502u32;
pub const DISPID_IHTMLAREASCOLLECTION__NEWENUM: i32 = -4i32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION2_GETNAMEDITEM: u32 = 1501u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION2_REMOVENAMEDITEM: u32 = 1503u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION2_SETNAMEDITEM: u32 = 1502u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION3_IE8_GETNAMEDITEM: u32 = 1150u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION3_IE8_ITEM: u32 = 1154u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION3_IE8_LENGTH: u32 = 1153u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION3_IE8_REMOVENAMEDITEM: u32 = 1152u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION3_IE8_SETNAMEDITEM: u32 = 1151u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION4_GETNAMEDITEMNS: u32 = 1155u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION4_IE9_GETNAMEDITEM: u32 = 1158u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION4_IE9_ITEM: u32 = 1161u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION4_IE9_LENGTH: u32 = 1162u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION4_IE9_REMOVENAMEDITEM: u32 = 1160u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION4_IE9_SETNAMEDITEM: u32 = 1159u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION4_REMOVENAMEDITEMNS: u32 = 1157u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION4_SETNAMEDITEMNS: u32 = 1156u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION_LENGTH: u32 = 1500u32;
pub const DISPID_IHTMLATTRIBUTECOLLECTION__NEWENUM: i32 = -4i32;
pub const DISPID_IHTMLAUDIOELEMENTFACTORY_CREATE: u32 = 0u32;
pub const DISPID_IHTMLBASEELEMENT2_IE8_HREF: u32 = 1150u32;
pub const DISPID_IHTMLBASEELEMENT_HREF: u32 = 1003u32;
pub const DISPID_IHTMLBASEELEMENT_TARGET: u32 = 1004u32;
pub const DISPID_IHTMLBASEFONTELEMENT_COLOR: u32 = 70538u32;
pub const DISPID_IHTMLBASEFONTELEMENT_FACE: u32 = 70554u32;
pub const DISPID_IHTMLBASEFONTELEMENT_SIZE: u32 = 70562u32;
pub const DISPID_IHTMLBGSOUND_BALANCE: u32 = 1004u32;
pub const DISPID_IHTMLBGSOUND_LOOP: u32 = 1002u32;
pub const DISPID_IHTMLBGSOUND_SRC: u32 = 1001u32;
pub const DISPID_IHTMLBGSOUND_VOLUME: u32 = 1003u32;
pub const DISPID_IHTMLBLOCKELEMENT2_CITE: u32 = 1001u32;
pub const DISPID_IHTMLBLOCKELEMENT2_WIDTH: u32 = 1002u32;
pub const DISPID_IHTMLBLOCKELEMENT3_IE8_CITE: u32 = 1150u32;
pub const DISPID_IHTMLBLOCKELEMENT_CLEAR: u32 = 70552u32;
pub const DISPID_IHTMLBODYELEMENT2_ONAFTERPRINT: u32 = 71603u32;
pub const DISPID_IHTMLBODYELEMENT2_ONBEFOREPRINT: u32 = 71602u32;
pub const DISPID_IHTMLBODYELEMENT3_IE8_BACKGROUND: u32 = 1150u32;
pub const DISPID_IHTMLBODYELEMENT3_ONHASHCHANGE: u32 = 71645u32;
pub const DISPID_IHTMLBODYELEMENT3_ONOFFLINE: u32 = 71644u32;
pub const DISPID_IHTMLBODYELEMENT3_ONONLINE: u32 = 71643u32;
pub const DISPID_IHTMLBODYELEMENT4_ONMESSAGE: u32 = 71646u32;
pub const DISPID_IHTMLBODYELEMENT4_ONSTORAGE: u32 = 71636u32;
pub const DISPID_IHTMLBODYELEMENT5_ONPOPSTATE: u32 = 71728u32;
pub const DISPID_IHTMLBODYELEMENT_ALINK: u32 = 2011u32;
pub const DISPID_IHTMLBODYELEMENT_BACKGROUND: u32 = 70537u32;
pub const DISPID_IHTMLBODYELEMENT_BGCOLOR: i32 = -501i32;
pub const DISPID_IHTMLBODYELEMENT_BGPROPERTIES: u32 = 70581u32;
pub const DISPID_IHTMLBODYELEMENT_BOTTOMMARGIN: u32 = 70575u32;
pub const DISPID_IHTMLBODYELEMENT_CREATETEXTRANGE: u32 = 2013u32;
pub const DISPID_IHTMLBODYELEMENT_LEFTMARGIN: u32 = 70576u32;
pub const DISPID_IHTMLBODYELEMENT_LINK: u32 = 2010u32;
pub const DISPID_IHTMLBODYELEMENT_NOWRAP: u32 = 70541u32;
pub const DISPID_IHTMLBODYELEMENT_ONBEFOREUNLOAD: u32 = 71575u32;
pub const DISPID_IHTMLBODYELEMENT_ONLOAD: u32 = 71568u32;
pub const DISPID_IHTMLBODYELEMENT_ONSELECT: u32 = 71546u32;
pub const DISPID_IHTMLBODYELEMENT_ONUNLOAD: u32 = 71569u32;
pub const DISPID_IHTMLBODYELEMENT_RIGHTMARGIN: u32 = 70574u32;
pub const DISPID_IHTMLBODYELEMENT_SCROLL: u32 = 70615u32;
pub const DISPID_IHTMLBODYELEMENT_TEXT: u32 = 70538u32;
pub const DISPID_IHTMLBODYELEMENT_TOPMARGIN: u32 = 70573u32;
pub const DISPID_IHTMLBODYELEMENT_VLINK: u32 = 2012u32;
pub const DISPID_IHTMLBOOKMARKCOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLBOOKMARKCOLLECTION_LENGTH: u32 = 1501u32;
pub const DISPID_IHTMLBOOKMARKCOLLECTION__NEWENUM: i32 = -4i32;
pub const DISPID_IHTMLBRELEMENT_CLEAR: u32 = 70552u32;
pub const DISPID_IHTMLBUTTONELEMENT2_IE9_TYPE: u32 = 8003u32;
pub const DISPID_IHTMLBUTTONELEMENT_CREATETEXTRANGE: u32 = 8002u32;
pub const DISPID_IHTMLBUTTONELEMENT_DISABLED: u32 = 65612u32;
pub const DISPID_IHTMLBUTTONELEMENT_FORM: u32 = 67540u32;
pub const DISPID_IHTMLBUTTONELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLBUTTONELEMENT_STATUS: u32 = 8001u32;
pub const DISPID_IHTMLBUTTONELEMENT_TYPE: u32 = 2000u32;
pub const DISPID_IHTMLBUTTONELEMENT_VALUE: u32 = 70637u32;
pub const DISPID_IHTMLCANVASELEMENT_GETCONTEXT: u32 = 1001u32;
pub const DISPID_IHTMLCANVASELEMENT_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLCANVASELEMENT_TODATAURL: u32 = 1002u32;
pub const DISPID_IHTMLCANVASELEMENT_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLCOMMENTELEMENT2_APPENDDATA: u32 = 1006u32;
pub const DISPID_IHTMLCOMMENTELEMENT2_DATA: u32 = 1003u32;
pub const DISPID_IHTMLCOMMENTELEMENT2_DELETEDATA: u32 = 1008u32;
pub const DISPID_IHTMLCOMMENTELEMENT2_INSERTDATA: u32 = 1007u32;
pub const DISPID_IHTMLCOMMENTELEMENT2_LENGTH: u32 = 1004u32;
pub const DISPID_IHTMLCOMMENTELEMENT2_REPLACEDATA: u32 = 1009u32;
pub const DISPID_IHTMLCOMMENTELEMENT2_SUBSTRINGDATA: u32 = 1005u32;
pub const DISPID_IHTMLCOMMENTELEMENT3_IE9_DELETEDATA: u32 = 1012u32;
pub const DISPID_IHTMLCOMMENTELEMENT3_IE9_INSERTDATA: u32 = 1011u32;
pub const DISPID_IHTMLCOMMENTELEMENT3_IE9_REPLACEDATA: u32 = 1013u32;
pub const DISPID_IHTMLCOMMENTELEMENT3_IE9_SUBSTRINGDATA: u32 = 1010u32;
pub const DISPID_IHTMLCOMMENTELEMENT_ATOMIC: u32 = 1002u32;
pub const DISPID_IHTMLCOMMENTELEMENT_TEXT: u32 = 1001u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE: u32 = 1000u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_BACKGROUNDCOLOR: u32 = 1014u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_BLOCKDIRECTION: u32 = 1017u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_BOLD: u32 = 1001u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_DIRECTION: u32 = 1016u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_EXPLICITFACE: u32 = 1008u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_FONTNAME: u32 = 1011u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_FONTSIZE: u32 = 1010u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_FONTWEIGHT: u32 = 1009u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_HASBGCOLOR: u32 = 1012u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_ITALIC: u32 = 1002u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_OL: u32 = 1018u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_OVERLINE: u32 = 1004u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_PREFORMATTED: u32 = 1015u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_STRIKEOUT: u32 = 1005u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_SUBSCRIPT: u32 = 1006u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_SUPERSCRIPT: u32 = 1007u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_TEXTCOLOR: u32 = 1013u32;
pub const DISPID_IHTMLCOMPUTEDSTYLE_UNDERLINE: u32 = 1003u32;
pub const DISPID_IHTMLCONTROLELEMENT_ACCESSKEY: u32 = 67541u32;
pub const DISPID_IHTMLCONTROLELEMENT_ADDFILTER: u32 = 67553u32;
pub const DISPID_IHTMLCONTROLELEMENT_BLUR: u32 = 67538u32;
pub const DISPID_IHTMLCONTROLELEMENT_CLIENTHEIGHT: u32 = 67555u32;
pub const DISPID_IHTMLCONTROLELEMENT_CLIENTLEFT: u32 = 67558u32;
pub const DISPID_IHTMLCONTROLELEMENT_CLIENTTOP: u32 = 67557u32;
pub const DISPID_IHTMLCONTROLELEMENT_CLIENTWIDTH: u32 = 67556u32;
pub const DISPID_IHTMLCONTROLELEMENT_FOCUS: u32 = 67536u32;
pub const DISPID_IHTMLCONTROLELEMENT_ONBLUR: u32 = 71551u32;
pub const DISPID_IHTMLCONTROLELEMENT_ONFOCUS: u32 = 71550u32;
pub const DISPID_IHTMLCONTROLELEMENT_ONRESIZE: u32 = 71572u32;
pub const DISPID_IHTMLCONTROLELEMENT_REMOVEFILTER: u32 = 67554u32;
pub const DISPID_IHTMLCONTROLELEMENT_TABINDEX: u32 = 65551u32;
pub const DISPID_IHTMLCONTROLRANGE2_ADDELEMENT: u32 = 1016u32;
pub const DISPID_IHTMLCONTROLRANGE_ADD: u32 = 1003u32;
pub const DISPID_IHTMLCONTROLRANGE_COMMONPARENTELEMENT: u32 = 1015u32;
pub const DISPID_IHTMLCONTROLRANGE_EXECCOMMAND: u32 = 1013u32;
pub const DISPID_IHTMLCONTROLRANGE_EXECCOMMANDSHOWHELP: u32 = 1014u32;
pub const DISPID_IHTMLCONTROLRANGE_ITEM: u32 = 0u32;
pub const DISPID_IHTMLCONTROLRANGE_LENGTH: u32 = 1005u32;
pub const DISPID_IHTMLCONTROLRANGE_QUERYCOMMANDENABLED: u32 = 1008u32;
pub const DISPID_IHTMLCONTROLRANGE_QUERYCOMMANDINDETERM: u32 = 1010u32;
pub const DISPID_IHTMLCONTROLRANGE_QUERYCOMMANDSTATE: u32 = 1009u32;
pub const DISPID_IHTMLCONTROLRANGE_QUERYCOMMANDSUPPORTED: u32 = 1007u32;
pub const DISPID_IHTMLCONTROLRANGE_QUERYCOMMANDTEXT: u32 = 1011u32;
pub const DISPID_IHTMLCONTROLRANGE_QUERYCOMMANDVALUE: u32 = 1012u32;
pub const DISPID_IHTMLCONTROLRANGE_REMOVE: u32 = 1004u32;
pub const DISPID_IHTMLCONTROLRANGE_SCROLLINTOVIEW: u32 = 1006u32;
pub const DISPID_IHTMLCONTROLRANGE_SELECT: u32 = 1002u32;
pub const DISPID_IHTMLCSSIMPORTRULE_HREF: u32 = 1001u32;
pub const DISPID_IHTMLCSSIMPORTRULE_MEDIA: u32 = 1002u32;
pub const DISPID_IHTMLCSSIMPORTRULE_STYLESHEET: u32 = 1003u32;
pub const DISPID_IHTMLCSSMEDIALIST_APPENDMEDIUM: u32 = 1004u32;
pub const DISPID_IHTMLCSSMEDIALIST_DELETEMEDIUM: u32 = 1005u32;
pub const DISPID_IHTMLCSSMEDIALIST_ITEM: u32 = 1003u32;
pub const DISPID_IHTMLCSSMEDIALIST_LENGTH: u32 = 1002u32;
pub const DISPID_IHTMLCSSMEDIALIST_MEDIATEXT: u32 = 1001u32;
pub const DISPID_IHTMLCSSMEDIARULE_CSSRULES: u32 = 1002u32;
pub const DISPID_IHTMLCSSMEDIARULE_DELETERULE: u32 = 1004u32;
pub const DISPID_IHTMLCSSMEDIARULE_INSERTRULE: u32 = 1003u32;
pub const DISPID_IHTMLCSSMEDIARULE_MEDIA: u32 = 1001u32;
pub const DISPID_IHTMLCSSNAMESPACERULE_NAMESPACEURI: u32 = 1001u32;
pub const DISPID_IHTMLCSSNAMESPACERULE_PREFIX: u32 = 1002u32;
pub const DISPID_IHTMLCSSRULE_CSSTEXT: u32 = 1102u32;
pub const DISPID_IHTMLCSSRULE_PARENTRULE: u32 = 1103u32;
pub const DISPID_IHTMLCSSRULE_PARENTSTYLESHEET: u32 = 1104u32;
pub const DISPID_IHTMLCSSRULE_TYPE: u32 = 1101u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATION: u32 = 70985u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATIONDELAY: u32 = 70981u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATIONDIRECTION: u32 = 70982u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATIONDURATION: u32 = 70979u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATIONFILLMODE: u32 = 70986u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATIONITERATIONCOUNT: u32 = 70984u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATIONNAME: u32 = 70978u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATIONPLAYSTATE: u32 = 70983u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATIONTIMINGFUNCTION: u32 = 70980u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_BACKFACEVISIBILITY: u32 = 70977u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_BREAKAFTER: u32 = 70882u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_BREAKBEFORE: u32 = 70881u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_BREAKINSIDE: u32 = 70883u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLORINTERPOLATIONFILTERS: u32 = 70928u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNCOUNT: u32 = 70872u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNFILL: u32 = 70875u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNGAP: u32 = 70874u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNRULE: u32 = 70877u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNRULECOLOR: u32 = 70880u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNRULESTYLE: u32 = 70878u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNRULEWIDTH: u32 = 70879u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNS: u32 = 70871u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNSPAN: u32 = 70876u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNWIDTH: u32 = 70873u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ENABLEBACKGROUND: u32 = 70946u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_FLOODCOLOR: u32 = 70926u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_FLOODOPACITY: u32 = 70927u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_FONTFEATURESETTINGS: u32 = 70987u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_LIGHTINGCOLOR: u32 = 70929u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATION: u32 = 70924u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATIONDELAY: u32 = 70920u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATIONDIRECTION: u32 = 70921u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATIONDURATION: u32 = 70918u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATIONFILLMODE: u32 = 70925u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATIONITERATIONCOUNT: u32 = 70923u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATIONNAME: u32 = 70917u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATIONPLAYSTATE: u32 = 70922u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATIONTIMINGFUNCTION: u32 = 70919u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSBACKFACEVISIBILITY: u32 = 70890u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSCONTENTZOOMCHAINING: u32 = 70895u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSCONTENTZOOMING: u32 = 70892u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSCONTENTZOOMLIMIT: u32 = 70897u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSCONTENTZOOMLIMITMAX: u32 = 70902u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSCONTENTZOOMLIMITMIN: u32 = 70901u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSCONTENTZOOMSNAP: u32 = 70898u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSCONTENTZOOMSNAPPOINTS: u32 = 70899u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSCONTENTZOOMSNAPTYPE: u32 = 70893u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEX: u32 = 70955u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXALIGN: u32 = 70962u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXDIRECTION: u32 = 70960u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXFLOW: u32 = 70959u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXITEMALIGN: u32 = 70963u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXLINEPACK: u32 = 70965u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXNEGATIVE: u32 = 70957u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXORDER: u32 = 70966u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXPACK: u32 = 70964u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXPOSITIVE: u32 = 70956u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXPREFERREDSIZE: u32 = 70958u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXWRAP: u32 = 70961u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLOWFROM: u32 = 70938u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLOWINTO: u32 = 70939u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFONTFEATURESETTINGS: u32 = 70950u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSGRIDCOLUMN: u32 = 70908u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSGRIDCOLUMNALIGN: u32 = 70909u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSGRIDCOLUMNS: u32 = 70910u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSGRIDCOLUMNSPAN: u32 = 70911u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSGRIDROW: u32 = 70913u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSGRIDROWALIGN: u32 = 70914u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSGRIDROWS: u32 = 70915u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSGRIDROWSPAN: u32 = 70916u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSHIGHCONTRASTADJUST: u32 = 70945u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSHYPHENATELIMITCHARS: u32 = 70942u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSHYPHENATELIMITLINES: u32 = 70943u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSHYPHENATELIMITZONE: u32 = 70941u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSHYPHENS: u32 = 70940u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSOVERFLOWSTYLE: u32 = 70935u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSPERSPECTIVE: u32 = 70885u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSPERSPECTIVEORIGIN: u32 = 70886u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLCHAINING: u32 = 70891u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLLIMIT: u32 = 70934u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLLIMITXMAX: u32 = 70932u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLLIMITXMIN: u32 = 70930u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLLIMITYMAX: u32 = 70933u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLLIMITYMIN: u32 = 70931u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLRAILS: u32 = 70894u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLSNAPPOINTSX: u32 = 70905u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLSNAPPOINTSY: u32 = 70906u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLSNAPTYPE: u32 = 70896u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLSNAPX: u32 = 70903u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLSNAPY: u32 = 70904u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLTRANSLATION: u32 = 70954u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSTOUCHACTION: u32 = 70952u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSTOUCHSELECT: u32 = 70994u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSTRANSFORMSTYLE: u32 = 70889u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSTRANSITION: u32 = 70870u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSTRANSITIONDELAY: u32 = 70869u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSTRANSITIONDURATION: u32 = 70867u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSTRANSITIONPROPERTY: u32 = 70866u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSTRANSITIONTIMINGFUNCTION: u32 = 70868u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSUSERSELECT: u32 = 70951u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSWRAPFLOW: u32 = 70949u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSWRAPMARGIN: u32 = 70947u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSWRAPTHROUGH: u32 = 70937u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_PERSPECTIVE: u32 = 70974u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_PERSPECTIVEORIGIN: u32 = 70975u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TEXTSHADOW: u32 = 70936u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TRANSFORM: u32 = 70967u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TRANSFORMORIGIN: u32 = 70968u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TRANSFORMSTYLE: u32 = 70976u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TRANSITION: u32 = 70973u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TRANSITIONDELAY: u32 = 70972u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TRANSITIONDURATION: u32 = 70970u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TRANSITIONPROPERTY: u32 = 70969u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TRANSITIONTIMINGFUNCTION: u32 = 70971u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_ALIGNCONTENT: u32 = 71009u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_ALIGNITEMS: u32 = 71007u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_ALIGNSELF: u32 = 71008u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_BORDERIMAGE: u32 = 71010u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_BORDERIMAGEOUTSET: u32 = 71014u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_BORDERIMAGEREPEAT: u32 = 71015u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_BORDERIMAGESLICE: u32 = 71012u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_BORDERIMAGESOURCE: u32 = 71011u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_BORDERIMAGEWIDTH: u32 = 71013u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_FLEX: u32 = 71002u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_FLEXBASIS: u32 = 71005u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_FLEXDIRECTION: u32 = 70998u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_FLEXFLOW: u32 = 71000u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_FLEXGROW: u32 = 71003u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_FLEXSHRINK: u32 = 71004u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_FLEXWRAP: u32 = 70999u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_JUSTIFYCONTENT: u32 = 71006u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_MSIMEALIGN: u32 = 71017u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_MSTEXTCOMBINEHORIZONTAL: u32 = 71018u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_TOUCHACTION: u32 = 71019u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_MSTEXTSIZEADJUST: u32 = 70864u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATION: u32 = 71033u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATIONDELAY: u32 = 71038u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATIONDIRECTION: u32 = 71040u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATIONDURATION: u32 = 71036u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATIONFILLMODE: u32 = 71027u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATIONITERATIONCOUNT: u32 = 71039u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATIONNAME: u32 = 71035u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATIONPLAYSTATE: u32 = 71041u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATIONTIMINGFUNCTION: u32 = 71037u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITAPPEARANCE: u32 = 71020u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKFACEVISIBILITY: u32 = 71030u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUND: u32 = 71055u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDATTACHMENT: u32 = 71046u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDCLIP: u32 = 71048u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDCOLOR: u32 = 71047u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDIMAGE: u32 = 71049u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDORIGIN: u32 = 71051u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDPOSITION: u32 = 71052u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDPOSITIONX: u32 = 71053u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDPOSITIONY: u32 = 71054u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDREPEAT: u32 = 71050u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDSIZE: u32 = 71029u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBORDERIMAGE: u32 = 71061u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBORDERIMAGEOUTSET: u32 = 71065u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBORDERIMAGEREPEAT: u32 = 71066u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBORDERIMAGESLICE: u32 = 71063u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBORDERIMAGESOURCE: u32 = 71062u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBORDERIMAGEWIDTH: u32 = 71064u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBOXALIGN: u32 = 71021u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBOXDIRECTION: u32 = 71026u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBOXFLEX: u32 = 71024u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBOXORDINALGROUP: u32 = 71022u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBOXORIENT: u32 = 71025u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBOXPACK: u32 = 71023u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBOXSIZING: u32 = 71031u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITTEXTSIZEADJUST: u32 = 71060u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITTRANSFORM: u32 = 71028u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITTRANSFORMORIGIN: u32 = 71056u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITTRANSITION: u32 = 71034u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITTRANSITIONDELAY: u32 = 71045u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITTRANSITIONDURATION: u32 = 71043u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITTRANSITIONPROPERTY: u32 = 71042u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITTRANSITIONTIMINGFUNCTION: u32 = 71044u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITUSERSELECT: u32 = 71032u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_ACCELERATOR: u32 = 70683u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_ALIGNMENTBASELINE: u32 = 70814u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUND: u32 = 70568u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDATTACHMENT: u32 = 70581u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDCLIP: u32 = 70852u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDCOLOR: i32 = -501i32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDIMAGE: u32 = 70537u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDORIGIN: u32 = 70853u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDPOSITION: u32 = 70582u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDPOSITIONX: u32 = 70569u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDPOSITIONY: u32 = 70570u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDREPEAT: u32 = 70580u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDSIZE: u32 = 70854u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BASELINESHIFT: u32 = 70815u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BEHAVIOR: u32 = 70651u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDER: u32 = 70585u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERBOTTOM: u32 = 70588u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERBOTTOMCOLOR: u32 = 70593u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERBOTTOMLEFTRADIUS: u32 = 70850u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERBOTTOMRIGHTRADIUS: u32 = 70849u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERBOTTOMSTYLE: u32 = 70603u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERBOTTOMWIDTH: u32 = 70598u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERCOLLAPSE: u32 = 70620u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERCOLOR: u32 = 70590u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERLEFT: u32 = 70589u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERLEFTCOLOR: u32 = 70594u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERLEFTSTYLE: u32 = 70604u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERLEFTWIDTH: u32 = 70599u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERRADIUS: u32 = 70846u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERRIGHT: u32 = 70587u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERRIGHTCOLOR: u32 = 70592u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERRIGHTSTYLE: u32 = 70602u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERRIGHTWIDTH: u32 = 70597u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERSPACING: u32 = 70763u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERSTYLE: u32 = 70600u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERTOP: u32 = 70586u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERTOPCOLOR: u32 = 70591u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERTOPLEFTRADIUS: u32 = 70847u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERTOPRIGHTRADIUS: u32 = 70848u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERTOPSTYLE: u32 = 70601u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERTOPWIDTH: u32 = 70596u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERWIDTH: u32 = 70595u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BOTTOM: u32 = 65614u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BOXSHADOW: u32 = 70855u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BOXSIZING: u32 = 70762u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CAPTIONSIDE: u32 = 70755u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CLEAR: u32 = 70552u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CLIP: u32 = 70628u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CLIPBOTTOM: u32 = 70631u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CLIPLEFT: u32 = 70632u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CLIPPATH: u32 = 70820u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CLIPRIGHT: u32 = 70630u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CLIPRULE: u32 = 70821u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CLIPTOP: u32 = 70629u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_COLOR: u32 = 70538u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CONTENT: u32 = 70754u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_COUNTERINCREMENT: u32 = 70756u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_COUNTERRESET: u32 = 70757u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CSSFLOAT: u32 = 70845u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CSSTEXT: u32 = 70635u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CURSOR: u32 = 70638u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_DIRECTION: u32 = 70655u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_DISPLAY: u32 = 70607u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_DOMINANTBASELINE: u32 = 70816u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_EMPTYCELLS: u32 = 70786u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FILL: u32 = 70822u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FILLOPACITY: u32 = 70823u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FILLRULE: u32 = 70824u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FILTER: u32 = 70618u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FONT: u32 = 70577u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FONTFAMILY: u32 = 70554u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FONTSIZE: u32 = 70555u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FONTSIZEADJUST: u32 = 70817u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FONTSTRETCH: u32 = 70818u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FONTSTYLE: u32 = 70560u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FONTVARIANT: u32 = 70561u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FONTWEIGHT: u32 = 70563u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_GETPROPERTYPRIORITY: u32 = 70040u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_GETPROPERTYVALUE: u32 = 70039u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_GLYPHORIENTATIONHORIZONTAL: u32 = 70843u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_GLYPHORIENTATIONVERTICAL: u32 = 70844u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_IMEMODE: u32 = 70656u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_KERNING: u32 = 70825u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LAYOUTFLOW: u32 = 70691u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LAYOUTGRID: u32 = 70667u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LAYOUTGRIDCHAR: u32 = 70663u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LAYOUTGRIDLINE: u32 = 70664u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LAYOUTGRIDMODE: u32 = 70665u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LAYOUTGRIDTYPE: u32 = 70666u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LEFT: u32 = 65539u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LENGTH: u32 = 70037u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LETTERSPACING: u32 = 70544u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LINEBREAK: u32 = 70669u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LINEHEIGHT: u32 = 70542u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LISTSTYLE: u32 = 70611u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LISTSTYLEIMAGE: u32 = 70610u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LISTSTYLEPOSITION: u32 = 70609u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LISTSTYLETYPE: u32 = 70608u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARGIN: u32 = 70572u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARGINBOTTOM: u32 = 70575u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARGINLEFT: u32 = 70576u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARGINRIGHT: u32 = 70574u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARGINTOP: u32 = 70573u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARKER: u32 = 70826u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARKEREND: u32 = 70827u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARKERMID: u32 = 70828u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARKERSTART: u32 = 70829u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MASK: u32 = 70830u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MAXHEIGHT: u32 = 70750u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MAXWIDTH: u32 = 70752u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MINHEIGHT: u32 = 70747u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MINWIDTH: u32 = 70751u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MSBLOCKPROGRESSION: u32 = 70787u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MSINTERPOLATIONMODE: u32 = 70749u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MSTRANSFORM: u32 = 70851u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MSTRANSFORMORIGIN: u32 = 70861u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_OPACITY: u32 = 70819u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_ORPHANS: u32 = 70764u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_OUTLINE: u32 = 70758u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_OUTLINECOLOR: u32 = 70761u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_OUTLINESTYLE: u32 = 70760u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_OUTLINEWIDTH: u32 = 70759u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_OVERFLOW: u32 = 70546u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_OVERFLOWX: u32 = 70675u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_OVERFLOWY: u32 = 70676u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PADDING: u32 = 70547u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PADDINGBOTTOM: u32 = 70550u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PADDINGLEFT: u32 = 70551u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PADDINGRIGHT: u32 = 70549u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PADDINGTOP: u32 = 70548u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PAGEBREAKAFTER: u32 = 70614u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PAGEBREAKBEFORE: u32 = 70613u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PAGEBREAKINSIDE: u32 = 70766u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PARENTRULE: u32 = 70038u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_POINTEREVENTS: u32 = 70831u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_POSITION: u32 = 70626u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_QUOTES: u32 = 70788u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_REMOVEPROPERTY: u32 = 70041u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_RIGHT: u32 = 65613u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_RUBYALIGN: u32 = 70657u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_RUBYOVERHANG: u32 = 70659u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_RUBYPOSITION: u32 = 70658u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SCROLLBAR3DLIGHTCOLOR: u32 = 70718u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SCROLLBARARROWCOLOR: u32 = 70722u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SCROLLBARBASECOLOR: u32 = 70716u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SCROLLBARDARKSHADOWCOLOR: u32 = 70721u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SCROLLBARFACECOLOR: u32 = 70717u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SCROLLBARHIGHLIGHTCOLOR: u32 = 70720u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SCROLLBARSHADOWCOLOR: u32 = 70719u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SCROLLBARTRACKCOLOR: u32 = 70732u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SETPROPERTY: u32 = 70042u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STOPCOLOR: u32 = 70832u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STOPOPACITY: u32 = 70833u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STROKE: u32 = 70834u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STROKEDASHARRAY: u32 = 70835u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STROKEDASHOFFSET: u32 = 70836u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STROKELINECAP: u32 = 70837u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STROKELINEJOIN: u32 = 70838u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STROKEMITERLIMIT: u32 = 70839u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STROKEOPACITY: u32 = 70840u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STROKEWIDTH: u32 = 70841u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STYLEFLOAT: u32 = 70606u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TABLELAYOUT: u32 = 70634u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTALIGN: u32 = 65608u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTALIGNLAST: u32 = 70739u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTANCHOR: u32 = 70842u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTAUTOSPACE: u32 = 70668u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTDECORATION: u32 = 70571u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTINDENT: u32 = 70543u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTJUSTIFY: u32 = 70671u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTJUSTIFYTRIM: u32 = 70672u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTKASHIDA: u32 = 70673u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTKASHIDASPACE: u32 = 70740u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTOVERFLOW: u32 = 70745u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTTRANSFORM: u32 = 70540u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTUNDERLINEPOSITION: u32 = 70695u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TOP: u32 = 65540u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_UNICODEBIDI: u32 = 70654u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_VERTICALALIGN: u32 = 70584u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_VISIBILITY: u32 = 70616u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_WHITESPACE: u32 = 70612u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_WIDOWS: u32 = 70765u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_WORDBREAK: u32 = 70670u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_WORDSPACING: u32 = 70583u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_WORDWRAP: u32 = 70694u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_WRITINGMODE: u32 = 70728u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_ZINDEX: u32 = 70627u32;
pub const DISPID_IHTMLCSSSTYLEDECLARATION_ZOOM: u32 = 70689u32;
pub const DISPID_IHTMLCURRENTSTYLE2_FILTER: u32 = 70618u32;
pub const DISPID_IHTMLCURRENTSTYLE2_HASLAYOUT: u32 = 70696u32;
pub const DISPID_IHTMLCURRENTSTYLE2_ISBLOCK: u32 = 70744u32;
pub const DISPID_IHTMLCURRENTSTYLE2_LAYOUTFLOW: u32 = 70691u32;
pub const DISPID_IHTMLCURRENTSTYLE2_SCROLLBAR3DLIGHTCOLOR: u32 = 70718u32;
pub const DISPID_IHTMLCURRENTSTYLE2_SCROLLBARARROWCOLOR: u32 = 70722u32;
pub const DISPID_IHTMLCURRENTSTYLE2_SCROLLBARBASECOLOR: u32 = 70716u32;
pub const DISPID_IHTMLCURRENTSTYLE2_SCROLLBARDARKSHADOWCOLOR: u32 = 70721u32;
pub const DISPID_IHTMLCURRENTSTYLE2_SCROLLBARFACECOLOR: u32 = 70717u32;
pub const DISPID_IHTMLCURRENTSTYLE2_SCROLLBARHIGHLIGHTCOLOR: u32 = 70720u32;
pub const DISPID_IHTMLCURRENTSTYLE2_SCROLLBARSHADOWCOLOR: u32 = 70719u32;
pub const DISPID_IHTMLCURRENTSTYLE2_SCROLLBARTRACKCOLOR: u32 = 70732u32;
pub const DISPID_IHTMLCURRENTSTYLE2_TEXTALIGNLAST: u32 = 70739u32;
pub const DISPID_IHTMLCURRENTSTYLE2_TEXTKASHIDASPACE: u32 = 70740u32;
pub const DISPID_IHTMLCURRENTSTYLE2_TEXTUNDERLINEPOSITION: u32 = 70695u32;
pub const DISPID_IHTMLCURRENTSTYLE2_WORDWRAP: u32 = 70694u32;
pub const DISPID_IHTMLCURRENTSTYLE2_WRITINGMODE: u32 = 70728u32;
pub const DISPID_IHTMLCURRENTSTYLE2_ZOOM: u32 = 70689u32;
pub const DISPID_IHTMLCURRENTSTYLE3_MINHEIGHT: u32 = 70747u32;
pub const DISPID_IHTMLCURRENTSTYLE3_TEXTOVERFLOW: u32 = 70745u32;
pub const DISPID_IHTMLCURRENTSTYLE3_WHITESPACE: u32 = 70612u32;
pub const DISPID_IHTMLCURRENTSTYLE3_WORDSPACING: u32 = 70583u32;
pub const DISPID_IHTMLCURRENTSTYLE4_MAXHEIGHT: u32 = 70750u32;
pub const DISPID_IHTMLCURRENTSTYLE4_MAXWIDTH: u32 = 70752u32;
pub const DISPID_IHTMLCURRENTSTYLE4_MINWIDTH: u32 = 70751u32;
pub const DISPID_IHTMLCURRENTSTYLE4_MSINTERPOLATIONMODE: u32 = 70749u32;
pub const DISPID_IHTMLCURRENTSTYLE5_BORDERSPACING: u32 = 70763u32;
pub const DISPID_IHTMLCURRENTSTYLE5_BOXSIZING: u32 = 70762u32;
pub const DISPID_IHTMLCURRENTSTYLE5_CAPTIONSIDE: u32 = 70755u32;
pub const DISPID_IHTMLCURRENTSTYLE5_EMPTYCELLS: u32 = 70786u32;
pub const DISPID_IHTMLCURRENTSTYLE5_MSBLOCKPROGRESSION: u32 = 70787u32;
pub const DISPID_IHTMLCURRENTSTYLE5_ORPHANS: u32 = 70764u32;
pub const DISPID_IHTMLCURRENTSTYLE5_OUTLINE: u32 = 70758u32;
pub const DISPID_IHTMLCURRENTSTYLE5_OUTLINECOLOR: u32 = 70761u32;
pub const DISPID_IHTMLCURRENTSTYLE5_OUTLINESTYLE: u32 = 70760u32;
pub const DISPID_IHTMLCURRENTSTYLE5_OUTLINEWIDTH: u32 = 70759u32;
pub const DISPID_IHTMLCURRENTSTYLE5_PAGEBREAKINSIDE: u32 = 70766u32;
pub const DISPID_IHTMLCURRENTSTYLE5_QUOTES: u32 = 70788u32;
pub const DISPID_IHTMLCURRENTSTYLE5_WIDOWS: u32 = 70765u32;
pub const DISPID_IHTMLCURRENTSTYLE_ACCELERATOR: u32 = 70683u32;
pub const DISPID_IHTMLCURRENTSTYLE_BACKGROUNDATTACHMENT: u32 = 70581u32;
pub const DISPID_IHTMLCURRENTSTYLE_BACKGROUNDCOLOR: i32 = -501i32;
pub const DISPID_IHTMLCURRENTSTYLE_BACKGROUNDIMAGE: u32 = 70537u32;
pub const DISPID_IHTMLCURRENTSTYLE_BACKGROUNDPOSITIONX: u32 = 70569u32;
pub const DISPID_IHTMLCURRENTSTYLE_BACKGROUNDPOSITIONY: u32 = 70570u32;
pub const DISPID_IHTMLCURRENTSTYLE_BACKGROUNDREPEAT: u32 = 70580u32;
pub const DISPID_IHTMLCURRENTSTYLE_BEHAVIOR: u32 = 70651u32;
pub const DISPID_IHTMLCURRENTSTYLE_BLOCKDIRECTION: u32 = 70653u32;
pub const DISPID_IHTMLCURRENTSTYLE_BORDERBOTTOMCOLOR: u32 = 70593u32;
pub const DISPID_IHTMLCURRENTSTYLE_BORDERBOTTOMSTYLE: u32 = 70603u32;
pub const DISPID_IHTMLCURRENTSTYLE_BORDERBOTTOMWIDTH: u32 = 70598u32;
pub const DISPID_IHTMLCURRENTSTYLE_BORDERCOLLAPSE: u32 = 70620u32;
pub const DISPID_IHTMLCURRENTSTYLE_BORDERCOLOR: u32 = 70590u32;
pub const DISPID_IHTMLCURRENTSTYLE_BORDERLEFTCOLOR: u32 = 70594u32;
pub const DISPID_IHTMLCURRENTSTYLE_BORDERLEFTSTYLE: u32 = 70604u32;
pub const DISPID_IHTMLCURRENTSTYLE_BORDERLEFTWIDTH: u32 = 70599u32;
pub const DISPID_IHTMLCURRENTSTYLE_BORDERRIGHTCOLOR: u32 = 70592u32;
pub const DISPID_IHTMLCURRENTSTYLE_BORDERRIGHTSTYLE: u32 = 70602u32;
pub const DISPID_IHTMLCURRENTSTYLE_BORDERRIGHTWIDTH: u32 = 70597u32;
pub const DISPID_IHTMLCURRENTSTYLE_BORDERSTYLE: u32 = 70600u32;
pub const DISPID_IHTMLCURRENTSTYLE_BORDERTOPCOLOR: u32 = 70591u32;
pub const DISPID_IHTMLCURRENTSTYLE_BORDERTOPSTYLE: u32 = 70601u32;
pub const DISPID_IHTMLCURRENTSTYLE_BORDERTOPWIDTH: u32 = 70596u32;
pub const DISPID_IHTMLCURRENTSTYLE_BORDERWIDTH: u32 = 70595u32;
pub const DISPID_IHTMLCURRENTSTYLE_BOTTOM: u32 = 65614u32;
pub const DISPID_IHTMLCURRENTSTYLE_CLEAR: u32 = 70552u32;
pub const DISPID_IHTMLCURRENTSTYLE_CLIPBOTTOM: u32 = 70631u32;
pub const DISPID_IHTMLCURRENTSTYLE_CLIPLEFT: u32 = 70632u32;
pub const DISPID_IHTMLCURRENTSTYLE_CLIPRIGHT: u32 = 70630u32;
pub const DISPID_IHTMLCURRENTSTYLE_CLIPTOP: u32 = 70629u32;
pub const DISPID_IHTMLCURRENTSTYLE_COLOR: u32 = 70538u32;
pub const DISPID_IHTMLCURRENTSTYLE_CURSOR: u32 = 70638u32;
pub const DISPID_IHTMLCURRENTSTYLE_DIRECTION: u32 = 70655u32;
pub const DISPID_IHTMLCURRENTSTYLE_DISPLAY: u32 = 70607u32;
pub const DISPID_IHTMLCURRENTSTYLE_FONTFAMILY: u32 = 70554u32;
pub const DISPID_IHTMLCURRENTSTYLE_FONTSIZE: u32 = 70555u32;
pub const DISPID_IHTMLCURRENTSTYLE_FONTSTYLE: u32 = 70560u32;
pub const DISPID_IHTMLCURRENTSTYLE_FONTVARIANT: u32 = 70561u32;
pub const DISPID_IHTMLCURRENTSTYLE_FONTWEIGHT: u32 = 70563u32;
pub const DISPID_IHTMLCURRENTSTYLE_GETATTRIBUTE: u32 = 66038u32;
pub const DISPID_IHTMLCURRENTSTYLE_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLCURRENTSTYLE_IMEMODE: u32 = 70656u32;
pub const DISPID_IHTMLCURRENTSTYLE_LAYOUTGRIDCHAR: u32 = 70663u32;
pub const DISPID_IHTMLCURRENTSTYLE_LAYOUTGRIDLINE: u32 = 70664u32;
pub const DISPID_IHTMLCURRENTSTYLE_LAYOUTGRIDMODE: u32 = 70665u32;
pub const DISPID_IHTMLCURRENTSTYLE_LAYOUTGRIDTYPE: u32 = 70666u32;
pub const DISPID_IHTMLCURRENTSTYLE_LEFT: u32 = 65539u32;
pub const DISPID_IHTMLCURRENTSTYLE_LETTERSPACING: u32 = 70544u32;
pub const DISPID_IHTMLCURRENTSTYLE_LINEBREAK: u32 = 70669u32;
pub const DISPID_IHTMLCURRENTSTYLE_LINEHEIGHT: u32 = 70542u32;
pub const DISPID_IHTMLCURRENTSTYLE_LISTSTYLEIMAGE: u32 = 70610u32;
pub const DISPID_IHTMLCURRENTSTYLE_LISTSTYLEPOSITION: u32 = 70609u32;
pub const DISPID_IHTMLCURRENTSTYLE_LISTSTYLETYPE: u32 = 70608u32;
pub const DISPID_IHTMLCURRENTSTYLE_MARGIN: u32 = 70572u32;
pub const DISPID_IHTMLCURRENTSTYLE_MARGINBOTTOM: u32 = 70575u32;
pub const DISPID_IHTMLCURRENTSTYLE_MARGINLEFT: u32 = 70576u32;
pub const DISPID_IHTMLCURRENTSTYLE_MARGINRIGHT: u32 = 70574u32;
pub const DISPID_IHTMLCURRENTSTYLE_MARGINTOP: u32 = 70573u32;
pub const DISPID_IHTMLCURRENTSTYLE_OVERFLOW: u32 = 70546u32;
pub const DISPID_IHTMLCURRENTSTYLE_OVERFLOWX: u32 = 70675u32;
pub const DISPID_IHTMLCURRENTSTYLE_OVERFLOWY: u32 = 70676u32;
pub const DISPID_IHTMLCURRENTSTYLE_PADDING: u32 = 70547u32;
pub const DISPID_IHTMLCURRENTSTYLE_PADDINGBOTTOM: u32 = 70550u32;
pub const DISPID_IHTMLCURRENTSTYLE_PADDINGLEFT: u32 = 70551u32;
pub const DISPID_IHTMLCURRENTSTYLE_PADDINGRIGHT: u32 = 70549u32;
pub const DISPID_IHTMLCURRENTSTYLE_PADDINGTOP: u32 = 70548u32;
pub const DISPID_IHTMLCURRENTSTYLE_PAGEBREAKAFTER: u32 = 70614u32;
pub const DISPID_IHTMLCURRENTSTYLE_PAGEBREAKBEFORE: u32 = 70613u32;
pub const DISPID_IHTMLCURRENTSTYLE_POSITION: u32 = 70626u32;
pub const DISPID_IHTMLCURRENTSTYLE_RIGHT: u32 = 65613u32;
pub const DISPID_IHTMLCURRENTSTYLE_RUBYALIGN: u32 = 70657u32;
pub const DISPID_IHTMLCURRENTSTYLE_RUBYOVERHANG: u32 = 70659u32;
pub const DISPID_IHTMLCURRENTSTYLE_RUBYPOSITION: u32 = 70658u32;
pub const DISPID_IHTMLCURRENTSTYLE_STYLEFLOAT: u32 = 70606u32;
pub const DISPID_IHTMLCURRENTSTYLE_TABLELAYOUT: u32 = 70634u32;
pub const DISPID_IHTMLCURRENTSTYLE_TEXTALIGN: u32 = 65608u32;
pub const DISPID_IHTMLCURRENTSTYLE_TEXTAUTOSPACE: u32 = 70668u32;
pub const DISPID_IHTMLCURRENTSTYLE_TEXTDECORATION: u32 = 70571u32;
pub const DISPID_IHTMLCURRENTSTYLE_TEXTINDENT: u32 = 70543u32;
pub const DISPID_IHTMLCURRENTSTYLE_TEXTJUSTIFY: u32 = 70671u32;
pub const DISPID_IHTMLCURRENTSTYLE_TEXTJUSTIFYTRIM: u32 = 70672u32;
pub const DISPID_IHTMLCURRENTSTYLE_TEXTKASHIDA: u32 = 70673u32;
pub const DISPID_IHTMLCURRENTSTYLE_TEXTTRANSFORM: u32 = 70540u32;
pub const DISPID_IHTMLCURRENTSTYLE_TOP: u32 = 65540u32;
pub const DISPID_IHTMLCURRENTSTYLE_UNICODEBIDI: u32 = 70654u32;
pub const DISPID_IHTMLCURRENTSTYLE_VERTICALALIGN: u32 = 70584u32;
pub const DISPID_IHTMLCURRENTSTYLE_VISIBILITY: u32 = 70616u32;
pub const DISPID_IHTMLCURRENTSTYLE_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLCURRENTSTYLE_WORDBREAK: u32 = 70670u32;
pub const DISPID_IHTMLCURRENTSTYLE_ZINDEX: u32 = 70627u32;
pub const DISPID_IHTMLDATABINDING_DATAFLD: u32 = 66557u32;
pub const DISPID_IHTMLDATABINDING_DATAFORMATAS: u32 = 66559u32;
pub const DISPID_IHTMLDATABINDING_DATASRC: u32 = 66558u32;
pub const DISPID_IHTMLDATATRANSFER_CLEARDATA: u32 = 1003u32;
pub const DISPID_IHTMLDATATRANSFER_DROPEFFECT: u32 = 1004u32;
pub const DISPID_IHTMLDATATRANSFER_EFFECTALLOWED: u32 = 1005u32;
pub const DISPID_IHTMLDATATRANSFER_GETDATA: u32 = 1002u32;
pub const DISPID_IHTMLDATATRANSFER_SETDATA: u32 = 1001u32;
pub const DISPID_IHTMLDDELEMENT_NOWRAP: u32 = 70541u32;
pub const DISPID_IHTMLDIALOG2_RESIZABLE: u32 = 25015u32;
pub const DISPID_IHTMLDIALOG2_STATUS: u32 = 25014u32;
pub const DISPID_IHTMLDIALOG3_DIALOGHIDE: u32 = 25007u32;
pub const DISPID_IHTMLDIALOG3_UNADORNED: u32 = 25016u32;
pub const DISPID_IHTMLDIALOG_CLOSE: u32 = 25011u32;
pub const DISPID_IHTMLDIALOG_DIALOGARGUMENTS: u32 = 25000u32;
pub const DISPID_IHTMLDIALOG_DIALOGHEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLDIALOG_DIALOGLEFT: u32 = 65539u32;
pub const DISPID_IHTMLDIALOG_DIALOGTOP: u32 = 65540u32;
pub const DISPID_IHTMLDIALOG_DIALOGWIDTH: u32 = 65541u32;
pub const DISPID_IHTMLDIALOG_MENUARGUMENTS: u32 = 25013u32;
pub const DISPID_IHTMLDIALOG_RETURNVALUE: u32 = 25001u32;
pub const DISPID_IHTMLDIALOG_TOSTRING: u32 = 25012u32;
pub const DISPID_IHTMLDIVELEMENT_ALIGN: u32 = 65608u32;
pub const DISPID_IHTMLDIVELEMENT_NOWRAP: u32 = 70541u32;
pub const DISPID_IHTMLDIVPOSITION_ALIGN: u32 = 65609u32;
pub const DISPID_IHTMLDLGSAFEHELPER_BLOCKFORMATS: u32 = 4u32;
pub const DISPID_IHTMLDLGSAFEHELPER_CHOOSECOLORDLG: u32 = 1u32;
pub const DISPID_IHTMLDLGSAFEHELPER_FONTS: u32 = 3u32;
pub const DISPID_IHTMLDLGSAFEHELPER_GETCHARSET: u32 = 2u32;
pub const DISPID_IHTMLDLISTELEMENT_COMPACT: u32 = 1001u32;
pub const DISPID_IHTMLDOCUMENT2_ACTIVEELEMENT: u32 = 1005u32;
pub const DISPID_IHTMLDOCUMENT2_ALINKCOLOR: u32 = 1022u32;
pub const DISPID_IHTMLDOCUMENT2_ALL: u32 = 1003u32;
pub const DISPID_IHTMLDOCUMENT2_ANCHORS: u32 = 1007u32;
pub const DISPID_IHTMLDOCUMENT2_APPLETS: u32 = 1008u32;
pub const DISPID_IHTMLDOCUMENT2_BGCOLOR: i32 = -501i32;
pub const DISPID_IHTMLDOCUMENT2_BODY: u32 = 1004u32;
pub const DISPID_IHTMLDOCUMENT2_CHARSET: u32 = 1032u32;
pub const DISPID_IHTMLDOCUMENT2_CLEAR: u32 = 1058u32;
pub const DISPID_IHTMLDOCUMENT2_CLOSE: u32 = 1057u32;
pub const DISPID_IHTMLDOCUMENT2_COOKIE: u32 = 1030u32;
pub const DISPID_IHTMLDOCUMENT2_CREATEELEMENT: u32 = 1067u32;
pub const DISPID_IHTMLDOCUMENT2_CREATESTYLESHEET: u32 = 1071u32;
pub const DISPID_IHTMLDOCUMENT2_DEFAULTCHARSET: u32 = 1033u32;
pub const DISPID_IHTMLDOCUMENT2_DESIGNMODE: u32 = 1014u32;
pub const DISPID_IHTMLDOCUMENT2_DOMAIN: u32 = 1029u32;
pub const DISPID_IHTMLDOCUMENT2_ELEMENTFROMPOINT: u32 = 1068u32;
pub const DISPID_IHTMLDOCUMENT2_EMBEDS: u32 = 1015u32;
pub const DISPID_IHTMLDOCUMENT2_EXECCOMMAND: u32 = 1065u32;
pub const DISPID_IHTMLDOCUMENT2_EXECCOMMANDSHOWHELP: u32 = 1066u32;
pub const DISPID_IHTMLDOCUMENT2_EXPANDO: u32 = 1031u32;
pub const DISPID_IHTMLDOCUMENT2_FGCOLOR: u32 = 70538u32;
pub const DISPID_IHTMLDOCUMENT2_FILECREATEDDATE: u32 = 1043u32;
pub const DISPID_IHTMLDOCUMENT2_FILEMODIFIEDDATE: u32 = 1044u32;
pub const DISPID_IHTMLDOCUMENT2_FILESIZE: u32 = 1042u32;
pub const DISPID_IHTMLDOCUMENT2_FILEUPDATEDDATE: u32 = 1045u32;
pub const DISPID_IHTMLDOCUMENT2_FORMS: u32 = 1010u32;
pub const DISPID_IHTMLDOCUMENT2_FRAMES: u32 = 1019u32;
pub const DISPID_IHTMLDOCUMENT2_IMAGES: u32 = 1011u32;
pub const DISPID_IHTMLDOCUMENT2_LASTMODIFIED: u32 = 1028u32;
pub const DISPID_IHTMLDOCUMENT2_LINKCOLOR: u32 = 1024u32;
pub const DISPID_IHTMLDOCUMENT2_LINKS: u32 = 1009u32;
pub const DISPID_IHTMLDOCUMENT2_LOCATION: u32 = 1026u32;
pub const DISPID_IHTMLDOCUMENT2_MIMETYPE: u32 = 1041u32;
pub const DISPID_IHTMLDOCUMENT2_NAMEPROP: u32 = 1048u32;
pub const DISPID_IHTMLDOCUMENT2_ONAFTERUPDATE: u32 = 71558u32;
pub const DISPID_IHTMLDOCUMENT2_ONBEFOREUPDATE: u32 = 71557u32;
pub const DISPID_IHTMLDOCUMENT2_ONCLICK: u32 = 71544u32;
pub const DISPID_IHTMLDOCUMENT2_ONDBLCLICK: u32 = 71545u32;
pub const DISPID_IHTMLDOCUMENT2_ONDRAGSTART: u32 = 71571u32;
pub const DISPID_IHTMLDOCUMENT2_ONERRORUPDATE: u32 = 71574u32;
pub const DISPID_IHTMLDOCUMENT2_ONHELP: u32 = 71549u32;
pub const DISPID_IHTMLDOCUMENT2_ONKEYDOWN: u32 = 71541u32;
pub const DISPID_IHTMLDOCUMENT2_ONKEYPRESS: u32 = 71543u32;
pub const DISPID_IHTMLDOCUMENT2_ONKEYUP: u32 = 71542u32;
pub const DISPID_IHTMLDOCUMENT2_ONMOUSEDOWN: u32 = 71538u32;
pub const DISPID_IHTMLDOCUMENT2_ONMOUSEMOVE: u32 = 71540u32;
pub const DISPID_IHTMLDOCUMENT2_ONMOUSEOUT: u32 = 71537u32;
pub const DISPID_IHTMLDOCUMENT2_ONMOUSEOVER: u32 = 71536u32;
pub const DISPID_IHTMLDOCUMENT2_ONMOUSEUP: u32 = 71539u32;
pub const DISPID_IHTMLDOCUMENT2_ONREADYSTATECHANGE: u32 = 71561u32;
pub const DISPID_IHTMLDOCUMENT2_ONROWENTER: u32 = 71555u32;
pub const DISPID_IHTMLDOCUMENT2_ONROWEXIT: u32 = 71554u32;
pub const DISPID_IHTMLDOCUMENT2_ONSELECTSTART: u32 = 71573u32;
pub const DISPID_IHTMLDOCUMENT2_OPEN: u32 = 1056u32;
pub const DISPID_IHTMLDOCUMENT2_PARENTWINDOW: u32 = 1034u32;
pub const DISPID_IHTMLDOCUMENT2_PLUGINS: u32 = 1021u32;
pub const DISPID_IHTMLDOCUMENT2_PROTOCOL: u32 = 1047u32;
pub const DISPID_IHTMLDOCUMENT2_QUERYCOMMANDENABLED: u32 = 1060u32;
pub const DISPID_IHTMLDOCUMENT2_QUERYCOMMANDINDETERM: u32 = 1062u32;
pub const DISPID_IHTMLDOCUMENT2_QUERYCOMMANDSTATE: u32 = 1061u32;
pub const DISPID_IHTMLDOCUMENT2_QUERYCOMMANDSUPPORTED: u32 = 1059u32;
pub const DISPID_IHTMLDOCUMENT2_QUERYCOMMANDTEXT: u32 = 1063u32;
pub const DISPID_IHTMLDOCUMENT2_QUERYCOMMANDVALUE: u32 = 1064u32;
pub const DISPID_IHTMLDOCUMENT2_READYSTATE: u32 = 1018u32;
pub const DISPID_IHTMLDOCUMENT2_REFERRER: u32 = 1027u32;
pub const DISPID_IHTMLDOCUMENT2_SCRIPTS: u32 = 1013u32;
pub const DISPID_IHTMLDOCUMENT2_SECURITY: u32 = 1046u32;
pub const DISPID_IHTMLDOCUMENT2_SELECTION: u32 = 1017u32;
pub const DISPID_IHTMLDOCUMENT2_STYLESHEETS: u32 = 1069u32;
pub const DISPID_IHTMLDOCUMENT2_TITLE: u32 = 1012u32;
pub const DISPID_IHTMLDOCUMENT2_TOSTRING: u32 = 1070u32;
pub const DISPID_IHTMLDOCUMENT2_URL: u32 = 1025u32;
pub const DISPID_IHTMLDOCUMENT2_VLINKCOLOR: u32 = 1023u32;
pub const DISPID_IHTMLDOCUMENT2_WRITE: u32 = 1054u32;
pub const DISPID_IHTMLDOCUMENT2_WRITELN: u32 = 1055u32;
pub const DISPID_IHTMLDOCUMENT3_ATTACHEVENT: u32 = 66043u32;
pub const DISPID_IHTMLDOCUMENT3_BASEURL: u32 = 1080u32;
pub const DISPID_IHTMLDOCUMENT3_CHILDNODES: u32 = 66585u32;
pub const DISPID_IHTMLDOCUMENT3_CREATEDOCUMENTFRAGMENT: u32 = 1076u32;
pub const DISPID_IHTMLDOCUMENT3_CREATETEXTNODE: u32 = 1074u32;
pub const DISPID_IHTMLDOCUMENT3_DETACHEVENT: u32 = 66044u32;
pub const DISPID_IHTMLDOCUMENT3_DIR: u32 = 70653u32;
pub const DISPID_IHTMLDOCUMENT3_DOCUMENTELEMENT: u32 = 1075u32;
pub const DISPID_IHTMLDOCUMENT3_ENABLEDOWNLOAD: u32 = 1079u32;
pub const DISPID_IHTMLDOCUMENT3_GETELEMENTBYID: u32 = 1088u32;
pub const DISPID_IHTMLDOCUMENT3_GETELEMENTSBYNAME: u32 = 1086u32;
pub const DISPID_IHTMLDOCUMENT3_GETELEMENTSBYTAGNAME: u32 = 1087u32;
pub const DISPID_IHTMLDOCUMENT3_INHERITSTYLESHEETS: u32 = 1082u32;
pub const DISPID_IHTMLDOCUMENT3_ONBEFOREEDITFOCUS: u32 = 71605u32;
pub const DISPID_IHTMLDOCUMENT3_ONCELLCHANGE: u32 = 71600u32;
pub const DISPID_IHTMLDOCUMENT3_ONCONTEXTMENU: u32 = 71601u32;
pub const DISPID_IHTMLDOCUMENT3_ONDATAAVAILABLE: u32 = 71577u32;
pub const DISPID_IHTMLDOCUMENT3_ONDATASETCHANGED: u32 = 71576u32;
pub const DISPID_IHTMLDOCUMENT3_ONDATASETCOMPLETE: u32 = 71578u32;
pub const DISPID_IHTMLDOCUMENT3_ONPROPERTYCHANGE: u32 = 71583u32;
pub const DISPID_IHTMLDOCUMENT3_ONROWSDELETE: u32 = 71598u32;
pub const DISPID_IHTMLDOCUMENT3_ONROWSINSERTED: u32 = 71599u32;
pub const DISPID_IHTMLDOCUMENT3_ONSTOP: u32 = 71604u32;
pub const DISPID_IHTMLDOCUMENT3_PARENTDOCUMENT: u32 = 1078u32;
pub const DISPID_IHTMLDOCUMENT3_RECALC: u32 = 1073u32;
pub const DISPID_IHTMLDOCUMENT3_RELEASECAPTURE: u32 = 1072u32;
pub const DISPID_IHTMLDOCUMENT3_UNIQUEID: u32 = 1077u32;
pub const DISPID_IHTMLDOCUMENT4_CREATEDOCUMENTFROMURL: u32 = 1092u32;
pub const DISPID_IHTMLDOCUMENT4_CREATEEVENTOBJECT: u32 = 1094u32;
pub const DISPID_IHTMLDOCUMENT4_CREATERENDERSTYLE: u32 = 1096u32;
pub const DISPID_IHTMLDOCUMENT4_FIREEVENT: u32 = 1095u32;
pub const DISPID_IHTMLDOCUMENT4_FOCUS: u32 = 1089u32;
pub const DISPID_IHTMLDOCUMENT4_HASFOCUS: u32 = 1090u32;
pub const DISPID_IHTMLDOCUMENT4_MEDIA: u32 = 1093u32;
pub const DISPID_IHTMLDOCUMENT4_NAMESPACES: u32 = 1091u32;
pub const DISPID_IHTMLDOCUMENT4_ONCONTROLSELECT: u32 = 71615u32;
pub const DISPID_IHTMLDOCUMENT4_ONSELECTIONCHANGE: u32 = 71616u32;
pub const DISPID_IHTMLDOCUMENT4_URLUNENCODED: u32 = 1097u32;
pub const DISPID_IHTMLDOCUMENT5_COMPATMODE: u32 = 1102u32;
pub const DISPID_IHTMLDOCUMENT5_CREATEATTRIBUTE: u32 = 1100u32;
pub const DISPID_IHTMLDOCUMENT5_CREATECOMMENT: u32 = 1101u32;
pub const DISPID_IHTMLDOCUMENT5_DOCTYPE: u32 = 1098u32;
pub const DISPID_IHTMLDOCUMENT5_IMPLEMENTATION: u32 = 1099u32;
pub const DISPID_IHTMLDOCUMENT5_ONACTIVATE: u32 = 71623u32;
pub const DISPID_IHTMLDOCUMENT5_ONBEFOREACTIVATE: u32 = 71626u32;
pub const DISPID_IHTMLDOCUMENT5_ONBEFOREDEACTIVATE: u32 = 71613u32;
pub const DISPID_IHTMLDOCUMENT5_ONDEACTIVATE: u32 = 71624u32;
pub const DISPID_IHTMLDOCUMENT5_ONFOCUSIN: u32 = 71627u32;
pub const DISPID_IHTMLDOCUMENT5_ONFOCUSOUT: u32 = 71628u32;
pub const DISPID_IHTMLDOCUMENT5_ONMOUSEWHEEL: u32 = 71612u32;
pub const DISPID_IHTMLDOCUMENT6_COMPATIBLE: u32 = 1103u32;
pub const DISPID_IHTMLDOCUMENT6_DOCUMENTMODE: u32 = 1104u32;
pub const DISPID_IHTMLDOCUMENT6_IE8_GETELEMENTBYID: u32 = 1107u32;
pub const DISPID_IHTMLDOCUMENT6_ONSTORAGE: u32 = 71636u32;
pub const DISPID_IHTMLDOCUMENT6_ONSTORAGECOMMIT: u32 = 71637u32;
pub const DISPID_IHTMLDOCUMENT6_UPDATESETTINGS: u32 = 1109u32;
pub const DISPID_IHTMLDOCUMENT7_ADOPTNODE: u32 = 1125u32;
pub const DISPID_IHTMLDOCUMENT7_CHARACTERSET: u32 = 1117u32;
pub const DISPID_IHTMLDOCUMENT7_CREATEATTRIBUTENS: u32 = 1115u32;
pub const DISPID_IHTMLDOCUMENT7_CREATECDATASECTION: u32 = 1123u32;
pub const DISPID_IHTMLDOCUMENT7_CREATEELEMENTNS: u32 = 1114u32;
pub const DISPID_IHTMLDOCUMENT7_CREATEPROCESSINGINSTRUCTION: u32 = 1124u32;
pub const DISPID_IHTMLDOCUMENT7_DEFAULTVIEW: u32 = 1110u32;
pub const DISPID_IHTMLDOCUMENT7_GETELEMENTSBYCLASSNAME: u32 = 1120u32;
pub const DISPID_IHTMLDOCUMENT7_GETELEMENTSBYTAGNAMENS: u32 = 1113u32;
pub const DISPID_IHTMLDOCUMENT7_GETSELECTION: u32 = 1112u32;
pub const DISPID_IHTMLDOCUMENT7_HASATTRIBUTES: u32 = 1132u32;
pub const DISPID_IHTMLDOCUMENT7_HEAD: u32 = 1138u32;
pub const DISPID_IHTMLDOCUMENT7_IE9_ALL: u32 = 1126u32;
pub const DISPID_IHTMLDOCUMENT7_IE9_BODY: u32 = 1137u32;
pub const DISPID_IHTMLDOCUMENT7_IE9_CREATEATTRIBUTE: u32 = 1119u32;
pub const DISPID_IHTMLDOCUMENT7_IE9_CREATEELEMENT: u32 = 1118u32;
pub const DISPID_IHTMLDOCUMENT7_IE9_PARENTWINDOW: u32 = 1136u32;
pub const DISPID_IHTMLDOCUMENT7_IMPORTNODE: u32 = 1135u32;
pub const DISPID_IHTMLDOCUMENT7_INPUTENCODING: u32 = 1127u32;
pub const DISPID_IHTMLDOCUMENT7_NORMALIZE: u32 = 1134u32;
pub const DISPID_IHTMLDOCUMENT7_ONABORT: u32 = 71564u32;
pub const DISPID_IHTMLDOCUMENT7_ONBLUR: u32 = 71551u32;
pub const DISPID_IHTMLDOCUMENT7_ONCANPLAY: u32 = 71670u32;
pub const DISPID_IHTMLDOCUMENT7_ONCANPLAYTHROUGH: u32 = 71671u32;
pub const DISPID_IHTMLDOCUMENT7_ONCHANGE: u32 = 71566u32;
pub const DISPID_IHTMLDOCUMENT7_ONDRAG: u32 = 71585u32;
pub const DISPID_IHTMLDOCUMENT7_ONDRAGEND: u32 = 71586u32;
pub const DISPID_IHTMLDOCUMENT7_ONDRAGENTER: u32 = 71587u32;
pub const DISPID_IHTMLDOCUMENT7_ONDRAGLEAVE: u32 = 71589u32;
pub const DISPID_IHTMLDOCUMENT7_ONDRAGOVER: u32 = 71588u32;
pub const DISPID_IHTMLDOCUMENT7_ONDROP: u32 = 71590u32;
pub const DISPID_IHTMLDOCUMENT7_ONDURATIONCHANGE: u32 = 71672u32;
pub const DISPID_IHTMLDOCUMENT7_ONEMPTIED: u32 = 71673u32;
pub const DISPID_IHTMLDOCUMENT7_ONENDED: u32 = 71674u32;
pub const DISPID_IHTMLDOCUMENT7_ONERROR: u32 = 71565u32;
pub const DISPID_IHTMLDOCUMENT7_ONFOCUS: u32 = 71550u32;
pub const DISPID_IHTMLDOCUMENT7_ONINPUT: u32 = 71663u32;
pub const DISPID_IHTMLDOCUMENT7_ONLOAD: u32 = 71568u32;
pub const DISPID_IHTMLDOCUMENT7_ONLOADEDDATA: u32 = 71675u32;
pub const DISPID_IHTMLDOCUMENT7_ONLOADEDMETADATA: u32 = 71676u32;
pub const DISPID_IHTMLDOCUMENT7_ONLOADSTART: u32 = 71677u32;
pub const DISPID_IHTMLDOCUMENT7_ONMSSITEMODEJUMPLISTITEMREMOVED: u32 = 71666u32;
pub const DISPID_IHTMLDOCUMENT7_ONMSTHUMBNAILCLICK: u32 = 71657u32;
pub const DISPID_IHTMLDOCUMENT7_ONPAUSE: u32 = 71678u32;
pub const DISPID_IHTMLDOCUMENT7_ONPLAY: u32 = 71679u32;
pub const DISPID_IHTMLDOCUMENT7_ONPLAYING: u32 = 71680u32;
pub const DISPID_IHTMLDOCUMENT7_ONPROGRESS: u32 = 71681u32;
pub const DISPID_IHTMLDOCUMENT7_ONRATECHANGE: u32 = 71682u32;
pub const DISPID_IHTMLDOCUMENT7_ONRESET: u32 = 71548u32;
pub const DISPID_IHTMLDOCUMENT7_ONSCROLL: u32 = 71567u32;
pub const DISPID_IHTMLDOCUMENT7_ONSEEKED: u32 = 71683u32;
pub const DISPID_IHTMLDOCUMENT7_ONSEEKING: u32 = 71684u32;
pub const DISPID_IHTMLDOCUMENT7_ONSELECT: u32 = 71546u32;
pub const DISPID_IHTMLDOCUMENT7_ONSTALLED: u32 = 71685u32;
pub const DISPID_IHTMLDOCUMENT7_ONSUBMIT: u32 = 71547u32;
pub const DISPID_IHTMLDOCUMENT7_ONSUSPEND: u32 = 71686u32;
pub const DISPID_IHTMLDOCUMENT7_ONTIMEUPDATE: u32 = 71687u32;
pub const DISPID_IHTMLDOCUMENT7_ONVOLUMECHANGE: u32 = 71688u32;
pub const DISPID_IHTMLDOCUMENT7_ONWAITING: u32 = 71689u32;
pub const DISPID_IHTMLDOCUMENT7_XMLENCODING: u32 = 1128u32;
pub const DISPID_IHTMLDOCUMENT7_XMLSTANDALONE: u32 = 1129u32;
pub const DISPID_IHTMLDOCUMENT7_XMLVERSION: u32 = 1130u32;
pub const DISPID_IHTMLDOCUMENT8_ELEMENTSFROMPOINT: u32 = 1139u32;
pub const DISPID_IHTMLDOCUMENT8_ELEMENTSFROMRECT: u32 = 1140u32;
pub const DISPID_IHTMLDOCUMENT8_MSCAPSLOCKWARNINGOFF: u32 = 1141u32;
pub const DISPID_IHTMLDOCUMENT8_ONMSCONTENTZOOM: u32 = 71708u32;
pub const DISPID_IHTMLDOCUMENT8_ONMSGESTURECHANGE: u32 = 71700u32;
pub const DISPID_IHTMLDOCUMENT8_ONMSGESTUREDOUBLETAP: u32 = 71704u32;
pub const DISPID_IHTMLDOCUMENT8_ONMSGESTUREEND: u32 = 71701u32;
pub const DISPID_IHTMLDOCUMENT8_ONMSGESTUREHOLD: u32 = 71702u32;
pub const DISPID_IHTMLDOCUMENT8_ONMSGESTURESTART: u32 = 71699u32;
pub const DISPID_IHTMLDOCUMENT8_ONMSGESTURETAP: u32 = 71703u32;
pub const DISPID_IHTMLDOCUMENT8_ONMSINERTIASTART: u32 = 71705u32;
pub const DISPID_IHTMLDOCUMENT8_ONMSMANIPULATIONSTATECHANGED: u32 = 71714u32;
pub const DISPID_IHTMLDOCUMENT8_ONMSPOINTERCANCEL: u32 = 71695u32;
pub const DISPID_IHTMLDOCUMENT8_ONMSPOINTERDOWN: u32 = 71690u32;
pub const DISPID_IHTMLDOCUMENT8_ONMSPOINTERHOVER: u32 = 71696u32;
pub const DISPID_IHTMLDOCUMENT8_ONMSPOINTERMOVE: u32 = 71691u32;
pub const DISPID_IHTMLDOCUMENT8_ONMSPOINTEROUT: u32 = 71694u32;
pub const DISPID_IHTMLDOCUMENT8_ONMSPOINTEROVER: u32 = 71693u32;
pub const DISPID_IHTMLDOCUMENT8_ONMSPOINTERUP: u32 = 71692u32;
pub const DISPID_IHTMLDOCUMENTCOMPATIBLEINFOCOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLDOCUMENTCOMPATIBLEINFOCOLLECTION_LENGTH: u32 = 1001u32;
pub const DISPID_IHTMLDOCUMENTCOMPATIBLEINFO_USERAGENT: u32 = 1001u32;
pub const DISPID_IHTMLDOCUMENTCOMPATIBLEINFO_VERSION: u32 = 1002u32;
pub const DISPID_IHTMLDOCUMENT_SCRIPT: u32 = 1001u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_APPENDCHILD: u32 = 1018u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_ATTRIBUTES: u32 = 1013u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_CHILDNODES: u32 = 1008u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_CLONENODE: u32 = 1020u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_EXPANDO: u32 = 1005u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_FIRSTCHILD: u32 = 1009u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_HASCHILDNODES: u32 = 1019u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_INSERTBEFORE: u32 = 1015u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_LASTCHILD: u32 = 1010u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_NAME: u32 = 1003u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_NEXTSIBLING: u32 = 1012u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_NODETYPE: u32 = 1006u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_OWNERDOCUMENT: u32 = 1014u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_PARENTNODE: u32 = 1007u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_PREVIOUSSIBLING: u32 = 1011u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_REMOVECHILD: u32 = 1017u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_REPLACECHILD: u32 = 1016u32;
pub const DISPID_IHTMLDOMATTRIBUTE2_VALUE: u32 = 1004u32;
pub const DISPID_IHTMLDOMATTRIBUTE3_IE8_NODEVALUE: u32 = 1153u32;
pub const DISPID_IHTMLDOMATTRIBUTE3_IE8_SPECIFIED: u32 = 1150u32;
pub const DISPID_IHTMLDOMATTRIBUTE3_IE8_VALUE: u32 = 1154u32;
pub const DISPID_IHTMLDOMATTRIBUTE3_OWNERELEMENT: u32 = 1151u32;
pub const DISPID_IHTMLDOMATTRIBUTE4_HASATTRIBUTES: u32 = 1166u32;
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_CHILDNODES: u32 = 1165u32;
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_FIRSTCHILD: u32 = 1163u32;
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_HASCHILDNODES: u32 = 1167u32;
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_LASTCHILD: u32 = 1164u32;
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_NAME: u32 = 1161u32;
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_NODENAME: u32 = 1160u32;
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_NODEVALUE: u32 = 1159u32;
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_SPECIFIED: u32 = 1171u32;
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_VALUE: u32 = 1162u32;
pub const DISPID_IHTMLDOMATTRIBUTE4_NORMALIZE: u32 = 1170u32;
pub const DISPID_IHTMLDOMATTRIBUTE_NODENAME: u32 = 1000u32;
pub const DISPID_IHTMLDOMATTRIBUTE_NODEVALUE: u32 = 1002u32;
pub const DISPID_IHTMLDOMATTRIBUTE_SPECIFIED: u32 = 1001u32;
pub const DISPID_IHTMLDOMCHILDRENCOLLECTION2_IE9_ITEM: u32 = 1u32;
pub const DISPID_IHTMLDOMCHILDRENCOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLDOMCHILDRENCOLLECTION_LENGTH: u32 = 1500u32;
pub const DISPID_IHTMLDOMCHILDRENCOLLECTION__NEWENUM: i32 = -4i32;
pub const DISPID_IHTMLDOMCONSTRUCTOR_CONSTRUCTOR: u32 = 66045u32;
pub const DISPID_IHTMLDOMIMPLEMENTATION2_CREATEDOCUMENT: u32 = 1002u32;
pub const DISPID_IHTMLDOMIMPLEMENTATION2_CREATEDOCUMENTTYPE: u32 = 1001u32;
pub const DISPID_IHTMLDOMIMPLEMENTATION2_CREATEHTMLDOCUMENT: u32 = 1003u32;
pub const DISPID_IHTMLDOMIMPLEMENTATION2_IE9_HASFEATURE: u32 = 1004u32;
pub const DISPID_IHTMLDOMIMPLEMENTATION_HASFEATURE: u32 = 1000u32;
pub const DISPID_IHTMLDOMNODE2_OWNERDOCUMENT: u32 = 66649u32;
pub const DISPID_IHTMLDOMNODE3_COMPAREDOCUMENTPOSITION: u32 = 66662u32;
pub const DISPID_IHTMLDOMNODE3_IE9_APPENDCHILD: u32 = 66804u32;
pub const DISPID_IHTMLDOMNODE3_IE9_INSERTBEFORE: u32 = 66805u32;
pub const DISPID_IHTMLDOMNODE3_IE9_REMOVECHILD: u32 = 66806u32;
pub const DISPID_IHTMLDOMNODE3_IE9_REPLACECHILD: u32 = 66807u32;
pub const DISPID_IHTMLDOMNODE3_ISDEFAULTNAMESPACE: u32 = 66660u32;
pub const DISPID_IHTMLDOMNODE3_ISEQUALNODE: u32 = 66657u32;
pub const DISPID_IHTMLDOMNODE3_ISSAMENODE: u32 = 66661u32;
pub const DISPID_IHTMLDOMNODE3_ISSUPPORTED: u32 = 66813u32;
pub const DISPID_IHTMLDOMNODE3_LOCALNAME: u32 = 66654u32;
pub const DISPID_IHTMLDOMNODE3_LOOKUPNAMESPACEURI: u32 = 66658u32;
pub const DISPID_IHTMLDOMNODE3_LOOKUPPREFIX: u32 = 66659u32;
pub const DISPID_IHTMLDOMNODE3_NAMESPACEURI: u32 = 66655u32;
pub const DISPID_IHTMLDOMNODE3_PREFIX: u32 = 66656u32;
pub const DISPID_IHTMLDOMNODE3_TEXTCONTENT: u32 = 66663u32;
pub const DISPID_IHTMLDOMNODE_APPENDCHILD: u32 = 66609u32;
pub const DISPID_IHTMLDOMNODE_ATTRIBUTES: u32 = 66586u32;
pub const DISPID_IHTMLDOMNODE_CHILDNODES: u32 = 66585u32;
pub const DISPID_IHTMLDOMNODE_CLONENODE: u32 = 66597u32;
pub const DISPID_IHTMLDOMNODE_FIRSTCHILD: u32 = 66612u32;
pub const DISPID_IHTMLDOMNODE_HASCHILDNODES: u32 = 66584u32;
pub const DISPID_IHTMLDOMNODE_INSERTBEFORE: u32 = 66587u32;
pub const DISPID_IHTMLDOMNODE_LASTCHILD: u32 = 66613u32;
pub const DISPID_IHTMLDOMNODE_NEXTSIBLING: u32 = 66615u32;
pub const DISPID_IHTMLDOMNODE_NODENAME: u32 = 66610u32;
pub const DISPID_IHTMLDOMNODE_NODETYPE: u32 = 66582u32;
pub const DISPID_IHTMLDOMNODE_NODEVALUE: u32 = 66611u32;
pub const DISPID_IHTMLDOMNODE_PARENTNODE: u32 = 66583u32;
pub const DISPID_IHTMLDOMNODE_PREVIOUSSIBLING: u32 = 66614u32;
pub const DISPID_IHTMLDOMNODE_REMOVECHILD: u32 = 66588u32;
pub const DISPID_IHTMLDOMNODE_REMOVENODE: u32 = 66602u32;
pub const DISPID_IHTMLDOMNODE_REPLACECHILD: u32 = 66589u32;
pub const DISPID_IHTMLDOMNODE_REPLACENODE: u32 = 66603u32;
pub const DISPID_IHTMLDOMNODE_SWAPNODE: u32 = 66604u32;
pub const DISPID_IHTMLDOMRANGE_CLONECONTENTS: u32 = 1019u32;
pub const DISPID_IHTMLDOMRANGE_CLONERANGE: u32 = 1022u32;
pub const DISPID_IHTMLDOMRANGE_COLLAPSE: u32 = 1013u32;
pub const DISPID_IHTMLDOMRANGE_COLLAPSED: u32 = 1005u32;
pub const DISPID_IHTMLDOMRANGE_COMMONANCESTORCONTAINER: u32 = 1006u32;
pub const DISPID_IHTMLDOMRANGE_COMPAREBOUNDARYPOINTS: u32 = 1016u32;
pub const DISPID_IHTMLDOMRANGE_DELETECONTENTS: u32 = 1017u32;
pub const DISPID_IHTMLDOMRANGE_DETACH: u32 = 1024u32;
pub const DISPID_IHTMLDOMRANGE_ENDCONTAINER: u32 = 1003u32;
pub const DISPID_IHTMLDOMRANGE_ENDOFFSET: u32 = 1004u32;
pub const DISPID_IHTMLDOMRANGE_EXTRACTCONTENTS: u32 = 1018u32;
pub const DISPID_IHTMLDOMRANGE_GETBOUNDINGCLIENTRECT: u32 = 1026u32;
pub const DISPID_IHTMLDOMRANGE_GETCLIENTRECTS: u32 = 1025u32;
pub const DISPID_IHTMLDOMRANGE_INSERTNODE: u32 = 1020u32;
pub const DISPID_IHTMLDOMRANGE_SELECTNODE: u32 = 1014u32;
pub const DISPID_IHTMLDOMRANGE_SELECTNODECONTENTS: u32 = 1015u32;
pub const DISPID_IHTMLDOMRANGE_SETEND: u32 = 1008u32;
pub const DISPID_IHTMLDOMRANGE_SETENDAFTER: u32 = 1012u32;
pub const DISPID_IHTMLDOMRANGE_SETENDBEFORE: u32 = 1011u32;
pub const DISPID_IHTMLDOMRANGE_SETSTART: u32 = 1007u32;
pub const DISPID_IHTMLDOMRANGE_SETSTARTAFTER: u32 = 1010u32;
pub const DISPID_IHTMLDOMRANGE_SETSTARTBEFORE: u32 = 1009u32;
pub const DISPID_IHTMLDOMRANGE_STARTCONTAINER: u32 = 1001u32;
pub const DISPID_IHTMLDOMRANGE_STARTOFFSET: u32 = 1002u32;
pub const DISPID_IHTMLDOMRANGE_SURROUNDCONTENTS: u32 = 1021u32;
pub const DISPID_IHTMLDOMRANGE_TOSTRING: u32 = 1023u32;
pub const DISPID_IHTMLDOMTEXTNODE2_APPENDDATA: u32 = 1005u32;
pub const DISPID_IHTMLDOMTEXTNODE2_DELETEDATA: u32 = 1007u32;
pub const DISPID_IHTMLDOMTEXTNODE2_INSERTDATA: u32 = 1006u32;
pub const DISPID_IHTMLDOMTEXTNODE2_REPLACEDATA: u32 = 1008u32;
pub const DISPID_IHTMLDOMTEXTNODE2_SUBSTRINGDATA: u32 = 1004u32;
pub const DISPID_IHTMLDOMTEXTNODE3_HASATTRIBUTES: u32 = 1017u32;
pub const DISPID_IHTMLDOMTEXTNODE3_IE9_DELETEDATA: u32 = 1012u32;
pub const DISPID_IHTMLDOMTEXTNODE3_IE9_INSERTDATA: u32 = 1011u32;
pub const DISPID_IHTMLDOMTEXTNODE3_IE9_REPLACEDATA: u32 = 1013u32;
pub const DISPID_IHTMLDOMTEXTNODE3_IE9_SPLITTEXT: u32 = 1009u32;
pub const DISPID_IHTMLDOMTEXTNODE3_IE9_SUBSTRINGDATA: u32 = 1010u32;
pub const DISPID_IHTMLDOMTEXTNODE3_NORMALIZE: u32 = 1019u32;
pub const DISPID_IHTMLDOMTEXTNODE3_REPLACEWHOLETEXT: u32 = 1015u32;
pub const DISPID_IHTMLDOMTEXTNODE3_WHOLETEXT: u32 = 1014u32;
pub const DISPID_IHTMLDOMTEXTNODE_DATA: u32 = 1000u32;
pub const DISPID_IHTMLDOMTEXTNODE_LENGTH: u32 = 1002u32;
pub const DISPID_IHTMLDOMTEXTNODE_SPLITTEXT: u32 = 1003u32;
pub const DISPID_IHTMLDOMTEXTNODE_TOSTRING: u32 = 1001u32;
pub const DISPID_IHTMLDTELEMENT_NOWRAP: u32 = 70541u32;
pub const DISPID_IHTMLELEMENT2_ACCESSKEY: u32 = 67541u32;
pub const DISPID_IHTMLELEMENT2_ADDBEHAVIOR: u32 = 66616u32;
pub const DISPID_IHTMLELEMENT2_ADDFILTER: u32 = 67553u32;
pub const DISPID_IHTMLELEMENT2_APPLYELEMENT: u32 = 66601u32;
pub const DISPID_IHTMLELEMENT2_ATTACHEVENT: u32 = 66043u32;
pub const DISPID_IHTMLELEMENT2_BEHAVIORURNS: u32 = 66618u32;
pub const DISPID_IHTMLELEMENT2_BLUR: u32 = 67538u32;
pub const DISPID_IHTMLELEMENT2_CANHAVECHILDREN: u32 = 66608u32;
pub const DISPID_IHTMLELEMENT2_CLEARATTRIBUTES: u32 = 66598u32;
pub const DISPID_IHTMLELEMENT2_CLIENTHEIGHT: u32 = 67555u32;
pub const DISPID_IHTMLELEMENT2_CLIENTLEFT: u32 = 67558u32;
pub const DISPID_IHTMLELEMENT2_CLIENTTOP: u32 = 67557u32;
pub const DISPID_IHTMLELEMENT2_CLIENTWIDTH: u32 = 67556u32;
pub const DISPID_IHTMLELEMENT2_COMPONENTFROMPOINT: u32 = 66578u32;
pub const DISPID_IHTMLELEMENT2_CREATECONTROLRANGE: u32 = 66592u32;
pub const DISPID_IHTMLELEMENT2_CURRENTSTYLE: u32 = 66543u32;
pub const DISPID_IHTMLELEMENT2_DETACHEVENT: u32 = 66044u32;
pub const DISPID_IHTMLELEMENT2_DIR: u32 = 70653u32;
pub const DISPID_IHTMLELEMENT2_DOSCROLL: u32 = 66579u32;
pub const DISPID_IHTMLELEMENT2_FOCUS: u32 = 67536u32;
pub const DISPID_IHTMLELEMENT2_GETADJACENTTEXT: u32 = 66606u32;
pub const DISPID_IHTMLELEMENT2_GETBOUNDINGCLIENTRECT: u32 = 66581u32;
pub const DISPID_IHTMLELEMENT2_GETCLIENTRECTS: u32 = 66580u32;
pub const DISPID_IHTMLELEMENT2_GETELEMENTSBYTAGNAME: u32 = 66621u32;
pub const DISPID_IHTMLELEMENT2_GETEXPRESSION: u32 = 66041u32;
pub const DISPID_IHTMLELEMENT2_INSERTADJACENTELEMENT: u32 = 66605u32;
pub const DISPID_IHTMLELEMENT2_MERGEATTRIBUTES: u32 = 66599u32;
pub const DISPID_IHTMLELEMENT2_ONBEFORECOPY: u32 = 71595u32;
pub const DISPID_IHTMLELEMENT2_ONBEFORECUT: u32 = 71594u32;
pub const DISPID_IHTMLELEMENT2_ONBEFOREEDITFOCUS: u32 = 71605u32;
pub const DISPID_IHTMLELEMENT2_ONBEFOREPASTE: u32 = 71596u32;
pub const DISPID_IHTMLELEMENT2_ONBLUR: u32 = 71551u32;
pub const DISPID_IHTMLELEMENT2_ONCELLCHANGE: u32 = 71600u32;
pub const DISPID_IHTMLELEMENT2_ONCONTEXTMENU: u32 = 71601u32;
pub const DISPID_IHTMLELEMENT2_ONCOPY: u32 = 71592u32;
pub const DISPID_IHTMLELEMENT2_ONCUT: u32 = 71591u32;
pub const DISPID_IHTMLELEMENT2_ONDRAG: u32 = 71585u32;
pub const DISPID_IHTMLELEMENT2_ONDRAGEND: u32 = 71586u32;
pub const DISPID_IHTMLELEMENT2_ONDRAGENTER: u32 = 71587u32;
pub const DISPID_IHTMLELEMENT2_ONDRAGLEAVE: u32 = 71589u32;
pub const DISPID_IHTMLELEMENT2_ONDRAGOVER: u32 = 71588u32;
pub const DISPID_IHTMLELEMENT2_ONDROP: u32 = 71590u32;
pub const DISPID_IHTMLELEMENT2_ONFOCUS: u32 = 71550u32;
pub const DISPID_IHTMLELEMENT2_ONLOSECAPTURE: u32 = 71582u32;
pub const DISPID_IHTMLELEMENT2_ONPASTE: u32 = 71593u32;
pub const DISPID_IHTMLELEMENT2_ONPROPERTYCHANGE: u32 = 71583u32;
pub const DISPID_IHTMLELEMENT2_ONREADYSTATECHANGE: u32 = 71561u32;
pub const DISPID_IHTMLELEMENT2_ONRESIZE: u32 = 71572u32;
pub const DISPID_IHTMLELEMENT2_ONROWSDELETE: u32 = 71598u32;
pub const DISPID_IHTMLELEMENT2_ONROWSINSERTED: u32 = 71599u32;
pub const DISPID_IHTMLELEMENT2_ONSCROLL: u32 = 71567u32;
pub const DISPID_IHTMLELEMENT2_READYSTATE: u32 = 70652u32;
pub const DISPID_IHTMLELEMENT2_READYSTATEVALUE: u32 = 66620u32;
pub const DISPID_IHTMLELEMENT2_RELEASECAPTURE: u32 = 66577u32;
pub const DISPID_IHTMLELEMENT2_REMOVEBEHAVIOR: u32 = 66617u32;
pub const DISPID_IHTMLELEMENT2_REMOVEEXPRESSION: u32 = 66042u32;
pub const DISPID_IHTMLELEMENT2_REMOVEFILTER: u32 = 67554u32;
pub const DISPID_IHTMLELEMENT2_REPLACEADJACENTTEXT: u32 = 66607u32;
pub const DISPID_IHTMLELEMENT2_RUNTIMESTYLE: u32 = 66600u32;
pub const DISPID_IHTMLELEMENT2_SCOPENAME: u32 = 66575u32;
pub const DISPID_IHTMLELEMENT2_SCROLLHEIGHT: u32 = 66593u32;
pub const DISPID_IHTMLELEMENT2_SCROLLLEFT: u32 = 66596u32;
pub const DISPID_IHTMLELEMENT2_SCROLLTOP: u32 = 66595u32;
pub const DISPID_IHTMLELEMENT2_SCROLLWIDTH: u32 = 66594u32;
pub const DISPID_IHTMLELEMENT2_SETCAPTURE: u32 = 66576u32;
pub const DISPID_IHTMLELEMENT2_SETEXPRESSION: u32 = 66040u32;
pub const DISPID_IHTMLELEMENT2_TABINDEX: u32 = 65551u32;
pub const DISPID_IHTMLELEMENT2_TAGURN: u32 = 66619u32;
pub const DISPID_IHTMLELEMENT3_CANHAVEHTML: u32 = 66634u32;
pub const DISPID_IHTMLELEMENT3_CONTENTEDITABLE: u32 = 70698u32;
pub const DISPID_IHTMLELEMENT3_DISABLED: u32 = 65612u32;
pub const DISPID_IHTMLELEMENT3_DRAGDROP: u32 = 66643u32;
pub const DISPID_IHTMLELEMENT3_FIREEVENT: u32 = 66642u32;
pub const DISPID_IHTMLELEMENT3_GLYPHMODE: u32 = 66644u32;
pub const DISPID_IHTMLELEMENT3_HIDEFOCUS: u32 = 70699u32;
pub const DISPID_IHTMLELEMENT3_INFLATEBLOCK: u32 = 66636u32;
pub const DISPID_IHTMLELEMENT3_ISCONTENTEDITABLE: u32 = 66638u32;
pub const DISPID_IHTMLELEMENT3_ISDISABLED: u32 = 66641u32;
pub const DISPID_IHTMLELEMENT3_ISMULTILINE: u32 = 66633u32;
pub const DISPID_IHTMLELEMENT3_MERGEATTRIBUTES: u32 = 66632u32;
pub const DISPID_IHTMLELEMENT3_ONACTIVATE: u32 = 71623u32;
pub const DISPID_IHTMLELEMENT3_ONBEFOREDEACTIVATE: u32 = 71613u32;
pub const DISPID_IHTMLELEMENT3_ONCONTROLSELECT: u32 = 71615u32;
pub const DISPID_IHTMLELEMENT3_ONDEACTIVATE: u32 = 71624u32;
pub const DISPID_IHTMLELEMENT3_ONLAYOUTCOMPLETE: u32 = 71609u32;
pub const DISPID_IHTMLELEMENT3_ONMOUSEENTER: u32 = 71621u32;
pub const DISPID_IHTMLELEMENT3_ONMOUSELEAVE: u32 = 71622u32;
pub const DISPID_IHTMLELEMENT3_ONMOVE: u32 = 71614u32;
pub const DISPID_IHTMLELEMENT3_ONMOVEEND: u32 = 71618u32;
pub const DISPID_IHTMLELEMENT3_ONMOVESTART: u32 = 71617u32;
pub const DISPID_IHTMLELEMENT3_ONPAGE: u32 = 71610u32;
pub const DISPID_IHTMLELEMENT3_ONRESIZEEND: u32 = 71620u32;
pub const DISPID_IHTMLELEMENT3_ONRESIZESTART: u32 = 71619u32;
pub const DISPID_IHTMLELEMENT3_SETACTIVE: u32 = 66637u32;
pub const DISPID_IHTMLELEMENT4_GETATTRIBUTENODE: u32 = 66645u32;
pub const DISPID_IHTMLELEMENT4_NORMALIZE: u32 = 66648u32;
pub const DISPID_IHTMLELEMENT4_ONBEFOREACTIVATE: u32 = 71626u32;
pub const DISPID_IHTMLELEMENT4_ONFOCUSIN: u32 = 71627u32;
pub const DISPID_IHTMLELEMENT4_ONFOCUSOUT: u32 = 71628u32;
pub const DISPID_IHTMLELEMENT4_ONMOUSEWHEEL: u32 = 71612u32;
pub const DISPID_IHTMLELEMENT4_REMOVEATTRIBUTENODE: u32 = 66647u32;
pub const DISPID_IHTMLELEMENT4_SETATTRIBUTENODE: u32 = 66646u32;
pub const DISPID_IHTMLELEMENT5_ARIAACTIVEDESCENDANT: u32 = 66768u32;
pub const DISPID_IHTMLELEMENT5_ARIABUSY: u32 = 66741u32;
pub const DISPID_IHTMLELEMENT5_ARIACHECKED: u32 = 66742u32;
pub const DISPID_IHTMLELEMENT5_ARIACONTROLS: u32 = 66764u32;
pub const DISPID_IHTMLELEMENT5_ARIADESCRIBEDBY: u32 = 66765u32;
pub const DISPID_IHTMLELEMENT5_ARIADISABLED: u32 = 66743u32;
pub const DISPID_IHTMLELEMENT5_ARIAEXPANDED: u32 = 66744u32;
pub const DISPID_IHTMLELEMENT5_ARIAFLOWTO: u32 = 66766u32;
pub const DISPID_IHTMLELEMENT5_ARIAHASPOPUP: u32 = 66745u32;
pub const DISPID_IHTMLELEMENT5_ARIAHIDDEN: u32 = 66746u32;
pub const DISPID_IHTMLELEMENT5_ARIAINVALID: u32 = 66747u32;
pub const DISPID_IHTMLELEMENT5_ARIALABELLEDBY: u32 = 66767u32;
pub const DISPID_IHTMLELEMENT5_ARIALEVEL: u32 = 66761u32;
pub const DISPID_IHTMLELEMENT5_ARIALIVE: u32 = 66771u32;
pub const DISPID_IHTMLELEMENT5_ARIAMULTISELECTABLE: u32 = 66748u32;
pub const DISPID_IHTMLELEMENT5_ARIAOWNS: u32 = 66769u32;
pub const DISPID_IHTMLELEMENT5_ARIAPOSINSET: u32 = 66759u32;
pub const DISPID_IHTMLELEMENT5_ARIAPRESSED: u32 = 66749u32;
pub const DISPID_IHTMLELEMENT5_ARIAREADONLY: u32 = 66750u32;
pub const DISPID_IHTMLELEMENT5_ARIARELEVANT: u32 = 66772u32;
pub const DISPID_IHTMLELEMENT5_ARIAREQUIRED: u32 = 66751u32;
pub const DISPID_IHTMLELEMENT5_ARIASECRET: u32 = 66752u32;
pub const DISPID_IHTMLELEMENT5_ARIASELECTED: u32 = 66753u32;
pub const DISPID_IHTMLELEMENT5_ARIASETSIZE: u32 = 66760u32;
pub const DISPID_IHTMLELEMENT5_ARIAVALUEMAX: u32 = 66763u32;
pub const DISPID_IHTMLELEMENT5_ARIAVALUEMIN: u32 = 66762u32;
pub const DISPID_IHTMLELEMENT5_ARIAVALUENOW: u32 = 66758u32;
pub const DISPID_IHTMLELEMENT5_HASATTRIBUTE: u32 = 66739u32;
pub const DISPID_IHTMLELEMENT5_HASATTRIBUTES: u32 = 66770u32;
pub const DISPID_IHTMLELEMENT5_IE8_ATTRIBUTES: u32 = 66757u32;
pub const DISPID_IHTMLELEMENT5_IE8_GETATTRIBUTE: u32 = 66754u32;
pub const DISPID_IHTMLELEMENT5_IE8_GETATTRIBUTENODE: u32 = 66736u32;
pub const DISPID_IHTMLELEMENT5_IE8_REMOVEATTRIBUTE: u32 = 66756u32;
pub const DISPID_IHTMLELEMENT5_IE8_REMOVEATTRIBUTENODE: u32 = 66738u32;
pub const DISPID_IHTMLELEMENT5_IE8_SETATTRIBUTE: u32 = 66755u32;
pub const DISPID_IHTMLELEMENT5_IE8_SETATTRIBUTENODE: u32 = 66737u32;
pub const DISPID_IHTMLELEMENT5_ROLE: u32 = 66740u32;
pub const DISPID_IHTMLELEMENT6_GETATTRIBUTENODENS: u32 = 66786u32;
pub const DISPID_IHTMLELEMENT6_GETATTRIBUTENS: u32 = 66789u32;
pub const DISPID_IHTMLELEMENT6_GETELEMENTSBYCLASSNAME: u32 = 66803u32;
pub const DISPID_IHTMLELEMENT6_GETELEMENTSBYTAGNAMENS: u32 = 66799u32;
pub const DISPID_IHTMLELEMENT6_HASATTRIBUTENS: u32 = 66788u32;
pub const DISPID_IHTMLELEMENT6_IE9_GETATTRIBUTE: u32 = 66796u32;
pub const DISPID_IHTMLELEMENT6_IE9_GETATTRIBUTENODE: u32 = 66792u32;
pub const DISPID_IHTMLELEMENT6_IE9_HASATTRIBUTE: u32 = 66795u32;
pub const DISPID_IHTMLELEMENT6_IE9_HASATTRIBUTES: u32 = 66815u32;
pub const DISPID_IHTMLELEMENT6_IE9_NODENAME: u32 = 66802u32;
pub const DISPID_IHTMLELEMENT6_IE9_REMOVEATTRIBUTE: u32 = 66798u32;
pub const DISPID_IHTMLELEMENT6_IE9_REMOVEATTRIBUTENODE: u32 = 66794u32;
pub const DISPID_IHTMLELEMENT6_IE9_SETATTRIBUTE: u32 = 66797u32;
pub const DISPID_IHTMLELEMENT6_IE9_SETATTRIBUTENODE: u32 = 66793u32;
pub const DISPID_IHTMLELEMENT6_IE9_TAGNAME: u32 = 66801u32;
pub const DISPID_IHTMLELEMENT6_MSMATCHESSELECTOR: u32 = 66814u32;
pub const DISPID_IHTMLELEMENT6_ONABORT: u32 = 71564u32;
pub const DISPID_IHTMLELEMENT6_ONCANPLAY: u32 = 71670u32;
pub const DISPID_IHTMLELEMENT6_ONCANPLAYTHROUGH: u32 = 71671u32;
pub const DISPID_IHTMLELEMENT6_ONCHANGE: u32 = 71566u32;
pub const DISPID_IHTMLELEMENT6_ONDURATIONCHANGE: u32 = 71672u32;
pub const DISPID_IHTMLELEMENT6_ONEMPTIED: u32 = 71673u32;
pub const DISPID_IHTMLELEMENT6_ONENDED: u32 = 71674u32;
pub const DISPID_IHTMLELEMENT6_ONERROR: u32 = 71565u32;
pub const DISPID_IHTMLELEMENT6_ONINPUT: u32 = 71663u32;
pub const DISPID_IHTMLELEMENT6_ONLOAD: u32 = 71568u32;
pub const DISPID_IHTMLELEMENT6_ONLOADEDDATA: u32 = 71675u32;
pub const DISPID_IHTMLELEMENT6_ONLOADEDMETADATA: u32 = 71676u32;
pub const DISPID_IHTMLELEMENT6_ONLOADSTART: u32 = 71677u32;
pub const DISPID_IHTMLELEMENT6_ONPAUSE: u32 = 71678u32;
pub const DISPID_IHTMLELEMENT6_ONPLAY: u32 = 71679u32;
pub const DISPID_IHTMLELEMENT6_ONPLAYING: u32 = 71680u32;
pub const DISPID_IHTMLELEMENT6_ONPROGRESS: u32 = 71681u32;
pub const DISPID_IHTMLELEMENT6_ONRATECHANGE: u32 = 71682u32;
pub const DISPID_IHTMLELEMENT6_ONRESET: u32 = 71548u32;
pub const DISPID_IHTMLELEMENT6_ONSEEKED: u32 = 71683u32;
pub const DISPID_IHTMLELEMENT6_ONSEEKING: u32 = 71684u32;
pub const DISPID_IHTMLELEMENT6_ONSELECT: u32 = 71546u32;
pub const DISPID_IHTMLELEMENT6_ONSTALLED: u32 = 71685u32;
pub const DISPID_IHTMLELEMENT6_ONSUBMIT: u32 = 71547u32;
pub const DISPID_IHTMLELEMENT6_ONSUSPEND: u32 = 71686u32;
pub const DISPID_IHTMLELEMENT6_ONTIMEUPDATE: u32 = 71687u32;
pub const DISPID_IHTMLELEMENT6_ONVOLUMECHANGE: u32 = 71688u32;
pub const DISPID_IHTMLELEMENT6_ONWAITING: u32 = 71689u32;
pub const DISPID_IHTMLELEMENT6_REMOVEATTRIBUTENS: u32 = 66791u32;
pub const DISPID_IHTMLELEMENT6_SETATTRIBUTENODENS: u32 = 66787u32;
pub const DISPID_IHTMLELEMENT6_SETATTRIBUTENS: u32 = 66790u32;
pub const DISPID_IHTMLELEMENT7_MSRELEASEPOINTERCAPTURE: u32 = 66823u32;
pub const DISPID_IHTMLELEMENT7_MSSETPOINTERCAPTURE: u32 = 66822u32;
pub const DISPID_IHTMLELEMENT7_ONCUECHANGE: u32 = 71729u32;
pub const DISPID_IHTMLELEMENT7_ONINVALID: u32 = 71724u32;
pub const DISPID_IHTMLELEMENT7_ONMSANIMATIONEND: u32 = 71712u32;
pub const DISPID_IHTMLELEMENT7_ONMSANIMATIONITERATION: u32 = 71713u32;
pub const DISPID_IHTMLELEMENT7_ONMSANIMATIONSTART: u32 = 71711u32;
pub const DISPID_IHTMLELEMENT7_ONMSGESTURECHANGE: u32 = 71700u32;
pub const DISPID_IHTMLELEMENT7_ONMSGESTUREDOUBLETAP: u32 = 71704u32;
pub const DISPID_IHTMLELEMENT7_ONMSGESTUREEND: u32 = 71701u32;
pub const DISPID_IHTMLELEMENT7_ONMSGESTUREHOLD: u32 = 71702u32;
pub const DISPID_IHTMLELEMENT7_ONMSGESTURESTART: u32 = 71699u32;
pub const DISPID_IHTMLELEMENT7_ONMSGESTURETAP: u32 = 71703u32;
pub const DISPID_IHTMLELEMENT7_ONMSGOTPOINTERCAPTURE: u32 = 71707u32;
pub const DISPID_IHTMLELEMENT7_ONMSINERTIASTART: u32 = 71705u32;
pub const DISPID_IHTMLELEMENT7_ONMSLOSTPOINTERCAPTURE: u32 = 71706u32;
pub const DISPID_IHTMLELEMENT7_ONMSMANIPULATIONSTATECHANGED: u32 = 71714u32;
pub const DISPID_IHTMLELEMENT7_ONMSPOINTERCANCEL: u32 = 71695u32;
pub const DISPID_IHTMLELEMENT7_ONMSPOINTERDOWN: u32 = 71690u32;
pub const DISPID_IHTMLELEMENT7_ONMSPOINTERHOVER: u32 = 71696u32;
pub const DISPID_IHTMLELEMENT7_ONMSPOINTERMOVE: u32 = 71691u32;
pub const DISPID_IHTMLELEMENT7_ONMSPOINTEROUT: u32 = 71694u32;
pub const DISPID_IHTMLELEMENT7_ONMSPOINTEROVER: u32 = 71693u32;
pub const DISPID_IHTMLELEMENT7_ONMSPOINTERUP: u32 = 71692u32;
pub const DISPID_IHTMLELEMENT7_ONMSTRANSITIONEND: u32 = 71710u32;
pub const DISPID_IHTMLELEMENT7_ONMSTRANSITIONSTART: u32 = 71709u32;
pub const DISPID_IHTMLELEMENT7_SPELLCHECK: u32 = 70907u32;
pub const DISPID_IHTMLELEMENT7_XMSACCELERATORKEY: u32 = 66834u32;
pub const DISPID_IHTMLELEMENTAPPLIEDSTYLES_MSGETRULESAPPLIED: u32 = 66652u32;
pub const DISPID_IHTMLELEMENTAPPLIEDSTYLES_MSGETRULESAPPLIEDWITHANCESTOR: u32 = 66653u32;
pub const DISPID_IHTMLELEMENTCOLLECTION2_URNS: u32 = 1505u32;
pub const DISPID_IHTMLELEMENTCOLLECTION3_NAMEDITEM: u32 = 1506u32;
pub const DISPID_IHTMLELEMENTCOLLECTION4_IE8_ITEM: u32 = 1152u32;
pub const DISPID_IHTMLELEMENTCOLLECTION4_IE8_LENGTH: u32 = 1150u32;
pub const DISPID_IHTMLELEMENTCOLLECTION4_IE8_NAMEDITEM: u32 = 1153u32;
pub const DISPID_IHTMLELEMENTCOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLELEMENTCOLLECTION_LENGTH: u32 = 1500u32;
pub const DISPID_IHTMLELEMENTCOLLECTION_TAGS: u32 = 1502u32;
pub const DISPID_IHTMLELEMENTCOLLECTION_TOSTRING: u32 = 1501u32;
pub const DISPID_IHTMLELEMENTCOLLECTION__NEWENUM: i32 = -4i32;
pub const DISPID_IHTMLELEMENTDEFAULTS_CANHAVEHTML: u32 = 1009u32;
pub const DISPID_IHTMLELEMENTDEFAULTS_CONTENTEDITABLE: u32 = 70698u32;
pub const DISPID_IHTMLELEMENTDEFAULTS_FROZEN: u32 = 70734u32;
pub const DISPID_IHTMLELEMENTDEFAULTS_ISMULTILINE: u32 = 1008u32;
pub const DISPID_IHTMLELEMENTDEFAULTS_SCROLLSEGMENTX: u32 = 1003u32;
pub const DISPID_IHTMLELEMENTDEFAULTS_SCROLLSEGMENTY: u32 = 1004u32;
pub const DISPID_IHTMLELEMENTDEFAULTS_STYLE: u32 = 1001u32;
pub const DISPID_IHTMLELEMENTDEFAULTS_TABSTOP: u32 = 1002u32;
pub const DISPID_IHTMLELEMENTDEFAULTS_VIEWINHERITSTYLE: u32 = 70735u32;
pub const DISPID_IHTMLELEMENTDEFAULTS_VIEWLINK: u32 = 1011u32;
pub const DISPID_IHTMLELEMENTDEFAULTS_VIEWMASTERTAB: u32 = 1006u32;
pub const DISPID_IHTMLELEMENT_ALL: u32 = 66574u32;
pub const DISPID_IHTMLELEMENT_CHILDREN: u32 = 66573u32;
pub const DISPID_IHTMLELEMENT_CLASSNAME: u32 = 66537u32;
pub const DISPID_IHTMLELEMENT_CLICK: u32 = 66569u32;
pub const DISPID_IHTMLELEMENT_CONTAINS: u32 = 66556u32;
pub const DISPID_IHTMLELEMENT_DOCUMENT: u32 = 66554u32;
pub const DISPID_IHTMLELEMENT_FILTERS: u32 = 66571u32;
pub const DISPID_IHTMLELEMENT_GETATTRIBUTE: u32 = 66038u32;
pub const DISPID_IHTMLELEMENT_ID: u32 = 66538u32;
pub const DISPID_IHTMLELEMENT_INNERHTML: u32 = 66562u32;
pub const DISPID_IHTMLELEMENT_INNERTEXT: u32 = 66563u32;
pub const DISPID_IHTMLELEMENT_INSERTADJACENTHTML: u32 = 66566u32;
pub const DISPID_IHTMLELEMENT_INSERTADJACENTTEXT: u32 = 66567u32;
pub const DISPID_IHTMLELEMENT_ISTEXTEDIT: u32 = 66570u32;
pub const DISPID_IHTMLELEMENT_LANG: u32 = 70545u32;
pub const DISPID_IHTMLELEMENT_LANGUAGE: u32 = 70636u32;
pub const DISPID_IHTMLELEMENT_OFFSETHEIGHT: u32 = 66547u32;
pub const DISPID_IHTMLELEMENT_OFFSETLEFT: u32 = 66544u32;
pub const DISPID_IHTMLELEMENT_OFFSETPARENT: u32 = 66548u32;
pub const DISPID_IHTMLELEMENT_OFFSETTOP: u32 = 66545u32;
pub const DISPID_IHTMLELEMENT_OFFSETWIDTH: u32 = 66546u32;
pub const DISPID_IHTMLELEMENT_ONAFTERUPDATE: u32 = 71558u32;
pub const DISPID_IHTMLELEMENT_ONBEFOREUPDATE: u32 = 71557u32;
pub const DISPID_IHTMLELEMENT_ONCLICK: u32 = 71544u32;
pub const DISPID_IHTMLELEMENT_ONDATAAVAILABLE: u32 = 71577u32;
pub const DISPID_IHTMLELEMENT_ONDATASETCHANGED: u32 = 71576u32;
pub const DISPID_IHTMLELEMENT_ONDATASETCOMPLETE: u32 = 71578u32;
pub const DISPID_IHTMLELEMENT_ONDBLCLICK: u32 = 71545u32;
pub const DISPID_IHTMLELEMENT_ONDRAGSTART: u32 = 71571u32;
pub const DISPID_IHTMLELEMENT_ONERRORUPDATE: u32 = 71574u32;
pub const DISPID_IHTMLELEMENT_ONFILTERCHANGE: u32 = 71579u32;
pub const DISPID_IHTMLELEMENT_ONHELP: u32 = 71549u32;
pub const DISPID_IHTMLELEMENT_ONKEYDOWN: u32 = 71541u32;
pub const DISPID_IHTMLELEMENT_ONKEYPRESS: u32 = 71543u32;
pub const DISPID_IHTMLELEMENT_ONKEYUP: u32 = 71542u32;
pub const DISPID_IHTMLELEMENT_ONMOUSEDOWN: u32 = 71538u32;
pub const DISPID_IHTMLELEMENT_ONMOUSEMOVE: u32 = 71540u32;
pub const DISPID_IHTMLELEMENT_ONMOUSEOUT: u32 = 71537u32;
pub const DISPID_IHTMLELEMENT_ONMOUSEOVER: u32 = 71536u32;
pub const DISPID_IHTMLELEMENT_ONMOUSEUP: u32 = 71539u32;
pub const DISPID_IHTMLELEMENT_ONROWENTER: u32 = 71555u32;
pub const DISPID_IHTMLELEMENT_ONROWEXIT: u32 = 71554u32;
pub const DISPID_IHTMLELEMENT_ONSELECTSTART: u32 = 71573u32;
pub const DISPID_IHTMLELEMENT_OUTERHTML: u32 = 66564u32;
pub const DISPID_IHTMLELEMENT_OUTERTEXT: u32 = 66565u32;
pub const DISPID_IHTMLELEMENT_PARENTELEMENT: u32 = 65544u32;
pub const DISPID_IHTMLELEMENT_PARENTTEXTEDIT: u32 = 66568u32;
pub const DISPID_IHTMLELEMENT_RECORDNUMBER: u32 = 66561u32;
pub const DISPID_IHTMLELEMENT_REMOVEATTRIBUTE: u32 = 66039u32;
pub const DISPID_IHTMLELEMENT_SCROLLINTOVIEW: u32 = 66555u32;
pub const DISPID_IHTMLELEMENT_SETATTRIBUTE: u32 = 66037u32;
pub const DISPID_IHTMLELEMENT_SOURCEINDEX: u32 = 66560u32;
pub const DISPID_IHTMLELEMENT_STYLE: u32 = 65610u32;
pub const DISPID_IHTMLELEMENT_TAGNAME: u32 = 66540u32;
pub const DISPID_IHTMLELEMENT_TITLE: u32 = 65605u32;
pub const DISPID_IHTMLELEMENT_TOSTRING: u32 = 66572u32;
pub const DISPID_IHTMLEMBEDELEMENT2_IE8_PLUGINSPAGE: u32 = 1151u32;
pub const DISPID_IHTMLEMBEDELEMENT2_IE8_SRC: u32 = 1150u32;
pub const DISPID_IHTMLEMBEDELEMENT_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLEMBEDELEMENT_HIDDEN: u32 = 68546u32;
pub const DISPID_IHTMLEMBEDELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLEMBEDELEMENT_PALETTE: u32 = 68540u32;
pub const DISPID_IHTMLEMBEDELEMENT_PLUGINSPAGE: u32 = 68541u32;
pub const DISPID_IHTMLEMBEDELEMENT_SRC: u32 = 68542u32;
pub const DISPID_IHTMLEMBEDELEMENT_UNITS: u32 = 68544u32;
pub const DISPID_IHTMLEMBEDELEMENT_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLEVENTOBJ2_ALTKEY: u32 = 1002u32;
pub const DISPID_IHTMLEVENTOBJ2_BOOKMARKS: u32 = 1031u32;
pub const DISPID_IHTMLEVENTOBJ2_BOUNDELEMENTS: u32 = 1034u32;
pub const DISPID_IHTMLEVENTOBJ2_BUTTON: u32 = 1012u32;
pub const DISPID_IHTMLEVENTOBJ2_CLIENTX: u32 = 1020u32;
pub const DISPID_IHTMLEVENTOBJ2_CLIENTY: u32 = 1021u32;
pub const DISPID_IHTMLEVENTOBJ2_CTRLKEY: u32 = 1003u32;
pub const DISPID_IHTMLEVENTOBJ2_DATAFLD: u32 = 1033u32;
pub const DISPID_IHTMLEVENTOBJ2_DATATRANSFER: u32 = 1037u32;
pub const DISPID_IHTMLEVENTOBJ2_FROMELEMENT: u32 = 1009u32;
pub const DISPID_IHTMLEVENTOBJ2_GETATTRIBUTE: u32 = 66038u32;
pub const DISPID_IHTMLEVENTOBJ2_OFFSETX: u32 = 1022u32;
pub const DISPID_IHTMLEVENTOBJ2_OFFSETY: u32 = 1023u32;
pub const DISPID_IHTMLEVENTOBJ2_PROPERTYNAME: u32 = 1027u32;
pub const DISPID_IHTMLEVENTOBJ2_QUALIFIER: u32 = 1014u32;
pub const DISPID_IHTMLEVENTOBJ2_REASON: u32 = 1015u32;
pub const DISPID_IHTMLEVENTOBJ2_RECORDSET: u32 = 1032u32;
pub const DISPID_IHTMLEVENTOBJ2_REMOVEATTRIBUTE: u32 = 66039u32;
pub const DISPID_IHTMLEVENTOBJ2_REPEAT: u32 = 1035u32;
pub const DISPID_IHTMLEVENTOBJ2_SCREENX: u32 = 1024u32;
pub const DISPID_IHTMLEVENTOBJ2_SCREENY: u32 = 1025u32;
pub const DISPID_IHTMLEVENTOBJ2_SETATTRIBUTE: u32 = 66037u32;
pub const DISPID_IHTMLEVENTOBJ2_SHIFTKEY: u32 = 1004u32;
pub const DISPID_IHTMLEVENTOBJ2_SRCELEMENT: u32 = 1001u32;
pub const DISPID_IHTMLEVENTOBJ2_SRCFILTER: u32 = 1026u32;
pub const DISPID_IHTMLEVENTOBJ2_SRCURN: u32 = 1036u32;
pub const DISPID_IHTMLEVENTOBJ2_TOELEMENT: u32 = 1010u32;
pub const DISPID_IHTMLEVENTOBJ2_TYPE: u32 = 1013u32;
pub const DISPID_IHTMLEVENTOBJ2_X: u32 = 1005u32;
pub const DISPID_IHTMLEVENTOBJ2_Y: u32 = 1006u32;
pub const DISPID_IHTMLEVENTOBJ3_ALTLEFT: u32 = 1040u32;
pub const DISPID_IHTMLEVENTOBJ3_BEHAVIORCOOKIE: u32 = 1048u32;
pub const DISPID_IHTMLEVENTOBJ3_BEHAVIORPART: u32 = 1049u32;
pub const DISPID_IHTMLEVENTOBJ3_CONTENTOVERFLOW: u32 = 1038u32;
pub const DISPID_IHTMLEVENTOBJ3_CTRLLEFT: u32 = 1041u32;
pub const DISPID_IHTMLEVENTOBJ3_IMECOMPOSITIONCHANGE: u32 = 1042u32;
pub const DISPID_IHTMLEVENTOBJ3_IMENOTIFYCOMMAND: u32 = 1043u32;
pub const DISPID_IHTMLEVENTOBJ3_IMENOTIFYDATA: u32 = 1044u32;
pub const DISPID_IHTMLEVENTOBJ3_IMEREQUEST: u32 = 1046u32;
pub const DISPID_IHTMLEVENTOBJ3_IMEREQUESTDATA: u32 = 1047u32;
pub const DISPID_IHTMLEVENTOBJ3_KEYBOARDLAYOUT: u32 = 1045u32;
pub const DISPID_IHTMLEVENTOBJ3_NEXTPAGE: u32 = 1050u32;
pub const DISPID_IHTMLEVENTOBJ3_SHIFTLEFT: u32 = 1039u32;
pub const DISPID_IHTMLEVENTOBJ4_WHEELDELTA: u32 = 1051u32;
pub const DISPID_IHTMLEVENTOBJ5_DATA: u32 = 1054u32;
pub const DISPID_IHTMLEVENTOBJ5_ISSESSION: u32 = 1056u32;
pub const DISPID_IHTMLEVENTOBJ5_ORIGIN: u32 = 1053u32;
pub const DISPID_IHTMLEVENTOBJ5_SOURCE: u32 = 1055u32;
pub const DISPID_IHTMLEVENTOBJ5_URL: u32 = 1052u32;
pub const DISPID_IHTMLEVENTOBJ6_ACTIONURL: u32 = 1058u32;
pub const DISPID_IHTMLEVENTOBJ6_BUTTONID: u32 = 1057u32;
pub const DISPID_IHTMLEVENTOBJ_ALTKEY: u32 = 1002u32;
pub const DISPID_IHTMLEVENTOBJ_BUTTON: u32 = 1012u32;
pub const DISPID_IHTMLEVENTOBJ_CANCELBUBBLE: u32 = 1008u32;
pub const DISPID_IHTMLEVENTOBJ_CLIENTX: u32 = 1020u32;
pub const DISPID_IHTMLEVENTOBJ_CLIENTY: u32 = 1021u32;
pub const DISPID_IHTMLEVENTOBJ_CTRLKEY: u32 = 1003u32;
pub const DISPID_IHTMLEVENTOBJ_FROMELEMENT: u32 = 1009u32;
pub const DISPID_IHTMLEVENTOBJ_KEYCODE: u32 = 1011u32;
pub const DISPID_IHTMLEVENTOBJ_OFFSETX: u32 = 1022u32;
pub const DISPID_IHTMLEVENTOBJ_OFFSETY: u32 = 1023u32;
pub const DISPID_IHTMLEVENTOBJ_QUALIFIER: u32 = 1014u32;
pub const DISPID_IHTMLEVENTOBJ_REASON: u32 = 1015u32;
pub const DISPID_IHTMLEVENTOBJ_RETURNVALUE: u32 = 1007u32;
pub const DISPID_IHTMLEVENTOBJ_SCREENX: u32 = 1024u32;
pub const DISPID_IHTMLEVENTOBJ_SCREENY: u32 = 1025u32;
pub const DISPID_IHTMLEVENTOBJ_SHIFTKEY: u32 = 1004u32;
pub const DISPID_IHTMLEVENTOBJ_SRCELEMENT: u32 = 1001u32;
pub const DISPID_IHTMLEVENTOBJ_SRCFILTER: u32 = 1026u32;
pub const DISPID_IHTMLEVENTOBJ_TOELEMENT: u32 = 1010u32;
pub const DISPID_IHTMLEVENTOBJ_TYPE: u32 = 1013u32;
pub const DISPID_IHTMLEVENTOBJ_X: u32 = 1005u32;
pub const DISPID_IHTMLEVENTOBJ_Y: u32 = 1006u32;
pub const DISPID_IHTMLFIELDSETELEMENT2_FORM: u32 = 67540u32;
pub const DISPID_IHTMLFIELDSETELEMENT_ALIGN: u32 = 65609u32;
pub const DISPID_IHTMLFILTERSCOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLFILTERSCOLLECTION_LENGTH: u32 = 1001u32;
pub const DISPID_IHTMLFILTERSCOLLECTION__NEWENUM: i32 = -4i32;
pub const DISPID_IHTMLFONTELEMENT_COLOR: u32 = 70538u32;
pub const DISPID_IHTMLFONTELEMENT_FACE: u32 = 70554u32;
pub const DISPID_IHTMLFONTELEMENT_SIZE: u32 = 70555u32;
pub const DISPID_IHTMLFONTNAMESCOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLFONTNAMESCOLLECTION_LENGTH: u32 = 1501u32;
pub const DISPID_IHTMLFONTNAMESCOLLECTION__NEWENUM: i32 = -4i32;
pub const DISPID_IHTMLFONTSIZESCOLLECTION_FORFONT: u32 = 1503u32;
pub const DISPID_IHTMLFONTSIZESCOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLFONTSIZESCOLLECTION_LENGTH: u32 = 1502u32;
pub const DISPID_IHTMLFONTSIZESCOLLECTION__NEWENUM: i32 = -4i32;
pub const DISPID_IHTMLFORMELEMENT2_ACCEPTCHARSET: u32 = 1011u32;
pub const DISPID_IHTMLFORMELEMENT2_URNS: u32 = 1505u32;
pub const DISPID_IHTMLFORMELEMENT3_NAMEDITEM: u32 = 1506u32;
pub const DISPID_IHTMLFORMELEMENT4_IE8_ACTION: u32 = 1150u32;
pub const DISPID_IHTMLFORMELEMENT_ACTION: u32 = 1001u32;
pub const DISPID_IHTMLFORMELEMENT_DIR: u32 = 70653u32;
pub const DISPID_IHTMLFORMELEMENT_ELEMENTS: u32 = 1005u32;
pub const DISPID_IHTMLFORMELEMENT_ENCODING: u32 = 1003u32;
pub const DISPID_IHTMLFORMELEMENT_ITEM: u32 = 0u32;
pub const DISPID_IHTMLFORMELEMENT_LENGTH: u32 = 1500u32;
pub const DISPID_IHTMLFORMELEMENT_METHOD: u32 = 1004u32;
pub const DISPID_IHTMLFORMELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLFORMELEMENT_ONRESET: u32 = 71548u32;
pub const DISPID_IHTMLFORMELEMENT_ONSUBMIT: u32 = 71547u32;
pub const DISPID_IHTMLFORMELEMENT_RESET: u32 = 1010u32;
pub const DISPID_IHTMLFORMELEMENT_SUBMIT: u32 = 1009u32;
pub const DISPID_IHTMLFORMELEMENT_TAGS: u32 = 1502u32;
pub const DISPID_IHTMLFORMELEMENT_TARGET: u32 = 1006u32;
pub const DISPID_IHTMLFORMELEMENT__NEWENUM: i32 = -4i32;
pub const DISPID_IHTMLFRAMEBASE2_ALLOWTRANSPARENCY: u32 = 70742u32;
pub const DISPID_IHTMLFRAMEBASE2_CONTENTWINDOW: u32 = 68545u32;
pub const DISPID_IHTMLFRAMEBASE2_ONLOAD: u32 = 71568u32;
pub const DISPID_IHTMLFRAMEBASE2_ONREADYSTATECHANGE: u32 = 71561u32;
pub const DISPID_IHTMLFRAMEBASE2_READYSTATE: u32 = 70652u32;
pub const DISPID_IHTMLFRAMEBASE3_LONGDESC: u32 = 68546u32;
pub const DISPID_IHTMLFRAMEBASE_BORDER: u32 = 68538u32;
pub const DISPID_IHTMLFRAMEBASE_FRAMEBORDER: u32 = 68539u32;
pub const DISPID_IHTMLFRAMEBASE_FRAMESPACING: u32 = 68540u32;
pub const DISPID_IHTMLFRAMEBASE_MARGINHEIGHT: u32 = 68542u32;
pub const DISPID_IHTMLFRAMEBASE_MARGINWIDTH: u32 = 68541u32;
pub const DISPID_IHTMLFRAMEBASE_NAME: u32 = 65536u32;
pub const DISPID_IHTMLFRAMEBASE_NORESIZE: u32 = 68543u32;
pub const DISPID_IHTMLFRAMEBASE_SCROLLING: u32 = 68544u32;
pub const DISPID_IHTMLFRAMEBASE_SRC: u32 = 68536u32;
pub const DISPID_IHTMLFRAMEELEMENT2_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLFRAMEELEMENT2_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLFRAMEELEMENT3_CONTENTDOCUMENT: u32 = 69656u32;
pub const DISPID_IHTMLFRAMEELEMENT3_IE8_FRAMEBORDER: u32 = 69659u32;
pub const DISPID_IHTMLFRAMEELEMENT3_IE8_LONGDESC: u32 = 69658u32;
pub const DISPID_IHTMLFRAMEELEMENT3_IE8_SRC: u32 = 69657u32;
pub const DISPID_IHTMLFRAMEELEMENT_BORDERCOLOR: u32 = 69537u32;
pub const DISPID_IHTMLFRAMESCOLLECTION2_ITEM: u32 = 0u32;
pub const DISPID_IHTMLFRAMESCOLLECTION2_LENGTH: u32 = 1001u32;
pub const DISPID_IHTMLFRAMESETELEMENT2_ONAFTERPRINT: u32 = 71603u32;
pub const DISPID_IHTMLFRAMESETELEMENT2_ONBEFOREPRINT: u32 = 71602u32;
pub const DISPID_IHTMLFRAMESETELEMENT3_ONHASHCHANGE: u32 = 71645u32;
pub const DISPID_IHTMLFRAMESETELEMENT3_ONMESSAGE: u32 = 71646u32;
pub const DISPID_IHTMLFRAMESETELEMENT3_ONOFFLINE: u32 = 71644u32;
pub const DISPID_IHTMLFRAMESETELEMENT3_ONONLINE: u32 = 71643u32;
pub const DISPID_IHTMLFRAMESETELEMENT3_ONSTORAGE: u32 = 71636u32;
pub const DISPID_IHTMLFRAMESETELEMENT_BORDER: u32 = 1002u32;
pub const DISPID_IHTMLFRAMESETELEMENT_BORDERCOLOR: u32 = 1003u32;
pub const DISPID_IHTMLFRAMESETELEMENT_COLS: u32 = 1001u32;
pub const DISPID_IHTMLFRAMESETELEMENT_FRAMEBORDER: u32 = 1004u32;
pub const DISPID_IHTMLFRAMESETELEMENT_FRAMESPACING: u32 = 1005u32;
pub const DISPID_IHTMLFRAMESETELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLFRAMESETELEMENT_ONBEFOREUNLOAD: u32 = 71575u32;
pub const DISPID_IHTMLFRAMESETELEMENT_ONLOAD: u32 = 71568u32;
pub const DISPID_IHTMLFRAMESETELEMENT_ONUNLOAD: u32 = 71569u32;
pub const DISPID_IHTMLFRAMESETELEMENT_ROWS: u32 = 1000u32;
pub const DISPID_IHTMLGENERICELEMENT_NAMEDRECORDSET: u32 = 1002u32;
pub const DISPID_IHTMLGENERICELEMENT_RECORDSET: u32 = 1001u32;
pub const DISPID_IHTMLHEADELEMENT2_IE8_PROFILE: u32 = 1150u32;
pub const DISPID_IHTMLHEADELEMENT_PROFILE: u32 = 1001u32;
pub const DISPID_IHTMLHEADERELEMENT_ALIGN: u32 = 65608u32;
pub const DISPID_IHTMLHRELEMENT_ALIGN: u32 = 65608u32;
pub const DISPID_IHTMLHRELEMENT_COLOR: u32 = 70538u32;
pub const DISPID_IHTMLHRELEMENT_NOSHADE: u32 = 1001u32;
pub const DISPID_IHTMLHRELEMENT_SIZE: u32 = 65542u32;
pub const DISPID_IHTMLHRELEMENT_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLHTMLELEMENT_VERSION: u32 = 1001u32;
pub const DISPID_IHTMLIFRAMEELEMENT2_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLIFRAMEELEMENT2_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLIFRAMEELEMENT3_CONTENTDOCUMENT: u32 = 69656u32;
pub const DISPID_IHTMLIFRAMEELEMENT3_IE8_FRAMEBORDER: u32 = 69659u32;
pub const DISPID_IHTMLIFRAMEELEMENT3_IE8_LONGDESC: u32 = 69658u32;
pub const DISPID_IHTMLIFRAMEELEMENT3_IE8_SRC: u32 = 69657u32;
pub const DISPID_IHTMLIFRAMEELEMENT_ALIGN: u32 = 65609u32;
pub const DISPID_IHTMLIFRAMEELEMENT_HSPACE: u32 = 69538u32;
pub const DISPID_IHTMLIFRAMEELEMENT_VSPACE: u32 = 69537u32;
pub const DISPID_IHTMLIMAGEELEMENTFACTORY_CREATE: u32 = 0u32;
pub const DISPID_IHTMLIMGELEMENT2_LONGDESC: u32 = 2019u32;
pub const DISPID_IHTMLIMGELEMENT3_IE8_DYNSRC: u32 = 1154u32;
pub const DISPID_IHTMLIMGELEMENT3_IE8_LONGDESC: u32 = 1151u32;
pub const DISPID_IHTMLIMGELEMENT3_IE8_LOWSRC: u32 = 1153u32;
pub const DISPID_IHTMLIMGELEMENT3_IE8_VRML: u32 = 1152u32;
pub const DISPID_IHTMLIMGELEMENT4_NATURALHEIGHT: u32 = 1156u32;
pub const DISPID_IHTMLIMGELEMENT4_NATURALWIDTH: u32 = 1155u32;
pub const DISPID_IHTMLIMGELEMENT_ALIGN: u32 = 65609u32;
pub const DISPID_IHTMLIMGELEMENT_ALT: u32 = 1002u32;
pub const DISPID_IHTMLIMGELEMENT_BORDER: u32 = 1004u32;
pub const DISPID_IHTMLIMGELEMENT_COMPLETE: u32 = 1010u32;
pub const DISPID_IHTMLIMGELEMENT_DYNSRC: u32 = 1009u32;
pub const DISPID_IHTMLIMGELEMENT_FILECREATEDDATE: u32 = 2012u32;
pub const DISPID_IHTMLIMGELEMENT_FILEMODIFIEDDATE: u32 = 2013u32;
pub const DISPID_IHTMLIMGELEMENT_FILESIZE: u32 = 2011u32;
pub const DISPID_IHTMLIMGELEMENT_FILEUPDATEDDATE: u32 = 2014u32;
pub const DISPID_IHTMLIMGELEMENT_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLIMGELEMENT_HREF: u32 = 2016u32;
pub const DISPID_IHTMLIMGELEMENT_HSPACE: u32 = 1006u32;
pub const DISPID_IHTMLIMGELEMENT_ISMAP: u32 = 2002u32;
pub const DISPID_IHTMLIMGELEMENT_LOOP: u32 = 1011u32;
pub const DISPID_IHTMLIMGELEMENT_LOWSRC: u32 = 1007u32;
pub const DISPID_IHTMLIMGELEMENT_MIMETYPE: u32 = 2010u32;
pub const DISPID_IHTMLIMGELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLIMGELEMENT_NAMEPROP: u32 = 2017u32;
pub const DISPID_IHTMLIMGELEMENT_ONABORT: u32 = 71564u32;
pub const DISPID_IHTMLIMGELEMENT_ONERROR: u32 = 71565u32;
pub const DISPID_IHTMLIMGELEMENT_ONLOAD: u32 = 71568u32;
pub const DISPID_IHTMLIMGELEMENT_PROTOCOL: u32 = 2015u32;
pub const DISPID_IHTMLIMGELEMENT_READYSTATE: u32 = 70652u32;
pub const DISPID_IHTMLIMGELEMENT_SRC: u32 = 1003u32;
pub const DISPID_IHTMLIMGELEMENT_START: u32 = 1013u32;
pub const DISPID_IHTMLIMGELEMENT_USEMAP: u32 = 2008u32;
pub const DISPID_IHTMLIMGELEMENT_VRML: u32 = 1008u32;
pub const DISPID_IHTMLIMGELEMENT_VSPACE: u32 = 1005u32;
pub const DISPID_IHTMLIMGELEMENT_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLINPUTBUTTONELEMENT_CREATETEXTRANGE: u32 = 2006u32;
pub const DISPID_IHTMLINPUTBUTTONELEMENT_DISABLED: u32 = 65612u32;
pub const DISPID_IHTMLINPUTBUTTONELEMENT_FORM: u32 = 67540u32;
pub const DISPID_IHTMLINPUTBUTTONELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLINPUTBUTTONELEMENT_STATUS: u32 = 2021u32;
pub const DISPID_IHTMLINPUTBUTTONELEMENT_TYPE: u32 = 2000u32;
pub const DISPID_IHTMLINPUTBUTTONELEMENT_VALUE: u32 = 70637u32;
pub const DISPID_IHTMLINPUTELEMENT2_ACCEPT: u32 = 2022u32;
pub const DISPID_IHTMLINPUTELEMENT2_USEMAP: u32 = 2023u32;
pub const DISPID_IHTMLINPUTELEMENT3_IE8_DYNSRC: u32 = 1153u32;
pub const DISPID_IHTMLINPUTELEMENT3_IE8_LOWSRC: u32 = 1151u32;
pub const DISPID_IHTMLINPUTELEMENT3_IE8_SRC: u32 = 1150u32;
pub const DISPID_IHTMLINPUTELEMENT3_IE8_VRML: u32 = 1152u32;
pub const DISPID_IHTMLINPUTELEMENT_ALIGN: u32 = 65609u32;
pub const DISPID_IHTMLINPUTELEMENT_ALT: u32 = 2010u32;
pub const DISPID_IHTMLINPUTELEMENT_BORDER: u32 = 2012u32;
pub const DISPID_IHTMLINPUTELEMENT_CHECKED: u32 = 2009u32;
pub const DISPID_IHTMLINPUTELEMENT_COMPLETE: u32 = 2018u32;
pub const DISPID_IHTMLINPUTELEMENT_CREATETEXTRANGE: u32 = 2006u32;
pub const DISPID_IHTMLINPUTELEMENT_DEFAULTCHECKED: u32 = 2008u32;
pub const DISPID_IHTMLINPUTELEMENT_DEFAULTVALUE: u32 = 70619u32;
pub const DISPID_IHTMLINPUTELEMENT_DISABLED: u32 = 65612u32;
pub const DISPID_IHTMLINPUTELEMENT_DYNSRC: u32 = 2017u32;
pub const DISPID_IHTMLINPUTELEMENT_FORM: u32 = 67540u32;
pub const DISPID_IHTMLINPUTELEMENT_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLINPUTELEMENT_HSPACE: u32 = 2014u32;
pub const DISPID_IHTMLINPUTELEMENT_INDETERMINATE: u32 = 2007u32;
pub const DISPID_IHTMLINPUTELEMENT_LOOP: u32 = 2019u32;
pub const DISPID_IHTMLINPUTELEMENT_LOWSRC: u32 = 2015u32;
pub const DISPID_IHTMLINPUTELEMENT_MAXLENGTH: u32 = 2003u32;
pub const DISPID_IHTMLINPUTELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLINPUTELEMENT_ONABORT: u32 = 71564u32;
pub const DISPID_IHTMLINPUTELEMENT_ONCHANGE: u32 = 71566u32;
pub const DISPID_IHTMLINPUTELEMENT_ONERROR: u32 = 71565u32;
pub const DISPID_IHTMLINPUTELEMENT_ONLOAD: u32 = 71568u32;
pub const DISPID_IHTMLINPUTELEMENT_ONSELECT: u32 = 71546u32;
pub const DISPID_IHTMLINPUTELEMENT_READONLY: u32 = 2005u32;
pub const DISPID_IHTMLINPUTELEMENT_READYSTATE: u32 = 70652u32;
pub const DISPID_IHTMLINPUTELEMENT_SELECT: u32 = 2004u32;
pub const DISPID_IHTMLINPUTELEMENT_SIZE: u32 = 2002u32;
pub const DISPID_IHTMLINPUTELEMENT_SRC: u32 = 2011u32;
pub const DISPID_IHTMLINPUTELEMENT_START: u32 = 2020u32;
pub const DISPID_IHTMLINPUTELEMENT_STATUS: u32 = 2001u32;
pub const DISPID_IHTMLINPUTELEMENT_TYPE: u32 = 2000u32;
pub const DISPID_IHTMLINPUTELEMENT_VALUE: u32 = 70637u32;
pub const DISPID_IHTMLINPUTELEMENT_VRML: u32 = 2016u32;
pub const DISPID_IHTMLINPUTELEMENT_VSPACE: u32 = 2013u32;
pub const DISPID_IHTMLINPUTELEMENT_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLINPUTFILEELEMENT_DISABLED: u32 = 65612u32;
pub const DISPID_IHTMLINPUTFILEELEMENT_FORM: u32 = 67540u32;
pub const DISPID_IHTMLINPUTFILEELEMENT_MAXLENGTH: u32 = 2003u32;
pub const DISPID_IHTMLINPUTFILEELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLINPUTFILEELEMENT_ONCHANGE: u32 = 71566u32;
pub const DISPID_IHTMLINPUTFILEELEMENT_ONSELECT: u32 = 71546u32;
pub const DISPID_IHTMLINPUTFILEELEMENT_SELECT: u32 = 2004u32;
pub const DISPID_IHTMLINPUTFILEELEMENT_SIZE: u32 = 2002u32;
pub const DISPID_IHTMLINPUTFILEELEMENT_STATUS: u32 = 2021u32;
pub const DISPID_IHTMLINPUTFILEELEMENT_TYPE: u32 = 2000u32;
pub const DISPID_IHTMLINPUTFILEELEMENT_VALUE: u32 = 70637u32;
pub const DISPID_IHTMLINPUTHIDDENELEMENT_CREATETEXTRANGE: u32 = 2006u32;
pub const DISPID_IHTMLINPUTHIDDENELEMENT_DISABLED: u32 = 65612u32;
pub const DISPID_IHTMLINPUTHIDDENELEMENT_FORM: u32 = 67540u32;
pub const DISPID_IHTMLINPUTHIDDENELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLINPUTHIDDENELEMENT_STATUS: u32 = 2021u32;
pub const DISPID_IHTMLINPUTHIDDENELEMENT_TYPE: u32 = 2000u32;
pub const DISPID_IHTMLINPUTHIDDENELEMENT_VALUE: u32 = 70637u32;
pub const DISPID_IHTMLINPUTIMAGE_ALIGN: u32 = 65609u32;
pub const DISPID_IHTMLINPUTIMAGE_ALT: u32 = 2010u32;
pub const DISPID_IHTMLINPUTIMAGE_BORDER: u32 = 2012u32;
pub const DISPID_IHTMLINPUTIMAGE_COMPLETE: u32 = 2018u32;
pub const DISPID_IHTMLINPUTIMAGE_DISABLED: u32 = 65612u32;
pub const DISPID_IHTMLINPUTIMAGE_DYNSRC: u32 = 2017u32;
pub const DISPID_IHTMLINPUTIMAGE_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLINPUTIMAGE_HSPACE: u32 = 2014u32;
pub const DISPID_IHTMLINPUTIMAGE_LOOP: u32 = 2019u32;
pub const DISPID_IHTMLINPUTIMAGE_LOWSRC: u32 = 2015u32;
pub const DISPID_IHTMLINPUTIMAGE_NAME: u32 = 65536u32;
pub const DISPID_IHTMLINPUTIMAGE_ONABORT: u32 = 71564u32;
pub const DISPID_IHTMLINPUTIMAGE_ONERROR: u32 = 71565u32;
pub const DISPID_IHTMLINPUTIMAGE_ONLOAD: u32 = 71568u32;
pub const DISPID_IHTMLINPUTIMAGE_READYSTATE: u32 = 70652u32;
pub const DISPID_IHTMLINPUTIMAGE_SRC: u32 = 2011u32;
pub const DISPID_IHTMLINPUTIMAGE_START: u32 = 2020u32;
pub const DISPID_IHTMLINPUTIMAGE_TYPE: u32 = 2000u32;
pub const DISPID_IHTMLINPUTIMAGE_VRML: u32 = 2016u32;
pub const DISPID_IHTMLINPUTIMAGE_VSPACE: u32 = 2013u32;
pub const DISPID_IHTMLINPUTIMAGE_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLINPUTRANGEELEMENT_ALT: u32 = 2010u32;
pub const DISPID_IHTMLINPUTRANGEELEMENT_DISABLED: u32 = 65612u32;
pub const DISPID_IHTMLINPUTRANGEELEMENT_MAX: u32 = 2029u32;
pub const DISPID_IHTMLINPUTRANGEELEMENT_MIN: u32 = 2028u32;
pub const DISPID_IHTMLINPUTRANGEELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLINPUTRANGEELEMENT_STEP: u32 = 2030u32;
pub const DISPID_IHTMLINPUTRANGEELEMENT_STEPDOWN: u32 = 2032u32;
pub const DISPID_IHTMLINPUTRANGEELEMENT_STEPUP: u32 = 2033u32;
pub const DISPID_IHTMLINPUTRANGEELEMENT_TYPE: u32 = 2000u32;
pub const DISPID_IHTMLINPUTRANGEELEMENT_VALUE: u32 = 70637u32;
pub const DISPID_IHTMLINPUTRANGEELEMENT_VALUEASNUMBER: u32 = 2031u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT2_SELECTIONEND: u32 = 2026u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT2_SELECTIONSTART: u32 = 2025u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT2_SETSELECTIONRANGE: u32 = 2027u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT_CREATETEXTRANGE: u32 = 2006u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT_DEFAULTVALUE: u32 = 70619u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT_DISABLED: u32 = 65612u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT_FORM: u32 = 67540u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT_MAXLENGTH: u32 = 2003u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT_ONCHANGE: u32 = 71566u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT_ONSELECT: u32 = 71546u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT_READONLY: u32 = 2005u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT_SELECT: u32 = 2004u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT_SIZE: u32 = 2002u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT_STATUS: u32 = 2021u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT_TYPE: u32 = 2000u32;
pub const DISPID_IHTMLINPUTTEXTELEMENT_VALUE: u32 = 70637u32;
pub const DISPID_IHTMLIPRINTCOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLIPRINTCOLLECTION_LENGTH: u32 = 1501u32;
pub const DISPID_IHTMLIPRINTCOLLECTION__NEWENUM: i32 = -4i32;
pub const DISPID_IHTMLISINDEXELEMENT2_FORM: u32 = 1012u32;
pub const DISPID_IHTMLISINDEXELEMENT_ACTION: u32 = 1011u32;
pub const DISPID_IHTMLISINDEXELEMENT_PROMPT: u32 = 1010u32;
pub const DISPID_IHTMLLABELELEMENT2_FORM: u32 = 1002u32;
pub const DISPID_IHTMLLABELELEMENT_ACCESSKEY: u32 = 67541u32;
pub const DISPID_IHTMLLABELELEMENT_HTMLFOR: u32 = 1000u32;
pub const DISPID_IHTMLLEGENDELEMENT2_FORM: u32 = 67540u32;
pub const DISPID_IHTMLLEGENDELEMENT_ALIGN: u32 = 65609u32;
pub const DISPID_IHTMLLIELEMENT_TYPE: u32 = 70553u32;
pub const DISPID_IHTMLLIELEMENT_VALUE: u32 = 1001u32;
pub const DISPID_IHTMLLINKELEMENT2_TARGET: u32 = 1017u32;
pub const DISPID_IHTMLLINKELEMENT3_CHARSET: u32 = 1018u32;
pub const DISPID_IHTMLLINKELEMENT3_HREFLANG: u32 = 1019u32;
pub const DISPID_IHTMLLINKELEMENT4_IE8_HREF: u32 = 1150u32;
pub const DISPID_IHTMLLINKELEMENT5_SHEET: u32 = 1020u32;
pub const DISPID_IHTMLLINKELEMENT_DISABLED: u32 = 65612u32;
pub const DISPID_IHTMLLINKELEMENT_HREF: u32 = 1005u32;
pub const DISPID_IHTMLLINKELEMENT_MEDIA: u32 = 1016u32;
pub const DISPID_IHTMLLINKELEMENT_ONERROR: u32 = 71565u32;
pub const DISPID_IHTMLLINKELEMENT_ONLOAD: u32 = 71568u32;
pub const DISPID_IHTMLLINKELEMENT_ONREADYSTATECHANGE: u32 = 71561u32;
pub const DISPID_IHTMLLINKELEMENT_READYSTATE: u32 = 70652u32;
pub const DISPID_IHTMLLINKELEMENT_REL: u32 = 1006u32;
pub const DISPID_IHTMLLINKELEMENT_REV: u32 = 1007u32;
pub const DISPID_IHTMLLINKELEMENT_STYLESHEET: u32 = 1014u32;
pub const DISPID_IHTMLLINKELEMENT_TYPE: u32 = 1008u32;
pub const DISPID_IHTMLLISTELEMENT2_COMPACT: u32 = 1001u32;
pub const DISPID_IHTMLLOCATION_ASSIGN: u32 = 10u32;
pub const DISPID_IHTMLLOCATION_HASH: u32 = 7u32;
pub const DISPID_IHTMLLOCATION_HOST: u32 = 2u32;
pub const DISPID_IHTMLLOCATION_HOSTNAME: u32 = 3u32;
pub const DISPID_IHTMLLOCATION_HREF: u32 = 0u32;
pub const DISPID_IHTMLLOCATION_PATHNAME: u32 = 5u32;
pub const DISPID_IHTMLLOCATION_PORT: u32 = 4u32;
pub const DISPID_IHTMLLOCATION_PROTOCOL: u32 = 1u32;
pub const DISPID_IHTMLLOCATION_RELOAD: u32 = 8u32;
pub const DISPID_IHTMLLOCATION_REPLACE: u32 = 9u32;
pub const DISPID_IHTMLLOCATION_SEARCH: u32 = 6u32;
pub const DISPID_IHTMLLOCATION_TOSTRING: u32 = 11u32;
pub const DISPID_IHTMLMAPELEMENT_AREAS: u32 = 1002u32;
pub const DISPID_IHTMLMAPELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLMARQUEEELEMENT_BEHAVIOR: u32 = 6002u32;
pub const DISPID_IHTMLMARQUEEELEMENT_BGCOLOR: i32 = -501i32;
pub const DISPID_IHTMLMARQUEEELEMENT_DIRECTION: u32 = 6001u32;
pub const DISPID_IHTMLMARQUEEELEMENT_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLMARQUEEELEMENT_HSPACE: u32 = 6006u32;
pub const DISPID_IHTMLMARQUEEELEMENT_LOOP: u32 = 6004u32;
pub const DISPID_IHTMLMARQUEEELEMENT_ONBOUNCE: u32 = 71556u32;
pub const DISPID_IHTMLMARQUEEELEMENT_ONFINISH: u32 = 71562u32;
pub const DISPID_IHTMLMARQUEEELEMENT_ONSTART: u32 = 71563u32;
pub const DISPID_IHTMLMARQUEEELEMENT_SCROLLAMOUNT: u32 = 6003u32;
pub const DISPID_IHTMLMARQUEEELEMENT_SCROLLDELAY: u32 = 6000u32;
pub const DISPID_IHTMLMARQUEEELEMENT_START: u32 = 6010u32;
pub const DISPID_IHTMLMARQUEEELEMENT_STOP: u32 = 6011u32;
pub const DISPID_IHTMLMARQUEEELEMENT_TRUESPEED: u32 = 6007u32;
pub const DISPID_IHTMLMARQUEEELEMENT_VSPACE: u32 = 6005u32;
pub const DISPID_IHTMLMARQUEEELEMENT_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLMEDIAELEMENT2_CURRENTTIMEDOUBLE: u32 = 1027u32;
pub const DISPID_IHTMLMEDIAELEMENT2_DEFAULTPLAYBACKRATEDOUBLE: u32 = 1030u32;
pub const DISPID_IHTMLMEDIAELEMENT2_DURATIONDOUBLE: u32 = 1029u32;
pub const DISPID_IHTMLMEDIAELEMENT2_INITIALTIMEDOUBLE: u32 = 1028u32;
pub const DISPID_IHTMLMEDIAELEMENT2_PLAYBACKRATEDOUBLE: u32 = 1031u32;
pub const DISPID_IHTMLMEDIAELEMENT2_VOLUMEDOUBLE: u32 = 1032u32;
pub const DISPID_IHTMLMEDIAELEMENT_AUTOBUFFER: u32 = 1026u32;
pub const DISPID_IHTMLMEDIAELEMENT_AUTOPLAY: u32 = 1019u32;
pub const DISPID_IHTMLMEDIAELEMENT_BUFFERED: u32 = 1005u32;
pub const DISPID_IHTMLMEDIAELEMENT_CANPLAYTYPE: u32 = 1007u32;
pub const DISPID_IHTMLMEDIAELEMENT_CONTROLS: u32 = 1023u32;
pub const DISPID_IHTMLMEDIAELEMENT_CURRENTSRC: u32 = 1002u32;
pub const DISPID_IHTMLMEDIAELEMENT_CURRENTTIME: u32 = 1010u32;
pub const DISPID_IHTMLMEDIAELEMENT_DEFAULTPLAYBACKRATE: u32 = 1014u32;
pub const DISPID_IHTMLMEDIAELEMENT_DURATION: u32 = 1012u32;
pub const DISPID_IHTMLMEDIAELEMENT_ENDED: u32 = 1018u32;
pub const DISPID_IHTMLMEDIAELEMENT_ERROR: u32 = 1000u32;
pub const DISPID_IHTMLMEDIAELEMENT_INITIALTIME: u32 = 1011u32;
pub const DISPID_IHTMLMEDIAELEMENT_LOAD: u32 = 1006u32;
pub const DISPID_IHTMLMEDIAELEMENT_LOOP: u32 = 1020u32;
pub const DISPID_IHTMLMEDIAELEMENT_MUTED: u32 = 1025u32;
pub const DISPID_IHTMLMEDIAELEMENT_NETWORKSTATE: u32 = 1003u32;
pub const DISPID_IHTMLMEDIAELEMENT_PAUSE: u32 = 1022u32;
pub const DISPID_IHTMLMEDIAELEMENT_PAUSED: u32 = 1013u32;
pub const DISPID_IHTMLMEDIAELEMENT_PLAY: u32 = 1021u32;
pub const DISPID_IHTMLMEDIAELEMENT_PLAYBACKRATE: u32 = 1015u32;
pub const DISPID_IHTMLMEDIAELEMENT_PLAYED: u32 = 1016u32;
pub const DISPID_IHTMLMEDIAELEMENT_PRELOAD: u32 = 1004u32;
pub const DISPID_IHTMLMEDIAELEMENT_SEEKABLE: u32 = 1017u32;
pub const DISPID_IHTMLMEDIAELEMENT_SEEKING: u32 = 1009u32;
pub const DISPID_IHTMLMEDIAELEMENT_SRC: u32 = 1001u32;
pub const DISPID_IHTMLMEDIAELEMENT_VOLUME: u32 = 1024u32;
pub const DISPID_IHTMLMEDIAERROR_CODE: u32 = 1000u32;
pub const DISPID_IHTMLMETAELEMENT2_SCHEME: u32 = 1020u32;
pub const DISPID_IHTMLMETAELEMENT3_IE8_URL: u32 = 1150u32;
pub const DISPID_IHTMLMETAELEMENT_CHARSET: u32 = 1013u32;
pub const DISPID_IHTMLMETAELEMENT_CONTENT: u32 = 1002u32;
pub const DISPID_IHTMLMETAELEMENT_HTTPEQUIV: u32 = 1001u32;
pub const DISPID_IHTMLMETAELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLMETAELEMENT_URL: u32 = 1003u32;
pub const DISPID_IHTMLMIMETYPESCOLLECTION_LENGTH: u32 = 1u32;
pub const DISPID_IHTMLMODELESSINIT_DOCUMENT: u32 = 25007u32;
pub const DISPID_IHTMLMODELESSINIT_MONIKER: u32 = 25006u32;
pub const DISPID_IHTMLMODELESSINIT_OPTIONSTRING: u32 = 25001u32;
pub const DISPID_IHTMLMODELESSINIT_PARAMETERS: u32 = 25000u32;
pub const DISPID_IHTMLMSCSSKEYFRAMERULE_KEYTEXT: u32 = 1001u32;
pub const DISPID_IHTMLMSCSSKEYFRAMERULE_STYLE: u32 = 1002u32;
pub const DISPID_IHTMLMSCSSKEYFRAMESRULE_APPENDRULE: u32 = 1003u32;
pub const DISPID_IHTMLMSCSSKEYFRAMESRULE_CSSRULES: u32 = 1002u32;
pub const DISPID_IHTMLMSCSSKEYFRAMESRULE_DELETERULE: u32 = 1004u32;
pub const DISPID_IHTMLMSCSSKEYFRAMESRULE_FINDRULE: u32 = 1005u32;
pub const DISPID_IHTMLMSCSSKEYFRAMESRULE_NAME: u32 = 1001u32;
pub const DISPID_IHTMLMSIMGELEMENT_MSPLAYTODISABLED: u32 = 1157u32;
pub const DISPID_IHTMLMSIMGELEMENT_MSPLAYTOPRIMARY: u32 = 1158u32;
pub const DISPID_IHTMLMSMEDIAELEMENT_MSPLAYTODISABLED: u32 = 1033u32;
pub const DISPID_IHTMLMSMEDIAELEMENT_MSPLAYTOPRIMARY: u32 = 1034u32;
pub const DISPID_IHTMLNAMESPACECOLLECTION_ADD: u32 = 1001u32;
pub const DISPID_IHTMLNAMESPACECOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLNAMESPACECOLLECTION_LENGTH: u32 = 1000u32;
pub const DISPID_IHTMLNAMESPACE_ATTACHEVENT: u32 = 66043u32;
pub const DISPID_IHTMLNAMESPACE_DETACHEVENT: u32 = 66044u32;
pub const DISPID_IHTMLNAMESPACE_DOIMPORT: u32 = 1003u32;
pub const DISPID_IHTMLNAMESPACE_NAME: u32 = 1000u32;
pub const DISPID_IHTMLNAMESPACE_ONREADYSTATECHANGE: u32 = 71561u32;
pub const DISPID_IHTMLNAMESPACE_READYSTATE: u32 = 70652u32;
pub const DISPID_IHTMLNAMESPACE_TAGNAMES: u32 = 1002u32;
pub const DISPID_IHTMLNAMESPACE_URN: u32 = 1001u32;
pub const DISPID_IHTMLNEXTIDELEMENT_N: u32 = 1012u32;
pub const DISPID_IHTMLOBJECTELEMENT2_CLASSID: u32 = 68538u32;
pub const DISPID_IHTMLOBJECTELEMENT2_DATA: u32 = 68539u32;
pub const DISPID_IHTMLOBJECTELEMENT2_NAMEDRECORDSET: u32 = 68550u32;
pub const DISPID_IHTMLOBJECTELEMENT3_ALT: u32 = 68552u32;
pub const DISPID_IHTMLOBJECTELEMENT3_ARCHIVE: u32 = 68551u32;
pub const DISPID_IHTMLOBJECTELEMENT3_BORDER: u32 = 68555u32;
pub const DISPID_IHTMLOBJECTELEMENT3_DECLARE: u32 = 68553u32;
pub const DISPID_IHTMLOBJECTELEMENT3_STANDBY: u32 = 68554u32;
pub const DISPID_IHTMLOBJECTELEMENT3_USEMAP: u32 = 68556u32;
pub const DISPID_IHTMLOBJECTELEMENT4_CONTENTDOCUMENT: u32 = 68566u32;
pub const DISPID_IHTMLOBJECTELEMENT4_IE8_CODEBASE: u32 = 68567u32;
pub const DISPID_IHTMLOBJECTELEMENT4_IE8_DATA: u32 = 68568u32;
pub const DISPID_IHTMLOBJECTELEMENT5_IE9_OBJECT: u32 = 68569u32;
pub const DISPID_IHTMLOBJECTELEMENT_ALIGN: u32 = 65609u32;
pub const DISPID_IHTMLOBJECTELEMENT_ALTHTML: u32 = 68547u32;
pub const DISPID_IHTMLOBJECTELEMENT_BASEHREF: u32 = 65538u32;
pub const DISPID_IHTMLOBJECTELEMENT_CLASSID: u32 = 68538u32;
pub const DISPID_IHTMLOBJECTELEMENT_CODE: u32 = 68544u32;
pub const DISPID_IHTMLOBJECTELEMENT_CODEBASE: u32 = 68542u32;
pub const DISPID_IHTMLOBJECTELEMENT_CODETYPE: u32 = 68543u32;
pub const DISPID_IHTMLOBJECTELEMENT_DATA: u32 = 68539u32;
pub const DISPID_IHTMLOBJECTELEMENT_FORM: u32 = 67540u32;
pub const DISPID_IHTMLOBJECTELEMENT_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLOBJECTELEMENT_HSPACE: u32 = 68549u32;
pub const DISPID_IHTMLOBJECTELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLOBJECTELEMENT_OBJECT: u32 = 68537u32;
pub const DISPID_IHTMLOBJECTELEMENT_ONERROR: u32 = 71565u32;
pub const DISPID_IHTMLOBJECTELEMENT_ONREADYSTATECHANGE: u32 = 71561u32;
pub const DISPID_IHTMLOBJECTELEMENT_READYSTATE: u32 = 68546u32;
pub const DISPID_IHTMLOBJECTELEMENT_RECORDSET: u32 = 68541u32;
pub const DISPID_IHTMLOBJECTELEMENT_TYPE: u32 = 68545u32;
pub const DISPID_IHTMLOBJECTELEMENT_VSPACE: u32 = 68548u32;
pub const DISPID_IHTMLOBJECTELEMENT_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLOLISTELEMENT_COMPACT: u32 = 1001u32;
pub const DISPID_IHTMLOLISTELEMENT_START: u32 = 1003u32;
pub const DISPID_IHTMLOLISTELEMENT_TYPE: u32 = 70553u32;
pub const DISPID_IHTMLOPSPROFILE_ADDREADREQUEST: u32 = 7u32;
pub const DISPID_IHTMLOPSPROFILE_ADDREQUEST: u32 = 1u32;
pub const DISPID_IHTMLOPSPROFILE_CLEARREQUEST: u32 = 2u32;
pub const DISPID_IHTMLOPSPROFILE_COMMITCHANGES: u32 = 6u32;
pub const DISPID_IHTMLOPSPROFILE_DOREADREQUEST: u32 = 8u32;
pub const DISPID_IHTMLOPSPROFILE_DOREQUEST: u32 = 3u32;
pub const DISPID_IHTMLOPSPROFILE_DOWRITEREQUEST: u32 = 9u32;
pub const DISPID_IHTMLOPSPROFILE_GETATTRIBUTE: u32 = 4u32;
pub const DISPID_IHTMLOPSPROFILE_SETATTRIBUTE: u32 = 5u32;
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_CHECKED: u32 = 2009u32;
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_DEFAULTCHECKED: u32 = 2008u32;
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_DISABLED: u32 = 65612u32;
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_FORM: u32 = 67540u32;
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_INDETERMINATE: u32 = 2007u32;
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_ONCHANGE: u32 = 71566u32;
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_STATUS: u32 = 2001u32;
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_TYPE: u32 = 2000u32;
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_VALUE: u32 = 70637u32;
pub const DISPID_IHTMLOPTIONELEMENT3_LABEL: u32 = 1007u32;
pub const DISPID_IHTMLOPTIONELEMENT4_IE9_VALUE: u32 = 1008u32;
pub const DISPID_IHTMLOPTIONELEMENTFACTORY_CREATE: u32 = 0u32;
pub const DISPID_IHTMLOPTIONELEMENT_DEFAULTSELECTED: u32 = 1003u32;
pub const DISPID_IHTMLOPTIONELEMENT_FORM: u32 = 1006u32;
pub const DISPID_IHTMLOPTIONELEMENT_INDEX: u32 = 1005u32;
pub const DISPID_IHTMLOPTIONELEMENT_SELECTED: u32 = 1001u32;
pub const DISPID_IHTMLOPTIONELEMENT_TEXT: u32 = 1004u32;
pub const DISPID_IHTMLOPTIONELEMENT_VALUE: u32 = 1002u32;
pub const DISPID_IHTMLOPTIONSHOLDER_ANYTHINGAFTERFRAMESET: u32 = 1513u32;
pub const DISPID_IHTMLOPTIONSHOLDER_CHOOSECOLORDLG: u32 = 1517u32;
pub const DISPID_IHTMLOPTIONSHOLDER_DOCUMENT: u32 = 1503u32;
pub const DISPID_IHTMLOPTIONSHOLDER_ERRORCHARACTER: u32 = 1507u32;
pub const DISPID_IHTMLOPTIONSHOLDER_ERRORCODE: u32 = 1508u32;
pub const DISPID_IHTMLOPTIONSHOLDER_ERRORDEBUG: u32 = 1510u32;
pub const DISPID_IHTMLOPTIONSHOLDER_ERRORLINE: u32 = 1506u32;
pub const DISPID_IHTMLOPTIONSHOLDER_ERRORMESSAGE: u32 = 1509u32;
pub const DISPID_IHTMLOPTIONSHOLDER_EXECARG: u32 = 1505u32;
pub const DISPID_IHTMLOPTIONSHOLDER_FINDTEXT: u32 = 1512u32;
pub const DISPID_IHTMLOPTIONSHOLDER_FONTS: u32 = 1504u32;
pub const DISPID_IHTMLOPTIONSHOLDER_GETCHARSET: u32 = 1520u32;
pub const DISPID_IHTMLOPTIONSHOLDER_ISAPARTMENTMODEL: u32 = 1519u32;
pub const DISPID_IHTMLOPTIONSHOLDER_OPENFILEDLG: u32 = 1515u32;
pub const DISPID_IHTMLOPTIONSHOLDER_SAVEFILEDLG: u32 = 1516u32;
pub const DISPID_IHTMLOPTIONSHOLDER_SECURECONNECTIONINFO: u32 = 1521u32;
pub const DISPID_IHTMLOPTIONSHOLDER_SHOWSECURITYINFO: u32 = 1518u32;
pub const DISPID_IHTMLOPTIONSHOLDER_SIZES: u32 = 1514u32;
pub const DISPID_IHTMLOPTIONSHOLDER_UNSECUREDWINDOWOFDOCUMENT: u32 = 1511u32;
pub const DISPID_IHTMLPARAELEMENT_ALIGN: u32 = 65608u32;
pub const DISPID_IHTMLPARAMELEMENT2_IE8_VALUETYPE: u32 = 1150u32;
pub const DISPID_IHTMLPARAMELEMENT2_NAME: u32 = 1001u32;
pub const DISPID_IHTMLPARAMELEMENT2_TYPE: u32 = 1003u32;
pub const DISPID_IHTMLPARAMELEMENT2_VALUE: u32 = 1002u32;
pub const DISPID_IHTMLPARAMELEMENT_NAME: u32 = 1001u32;
pub const DISPID_IHTMLPARAMELEMENT_TYPE: u32 = 1003u32;
pub const DISPID_IHTMLPARAMELEMENT_VALUE: u32 = 1002u32;
pub const DISPID_IHTMLPARAMELEMENT_VALUETYPE: u32 = 1004u32;
pub const DISPID_IHTMLPERFORMANCENAVIGATION_REDIRECTCOUNT: u32 = 1001u32;
pub const DISPID_IHTMLPERFORMANCENAVIGATION_TOJSON: u32 = 1003u32;
pub const DISPID_IHTMLPERFORMANCENAVIGATION_TOSTRING: u32 = 1002u32;
pub const DISPID_IHTMLPERFORMANCENAVIGATION_TYPE: u32 = 1000u32;
pub const DISPID_IHTMLPERFORMANCETIMING_CONNECTEND: u32 = 1009u32;
pub const DISPID_IHTMLPERFORMANCETIMING_CONNECTSTART: u32 = 1008u32;
pub const DISPID_IHTMLPERFORMANCETIMING_DOMAINLOOKUPEND: u32 = 1007u32;
pub const DISPID_IHTMLPERFORMANCETIMING_DOMAINLOOKUPSTART: u32 = 1006u32;
pub const DISPID_IHTMLPERFORMANCETIMING_DOMCOMPLETE: u32 = 1017u32;
pub const DISPID_IHTMLPERFORMANCETIMING_DOMCONTENTLOADEDEVENTEND: u32 = 1016u32;
pub const DISPID_IHTMLPERFORMANCETIMING_DOMCONTENTLOADEDEVENTSTART: u32 = 1015u32;
pub const DISPID_IHTMLPERFORMANCETIMING_DOMINTERACTIVE: u32 = 1014u32;
pub const DISPID_IHTMLPERFORMANCETIMING_DOMLOADING: u32 = 1013u32;
pub const DISPID_IHTMLPERFORMANCETIMING_FETCHSTART: u32 = 1005u32;
pub const DISPID_IHTMLPERFORMANCETIMING_LOADEVENTEND: u32 = 1019u32;
pub const DISPID_IHTMLPERFORMANCETIMING_LOADEVENTSTART: u32 = 1018u32;
pub const DISPID_IHTMLPERFORMANCETIMING_MSFIRSTPAINT: u32 = 1020u32;
pub const DISPID_IHTMLPERFORMANCETIMING_NAVIGATIONSTART: u32 = 1000u32;
pub const DISPID_IHTMLPERFORMANCETIMING_REDIRECTEND: u32 = 1004u32;
pub const DISPID_IHTMLPERFORMANCETIMING_REDIRECTSTART: u32 = 1003u32;
pub const DISPID_IHTMLPERFORMANCETIMING_REQUESTSTART: u32 = 1010u32;
pub const DISPID_IHTMLPERFORMANCETIMING_RESPONSEEND: u32 = 1012u32;
pub const DISPID_IHTMLPERFORMANCETIMING_RESPONSESTART: u32 = 1011u32;
pub const DISPID_IHTMLPERFORMANCETIMING_TOJSON: u32 = 1022u32;
pub const DISPID_IHTMLPERFORMANCETIMING_TOSTRING: u32 = 1021u32;
pub const DISPID_IHTMLPERFORMANCETIMING_UNLOADEVENTEND: u32 = 1002u32;
pub const DISPID_IHTMLPERFORMANCETIMING_UNLOADEVENTSTART: u32 = 1001u32;
pub const DISPID_IHTMLPERFORMANCE_NAVIGATION: u32 = 1000u32;
pub const DISPID_IHTMLPERFORMANCE_TIMING: u32 = 1001u32;
pub const DISPID_IHTMLPERFORMANCE_TOJSON: u32 = 1003u32;
pub const DISPID_IHTMLPERFORMANCE_TOSTRING: u32 = 1002u32;
pub const DISPID_IHTMLPHRASEELEMENT2_CITE: u32 = 1001u32;
pub const DISPID_IHTMLPHRASEELEMENT2_DATETIME: u32 = 1002u32;
pub const DISPID_IHTMLPHRASEELEMENT3_IE8_CITE: u32 = 1150u32;
pub const DISPID_IHTMLPLUGINSCOLLECTION_LENGTH: u32 = 1u32;
pub const DISPID_IHTMLPLUGINSCOLLECTION_REFRESH: u32 = 2u32;
pub const DISPID_IHTMLPOPUP_DOCUMENT: u32 = 27003u32;
pub const DISPID_IHTMLPOPUP_HIDE: u32 = 27002u32;
pub const DISPID_IHTMLPOPUP_ISOPEN: u32 = 27004u32;
pub const DISPID_IHTMLPOPUP_SHOW: u32 = 27001u32;
pub const DISPID_IHTMLPROGRESSELEMENT_FORM: u32 = 67540u32;
pub const DISPID_IHTMLPROGRESSELEMENT_MAX: u32 = 1000u32;
pub const DISPID_IHTMLPROGRESSELEMENT_POSITION: u32 = 1001u32;
pub const DISPID_IHTMLPROGRESSELEMENT_VALUE: u32 = 0u32;
pub const DISPID_IHTMLRECT2_HEIGHT: u32 = 1006u32;
pub const DISPID_IHTMLRECT2_WIDTH: u32 = 1005u32;
pub const DISPID_IHTMLRECTCOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLRECTCOLLECTION_LENGTH: u32 = 1500u32;
pub const DISPID_IHTMLRECTCOLLECTION__NEWENUM: i32 = -4i32;
pub const DISPID_IHTMLRECT_BOTTOM: u32 = 1004u32;
pub const DISPID_IHTMLRECT_LEFT: u32 = 1001u32;
pub const DISPID_IHTMLRECT_RIGHT: u32 = 1003u32;
pub const DISPID_IHTMLRECT_TOP: u32 = 1002u32;
pub const DISPID_IHTMLRENDERSTYLE_DEFAULTTEXTSELECTION: u32 = 70724u32;
pub const DISPID_IHTMLRENDERSTYLE_RENDERINGPRIORITY: u32 = 70706u32;
pub const DISPID_IHTMLRENDERSTYLE_TEXTBACKGROUNDCOLOR: u32 = 70705u32;
pub const DISPID_IHTMLRENDERSTYLE_TEXTCOLOR: u32 = 70726u32;
pub const DISPID_IHTMLRENDERSTYLE_TEXTDECORATION: u32 = 70727u32;
pub const DISPID_IHTMLRENDERSTYLE_TEXTDECORATIONCOLOR: u32 = 70725u32;
pub const DISPID_IHTMLRENDERSTYLE_TEXTEFFECT: u32 = 70704u32;
pub const DISPID_IHTMLRENDERSTYLE_TEXTLINETHROUGHSTYLE: u32 = 70702u32;
pub const DISPID_IHTMLRENDERSTYLE_TEXTUNDERLINESTYLE: u32 = 70703u32;
pub const DISPID_IHTMLRULESTYLE2_ACCELERATOR: u32 = 70683u32;
pub const DISPID_IHTMLRULESTYLE2_BEHAVIOR: u32 = 70651u32;
pub const DISPID_IHTMLRULESTYLE2_BORDERCOLLAPSE: u32 = 70620u32;
pub const DISPID_IHTMLRULESTYLE2_BOTTOM: u32 = 65614u32;
pub const DISPID_IHTMLRULESTYLE2_DIRECTION: u32 = 70655u32;
pub const DISPID_IHTMLRULESTYLE2_IMEMODE: u32 = 70656u32;
pub const DISPID_IHTMLRULESTYLE2_LAYOUTGRID: u32 = 70667u32;
pub const DISPID_IHTMLRULESTYLE2_LAYOUTGRIDCHAR: u32 = 70663u32;
pub const DISPID_IHTMLRULESTYLE2_LAYOUTGRIDLINE: u32 = 70664u32;
pub const DISPID_IHTMLRULESTYLE2_LAYOUTGRIDMODE: u32 = 70665u32;
pub const DISPID_IHTMLRULESTYLE2_LAYOUTGRIDTYPE: u32 = 70666u32;
pub const DISPID_IHTMLRULESTYLE2_LINEBREAK: u32 = 70669u32;
pub const DISPID_IHTMLRULESTYLE2_OVERFLOWX: u32 = 70675u32;
pub const DISPID_IHTMLRULESTYLE2_OVERFLOWY: u32 = 70676u32;
pub const DISPID_IHTMLRULESTYLE2_PIXELBOTTOM: u32 = 69545u32;
pub const DISPID_IHTMLRULESTYLE2_PIXELRIGHT: u32 = 69546u32;
pub const DISPID_IHTMLRULESTYLE2_POSBOTTOM: u32 = 69547u32;
pub const DISPID_IHTMLRULESTYLE2_POSITION: u32 = 70626u32;
pub const DISPID_IHTMLRULESTYLE2_POSRIGHT: u32 = 69548u32;
pub const DISPID_IHTMLRULESTYLE2_RIGHT: u32 = 65613u32;
pub const DISPID_IHTMLRULESTYLE2_RUBYALIGN: u32 = 70657u32;
pub const DISPID_IHTMLRULESTYLE2_RUBYOVERHANG: u32 = 70659u32;
pub const DISPID_IHTMLRULESTYLE2_RUBYPOSITION: u32 = 70658u32;
pub const DISPID_IHTMLRULESTYLE2_TABLELAYOUT: u32 = 70634u32;
pub const DISPID_IHTMLRULESTYLE2_TEXTAUTOSPACE: u32 = 70668u32;
pub const DISPID_IHTMLRULESTYLE2_TEXTJUSTIFY: u32 = 70671u32;
pub const DISPID_IHTMLRULESTYLE2_TEXTJUSTIFYTRIM: u32 = 70672u32;
pub const DISPID_IHTMLRULESTYLE2_TEXTKASHIDA: u32 = 70673u32;
pub const DISPID_IHTMLRULESTYLE2_UNICODEBIDI: u32 = 70654u32;
pub const DISPID_IHTMLRULESTYLE2_WORDBREAK: u32 = 70670u32;
pub const DISPID_IHTMLRULESTYLE3_LAYOUTFLOW: u32 = 70691u32;
pub const DISPID_IHTMLRULESTYLE3_SCROLLBAR3DLIGHTCOLOR: u32 = 70718u32;
pub const DISPID_IHTMLRULESTYLE3_SCROLLBARARROWCOLOR: u32 = 70722u32;
pub const DISPID_IHTMLRULESTYLE3_SCROLLBARBASECOLOR: u32 = 70716u32;
pub const DISPID_IHTMLRULESTYLE3_SCROLLBARDARKSHADOWCOLOR: u32 = 70721u32;
pub const DISPID_IHTMLRULESTYLE3_SCROLLBARFACECOLOR: u32 = 70717u32;
pub const DISPID_IHTMLRULESTYLE3_SCROLLBARHIGHLIGHTCOLOR: u32 = 70720u32;
pub const DISPID_IHTMLRULESTYLE3_SCROLLBARSHADOWCOLOR: u32 = 70719u32;
pub const DISPID_IHTMLRULESTYLE3_SCROLLBARTRACKCOLOR: u32 = 70732u32;
pub const DISPID_IHTMLRULESTYLE3_TEXTALIGNLAST: u32 = 70739u32;
pub const DISPID_IHTMLRULESTYLE3_TEXTKASHIDASPACE: u32 = 70740u32;
pub const DISPID_IHTMLRULESTYLE3_TEXTUNDERLINEPOSITION: u32 = 70695u32;
pub const DISPID_IHTMLRULESTYLE3_WORDWRAP: u32 = 70694u32;
pub const DISPID_IHTMLRULESTYLE3_WRITINGMODE: u32 = 70728u32;
pub const DISPID_IHTMLRULESTYLE3_ZOOM: u32 = 70689u32;
pub const DISPID_IHTMLRULESTYLE4_MINHEIGHT: u32 = 70747u32;
pub const DISPID_IHTMLRULESTYLE4_TEXTOVERFLOW: u32 = 70745u32;
pub const DISPID_IHTMLRULESTYLE5_MAXHEIGHT: u32 = 70750u32;
pub const DISPID_IHTMLRULESTYLE5_MAXWIDTH: u32 = 70752u32;
pub const DISPID_IHTMLRULESTYLE5_MINWIDTH: u32 = 70751u32;
pub const DISPID_IHTMLRULESTYLE5_MSINTERPOLATIONMODE: u32 = 70749u32;
pub const DISPID_IHTMLRULESTYLE6_BORDERSPACING: u32 = 70763u32;
pub const DISPID_IHTMLRULESTYLE6_BOXSIZING: u32 = 70762u32;
pub const DISPID_IHTMLRULESTYLE6_CAPTIONSIDE: u32 = 70755u32;
pub const DISPID_IHTMLRULESTYLE6_CONTENT: u32 = 70754u32;
pub const DISPID_IHTMLRULESTYLE6_COUNTERINCREMENT: u32 = 70756u32;
pub const DISPID_IHTMLRULESTYLE6_COUNTERRESET: u32 = 70757u32;
pub const DISPID_IHTMLRULESTYLE6_EMPTYCELLS: u32 = 70786u32;
pub const DISPID_IHTMLRULESTYLE6_MSBLOCKPROGRESSION: u32 = 70787u32;
pub const DISPID_IHTMLRULESTYLE6_ORPHANS: u32 = 70764u32;
pub const DISPID_IHTMLRULESTYLE6_OUTLINE: u32 = 70758u32;
pub const DISPID_IHTMLRULESTYLE6_OUTLINECOLOR: u32 = 70761u32;
pub const DISPID_IHTMLRULESTYLE6_OUTLINESTYLE: u32 = 70760u32;
pub const DISPID_IHTMLRULESTYLE6_OUTLINEWIDTH: u32 = 70759u32;
pub const DISPID_IHTMLRULESTYLE6_PAGEBREAKINSIDE: u32 = 70766u32;
pub const DISPID_IHTMLRULESTYLE6_QUOTES: u32 = 70788u32;
pub const DISPID_IHTMLRULESTYLE6_WIDOWS: u32 = 70765u32;
pub const DISPID_IHTMLRULESTYLE_BACKGROUND: u32 = 70568u32;
pub const DISPID_IHTMLRULESTYLE_BACKGROUNDATTACHMENT: u32 = 70581u32;
pub const DISPID_IHTMLRULESTYLE_BACKGROUNDCOLOR: i32 = -501i32;
pub const DISPID_IHTMLRULESTYLE_BACKGROUNDIMAGE: u32 = 70537u32;
pub const DISPID_IHTMLRULESTYLE_BACKGROUNDPOSITION: u32 = 70582u32;
pub const DISPID_IHTMLRULESTYLE_BACKGROUNDPOSITIONX: u32 = 70569u32;
pub const DISPID_IHTMLRULESTYLE_BACKGROUNDPOSITIONY: u32 = 70570u32;
pub const DISPID_IHTMLRULESTYLE_BACKGROUNDREPEAT: u32 = 70580u32;
pub const DISPID_IHTMLRULESTYLE_BORDER: u32 = 70585u32;
pub const DISPID_IHTMLRULESTYLE_BORDERBOTTOM: u32 = 70588u32;
pub const DISPID_IHTMLRULESTYLE_BORDERBOTTOMCOLOR: u32 = 70593u32;
pub const DISPID_IHTMLRULESTYLE_BORDERBOTTOMSTYLE: u32 = 70603u32;
pub const DISPID_IHTMLRULESTYLE_BORDERBOTTOMWIDTH: u32 = 70598u32;
pub const DISPID_IHTMLRULESTYLE_BORDERCOLOR: u32 = 70590u32;
pub const DISPID_IHTMLRULESTYLE_BORDERLEFT: u32 = 70589u32;
pub const DISPID_IHTMLRULESTYLE_BORDERLEFTCOLOR: u32 = 70594u32;
pub const DISPID_IHTMLRULESTYLE_BORDERLEFTSTYLE: u32 = 70604u32;
pub const DISPID_IHTMLRULESTYLE_BORDERLEFTWIDTH: u32 = 70599u32;
pub const DISPID_IHTMLRULESTYLE_BORDERRIGHT: u32 = 70587u32;
pub const DISPID_IHTMLRULESTYLE_BORDERRIGHTCOLOR: u32 = 70592u32;
pub const DISPID_IHTMLRULESTYLE_BORDERRIGHTSTYLE: u32 = 70602u32;
pub const DISPID_IHTMLRULESTYLE_BORDERRIGHTWIDTH: u32 = 70597u32;
pub const DISPID_IHTMLRULESTYLE_BORDERSTYLE: u32 = 70600u32;
pub const DISPID_IHTMLRULESTYLE_BORDERTOP: u32 = 70586u32;
pub const DISPID_IHTMLRULESTYLE_BORDERTOPCOLOR: u32 = 70591u32;
pub const DISPID_IHTMLRULESTYLE_BORDERTOPSTYLE: u32 = 70601u32;
pub const DISPID_IHTMLRULESTYLE_BORDERTOPWIDTH: u32 = 70596u32;
pub const DISPID_IHTMLRULESTYLE_BORDERWIDTH: u32 = 70595u32;
pub const DISPID_IHTMLRULESTYLE_CLEAR: u32 = 70552u32;
pub const DISPID_IHTMLRULESTYLE_CLIP: u32 = 70628u32;
pub const DISPID_IHTMLRULESTYLE_COLOR: u32 = 70538u32;
pub const DISPID_IHTMLRULESTYLE_CSSTEXT: u32 = 70635u32;
pub const DISPID_IHTMLRULESTYLE_CURSOR: u32 = 70638u32;
pub const DISPID_IHTMLRULESTYLE_DISPLAY: u32 = 70607u32;
pub const DISPID_IHTMLRULESTYLE_FILTER: u32 = 70618u32;
pub const DISPID_IHTMLRULESTYLE_FONT: u32 = 70577u32;
pub const DISPID_IHTMLRULESTYLE_FONTFAMILY: u32 = 70554u32;
pub const DISPID_IHTMLRULESTYLE_FONTSIZE: u32 = 70555u32;
pub const DISPID_IHTMLRULESTYLE_FONTSTYLE: u32 = 70560u32;
pub const DISPID_IHTMLRULESTYLE_FONTVARIANT: u32 = 70561u32;
pub const DISPID_IHTMLRULESTYLE_FONTWEIGHT: u32 = 70563u32;
pub const DISPID_IHTMLRULESTYLE_GETATTRIBUTE: u32 = 66038u32;
pub const DISPID_IHTMLRULESTYLE_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLRULESTYLE_LEFT: u32 = 65539u32;
pub const DISPID_IHTMLRULESTYLE_LETTERSPACING: u32 = 70544u32;
pub const DISPID_IHTMLRULESTYLE_LINEHEIGHT: u32 = 70542u32;
pub const DISPID_IHTMLRULESTYLE_LISTSTYLE: u32 = 70611u32;
pub const DISPID_IHTMLRULESTYLE_LISTSTYLEIMAGE: u32 = 70610u32;
pub const DISPID_IHTMLRULESTYLE_LISTSTYLEPOSITION: u32 = 70609u32;
pub const DISPID_IHTMLRULESTYLE_LISTSTYLETYPE: u32 = 70608u32;
pub const DISPID_IHTMLRULESTYLE_MARGIN: u32 = 70572u32;
pub const DISPID_IHTMLRULESTYLE_MARGINBOTTOM: u32 = 70575u32;
pub const DISPID_IHTMLRULESTYLE_MARGINLEFT: u32 = 70576u32;
pub const DISPID_IHTMLRULESTYLE_MARGINRIGHT: u32 = 70574u32;
pub const DISPID_IHTMLRULESTYLE_MARGINTOP: u32 = 70573u32;
pub const DISPID_IHTMLRULESTYLE_OVERFLOW: u32 = 70546u32;
pub const DISPID_IHTMLRULESTYLE_PADDING: u32 = 70547u32;
pub const DISPID_IHTMLRULESTYLE_PADDINGBOTTOM: u32 = 70550u32;
pub const DISPID_IHTMLRULESTYLE_PADDINGLEFT: u32 = 70551u32;
pub const DISPID_IHTMLRULESTYLE_PADDINGRIGHT: u32 = 70549u32;
pub const DISPID_IHTMLRULESTYLE_PADDINGTOP: u32 = 70548u32;
pub const DISPID_IHTMLRULESTYLE_PAGEBREAKAFTER: u32 = 70614u32;
pub const DISPID_IHTMLRULESTYLE_PAGEBREAKBEFORE: u32 = 70613u32;
pub const DISPID_IHTMLRULESTYLE_POSITION: u32 = 70626u32;
pub const DISPID_IHTMLRULESTYLE_REMOVEATTRIBUTE: u32 = 66039u32;
pub const DISPID_IHTMLRULESTYLE_SETATTRIBUTE: u32 = 66037u32;
pub const DISPID_IHTMLRULESTYLE_STYLEFLOAT: u32 = 70606u32;
pub const DISPID_IHTMLRULESTYLE_TEXTALIGN: u32 = 65608u32;
pub const DISPID_IHTMLRULESTYLE_TEXTDECORATION: u32 = 70571u32;
pub const DISPID_IHTMLRULESTYLE_TEXTDECORATIONBLINK: u32 = 70558u32;
pub const DISPID_IHTMLRULESTYLE_TEXTDECORATIONLINETHROUGH: u32 = 70556u32;
pub const DISPID_IHTMLRULESTYLE_TEXTDECORATIONNONE: u32 = 70559u32;
pub const DISPID_IHTMLRULESTYLE_TEXTDECORATIONOVERLINE: u32 = 70605u32;
pub const DISPID_IHTMLRULESTYLE_TEXTDECORATIONUNDERLINE: u32 = 70557u32;
pub const DISPID_IHTMLRULESTYLE_TEXTINDENT: u32 = 70543u32;
pub const DISPID_IHTMLRULESTYLE_TEXTTRANSFORM: u32 = 70540u32;
pub const DISPID_IHTMLRULESTYLE_TOP: u32 = 65540u32;
pub const DISPID_IHTMLRULESTYLE_VERTICALALIGN: u32 = 70584u32;
pub const DISPID_IHTMLRULESTYLE_VISIBILITY: u32 = 70616u32;
pub const DISPID_IHTMLRULESTYLE_WHITESPACE: u32 = 70612u32;
pub const DISPID_IHTMLRULESTYLE_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLRULESTYLE_WORDSPACING: u32 = 70583u32;
pub const DISPID_IHTMLRULESTYLE_ZINDEX: u32 = 70627u32;
pub const DISPID_IHTMLSCREEN2_DEVICEXDPI: u32 = 1011u32;
pub const DISPID_IHTMLSCREEN2_DEVICEYDPI: u32 = 1012u32;
pub const DISPID_IHTMLSCREEN2_LOGICALXDPI: u32 = 1009u32;
pub const DISPID_IHTMLSCREEN2_LOGICALYDPI: u32 = 1010u32;
pub const DISPID_IHTMLSCREEN3_SYSTEMXDPI: u32 = 1013u32;
pub const DISPID_IHTMLSCREEN3_SYSTEMYDPI: u32 = 1014u32;
pub const DISPID_IHTMLSCREEN4_PIXELDEPTH: u32 = 1015u32;
pub const DISPID_IHTMLSCREEN_AVAILHEIGHT: u32 = 1006u32;
pub const DISPID_IHTMLSCREEN_AVAILWIDTH: u32 = 1007u32;
pub const DISPID_IHTMLSCREEN_BUFFERDEPTH: u32 = 1002u32;
pub const DISPID_IHTMLSCREEN_COLORDEPTH: u32 = 1001u32;
pub const DISPID_IHTMLSCREEN_FONTSMOOTHINGENABLED: u32 = 1008u32;
pub const DISPID_IHTMLSCREEN_HEIGHT: u32 = 1004u32;
pub const DISPID_IHTMLSCREEN_UPDATEINTERVAL: u32 = 1005u32;
pub const DISPID_IHTMLSCREEN_WIDTH: u32 = 1003u32;
pub const DISPID_IHTMLSCRIPTELEMENT2_CHARSET: u32 = 1010u32;
pub const DISPID_IHTMLSCRIPTELEMENT3_IE8_SRC: u32 = 1150u32;
pub const DISPID_IHTMLSCRIPTELEMENT4_USEDCHARSET: u32 = 1011u32;
pub const DISPID_IHTMLSCRIPTELEMENT_DEFER: u32 = 1007u32;
pub const DISPID_IHTMLSCRIPTELEMENT_EVENT: u32 = 1005u32;
pub const DISPID_IHTMLSCRIPTELEMENT_HTMLFOR: u32 = 1004u32;
pub const DISPID_IHTMLSCRIPTELEMENT_ONERROR: u32 = 71565u32;
pub const DISPID_IHTMLSCRIPTELEMENT_READYSTATE: u32 = 70652u32;
pub const DISPID_IHTMLSCRIPTELEMENT_SRC: u32 = 1001u32;
pub const DISPID_IHTMLSCRIPTELEMENT_TEXT: u32 = 1006u32;
pub const DISPID_IHTMLSCRIPTELEMENT_TYPE: u32 = 1009u32;
pub const DISPID_IHTMLSELECTELEMENT2_URNS: u32 = 1505u32;
pub const DISPID_IHTMLSELECTELEMENT4_NAMEDITEM: u32 = 1506u32;
pub const DISPID_IHTMLSELECTELEMENT5_IE8_ADD: u32 = 1150u32;
pub const DISPID_IHTMLSELECTELEMENT6_IE9_ADD: u32 = 1151u32;
pub const DISPID_IHTMLSELECTELEMENT6_IE9_VALUE: u32 = 1152u32;
pub const DISPID_IHTMLSELECTELEMENT_ADD: u32 = 1503u32;
pub const DISPID_IHTMLSELECTELEMENT_DISABLED: u32 = 65612u32;
pub const DISPID_IHTMLSELECTELEMENT_FORM: u32 = 67540u32;
pub const DISPID_IHTMLSELECTELEMENT_ITEM: u32 = 0u32;
pub const DISPID_IHTMLSELECTELEMENT_LENGTH: u32 = 1500u32;
pub const DISPID_IHTMLSELECTELEMENT_MULTIPLE: u32 = 1003u32;
pub const DISPID_IHTMLSELECTELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLSELECTELEMENT_ONCHANGE: u32 = 71566u32;
pub const DISPID_IHTMLSELECTELEMENT_OPTIONS: u32 = 1005u32;
pub const DISPID_IHTMLSELECTELEMENT_REMOVE: u32 = 1504u32;
pub const DISPID_IHTMLSELECTELEMENT_SELECTEDINDEX: u32 = 1010u32;
pub const DISPID_IHTMLSELECTELEMENT_SIZE: u32 = 1002u32;
pub const DISPID_IHTMLSELECTELEMENT_TAGS: u32 = 1502u32;
pub const DISPID_IHTMLSELECTELEMENT_TYPE: u32 = 1012u32;
pub const DISPID_IHTMLSELECTELEMENT_VALUE: u32 = 1011u32;
pub const DISPID_IHTMLSELECTELEMENT__NEWENUM: i32 = -4i32;
pub const DISPID_IHTMLSELECTIONOBJECT2_CREATERANGECOLLECTION: u32 = 1005u32;
pub const DISPID_IHTMLSELECTIONOBJECT2_TYPEDETAIL: u32 = 1006u32;
pub const DISPID_IHTMLSELECTIONOBJECT_CLEAR: u32 = 1003u32;
pub const DISPID_IHTMLSELECTIONOBJECT_CREATERANGE: u32 = 1001u32;
pub const DISPID_IHTMLSELECTIONOBJECT_EMPTY: u32 = 1002u32;
pub const DISPID_IHTMLSELECTIONOBJECT_TYPE: u32 = 1004u32;
pub const DISPID_IHTMLSELECTION_ADDRANGE: u32 = 1013u32;
pub const DISPID_IHTMLSELECTION_ANCHORNODE: u32 = 1001u32;
pub const DISPID_IHTMLSELECTION_ANCHOROFFSET: u32 = 1002u32;
pub const DISPID_IHTMLSELECTION_COLLAPSE: u32 = 1006u32;
pub const DISPID_IHTMLSELECTION_COLLAPSETOEND: u32 = 1008u32;
pub const DISPID_IHTMLSELECTION_COLLAPSETOSTART: u32 = 1007u32;
pub const DISPID_IHTMLSELECTION_DELETEFROMDOCUMENT: u32 = 1010u32;
pub const DISPID_IHTMLSELECTION_FOCUSNODE: u32 = 1003u32;
pub const DISPID_IHTMLSELECTION_FOCUSOFFSET: u32 = 1004u32;
pub const DISPID_IHTMLSELECTION_GETRANGEAT: u32 = 1012u32;
pub const DISPID_IHTMLSELECTION_ISCOLLAPSED: u32 = 1005u32;
pub const DISPID_IHTMLSELECTION_RANGECOUNT: u32 = 1011u32;
pub const DISPID_IHTMLSELECTION_REMOVEALLRANGES: u32 = 1015u32;
pub const DISPID_IHTMLSELECTION_REMOVERANGE: u32 = 1014u32;
pub const DISPID_IHTMLSELECTION_SELECTALLCHILDREN: u32 = 1009u32;
pub const DISPID_IHTMLSELECTION_TOSTRING: u32 = 1016u32;
pub const DISPID_IHTMLSOURCEELEMENT_MEDIA: u32 = 1002u32;
pub const DISPID_IHTMLSOURCEELEMENT_SRC: u32 = 1000u32;
pub const DISPID_IHTMLSOURCEELEMENT_TYPE: u32 = 1001u32;
pub const DISPID_IHTMLSPANFLOW_ALIGN: u32 = 65609u32;
pub const DISPID_IHTMLSTORAGE2_IE9_SETITEM: u32 = 1008u32;
pub const DISPID_IHTMLSTORAGE_CLEAR: u32 = 1007u32;
pub const DISPID_IHTMLSTORAGE_GETITEM: u32 = 1003u32;
pub const DISPID_IHTMLSTORAGE_KEY: u32 = 1006u32;
pub const DISPID_IHTMLSTORAGE_LENGTH: u32 = 1001u32;
pub const DISPID_IHTMLSTORAGE_REMAININGSPACE: u32 = 1002u32;
pub const DISPID_IHTMLSTORAGE_REMOVEITEM: u32 = 1005u32;
pub const DISPID_IHTMLSTORAGE_SETITEM: u32 = 1004u32;
pub const DISPID_IHTMLSTYLE2_ACCELERATOR: u32 = 70683u32;
pub const DISPID_IHTMLSTYLE2_BEHAVIOR: u32 = 70651u32;
pub const DISPID_IHTMLSTYLE2_BORDERCOLLAPSE: u32 = 70620u32;
pub const DISPID_IHTMLSTYLE2_BOTTOM: u32 = 65614u32;
pub const DISPID_IHTMLSTYLE2_DIRECTION: u32 = 70655u32;
pub const DISPID_IHTMLSTYLE2_GETEXPRESSION: u32 = 66041u32;
pub const DISPID_IHTMLSTYLE2_IMEMODE: u32 = 70656u32;
pub const DISPID_IHTMLSTYLE2_LAYOUTGRID: u32 = 70667u32;
pub const DISPID_IHTMLSTYLE2_LAYOUTGRIDCHAR: u32 = 70663u32;
pub const DISPID_IHTMLSTYLE2_LAYOUTGRIDLINE: u32 = 70664u32;
pub const DISPID_IHTMLSTYLE2_LAYOUTGRIDMODE: u32 = 70665u32;
pub const DISPID_IHTMLSTYLE2_LAYOUTGRIDTYPE: u32 = 70666u32;
pub const DISPID_IHTMLSTYLE2_LINEBREAK: u32 = 70669u32;
pub const DISPID_IHTMLSTYLE2_OVERFLOWX: u32 = 70675u32;
pub const DISPID_IHTMLSTYLE2_OVERFLOWY: u32 = 70676u32;
pub const DISPID_IHTMLSTYLE2_PIXELBOTTOM: u32 = 69545u32;
pub const DISPID_IHTMLSTYLE2_PIXELRIGHT: u32 = 69546u32;
pub const DISPID_IHTMLSTYLE2_POSBOTTOM: u32 = 69547u32;
pub const DISPID_IHTMLSTYLE2_POSITION: u32 = 70626u32;
pub const DISPID_IHTMLSTYLE2_POSRIGHT: u32 = 69548u32;
pub const DISPID_IHTMLSTYLE2_REMOVEEXPRESSION: u32 = 66042u32;
pub const DISPID_IHTMLSTYLE2_RIGHT: u32 = 65613u32;
pub const DISPID_IHTMLSTYLE2_RUBYALIGN: u32 = 70657u32;
pub const DISPID_IHTMLSTYLE2_RUBYOVERHANG: u32 = 70659u32;
pub const DISPID_IHTMLSTYLE2_RUBYPOSITION: u32 = 70658u32;
pub const DISPID_IHTMLSTYLE2_SETEXPRESSION: u32 = 66040u32;
pub const DISPID_IHTMLSTYLE2_TABLELAYOUT: u32 = 70634u32;
pub const DISPID_IHTMLSTYLE2_TEXTAUTOSPACE: u32 = 70668u32;
pub const DISPID_IHTMLSTYLE2_TEXTJUSTIFY: u32 = 70671u32;
pub const DISPID_IHTMLSTYLE2_TEXTJUSTIFYTRIM: u32 = 70672u32;
pub const DISPID_IHTMLSTYLE2_TEXTKASHIDA: u32 = 70673u32;
pub const DISPID_IHTMLSTYLE2_UNICODEBIDI: u32 = 70654u32;
pub const DISPID_IHTMLSTYLE2_WORDBREAK: u32 = 70670u32;
pub const DISPID_IHTMLSTYLE3_LAYOUTFLOW: u32 = 70691u32;
pub const DISPID_IHTMLSTYLE3_SCROLLBAR3DLIGHTCOLOR: u32 = 70718u32;
pub const DISPID_IHTMLSTYLE3_SCROLLBARARROWCOLOR: u32 = 70722u32;
pub const DISPID_IHTMLSTYLE3_SCROLLBARBASECOLOR: u32 = 70716u32;
pub const DISPID_IHTMLSTYLE3_SCROLLBARDARKSHADOWCOLOR: u32 = 70721u32;
pub const DISPID_IHTMLSTYLE3_SCROLLBARFACECOLOR: u32 = 70717u32;
pub const DISPID_IHTMLSTYLE3_SCROLLBARHIGHLIGHTCOLOR: u32 = 70720u32;
pub const DISPID_IHTMLSTYLE3_SCROLLBARSHADOWCOLOR: u32 = 70719u32;
pub const DISPID_IHTMLSTYLE3_SCROLLBARTRACKCOLOR: u32 = 70732u32;
pub const DISPID_IHTMLSTYLE3_TEXTALIGNLAST: u32 = 70739u32;
pub const DISPID_IHTMLSTYLE3_TEXTKASHIDASPACE: u32 = 70740u32;
pub const DISPID_IHTMLSTYLE3_TEXTUNDERLINEPOSITION: u32 = 70695u32;
pub const DISPID_IHTMLSTYLE3_WORDWRAP: u32 = 70694u32;
pub const DISPID_IHTMLSTYLE3_WRITINGMODE: u32 = 70728u32;
pub const DISPID_IHTMLSTYLE3_ZOOM: u32 = 70689u32;
pub const DISPID_IHTMLSTYLE4_MINHEIGHT: u32 = 70747u32;
pub const DISPID_IHTMLSTYLE4_TEXTOVERFLOW: u32 = 70745u32;
pub const DISPID_IHTMLSTYLE5_MAXHEIGHT: u32 = 70750u32;
pub const DISPID_IHTMLSTYLE5_MAXWIDTH: u32 = 70752u32;
pub const DISPID_IHTMLSTYLE5_MINWIDTH: u32 = 70751u32;
pub const DISPID_IHTMLSTYLE5_MSINTERPOLATIONMODE: u32 = 70749u32;
pub const DISPID_IHTMLSTYLE6_BORDERSPACING: u32 = 70763u32;
pub const DISPID_IHTMLSTYLE6_BOXSIZING: u32 = 70762u32;
pub const DISPID_IHTMLSTYLE6_CAPTIONSIDE: u32 = 70755u32;
pub const DISPID_IHTMLSTYLE6_CONTENT: u32 = 70754u32;
pub const DISPID_IHTMLSTYLE6_COUNTERINCREMENT: u32 = 70756u32;
pub const DISPID_IHTMLSTYLE6_COUNTERRESET: u32 = 70757u32;
pub const DISPID_IHTMLSTYLE6_EMPTYCELLS: u32 = 70786u32;
pub const DISPID_IHTMLSTYLE6_MSBLOCKPROGRESSION: u32 = 70787u32;
pub const DISPID_IHTMLSTYLE6_ORPHANS: u32 = 70764u32;
pub const DISPID_IHTMLSTYLE6_OUTLINE: u32 = 70758u32;
pub const DISPID_IHTMLSTYLE6_OUTLINECOLOR: u32 = 70761u32;
pub const DISPID_IHTMLSTYLE6_OUTLINESTYLE: u32 = 70760u32;
pub const DISPID_IHTMLSTYLE6_OUTLINEWIDTH: u32 = 70759u32;
pub const DISPID_IHTMLSTYLE6_PAGEBREAKINSIDE: u32 = 70766u32;
pub const DISPID_IHTMLSTYLE6_QUOTES: u32 = 70788u32;
pub const DISPID_IHTMLSTYLE6_WIDOWS: u32 = 70765u32;
pub const DISPID_IHTMLSTYLEELEMENT2_SHEET: u32 = 1007u32;
pub const DISPID_IHTMLSTYLEELEMENT_DISABLED: u32 = 65612u32;
pub const DISPID_IHTMLSTYLEELEMENT_MEDIA: u32 = 1006u32;
pub const DISPID_IHTMLSTYLEELEMENT_ONERROR: u32 = 71565u32;
pub const DISPID_IHTMLSTYLEELEMENT_ONLOAD: u32 = 71568u32;
pub const DISPID_IHTMLSTYLEELEMENT_ONREADYSTATECHANGE: u32 = 71561u32;
pub const DISPID_IHTMLSTYLEELEMENT_READYSTATE: u32 = 70652u32;
pub const DISPID_IHTMLSTYLEELEMENT_STYLESHEET: u32 = 1004u32;
pub const DISPID_IHTMLSTYLEELEMENT_TYPE: u32 = 1002u32;
pub const DISPID_IHTMLSTYLEENABLED_MSGETPROPERTYENABLED: u32 = 70043u32;
pub const DISPID_IHTMLSTYLEENABLED_MSPUTPROPERTYENABLED: u32 = 70044u32;
pub const DISPID_IHTMLSTYLEFONTFACE2_STYLE: u32 = 65610u32;
pub const DISPID_IHTMLSTYLEFONTFACE_FONTSRC: u32 = 70633u32;
pub const DISPID_IHTMLSTYLEMEDIA_MATCHMEDIUM: u32 = 1002u32;
pub const DISPID_IHTMLSTYLEMEDIA_TYPE: u32 = 1001u32;
pub const DISPID_IHTMLSTYLESHEET2_ADDPAGERULE: u32 = 1017u32;
pub const DISPID_IHTMLSTYLESHEET2_PAGES: u32 = 1016u32;
pub const DISPID_IHTMLSTYLESHEET3_IE8_HREF: u32 = 1150u32;
pub const DISPID_IHTMLSTYLESHEET3_ISALTERNATE: u32 = 1151u32;
pub const DISPID_IHTMLSTYLESHEET3_ISPREFALTERNATE: u32 = 1152u32;
pub const DISPID_IHTMLSTYLESHEET4_CSSRULES: u32 = 1158u32;
pub const DISPID_IHTMLSTYLESHEET4_DELETERULE: u32 = 1161u32;
pub const DISPID_IHTMLSTYLESHEET4_IE9_HREF: u32 = 1154u32;
pub const DISPID_IHTMLSTYLESHEET4_IE9_MEDIA: u32 = 1159u32;
pub const DISPID_IHTMLSTYLESHEET4_IE9_TITLE: u32 = 1155u32;
pub const DISPID_IHTMLSTYLESHEET4_IE9_TYPE: u32 = 1153u32;
pub const DISPID_IHTMLSTYLESHEET4_INSERTRULE: u32 = 1160u32;
pub const DISPID_IHTMLSTYLESHEET4_OWNERNODE: u32 = 1156u32;
pub const DISPID_IHTMLSTYLESHEET4_OWNERRULE: u32 = 1157u32;
pub const DISPID_IHTMLSTYLESHEETPAGE2_SELECTORTEXT: u32 = 1003u32;
pub const DISPID_IHTMLSTYLESHEETPAGE2_STYLE: u32 = 65610u32;
pub const DISPID_IHTMLSTYLESHEETPAGESCOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLSTYLESHEETPAGESCOLLECTION_LENGTH: u32 = 1001u32;
pub const DISPID_IHTMLSTYLESHEETPAGE_PSEUDOCLASS: u32 = 1002u32;
pub const DISPID_IHTMLSTYLESHEETPAGE_SELECTOR: u32 = 1001u32;
pub const DISPID_IHTMLSTYLESHEETRULE2_IE9_SELECTORTEXT: u32 = 1005u32;
pub const DISPID_IHTMLSTYLESHEETRULEAPPLIED_MSGETSPECIFICITY: u32 = 1004u32;
pub const DISPID_IHTMLSTYLESHEETRULEAPPLIED_MSSPECIFICITY: u32 = 1003u32;
pub const DISPID_IHTMLSTYLESHEETRULESAPPLIEDCOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLSTYLESHEETRULESAPPLIEDCOLLECTION_LENGTH: u32 = 1001u32;
pub const DISPID_IHTMLSTYLESHEETRULESAPPLIEDCOLLECTION_PROPERTYAPPLIEDBY: u32 = 1002u32;
pub const DISPID_IHTMLSTYLESHEETRULESAPPLIEDCOLLECTION_PROPERTYAPPLIEDTRACE: u32 = 1004u32;
pub const DISPID_IHTMLSTYLESHEETRULESAPPLIEDCOLLECTION_PROPERTYAPPLIEDTRACELENGTH: u32 = 1005u32;
pub const DISPID_IHTMLSTYLESHEETRULESCOLLECTION2_IE9_ITEM: u32 = 1002u32;
pub const DISPID_IHTMLSTYLESHEETRULESCOLLECTION2_IE9_LENGTH: u32 = 1003u32;
pub const DISPID_IHTMLSTYLESHEETRULESCOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLSTYLESHEETRULESCOLLECTION_LENGTH: u32 = 1001u32;
pub const DISPID_IHTMLSTYLESHEETRULE_READONLY: u32 = 1002u32;
pub const DISPID_IHTMLSTYLESHEETRULE_SELECTORTEXT: u32 = 1001u32;
pub const DISPID_IHTMLSTYLESHEETRULE_STYLE: u32 = 65610u32;
pub const DISPID_IHTMLSTYLESHEETSCOLLECTION2_IE9_ITEM: u32 = 1002u32;
pub const DISPID_IHTMLSTYLESHEETSCOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLSTYLESHEETSCOLLECTION_LENGTH: u32 = 1001u32;
pub const DISPID_IHTMLSTYLESHEETSCOLLECTION__NEWENUM: i32 = -4i32;
pub const DISPID_IHTMLSTYLESHEET_ADDIMPORT: u32 = 1009u32;
pub const DISPID_IHTMLSTYLESHEET_ADDRULE: u32 = 1010u32;
pub const DISPID_IHTMLSTYLESHEET_CSSTEXT: u32 = 1014u32;
pub const DISPID_IHTMLSTYLESHEET_DISABLED: u32 = 65612u32;
pub const DISPID_IHTMLSTYLESHEET_HREF: u32 = 1006u32;
pub const DISPID_IHTMLSTYLESHEET_ID: u32 = 1008u32;
pub const DISPID_IHTMLSTYLESHEET_IMPORTS: u32 = 1005u32;
pub const DISPID_IHTMLSTYLESHEET_MEDIA: u32 = 1013u32;
pub const DISPID_IHTMLSTYLESHEET_OWNINGELEMENT: u32 = 1003u32;
pub const DISPID_IHTMLSTYLESHEET_PARENTSTYLESHEET: u32 = 1002u32;
pub const DISPID_IHTMLSTYLESHEET_READONLY: u32 = 1004u32;
pub const DISPID_IHTMLSTYLESHEET_REMOVEIMPORT: u32 = 1011u32;
pub const DISPID_IHTMLSTYLESHEET_REMOVERULE: u32 = 1012u32;
pub const DISPID_IHTMLSTYLESHEET_RULES: u32 = 1015u32;
pub const DISPID_IHTMLSTYLESHEET_TITLE: u32 = 1001u32;
pub const DISPID_IHTMLSTYLESHEET_TYPE: u32 = 1007u32;
pub const DISPID_IHTMLSTYLE_BACKGROUND: u32 = 70568u32;
pub const DISPID_IHTMLSTYLE_BACKGROUNDATTACHMENT: u32 = 70581u32;
pub const DISPID_IHTMLSTYLE_BACKGROUNDCOLOR: i32 = -501i32;
pub const DISPID_IHTMLSTYLE_BACKGROUNDIMAGE: u32 = 70537u32;
pub const DISPID_IHTMLSTYLE_BACKGROUNDPOSITION: u32 = 70582u32;
pub const DISPID_IHTMLSTYLE_BACKGROUNDPOSITIONX: u32 = 70569u32;
pub const DISPID_IHTMLSTYLE_BACKGROUNDPOSITIONY: u32 = 70570u32;
pub const DISPID_IHTMLSTYLE_BACKGROUNDREPEAT: u32 = 70580u32;
pub const DISPID_IHTMLSTYLE_BORDER: u32 = 70585u32;
pub const DISPID_IHTMLSTYLE_BORDERBOTTOM: u32 = 70588u32;
pub const DISPID_IHTMLSTYLE_BORDERBOTTOMCOLOR: u32 = 70593u32;
pub const DISPID_IHTMLSTYLE_BORDERBOTTOMSTYLE: u32 = 70603u32;
pub const DISPID_IHTMLSTYLE_BORDERBOTTOMWIDTH: u32 = 70598u32;
pub const DISPID_IHTMLSTYLE_BORDERCOLOR: u32 = 70590u32;
pub const DISPID_IHTMLSTYLE_BORDERLEFT: u32 = 70589u32;
pub const DISPID_IHTMLSTYLE_BORDERLEFTCOLOR: u32 = 70594u32;
pub const DISPID_IHTMLSTYLE_BORDERLEFTSTYLE: u32 = 70604u32;
pub const DISPID_IHTMLSTYLE_BORDERLEFTWIDTH: u32 = 70599u32;
pub const DISPID_IHTMLSTYLE_BORDERRIGHT: u32 = 70587u32;
pub const DISPID_IHTMLSTYLE_BORDERRIGHTCOLOR: u32 = 70592u32;
pub const DISPID_IHTMLSTYLE_BORDERRIGHTSTYLE: u32 = 70602u32;
pub const DISPID_IHTMLSTYLE_BORDERRIGHTWIDTH: u32 = 70597u32;
pub const DISPID_IHTMLSTYLE_BORDERSTYLE: u32 = 70600u32;
pub const DISPID_IHTMLSTYLE_BORDERTOP: u32 = 70586u32;
pub const DISPID_IHTMLSTYLE_BORDERTOPCOLOR: u32 = 70591u32;
pub const DISPID_IHTMLSTYLE_BORDERTOPSTYLE: u32 = 70601u32;
pub const DISPID_IHTMLSTYLE_BORDERTOPWIDTH: u32 = 70596u32;
pub const DISPID_IHTMLSTYLE_BORDERWIDTH: u32 = 70595u32;
pub const DISPID_IHTMLSTYLE_CLEAR: u32 = 70552u32;
pub const DISPID_IHTMLSTYLE_CLIP: u32 = 70628u32;
pub const DISPID_IHTMLSTYLE_COLOR: u32 = 70538u32;
pub const DISPID_IHTMLSTYLE_CSSTEXT: u32 = 70635u32;
pub const DISPID_IHTMLSTYLE_CURSOR: u32 = 70638u32;
pub const DISPID_IHTMLSTYLE_DISPLAY: u32 = 70607u32;
pub const DISPID_IHTMLSTYLE_FILTER: u32 = 70618u32;
pub const DISPID_IHTMLSTYLE_FONT: u32 = 70577u32;
pub const DISPID_IHTMLSTYLE_FONTFAMILY: u32 = 70554u32;
pub const DISPID_IHTMLSTYLE_FONTSIZE: u32 = 70555u32;
pub const DISPID_IHTMLSTYLE_FONTSTYLE: u32 = 70560u32;
pub const DISPID_IHTMLSTYLE_FONTVARIANT: u32 = 70561u32;
pub const DISPID_IHTMLSTYLE_FONTWEIGHT: u32 = 70563u32;
pub const DISPID_IHTMLSTYLE_GETATTRIBUTE: u32 = 66038u32;
pub const DISPID_IHTMLSTYLE_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLSTYLE_LEFT: u32 = 65539u32;
pub const DISPID_IHTMLSTYLE_LETTERSPACING: u32 = 70544u32;
pub const DISPID_IHTMLSTYLE_LINEHEIGHT: u32 = 70542u32;
pub const DISPID_IHTMLSTYLE_LISTSTYLE: u32 = 70611u32;
pub const DISPID_IHTMLSTYLE_LISTSTYLEIMAGE: u32 = 70610u32;
pub const DISPID_IHTMLSTYLE_LISTSTYLEPOSITION: u32 = 70609u32;
pub const DISPID_IHTMLSTYLE_LISTSTYLETYPE: u32 = 70608u32;
pub const DISPID_IHTMLSTYLE_MARGIN: u32 = 70572u32;
pub const DISPID_IHTMLSTYLE_MARGINBOTTOM: u32 = 70575u32;
pub const DISPID_IHTMLSTYLE_MARGINLEFT: u32 = 70576u32;
pub const DISPID_IHTMLSTYLE_MARGINRIGHT: u32 = 70574u32;
pub const DISPID_IHTMLSTYLE_MARGINTOP: u32 = 70573u32;
pub const DISPID_IHTMLSTYLE_OVERFLOW: u32 = 70546u32;
pub const DISPID_IHTMLSTYLE_PADDING: u32 = 70547u32;
pub const DISPID_IHTMLSTYLE_PADDINGBOTTOM: u32 = 70550u32;
pub const DISPID_IHTMLSTYLE_PADDINGLEFT: u32 = 70551u32;
pub const DISPID_IHTMLSTYLE_PADDINGRIGHT: u32 = 70549u32;
pub const DISPID_IHTMLSTYLE_PADDINGTOP: u32 = 70548u32;
pub const DISPID_IHTMLSTYLE_PAGEBREAKAFTER: u32 = 70614u32;
pub const DISPID_IHTMLSTYLE_PAGEBREAKBEFORE: u32 = 70613u32;
pub const DISPID_IHTMLSTYLE_PIXELHEIGHT: u32 = 69539u32;
pub const DISPID_IHTMLSTYLE_PIXELLEFT: u32 = 69537u32;
pub const DISPID_IHTMLSTYLE_PIXELTOP: u32 = 69536u32;
pub const DISPID_IHTMLSTYLE_PIXELWIDTH: u32 = 69538u32;
pub const DISPID_IHTMLSTYLE_POSHEIGHT: u32 = 69543u32;
pub const DISPID_IHTMLSTYLE_POSITION: u32 = 70626u32;
pub const DISPID_IHTMLSTYLE_POSLEFT: u32 = 69541u32;
pub const DISPID_IHTMLSTYLE_POSTOP: u32 = 69540u32;
pub const DISPID_IHTMLSTYLE_POSWIDTH: u32 = 69542u32;
pub const DISPID_IHTMLSTYLE_REMOVEATTRIBUTE: u32 = 66039u32;
pub const DISPID_IHTMLSTYLE_SETATTRIBUTE: u32 = 66037u32;
pub const DISPID_IHTMLSTYLE_STYLEFLOAT: u32 = 70606u32;
pub const DISPID_IHTMLSTYLE_TEXTALIGN: u32 = 65608u32;
pub const DISPID_IHTMLSTYLE_TEXTDECORATION: u32 = 70571u32;
pub const DISPID_IHTMLSTYLE_TEXTDECORATIONBLINK: u32 = 70558u32;
pub const DISPID_IHTMLSTYLE_TEXTDECORATIONLINETHROUGH: u32 = 70556u32;
pub const DISPID_IHTMLSTYLE_TEXTDECORATIONNONE: u32 = 70559u32;
pub const DISPID_IHTMLSTYLE_TEXTDECORATIONOVERLINE: u32 = 70605u32;
pub const DISPID_IHTMLSTYLE_TEXTDECORATIONUNDERLINE: u32 = 70557u32;
pub const DISPID_IHTMLSTYLE_TEXTINDENT: u32 = 70543u32;
pub const DISPID_IHTMLSTYLE_TEXTTRANSFORM: u32 = 70540u32;
pub const DISPID_IHTMLSTYLE_TOP: u32 = 65540u32;
pub const DISPID_IHTMLSTYLE_TOSTRING: u32 = 69544u32;
pub const DISPID_IHTMLSTYLE_VERTICALALIGN: u32 = 70584u32;
pub const DISPID_IHTMLSTYLE_VISIBILITY: u32 = 70616u32;
pub const DISPID_IHTMLSTYLE_WHITESPACE: u32 = 70612u32;
pub const DISPID_IHTMLSTYLE_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLSTYLE_WORDSPACING: u32 = 70583u32;
pub const DISPID_IHTMLSTYLE_ZINDEX: u32 = 70627u32;
pub const DISPID_IHTMLSUBMITDATA_APPENDITEMSEPARATOR: u32 = 1014u32;
pub const DISPID_IHTMLSUBMITDATA_APPENDNAMEFILEPAIR: u32 = 1013u32;
pub const DISPID_IHTMLSUBMITDATA_APPENDNAMEVALUEPAIR: u32 = 1012u32;
pub const DISPID_IHTMLTABLE2_CELLS: u32 = 1037u32;
pub const DISPID_IHTMLTABLE2_FIRSTPAGE: u32 = 1035u32;
pub const DISPID_IHTMLTABLE2_LASTPAGE: u32 = 1036u32;
pub const DISPID_IHTMLTABLE2_MOVEROW: u32 = 1038u32;
pub const DISPID_IHTMLTABLE3_SUMMARY: u32 = 1039u32;
pub const DISPID_IHTMLTABLE4_CREATETBODY: u32 = 1045u32;
pub const DISPID_IHTMLTABLE4_IE9_CAPTION: u32 = 1042u32;
pub const DISPID_IHTMLTABLE4_IE9_DELETEROW: u32 = 1044u32;
pub const DISPID_IHTMLTABLE4_IE9_INSERTROW: u32 = 1043u32;
pub const DISPID_IHTMLTABLE4_IE9_TFOOT: u32 = 1041u32;
pub const DISPID_IHTMLTABLE4_IE9_THEAD: u32 = 1040u32;
pub const DISPID_IHTMLTABLECAPTION_ALIGN: u32 = 65608u32;
pub const DISPID_IHTMLTABLECAPTION_VALIGN: u32 = 70567u32;
pub const DISPID_IHTMLTABLECELL2_ABBR: u32 = 2004u32;
pub const DISPID_IHTMLTABLECELL2_AXIS: u32 = 2005u32;
pub const DISPID_IHTMLTABLECELL2_CH: u32 = 2006u32;
pub const DISPID_IHTMLTABLECELL2_CHOFF: u32 = 2007u32;
pub const DISPID_IHTMLTABLECELL2_HEADERS: u32 = 2008u32;
pub const DISPID_IHTMLTABLECELL2_SCOPE: u32 = 2009u32;
pub const DISPID_IHTMLTABLECELL3_IE9_CH: u32 = 2010u32;
pub const DISPID_IHTMLTABLECELL3_IE9_CHOFF: u32 = 2011u32;
pub const DISPID_IHTMLTABLECELL_ALIGN: u32 = 65608u32;
pub const DISPID_IHTMLTABLECELL_BACKGROUND: u32 = 70537u32;
pub const DISPID_IHTMLTABLECELL_BGCOLOR: i32 = -501i32;
pub const DISPID_IHTMLTABLECELL_BORDERCOLOR: u32 = 70564u32;
pub const DISPID_IHTMLTABLECELL_BORDERCOLORDARK: u32 = 70566u32;
pub const DISPID_IHTMLTABLECELL_BORDERCOLORLIGHT: u32 = 70565u32;
pub const DISPID_IHTMLTABLECELL_CELLINDEX: u32 = 2003u32;
pub const DISPID_IHTMLTABLECELL_COLSPAN: u32 = 2002u32;
pub const DISPID_IHTMLTABLECELL_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLTABLECELL_NOWRAP: u32 = 70541u32;
pub const DISPID_IHTMLTABLECELL_ROWSPAN: u32 = 2001u32;
pub const DISPID_IHTMLTABLECELL_VALIGN: u32 = 70567u32;
pub const DISPID_IHTMLTABLECELL_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLTABLECOL2_CH: u32 = 1002u32;
pub const DISPID_IHTMLTABLECOL2_CHOFF: u32 = 1003u32;
pub const DISPID_IHTMLTABLECOL3_IE9_CH: u32 = 1004u32;
pub const DISPID_IHTMLTABLECOL3_IE9_CHOFF: u32 = 1005u32;
pub const DISPID_IHTMLTABLECOL_ALIGN: u32 = 65608u32;
pub const DISPID_IHTMLTABLECOL_SPAN: u32 = 1001u32;
pub const DISPID_IHTMLTABLECOL_VALIGN: u32 = 70567u32;
pub const DISPID_IHTMLTABLECOL_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLTABLEROW2_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLTABLEROW3_CH: u32 = 1009u32;
pub const DISPID_IHTMLTABLEROW3_CHOFF: u32 = 1010u32;
pub const DISPID_IHTMLTABLEROW4_IE9_CH: u32 = 1011u32;
pub const DISPID_IHTMLTABLEROW4_IE9_CHOFF: u32 = 1012u32;
pub const DISPID_IHTMLTABLEROW4_IE9_DELETECELL: u32 = 1014u32;
pub const DISPID_IHTMLTABLEROW4_IE9_INSERTCELL: u32 = 1013u32;
pub const DISPID_IHTMLTABLEROWMETRICS_CLIENTHEIGHT: u32 = 67555u32;
pub const DISPID_IHTMLTABLEROWMETRICS_CLIENTLEFT: u32 = 67558u32;
pub const DISPID_IHTMLTABLEROWMETRICS_CLIENTTOP: u32 = 67557u32;
pub const DISPID_IHTMLTABLEROWMETRICS_CLIENTWIDTH: u32 = 67556u32;
pub const DISPID_IHTMLTABLEROW_ALIGN: u32 = 65608u32;
pub const DISPID_IHTMLTABLEROW_BGCOLOR: i32 = -501i32;
pub const DISPID_IHTMLTABLEROW_BORDERCOLOR: u32 = 70564u32;
pub const DISPID_IHTMLTABLEROW_BORDERCOLORDARK: u32 = 70566u32;
pub const DISPID_IHTMLTABLEROW_BORDERCOLORLIGHT: u32 = 70565u32;
pub const DISPID_IHTMLTABLEROW_CELLS: u32 = 1002u32;
pub const DISPID_IHTMLTABLEROW_DELETECELL: u32 = 1004u32;
pub const DISPID_IHTMLTABLEROW_INSERTCELL: u32 = 1003u32;
pub const DISPID_IHTMLTABLEROW_ROWINDEX: u32 = 1000u32;
pub const DISPID_IHTMLTABLEROW_SECTIONROWINDEX: u32 = 1001u32;
pub const DISPID_IHTMLTABLEROW_VALIGN: u32 = 70567u32;
pub const DISPID_IHTMLTABLESECTION2_MOVEROW: u32 = 1003u32;
pub const DISPID_IHTMLTABLESECTION3_CH: u32 = 1004u32;
pub const DISPID_IHTMLTABLESECTION3_CHOFF: u32 = 1005u32;
pub const DISPID_IHTMLTABLESECTION4_IE9_CH: u32 = 1006u32;
pub const DISPID_IHTMLTABLESECTION4_IE9_CHOFF: u32 = 1007u32;
pub const DISPID_IHTMLTABLESECTION4_IE9_DELETEROW: u32 = 1009u32;
pub const DISPID_IHTMLTABLESECTION4_IE9_INSERTROW: u32 = 1008u32;
pub const DISPID_IHTMLTABLESECTION_ALIGN: u32 = 65608u32;
pub const DISPID_IHTMLTABLESECTION_BGCOLOR: i32 = -501i32;
pub const DISPID_IHTMLTABLESECTION_DELETEROW: u32 = 1002u32;
pub const DISPID_IHTMLTABLESECTION_INSERTROW: u32 = 1001u32;
pub const DISPID_IHTMLTABLESECTION_ROWS: u32 = 1000u32;
pub const DISPID_IHTMLTABLESECTION_VALIGN: u32 = 70567u32;
pub const DISPID_IHTMLTABLE_ALIGN: u32 = 65609u32;
pub const DISPID_IHTMLTABLE_BACKGROUND: u32 = 70537u32;
pub const DISPID_IHTMLTABLE_BGCOLOR: i32 = -501i32;
pub const DISPID_IHTMLTABLE_BORDER: u32 = 1002u32;
pub const DISPID_IHTMLTABLE_BORDERCOLOR: u32 = 70564u32;
pub const DISPID_IHTMLTABLE_BORDERCOLORDARK: u32 = 70566u32;
pub const DISPID_IHTMLTABLE_BORDERCOLORLIGHT: u32 = 70565u32;
pub const DISPID_IHTMLTABLE_CAPTION: u32 = 1025u32;
pub const DISPID_IHTMLTABLE_CELLPADDING: u32 = 1006u32;
pub const DISPID_IHTMLTABLE_CELLSPACING: u32 = 1005u32;
pub const DISPID_IHTMLTABLE_COLS: u32 = 1001u32;
pub const DISPID_IHTMLTABLE_CREATECAPTION: u32 = 1030u32;
pub const DISPID_IHTMLTABLE_CREATETFOOT: u32 = 1028u32;
pub const DISPID_IHTMLTABLE_CREATETHEAD: u32 = 1026u32;
pub const DISPID_IHTMLTABLE_DATAPAGESIZE: u32 = 1017u32;
pub const DISPID_IHTMLTABLE_DELETECAPTION: u32 = 1031u32;
pub const DISPID_IHTMLTABLE_DELETEROW: u32 = 1033u32;
pub const DISPID_IHTMLTABLE_DELETETFOOT: u32 = 1029u32;
pub const DISPID_IHTMLTABLE_DELETETHEAD: u32 = 1027u32;
pub const DISPID_IHTMLTABLE_FRAME: u32 = 1004u32;
pub const DISPID_IHTMLTABLE_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLTABLE_INSERTROW: u32 = 1032u32;
pub const DISPID_IHTMLTABLE_NEXTPAGE: u32 = 1018u32;
pub const DISPID_IHTMLTABLE_ONREADYSTATECHANGE: u32 = 71561u32;
pub const DISPID_IHTMLTABLE_PREVIOUSPAGE: u32 = 1019u32;
pub const DISPID_IHTMLTABLE_READYSTATE: u32 = 70652u32;
pub const DISPID_IHTMLTABLE_REFRESH: u32 = 1015u32;
pub const DISPID_IHTMLTABLE_ROWS: u32 = 1016u32;
pub const DISPID_IHTMLTABLE_RULES: u32 = 1003u32;
pub const DISPID_IHTMLTABLE_TBODIES: u32 = 1024u32;
pub const DISPID_IHTMLTABLE_TFOOT: u32 = 1021u32;
pub const DISPID_IHTMLTABLE_THEAD: u32 = 1020u32;
pub const DISPID_IHTMLTABLE_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLTEXTAREAELEMENT2_SELECTIONEND: u32 = 7008u32;
pub const DISPID_IHTMLTEXTAREAELEMENT2_SELECTIONSTART: u32 = 7007u32;
pub const DISPID_IHTMLTEXTAREAELEMENT2_SETSELECTIONRANGE: u32 = 7009u32;
pub const DISPID_IHTMLTEXTAREAELEMENT_COLS: u32 = 7002u32;
pub const DISPID_IHTMLTEXTAREAELEMENT_CREATETEXTRANGE: u32 = 7006u32;
pub const DISPID_IHTMLTEXTAREAELEMENT_DEFAULTVALUE: u32 = 70619u32;
pub const DISPID_IHTMLTEXTAREAELEMENT_DISABLED: u32 = 65612u32;
pub const DISPID_IHTMLTEXTAREAELEMENT_FORM: u32 = 67540u32;
pub const DISPID_IHTMLTEXTAREAELEMENT_NAME: u32 = 65536u32;
pub const DISPID_IHTMLTEXTAREAELEMENT_ONCHANGE: u32 = 71566u32;
pub const DISPID_IHTMLTEXTAREAELEMENT_ONSELECT: u32 = 71546u32;
pub const DISPID_IHTMLTEXTAREAELEMENT_READONLY: u32 = 7004u32;
pub const DISPID_IHTMLTEXTAREAELEMENT_ROWS: u32 = 7001u32;
pub const DISPID_IHTMLTEXTAREAELEMENT_SELECT: u32 = 7005u32;
pub const DISPID_IHTMLTEXTAREAELEMENT_STATUS: u32 = 2001u32;
pub const DISPID_IHTMLTEXTAREAELEMENT_TYPE: u32 = 2000u32;
pub const DISPID_IHTMLTEXTAREAELEMENT_VALUE: u32 = 70637u32;
pub const DISPID_IHTMLTEXTAREAELEMENT_WRAP: u32 = 7003u32;
pub const DISPID_IHTMLTEXTCONTAINER_CREATECONTROLRANGE: u32 = 1001u32;
pub const DISPID_IHTMLTEXTCONTAINER_ONSCROLL: u32 = 71567u32;
pub const DISPID_IHTMLTEXTCONTAINER_SCROLLHEIGHT: u32 = 1002u32;
pub const DISPID_IHTMLTEXTCONTAINER_SCROLLLEFT: u32 = 1005u32;
pub const DISPID_IHTMLTEXTCONTAINER_SCROLLTOP: u32 = 1004u32;
pub const DISPID_IHTMLTEXTCONTAINER_SCROLLWIDTH: u32 = 1003u32;
pub const DISPID_IHTMLTEXTRANGEMETRICS2_GETBOUNDINGCLIENTRECT: u32 = 1042u32;
pub const DISPID_IHTMLTEXTRANGEMETRICS2_GETCLIENTRECTS: u32 = 1041u32;
pub const DISPID_IHTMLTEXTRANGEMETRICS_BOUNDINGHEIGHT: u32 = 1040u32;
pub const DISPID_IHTMLTEXTRANGEMETRICS_BOUNDINGLEFT: u32 = 1038u32;
pub const DISPID_IHTMLTEXTRANGEMETRICS_BOUNDINGTOP: u32 = 1037u32;
pub const DISPID_IHTMLTEXTRANGEMETRICS_BOUNDINGWIDTH: u32 = 1039u32;
pub const DISPID_IHTMLTEXTRANGEMETRICS_OFFSETLEFT: u32 = 1036u32;
pub const DISPID_IHTMLTEXTRANGEMETRICS_OFFSETTOP: u32 = 1035u32;
pub const DISPID_IHTMLTIMERANGES2_ENDDOUBLE: u32 = 1004u32;
pub const DISPID_IHTMLTIMERANGES2_STARTDOUBLE: u32 = 1003u32;
pub const DISPID_IHTMLTIMERANGES_END: u32 = 1002u32;
pub const DISPID_IHTMLTIMERANGES_LENGTH: u32 = 1000u32;
pub const DISPID_IHTMLTIMERANGES_START: u32 = 1001u32;
pub const DISPID_IHTMLTITLEELEMENT_TEXT: u32 = 70637u32;
pub const DISPID_IHTMLTXTRANGECOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLTXTRANGECOLLECTION_LENGTH: u32 = 1500u32;
pub const DISPID_IHTMLTXTRANGECOLLECTION__NEWENUM: i32 = -4i32;
pub const DISPID_IHTMLTXTRANGEINTERNAL_GET_VISIBLETEXT: u32 = 1050u32;
pub const DISPID_IHTMLTXTRANGE_COLLAPSE: u32 = 1013u32;
pub const DISPID_IHTMLTXTRANGE_COMPAREENDPOINTS: u32 = 1018u32;
pub const DISPID_IHTMLTXTRANGE_DUPLICATE: u32 = 1008u32;
pub const DISPID_IHTMLTXTRANGE_EXECCOMMAND: u32 = 1033u32;
pub const DISPID_IHTMLTXTRANGE_EXECCOMMANDSHOWHELP: u32 = 1034u32;
pub const DISPID_IHTMLTXTRANGE_EXPAND: u32 = 1014u32;
pub const DISPID_IHTMLTXTRANGE_FINDTEXT: u32 = 1019u32;
pub const DISPID_IHTMLTXTRANGE_GETBOOKMARK: u32 = 1021u32;
pub const DISPID_IHTMLTXTRANGE_HTMLTEXT: u32 = 1003u32;
pub const DISPID_IHTMLTXTRANGE_INRANGE: u32 = 1010u32;
pub const DISPID_IHTMLTXTRANGE_ISEQUAL: u32 = 1011u32;
pub const DISPID_IHTMLTXTRANGE_MOVE: u32 = 1015u32;
pub const DISPID_IHTMLTXTRANGE_MOVEEND: u32 = 1017u32;
pub const DISPID_IHTMLTXTRANGE_MOVESTART: u32 = 1016u32;
pub const DISPID_IHTMLTXTRANGE_MOVETOBOOKMARK: u32 = 1009u32;
pub const DISPID_IHTMLTXTRANGE_MOVETOELEMENTTEXT: u32 = 1001u32;
pub const DISPID_IHTMLTXTRANGE_MOVETOPOINT: u32 = 1020u32;
pub const DISPID_IHTMLTXTRANGE_PARENTELEMENT: u32 = 1006u32;
pub const DISPID_IHTMLTXTRANGE_PASTEHTML: u32 = 1026u32;
pub const DISPID_IHTMLTXTRANGE_QUERYCOMMANDENABLED: u32 = 1028u32;
pub const DISPID_IHTMLTXTRANGE_QUERYCOMMANDINDETERM: u32 = 1030u32;
pub const DISPID_IHTMLTXTRANGE_QUERYCOMMANDSTATE: u32 = 1029u32;
pub const DISPID_IHTMLTXTRANGE_QUERYCOMMANDSUPPORTED: u32 = 1027u32;
pub const DISPID_IHTMLTXTRANGE_QUERYCOMMANDTEXT: u32 = 1031u32;
pub const DISPID_IHTMLTXTRANGE_QUERYCOMMANDVALUE: u32 = 1032u32;
pub const DISPID_IHTMLTXTRANGE_SCROLLINTOVIEW: u32 = 1012u32;
pub const DISPID_IHTMLTXTRANGE_SELECT: u32 = 1024u32;
pub const DISPID_IHTMLTXTRANGE_SETENDPOINT: u32 = 1025u32;
pub const DISPID_IHTMLTXTRANGE_TEXT: u32 = 1004u32;
pub const DISPID_IHTMLULISTELEMENT_COMPACT: u32 = 1001u32;
pub const DISPID_IHTMLULISTELEMENT_TYPE: u32 = 70553u32;
pub const DISPID_IHTMLUNIQUENAME_UNIQUEID: u32 = 66591u32;
pub const DISPID_IHTMLUNIQUENAME_UNIQUENUMBER: u32 = 66590u32;
pub const DISPID_IHTMLURNCOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IHTMLURNCOLLECTION_LENGTH: u32 = 1001u32;
pub const DISPID_IHTMLVIDEOELEMENT_HEIGHT: u32 = 65542u32;
pub const DISPID_IHTMLVIDEOELEMENT_POSTER: u32 = 1052u32;
pub const DISPID_IHTMLVIDEOELEMENT_VIDEOHEIGHT: u32 = 1051u32;
pub const DISPID_IHTMLVIDEOELEMENT_VIDEOWIDTH: u32 = 1050u32;
pub const DISPID_IHTMLVIDEOELEMENT_WIDTH: u32 = 65541u32;
pub const DISPID_IHTMLWINDOW2_ALERT: u32 = 1105u32;
pub const DISPID_IHTMLWINDOW2_BLUR: u32 = 1159u32;
pub const DISPID_IHTMLWINDOW2_CLEARINTERVAL: u32 = 1163u32;
pub const DISPID_IHTMLWINDOW2_CLEARTIMEOUT: u32 = 1104u32;
pub const DISPID_IHTMLWINDOW2_CLIENTINFORMATION: u32 = 1161u32;
pub const DISPID_IHTMLWINDOW2_CLOSE: u32 = 3u32;
pub const DISPID_IHTMLWINDOW2_CLOSED: u32 = 23u32;
pub const DISPID_IHTMLWINDOW2_CONFIRM: u32 = 1110u32;
pub const DISPID_IHTMLWINDOW2_DEFAULTSTATUS: u32 = 1101u32;
pub const DISPID_IHTMLWINDOW2_DOCUMENT: u32 = 1151u32;
pub const DISPID_IHTMLWINDOW2_EVENT: u32 = 1152u32;
pub const DISPID_IHTMLWINDOW2_EXECSCRIPT: u32 = 1165u32;
pub const DISPID_IHTMLWINDOW2_EXTERNAL: u32 = 1169u32;
pub const DISPID_IHTMLWINDOW2_FOCUS: u32 = 1158u32;
pub const DISPID_IHTMLWINDOW2_FRAMES: u32 = 1100u32;
pub const DISPID_IHTMLWINDOW2_HISTORY: u32 = 2u32;
pub const DISPID_IHTMLWINDOW2_IMAGE: u32 = 1125u32;
pub const DISPID_IHTMLWINDOW2_LOCATION: u32 = 14u32;
pub const DISPID_IHTMLWINDOW2_MOVEBY: u32 = 7u32;
pub const DISPID_IHTMLWINDOW2_MOVETO: u32 = 6u32;
pub const DISPID_IHTMLWINDOW2_NAME: u32 = 11u32;
pub const DISPID_IHTMLWINDOW2_NAVIGATE: u32 = 25u32;
pub const DISPID_IHTMLWINDOW2_NAVIGATOR: u32 = 5u32;
pub const DISPID_IHTMLWINDOW2_OFFSCREENBUFFERING: u32 = 1164u32;
pub const DISPID_IHTMLWINDOW2_ONBEFOREUNLOAD: u32 = 71575u32;
pub const DISPID_IHTMLWINDOW2_ONBLUR: u32 = 71551u32;
pub const DISPID_IHTMLWINDOW2_ONERROR: u32 = 71565u32;
pub const DISPID_IHTMLWINDOW2_ONFOCUS: u32 = 71550u32;
pub const DISPID_IHTMLWINDOW2_ONHELP: u32 = 71549u32;
pub const DISPID_IHTMLWINDOW2_ONLOAD: u32 = 71568u32;
pub const DISPID_IHTMLWINDOW2_ONRESIZE: u32 = 71572u32;
pub const DISPID_IHTMLWINDOW2_ONSCROLL: u32 = 71567u32;
pub const DISPID_IHTMLWINDOW2_ONUNLOAD: u32 = 71569u32;
pub const DISPID_IHTMLWINDOW2_OPEN: u32 = 13u32;
pub const DISPID_IHTMLWINDOW2_OPENER: u32 = 4u32;
pub const DISPID_IHTMLWINDOW2_OPTION: u32 = 1157u32;
pub const DISPID_IHTMLWINDOW2_PARENT: u32 = 12u32;
pub const DISPID_IHTMLWINDOW2_PROMPT: u32 = 1111u32;
pub const DISPID_IHTMLWINDOW2_RESIZEBY: u32 = 8u32;
pub const DISPID_IHTMLWINDOW2_RESIZETO: u32 = 9u32;
pub const DISPID_IHTMLWINDOW2_SCREEN: u32 = 1156u32;
pub const DISPID_IHTMLWINDOW2_SCROLL: u32 = 1160u32;
pub const DISPID_IHTMLWINDOW2_SCROLLBY: u32 = 1167u32;
pub const DISPID_IHTMLWINDOW2_SCROLLTO: u32 = 1168u32;
pub const DISPID_IHTMLWINDOW2_SELF: u32 = 20u32;
pub const DISPID_IHTMLWINDOW2_SETINTERVAL: u32 = 1173u32;
pub const DISPID_IHTMLWINDOW2_SETTIMEOUT: u32 = 1172u32;
pub const DISPID_IHTMLWINDOW2_SHOWHELP: u32 = 1155u32;
pub const DISPID_IHTMLWINDOW2_SHOWMODALDIALOG: u32 = 1154u32;
pub const DISPID_IHTMLWINDOW2_STATUS: u32 = 1102u32;
pub const DISPID_IHTMLWINDOW2_TOP: u32 = 21u32;
pub const DISPID_IHTMLWINDOW2_TOSTRING: u32 = 1166u32;
pub const DISPID_IHTMLWINDOW2_WINDOW: u32 = 22u32;
pub const DISPID_IHTMLWINDOW2__NEWENUM: u32 = 1153u32;
pub const DISPID_IHTMLWINDOW3_ATTACHEVENT: u32 = 66043u32;
pub const DISPID_IHTMLWINDOW3_CLIPBOARDDATA: u32 = 1175u32;
pub const DISPID_IHTMLWINDOW3_DETACHEVENT: u32 = 66044u32;
pub const DISPID_IHTMLWINDOW3_ONAFTERPRINT: u32 = 71603u32;
pub const DISPID_IHTMLWINDOW3_ONBEFOREPRINT: u32 = 71602u32;
pub const DISPID_IHTMLWINDOW3_PRINT: u32 = 1174u32;
pub const DISPID_IHTMLWINDOW3_SCREENLEFT: u32 = 1170u32;
pub const DISPID_IHTMLWINDOW3_SCREENTOP: u32 = 1171u32;
pub const DISPID_IHTMLWINDOW3_SETINTERVAL: u32 = 1162u32;
pub const DISPID_IHTMLWINDOW3_SETTIMEOUT: u32 = 1103u32;
pub const DISPID_IHTMLWINDOW3_SHOWMODELESSDIALOG: u32 = 1176u32;
pub const DISPID_IHTMLWINDOW4_CREATEPOPUP: u32 = 1180u32;
pub const DISPID_IHTMLWINDOW4_FRAMEELEMENT: u32 = 1181u32;
pub const DISPID_IHTMLWINDOW5_XMLHTTPREQUEST: u32 = 1190u32;
pub const DISPID_IHTMLWINDOW6_LOCALSTORAGE: u32 = 1193u32;
pub const DISPID_IHTMLWINDOW6_MAXCONNECTIONSPERSERVER: u32 = 1194u32;
pub const DISPID_IHTMLWINDOW6_MSWRITEPROFILERMARK: u32 = 1198u32;
pub const DISPID_IHTMLWINDOW6_ONHASHCHANGE: u32 = 71645u32;
pub const DISPID_IHTMLWINDOW6_ONMESSAGE: u32 = 71646u32;
pub const DISPID_IHTMLWINDOW6_POSTMESSAGE: u32 = 1196u32;
pub const DISPID_IHTMLWINDOW6_SESSIONSTORAGE: u32 = 1192u32;
pub const DISPID_IHTMLWINDOW6_TOSTATICHTML: u32 = 1197u32;
pub const DISPID_IHTMLWINDOW6_XDOMAINREQUEST: u32 = 1191u32;
pub const DISPID_IHTMLWINDOW7_GETCOMPUTEDSTYLE: u32 = 1200u32;
pub const DISPID_IHTMLWINDOW7_GETSELECTION: u32 = 1199u32;
pub const DISPID_IHTMLWINDOW7_INNERHEIGHT: u32 = 1205u32;
pub const DISPID_IHTMLWINDOW7_INNERWIDTH: u32 = 1204u32;
pub const DISPID_IHTMLWINDOW7_ONABORT: u32 = 71564u32;
pub const DISPID_IHTMLWINDOW7_ONCANPLAY: u32 = 71670u32;
pub const DISPID_IHTMLWINDOW7_ONCANPLAYTHROUGH: u32 = 71671u32;
pub const DISPID_IHTMLWINDOW7_ONCHANGE: u32 = 71566u32;
pub const DISPID_IHTMLWINDOW7_ONCLICK: u32 = 71544u32;
pub const DISPID_IHTMLWINDOW7_ONCONTEXTMENU: u32 = 71601u32;
pub const DISPID_IHTMLWINDOW7_ONDBLCLICK: u32 = 71545u32;
pub const DISPID_IHTMLWINDOW7_ONDRAG: u32 = 71585u32;
pub const DISPID_IHTMLWINDOW7_ONDRAGEND: u32 = 71586u32;
pub const DISPID_IHTMLWINDOW7_ONDRAGENTER: u32 = 71587u32;
pub const DISPID_IHTMLWINDOW7_ONDRAGLEAVE: u32 = 71589u32;
pub const DISPID_IHTMLWINDOW7_ONDRAGOVER: u32 = 71588u32;
pub const DISPID_IHTMLWINDOW7_ONDRAGSTART: u32 = 71571u32;
pub const DISPID_IHTMLWINDOW7_ONDROP: u32 = 71590u32;
pub const DISPID_IHTMLWINDOW7_ONDURATIONCHANGE: u32 = 71672u32;
pub const DISPID_IHTMLWINDOW7_ONEMPTIED: u32 = 71673u32;
pub const DISPID_IHTMLWINDOW7_ONENDED: u32 = 71674u32;
pub const DISPID_IHTMLWINDOW7_ONFOCUSIN: u32 = 71627u32;
pub const DISPID_IHTMLWINDOW7_ONFOCUSOUT: u32 = 71628u32;
pub const DISPID_IHTMLWINDOW7_ONINPUT: u32 = 71663u32;
pub const DISPID_IHTMLWINDOW7_ONKEYDOWN: u32 = 71541u32;
pub const DISPID_IHTMLWINDOW7_ONKEYPRESS: u32 = 71543u32;
pub const DISPID_IHTMLWINDOW7_ONKEYUP: u32 = 71542u32;
pub const DISPID_IHTMLWINDOW7_ONLOADEDDATA: u32 = 71675u32;
pub const DISPID_IHTMLWINDOW7_ONLOADEDMETADATA: u32 = 71676u32;
pub const DISPID_IHTMLWINDOW7_ONLOADSTART: u32 = 71677u32;
pub const DISPID_IHTMLWINDOW7_ONMOUSEDOWN: u32 = 71538u32;
pub const DISPID_IHTMLWINDOW7_ONMOUSEENTER: u32 = 71621u32;
pub const DISPID_IHTMLWINDOW7_ONMOUSELEAVE: u32 = 71622u32;
pub const DISPID_IHTMLWINDOW7_ONMOUSEMOVE: u32 = 71540u32;
pub const DISPID_IHTMLWINDOW7_ONMOUSEOUT: u32 = 71537u32;
pub const DISPID_IHTMLWINDOW7_ONMOUSEOVER: u32 = 71536u32;
pub const DISPID_IHTMLWINDOW7_ONMOUSEUP: u32 = 71539u32;
pub const DISPID_IHTMLWINDOW7_ONMOUSEWHEEL: u32 = 71612u32;
pub const DISPID_IHTMLWINDOW7_ONOFFLINE: u32 = 71644u32;
pub const DISPID_IHTMLWINDOW7_ONONLINE: u32 = 71643u32;
pub const DISPID_IHTMLWINDOW7_ONPAUSE: u32 = 71678u32;
pub const DISPID_IHTMLWINDOW7_ONPLAY: u32 = 71679u32;
pub const DISPID_IHTMLWINDOW7_ONPLAYING: u32 = 71680u32;
pub const DISPID_IHTMLWINDOW7_ONPROGRESS: u32 = 71681u32;
pub const DISPID_IHTMLWINDOW7_ONRATECHANGE: u32 = 71682u32;
pub const DISPID_IHTMLWINDOW7_ONREADYSTATECHANGE: u32 = 71561u32;
pub const DISPID_IHTMLWINDOW7_ONRESET: u32 = 71548u32;
pub const DISPID_IHTMLWINDOW7_ONSEEKED: u32 = 71683u32;
pub const DISPID_IHTMLWINDOW7_ONSEEKING: u32 = 71684u32;
pub const DISPID_IHTMLWINDOW7_ONSELECT: u32 = 71546u32;
pub const DISPID_IHTMLWINDOW7_ONSTALLED: u32 = 71685u32;
pub const DISPID_IHTMLWINDOW7_ONSTORAGE: u32 = 71636u32;
pub const DISPID_IHTMLWINDOW7_ONSUBMIT: u32 = 71547u32;
pub const DISPID_IHTMLWINDOW7_ONSUSPEND: u32 = 71686u32;
pub const DISPID_IHTMLWINDOW7_ONTIMEUPDATE: u32 = 71687u32;
pub const DISPID_IHTMLWINDOW7_ONVOLUMECHANGE: u32 = 71688u32;
pub const DISPID_IHTMLWINDOW7_ONWAITING: u32 = 71689u32;
pub const DISPID_IHTMLWINDOW7_OUTERHEIGHT: u32 = 1211u32;
pub const DISPID_IHTMLWINDOW7_OUTERWIDTH: u32 = 1210u32;
pub const DISPID_IHTMLWINDOW7_PAGEXOFFSET: u32 = 1206u32;
pub const DISPID_IHTMLWINDOW7_PAGEYOFFSET: u32 = 1207u32;
pub const DISPID_IHTMLWINDOW7_PERFORMANCE: u32 = 1203u32;
pub const DISPID_IHTMLWINDOW7_SCREENX: u32 = 1208u32;
pub const DISPID_IHTMLWINDOW7_SCREENY: u32 = 1209u32;
pub const DISPID_IHTMLWINDOW7_STYLEMEDIA: u32 = 1202u32;
pub const DISPID_IHTMLWINDOW8_APPLICATIONCACHE: u32 = 1213u32;
pub const DISPID_IHTMLWINDOW8_ONMSGESTURECHANGE: u32 = 71700u32;
pub const DISPID_IHTMLWINDOW8_ONMSGESTUREDOUBLETAP: u32 = 71704u32;
pub const DISPID_IHTMLWINDOW8_ONMSGESTUREEND: u32 = 71701u32;
pub const DISPID_IHTMLWINDOW8_ONMSGESTUREHOLD: u32 = 71702u32;
pub const DISPID_IHTMLWINDOW8_ONMSGESTURESTART: u32 = 71699u32;
pub const DISPID_IHTMLWINDOW8_ONMSGESTURETAP: u32 = 71703u32;
pub const DISPID_IHTMLWINDOW8_ONMSINERTIASTART: u32 = 71705u32;
pub const DISPID_IHTMLWINDOW8_ONMSPOINTERCANCEL: u32 = 71695u32;
pub const DISPID_IHTMLWINDOW8_ONMSPOINTERDOWN: u32 = 71690u32;
pub const DISPID_IHTMLWINDOW8_ONMSPOINTERHOVER: u32 = 71696u32;
pub const DISPID_IHTMLWINDOW8_ONMSPOINTERMOVE: u32 = 71691u32;
pub const DISPID_IHTMLWINDOW8_ONMSPOINTEROUT: u32 = 71694u32;
pub const DISPID_IHTMLWINDOW8_ONMSPOINTEROVER: u32 = 71693u32;
pub const DISPID_IHTMLWINDOW8_ONMSPOINTERUP: u32 = 71692u32;
pub const DISPID_IHTMLWINDOW8_ONPOPSTATE: u32 = 71728u32;
pub const DISPID_IHTMLXDOMAINREQUESTFACTORY_CREATE: u32 = 0u32;
pub const DISPID_IHTMLXDOMAINREQUEST_ABORT: u32 = 1010u32;
pub const DISPID_IHTMLXDOMAINREQUEST_CONTENTTYPE: u32 = 1005u32;
pub const DISPID_IHTMLXDOMAINREQUEST_ONERROR: u32 = 71565u32;
pub const DISPID_IHTMLXDOMAINREQUEST_ONLOAD: u32 = 71568u32;
pub const DISPID_IHTMLXDOMAINREQUEST_ONPROGRESS: u32 = 1006u32;
pub const DISPID_IHTMLXDOMAINREQUEST_ONTIMEOUT: u32 = 71648u32;
pub const DISPID_IHTMLXDOMAINREQUEST_OPEN: u32 = 1011u32;
pub const DISPID_IHTMLXDOMAINREQUEST_RESPONSETEXT: u32 = 1003u32;
pub const DISPID_IHTMLXDOMAINREQUEST_SEND: u32 = 1012u32;
pub const DISPID_IHTMLXDOMAINREQUEST_TIMEOUT: u32 = 1004u32;
pub const DISPID_IHTMLXMLHTTPREQUEST2_ONTIMEOUT: u32 = 71648u32;
pub const DISPID_IHTMLXMLHTTPREQUEST2_TIMEOUT: u32 = 1015u32;
pub const DISPID_IHTMLXMLHTTPREQUESTFACTORY_CREATE: u32 = 0u32;
pub const DISPID_IHTMLXMLHTTPREQUEST_ABORT: u32 = 1009u32;
pub const DISPID_IHTMLXMLHTTPREQUEST_GETALLRESPONSEHEADERS: u32 = 1012u32;
pub const DISPID_IHTMLXMLHTTPREQUEST_GETRESPONSEHEADER: u32 = 1013u32;
pub const DISPID_IHTMLXMLHTTPREQUEST_ONREADYSTATECHANGE: u32 = 71561u32;
pub const DISPID_IHTMLXMLHTTPREQUEST_OPEN: u32 = 1010u32;
pub const DISPID_IHTMLXMLHTTPREQUEST_READYSTATE: u32 = 1002u32;
pub const DISPID_IHTMLXMLHTTPREQUEST_RESPONSEBODY: u32 = 1003u32;
pub const DISPID_IHTMLXMLHTTPREQUEST_RESPONSETEXT: u32 = 1004u32;
pub const DISPID_IHTMLXMLHTTPREQUEST_RESPONSEXML: u32 = 1005u32;
pub const DISPID_IHTMLXMLHTTPREQUEST_SEND: u32 = 1011u32;
pub const DISPID_IHTMLXMLHTTPREQUEST_SETREQUESTHEADER: u32 = 1014u32;
pub const DISPID_IHTMLXMLHTTPREQUEST_STATUS: u32 = 1006u32;
pub const DISPID_IHTMLXMLHTTPREQUEST_STATUSTEXT: u32 = 1007u32;
pub const DISPID_ILINEINFO: u32 = 1000u32;
pub const DISPID_ILINEINFO_BASELINE: u32 = 1002u32;
pub const DISPID_ILINEINFO_LINEDIRECTION: u32 = 1005u32;
pub const DISPID_ILINEINFO_TEXTDESCENT: u32 = 1003u32;
pub const DISPID_ILINEINFO_TEXTHEIGHT: u32 = 1004u32;
pub const DISPID_ILINEINFO_X: u32 = 1001u32;
pub const DISPID_IMG: u32 = 2000u32;
pub const DISPID_IMGBASE: u32 = 1000u32;
pub const DISPID_IMPORT: u32 = 6u32;
pub const DISPID_IMPORTEXPORTFAVORITES: u32 = 9u32;
pub const DISPID_INAVIGATORDONOTTRACK_MSDONOTTRACK: u32 = 22u32;
pub const DISPID_INAVIGATORGEOLOCATION_GEOLOCATION: u32 = 21u32;
pub const DISPID_INITIALIZED: u32 = 4u32;
pub const DISPID_INPRIVATEFILTERINGENABLED: u32 = 37u32;
pub const DISPID_INPUT: u32 = 2000u32;
pub const DISPID_INPUTIMAGE: u32 = 2000u32;
pub const DISPID_INPUTTEXT: u32 = 4000u32;
pub const DISPID_INPUTTEXTBASE: u32 = 3000u32;
pub const DISPID_INTERNAL_ARIAATOMIC: u32 = 71179u32;
pub const DISPID_INTERNAL_ARIAAUTOCOMPLETE: u32 = 71180u32;
pub const DISPID_INTERNAL_ARIADROPEFFECT: u32 = 71181u32;
pub const DISPID_INTERNAL_ARIAGRABBED: u32 = 71182u32;
pub const DISPID_INTERNAL_ARIALABEL: u32 = 71183u32;
pub const DISPID_INTERNAL_ARIAMULTILINE: u32 = 71184u32;
pub const DISPID_INTERNAL_ARIAORIENTATION: u32 = 71185u32;
pub const DISPID_INTERNAL_ARIASORT: u32 = 71186u32;
pub const DISPID_INTERNAL_ARIAVALUETEXT: u32 = 71187u32;
pub const DISPID_INTERNAL_ARYELEMENTRELEASENOTIFYPTRCACHE: u32 = 70712u32;
pub const DISPID_INTERNAL_ARYOBJECTRELEASECLEANUPPTRCACHE: u32 = 70753u32;
pub const DISPID_INTERNAL_A_MS_HYPHENATE_LIMIT_AFTER: u32 = 71177u32;
pub const DISPID_INTERNAL_A_MS_HYPHENATE_LIMIT_BEFORE: u32 = 71176u32;
pub const DISPID_INTERNAL_A_MS_HYPHENATE_LIMIT_WORDS: u32 = 71175u32;
pub const DISPID_INTERNAL_BACKGROUNDDEFINITION: u32 = 71137u32;
pub const DISPID_INTERNAL_BGURLIMGCTXCACHEINDEX_GCAFTER: u32 = 70790u32;
pub const DISPID_INTERNAL_BGURLIMGCTXCACHEINDEX_GCBEFORE: u32 = 70789u32;
pub const DISPID_INTERNAL_BGURLIMGCTXCACHEINDEX_URLAFTER: u32 = 70792u32;
pub const DISPID_INTERNAL_BGURLIMGCTXCACHEINDEX_URLBEFORE: u32 = 70791u32;
pub const DISPID_INTERNAL_CATTRIBUTECOLLPTRCACHE: u32 = 70746u32;
pub const DISPID_INTERNAL_CATTRIBUTEPTRCACHE: u32 = 71169u32;
pub const DISPID_INTERNAL_CDOMCHILDRENPTRCACHE: u32 = 70662u32;
pub const DISPID_INTERNAL_CELEMENTCLASSCACHE: u32 = 2147483648u32;
pub const DISPID_INTERNAL_CODEPAGESETTINGSPTRCACHE: u32 = 70708u32;
pub const DISPID_INTERNAL_COMPUTEFORMATSTATECACHE: u32 = 70715u32;
pub const DISPID_INTERNAL_CRUNTIMESTYLEPTRCACHE: u32 = 70644u32;
pub const DISPID_INTERNAL_CSS_PARSEDARY: u32 = 71211u32;
pub const DISPID_INTERNAL_CSS_TRACEDSTYLES: u32 = 71213u32;
pub const DISPID_INTERNAL_CSTYLEPTRCACHE: u32 = 70643u32;
pub const DISPID_INTERNAL_DATABINDTASKPTRCACHE: u32 = 70710u32;
pub const DISPID_INTERNAL_DWNDOCPTRCACHE: u32 = 70709u32;
pub const DISPID_INTERNAL_DWNHEADERCACHE: u32 = 70733u32;
pub const DISPID_INTERNAL_DWNPOSTPTRCACHE: u32 = 70707u32;
pub const DISPID_INTERNAL_ERRORPAGEDWNPOST: u32 = 70997u32;
pub const DISPID_INTERNAL_ERRORPAGEREASON: u32 = 70996u32;
pub const DISPID_INTERNAL_ERRORPAGEREFRESHURL: u32 = 70995u32;
pub const DISPID_INTERNAL_FILTERNATIVEINFOPTRCACHE: u32 = 70692u32;
pub const DISPID_INTERNAL_FILTERPTRCACHE: u32 = 70649u32;
pub const DISPID_INTERNAL_FIRST: u32 = 71136u32;
pub const DISPID_INTERNAL_FOCUSITEMS: u32 = 70731u32;
pub const DISPID_INTERNAL_FONTFACEUNICODERANGE: u32 = 71170u32;
pub const DISPID_INTERNAL_FONTHISTORYINDEX: u32 = 70741u32;
pub const DISPID_INTERNAL_FRAMESCOLLECTION: u32 = 70736u32;
pub const DISPID_INTERNAL_GENERICCOMPLUSREF: u32 = 70730u32;
pub const DISPID_INTERNAL_GETTERSETTERCOLLECTION: u32 = 70794u32;
pub const DISPID_INTERNAL_INLINESTYLEAA: u32 = 70642u32;
pub const DISPID_INTERNAL_INVOKECONTEXT: u32 = 70645u32;
pub const DISPID_INTERNAL_INVOKECONTEXTDOCUMENT: u32 = 70748u32;
pub const DISPID_INTERNAL_LAYOUTRECTREGISTRYPTRCACHE: u32 = 70700u32;
pub const DISPID_INTERNAL_MEDIA_REFERENCE: u32 = 70729u32;
pub const DISPID_INTERNAL_NAMEDFLOWCOLLECTION: u32 = 71173u32;
pub const DISPID_INTERNAL_ONBEHAVIOR_APPLYSTYLE: u32 = 70684u32;
pub const DISPID_INTERNAL_ONBEHAVIOR_CONTENTREADY: u32 = 70660u32;
pub const DISPID_INTERNAL_ONBEHAVIOR_CONTENTSAVE: u32 = 70723u32;
pub const DISPID_INTERNAL_ONBEHAVIOR_DOCUMENTREADY: u32 = 70661u32;
pub const DISPID_INTERNAL_PAGEFLOWCOLLECTION: u32 = 71172u32;
pub const DISPID_INTERNAL_PEERFACTORYURLMAPPTRCACHE: u32 = 70713u32;
pub const DISPID_INTERNAL_REQUIRED: u32 = 71210u32;
pub const DISPID_INTERNAL_RUNTIMESTYLEAA: u32 = 70685u32;
pub const DISPID_INTERNAL_SOURCELOCATION: u32 = 71212u32;
pub const DISPID_INTERNAL_STMDIRTYPTRCACHE: u32 = 70714u32;
pub const DISPID_INTERNAL_TOUCHTARGETHANDLER: u32 = 71171u32;
pub const DISPID_INTERNAL_URIBEFOREREDIRECT: u32 = 70809u32;
pub const DISPID_INTERNAL_URLLOCATIONCACHE: u32 = 70711u32;
pub const DISPID_INTERNAL_URLSEARCHCACHE: u32 = 70743u32;
pub const DISPID_INVOKECONTEXTMENU: u32 = 8u32;
pub const DISPID_IOMHISTORY_BACK: u32 = 2u32;
pub const DISPID_IOMHISTORY_FORWARD: u32 = 3u32;
pub const DISPID_IOMHISTORY_GO: u32 = 4u32;
pub const DISPID_IOMHISTORY_LENGTH: u32 = 1u32;
pub const DISPID_IOMNAVIGATOR_APPCODENAME: u32 = 1u32;
pub const DISPID_IOMNAVIGATOR_APPMINORVERSION: u32 = 17u32;
pub const DISPID_IOMNAVIGATOR_APPNAME: u32 = 2u32;
pub const DISPID_IOMNAVIGATOR_APPVERSION: u32 = 3u32;
pub const DISPID_IOMNAVIGATOR_BROWSERLANGUAGE: u32 = 14u32;
pub const DISPID_IOMNAVIGATOR_CONNECTIONSPEED: u32 = 18u32;
pub const DISPID_IOMNAVIGATOR_COOKIEENABLED: u32 = 9u32;
pub const DISPID_IOMNAVIGATOR_CPUCLASS: u32 = 12u32;
pub const DISPID_IOMNAVIGATOR_JAVAENABLED: u32 = 5u32;
pub const DISPID_IOMNAVIGATOR_MIMETYPES: u32 = 7u32;
pub const DISPID_IOMNAVIGATOR_ONLINE: u32 = 19u32;
pub const DISPID_IOMNAVIGATOR_OPSPROFILE: u32 = 10u32;
pub const DISPID_IOMNAVIGATOR_PLATFORM: u32 = 16u32;
pub const DISPID_IOMNAVIGATOR_PLUGINS: u32 = 8u32;
pub const DISPID_IOMNAVIGATOR_SYSTEMLANGUAGE: u32 = 13u32;
pub const DISPID_IOMNAVIGATOR_TAINTENABLED: u32 = 6u32;
pub const DISPID_IOMNAVIGATOR_TOSTRING: u32 = 11u32;
pub const DISPID_IOMNAVIGATOR_USERAGENT: u32 = 4u32;
pub const DISPID_IOMNAVIGATOR_USERLANGUAGE: u32 = 15u32;
pub const DISPID_IOMNAVIGATOR_USERPROFILE: u32 = 20u32;
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER2_PERCENTSCALE: u32 = 509u32;
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER2_SHOWHEADERFOOTER: u32 = 507u32;
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER2_SHRINKTOFIT: u32 = 508u32;
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER_DRAWPREVIEWPAGE: u32 = 502u32;
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER_ENDPRINT: u32 = 506u32;
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER_GETPRINTTASKOPTIONVALUE: u32 = 505u32;
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER_INVALIDATEPREVIEW: u32 = 504u32;
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER_SETPAGECOUNT: u32 = 503u32;
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER_STARTPRINT: u32 = 501u32;
pub const DISPID_IRANGEEXCEPTION_CODE: u32 = 1000u32;
pub const DISPID_IRANGEEXCEPTION_MESSAGE: u32 = 1001u32;
pub const DISPID_IRULESAPPLIEDCOLLECTION_ELEMENT: u32 = 1002u32;
pub const DISPID_IRULESAPPLIEDCOLLECTION_ITEM: u32 = 0u32;
pub const DISPID_IRULESAPPLIEDCOLLECTION_LENGTH: u32 = 1001u32;
pub const DISPID_IRULESAPPLIEDCOLLECTION_PROPERTY: u32 = 1005u32;
pub const DISPID_IRULESAPPLIEDCOLLECTION_PROPERTYCOUNT: u32 = 1004u32;
pub const DISPID_IRULESAPPLIEDCOLLECTION_PROPERTYINHERITEDFROM: u32 = 1003u32;
pub const DISPID_IRULESAPPLIEDCOLLECTION_PROPERTYINHERITEDTRACE: u32 = 1006u32;
pub const DISPID_IRULESAPPLIEDCOLLECTION_PROPERTYINHERITEDTRACELENGTH: u32 = 1007u32;
pub const DISPID_IRULESAPPLIED_APPLIEDRULES: u32 = 1003u32;
pub const DISPID_IRULESAPPLIED_ELEMENT: u32 = 1001u32;
pub const DISPID_IRULESAPPLIED_HASINHERITABLEPROPERTY: u32 = 1006u32;
pub const DISPID_IRULESAPPLIED_INLINESTYLES: u32 = 1002u32;
pub const DISPID_IRULESAPPLIED_PROPERTYISINHERITABLE: u32 = 1005u32;
pub const DISPID_IRULESAPPLIED_PROPERTYISINLINE: u32 = 1004u32;
pub const DISPID_ISMETAREFERRERAVAILABLE: u32 = 83u32;
pub const DISPID_ISSEARCHMIGRATED: u32 = 25u32;
pub const DISPID_ISSEARCHPROVIDERINSTALLED: u32 = 24u32;
pub const DISPID_ISSERVICEINSTALLED: u32 = 31u32;
pub const DISPID_ISSITEMODE: u32 = 43u32;
pub const DISPID_ISSITEMODEFIRSTRUN: u32 = 59u32;
pub const DISPID_ISSUBSCRIBED: u32 = 7u32;
pub const DISPID_ISVGAELEMENT_TARGET: u32 = 1052u32;
pub const DISPID_ISVGANGLE_CONVERTTOSPECIFIEDUNITS: u32 = 1005u32;
pub const DISPID_ISVGANGLE_NEWVALUESPECIFIEDUNITS: u32 = 1004u32;
pub const DISPID_ISVGANGLE_UNITTYPE: u32 = 1000u32;
pub const DISPID_ISVGANGLE_VALUE: u32 = 1001u32;
pub const DISPID_ISVGANGLE_VALUEASSTRING: u32 = 1003u32;
pub const DISPID_ISVGANGLE_VALUEINSPECIFIEDUNITS: u32 = 1002u32;
pub const DISPID_ISVGANIMATEDANGLE_ANIMVAL: u32 = 1001u32;
pub const DISPID_ISVGANIMATEDANGLE_BASEVAL: u32 = 1000u32;
pub const DISPID_ISVGANIMATEDBOOLEAN_ANIMVAL: u32 = 1001u32;
pub const DISPID_ISVGANIMATEDBOOLEAN_BASEVAL: u32 = 1000u32;
pub const DISPID_ISVGANIMATEDENUMERATION_ANIMVAL: u32 = 1001u32;
pub const DISPID_ISVGANIMATEDENUMERATION_BASEVAL: u32 = 1000u32;
pub const DISPID_ISVGANIMATEDINTEGER_ANIMVAL: u32 = 1001u32;
pub const DISPID_ISVGANIMATEDINTEGER_BASEVAL: u32 = 1000u32;
pub const DISPID_ISVGANIMATEDLENGTHLIST_ANIMVAL: u32 = 1001u32;
pub const DISPID_ISVGANIMATEDLENGTHLIST_BASEVAL: u32 = 1000u32;
pub const DISPID_ISVGANIMATEDLENGTH_ANIMVAL: u32 = 1001u32;
pub const DISPID_ISVGANIMATEDLENGTH_BASEVAL: u32 = 1000u32;
pub const DISPID_ISVGANIMATEDNUMBERLIST_ANIMVAL: u32 = 1001u32;
pub const DISPID_ISVGANIMATEDNUMBERLIST_BASEVAL: u32 = 1000u32;
pub const DISPID_ISVGANIMATEDNUMBER_ANIMVAL: u32 = 1001u32;
pub const DISPID_ISVGANIMATEDNUMBER_BASEVAL: u32 = 1000u32;
pub const DISPID_ISVGANIMATEDPATHDATA_ANIMATEDNORMALIZEDPATHSEGLIST: u32 = 1078u32;
pub const DISPID_ISVGANIMATEDPATHDATA_ANIMATEDPATHSEGLIST: u32 = 1077u32;
pub const DISPID_ISVGANIMATEDPATHDATA_NORMALIZEDPATHSEGLIST: u32 = 1076u32;
pub const DISPID_ISVGANIMATEDPATHDATA_PATHSEGLIST: u32 = 1052u32;
pub const DISPID_ISVGANIMATEDPOINTS_ANIMATEDPOINTS: u32 = 1052u32;
pub const DISPID_ISVGANIMATEDPOINTS_POINTS: u32 = 1050u32;
pub const DISPID_ISVGANIMATEDPRESERVEASPECTRATIO_ANIMVAL: u32 = 1001u32;
pub const DISPID_ISVGANIMATEDPRESERVEASPECTRATIO_BASEVAL: u32 = 1000u32;
pub const DISPID_ISVGANIMATEDRECT_ANIMVAL: u32 = 1001u32;
pub const DISPID_ISVGANIMATEDRECT_BASEVAL: u32 = 1000u32;
pub const DISPID_ISVGANIMATEDSTRING_ANIMVAL: u32 = 1001u32;
pub const DISPID_ISVGANIMATEDSTRING_BASEVAL: u32 = 1000u32;
pub const DISPID_ISVGANIMATEDTRANSFORMLIST_ANIMVAL: u32 = 1001u32;
pub const DISPID_ISVGANIMATEDTRANSFORMLIST_BASEVAL: u32 = 1000u32;
pub const DISPID_ISVGCIRCLEELEMENT_CX: u32 = 1052u32;
pub const DISPID_ISVGCIRCLEELEMENT_CY: u32 = 1054u32;
pub const DISPID_ISVGCIRCLEELEMENT_R: u32 = 1056u32;
pub const DISPID_ISVGCLIPPATHELEMENT_CLIPPATHUNITS: u32 = 1051u32;
pub const DISPID_ISVGDOCUMENT_ROOTELEMENT: u32 = 1116u32;
pub const DISPID_ISVGELEMENTINSTANCELIST_ITEM: u32 = 1001u32;
pub const DISPID_ISVGELEMENTINSTANCELIST_LENGTH: u32 = 1000u32;
pub const DISPID_ISVGELEMENTINSTANCE_CHILDNODES: u32 = 1003u32;
pub const DISPID_ISVGELEMENTINSTANCE_CORRESPONDINGELEMENT: u32 = 1000u32;
pub const DISPID_ISVGELEMENTINSTANCE_CORRESPONDINGUSEELEMENT: u32 = 1001u32;
pub const DISPID_ISVGELEMENTINSTANCE_FIRSTCHILD: u32 = 1004u32;
pub const DISPID_ISVGELEMENTINSTANCE_LASTCHILD: u32 = 1005u32;
pub const DISPID_ISVGELEMENTINSTANCE_NEXTSIBLING: u32 = 1007u32;
pub const DISPID_ISVGELEMENTINSTANCE_PARENTNODE: u32 = 1002u32;
pub const DISPID_ISVGELEMENTINSTANCE_PREVIOUSSIBLING: u32 = 1006u32;
pub const DISPID_ISVGELEMENT_FOCUSABLE: u32 = 1036u32;
pub const DISPID_ISVGELEMENT_OWNERSVGELEMENT: u32 = 1033u32;
pub const DISPID_ISVGELEMENT_VIEWPORTELEMENT: u32 = 1034u32;
pub const DISPID_ISVGELEMENT_XMLBASE: u32 = 1032u32;
pub const DISPID_ISVGELLIPSEELEMENT_CX: u32 = 1052u32;
pub const DISPID_ISVGELLIPSEELEMENT_CY: u32 = 1054u32;
pub const DISPID_ISVGELLIPSEELEMENT_RX: u32 = 1056u32;
pub const DISPID_ISVGELLIPSEELEMENT_RY: u32 = 1058u32;
pub const DISPID_ISVGEXCEPTION_CODE: u32 = 1000u32;
pub const DISPID_ISVGEXCEPTION_MESSAGE: u32 = 1001u32;
pub const DISPID_ISVGEXTERNALRESOURCESREQUIRED_EXTERNALRESOURCESREQUIRED: u32 = 1020u32;
pub const DISPID_ISVGFITTOVIEWBOX_PRESERVEASPECTRATIO: u32 = 1024u32;
pub const DISPID_ISVGFITTOVIEWBOX_VIEWBOX: u32 = 1022u32;
pub const DISPID_ISVGGRADIENTELEMENT_GRADIENTTRANSFORM: u32 = 1053u32;
pub const DISPID_ISVGGRADIENTELEMENT_GRADIENTUNITS: u32 = 1051u32;
pub const DISPID_ISVGGRADIENTELEMENT_SPREADMETHOD: u32 = 1055u32;
pub const DISPID_ISVGIMAGEELEMENT_HEIGHT: u32 = 1057u32;
pub const DISPID_ISVGIMAGEELEMENT_WIDTH: u32 = 1055u32;
pub const DISPID_ISVGIMAGEELEMENT_X: u32 = 1051u32;
pub const DISPID_ISVGIMAGEELEMENT_Y: u32 = 1053u32;
pub const DISPID_ISVGLANGSPACE_XMLLANG: u32 = 1017u32;
pub const DISPID_ISVGLANGSPACE_XMLSPACE: u32 = 1018u32;
pub const DISPID_ISVGLENGTHLIST_APPENDITEM: u32 = 1007u32;
pub const DISPID_ISVGLENGTHLIST_CLEAR: u32 = 1001u32;
pub const DISPID_ISVGLENGTHLIST_GETITEM: u32 = 1003u32;
pub const DISPID_ISVGLENGTHLIST_INITIALIZE: u32 = 1002u32;
pub const DISPID_ISVGLENGTHLIST_INSERTITEMBEFORE: u32 = 1004u32;
pub const DISPID_ISVGLENGTHLIST_NUMBEROFITEMS: u32 = 1000u32;
pub const DISPID_ISVGLENGTHLIST_REMOVEITEM: u32 = 1006u32;
pub const DISPID_ISVGLENGTHLIST_REPLACEITEM: u32 = 1005u32;
pub const DISPID_ISVGLENGTH_CONVERTTOSPECIFIEDUNITS: u32 = 1005u32;
pub const DISPID_ISVGLENGTH_NEWVALUESPECIFIEDUNITS: u32 = 1004u32;
pub const DISPID_ISVGLENGTH_UNITTYPE: u32 = 1000u32;
pub const DISPID_ISVGLENGTH_VALUE: u32 = 1001u32;
pub const DISPID_ISVGLENGTH_VALUEASSTRING: u32 = 1003u32;
pub const DISPID_ISVGLENGTH_VALUEINSPECIFIEDUNITS: u32 = 1002u32;
pub const DISPID_ISVGLINEARGRADIENTELEMENT_X1: u32 = 1071u32;
pub const DISPID_ISVGLINEARGRADIENTELEMENT_X2: u32 = 1075u32;
pub const DISPID_ISVGLINEARGRADIENTELEMENT_Y1: u32 = 1073u32;
pub const DISPID_ISVGLINEARGRADIENTELEMENT_Y2: u32 = 1077u32;
pub const DISPID_ISVGLINEELEMENT_X1: u32 = 1052u32;
pub const DISPID_ISVGLINEELEMENT_X2: u32 = 1056u32;
pub const DISPID_ISVGLINEELEMENT_Y1: u32 = 1054u32;
pub const DISPID_ISVGLINEELEMENT_Y2: u32 = 1058u32;
pub const DISPID_ISVGLOCATABLE_FARTHESTVIEWPORTELEMENT: u32 = 1003u32;
pub const DISPID_ISVGLOCATABLE_GETBBOX: u32 = 1004u32;
pub const DISPID_ISVGLOCATABLE_GETCTM: u32 = 1005u32;
pub const DISPID_ISVGLOCATABLE_GETSCREENCTM: u32 = 1006u32;
pub const DISPID_ISVGLOCATABLE_GETTRANSFORMTOELEMENT: u32 = 1007u32;
pub const DISPID_ISVGLOCATABLE_NEARESTVIEWPORTELEMENT: u32 = 1002u32;
pub const DISPID_ISVGMARKERELEMENT_MARKERHEIGHT: u32 = 1059u32;
pub const DISPID_ISVGMARKERELEMENT_MARKERUNITS: u32 = 1055u32;
pub const DISPID_ISVGMARKERELEMENT_MARKERWIDTH: u32 = 1057u32;
pub const DISPID_ISVGMARKERELEMENT_ORIENTANGLE: u32 = 1062u32;
pub const DISPID_ISVGMARKERELEMENT_ORIENTTYPE: u32 = 1061u32;
pub const DISPID_ISVGMARKERELEMENT_REFX: u32 = 1051u32;
pub const DISPID_ISVGMARKERELEMENT_REFY: u32 = 1053u32;
pub const DISPID_ISVGMARKERELEMENT_SETORIENTTOANGLE: u32 = 1064u32;
pub const DISPID_ISVGMARKERELEMENT_SETORIENTTOAUTO: u32 = 1063u32;
pub const DISPID_ISVGMASKELEMENT_HEIGHT: u32 = 1061u32;
pub const DISPID_ISVGMASKELEMENT_MASKCONTENTUNITS: u32 = 1053u32;
pub const DISPID_ISVGMASKELEMENT_MASKUNITS: u32 = 1051u32;
pub const DISPID_ISVGMASKELEMENT_WIDTH: u32 = 1059u32;
pub const DISPID_ISVGMASKELEMENT_X: u32 = 1055u32;
pub const DISPID_ISVGMASKELEMENT_Y: u32 = 1057u32;
pub const DISPID_ISVGMATRIX_A: u32 = 1000u32;
pub const DISPID_ISVGMATRIX_B: u32 = 1001u32;
pub const DISPID_ISVGMATRIX_C: u32 = 1002u32;
pub const DISPID_ISVGMATRIX_D: u32 = 1003u32;
pub const DISPID_ISVGMATRIX_E: u32 = 1004u32;
pub const DISPID_ISVGMATRIX_F: u32 = 1005u32;
pub const DISPID_ISVGMATRIX_FLIPX: u32 = 1013u32;
pub const DISPID_ISVGMATRIX_FLIPY: u32 = 1014u32;
pub const DISPID_ISVGMATRIX_INVERSE: u32 = 1007u32;
pub const DISPID_ISVGMATRIX_MULTIPLY: u32 = 1006u32;
pub const DISPID_ISVGMATRIX_ROTATE: u32 = 1011u32;
pub const DISPID_ISVGMATRIX_ROTATEFROMVECTOR: u32 = 1012u32;
pub const DISPID_ISVGMATRIX_SCALE: u32 = 1009u32;
pub const DISPID_ISVGMATRIX_SCALENONUNIFORM: u32 = 1010u32;
pub const DISPID_ISVGMATRIX_SKEWX: u32 = 1015u32;
pub const DISPID_ISVGMATRIX_SKEWY: u32 = 1016u32;
pub const DISPID_ISVGMATRIX_TRANSLATE: u32 = 1008u32;
pub const DISPID_ISVGNUMBERLIST_APPENDITEM: u32 = 1007u32;
pub const DISPID_ISVGNUMBERLIST_CLEAR: u32 = 1001u32;
pub const DISPID_ISVGNUMBERLIST_GETITEM: u32 = 1003u32;
pub const DISPID_ISVGNUMBERLIST_INITIALIZE: u32 = 1002u32;
pub const DISPID_ISVGNUMBERLIST_INSERTITEMBEFORE: u32 = 1004u32;
pub const DISPID_ISVGNUMBERLIST_NUMBEROFITEMS: u32 = 1000u32;
pub const DISPID_ISVGNUMBERLIST_REMOVEITEM: u32 = 1006u32;
pub const DISPID_ISVGNUMBERLIST_REPLACEITEM: u32 = 1005u32;
pub const DISPID_ISVGNUMBER_VALUE: u32 = 1000u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGARCABS: u32 = 1063u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGARCREL: u32 = 1064u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCLOSEPATH: u32 = 1054u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCURVETOCUBICABS: u32 = 1059u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCURVETOCUBICREL: u32 = 1060u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCURVETOCUBICSMOOTHABS: u32 = 1069u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCURVETOCUBICSMOOTHREL: u32 = 1070u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCURVETOQUADRATICABS: u32 = 1061u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCURVETOQUADRATICREL: u32 = 1062u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCURVETOQUADRATICSMOOTHABS: u32 = 1071u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCURVETOQUADRATICSMOOTHREL: u32 = 1072u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGLINETOABS: u32 = 1057u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGLINETOHORIZONTALABS: u32 = 1065u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGLINETOHORIZONTALREL: u32 = 1066u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGLINETOREL: u32 = 1058u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGLINETOVERTICALABS: u32 = 1067u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGLINETOVERTICALREL: u32 = 1068u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGMOVETOABS: u32 = 1055u32;
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGMOVETOREL: u32 = 1056u32;
pub const DISPID_ISVGPATHELEMENT_GETPATHSEGATLENGTH: u32 = 1075u32;
pub const DISPID_ISVGPATHELEMENT_GETPOINTATLENGTH: u32 = 1074u32;
pub const DISPID_ISVGPATHELEMENT_GETTOTALLENGTH: u32 = 1073u32;
pub const DISPID_ISVGPATHELEMENT_PATHLENGTH: u32 = 1053u32;
pub const DISPID_ISVGPATHSEGARCABS_ANGLE: u32 = 1024u32;
pub const DISPID_ISVGPATHSEGARCABS_LARGEARCFLAG: u32 = 1025u32;
pub const DISPID_ISVGPATHSEGARCABS_R1: u32 = 1022u32;
pub const DISPID_ISVGPATHSEGARCABS_R2: u32 = 1023u32;
pub const DISPID_ISVGPATHSEGARCABS_SWEEPFLAG: u32 = 1026u32;
pub const DISPID_ISVGPATHSEGARCABS_X: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGARCABS_Y: u32 = 1021u32;
pub const DISPID_ISVGPATHSEGARCREL_ANGLE: u32 = 1024u32;
pub const DISPID_ISVGPATHSEGARCREL_LARGEARCFLAG: u32 = 1025u32;
pub const DISPID_ISVGPATHSEGARCREL_R1: u32 = 1022u32;
pub const DISPID_ISVGPATHSEGARCREL_R2: u32 = 1023u32;
pub const DISPID_ISVGPATHSEGARCREL_SWEEPFLAG: u32 = 1026u32;
pub const DISPID_ISVGPATHSEGARCREL_X: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGARCREL_Y: u32 = 1021u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICABS_X: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICABS_X1: u32 = 1022u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICABS_X2: u32 = 1024u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICABS_Y: u32 = 1021u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICABS_Y1: u32 = 1023u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICABS_Y2: u32 = 1025u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICREL_X: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICREL_X1: u32 = 1022u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICREL_X2: u32 = 1024u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICREL_Y: u32 = 1021u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICREL_Y1: u32 = 1023u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICREL_Y2: u32 = 1025u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICSMOOTHABS_X: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICSMOOTHABS_X2: u32 = 1022u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICSMOOTHABS_Y: u32 = 1021u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICSMOOTHABS_Y2: u32 = 1023u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICSMOOTHREL_X: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICSMOOTHREL_X2: u32 = 1022u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICSMOOTHREL_Y: u32 = 1021u32;
pub const DISPID_ISVGPATHSEGCURVETOCUBICSMOOTHREL_Y2: u32 = 1023u32;
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICABS_X: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICABS_X1: u32 = 1022u32;
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICABS_Y: u32 = 1021u32;
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICABS_Y1: u32 = 1023u32;
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICREL_X: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICREL_X1: u32 = 1022u32;
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICREL_Y: u32 = 1021u32;
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICREL_Y1: u32 = 1023u32;
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICSMOOTHABS_X: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICSMOOTHABS_Y: u32 = 1021u32;
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICSMOOTHREL_X: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICSMOOTHREL_Y: u32 = 1021u32;
pub const DISPID_ISVGPATHSEGLINETOABS_X: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGLINETOABS_Y: u32 = 1021u32;
pub const DISPID_ISVGPATHSEGLINETOHORIZONTALABS_X: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGLINETOHORIZONTALREL_X: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGLINETOREL_X: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGLINETOREL_Y: u32 = 1021u32;
pub const DISPID_ISVGPATHSEGLINETOVERTICALABS_Y: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGLINETOVERTICALREL_Y: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGLIST_APPENDITEM: u32 = 1007u32;
pub const DISPID_ISVGPATHSEGLIST_CLEAR: u32 = 1001u32;
pub const DISPID_ISVGPATHSEGLIST_GETITEM: u32 = 1003u32;
pub const DISPID_ISVGPATHSEGLIST_INITIALIZE: u32 = 1002u32;
pub const DISPID_ISVGPATHSEGLIST_INSERTITEMBEFORE: u32 = 1004u32;
pub const DISPID_ISVGPATHSEGLIST_NUMBEROFITEMS: u32 = 1000u32;
pub const DISPID_ISVGPATHSEGLIST_REMOVEITEM: u32 = 1006u32;
pub const DISPID_ISVGPATHSEGLIST_REPLACEITEM: u32 = 1005u32;
pub const DISPID_ISVGPATHSEGMOVETOABS_X: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGMOVETOABS_Y: u32 = 1021u32;
pub const DISPID_ISVGPATHSEGMOVETOREL_X: u32 = 1020u32;
pub const DISPID_ISVGPATHSEGMOVETOREL_Y: u32 = 1021u32;
pub const DISPID_ISVGPATHSEG_PATHSEGTYPE: u32 = 1000u32;
pub const DISPID_ISVGPATHSEG_PATHSEGTYPEASLETTER: u32 = 1001u32;
pub const DISPID_ISVGPATTERNELEMENT_HEIGHT: u32 = 1063u32;
pub const DISPID_ISVGPATTERNELEMENT_PATTERNCONTENTUNITS: u32 = 1053u32;
pub const DISPID_ISVGPATTERNELEMENT_PATTERNTRANSFORM: u32 = 1055u32;
pub const DISPID_ISVGPATTERNELEMENT_PATTERNUNITS: u32 = 1051u32;
pub const DISPID_ISVGPATTERNELEMENT_WIDTH: u32 = 1061u32;
pub const DISPID_ISVGPATTERNELEMENT_X: u32 = 1057u32;
pub const DISPID_ISVGPATTERNELEMENT_Y: u32 = 1059u32;
pub const DISPID_ISVGPOINTLIST_APPENDITEM: u32 = 1007u32;
pub const DISPID_ISVGPOINTLIST_CLEAR: u32 = 1001u32;
pub const DISPID_ISVGPOINTLIST_GETITEM: u32 = 1003u32;
pub const DISPID_ISVGPOINTLIST_INITIALIZE: u32 = 1002u32;
pub const DISPID_ISVGPOINTLIST_INSERTITEMBEFORE: u32 = 1004u32;
pub const DISPID_ISVGPOINTLIST_NUMBEROFITEMS: u32 = 1000u32;
pub const DISPID_ISVGPOINTLIST_REMOVEITEM: u32 = 1006u32;
pub const DISPID_ISVGPOINTLIST_REPLACEITEM: u32 = 1005u32;
pub const DISPID_ISVGPOINT_MATRIXTRANSFORM: u32 = 1002u32;
pub const DISPID_ISVGPOINT_X: u32 = 1000u32;
pub const DISPID_ISVGPOINT_Y: u32 = 1001u32;
pub const DISPID_ISVGPRESERVEASPECTRATIO_ALIGN: u32 = 1000u32;
pub const DISPID_ISVGPRESERVEASPECTRATIO_MEETORSLICE: u32 = 1001u32;
pub const DISPID_ISVGRADIALGRADIENTELEMENT_CX: u32 = 1071u32;
pub const DISPID_ISVGRADIALGRADIENTELEMENT_CY: u32 = 1073u32;
pub const DISPID_ISVGRADIALGRADIENTELEMENT_FX: u32 = 1077u32;
pub const DISPID_ISVGRADIALGRADIENTELEMENT_FY: u32 = 1079u32;
pub const DISPID_ISVGRADIALGRADIENTELEMENT_R: u32 = 1075u32;
pub const DISPID_ISVGRECTELEMENT_HEIGHT: u32 = 1058u32;
pub const DISPID_ISVGRECTELEMENT_RX: u32 = 1060u32;
pub const DISPID_ISVGRECTELEMENT_RY: u32 = 1062u32;
pub const DISPID_ISVGRECTELEMENT_WIDTH: u32 = 1056u32;
pub const DISPID_ISVGRECTELEMENT_X: u32 = 1052u32;
pub const DISPID_ISVGRECTELEMENT_Y: u32 = 1054u32;
pub const DISPID_ISVGRECT_HEIGHT: u32 = 1003u32;
pub const DISPID_ISVGRECT_WIDTH: u32 = 1002u32;
pub const DISPID_ISVGRECT_X: u32 = 1000u32;
pub const DISPID_ISVGRECT_Y: u32 = 1001u32;
pub const DISPID_ISVGSCRIPTELEMENT_TYPE: u32 = 1052u32;
pub const DISPID_ISVGSTOPELEMENT_OFFSET: u32 = 1051u32;
pub const DISPID_ISVGSTRINGLIST_APPENDITEM: u32 = 1007u32;
pub const DISPID_ISVGSTRINGLIST_CLEAR: u32 = 1001u32;
pub const DISPID_ISVGSTRINGLIST_GETITEM: u32 = 1003u32;
pub const DISPID_ISVGSTRINGLIST_INITIALIZE: u32 = 1002u32;
pub const DISPID_ISVGSTRINGLIST_INSERTITEMBEFORE: u32 = 1004u32;
pub const DISPID_ISVGSTRINGLIST_NUMBEROFITEMS: u32 = 1000u32;
pub const DISPID_ISVGSTRINGLIST_REMOVEITEM: u32 = 1006u32;
pub const DISPID_ISVGSTRINGLIST_REPLACEITEM: u32 = 1005u32;
pub const DISPID_ISVGSTYLABLE_CLASSNAME: u32 = 1001u32;
pub const DISPID_ISVGSTYLEELEMENT_MEDIA: u32 = 1052u32;
pub const DISPID_ISVGSTYLEELEMENT_TYPE: u32 = 1051u32;
pub const DISPID_ISVGSVGELEMENT_ANIMATIONSPAUSED: u32 = 1076u32;
pub const DISPID_ISVGSVGELEMENT_CHECKENCLOSURE: u32 = 1082u32;
pub const DISPID_ISVGSVGELEMENT_CHECKINTERSECTION: u32 = 1081u32;
pub const DISPID_ISVGSVGELEMENT_CONTENTSCRIPTTYPE: u32 = 1059u32;
pub const DISPID_ISVGSVGELEMENT_CONTENTSTYLETYPE: u32 = 1060u32;
pub const DISPID_ISVGSVGELEMENT_CREATESVGANGLE: u32 = 1086u32;
pub const DISPID_ISVGSVGELEMENT_CREATESVGLENGTH: u32 = 1085u32;
pub const DISPID_ISVGSVGELEMENT_CREATESVGMATRIX: u32 = 1088u32;
pub const DISPID_ISVGSVGELEMENT_CREATESVGNUMBER: u32 = 1084u32;
pub const DISPID_ISVGSVGELEMENT_CREATESVGPOINT: u32 = 1087u32;
pub const DISPID_ISVGSVGELEMENT_CREATESVGRECT: u32 = 1089u32;
pub const DISPID_ISVGSVGELEMENT_CREATESVGTRANSFORM: u32 = 1090u32;
pub const DISPID_ISVGSVGELEMENT_CREATESVGTRANSFORMFROMMATRIX: u32 = 1091u32;
pub const DISPID_ISVGSVGELEMENT_CURRENTSCALE: u32 = 1068u32;
pub const DISPID_ISVGSVGELEMENT_CURRENTTRANSLATE: u32 = 1069u32;
pub const DISPID_ISVGSVGELEMENT_CURRENTVIEW: u32 = 1067u32;
pub const DISPID_ISVGSVGELEMENT_DESELECTALL: u32 = 1083u32;
pub const DISPID_ISVGSVGELEMENT_FORCEREDRAW: u32 = 1073u32;
pub const DISPID_ISVGSVGELEMENT_GETCURRENTTIME: u32 = 1077u32;
pub const DISPID_ISVGSVGELEMENT_GETELEMENTBYID: u32 = 1092u32;
pub const DISPID_ISVGSVGELEMENT_GETENCLOSURELIST: u32 = 1080u32;
pub const DISPID_ISVGSVGELEMENT_GETINTERSECTIONLIST: u32 = 1079u32;
pub const DISPID_ISVGSVGELEMENT_HEIGHT: u32 = 1058u32;
pub const DISPID_ISVGSVGELEMENT_PAUSEANIMATIONS: u32 = 1074u32;
pub const DISPID_ISVGSVGELEMENT_PIXELUNITTOMILLIMETERX: u32 = 1062u32;
pub const DISPID_ISVGSVGELEMENT_PIXELUNITTOMILLIMETERY: u32 = 1063u32;
pub const DISPID_ISVGSVGELEMENT_SCREENPIXELTOMILLIMETERX: u32 = 1064u32;
pub const DISPID_ISVGSVGELEMENT_SCREENPIXELTOMILLIMETERY: u32 = 1065u32;
pub const DISPID_ISVGSVGELEMENT_SETCURRENTTIME: u32 = 1078u32;
pub const DISPID_ISVGSVGELEMENT_SUSPENDREDRAW: u32 = 1070u32;
pub const DISPID_ISVGSVGELEMENT_UNPAUSEANIMATIONS: u32 = 1075u32;
pub const DISPID_ISVGSVGELEMENT_UNSUSPENDREDRAW: u32 = 1071u32;
pub const DISPID_ISVGSVGELEMENT_UNSUSPENDREDRAWALL: u32 = 1072u32;
pub const DISPID_ISVGSVGELEMENT_USECURRENTVIEW: u32 = 1066u32;
pub const DISPID_ISVGSVGELEMENT_VIEWPORT: u32 = 1061u32;
pub const DISPID_ISVGSVGELEMENT_WIDTH: u32 = 1056u32;
pub const DISPID_ISVGSVGELEMENT_X: u32 = 1052u32;
pub const DISPID_ISVGSVGELEMENT_Y: u32 = 1054u32;
pub const DISPID_ISVGTESTS_HASEXTENSION: u32 = 1016u32;
pub const DISPID_ISVGTESTS_REQUIREDEXTENSIONS: u32 = 1013u32;
pub const DISPID_ISVGTESTS_REQUIREDFEATURES: u32 = 1011u32;
pub const DISPID_ISVGTESTS_SYSTEMLANGUAGE: u32 = 1015u32;
pub const DISPID_ISVGTEXTCONTENTELEMENT_GETCHARNUMATPOSITION: u32 = 1061u32;
pub const DISPID_ISVGTEXTCONTENTELEMENT_GETCOMPUTEDTEXTLENGTH: u32 = 1055u32;
pub const DISPID_ISVGTEXTCONTENTELEMENT_GETENDPOSITIONOFCHAR: u32 = 1058u32;
pub const DISPID_ISVGTEXTCONTENTELEMENT_GETEXTENTOFCHAR: u32 = 1059u32;
pub const DISPID_ISVGTEXTCONTENTELEMENT_GETNUMBEROFCHARS: u32 = 1054u32;
pub const DISPID_ISVGTEXTCONTENTELEMENT_GETROTATIONOFCHAR: u32 = 1060u32;
pub const DISPID_ISVGTEXTCONTENTELEMENT_GETSTARTPOSITIONOFCHAR: u32 = 1057u32;
pub const DISPID_ISVGTEXTCONTENTELEMENT_GETSUBSTRINGLENGTH: u32 = 1056u32;
pub const DISPID_ISVGTEXTCONTENTELEMENT_LENGTHADJUST: u32 = 1051u32;
pub const DISPID_ISVGTEXTCONTENTELEMENT_SELECTSUBSTRING: u32 = 1062u32;
pub const DISPID_ISVGTEXTCONTENTELEMENT_TEXTLENGTH: u32 = 1053u32;
pub const DISPID_ISVGTEXTPATHELEMENT_METHOD: u32 = 1073u32;
pub const DISPID_ISVGTEXTPATHELEMENT_SPACING: u32 = 1075u32;
pub const DISPID_ISVGTEXTPATHELEMENT_STARTOFFSET: u32 = 1071u32;
pub const DISPID_ISVGTEXTPOSITIONINGELEMENT_DX: u32 = 1075u32;
pub const DISPID_ISVGTEXTPOSITIONINGELEMENT_DY: u32 = 1077u32;
pub const DISPID_ISVGTEXTPOSITIONINGELEMENT_ROTATE: u32 = 1079u32;
pub const DISPID_ISVGTEXTPOSITIONINGELEMENT_X: u32 = 1071u32;
pub const DISPID_ISVGTEXTPOSITIONINGELEMENT_Y: u32 = 1073u32;
pub const DISPID_ISVGTRANSFORMABLE_TRANSFORM: u32 = 1009u32;
pub const DISPID_ISVGTRANSFORMLIST_APPENDITEM: u32 = 1007u32;
pub const DISPID_ISVGTRANSFORMLIST_CLEAR: u32 = 1001u32;
pub const DISPID_ISVGTRANSFORMLIST_CONSOLIDATE: u32 = 1009u32;
pub const DISPID_ISVGTRANSFORMLIST_CREATESVGTRANSFORMFROMMATRIX: u32 = 1008u32;
pub const DISPID_ISVGTRANSFORMLIST_GETITEM: u32 = 1003u32;
pub const DISPID_ISVGTRANSFORMLIST_INITIALIZE: u32 = 1002u32;
pub const DISPID_ISVGTRANSFORMLIST_INSERTITEMBEFORE: u32 = 1004u32;
pub const DISPID_ISVGTRANSFORMLIST_NUMBEROFITEMS: u32 = 1000u32;
pub const DISPID_ISVGTRANSFORMLIST_REMOVEITEM: u32 = 1006u32;
pub const DISPID_ISVGTRANSFORMLIST_REPLACEITEM: u32 = 1005u32;
pub const DISPID_ISVGTRANSFORM_ANGLE: u32 = 1002u32;
pub const DISPID_ISVGTRANSFORM_MATRIX: u32 = 1001u32;
pub const DISPID_ISVGTRANSFORM_SETMATRIX: u32 = 1003u32;
pub const DISPID_ISVGTRANSFORM_SETROTATE: u32 = 1006u32;
pub const DISPID_ISVGTRANSFORM_SETSCALE: u32 = 1005u32;
pub const DISPID_ISVGTRANSFORM_SETSKEWX: u32 = 1007u32;
pub const DISPID_ISVGTRANSFORM_SETSKEWY: u32 = 1008u32;
pub const DISPID_ISVGTRANSFORM_SETTRANSLATE: u32 = 1004u32;
pub const DISPID_ISVGTRANSFORM_TYPE: u32 = 1000u32;
pub const DISPID_ISVGURIREFERENCE_HREF: u32 = 1026u32;
pub const DISPID_ISVGUSEELEMENT_ANIMATEDINSTANCEROOT: u32 = 1060u32;
pub const DISPID_ISVGUSEELEMENT_HEIGHT: u32 = 1058u32;
pub const DISPID_ISVGUSEELEMENT_INSTANCEROOT: u32 = 1059u32;
pub const DISPID_ISVGUSEELEMENT_WIDTH: u32 = 1056u32;
pub const DISPID_ISVGUSEELEMENT_X: u32 = 1052u32;
pub const DISPID_ISVGUSEELEMENT_Y: u32 = 1054u32;
pub const DISPID_ISVGVIEWELEMENT_VIEWTARGET: u32 = 1052u32;
pub const DISPID_ISVGZOOMANDPAN_ZOOMANDPAN: u32 = 1025u32;
pub const DISPID_ISVGZOOMEVENT_NEWSCALE: u32 = 1279u32;
pub const DISPID_ISVGZOOMEVENT_NEWTRANSLATE: u32 = 1280u32;
pub const DISPID_ISVGZOOMEVENT_PREVIOUSSCALE: u32 = 1277u32;
pub const DISPID_ISVGZOOMEVENT_PREVIOUSTRANSLATE: u32 = 1278u32;
pub const DISPID_ISVGZOOMEVENT_ZOOMRECTSCREEN: u32 = 1276u32;
pub const DISPID_ITEMPLATEPRINTER2_DEVICESUPPORTS: u32 = 41u32;
pub const DISPID_ITEMPLATEPRINTER2_FRAMEACTIVEENABLED: u32 = 38u32;
pub const DISPID_ITEMPLATEPRINTER2_ORIENTATION: u32 = 39u32;
pub const DISPID_ITEMPLATEPRINTER2_SELECTIONENABLED: u32 = 37u32;
pub const DISPID_ITEMPLATEPRINTER2_USEPRINTERCOPYCOLLATE: u32 = 40u32;
pub const DISPID_ITEMPLATEPRINTER3_GETPAGEMARGINBOTTOM: u32 = 45u32;
pub const DISPID_ITEMPLATEPRINTER3_GETPAGEMARGINBOTTOMIMPORTANT: u32 = 49u32;
pub const DISPID_ITEMPLATEPRINTER3_GETPAGEMARGINLEFT: u32 = 46u32;
pub const DISPID_ITEMPLATEPRINTER3_GETPAGEMARGINLEFTIMPORTANT: u32 = 50u32;
pub const DISPID_ITEMPLATEPRINTER3_GETPAGEMARGINRIGHT: u32 = 44u32;
pub const DISPID_ITEMPLATEPRINTER3_GETPAGEMARGINRIGHTIMPORTANT: u32 = 48u32;
pub const DISPID_ITEMPLATEPRINTER3_GETPAGEMARGINTOP: u32 = 43u32;
pub const DISPID_ITEMPLATEPRINTER3_GETPAGEMARGINTOPIMPORTANT: u32 = 47u32;
pub const DISPID_ITEMPLATEPRINTER3_HEADERFOOTERFONT: u32 = 42u32;
pub const DISPID_ITEMPLATEPRINTER_ALLLINKEDDOCUMENTS: u32 = 23u32;
pub const DISPID_ITEMPLATEPRINTER_COLLATE: u32 = 17u32;
pub const DISPID_ITEMPLATEPRINTER_COPIES: u32 = 19u32;
pub const DISPID_ITEMPLATEPRINTER_CURRENTPAGE: u32 = 15u32;
pub const DISPID_ITEMPLATEPRINTER_CURRENTPAGEAVAIL: u32 = 16u32;
pub const DISPID_ITEMPLATEPRINTER_DUPLEX: u32 = 18u32;
pub const DISPID_ITEMPLATEPRINTER_ENSUREPRINTDIALOGDEFAULTS: u32 = 5u32;
pub const DISPID_ITEMPLATEPRINTER_FOOTER: u32 = 25u32;
pub const DISPID_ITEMPLATEPRINTER_FRAMEACTIVE: u32 = 11u32;
pub const DISPID_ITEMPLATEPRINTER_FRAMEASSHOWN: u32 = 12u32;
pub const DISPID_ITEMPLATEPRINTER_FRAMESETDOCUMENT: u32 = 10u32;
pub const DISPID_ITEMPLATEPRINTER_HEADER: u32 = 24u32;
pub const DISPID_ITEMPLATEPRINTER_MARGINBOTTOM: u32 = 29u32;
pub const DISPID_ITEMPLATEPRINTER_MARGINLEFT: u32 = 26u32;
pub const DISPID_ITEMPLATEPRINTER_MARGINRIGHT: u32 = 27u32;
pub const DISPID_ITEMPLATEPRINTER_MARGINTOP: u32 = 28u32;
pub const DISPID_ITEMPLATEPRINTER_PAGEFROM: u32 = 20u32;
pub const DISPID_ITEMPLATEPRINTER_PAGEHEIGHT: u32 = 31u32;
pub const DISPID_ITEMPLATEPRINTER_PAGETO: u32 = 21u32;
pub const DISPID_ITEMPLATEPRINTER_PAGEWIDTH: u32 = 30u32;
pub const DISPID_ITEMPLATEPRINTER_PRINTBLANKPAGE: u32 = 3u32;
pub const DISPID_ITEMPLATEPRINTER_PRINTNONNATIVE: u32 = 8u32;
pub const DISPID_ITEMPLATEPRINTER_PRINTNONNATIVEFRAMES: u32 = 9u32;
pub const DISPID_ITEMPLATEPRINTER_PRINTPAGE: u32 = 4u32;
pub const DISPID_ITEMPLATEPRINTER_SELECTEDPAGES: u32 = 14u32;
pub const DISPID_ITEMPLATEPRINTER_SELECTION: u32 = 13u32;
pub const DISPID_ITEMPLATEPRINTER_SHOWPAGESETUPDIALOG: u32 = 7u32;
pub const DISPID_ITEMPLATEPRINTER_SHOWPRINTDIALOG: u32 = 6u32;
pub const DISPID_ITEMPLATEPRINTER_STARTDOC: u32 = 1u32;
pub const DISPID_ITEMPLATEPRINTER_STOPDOC: u32 = 2u32;
pub const DISPID_ITEMPLATEPRINTER_TABLEOFLINKS: u32 = 22u32;
pub const DISPID_ITEMPLATEPRINTER_UNPRINTABLEBOTTOM: u32 = 35u32;
pub const DISPID_ITEMPLATEPRINTER_UNPRINTABLELEFT: u32 = 32u32;
pub const DISPID_ITEMPLATEPRINTER_UNPRINTABLERIGHT: u32 = 34u32;
pub const DISPID_ITEMPLATEPRINTER_UNPRINTABLETOP: u32 = 33u32;
pub const DISPID_ITEMPLATEPRINTER_UPDATEPAGESTATUS: u32 = 36u32;
pub const DISPID_IWBSCRIPTCONTROL_BUBBLEEVENT: u32 = 2u32;
pub const DISPID_IWBSCRIPTCONTROL_FROZEN: u32 = 5u32;
pub const DISPID_IWBSCRIPTCONTROL_ONVISIBILITYCHANGE: u32 = 10u32;
pub const DISPID_IWBSCRIPTCONTROL_RAISEEVENT: u32 = 1u32;
pub const DISPID_IWBSCRIPTCONTROL_SCROLLBAR: u32 = 7u32;
pub const DISPID_IWBSCRIPTCONTROL_SELECTABLECONTENT: u32 = 4u32;
pub const DISPID_IWBSCRIPTCONTROL_SETCONTEXTMENU: u32 = 3u32;
pub const DISPID_IWBSCRIPTCONTROL_VERSION: u32 = 8u32;
pub const DISPID_IWBSCRIPTCONTROL_VISIBILITY: u32 = 9u32;
pub const DISPID_IWEBBRIDGE_ABOUTBOX: i32 = -552i32;
pub const DISPID_IWEBBRIDGE_EMBED: u32 = 3u32;
pub const DISPID_IWEBBRIDGE_EVENT: u32 = 1152u32;
pub const DISPID_IWEBBRIDGE_READYSTATE: i32 = -525i32;
pub const DISPID_IWEBBRIDGE_SCROLLBAR: u32 = 2u32;
pub const DISPID_IWEBBRIDGE_URL: u32 = 1u32;
pub const DISPID_IWEBGEOCOORDINATES_ACCURACY: u32 = 1004u32;
pub const DISPID_IWEBGEOCOORDINATES_ALTITUDE: u32 = 1003u32;
pub const DISPID_IWEBGEOCOORDINATES_ALTITUDEACCURACY: u32 = 1005u32;
pub const DISPID_IWEBGEOCOORDINATES_HEADING: u32 = 1006u32;
pub const DISPID_IWEBGEOCOORDINATES_LATITUDE: u32 = 1001u32;
pub const DISPID_IWEBGEOCOORDINATES_LONGITUDE: u32 = 1002u32;
pub const DISPID_IWEBGEOCOORDINATES_SPEED: u32 = 1007u32;
pub const DISPID_IWEBGEOLOCATION_CLEARWATCH: u32 = 1003u32;
pub const DISPID_IWEBGEOLOCATION_GETCURRENTPOSITION: u32 = 1001u32;
pub const DISPID_IWEBGEOLOCATION_WATCHPOSITION: u32 = 1002u32;
pub const DISPID_IWEBGEOPOSITIONERROR_CODE: u32 = 1001u32;
pub const DISPID_IWEBGEOPOSITIONERROR_MESSAGE: u32 = 1002u32;
pub const DISPID_IWEBGEOPOSITION_COORDS: u32 = 1001u32;
pub const DISPID_IWEBGEOPOSITION_TIMESTAMP: u32 = 1002u32;
pub const DISPID_LABEL: u32 = 1000u32;
pub const DISPID_LAUNCHIE: u32 = 91u32;
pub const DISPID_LAUNCHINHVSI: u32 = 99u32;
pub const DISPID_LAUNCHINTERNETOPTIONS: u32 = 74u32;
pub const DISPID_LAUNCHNETWORKCLIENTHELP: u32 = 67u32;
pub const DISPID_LI: u32 = 1000u32;
pub const DISPID_LINK: u32 = 1000u32;
pub const DISPID_LOCATION: u32 = 1u32;
pub const DISPID_LOCATIONOBJECT: i32 = -5506i32;
pub const DISPID_MAP: u32 = 1000u32;
pub const DISPID_MARKUP: u32 = 1000u32;
pub const DISPID_MARQUEE: u32 = 6000u32;
pub const DISPID_MEDIA: u32 = 1000u32;
pub const DISPID_MEDIAERROR: u32 = 1000u32;
pub const DISPID_MEDIALIST: u32 = 1000u32;
pub const DISPID_MEDIAQUERY: u32 = 1000u32;
pub const DISPID_MENU: u32 = 1000u32;
pub const DISPID_MIMETYPES_COL: u32 = 1000u32;
pub const DISPID_MODE: u32 = 18u32;
pub const DISPID_MOVESELECTIONDOWN: u32 = 2u32;
pub const DISPID_MOVESELECTIONTO: u32 = 9u32;
pub const DISPID_MOVESELECTIONUP: u32 = 1u32;
pub const DISPID_MSANIMATIONEND: u32 = 1095u32;
pub const DISPID_MSANIMATIONITERATION: u32 = 1096u32;
pub const DISPID_MSANIMATIONSTART: u32 = 1094u32;
pub const DISPID_MSDATASRCINTERFACE: i32 = -3900i32;
pub const DISPID_MSGESTURECHANGE: u32 = 1084u32;
pub const DISPID_MSGESTUREDOUBLETAP: u32 = 1088u32;
pub const DISPID_MSGESTUREEND: u32 = 1085u32;
pub const DISPID_MSGESTUREHOLD: u32 = 1086u32;
pub const DISPID_MSGESTUREINIT: u32 = 1097u32;
pub const DISPID_MSGESTURESTART: u32 = 1083u32;
pub const DISPID_MSGESTURETAP: u32 = 1087u32;
pub const DISPID_MSGOTPOINTERCAPTURE: u32 = 1091u32;
pub const DISPID_MSHTMLWEBVIEWELEMENT: u32 = 1000u32;
pub const DISPID_MSINERTIASTART: u32 = 1089u32;
pub const DISPID_MSLOSTPOINTERCAPTURE: u32 = 1090u32;
pub const DISPID_MSMANIPULATIONSTATECHANGED: u32 = 1098u32;
pub const DISPID_MSORIENTATIONCHANGE: u32 = 1103u32;
pub const DISPID_MSPOINTERCANCEL: u32 = 1081u32;
pub const DISPID_MSPOINTERDOWN: u32 = 1076u32;
pub const DISPID_MSPOINTERENTER: u32 = 1101u32;
pub const DISPID_MSPOINTERHOVER: u32 = 1082u32;
pub const DISPID_MSPOINTERLEAVE: u32 = 1102u32;
pub const DISPID_MSPOINTERMOVE: u32 = 1077u32;
pub const DISPID_MSPOINTEROUT: u32 = 1080u32;
pub const DISPID_MSPOINTEROVER: u32 = 1079u32;
pub const DISPID_MSPOINTERPOINT: u32 = 1000u32;
pub const DISPID_MSPOINTERUP: u32 = 1078u32;
pub const DISPID_MSTRANSITIONEND: u32 = 1093u32;
pub const DISPID_MSTRANSITIONSTART: u32 = 1092u32;
pub const DISPID_NAMESPACE: u32 = 1000u32;
pub const DISPID_NAMESPACE_COLLECTION: u32 = 1000u32;
pub const DISPID_NAVIGATEANDFIND: u32 = 8u32;
pub const DISPID_NAVIGATECOMPLETE: u32 = 101u32;
pub const DISPID_NAVIGATECOMPLETE2: u32 = 252u32;
pub const DISPID_NAVIGATEERROR: u32 = 271u32;
pub const DISPID_NAVIGATETOSUGGESTEDSITES: u32 = 40u32;
pub const DISPID_NAVIGATOR: u32 = 1u32;
pub const DISPID_NAVIGATOROBJECT: i32 = -5508i32;
pub const DISPID_NEWFOLDER: u32 = 4u32;
pub const DISPID_NEWPROCESS: u32 = 284u32;
pub const DISPID_NEWWINDOW: u32 = 107u32;
pub const DISPID_NEWWINDOW2: u32 = 251u32;
pub const DISPID_NEWWINDOW3: u32 = 273u32;
pub const DISPID_NORMAL_FIRST: u32 = 1000u32;
pub const DISPID_NSCOLUMNS: u32 = 21u32;
pub const DISPID_OBJECT: u32 = 68536u32;
pub const DISPID_OBJECT_ORDINAL_BASE: u32 = 73536u32;
pub const DISPID_OBJECT_ORDINAL_MAX: u32 = 74535u32;
pub const DISPID_OL: u32 = 1000u32;
pub const DISPID_OLESITE: u32 = 1000u32;
pub const DISPID_OMDOCUMENT: u32 = 1000u32;
pub const DISPID_OMRECT: u32 = 1000u32;
pub const DISPID_OMWINDOW: u32 = 1000u32;
pub const DISPID_ONABORT: u32 = 1000u32;
pub const DISPID_ONACTIVATE: u32 = 1044u32;
pub const DISPID_ONADDRESSBAR: u32 = 261u32;
pub const DISPID_ONAFTERPRINT: u32 = 1025u32;
pub const DISPID_ONALERT: u32 = 1061u32;
pub const DISPID_ONBEFOREACTIVATE: u32 = 1047u32;
pub const DISPID_ONBEFOREDEACTIVATE: u32 = 1034u32;
pub const DISPID_ONBEFOREEDITFOCUS: u32 = 1027u32;
pub const DISPID_ONBEFOREPRINT: u32 = 1024u32;
pub const DISPID_ONBEFOREUNLOAD: u32 = 1017u32;
pub const DISPID_ONBOUNCE: u32 = 1009u32;
pub const DISPID_ONCHANGE: u32 = 1001u32;
pub const DISPID_ONCHANGEBLUR: u32 = 1019u32;
pub const DISPID_ONCHANGEFOCUS: u32 = 1018u32;
pub const DISPID_ONCLOSE: u32 = 1100u32;
pub const DISPID_ONCOMPASSNEEDSCALIBRATION: u32 = 1108u32;
pub const DISPID_ONCONTENTREADY: u32 = 1029u32;
pub const DISPID_ONCONTEXTMENU: u32 = 1023u32;
pub const DISPID_ONCONTROLSELECT: u32 = 1036u32;
pub const DISPID_ONDEACTIVATE: u32 = 1045u32;
pub const DISPID_ONDEVICEMOTION: u32 = 1105u32;
pub const DISPID_ONDEVICEORIENTATION: u32 = 1104u32;
pub const DISPID_ONDOMMUTATION: u32 = 1068u32;
pub const DISPID_ONERROR: u32 = 1002u32;
pub const DISPID_ONFINISH: u32 = 1010u32;
pub const DISPID_ONFOCUSIN: u32 = 1048u32;
pub const DISPID_ONFOCUSOUT: u32 = 1049u32;
pub const DISPID_ONFULLSCREEN: u32 = 258u32;
pub const DISPID_ONHASHCHANGE: u32 = 1066u32;
pub const DISPID_ONHIDE: u32 = 1060u32;
pub const DISPID_ONLAYOUT: u32 = 1013u32;
pub const DISPID_ONLAYOUTCOMPLETE: u32 = 1030u32;
pub const DISPID_ONLINKEDOVERFLOW: u32 = 1032u32;
pub const DISPID_ONLOAD: u32 = 1003u32;
pub const DISPID_ONMENUBAR: u32 = 256u32;
pub const DISPID_ONMESSAGE: u32 = 1067u32;
pub const DISPID_ONMOUSEENTER: u32 = 1042u32;
pub const DISPID_ONMOUSEHOVER: u32 = 1028u32;
pub const DISPID_ONMOUSELEAVE: u32 = 1043u32;
pub const DISPID_ONMOUSEWHEEL: u32 = 1033u32;
pub const DISPID_ONMOVE: u32 = 1035u32;
pub const DISPID_ONMOVEEND: u32 = 1039u32;
pub const DISPID_ONMOVESTART: u32 = 1038u32;
pub const DISPID_ONMULTILAYOUTCLEANUP: u32 = 1046u32;
pub const DISPID_ONOBJECTCONTENTSCROLLED: u32 = 1056u32;
pub const DISPID_ONOFFLINE: u32 = 1065u32;
pub const DISPID_ONONLINE: u32 = 1064u32;
pub const DISPID_ONOPEN: u32 = 1099u32;
pub const DISPID_ONPAGE: u32 = 1031u32;
pub const DISPID_ONPAGEHIDE: u32 = 1107u32;
pub const DISPID_ONPAGESHOW: u32 = 1106u32;
pub const DISPID_ONPERSIST: u32 = 1020u32;
pub const DISPID_ONPERSISTLOAD: u32 = 1022u32;
pub const DISPID_ONPERSISTSAVE: u32 = 1021u32;
pub const DISPID_ONPOPUPMENUEND: u32 = 1063u32;
pub const DISPID_ONPOPUPMENUSTART: u32 = 1062u32;
pub const DISPID_ONQUIT: u32 = 253u32;
pub const DISPID_ONRESET: u32 = 1015u32;
pub const DISPID_ONRESIZE: u32 = 1016u32;
pub const DISPID_ONRESIZEEND: u32 = 1041u32;
pub const DISPID_ONRESIZESTART: u32 = 1040u32;
pub const DISPID_ONSCROLL: u32 = 1014u32;
pub const DISPID_ONSELECT: u32 = 1006u32;
pub const DISPID_ONSELECTADD: u32 = 1051u32;
pub const DISPID_ONSELECTIONCHANGE: u32 = 1037u32;
pub const DISPID_ONSELECTREMOVE: u32 = 1052u32;
pub const DISPID_ONSELECTWITHIN: u32 = 1053u32;
pub const DISPID_ONSHOW: u32 = 1059u32;
pub const DISPID_ONSTART: u32 = 1011u32;
pub const DISPID_ONSTATUSBAR: u32 = 257u32;
pub const DISPID_ONSTOP: u32 = 1026u32;
pub const DISPID_ONSTORAGE: u32 = 1057u32;
pub const DISPID_ONSTORAGECOMMIT: u32 = 1058u32;
pub const DISPID_ONSUBMIT: u32 = 1007u32;
pub const DISPID_ONSYSTEMSCROLLINGEND: u32 = 1055u32;
pub const DISPID_ONSYSTEMSCROLLINGSTART: u32 = 1054u32;
pub const DISPID_ONTHEATERMODE: u32 = 260u32;
pub const DISPID_ONTOOLBAR: u32 = 255u32;
pub const DISPID_ONUNLOAD: u32 = 1008u32;
pub const DISPID_ONVALUECHANGE: u32 = 1050u32;
pub const DISPID_ONVISIBLE: u32 = 254u32;
pub const DISPID_OPENFAVORITESPANE: u32 = 97u32;
pub const DISPID_OPENFAVORITESSETTINGS: u32 = 98u32;
pub const DISPID_OPTION: u32 = 1000u32;
pub const DISPID_OPTIONS_COL: u32 = 1500u32;
pub const DISPID_PARA: u32 = 1000u32;
pub const DISPID_PARAM: u32 = 1000u32;
pub const DISPID_PEER_HOLDER_BASE: u32 = 5000000u32;
pub const DISPID_PERFORMANCE: u32 = 1000u32;
pub const DISPID_PERFORMANCENAVIGATION: u32 = 1000u32;
pub const DISPID_PERFORMANCEOBJECT: i32 = -5505i32;
pub const DISPID_PERFORMANCETIMING: u32 = 1000u32;
pub const DISPID_PERSISTDATA: u32 = 1000u32;
pub const DISPID_PHISHINGENABLED: u32 = 19u32;
pub const DISPID_PHRASE: u32 = 1000u32;
pub const DISPID_PINNEDSITESTATE: u32 = 73u32;
pub const DISPID_PLAYTO: u32 = 1000u32;
pub const DISPID_PLAYTODEVICE: u32 = 1000u32;
pub const DISPID_PLUGINS_COL: u32 = 1000u32;
pub const DISPID_PRINTMANAGER_TEMPLATE_PRINTER: u32 = 501u32;
pub const DISPID_PRINTTEMPLATEINSTANTIATION: u32 = 225u32;
pub const DISPID_PRINTTEMPLATETEARDOWN: u32 = 226u32;
pub const DISPID_PRIVACYIMPACTEDSTATECHANGE: u32 = 272u32;
pub const DISPID_PROCESSINGINSTRUCTION: u32 = 1000u32;
pub const DISPID_PROGRESS: u32 = 1000u32;
pub const DISPID_PROGRESSCHANGE: u32 = 108u32;
pub const DISPID_PROPERTYCHANGE: u32 = 112u32;
pub const DISPID_PROTECTEDELEMENT: u32 = 1000u32;
pub const DISPID_PROVISIONNETWORKS: u32 = 62u32;
pub const DISPID_QUIT: u32 = 103u32;
pub const DISPID_RADIO: u32 = 2000u32;
pub const DISPID_RANGE: u32 = 1000u32;
pub const DISPID_RANGEEXCEPTION: u32 = 1000u32;
pub const DISPID_REDIRECTXDOMAINBLOCKED: u32 = 286u32;
pub const DISPID_REFRESHOFFLINEDESKTOP: u32 = 3u32;
pub const DISPID_REMOVESCHEDULEDTILENOTIFICATION: u32 = 80u32;
pub const DISPID_REPORTSAFEURL: u32 = 63u32;
pub const DISPID_RESETEXPERIMENTALFLAGS: u32 = 92u32;
pub const DISPID_RESETFIRSTBOOTMODE: u32 = 1u32;
pub const DISPID_RESETSAFEMODE: u32 = 2u32;
pub const DISPID_RESETSORT: u32 = 3u32;
pub const DISPID_RETREATERROR: u32 = 11u32;
pub const DISPID_RICHTEXT: u32 = 7000u32;
pub const DISPID_ROOT: u32 = 16u32;
pub const DISPID_RULESAPPLIED: u32 = 1000u32;
pub const DISPID_RULESAPPLIED_COLLECTION: u32 = 1000u32;
pub const DISPID_RUNONCEHASSHOWN: u32 = 28u32;
pub const DISPID_RUNONCEREQUIREDSETTINGSCOMPLETE: u32 = 27u32;
pub const DISPID_RUNONCESHOWN: u32 = 15u32;
pub const DISPID_SCHEDULEDTILENOTIFICATION: u32 = 79u32;
pub const DISPID_SCREEN: u32 = 1000u32;
pub const DISPID_SCRIPT: u32 = 1000u32;
pub const DISPID_SEARCHGUIDEURL: u32 = 29u32;
pub const DISPID_SECURITYCTX: i32 = -5511i32;
pub const DISPID_SECURITYDOMAIN: i32 = -5514i32;
pub const DISPID_SELECT: u32 = 1000u32;
pub const DISPID_SELECTEDITEM: u32 = 15u32;
pub const DISPID_SELECTEDITEMS: u32 = 24u32;
pub const DISPID_SELECTION: u32 = 1000u32;
pub const DISPID_SELECTIONCHANGE: u32 = 2u32;
pub const DISPID_SELECTOBJ: u32 = 1000u32;
pub const DISPID_SETACTIVITIESVISIBLE: u32 = 35u32;
pub const DISPID_SETDETAILSSTATE: u32 = 20u32;
pub const DISPID_SETEXPERIMENTALFLAG: u32 = 84u32;
pub const DISPID_SETEXPERIMENTALVALUE: u32 = 86u32;
pub const DISPID_SETMSDEFAULTS: u32 = 104u32;
pub const DISPID_SETNEEDHVSIAUTOLAUNCHFLAG: u32 = 101u32;
pub const DISPID_SETNEEDIEAUTOLAUNCHFLAG: u32 = 90u32;
pub const DISPID_SETPERERRSTATE: u32 = 22u32;
pub const DISPID_SETPHISHINGFILTERSTATUS: u32 = 282u32;
pub const DISPID_SETRECENTLYCLOSEDVISIBLE: u32 = 34u32;
pub const DISPID_SETROOT: u32 = 13u32;
pub const DISPID_SETSECURELOCKICON: u32 = 269u32;
pub const DISPID_SETSITEMODEICONOVERLAY: u32 = 44u32;
pub const DISPID_SETSITEMODEPROPERTIES: u32 = 50u32;
pub const DISPID_SETTHUMBNAILBUTTONS: u32 = 47u32;
pub const DISPID_SETVIEWTYPE: u32 = 23u32;
pub const DISPID_SHELLUIHELPERLAST: u32 = 105u32;
pub const DISPID_SHOWBROWSERUI: u32 = 13u32;
pub const DISPID_SHOWINPRIVATEHELP: u32 = 42u32;
pub const DISPID_SHOWTABSHELP: u32 = 41u32;
pub const DISPID_SITE: u32 = 67536u32;
pub const DISPID_SITEMODEACTIVATE: u32 = 58u32;
pub const DISPID_SITEMODEADDBUTTONSTYLE: u32 = 54u32;
pub const DISPID_SITEMODEADDJUMPLISTITEM: u32 = 52u32;
pub const DISPID_SITEMODECLEARBADGE: u32 = 65u32;
pub const DISPID_SITEMODECLEARJUMPLIST: u32 = 53u32;
pub const DISPID_SITEMODECREATEJUMPLIST: u32 = 51u32;
pub const DISPID_SITEMODEREFRESHBADGE: u32 = 64u32;
pub const DISPID_SITEMODESHOWBUTTONSTYLE: u32 = 55u32;
pub const DISPID_SITEMODESHOWJUMPLIST: u32 = 56u32;
pub const DISPID_SKIPRUNONCE: u32 = 16u32;
pub const DISPID_SKIPTABSWELCOME: u32 = 21u32;
pub const DISPID_SOURCE: u32 = 1000u32;
pub const DISPID_SQMENABLED: u32 = 18u32;
pub const DISPID_STARTBADGEUPDATE: u32 = 81u32;
pub const DISPID_STARTPERIODICUPDATE: u32 = 70u32;
pub const DISPID_STARTPERIODICUPDATEBATCH: u32 = 75u32;
pub const DISPID_STATUSTEXTCHANGE: u32 = 102u32;
pub const DISPID_STOPBADGEUPDATE: u32 = 82u32;
pub const DISPID_STOPPERIODICUPDATE: u32 = 69u32;
pub const DISPID_STYLE: u32 = 69536u32;
pub const DISPID_STYLEELEMENT: u32 = 1000u32;
pub const DISPID_STYLEMEDIA: u32 = 1000u32;
pub const DISPID_STYLEPAGE: u32 = 1000u32;
pub const DISPID_STYLEPAGES_COL: u32 = 1000u32;
pub const DISPID_STYLERULE: u32 = 1000u32;
pub const DISPID_STYLERULES_COL: u32 = 1000u32;
pub const DISPID_STYLESHEET: u32 = 1000u32;
pub const DISPID_STYLESHEETRULESAPPLIED_COLLECTION: u32 = 1000u32;
pub const DISPID_STYLESHEETSCOLLECTION_NAMED_BASE: u32 = 1000000u32;
pub const DISPID_STYLESHEETSCOLLECTION_NAMED_MAX: u32 = 1999999u32;
pub const DISPID_STYLESHEETSCOLLECTION_ORDINAL_BASE: u32 = 2000000u32;
pub const DISPID_STYLESHEETSCOLLECTION_ORDINAL_MAX: u32 = 2999999u32;
pub const DISPID_STYLESHEETS_COL: u32 = 1000u32;
pub const DISPID_SUBSCRIPTIONSENABLED: u32 = 10u32;
pub const DISPID_SUGGESTEDSITESENABLED: u32 = 38u32;
pub const DISPID_SVGABORT: u32 = 1071u32;
pub const DISPID_SVGAELEMENT: u32 = 1050u32;
pub const DISPID_SVGALTGLYPHDEFELEMENT: u32 = 1050u32;
pub const DISPID_SVGALTGLYPHELEMENT: u32 = 1050u32;
pub const DISPID_SVGALTGLYPHITEMELEMENT: u32 = 1050u32;
pub const DISPID_SVGANGLE: u32 = 1000u32;
pub const DISPID_SVGANIMATECOLORELEMENT: u32 = 1050u32;
pub const DISPID_SVGANIMATEDANGLE: u32 = 1000u32;
pub const DISPID_SVGANIMATEDBOOLEAN: u32 = 1000u32;
pub const DISPID_SVGANIMATEDENUMERATION: u32 = 1000u32;
pub const DISPID_SVGANIMATEDINTEGER: u32 = 1000u32;
pub const DISPID_SVGANIMATEDLENGTH: u32 = 1000u32;
pub const DISPID_SVGANIMATEDLENGTHLIST: u32 = 1000u32;
pub const DISPID_SVGANIMATEDNUMBER: u32 = 1000u32;
pub const DISPID_SVGANIMATEDNUMBERLIST: u32 = 1000u32;
pub const DISPID_SVGANIMATEDPOINTS: u32 = 1000u32;
pub const DISPID_SVGANIMATEDPRESERVEASPECTRATIO: u32 = 1000u32;
pub const DISPID_SVGANIMATEDRECT: u32 = 1000u32;
pub const DISPID_SVGANIMATEDSTRING: u32 = 1000u32;
pub const DISPID_SVGANIMATEDTRANSFORMLIST: u32 = 1000u32;
pub const DISPID_SVGANIMATEELEMENT: u32 = 1050u32;
pub const DISPID_SVGANIMATEMOTIONELEMENT: u32 = 1050u32;
pub const DISPID_SVGANIMATETRANSFORMELEMENT: u32 = 1050u32;
pub const DISPID_SVGCIRCLEELEMENT: u32 = 1050u32;
pub const DISPID_SVGCLIPPATHELEMENT: u32 = 1050u32;
pub const DISPID_SVGCOLOR_PROFILEELEMENT: u32 = 1050u32;
pub const DISPID_SVGCOMPONENTTRANSFERFUNCTIONELEMENT: u32 = 1050u32;
pub const DISPID_SVGCURSORELEMENT: u32 = 1050u32;
pub const DISPID_SVGDEFINITION_SRCELEMENT: u32 = 1050u32;
pub const DISPID_SVGDEFSELEMENT: u32 = 1050u32;
pub const DISPID_SVGDESCELEMENT: u32 = 1050u32;
pub const DISPID_SVGELEMENT: u32 = 1030u32;
pub const DISPID_SVGELEMENTINSTANCE: u32 = 1000u32;
pub const DISPID_SVGELEMENTINSTANCELIST: u32 = 1000u32;
pub const DISPID_SVGELEMENT_BASE: u32 = 1050u32;
pub const DISPID_SVGELLIPSEELEMENT: u32 = 1050u32;
pub const DISPID_SVGERROR: u32 = 1072u32;
pub const DISPID_SVGEXCEPTION: u32 = 1000u32;
pub const DISPID_SVGEXTERNALRESOURCESREQUIRED_EXTERNALRESOURCESREQUIRED_ATTR: u32 = 1019u32;
pub const DISPID_SVGEXTERNALRESOURCESREQUIRED_EXTERNALRESOURCESREQUIRED_PROP: u32 = 1020u32;
pub const DISPID_SVGFEBLENDELEMENT: u32 = 1050u32;
pub const DISPID_SVGFECOLORMATRIXELEMENT: u32 = 1050u32;
pub const DISPID_SVGFECOMPONENTTRANSFERELEMENT: u32 = 1050u32;
pub const DISPID_SVGFECOMPOSITEELEMENT: u32 = 1050u32;
pub const DISPID_SVGFECONVOLVEMATRIXELEMENT: u32 = 1050u32;
pub const DISPID_SVGFEDIFFUSELIGHTINGELEMENT: u32 = 1050u32;
pub const DISPID_SVGFEDISPLACEMENTMAPELEMENT: u32 = 1050u32;
pub const DISPID_SVGFEDISTANTLIGHTELEMENT: u32 = 1050u32;
pub const DISPID_SVGFEFLOODELEMENT: u32 = 1050u32;
pub const DISPID_SVGFEFUNCAELEMENT: u32 = 1050u32;
pub const DISPID_SVGFEFUNCBELEMENT: u32 = 1050u32;
pub const DISPID_SVGFEFUNCGELEMENT: u32 = 1050u32;
pub const DISPID_SVGFEFUNCRELEMENT: u32 = 1050u32;
pub const DISPID_SVGFEGAUSSIANBLURELEMENT: u32 = 1050u32;
pub const DISPID_SVGFEIMAGEELEMENT: u32 = 1050u32;
pub const DISPID_SVGFEMERGEELEMENT: u32 = 1050u32;
pub const DISPID_SVGFEMERGENODEELEMENT: u32 = 1050u32;
pub const DISPID_SVGFEMORPHOLOGYELEMENT: u32 = 1050u32;
pub const DISPID_SVGFEOFFSETELEMENT: u32 = 1050u32;
pub const DISPID_SVGFEPOINTLIGHTELEMENT: u32 = 1050u32;
pub const DISPID_SVGFESPECULARLIGHTINGELEMENT: u32 = 1050u32;
pub const DISPID_SVGFESPOTLIGHTELEMENT: u32 = 1050u32;
pub const DISPID_SVGFETILEELEMENT: u32 = 1050u32;
pub const DISPID_SVGFETURBULENCEELEMENT: u32 = 1050u32;
pub const DISPID_SVGFILTERELEMENT: u32 = 1050u32;
pub const DISPID_SVGFITTOVIEWBOX_PRESERVEASPECTRATIO_ATTR: u32 = 1023u32;
pub const DISPID_SVGFITTOVIEWBOX_PRESERVEASPECTRATIO_PROP: u32 = 1024u32;
pub const DISPID_SVGFITTOVIEWBOX_VIEWBOX_ATTR: u32 = 1021u32;
pub const DISPID_SVGFITTOVIEWBOX_VIEWBOX_PROP: u32 = 1022u32;
pub const DISPID_SVGFONTELEMENT: u32 = 1050u32;
pub const DISPID_SVGFONT_FACEELEMENT: u32 = 1050u32;
pub const DISPID_SVGFONT_FACE_FORMATELEMENT: u32 = 1050u32;
pub const DISPID_SVGFONT_FACE_NAMEELEMENT: u32 = 1050u32;
pub const DISPID_SVGFONT_FACE_SRCELEMENT: u32 = 1050u32;
pub const DISPID_SVGFONT_FACE_URIELEMENT: u32 = 1050u32;
pub const DISPID_SVGFOREIGNOBJECTELEMENT: u32 = 1050u32;
pub const DISPID_SVGGELEMENT: u32 = 1050u32;
pub const DISPID_SVGGLYPHELEMENT: u32 = 1050u32;
pub const DISPID_SVGGLYPHREFELEMENT: u32 = 1050u32;
pub const DISPID_SVGGRADIENTELEMENT: u32 = 1050u32;
pub const DISPID_SVGHKERNELEMENT: u32 = 1050u32;
pub const DISPID_SVGIMAGEELEMENT: u32 = 1050u32;
pub const DISPID_SVGLANGSPACE_XMLLANG: u32 = 1017u32;
pub const DISPID_SVGLANGSPACE_XMLSPACE: u32 = 1018u32;
pub const DISPID_SVGLENGTH: u32 = 1000u32;
pub const DISPID_SVGLENGTHLIST: u32 = 1000u32;
pub const DISPID_SVGLINEARGRADIENTELEMENT: u32 = 1070u32;
pub const DISPID_SVGLINEELEMENT: u32 = 1050u32;
pub const DISPID_SVGLOAD: u32 = 1069u32;
pub const DISPID_SVGLOCATABLE_FARTHESTVIEWPORTELEMENT: u32 = 1003u32;
pub const DISPID_SVGLOCATABLE_GETBBOX: u32 = 1004u32;
pub const DISPID_SVGLOCATABLE_GETCTM: u32 = 1005u32;
pub const DISPID_SVGLOCATABLE_GETSCREENCTM: u32 = 1006u32;
pub const DISPID_SVGLOCATABLE_GETTRANSFORMTOELEMENT: u32 = 1007u32;
pub const DISPID_SVGLOCATABLE_NEARESTVIEWPORTELEMENT: u32 = 1002u32;
pub const DISPID_SVGMARKERELEMENT: u32 = 1050u32;
pub const DISPID_SVGMASKELEMENT: u32 = 1050u32;
pub const DISPID_SVGMATRIX: u32 = 1000u32;
pub const DISPID_SVGMETADATAELEMENT: u32 = 1050u32;
pub const DISPID_SVGMISSING_GLYPHELEMENT: u32 = 1050u32;
pub const DISPID_SVGMIXINS: u32 = 1000u32;
pub const DISPID_SVGMPATHELEMENT: u32 = 1050u32;
pub const DISPID_SVGNUMBER: u32 = 1000u32;
pub const DISPID_SVGNUMBERLIST: u32 = 1000u32;
pub const DISPID_SVGPATHELEMENT: u32 = 1050u32;
pub const DISPID_SVGPATHSEG: u32 = 1000u32;
pub const DISPID_SVGPATHSEGARCABS: u32 = 1020u32;
pub const DISPID_SVGPATHSEGARCREL: u32 = 1020u32;
pub const DISPID_SVGPATHSEGCLOSEPATH: u32 = 1020u32;
pub const DISPID_SVGPATHSEGCURVETOCUBICABS: u32 = 1020u32;
pub const DISPID_SVGPATHSEGCURVETOCUBICREL: u32 = 1020u32;
pub const DISPID_SVGPATHSEGCURVETOCUBICSMOOTHABS: u32 = 1020u32;
pub const DISPID_SVGPATHSEGCURVETOCUBICSMOOTHREL: u32 = 1020u32;
pub const DISPID_SVGPATHSEGCURVETOQUADRATICABS: u32 = 1020u32;
pub const DISPID_SVGPATHSEGCURVETOQUADRATICREL: u32 = 1020u32;
pub const DISPID_SVGPATHSEGCURVETOQUADRATICSMOOTHABS: u32 = 1020u32;
pub const DISPID_SVGPATHSEGCURVETOQUADRATICSMOOTHREL: u32 = 1020u32;
pub const DISPID_SVGPATHSEGLINETOABS: u32 = 1020u32;
pub const DISPID_SVGPATHSEGLINETOHORIZONTALABS: u32 = 1020u32;
pub const DISPID_SVGPATHSEGLINETOHORIZONTALREL: u32 = 1020u32;
pub const DISPID_SVGPATHSEGLINETOREL: u32 = 1020u32;
pub const DISPID_SVGPATHSEGLINETOVERTICALABS: u32 = 1020u32;
pub const DISPID_SVGPATHSEGLINETOVERTICALREL: u32 = 1020u32;
pub const DISPID_SVGPATHSEGLIST: u32 = 1000u32;
pub const DISPID_SVGPATHSEGMOVETOABS: u32 = 1020u32;
pub const DISPID_SVGPATHSEGMOVETOREL: u32 = 1020u32;
pub const DISPID_SVGPATHSEG_BASE: u32 = 1020u32;
pub const DISPID_SVGPATTERNELEMENT: u32 = 1050u32;
pub const DISPID_SVGPOINT: u32 = 1000u32;
pub const DISPID_SVGPOINTLIST: u32 = 1000u32;
pub const DISPID_SVGPOLYGONELEMENT: u32 = 1050u32;
pub const DISPID_SVGPOLYLINEELEMENT: u32 = 1050u32;
pub const DISPID_SVGPRESERVEASPECTRATIO: u32 = 1000u32;
pub const DISPID_SVGRADIALGRADIENTELEMENT: u32 = 1070u32;
pub const DISPID_SVGRECT: u32 = 1000u32;
pub const DISPID_SVGRECTELEMENT: u32 = 1050u32;
pub const DISPID_SVGRESIZE: u32 = 1073u32;
pub const DISPID_SVGSCRIPTELEMENT: u32 = 1050u32;
pub const DISPID_SVGSCROLL: u32 = 1074u32;
pub const DISPID_SVGSETELEMENT: u32 = 1050u32;
pub const DISPID_SVGSTOPELEMENT: u32 = 1050u32;
pub const DISPID_SVGSTRINGLIST: u32 = 1000u32;
pub const DISPID_SVGSTYLABLE_CLASSNAME_PROP: u32 = 1001u32;
pub const DISPID_SVGSTYLEELEMENT: u32 = 1050u32;
pub const DISPID_SVGSVGELEMENT: u32 = 1050u32;
pub const DISPID_SVGSWITCHELEMENT: u32 = 1050u32;
pub const DISPID_SVGSYMBOLELEMENT: u32 = 1050u32;
pub const DISPID_SVGTESTS_HASEXTENSION: u32 = 1016u32;
pub const DISPID_SVGTESTS_REQUIREDEXTENSIONS_ATTR: u32 = 1012u32;
pub const DISPID_SVGTESTS_REQUIREDEXTENSIONS_PROP: u32 = 1013u32;
pub const DISPID_SVGTESTS_REQUIREDFEATURES_ATTR: u32 = 1010u32;
pub const DISPID_SVGTESTS_REQUIREDFEATURES_PROP: u32 = 1011u32;
pub const DISPID_SVGTESTS_SYSTEMLANGUAGE_ATTR: u32 = 1014u32;
pub const DISPID_SVGTESTS_SYSTEMLANGUAGE_PROP: u32 = 1015u32;
pub const DISPID_SVGTEXTCONTENTELEMENT: u32 = 1050u32;
pub const DISPID_SVGTEXTCONTENTELEMENT_BASE: u32 = 1070u32;
pub const DISPID_SVGTEXTELEMENT: u32 = 1090u32;
pub const DISPID_SVGTEXTPATHELEMENT: u32 = 1070u32;
pub const DISPID_SVGTEXTPOSITIONINGELEMENT: u32 = 1070u32;
pub const DISPID_SVGTEXTPOSITIONINGELEMENT_BASE: u32 = 1090u32;
pub const DISPID_SVGTITLEELEMENT: u32 = 1050u32;
pub const DISPID_SVGTRANSFORM: u32 = 1000u32;
pub const DISPID_SVGTRANSFORMABLE_TRANSFORM_ATTR: u32 = 1008u32;
pub const DISPID_SVGTRANSFORMABLE_TRANSFORM_PROP: u32 = 1009u32;
pub const DISPID_SVGTRANSFORMLIST: u32 = 1000u32;
pub const DISPID_SVGTREFELEMENT: u32 = 1050u32;
pub const DISPID_SVGTSPANELEMENT: u32 = 1090u32;
pub const DISPID_SVGUNLOAD: u32 = 1070u32;
pub const DISPID_SVGURIREFERENCE_HREF: u32 = 1026u32;
pub const DISPID_SVGUSEELEMENT: u32 = 1050u32;
pub const DISPID_SVGVIEWELEMENT: u32 = 1050u32;
pub const DISPID_SVGVKERNELEMENT: u32 = 1050u32;
pub const DISPID_SVGZOOM: u32 = 1075u32;
pub const DISPID_SVGZOOMANDPAN_ZOOMANDPAN: u32 = 1025u32;
pub const DISPID_SVGZOOMEVENT: u32 = 1275u32;
pub const DISPID_SYNCHRONIZE: u32 = 5u32;
pub const DISPID_TABLE: u32 = 1000u32;
pub const DISPID_TABLECELL: u32 = 2000u32;
pub const DISPID_TABLECOL: u32 = 1000u32;
pub const DISPID_TABLEROW: u32 = 1000u32;
pub const DISPID_TABLESECTION: u32 = 1000u32;
pub const DISPID_TAGNAMES_COLLECTION: u32 = 1000u32;
pub const DISPID_TEMPLATE_PRINTER: u32 = 1u32;
pub const DISPID_TEXTAREA: u32 = 5000u32;
pub const DISPID_TEXTSITE: u32 = 1000u32;
pub const DISPID_THIRDPARTYURLBLOCKED: u32 = 285u32;
pub const DISPID_TIMERANGES: u32 = 1000u32;
pub const DISPID_TITLECHANGE: u32 = 113u32;
pub const DISPID_TITLEICONCHANGE: u32 = 114u32;
pub const DISPID_TRACK: u32 = 1000u32;
pub const DISPID_TRACKINGPROTECTIONENABLED: u32 = 60u32;
pub const DISPID_TVFLAGS: u32 = 20u32;
pub const DISPID_UL: u32 = 1000u32;
pub const DISPID_UNKNOWNPDL: u32 = 1000u32;
pub const DISPID_UNSELECTALL: u32 = 26u32;
pub const DISPID_UPDATEPAGESTATUS: u32 = 227u32;
pub const DISPID_UPDATETHUMBNAILBUTTON: u32 = 46u32;
pub const DISPID_URN_COLL: u32 = 1000u32;
pub const DISPID_VIDEO: u32 = 1050u32;
pub const DISPID_VIEWUPDATE: u32 = 281u32;
pub const DISPID_WEBGEOCOORDINATES: u32 = 1000u32;
pub const DISPID_WEBGEOLOCATION: u32 = 1000u32;
pub const DISPID_WEBGEOPOSITION: u32 = 1000u32;
pub const DISPID_WEBGEOPOSITION_ERROR: u32 = 1000u32;
pub const DISPID_WEBSOCKET: u32 = 1000u32;
pub const DISPID_WEBWORKERFINISHED: u32 = 289u32;
pub const DISPID_WEBWORKERSTARTED: u32 = 288u32;
pub const DISPID_WINDOW: u32 = 1u32;
pub const DISPID_WINDOWACTIVATE: u32 = 111u32;
pub const DISPID_WINDOWCLOSING: u32 = 263u32;
pub const DISPID_WINDOWMOVE: u32 = 109u32;
pub const DISPID_WINDOWOBJECT: i32 = -5500i32;
pub const DISPID_WINDOWREGISTERED: u32 = 200u32;
pub const DISPID_WINDOWRESIZE: u32 = 110u32;
pub const DISPID_WINDOWREVOKED: u32 = 201u32;
pub const DISPID_WINDOWSETHEIGHT: u32 = 267u32;
pub const DISPID_WINDOWSETLEFT: u32 = 264u32;
pub const DISPID_WINDOWSETRESIZABLE: u32 = 262u32;
pub const DISPID_WINDOWSETTOP: u32 = 265u32;
pub const DISPID_WINDOWSETWIDTH: u32 = 266u32;
pub const DISPID_WINDOWSTATECHANGED: u32 = 283u32;
pub const DISPID_XDOMAINREQUEST: u32 = 1000u32;
pub const DISPID_XMLDECL: u32 = 1000u32;
pub const DISPID_XMLHTTPREQUEST: u32 = 1000u32;
pub const DISPID_XMLSERIALIZER: u32 = 1000u32;
pub const DISPID_XOBJ_EXPANDO: u32 = 72536u32;
pub const DISPID_XOBJ_ORDINAL: u32 = 73536u32;
#[repr(transparent)]
pub struct DISPLAY_BREAK(pub i32);
pub const DISPLAY_BREAK_None: DISPLAY_BREAK = DISPLAY_BREAK(0i32);
pub const DISPLAY_BREAK_Block: DISPLAY_BREAK = DISPLAY_BREAK(1i32);
pub const DISPLAY_BREAK_Break: DISPLAY_BREAK = DISPLAY_BREAK(2i32);
pub const DISPLAY_BREAK_Max: DISPLAY_BREAK = DISPLAY_BREAK(2147483647i32);
impl ::core::marker::Copy for DISPLAY_BREAK {}
impl ::core::clone::Clone for DISPLAY_BREAK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPLAY_GRAVITY(pub i32);
pub const DISPLAY_GRAVITY_PreviousLine: DISPLAY_GRAVITY = DISPLAY_GRAVITY(1i32);
pub const DISPLAY_GRAVITY_NextLine: DISPLAY_GRAVITY = DISPLAY_GRAVITY(2i32);
pub const DISPLAY_GRAVITY_Max: DISPLAY_GRAVITY = DISPLAY_GRAVITY(2147483647i32);
impl ::core::marker::Copy for DISPLAY_GRAVITY {}
impl ::core::clone::Clone for DISPLAY_GRAVITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPLAY_MOVEUNIT(pub i32);
pub const DISPLAY_MOVEUNIT_PreviousLine: DISPLAY_MOVEUNIT = DISPLAY_MOVEUNIT(1i32);
pub const DISPLAY_MOVEUNIT_NextLine: DISPLAY_MOVEUNIT = DISPLAY_MOVEUNIT(2i32);
pub const DISPLAY_MOVEUNIT_CurrentLineStart: DISPLAY_MOVEUNIT = DISPLAY_MOVEUNIT(3i32);
pub const DISPLAY_MOVEUNIT_CurrentLineEnd: DISPLAY_MOVEUNIT = DISPLAY_MOVEUNIT(4i32);
pub const DISPLAY_MOVEUNIT_TopOfWindow: DISPLAY_MOVEUNIT = DISPLAY_MOVEUNIT(5i32);
pub const DISPLAY_MOVEUNIT_BottomOfWindow: DISPLAY_MOVEUNIT = DISPLAY_MOVEUNIT(6i32);
pub const DISPLAY_MOVEUNIT_Max: DISPLAY_MOVEUNIT = DISPLAY_MOVEUNIT(2147483647i32);
impl ::core::marker::Copy for DISPLAY_MOVEUNIT {}
impl ::core::clone::Clone for DISPLAY_MOVEUNIT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DLCTL_BGSOUNDS: u32 = 64u32;
pub const DLCTL_DLIMAGES: u32 = 16u32;
pub const DLCTL_DOWNLOADONLY: u32 = 2048u32;
pub const DLCTL_FORCEOFFLINE: u32 = 268435456u32;
pub const DLCTL_NOFRAMES: u32 = 524288u32;
pub const DLCTL_NO_BEHAVIORS: u32 = 32768u32;
pub const DLCTL_NO_CLIENTPULL: u32 = 536870912u32;
pub const DLCTL_NO_DLACTIVEXCTLS: u32 = 1024u32;
pub const DLCTL_NO_FRAMEDOWNLOAD: u32 = 4096u32;
pub const DLCTL_NO_JAVA: u32 = 256u32;
pub const DLCTL_NO_METACHARSET: u32 = 65536u32;
pub const DLCTL_NO_RUNACTIVEXCTLS: u32 = 512u32;
pub const DLCTL_NO_SCRIPTS: u32 = 128u32;
pub const DLCTL_OFFLINE: u32 = 2147483648u32;
pub const DLCTL_OFFLINEIFNOTCONNECTED: u32 = 2147483648u32;
pub const DLCTL_PRAGMA_NO_CACHE: u32 = 16384u32;
pub const DLCTL_RESYNCHRONIZE: u32 = 8192u32;
pub const DLCTL_SILENT: u32 = 1073741824u32;
pub const DLCTL_URL_ENCODING_DISABLE_UTF8: u32 = 131072u32;
pub const DLCTL_URL_ENCODING_ENABLE_UTF8: u32 = 262144u32;
pub const DLCTL_VIDEOS: u32 = 32u32;
#[repr(transparent)]
pub struct DOCHOSTUIDBLCLK(pub i32);
pub const DOCHOSTUIDBLCLK_DEFAULT: DOCHOSTUIDBLCLK = DOCHOSTUIDBLCLK(0i32);
pub const DOCHOSTUIDBLCLK_SHOWPROPERTIES: DOCHOSTUIDBLCLK = DOCHOSTUIDBLCLK(1i32);
pub const DOCHOSTUIDBLCLK_SHOWCODE: DOCHOSTUIDBLCLK = DOCHOSTUIDBLCLK(2i32);
impl ::core::marker::Copy for DOCHOSTUIDBLCLK {}
impl ::core::clone::Clone for DOCHOSTUIDBLCLK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DOCHOSTUIFLAG(pub i32);
pub const DOCHOSTUIFLAG_DIALOG: DOCHOSTUIFLAG = DOCHOSTUIFLAG(1i32);
pub const DOCHOSTUIFLAG_DISABLE_HELP_MENU: DOCHOSTUIFLAG = DOCHOSTUIFLAG(2i32);
pub const DOCHOSTUIFLAG_NO3DBORDER: DOCHOSTUIFLAG = DOCHOSTUIFLAG(4i32);
pub const DOCHOSTUIFLAG_SCROLL_NO: DOCHOSTUIFLAG = DOCHOSTUIFLAG(8i32);
pub const DOCHOSTUIFLAG_DISABLE_SCRIPT_INACTIVE: DOCHOSTUIFLAG = DOCHOSTUIFLAG(16i32);
pub const DOCHOSTUIFLAG_OPENNEWWIN: DOCHOSTUIFLAG = DOCHOSTUIFLAG(32i32);
pub const DOCHOSTUIFLAG_DISABLE_OFFSCREEN: DOCHOSTUIFLAG = DOCHOSTUIFLAG(64i32);
pub const DOCHOSTUIFLAG_FLAT_SCROLLBAR: DOCHOSTUIFLAG = DOCHOSTUIFLAG(128i32);
pub const DOCHOSTUIFLAG_DIV_BLOCKDEFAULT: DOCHOSTUIFLAG = DOCHOSTUIFLAG(256i32);
pub const DOCHOSTUIFLAG_ACTIVATE_CLIENTHIT_ONLY: DOCHOSTUIFLAG = DOCHOSTUIFLAG(512i32);
pub const DOCHOSTUIFLAG_OVERRIDEBEHAVIORFACTORY: DOCHOSTUIFLAG = DOCHOSTUIFLAG(1024i32);
pub const DOCHOSTUIFLAG_CODEPAGELINKEDFONTS: DOCHOSTUIFLAG = DOCHOSTUIFLAG(2048i32);
pub const DOCHOSTUIFLAG_URL_ENCODING_DISABLE_UTF8: DOCHOSTUIFLAG = DOCHOSTUIFLAG(4096i32);
pub const DOCHOSTUIFLAG_URL_ENCODING_ENABLE_UTF8: DOCHOSTUIFLAG = DOCHOSTUIFLAG(8192i32);
pub const DOCHOSTUIFLAG_ENABLE_FORMS_AUTOCOMPLETE: DOCHOSTUIFLAG = DOCHOSTUIFLAG(16384i32);
pub const DOCHOSTUIFLAG_ENABLE_INPLACE_NAVIGATION: DOCHOSTUIFLAG = DOCHOSTUIFLAG(65536i32);
pub const DOCHOSTUIFLAG_IME_ENABLE_RECONVERSION: DOCHOSTUIFLAG = DOCHOSTUIFLAG(131072i32);
pub const DOCHOSTUIFLAG_THEME: DOCHOSTUIFLAG = DOCHOSTUIFLAG(262144i32);
pub const DOCHOSTUIFLAG_NOTHEME: DOCHOSTUIFLAG = DOCHOSTUIFLAG(524288i32);
pub const DOCHOSTUIFLAG_NOPICS: DOCHOSTUIFLAG = DOCHOSTUIFLAG(1048576i32);
pub const DOCHOSTUIFLAG_NO3DOUTERBORDER: DOCHOSTUIFLAG = DOCHOSTUIFLAG(2097152i32);
pub const DOCHOSTUIFLAG_DISABLE_EDIT_NS_FIXUP: DOCHOSTUIFLAG = DOCHOSTUIFLAG(4194304i32);
pub const DOCHOSTUIFLAG_LOCAL_MACHINE_ACCESS_CHECK: DOCHOSTUIFLAG = DOCHOSTUIFLAG(8388608i32);
pub const DOCHOSTUIFLAG_DISABLE_UNTRUSTEDPROTOCOL: DOCHOSTUIFLAG = DOCHOSTUIFLAG(16777216i32);
pub const DOCHOSTUIFLAG_HOST_NAVIGATES: DOCHOSTUIFLAG = DOCHOSTUIFLAG(33554432i32);
pub const DOCHOSTUIFLAG_ENABLE_REDIRECT_NOTIFICATION: DOCHOSTUIFLAG = DOCHOSTUIFLAG(67108864i32);
pub const DOCHOSTUIFLAG_USE_WINDOWLESS_SELECTCONTROL: DOCHOSTUIFLAG = DOCHOSTUIFLAG(134217728i32);
pub const DOCHOSTUIFLAG_USE_WINDOWED_SELECTCONTROL: DOCHOSTUIFLAG = DOCHOSTUIFLAG(268435456i32);
pub const DOCHOSTUIFLAG_ENABLE_ACTIVEX_INACTIVATE_MODE: DOCHOSTUIFLAG = DOCHOSTUIFLAG(536870912i32);
pub const DOCHOSTUIFLAG_DPI_AWARE: DOCHOSTUIFLAG = DOCHOSTUIFLAG(1073741824i32);
impl ::core::marker::Copy for DOCHOSTUIFLAG {}
impl ::core::clone::Clone for DOCHOSTUIFLAG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOCHOSTUIINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwDoubleClick: u32,
    pub pchHostCss: super::super::Foundation::PWSTR,
    pub pchHostNS: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOCHOSTUIINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOCHOSTUIINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DOCHOSTUITYPE(pub i32);
pub const DOCHOSTUITYPE_BROWSE: DOCHOSTUITYPE = DOCHOSTUITYPE(0i32);
pub const DOCHOSTUITYPE_AUTHOR: DOCHOSTUITYPE = DOCHOSTUITYPE(1i32);
impl ::core::marker::Copy for DOCHOSTUITYPE {}
impl ::core::clone::Clone for DOCHOSTUITYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DOMBeforeUnloadEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616676, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMChildrenCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612138, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMCloseEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616832, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMCompositionEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616537, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMCustomEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616543, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMDocumentType: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616633, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMDragEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616674, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810615995, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMException: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616620, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMFocusEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616525, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMKeyboardEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616535, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMMSAnimationEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616760, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMMSManipulationEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616855, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMMSTransitionEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616758, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMMessageEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616609, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMMouseEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616527, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMMouseWheelEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616529, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMMutationEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616539, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMParser: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616706, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMParserFactory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616708, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMProcessingInstruction: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616643, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMProgressEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616607, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMSiteModeEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616678, data2: 39094, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMStorageEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616611, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMTextEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616533, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMUIEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616523, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const DOMWheelEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616531, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct DOM_EVENT_PHASE(pub i32);
pub const DEP_CAPTURING_PHASE: DOM_EVENT_PHASE = DOM_EVENT_PHASE(1i32);
pub const DEP_AT_TARGET: DOM_EVENT_PHASE = DOM_EVENT_PHASE(2i32);
pub const DEP_BUBBLING_PHASE: DOM_EVENT_PHASE = DOM_EVENT_PHASE(3i32);
pub const DOM_EVENT_PHASE_Max: DOM_EVENT_PHASE = DOM_EVENT_PHASE(2147483647i32);
impl ::core::marker::Copy for DOM_EVENT_PHASE {}
impl ::core::clone::Clone for DOM_EVENT_PHASE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DWebBridgeEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispApplicationCache(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispCEventObj(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispCPlugins(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispCPrintManagerTemplatePrinter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispCanvasGradient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispCanvasImageData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispCanvasPattern(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispCanvasRenderingContext2D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispCanvasTextMetrics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMBeforeUnloadEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMChildrenCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMCloseEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMCompositionEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMCustomEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMDocumentType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMDragEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMFocusEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMKeyboardEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMMSAnimationEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMMSManipulationEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMMSTransitionEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMMessageEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMMouseEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMMouseWheelEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMMutationEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMParser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMProcessingInstruction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMProgressEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMSiteModeEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMStorageEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMTextEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMUIEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispDOMWheelEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispEventException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTCAttachBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTCDefaultDispatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTCDescBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTCEventBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTCMethodBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTCPropertyBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLAnchorElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLAppBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLAreaElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLAreasCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLAttributeCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLAudioElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLBGsound(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLBRElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLBaseElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLBaseFontElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLBlockElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLBody(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLButtonElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLCSSImportRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLCSSMediaList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLCSSMediaRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLCSSNamespaceRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLCSSRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLCSSStyleDeclaration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLCanvasElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLCommentElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLCurrentStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLDDElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLDListElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLDOMAttribute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLDOMImplementation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLDOMRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLDOMTextNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLDTElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLDefaults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLDivElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLDivPosition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLDocumentCompatibleInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLDocumentCompatibleInfoCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLElementCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLEmbed(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLFieldSetElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLFontElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLFormElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLFrameBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLFrameElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLFrameSetSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLGenericElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLHRElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLHeadElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLHeaderElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLHistory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLHtmlElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLIFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLImg(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLInputElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLIsIndexElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLLIElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLLabelElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLLegendElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLLinkElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLListElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLMSCSSKeyframeRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLMSCSSKeyframesRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLMapElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLMarqueeElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLMediaElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLMediaError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLMetaElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLNamespace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLNamespaceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLNavigator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLNextIdElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLNoShowElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLOListElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLObjectElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLOptionElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLParaElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLParamElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLPerformance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLPerformanceNavigation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLPerformanceTiming(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLPhraseElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLPopup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLProgressElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLRenderStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLRichtextElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLRuleStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLScreen(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLScriptElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLSelectElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLSemanticElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLSourceElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLSpanElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLSpanFlow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLStorage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLStyleElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLStyleFontFace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLStyleMedia(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLStyleSheet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLStyleSheetPage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLStyleSheetPagesCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLStyleSheetRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLStyleSheetRulesAppliedCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLStyleSheetRulesCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLStyleSheetsCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLTable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLTableCaption(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLTableCell(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLTableCol(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLTableRow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLTableSection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLTextAreaElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLTextElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLTimeRanges(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLTitleElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLUListElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLUnknownElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLUrnCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLVideoElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLW3CComputedStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLWindow2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLWindowProxy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLWndOptionElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLWndSelectElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispHTMLXMLHttpRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispIHTMLInputButtonElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispIHTMLInputFileElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispIHTMLInputImage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispIHTMLInputTextElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispIHTMLOptionButtonElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispNodeIterator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispRangeException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispRulesApplied(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispRulesAppliedCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGAElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGCircleElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGClipPathElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGDefsElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGDescElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGElementInstance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGElementInstanceList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGEllipseElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGGElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGGradientElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGImageElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGLineElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGLinearGradientElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGMarkerElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGMaskElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGMetadataElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegArcAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegArcRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegClosePath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegCurvetoCubicAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegCurvetoCubicRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegCurvetoCubicSmoothAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegCurvetoCubicSmoothRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegCurvetoQuadraticAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegCurvetoQuadraticRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegCurvetoQuadraticSmoothAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegCurvetoQuadraticSmoothRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegLinetoAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegLinetoHorizontalAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegLinetoHorizontalRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegLinetoRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegLinetoVerticalAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegLinetoVerticalRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegMovetoAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPathSegMovetoRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPatternElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPolygonElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGPolylineElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGRadialGradientElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGRectElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGSVGElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGScriptElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGStopElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGStyleElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGSwitchElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGSymbolElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGTSpanElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGTextContentElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGTextElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGTextPathElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGTextPositioningElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGTitleElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGUseElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGViewElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispSVGZoomEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispStaticNodeList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispTreeWalker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispWebGeocoordinates(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispWebGeolocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispWebGeoposition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispWebGeopositionError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispXDomainRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispXMLHttpRequestEventTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispXMLSerializer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DomConstructor(pub i32);
pub const DomConstructorObject: DomConstructor = DomConstructor(0i32);
pub const DomConstructorAttr: DomConstructor = DomConstructor(1i32);
pub const DomConstructorBehaviorUrnsCollection: DomConstructor = DomConstructor(2i32);
pub const DomConstructorBookmarkCollection: DomConstructor = DomConstructor(3i32);
pub const DomConstructorCompatibleInfo: DomConstructor = DomConstructor(4i32);
pub const DomConstructorCompatibleInfoCollection: DomConstructor = DomConstructor(5i32);
pub const DomConstructorControlRangeCollection: DomConstructor = DomConstructor(6i32);
pub const DomConstructorCSSCurrentStyleDeclaration: DomConstructor = DomConstructor(7i32);
pub const DomConstructorCSSRuleList: DomConstructor = DomConstructor(8i32);
pub const DomConstructorCSSRuleStyleDeclaration: DomConstructor = DomConstructor(9i32);
pub const DomConstructorCSSStyleDeclaration: DomConstructor = DomConstructor(10i32);
pub const DomConstructorCSSStyleRule: DomConstructor = DomConstructor(11i32);
pub const DomConstructorCSSStyleSheet: DomConstructor = DomConstructor(12i32);
pub const DomConstructorDataTransfer: DomConstructor = DomConstructor(13i32);
pub const DomConstructorDOMImplementation: DomConstructor = DomConstructor(14i32);
pub const DomConstructorElement: DomConstructor = DomConstructor(15i32);
pub const DomConstructorEvent: DomConstructor = DomConstructor(16i32);
pub const DomConstructorHistory: DomConstructor = DomConstructor(17i32);
pub const DomConstructorHTCElementBehaviorDefaults: DomConstructor = DomConstructor(18i32);
pub const DomConstructorHTMLAnchorElement: DomConstructor = DomConstructor(19i32);
pub const DomConstructorHTMLAreaElement: DomConstructor = DomConstructor(20i32);
pub const DomConstructorHTMLAreasCollection: DomConstructor = DomConstructor(21i32);
pub const DomConstructorHTMLBaseElement: DomConstructor = DomConstructor(22i32);
pub const DomConstructorHTMLBaseFontElement: DomConstructor = DomConstructor(23i32);
pub const DomConstructorHTMLBGSoundElement: DomConstructor = DomConstructor(24i32);
pub const DomConstructorHTMLBlockElement: DomConstructor = DomConstructor(25i32);
pub const DomConstructorHTMLBodyElement: DomConstructor = DomConstructor(26i32);
pub const DomConstructorHTMLBRElement: DomConstructor = DomConstructor(27i32);
pub const DomConstructorHTMLButtonElement: DomConstructor = DomConstructor(28i32);
pub const DomConstructorHTMLCollection: DomConstructor = DomConstructor(29i32);
pub const DomConstructorHTMLCommentElement: DomConstructor = DomConstructor(30i32);
pub const DomConstructorHTMLDDElement: DomConstructor = DomConstructor(31i32);
pub const DomConstructorHTMLDivElement: DomConstructor = DomConstructor(32i32);
pub const DomConstructorHTMLDocument: DomConstructor = DomConstructor(33i32);
pub const DomConstructorHTMLDListElement: DomConstructor = DomConstructor(34i32);
pub const DomConstructorHTMLDTElement: DomConstructor = DomConstructor(35i32);
pub const DomConstructorHTMLEmbedElement: DomConstructor = DomConstructor(36i32);
pub const DomConstructorHTMLFieldSetElement: DomConstructor = DomConstructor(37i32);
pub const DomConstructorHTMLFontElement: DomConstructor = DomConstructor(38i32);
pub const DomConstructorHTMLFormElement: DomConstructor = DomConstructor(39i32);
pub const DomConstructorHTMLFrameElement: DomConstructor = DomConstructor(40i32);
pub const DomConstructorHTMLFrameSetElement: DomConstructor = DomConstructor(41i32);
pub const DomConstructorHTMLGenericElement: DomConstructor = DomConstructor(42i32);
pub const DomConstructorHTMLHeadElement: DomConstructor = DomConstructor(43i32);
pub const DomConstructorHTMLHeadingElement: DomConstructor = DomConstructor(44i32);
pub const DomConstructorHTMLHRElement: DomConstructor = DomConstructor(45i32);
pub const DomConstructorHTMLHtmlElement: DomConstructor = DomConstructor(46i32);
pub const DomConstructorHTMLIFrameElement: DomConstructor = DomConstructor(47i32);
pub const DomConstructorHTMLImageElement: DomConstructor = DomConstructor(48i32);
pub const DomConstructorHTMLInputElement: DomConstructor = DomConstructor(49i32);
pub const DomConstructorHTMLIsIndexElement: DomConstructor = DomConstructor(50i32);
pub const DomConstructorHTMLLabelElement: DomConstructor = DomConstructor(51i32);
pub const DomConstructorHTMLLegendElement: DomConstructor = DomConstructor(52i32);
pub const DomConstructorHTMLLIElement: DomConstructor = DomConstructor(53i32);
pub const DomConstructorHTMLLinkElement: DomConstructor = DomConstructor(54i32);
pub const DomConstructorHTMLMapElement: DomConstructor = DomConstructor(55i32);
pub const DomConstructorHTMLMarqueeElement: DomConstructor = DomConstructor(56i32);
pub const DomConstructorHTMLMetaElement: DomConstructor = DomConstructor(57i32);
pub const DomConstructorHTMLModelessDialog: DomConstructor = DomConstructor(58i32);
pub const DomConstructorHTMLNamespaceInfo: DomConstructor = DomConstructor(59i32);
pub const DomConstructorHTMLNamespaceInfoCollection: DomConstructor = DomConstructor(60i32);
pub const DomConstructorHTMLNextIdElement: DomConstructor = DomConstructor(61i32);
pub const DomConstructorHTMLNoShowElement: DomConstructor = DomConstructor(62i32);
pub const DomConstructorHTMLObjectElement: DomConstructor = DomConstructor(63i32);
pub const DomConstructorHTMLOListElement: DomConstructor = DomConstructor(64i32);
pub const DomConstructorHTMLOptionElement: DomConstructor = DomConstructor(65i32);
pub const DomConstructorHTMLParagraphElement: DomConstructor = DomConstructor(66i32);
pub const DomConstructorHTMLParamElement: DomConstructor = DomConstructor(67i32);
pub const DomConstructorHTMLPhraseElement: DomConstructor = DomConstructor(68i32);
pub const DomConstructorHTMLPluginsCollection: DomConstructor = DomConstructor(69i32);
pub const DomConstructorHTMLPopup: DomConstructor = DomConstructor(70i32);
pub const DomConstructorHTMLScriptElement: DomConstructor = DomConstructor(71i32);
pub const DomConstructorHTMLSelectElement: DomConstructor = DomConstructor(72i32);
pub const DomConstructorHTMLSpanElement: DomConstructor = DomConstructor(73i32);
pub const DomConstructorHTMLStyleElement: DomConstructor = DomConstructor(74i32);
pub const DomConstructorHTMLTableCaptionElement: DomConstructor = DomConstructor(75i32);
pub const DomConstructorHTMLTableCellElement: DomConstructor = DomConstructor(76i32);
pub const DomConstructorHTMLTableColElement: DomConstructor = DomConstructor(77i32);
pub const DomConstructorHTMLTableElement: DomConstructor = DomConstructor(78i32);
pub const DomConstructorHTMLTableRowElement: DomConstructor = DomConstructor(79i32);
pub const DomConstructorHTMLTableSectionElement: DomConstructor = DomConstructor(80i32);
pub const DomConstructorHTMLTextAreaElement: DomConstructor = DomConstructor(81i32);
pub const DomConstructorHTMLTextElement: DomConstructor = DomConstructor(82i32);
pub const DomConstructorHTMLTitleElement: DomConstructor = DomConstructor(83i32);
pub const DomConstructorHTMLUListElement: DomConstructor = DomConstructor(84i32);
pub const DomConstructorHTMLUnknownElement: DomConstructor = DomConstructor(85i32);
pub const DomConstructorImage: DomConstructor = DomConstructor(86i32);
pub const DomConstructorLocation: DomConstructor = DomConstructor(87i32);
pub const DomConstructorNamedNodeMap: DomConstructor = DomConstructor(88i32);
pub const DomConstructorNavigator: DomConstructor = DomConstructor(89i32);
pub const DomConstructorNodeList: DomConstructor = DomConstructor(90i32);
pub const DomConstructorOption: DomConstructor = DomConstructor(91i32);
pub const DomConstructorScreen: DomConstructor = DomConstructor(92i32);
pub const DomConstructorSelection: DomConstructor = DomConstructor(93i32);
pub const DomConstructorStaticNodeList: DomConstructor = DomConstructor(94i32);
pub const DomConstructorStorage: DomConstructor = DomConstructor(95i32);
pub const DomConstructorStyleSheetList: DomConstructor = DomConstructor(96i32);
pub const DomConstructorStyleSheetPage: DomConstructor = DomConstructor(97i32);
pub const DomConstructorStyleSheetPageList: DomConstructor = DomConstructor(98i32);
pub const DomConstructorText: DomConstructor = DomConstructor(99i32);
pub const DomConstructorTextRange: DomConstructor = DomConstructor(100i32);
pub const DomConstructorTextRangeCollection: DomConstructor = DomConstructor(101i32);
pub const DomConstructorTextRectangle: DomConstructor = DomConstructor(102i32);
pub const DomConstructorTextRectangleList: DomConstructor = DomConstructor(103i32);
pub const DomConstructorWindow: DomConstructor = DomConstructor(104i32);
pub const DomConstructorXDomainRequest: DomConstructor = DomConstructor(105i32);
pub const DomConstructorXMLHttpRequest: DomConstructor = DomConstructor(106i32);
pub const DomConstructorMax: DomConstructor = DomConstructor(107i32);
pub const DomConstructor_Max: DomConstructor = DomConstructor(2147483647i32);
impl ::core::marker::Copy for DomConstructor {}
impl ::core::clone::Clone for DomConstructor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ELEMENTDESCRIPTOR_FLAGS(pub i32);
pub const ELEMENTDESCRIPTORFLAGS_LITERAL: ELEMENTDESCRIPTOR_FLAGS = ELEMENTDESCRIPTOR_FLAGS(1i32);
pub const ELEMENTDESCRIPTORFLAGS_NESTED_LITERAL: ELEMENTDESCRIPTOR_FLAGS = ELEMENTDESCRIPTOR_FLAGS(2i32);
pub const ELEMENTDESCRIPTOR_FLAGS_Max: ELEMENTDESCRIPTOR_FLAGS = ELEMENTDESCRIPTOR_FLAGS(2147483647i32);
impl ::core::marker::Copy for ELEMENTDESCRIPTOR_FLAGS {}
impl ::core::clone::Clone for ELEMENTDESCRIPTOR_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ELEMENTNAMESPACE_FLAGS(pub i32);
pub const ELEMENTNAMESPACEFLAGS_ALLOWANYTAG: ELEMENTNAMESPACE_FLAGS = ELEMENTNAMESPACE_FLAGS(1i32);
pub const ELEMENTNAMESPACEFLAGS_QUERYFORUNKNOWNTAGS: ELEMENTNAMESPACE_FLAGS = ELEMENTNAMESPACE_FLAGS(2i32);
pub const ELEMENTNAMESPACE_FLAGS_Max: ELEMENTNAMESPACE_FLAGS = ELEMENTNAMESPACE_FLAGS(2147483647i32);
impl ::core::marker::Copy for ELEMENTNAMESPACE_FLAGS {}
impl ::core::clone::Clone for ELEMENTNAMESPACE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ELEMENT_ADJACENCY(pub i32);
pub const ELEM_ADJ_BeforeBegin: ELEMENT_ADJACENCY = ELEMENT_ADJACENCY(0i32);
pub const ELEM_ADJ_AfterBegin: ELEMENT_ADJACENCY = ELEMENT_ADJACENCY(1i32);
pub const ELEM_ADJ_BeforeEnd: ELEMENT_ADJACENCY = ELEMENT_ADJACENCY(2i32);
pub const ELEM_ADJ_AfterEnd: ELEMENT_ADJACENCY = ELEMENT_ADJACENCY(3i32);
pub const ELEMENT_ADJACENCY_Max: ELEMENT_ADJACENCY = ELEMENT_ADJACENCY(2147483647i32);
impl ::core::marker::Copy for ELEMENT_ADJACENCY {}
impl ::core::clone::Clone for ELEMENT_ADJACENCY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ELEMENT_CORNER(pub i32);
pub const ELEMENT_CORNER_NONE: ELEMENT_CORNER = ELEMENT_CORNER(0i32);
pub const ELEMENT_CORNER_TOP: ELEMENT_CORNER = ELEMENT_CORNER(1i32);
pub const ELEMENT_CORNER_LEFT: ELEMENT_CORNER = ELEMENT_CORNER(2i32);
pub const ELEMENT_CORNER_BOTTOM: ELEMENT_CORNER = ELEMENT_CORNER(3i32);
pub const ELEMENT_CORNER_RIGHT: ELEMENT_CORNER = ELEMENT_CORNER(4i32);
pub const ELEMENT_CORNER_TOPLEFT: ELEMENT_CORNER = ELEMENT_CORNER(5i32);
pub const ELEMENT_CORNER_TOPRIGHT: ELEMENT_CORNER = ELEMENT_CORNER(6i32);
pub const ELEMENT_CORNER_BOTTOMLEFT: ELEMENT_CORNER = ELEMENT_CORNER(7i32);
pub const ELEMENT_CORNER_BOTTOMRIGHT: ELEMENT_CORNER = ELEMENT_CORNER(8i32);
pub const ELEMENT_CORNER_Max: ELEMENT_CORNER = ELEMENT_CORNER(2147483647i32);
impl ::core::marker::Copy for ELEMENT_CORNER {}
impl ::core::clone::Clone for ELEMENT_CORNER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ELEMENT_TAG_ID(pub i32);
pub const TAGID_NULL: ELEMENT_TAG_ID = ELEMENT_TAG_ID(0i32);
pub const TAGID_UNKNOWN: ELEMENT_TAG_ID = ELEMENT_TAG_ID(1i32);
pub const TAGID_A: ELEMENT_TAG_ID = ELEMENT_TAG_ID(2i32);
pub const TAGID_ACRONYM: ELEMENT_TAG_ID = ELEMENT_TAG_ID(3i32);
pub const TAGID_ADDRESS: ELEMENT_TAG_ID = ELEMENT_TAG_ID(4i32);
pub const TAGID_APPLET: ELEMENT_TAG_ID = ELEMENT_TAG_ID(5i32);
pub const TAGID_AREA: ELEMENT_TAG_ID = ELEMENT_TAG_ID(6i32);
pub const TAGID_B: ELEMENT_TAG_ID = ELEMENT_TAG_ID(7i32);
pub const TAGID_BASE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(8i32);
pub const TAGID_BASEFONT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(9i32);
pub const TAGID_BDO: ELEMENT_TAG_ID = ELEMENT_TAG_ID(10i32);
pub const TAGID_BGSOUND: ELEMENT_TAG_ID = ELEMENT_TAG_ID(11i32);
pub const TAGID_BIG: ELEMENT_TAG_ID = ELEMENT_TAG_ID(12i32);
pub const TAGID_BLINK: ELEMENT_TAG_ID = ELEMENT_TAG_ID(13i32);
pub const TAGID_BLOCKQUOTE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(14i32);
pub const TAGID_BODY: ELEMENT_TAG_ID = ELEMENT_TAG_ID(15i32);
pub const TAGID_BR: ELEMENT_TAG_ID = ELEMENT_TAG_ID(16i32);
pub const TAGID_BUTTON: ELEMENT_TAG_ID = ELEMENT_TAG_ID(17i32);
pub const TAGID_CAPTION: ELEMENT_TAG_ID = ELEMENT_TAG_ID(18i32);
pub const TAGID_CENTER: ELEMENT_TAG_ID = ELEMENT_TAG_ID(19i32);
pub const TAGID_CITE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(20i32);
pub const TAGID_CODE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(21i32);
pub const TAGID_COL: ELEMENT_TAG_ID = ELEMENT_TAG_ID(22i32);
pub const TAGID_COLGROUP: ELEMENT_TAG_ID = ELEMENT_TAG_ID(23i32);
pub const TAGID_COMMENT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(24i32);
pub const TAGID_COMMENT_RAW: ELEMENT_TAG_ID = ELEMENT_TAG_ID(25i32);
pub const TAGID_DD: ELEMENT_TAG_ID = ELEMENT_TAG_ID(26i32);
pub const TAGID_DEL: ELEMENT_TAG_ID = ELEMENT_TAG_ID(27i32);
pub const TAGID_DFN: ELEMENT_TAG_ID = ELEMENT_TAG_ID(28i32);
pub const TAGID_DIR: ELEMENT_TAG_ID = ELEMENT_TAG_ID(29i32);
pub const TAGID_DIV: ELEMENT_TAG_ID = ELEMENT_TAG_ID(30i32);
pub const TAGID_DL: ELEMENT_TAG_ID = ELEMENT_TAG_ID(31i32);
pub const TAGID_DT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(32i32);
pub const TAGID_EM: ELEMENT_TAG_ID = ELEMENT_TAG_ID(33i32);
pub const TAGID_EMBED: ELEMENT_TAG_ID = ELEMENT_TAG_ID(34i32);
pub const TAGID_FIELDSET: ELEMENT_TAG_ID = ELEMENT_TAG_ID(35i32);
pub const TAGID_FONT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(36i32);
pub const TAGID_FORM: ELEMENT_TAG_ID = ELEMENT_TAG_ID(37i32);
pub const TAGID_FRAME: ELEMENT_TAG_ID = ELEMENT_TAG_ID(38i32);
pub const TAGID_FRAMESET: ELEMENT_TAG_ID = ELEMENT_TAG_ID(39i32);
pub const TAGID_GENERIC: ELEMENT_TAG_ID = ELEMENT_TAG_ID(40i32);
pub const TAGID_H1: ELEMENT_TAG_ID = ELEMENT_TAG_ID(41i32);
pub const TAGID_H2: ELEMENT_TAG_ID = ELEMENT_TAG_ID(42i32);
pub const TAGID_H3: ELEMENT_TAG_ID = ELEMENT_TAG_ID(43i32);
pub const TAGID_H4: ELEMENT_TAG_ID = ELEMENT_TAG_ID(44i32);
pub const TAGID_H5: ELEMENT_TAG_ID = ELEMENT_TAG_ID(45i32);
pub const TAGID_H6: ELEMENT_TAG_ID = ELEMENT_TAG_ID(46i32);
pub const TAGID_HEAD: ELEMENT_TAG_ID = ELEMENT_TAG_ID(47i32);
pub const TAGID_HR: ELEMENT_TAG_ID = ELEMENT_TAG_ID(48i32);
pub const TAGID_HTML: ELEMENT_TAG_ID = ELEMENT_TAG_ID(49i32);
pub const TAGID_I: ELEMENT_TAG_ID = ELEMENT_TAG_ID(50i32);
pub const TAGID_IFRAME: ELEMENT_TAG_ID = ELEMENT_TAG_ID(51i32);
pub const TAGID_IMG: ELEMENT_TAG_ID = ELEMENT_TAG_ID(52i32);
pub const TAGID_INPUT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(53i32);
pub const TAGID_INS: ELEMENT_TAG_ID = ELEMENT_TAG_ID(54i32);
pub const TAGID_KBD: ELEMENT_TAG_ID = ELEMENT_TAG_ID(55i32);
pub const TAGID_LABEL: ELEMENT_TAG_ID = ELEMENT_TAG_ID(56i32);
pub const TAGID_LEGEND: ELEMENT_TAG_ID = ELEMENT_TAG_ID(57i32);
pub const TAGID_LI: ELEMENT_TAG_ID = ELEMENT_TAG_ID(58i32);
pub const TAGID_LINK: ELEMENT_TAG_ID = ELEMENT_TAG_ID(59i32);
pub const TAGID_LISTING: ELEMENT_TAG_ID = ELEMENT_TAG_ID(60i32);
pub const TAGID_MAP: ELEMENT_TAG_ID = ELEMENT_TAG_ID(61i32);
pub const TAGID_MARQUEE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(62i32);
pub const TAGID_MENU: ELEMENT_TAG_ID = ELEMENT_TAG_ID(63i32);
pub const TAGID_META: ELEMENT_TAG_ID = ELEMENT_TAG_ID(64i32);
pub const TAGID_NEXTID: ELEMENT_TAG_ID = ELEMENT_TAG_ID(65i32);
pub const TAGID_NOBR: ELEMENT_TAG_ID = ELEMENT_TAG_ID(66i32);
pub const TAGID_NOEMBED: ELEMENT_TAG_ID = ELEMENT_TAG_ID(67i32);
pub const TAGID_NOFRAMES: ELEMENT_TAG_ID = ELEMENT_TAG_ID(68i32);
pub const TAGID_NOSCRIPT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(69i32);
pub const TAGID_OBJECT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(70i32);
pub const TAGID_OL: ELEMENT_TAG_ID = ELEMENT_TAG_ID(71i32);
pub const TAGID_OPTION: ELEMENT_TAG_ID = ELEMENT_TAG_ID(72i32);
pub const TAGID_P: ELEMENT_TAG_ID = ELEMENT_TAG_ID(73i32);
pub const TAGID_PARAM: ELEMENT_TAG_ID = ELEMENT_TAG_ID(74i32);
pub const TAGID_PLAINTEXT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(75i32);
pub const TAGID_PRE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(76i32);
pub const TAGID_Q: ELEMENT_TAG_ID = ELEMENT_TAG_ID(77i32);
pub const TAGID_RP: ELEMENT_TAG_ID = ELEMENT_TAG_ID(78i32);
pub const TAGID_RT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(79i32);
pub const TAGID_RUBY: ELEMENT_TAG_ID = ELEMENT_TAG_ID(80i32);
pub const TAGID_S: ELEMENT_TAG_ID = ELEMENT_TAG_ID(81i32);
pub const TAGID_SAMP: ELEMENT_TAG_ID = ELEMENT_TAG_ID(82i32);
pub const TAGID_SCRIPT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(83i32);
pub const TAGID_SELECT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(84i32);
pub const TAGID_SMALL: ELEMENT_TAG_ID = ELEMENT_TAG_ID(85i32);
pub const TAGID_SPAN: ELEMENT_TAG_ID = ELEMENT_TAG_ID(86i32);
pub const TAGID_STRIKE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(87i32);
pub const TAGID_STRONG: ELEMENT_TAG_ID = ELEMENT_TAG_ID(88i32);
pub const TAGID_STYLE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(89i32);
pub const TAGID_SUB: ELEMENT_TAG_ID = ELEMENT_TAG_ID(90i32);
pub const TAGID_SUP: ELEMENT_TAG_ID = ELEMENT_TAG_ID(91i32);
pub const TAGID_TABLE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(92i32);
pub const TAGID_TBODY: ELEMENT_TAG_ID = ELEMENT_TAG_ID(93i32);
pub const TAGID_TC: ELEMENT_TAG_ID = ELEMENT_TAG_ID(94i32);
pub const TAGID_TD: ELEMENT_TAG_ID = ELEMENT_TAG_ID(95i32);
pub const TAGID_TEXTAREA: ELEMENT_TAG_ID = ELEMENT_TAG_ID(96i32);
pub const TAGID_TFOOT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(97i32);
pub const TAGID_TH: ELEMENT_TAG_ID = ELEMENT_TAG_ID(98i32);
pub const TAGID_THEAD: ELEMENT_TAG_ID = ELEMENT_TAG_ID(99i32);
pub const TAGID_TITLE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(100i32);
pub const TAGID_TR: ELEMENT_TAG_ID = ELEMENT_TAG_ID(101i32);
pub const TAGID_TT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(102i32);
pub const TAGID_U: ELEMENT_TAG_ID = ELEMENT_TAG_ID(103i32);
pub const TAGID_UL: ELEMENT_TAG_ID = ELEMENT_TAG_ID(104i32);
pub const TAGID_VAR: ELEMENT_TAG_ID = ELEMENT_TAG_ID(105i32);
pub const TAGID_WBR: ELEMENT_TAG_ID = ELEMENT_TAG_ID(106i32);
pub const TAGID_XMP: ELEMENT_TAG_ID = ELEMENT_TAG_ID(107i32);
pub const TAGID_ROOT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(108i32);
pub const TAGID_OPTGROUP: ELEMENT_TAG_ID = ELEMENT_TAG_ID(109i32);
pub const TAGID_ABBR: ELEMENT_TAG_ID = ELEMENT_TAG_ID(110i32);
pub const TAGID_SVG_A: ELEMENT_TAG_ID = ELEMENT_TAG_ID(111i32);
pub const TAGID_SVG_ALTGLYPH: ELEMENT_TAG_ID = ELEMENT_TAG_ID(112i32);
pub const TAGID_SVG_ALTGLYPHDEF: ELEMENT_TAG_ID = ELEMENT_TAG_ID(113i32);
pub const TAGID_SVG_ALTGLYPHITEM: ELEMENT_TAG_ID = ELEMENT_TAG_ID(114i32);
pub const TAGID_SVG_ANIMATE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(115i32);
pub const TAGID_SVG_ANIMATECOLOR: ELEMENT_TAG_ID = ELEMENT_TAG_ID(116i32);
pub const TAGID_SVG_ANIMATEMOTION: ELEMENT_TAG_ID = ELEMENT_TAG_ID(117i32);
pub const TAGID_SVG_ANIMATETRANSFORM: ELEMENT_TAG_ID = ELEMENT_TAG_ID(118i32);
pub const TAGID_SVG_CIRCLE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(119i32);
pub const TAGID_SVG_CLIPPATH: ELEMENT_TAG_ID = ELEMENT_TAG_ID(120i32);
pub const TAGID_SVG_COLOR_PROFILE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(121i32);
pub const TAGID_SVG_CURSOR: ELEMENT_TAG_ID = ELEMENT_TAG_ID(122i32);
pub const TAGID_SVG_DEFINITION_SRC: ELEMENT_TAG_ID = ELEMENT_TAG_ID(123i32);
pub const TAGID_SVG_DEFS: ELEMENT_TAG_ID = ELEMENT_TAG_ID(124i32);
pub const TAGID_SVG_DESC: ELEMENT_TAG_ID = ELEMENT_TAG_ID(125i32);
pub const TAGID_SVG_ELLIPSE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(126i32);
pub const TAGID_SVG_FEBLEND: ELEMENT_TAG_ID = ELEMENT_TAG_ID(127i32);
pub const TAGID_SVG_FECOLORMATRIX: ELEMENT_TAG_ID = ELEMENT_TAG_ID(128i32);
pub const TAGID_SVG_FECOMPONENTTRANSFER: ELEMENT_TAG_ID = ELEMENT_TAG_ID(129i32);
pub const TAGID_SVG_FECOMPOSITE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(130i32);
pub const TAGID_SVG_FECONVOLVEMATRIX: ELEMENT_TAG_ID = ELEMENT_TAG_ID(131i32);
pub const TAGID_SVG_FEDIFFUSELIGHTING: ELEMENT_TAG_ID = ELEMENT_TAG_ID(132i32);
pub const TAGID_SVG_FEDISPLACEMENTMAP: ELEMENT_TAG_ID = ELEMENT_TAG_ID(133i32);
pub const TAGID_SVG_FEDISTANTLIGHT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(134i32);
pub const TAGID_SVG_FEFLOOD: ELEMENT_TAG_ID = ELEMENT_TAG_ID(135i32);
pub const TAGID_SVG_FEFUNCA: ELEMENT_TAG_ID = ELEMENT_TAG_ID(136i32);
pub const TAGID_SVG_FEFUNCB: ELEMENT_TAG_ID = ELEMENT_TAG_ID(137i32);
pub const TAGID_SVG_FEFUNCG: ELEMENT_TAG_ID = ELEMENT_TAG_ID(138i32);
pub const TAGID_SVG_FEFUNCR: ELEMENT_TAG_ID = ELEMENT_TAG_ID(139i32);
pub const TAGID_SVG_FEGAUSSIANBLUR: ELEMENT_TAG_ID = ELEMENT_TAG_ID(140i32);
pub const TAGID_SVG_FEIMAGE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(141i32);
pub const TAGID_SVG_FEMERGE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(142i32);
pub const TAGID_SVG_FEMERGENODE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(143i32);
pub const TAGID_SVG_FEMORPHOLOGY: ELEMENT_TAG_ID = ELEMENT_TAG_ID(144i32);
pub const TAGID_SVG_FEOFFSET: ELEMENT_TAG_ID = ELEMENT_TAG_ID(145i32);
pub const TAGID_SVG_FEPOINTLIGHT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(146i32);
pub const TAGID_SVG_FESPECULARLIGHTING: ELEMENT_TAG_ID = ELEMENT_TAG_ID(147i32);
pub const TAGID_SVG_FESPOTLIGHT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(148i32);
pub const TAGID_SVG_FETILE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(149i32);
pub const TAGID_SVG_FETURBULENCE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(150i32);
pub const TAGID_SVG_FILTER: ELEMENT_TAG_ID = ELEMENT_TAG_ID(151i32);
pub const TAGID_SVG_FONT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(152i32);
pub const TAGID_SVG_FONT_FACE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(153i32);
pub const TAGID_SVG_FONT_FACE_FORMAT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(154i32);
pub const TAGID_SVG_FONT_FACE_NAME: ELEMENT_TAG_ID = ELEMENT_TAG_ID(155i32);
pub const TAGID_SVG_FONT_FACE_SRC: ELEMENT_TAG_ID = ELEMENT_TAG_ID(156i32);
pub const TAGID_SVG_FONT_FACE_URI: ELEMENT_TAG_ID = ELEMENT_TAG_ID(157i32);
pub const TAGID_SVG_FOREIGNOBJECT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(158i32);
pub const TAGID_SVG_G: ELEMENT_TAG_ID = ELEMENT_TAG_ID(159i32);
pub const TAGID_SVG_GLYPH: ELEMENT_TAG_ID = ELEMENT_TAG_ID(160i32);
pub const TAGID_SVG_GLYPHREF: ELEMENT_TAG_ID = ELEMENT_TAG_ID(161i32);
pub const TAGID_SVG_HKERN: ELEMENT_TAG_ID = ELEMENT_TAG_ID(162i32);
pub const TAGID_SVG_IMAGE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(163i32);
pub const TAGID_SVG_LINE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(164i32);
pub const TAGID_SVG_LINEARGRADIENT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(165i32);
pub const TAGID_SVG_MARKER: ELEMENT_TAG_ID = ELEMENT_TAG_ID(166i32);
pub const TAGID_SVG_MASK: ELEMENT_TAG_ID = ELEMENT_TAG_ID(167i32);
pub const TAGID_SVG_METADATA: ELEMENT_TAG_ID = ELEMENT_TAG_ID(168i32);
pub const TAGID_SVG_MISSING_GLYPH: ELEMENT_TAG_ID = ELEMENT_TAG_ID(169i32);
pub const TAGID_SVG_MPATH: ELEMENT_TAG_ID = ELEMENT_TAG_ID(170i32);
pub const TAGID_SVG_PATH: ELEMENT_TAG_ID = ELEMENT_TAG_ID(171i32);
pub const TAGID_SVG_PATTERN: ELEMENT_TAG_ID = ELEMENT_TAG_ID(172i32);
pub const TAGID_SVG_POLYGON: ELEMENT_TAG_ID = ELEMENT_TAG_ID(173i32);
pub const TAGID_SVG_POLYLINE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(174i32);
pub const TAGID_SVG_RADIALGRADIENT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(175i32);
pub const TAGID_SVG_RECT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(176i32);
pub const TAGID_SVG_SCRIPT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(177i32);
pub const TAGID_SVG_SET: ELEMENT_TAG_ID = ELEMENT_TAG_ID(178i32);
pub const TAGID_SVG_STOP: ELEMENT_TAG_ID = ELEMENT_TAG_ID(179i32);
pub const TAGID_SVG_STYLE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(180i32);
pub const TAGID_SVG_SVG: ELEMENT_TAG_ID = ELEMENT_TAG_ID(181i32);
pub const TAGID_SVG_SWITCH: ELEMENT_TAG_ID = ELEMENT_TAG_ID(182i32);
pub const TAGID_SVG_SYMBOL: ELEMENT_TAG_ID = ELEMENT_TAG_ID(183i32);
pub const TAGID_SVG_TEXT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(184i32);
pub const TAGID_SVG_TEXTPATH: ELEMENT_TAG_ID = ELEMENT_TAG_ID(185i32);
pub const TAGID_SVG_TITLE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(186i32);
pub const TAGID_SVG_TREF: ELEMENT_TAG_ID = ELEMENT_TAG_ID(187i32);
pub const TAGID_SVG_TSPAN: ELEMENT_TAG_ID = ELEMENT_TAG_ID(188i32);
pub const TAGID_SVG_USE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(189i32);
pub const TAGID_SVG_VIEW: ELEMENT_TAG_ID = ELEMENT_TAG_ID(190i32);
pub const TAGID_SVG_VKERN: ELEMENT_TAG_ID = ELEMENT_TAG_ID(191i32);
pub const TAGID_AUDIO: ELEMENT_TAG_ID = ELEMENT_TAG_ID(192i32);
pub const TAGID_SOURCE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(193i32);
pub const TAGID_VIDEO: ELEMENT_TAG_ID = ELEMENT_TAG_ID(194i32);
pub const TAGID_CANVAS: ELEMENT_TAG_ID = ELEMENT_TAG_ID(195i32);
pub const TAGID_DOCTYPE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(196i32);
pub const TAGID_KEYGEN: ELEMENT_TAG_ID = ELEMENT_TAG_ID(197i32);
pub const TAGID_PROCESSINGINSTRUCTION: ELEMENT_TAG_ID = ELEMENT_TAG_ID(198i32);
pub const TAGID_ARTICLE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(199i32);
pub const TAGID_ASIDE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(200i32);
pub const TAGID_FIGCAPTION: ELEMENT_TAG_ID = ELEMENT_TAG_ID(201i32);
pub const TAGID_FIGURE: ELEMENT_TAG_ID = ELEMENT_TAG_ID(202i32);
pub const TAGID_FOOTER: ELEMENT_TAG_ID = ELEMENT_TAG_ID(203i32);
pub const TAGID_HEADER: ELEMENT_TAG_ID = ELEMENT_TAG_ID(204i32);
pub const TAGID_HGROUP: ELEMENT_TAG_ID = ELEMENT_TAG_ID(205i32);
pub const TAGID_MARK: ELEMENT_TAG_ID = ELEMENT_TAG_ID(206i32);
pub const TAGID_NAV: ELEMENT_TAG_ID = ELEMENT_TAG_ID(207i32);
pub const TAGID_SECTION: ELEMENT_TAG_ID = ELEMENT_TAG_ID(208i32);
pub const TAGID_PROGRESS: ELEMENT_TAG_ID = ELEMENT_TAG_ID(209i32);
pub const TAGID_MATHML_ANNOTATION_XML: ELEMENT_TAG_ID = ELEMENT_TAG_ID(210i32);
pub const TAGID_MATHML_MATH: ELEMENT_TAG_ID = ELEMENT_TAG_ID(211i32);
pub const TAGID_MATHML_MI: ELEMENT_TAG_ID = ELEMENT_TAG_ID(212i32);
pub const TAGID_MATHML_MN: ELEMENT_TAG_ID = ELEMENT_TAG_ID(213i32);
pub const TAGID_MATHML_MO: ELEMENT_TAG_ID = ELEMENT_TAG_ID(214i32);
pub const TAGID_MATHML_MS: ELEMENT_TAG_ID = ELEMENT_TAG_ID(215i32);
pub const TAGID_MATHML_MTEXT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(216i32);
pub const TAGID_DATALIST: ELEMENT_TAG_ID = ELEMENT_TAG_ID(217i32);
pub const TAGID_TRACK: ELEMENT_TAG_ID = ELEMENT_TAG_ID(218i32);
pub const TAGID_ISINDEX: ELEMENT_TAG_ID = ELEMENT_TAG_ID(219i32);
pub const TAGID_COMMAND: ELEMENT_TAG_ID = ELEMENT_TAG_ID(220i32);
pub const TAGID_DETAILS: ELEMENT_TAG_ID = ELEMENT_TAG_ID(221i32);
pub const TAGID_SUMMARY: ELEMENT_TAG_ID = ELEMENT_TAG_ID(222i32);
pub const TAGID_X_MS_WEBVIEW: ELEMENT_TAG_ID = ELEMENT_TAG_ID(223i32);
pub const TAGID_COUNT: ELEMENT_TAG_ID = ELEMENT_TAG_ID(224i32);
pub const TAGID_LAST_PREDEFINED: ELEMENT_TAG_ID = ELEMENT_TAG_ID(10000i32);
pub const ELEMENT_TAG_ID_Max: ELEMENT_TAG_ID = ELEMENT_TAG_ID(2147483647i32);
impl ::core::marker::Copy for ELEMENT_TAG_ID {}
impl ::core::clone::Clone for ELEMENT_TAG_ID {
    fn clone(&self) -> Self {
        *self
    }
}
pub const E_SURFACE_DISCARDED: i32 = -2147434493i32;
pub const E_SURFACE_NODC: i32 = -2147434492i32;
pub const E_SURFACE_NOSURFACE: i32 = -2147434496i32;
pub const E_SURFACE_NOTMYDC: i32 = -2147434491i32;
pub const E_SURFACE_NOTMYPOINTER: i32 = -2147434494i32;
pub const E_SURFACE_UNKNOWN_FORMAT: i32 = -2147434495i32;
pub const EventException: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616635, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct ExtensionValidationContexts(pub i32);
pub const ExtensionValidationContextNone: ExtensionValidationContexts = ExtensionValidationContexts(0i32);
pub const ExtensionValidationContextDynamic: ExtensionValidationContexts = ExtensionValidationContexts(1i32);
pub const ExtensionValidationContextParsed: ExtensionValidationContexts = ExtensionValidationContexts(2i32);
impl ::core::marker::Copy for ExtensionValidationContexts {}
impl ::core::clone::Clone for ExtensionValidationContexts {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExtensionValidationResults(pub i32);
pub const ExtensionValidationResultNone: ExtensionValidationResults = ExtensionValidationResults(0i32);
pub const ExtensionValidationResultDoNotInstantiate: ExtensionValidationResults = ExtensionValidationResults(1i32);
pub const ExtensionValidationResultArrestPageLoad: ExtensionValidationResults = ExtensionValidationResults(2i32);
impl ::core::marker::Copy for ExtensionValidationResults {}
impl ::core::clone::Clone for ExtensionValidationResults {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FINDTEXT_FLAGS(pub i32);
pub const FINDTEXT_BACKWARDS: FINDTEXT_FLAGS = FINDTEXT_FLAGS(1i32);
pub const FINDTEXT_WHOLEWORD: FINDTEXT_FLAGS = FINDTEXT_FLAGS(2i32);
pub const FINDTEXT_MATCHCASE: FINDTEXT_FLAGS = FINDTEXT_FLAGS(4i32);
pub const FINDTEXT_RAW: FINDTEXT_FLAGS = FINDTEXT_FLAGS(131072i32);
pub const FINDTEXT_MATCHREPEATEDWHITESPACE: FINDTEXT_FLAGS = FINDTEXT_FLAGS(262144i32);
pub const FINDTEXT_MATCHDIAC: FINDTEXT_FLAGS = FINDTEXT_FLAGS(536870912i32);
pub const FINDTEXT_MATCHKASHIDA: FINDTEXT_FLAGS = FINDTEXT_FLAGS(1073741824i32);
pub const FINDTEXT_MATCHALEFHAMZA: FINDTEXT_FLAGS = FINDTEXT_FLAGS(-2147483648i32);
pub const FINDTEXT_FLAGS_Max: FINDTEXT_FLAGS = FINDTEXT_FLAGS(2147483647i32);
impl ::core::marker::Copy for FINDTEXT_FLAGS {}
impl ::core::clone::Clone for FINDTEXT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FontNames: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612794, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const FramesCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612726, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTCAttachBehavior: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612213, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTCDefaultDispatch: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611964, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTCDescBehavior: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612189, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTCEventBehavior: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611966, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTCMethodBehavior: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612272, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTCPropertyBehavior: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612190, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLAnchorElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611272, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLAnchorEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLAnchorEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLAppBehavior: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612171, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLAppFlag(pub i32);
pub const HTMLAppFlagNo: HTMLAppFlag = HTMLAppFlag(0i32);
pub const HTMLAppFlagOff: HTMLAppFlag = HTMLAppFlag(0i32);
pub const HTMLAppFlag0: HTMLAppFlag = HTMLAppFlag(0i32);
pub const HTMLAppFlagYes: HTMLAppFlag = HTMLAppFlag(1i32);
pub const HTMLAppFlagOn: HTMLAppFlag = HTMLAppFlag(1i32);
pub const HTMLAppFlag1: HTMLAppFlag = HTMLAppFlag(1i32);
pub const HTMLAppFlag_Max: HTMLAppFlag = HTMLAppFlag(2147483647i32);
impl ::core::marker::Copy for HTMLAppFlag {}
impl ::core::clone::Clone for HTMLAppFlag {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HTMLAreaElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611331, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLAreaEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLAreaEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLAreasCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611914, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLAttributeCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611916, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLAudioElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616590, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLAudioElementFactory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616812, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLBGsound: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611568, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLBRElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611328, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLBaseElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611318, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLBaseFontElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611330, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLBlockElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611329, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLBody: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611274, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLBorder(pub i32);
pub const HTMLBorderNone: HTMLBorder = HTMLBorder(0i32);
pub const HTMLBorderThick: HTMLBorder = HTMLBorder(262144i32);
pub const HTMLBorderDialog: HTMLBorder = HTMLBorder(4194304i32);
pub const HTMLBorderThin: HTMLBorder = HTMLBorder(8388608i32);
pub const HTMLBorder_Max: HTMLBorder = HTMLBorder(2147483647i32);
impl ::core::marker::Copy for HTMLBorder {}
impl ::core::clone::Clone for HTMLBorder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HTMLBorderStyle(pub i32);
pub const HTMLBorderStyleNormal: HTMLBorderStyle = HTMLBorderStyle(0i32);
pub const HTMLBorderStyleRaised: HTMLBorderStyle = HTMLBorderStyle(256i32);
pub const HTMLBorderStyleSunken: HTMLBorderStyle = HTMLBorderStyle(512i32);
pub const HTMLBorderStylecombined: HTMLBorderStyle = HTMLBorderStyle(768i32);
pub const HTMLBorderStyleStatic: HTMLBorderStyle = HTMLBorderStyle(131072i32);
pub const HTMLBorderStyle_Max: HTMLBorderStyle = HTMLBorderStyle(2147483647i32);
impl ::core::marker::Copy for HTMLBorderStyle {}
impl ::core::clone::Clone for HTMLBorderStyle {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HTMLButtonElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611398, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLButtonElementEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLButtonElementEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLCSSImportRule: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616560, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLCSSMediaList: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616626, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLCSSMediaRule: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616561, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLCSSNamespaceRule: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616562, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLCSSRule: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616559, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLCSSStyleDeclaration: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616641, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLCanvasElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616549, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLCaptionFlag(pub i32);
pub const HTMLCaptionFlagNo: HTMLCaptionFlag = HTMLCaptionFlag(0i32);
pub const HTMLCaptionFlagYes: HTMLCaptionFlag = HTMLCaptionFlag(12582912i32);
pub const HTMLCaptionFlag_Max: HTMLCaptionFlag = HTMLCaptionFlag(2147483647i32);
impl ::core::marker::Copy for HTMLCaptionFlag {}
impl ::core::clone::Clone for HTMLCaptionFlag {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HTMLCommentElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611479, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLControlElementEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLControlElementEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLCurrentStyle: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611676, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLDDElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611327, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLDLG_ALLOW_UNKNOWN_THREAD: u32 = 512u32;
pub const HTMLDLG_MODAL: u32 = 32u32;
pub const HTMLDLG_MODELESS: u32 = 64u32;
pub const HTMLDLG_NOUI: u32 = 16u32;
pub const HTMLDLG_PRINT_TEMPLATE: u32 = 128u32;
pub const HTMLDLG_VERIFY: u32 = 256u32;
pub const HTMLDListElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611325, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLDOMAttribute: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611890, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLDOMImplementation: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612750, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLDOMRange: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616515, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLDOMTextNode: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611898, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLDOMXmlSerializerFactory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616704, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLDTElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611324, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLDefaults: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612424, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLDialog: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611338, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLDivElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611326, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLDivPosition: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611273, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLDlgBorder(pub i32);
pub const HTMLDlgBorderThin: HTMLDlgBorder = HTMLDlgBorder(0i32);
pub const HTMLDlgBorderThick: HTMLDlgBorder = HTMLDlgBorder(262144i32);
pub const HTMLDlgBorder_Max: HTMLDlgBorder = HTMLDlgBorder(2147483647i32);
impl ::core::marker::Copy for HTMLDlgBorder {}
impl ::core::clone::Clone for HTMLDlgBorder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HTMLDlgCenter(pub i32);
pub const HTMLDlgCenterNo: HTMLDlgCenter = HTMLDlgCenter(0i32);
pub const HTMLDlgCenterOff: HTMLDlgCenter = HTMLDlgCenter(0i32);
pub const HTMLDlgCenter0: HTMLDlgCenter = HTMLDlgCenter(0i32);
pub const HTMLDlgCenterYes: HTMLDlgCenter = HTMLDlgCenter(1i32);
pub const HTMLDlgCenterOn: HTMLDlgCenter = HTMLDlgCenter(1i32);
pub const HTMLDlgCenter1: HTMLDlgCenter = HTMLDlgCenter(1i32);
pub const HTMLDlgCenterParent: HTMLDlgCenter = HTMLDlgCenter(1i32);
pub const HTMLDlgCenterDesktop: HTMLDlgCenter = HTMLDlgCenter(2i32);
pub const HTMLDlgCenter_Max: HTMLDlgCenter = HTMLDlgCenter(2147483647i32);
impl ::core::marker::Copy for HTMLDlgCenter {}
impl ::core::clone::Clone for HTMLDlgCenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HTMLDlgEdge(pub i32);
pub const HTMLDlgEdgeSunken: HTMLDlgEdge = HTMLDlgEdge(0i32);
pub const HTMLDlgEdgeRaised: HTMLDlgEdge = HTMLDlgEdge(16i32);
pub const HTMLDlgEdge_Max: HTMLDlgEdge = HTMLDlgEdge(2147483647i32);
impl ::core::marker::Copy for HTMLDlgEdge {}
impl ::core::clone::Clone for HTMLDlgEdge {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HTMLDlgFlag(pub i32);
pub const HTMLDlgFlagNo: HTMLDlgFlag = HTMLDlgFlag(0i32);
pub const HTMLDlgFlagOff: HTMLDlgFlag = HTMLDlgFlag(0i32);
pub const HTMLDlgFlag0: HTMLDlgFlag = HTMLDlgFlag(0i32);
pub const HTMLDlgFlagYes: HTMLDlgFlag = HTMLDlgFlag(1i32);
pub const HTMLDlgFlagOn: HTMLDlgFlag = HTMLDlgFlag(1i32);
pub const HTMLDlgFlag1: HTMLDlgFlag = HTMLDlgFlag(1i32);
pub const HTMLDlgFlagNotSet: HTMLDlgFlag = HTMLDlgFlag(-1i32);
pub const HTMLDlgFlag_Max: HTMLDlgFlag = HTMLDlgFlag(2147483647i32);
impl ::core::marker::Copy for HTMLDlgFlag {}
impl ::core::clone::Clone for HTMLDlgFlag {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HTMLDocument: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 624126240, data2: 1017, data3: 4559, data4: [143, 208, 0, 170, 0, 104, 111, 19] };
pub const HTMLDocumentCompatibleInfo: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810615835, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLDocumentCompatibleInfoCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810615833, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLDocumentEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLDocumentEvents2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLDocumentEvents3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLDocumentEvents4(pub *mut ::core::ffi::c_void);
pub const HTMLElementCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611915, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLElementEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLElementEvents2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLElementEvents3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLElementEvents4(pub *mut ::core::ffi::c_void);
pub const HTMLEmbed: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611293, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLFieldSetElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611688, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLFontElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611323, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLFormElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611281, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLFormElementEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLFormElementEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLFrameBase: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611474, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLFrameElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611476, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLFrameSetSite: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611482, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLFrameSiteEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLFrameSiteEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLGenericElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611896, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLHRElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611282, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLHeadElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611859, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLHeaderElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611322, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLHistory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4274956963, data2: 33797, data3: 4559, data4: [139, 161, 0, 170, 0, 71, 109, 166] };
pub const HTMLHtmlElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611857, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLIFrame: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611478, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLImageElementFactory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611599, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLImg: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611265, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLImgEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLImgEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLInputButtonElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611380, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLInputElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612184, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLInputFileElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611374, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLInputFileElementEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLInputFileElementEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLInputImage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611396, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLInputImageEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLInputImageEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLInputTextElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611371, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLInputTextElementEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLInputTextElementEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLIsIndexElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611320, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLLIElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611315, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLLabelElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611499, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLLabelEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLLabelEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLLegendElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611689, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLLinkElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611319, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLLinkElementEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLLinkElementEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLListElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611314, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLLocation: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 373010913, data2: 28160, data3: 4559, data4: [131, 122, 72, 220, 4, 193, 0, 0] };
pub const HTMLMSCSSKeyframeRule: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616846, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLMSCSSKeyframesRule: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616847, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLMapElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611313, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLMapEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLMapEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLMarqueeElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611385, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLMarqueeElementEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLMarqueeElementEvents2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLMaximizeFlag(pub i32);
pub const HTMLMaximizeFlagNo: HTMLMaximizeFlag = HTMLMaximizeFlag(0i32);
pub const HTMLMaximizeFlagYes: HTMLMaximizeFlag = HTMLMaximizeFlag(65536i32);
pub const HTMLMaximizeFlag_Max: HTMLMaximizeFlag = HTMLMaximizeFlag(2147483647i32);
impl ::core::marker::Copy for HTMLMaximizeFlag {}
impl ::core::clone::Clone for HTMLMaximizeFlag {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HTMLMediaElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616588, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLMediaError: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616586, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLMetaElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611317, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLMinimizeFlag(pub i32);
pub const HTMLMinimizeFlagNo: HTMLMinimizeFlag = HTMLMinimizeFlag(0i32);
pub const HTMLMinimizeFlagYes: HTMLMinimizeFlag = HTMLMinimizeFlag(131072i32);
pub const HTMLMinimizeFlag_Max: HTMLMinimizeFlag = HTMLMinimizeFlag(2147483647i32);
impl ::core::marker::Copy for HTMLMinimizeFlag {}
impl ::core::clone::Clone for HTMLMinimizeFlag {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HTMLNamespace: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612412, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLNamespaceCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612409, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLNamespaceEvents(pub *mut ::core::ffi::c_void);
pub const HTMLNavigator: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4274956966, data2: 33797, data3: 4559, data4: [139, 161, 0, 170, 0, 71, 109, 166] };
pub const HTMLNextIdElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611321, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLNoShowElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611595, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLOListElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611312, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLObjectElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611278, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLObjectElementEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLObjectElementEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLOptionButtonElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611390, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLOptionButtonElementEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLOptionButtonElementEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLOptionElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611277, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLOptionElementFactory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611597, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLParaElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611311, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLParamElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612798, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLPerformance: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616655, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLPerformanceNavigation: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616657, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLPerformanceTiming: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616659, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(C)]
pub struct HTMLPersistEvents(pub u8);
pub const HTMLPhraseElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611310, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLPopup: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612327, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLProgressElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611413, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLRenderStyle: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612394, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLRichtextElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611423, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLRuleStyle: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611664, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLScreen: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611549, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLScriptElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611340, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLScriptEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLScriptEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLSelectElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611269, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLSelectElementEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLSelectElementEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLSemanticElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616752, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLSourceElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616589, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLSpanElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611701, data2: 39092, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLSpanFlow: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611686, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLStorage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810615925, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLStyle: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611333, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLStyleElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611581, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLStyleElementEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLStyleElementEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLStyleFontFace: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611668, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLStyleMedia: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616652, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLStyleSheet: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611428, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLStyleSheetPage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612719, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLStyleSheetPagesCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612721, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLStyleSheetRule: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611662, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLStyleSheetRulesAppliedCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3946248261, data2: 9109, data3: 18201, data4: [184, 92, 208, 216, 14, 24, 75, 217] };
pub const HTMLStyleSheetRulesCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611661, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLStyleSheetsCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611583, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLSysMenuFlag(pub i32);
pub const HTMLSysMenuFlagNo: HTMLSysMenuFlag = HTMLSysMenuFlag(0i32);
pub const HTMLSysMenuFlagYes: HTMLSysMenuFlag = HTMLSysMenuFlag(524288i32);
pub const HTMLSysMenuFlag_Max: HTMLSysMenuFlag = HTMLSysMenuFlag(2147483647i32);
impl ::core::marker::Copy for HTMLSysMenuFlag {}
impl ::core::clone::Clone for HTMLSysMenuFlag {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HTMLTable: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611307, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLTableCaption: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611436, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLTableCell: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611270, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLTableCol: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611308, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLTableEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLTableEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLTableRow: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611309, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLTableSection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611433, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLTextAreaElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611372, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLTextContainerEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLTextContainerEvents2(pub *mut ::core::ffi::c_void);
pub const HTMLTextElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611306, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLTimeRanges: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616587, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLTitleElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611332, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLUListElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611305, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLUnknownElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611304, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLUrnCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612096, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLVideoElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616591, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLW3CComputedStyle: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616520, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLWindow2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3565842118, data2: 27210, data3: 4559, data4: [148, 167, 68, 69, 83, 84, 0, 0] };
#[repr(transparent)]
pub struct HTMLWindowEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLWindowEvents2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HTMLWindowEvents3(pub *mut ::core::ffi::c_void);
pub const HTMLWindowProxy: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611601, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLWindowState(pub i32);
pub const HTMLWindowStateNormal: HTMLWindowState = HTMLWindowState(1i32);
pub const HTMLWindowStateMaximize: HTMLWindowState = HTMLWindowState(3i32);
pub const HTMLWindowStateMinimize: HTMLWindowState = HTMLWindowState(6i32);
pub const HTMLWindowState_Max: HTMLWindowState = HTMLWindowState(2147483647i32);
impl ::core::marker::Copy for HTMLWindowState {}
impl ::core::clone::Clone for HTMLWindowState {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HTMLWndOptionElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611408, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLWndSelectElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611407, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const HTMLXMLHttpRequest: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810615819, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTMLXMLHttpRequestEvents(pub *mut ::core::ffi::c_void);
pub const HTMLXMLHttpRequestFactory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810615821, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct HTML_PAINTER(pub i32);
pub const HTMLPAINTER_OPAQUE: HTML_PAINTER = HTML_PAINTER(1i32);
pub const HTMLPAINTER_TRANSPARENT: HTML_PAINTER = HTML_PAINTER(2i32);
pub const HTMLPAINTER_ALPHA: HTML_PAINTER = HTML_PAINTER(4i32);
pub const HTMLPAINTER_COMPLEX: HTML_PAINTER = HTML_PAINTER(8i32);
pub const HTMLPAINTER_OVERLAY: HTML_PAINTER = HTML_PAINTER(16i32);
pub const HTMLPAINTER_HITTEST: HTML_PAINTER = HTML_PAINTER(32i32);
pub const HTMLPAINTER_SURFACE: HTML_PAINTER = HTML_PAINTER(256i32);
pub const HTMLPAINTER_3DSURFACE: HTML_PAINTER = HTML_PAINTER(512i32);
pub const HTMLPAINTER_NOBAND: HTML_PAINTER = HTML_PAINTER(1024i32);
pub const HTMLPAINTER_NODC: HTML_PAINTER = HTML_PAINTER(4096i32);
pub const HTMLPAINTER_NOPHYSICALCLIP: HTML_PAINTER = HTML_PAINTER(8192i32);
pub const HTMLPAINTER_NOSAVEDC: HTML_PAINTER = HTML_PAINTER(16384i32);
pub const HTMLPAINTER_SUPPORTS_XFORM: HTML_PAINTER = HTML_PAINTER(32768i32);
pub const HTMLPAINTER_EXPAND: HTML_PAINTER = HTML_PAINTER(65536i32);
pub const HTMLPAINTER_NOSCROLLBITS: HTML_PAINTER = HTML_PAINTER(131072i32);
pub const HTML_PAINTER_Max: HTML_PAINTER = HTML_PAINTER(2147483647i32);
impl ::core::marker::Copy for HTML_PAINTER {}
impl ::core::clone::Clone for HTML_PAINTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTML_PAINTER_INFO {
    pub lFlags: i32,
    pub lZOrder: i32,
    pub iidDrawObject: ::windows_sys::core::GUID,
    pub rcExpand: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTML_PAINTER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTML_PAINTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HTML_PAINT_DRAW_FLAGS(pub i32);
pub const HTMLPAINT_DRAW_UPDATEREGION: HTML_PAINT_DRAW_FLAGS = HTML_PAINT_DRAW_FLAGS(1i32);
pub const HTMLPAINT_DRAW_USE_XFORM: HTML_PAINT_DRAW_FLAGS = HTML_PAINT_DRAW_FLAGS(2i32);
pub const HTML_PAINT_DRAW_FLAGS_Max: HTML_PAINT_DRAW_FLAGS = HTML_PAINT_DRAW_FLAGS(2147483647i32);
impl ::core::marker::Copy for HTML_PAINT_DRAW_FLAGS {}
impl ::core::clone::Clone for HTML_PAINT_DRAW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct HTML_PAINT_DRAW_INFO {
    pub rcViewport: super::super::Foundation::RECT,
    pub hrgnUpdate: super::super::Graphics::Gdi::HRGN,
    pub xform: HTML_PAINT_XFORM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for HTML_PAINT_DRAW_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for HTML_PAINT_DRAW_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HTML_PAINT_DRAW_INFO_FLAGS(pub i32);
pub const HTMLPAINT_DRAWINFO_VIEWPORT: HTML_PAINT_DRAW_INFO_FLAGS = HTML_PAINT_DRAW_INFO_FLAGS(1i32);
pub const HTMLPAINT_DRAWINFO_UPDATEREGION: HTML_PAINT_DRAW_INFO_FLAGS = HTML_PAINT_DRAW_INFO_FLAGS(2i32);
pub const HTMLPAINT_DRAWINFO_XFORM: HTML_PAINT_DRAW_INFO_FLAGS = HTML_PAINT_DRAW_INFO_FLAGS(4i32);
pub const HTML_PAINT_DRAW_INFO_FLAGS_Max: HTML_PAINT_DRAW_INFO_FLAGS = HTML_PAINT_DRAW_INFO_FLAGS(2147483647i32);
impl ::core::marker::Copy for HTML_PAINT_DRAW_INFO_FLAGS {}
impl ::core::clone::Clone for HTML_PAINT_DRAW_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HTML_PAINT_EVENT_FLAGS(pub i32);
pub const HTMLPAINT_EVENT_TARGET: HTML_PAINT_EVENT_FLAGS = HTML_PAINT_EVENT_FLAGS(1i32);
pub const HTMLPAINT_EVENT_SETCURSOR: HTML_PAINT_EVENT_FLAGS = HTML_PAINT_EVENT_FLAGS(2i32);
pub const HTML_PAINT_EVENT_FLAGS_Max: HTML_PAINT_EVENT_FLAGS = HTML_PAINT_EVENT_FLAGS(2147483647i32);
impl ::core::marker::Copy for HTML_PAINT_EVENT_FLAGS {}
impl ::core::clone::Clone for HTML_PAINT_EVENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct HTML_PAINT_XFORM {
    pub eM11: f32,
    pub eM12: f32,
    pub eM21: f32,
    pub eM22: f32,
    pub eDx: f32,
    pub eDy: f32,
}
impl ::core::marker::Copy for HTML_PAINT_XFORM {}
impl ::core::clone::Clone for HTML_PAINT_XFORM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HTML_PAINT_ZORDER(pub i32);
pub const HTMLPAINT_ZORDER_NONE: HTML_PAINT_ZORDER = HTML_PAINT_ZORDER(0i32);
pub const HTMLPAINT_ZORDER_REPLACE_ALL: HTML_PAINT_ZORDER = HTML_PAINT_ZORDER(1i32);
pub const HTMLPAINT_ZORDER_REPLACE_CONTENT: HTML_PAINT_ZORDER = HTML_PAINT_ZORDER(2i32);
pub const HTMLPAINT_ZORDER_REPLACE_BACKGROUND: HTML_PAINT_ZORDER = HTML_PAINT_ZORDER(3i32);
pub const HTMLPAINT_ZORDER_BELOW_CONTENT: HTML_PAINT_ZORDER = HTML_PAINT_ZORDER(4i32);
pub const HTMLPAINT_ZORDER_BELOW_FLOW: HTML_PAINT_ZORDER = HTML_PAINT_ZORDER(5i32);
pub const HTMLPAINT_ZORDER_ABOVE_FLOW: HTML_PAINT_ZORDER = HTML_PAINT_ZORDER(6i32);
pub const HTMLPAINT_ZORDER_ABOVE_CONTENT: HTML_PAINT_ZORDER = HTML_PAINT_ZORDER(7i32);
pub const HTMLPAINT_ZORDER_WINDOW_TOP: HTML_PAINT_ZORDER = HTML_PAINT_ZORDER(8i32);
pub const HTML_PAINT_ZORDER_Max: HTML_PAINT_ZORDER = HTML_PAINT_ZORDER(2147483647i32);
impl ::core::marker::Copy for HTML_PAINT_ZORDER {}
impl ::core::clone::Clone for HTML_PAINT_ZORDER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HT_OPTIONS(pub i32);
pub const HT_OPT_AllowAfterEOL: HT_OPTIONS = HT_OPTIONS(1i32);
pub const HT_OPTIONS_Max: HT_OPTIONS = HT_OPTIONS(2147483647i32);
impl ::core::marker::Copy for HT_OPTIONS {}
impl ::core::clone::Clone for HT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HT_RESULTS(pub i32);
pub const HT_RESULTS_Glyph: HT_RESULTS = HT_RESULTS(1i32);
pub const HT_RESULTS_Max: HT_RESULTS = HT_RESULTS(2147483647i32);
impl ::core::marker::Copy for HT_RESULTS {}
impl ::core::clone::Clone for HT_RESULTS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HomePage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1986785966, data2: 54864, data3: 4561, data4: [152, 17, 0, 192, 79, 195, 29, 46] };
pub const HomePageSetting: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 927788512,
    data2: 34618,
    data3: 19535,
    data4: [188, 134, 188, 200, 207, 81, 22, 163],
};
#[repr(C)]
pub struct HostDialogHelper(pub u8);
pub const HtmlDlgSafeHelper: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612761, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct IActiveXUIHandlerSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveXUIHandlerSite2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveXUIHandlerSite3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnchorClick(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioSessionSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBFCacheable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBlockFormats(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICSSFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICSSFilterSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICanvasGradient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICanvasImageData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICanvasPattern(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICanvasPixelArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICanvasPixelArrayData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICanvasRenderingContext2D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICanvasTextMetrics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICaretPositionProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClassFactoryEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClientCaps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomDoc(pub *mut ::core::ffi::c_void);
pub const IDM_1D: u32 = 2170u32;
pub const IDM_1D_ELEMENT: u32 = 2396u32;
pub const IDM_2D_ELEMENT: u32 = 2395u32;
pub const IDM_2D_POSITION: u32 = 2394u32;
pub const IDM_ABSOLUTE_POSITION: u32 = 2397u32;
pub const IDM_ACTIVEXFILTERINGENABLED: u32 = 15030u32;
pub const IDM_ACTIVEXINSTALLSCOPE: u32 = 15007u32;
pub const IDM_ADDCONSOLEMESSAGERECEIVER: u32 = 3800u32;
pub const IDM_ADDDEBUGCALLBACKRECEIVER: u32 = 3804u32;
pub const IDM_ADDFAVORITES: u32 = 2261u32;
pub const IDM_ADDPARTIALTESTSTEPCOUNT: u32 = 15023u32;
pub const IDM_ADDPDFHIGHLIGHT: u32 = 15210u32;
pub const IDM_ADDPDFNOTE: u32 = 15212u32;
pub const IDM_ADDRESS: u32 = 2189u32;
pub const IDM_ADDTOGLYPHTABLE: u32 = 2337u32;
pub const IDM_ALIGNBOTTOM: u32 = 1u32;
pub const IDM_ALIGNHORIZONTALCENTERS: u32 = 2u32;
pub const IDM_ALIGNLEFT: u32 = 3u32;
pub const IDM_ALIGNRIGHT: u32 = 4u32;
pub const IDM_ALIGNTOGRID: u32 = 5u32;
pub const IDM_ALIGNTOP: u32 = 6u32;
pub const IDM_ALIGNVERTICALCENTERS: u32 = 7u32;
pub const IDM_APPLYHEADING1: u32 = 2255u32;
pub const IDM_APPLYHEADING2: u32 = 2256u32;
pub const IDM_APPLYHEADING3: u32 = 2257u32;
pub const IDM_APPLYNORMAL: u32 = 2254u32;
pub const IDM_ARRANGEBOTTOM: u32 = 8u32;
pub const IDM_ARRANGERIGHT: u32 = 9u32;
pub const IDM_ATOMICSELECTION: u32 = 2399u32;
pub const IDM_AUTODETECT: u32 = 2329u32;
pub const IDM_AUTOURLDETECT_MODE: u32 = 2400u32;
pub const IDM_BACKCOLOR: u32 = 51u32;
pub const IDM_BACKGROUNDIMAGECACHE: u32 = 2430u32;
pub const IDM_BASELINEFONT1: u32 = 2141u32;
pub const IDM_BASELINEFONT2: u32 = 2142u32;
pub const IDM_BASELINEFONT3: u32 = 2143u32;
pub const IDM_BASELINEFONT4: u32 = 2144u32;
pub const IDM_BASELINEFONT5: u32 = 2145u32;
pub const IDM_BEGINUNDOUNIT: u32 = 3901u32;
pub const IDM_BEGINUSERACTION: u32 = 2432u32;
pub const IDM_BLINK: u32 = 2190u32;
pub const IDM_BLOCKDIRLTR: u32 = 2352u32;
pub const IDM_BLOCKDIRRTL: u32 = 2353u32;
pub const IDM_BLOCKFMT: u32 = 2234u32;
pub const IDM_BLUEHIGHLIGHT: u32 = 15216u32;
pub const IDM_BOLD: u32 = 52u32;
pub const IDM_BOOKMARK: u32 = 2123u32;
pub const IDM_BORDERCOLOR: u32 = 53u32;
pub const IDM_BREAKATNEXT: u32 = 2311u32;
pub const IDM_BRINGFORWARD: u32 = 10u32;
pub const IDM_BRINGTOFRONT: u32 = 11u32;
pub const IDM_BROWSEMODE: u32 = 2126u32;
pub const IDM_BUTTON: u32 = 2167u32;
pub const IDM_CANCEL: u32 = 89u32;
pub const IDM_CAPTIONINSERT: u32 = 2203u32;
pub const IDM_CARETBROWSINGMODE: u32 = 2436u32;
pub const IDM_CELLINSERT: u32 = 2202u32;
pub const IDM_CELLMERGE: u32 = 2204u32;
pub const IDM_CELLPROPERTIES: u32 = 2211u32;
pub const IDM_CELLSELECT: u32 = 2206u32;
pub const IDM_CELLSPLIT: u32 = 2205u32;
pub const IDM_CENTERALIGNPARA: u32 = 2250u32;
pub const IDM_CENTERHORIZONTALLY: u32 = 12u32;
pub const IDM_CENTERVERTICALLY: u32 = 13u32;
pub const IDM_CHANGECASE: u32 = 2246u32;
pub const IDM_CHANGEFONT: u32 = 2240u32;
pub const IDM_CHANGEFONTSIZE: u32 = 2241u32;
pub const IDM_CHECKBOX: u32 = 2163u32;
pub const IDM_CHISELED: u32 = 64u32;
pub const IDM_CLEARAUTHENTICATIONCACHE: u32 = 15003u32;
pub const IDM_CLEARSELECTION: u32 = 2007u32;
pub const IDM_CLEARUNDO: u32 = 3903u32;
pub const IDM_CODE: u32 = 14u32;
pub const IDM_COLUMNINSERT: u32 = 2213u32;
pub const IDM_COLUMNSELECT: u32 = 2208u32;
pub const IDM_COMMENT: u32 = 2173u32;
pub const IDM_COMPOSESETTINGS: u32 = 2318u32;
pub const IDM_CONTEXT: u32 = 1u32;
pub const IDM_CONTEXTMENU: u32 = 2280u32;
pub const IDM_CONVERTOBJECT: u32 = 82u32;
pub const IDM_COPY: u32 = 15u32;
pub const IDM_COPYBACKGROUND: u32 = 2265u32;
pub const IDM_COPYCONTENT: u32 = 2291u32;
pub const IDM_COPYFORMAT: u32 = 2237u32;
pub const IDM_COPYSHORTCUT: u32 = 2262u32;
pub const IDM_CREATELINK: u32 = 2290u32;
pub const IDM_CREATESHORTCUT: u32 = 2266u32;
pub const IDM_CSSEDITING_LEVEL: u32 = 2406u32;
pub const IDM_CUSTOMCONTROL: u32 = 83u32;
pub const IDM_CUSTOMIZEITEM: u32 = 84u32;
pub const IDM_CUT: u32 = 16u32;
pub const IDM_DEBUGGERDYNAMICATTACH: u32 = 15202u32;
pub const IDM_DEBUGGERDYNAMICATTACHSOURCERUNDOWN: u32 = 15204u32;
pub const IDM_DEBUGGERDYNAMICDETACH: u32 = 15203u32;
pub const IDM_DEFAULTBLOCK: u32 = 6046u32;
pub const IDM_DEFAULTPARAGRAPHSEPARATOR: u32 = 3900u32;
pub const IDM_DELETE: u32 = 17u32;
pub const IDM_DELETEPDFHIGHLIGHT: u32 = 15211u32;
pub const IDM_DELETEWORD: u32 = 92u32;
pub const IDM_DIRLTR: u32 = 2350u32;
pub const IDM_DIRRTL: u32 = 2351u32;
pub const IDM_DISABLE_EDITFOCUS_UI: u32 = 2404u32;
pub const IDM_DIV: u32 = 2191u32;
pub const IDM_DOCPROPERTIES: u32 = 2260u32;
pub const IDM_DROPDOWNBOX: u32 = 2165u32;
pub const IDM_DYNSRCPLAY: u32 = 2271u32;
pub const IDM_DYNSRCSTOP: u32 = 2272u32;
pub const IDM_EDITMODE: u32 = 2127u32;
pub const IDM_EDITPDFHIGHLIGHT: u32 = 15214u32;
pub const IDM_EDITSOURCE: u32 = 2122u32;
pub const IDM_EMPTYGLYPHTABLE: u32 = 2336u32;
pub const IDM_ENABLEFLIPAHEADTARGET: u32 = 15201u32;
pub const IDM_ENABLE_INTERACTION: u32 = 2302u32;
pub const IDM_ENABLE_OBJECT_RESIZING: u32 = 3906u32;
pub const IDM_ENDUNDOUNIT: u32 = 3902u32;
pub const IDM_ENDUSERACTION: u32 = 2433u32;
pub const IDM_ETCHED: u32 = 65u32;
pub const IDM_EXECPRINT: u32 = 93u32;
pub const IDM_FILE: u32 = 2172u32;
pub const IDM_FIND: u32 = 67u32;
pub const IDM_FIRE_PRINTTEMPLATEDOWN: u32 = 15001u32;
pub const IDM_FIRE_PRINTTEMPLATEUP: u32 = 15000u32;
pub const IDM_FLAT: u32 = 54u32;
pub const IDM_FOLLOWLINKC: u32 = 2136u32;
pub const IDM_FOLLOWLINKEDGE: u32 = 3911u32;
pub const IDM_FOLLOWLINKN: u32 = 2137u32;
pub const IDM_FOLLOWLINKN_INPRIVATE: u32 = 3909u32;
pub const IDM_FOLLOWLINKT: u32 = 2435u32;
pub const IDM_FOLLOWLINKT_INPRIVATE: u32 = 3910u32;
pub const IDM_FOLLOW_ANCHOR: u32 = 2008u32;
pub const IDM_FONT: u32 = 90u32;
pub const IDM_FONTNAME: u32 = 18u32;
pub const IDM_FONTSIZE: u32 = 19u32;
pub const IDM_FORECOLOR: u32 = 55u32;
pub const IDM_FORM: u32 = 2181u32;
pub const IDM_FORMATMARK: u32 = 2132u32;
pub const IDM_FORWARDDELETE: u32 = 98u32;
pub const IDM_GETBLOCKFMTS: u32 = 2233u32;
pub const IDM_GETBYTESDOWNLOADED: u32 = 2331u32;
pub const IDM_GETDEBUGGERSTATE: u32 = 15205u32;
pub const IDM_GETDEFAULTBACKGROUNDCOLOR: u32 = 15044u32;
pub const IDM_GETDEFAULTZOOMLEVEL: u32 = 15027u32;
pub const IDM_GETDOCDLGFLAGS: u32 = 15005u32;
pub const IDM_GETELEMENTBOUNDINGBOX: u32 = 15028u32;
pub const IDM_GETFRAMEZONE: u32 = 6037u32;
pub const IDM_GETIPRINT: u32 = 2403u32;
pub const IDM_GETL9QUIRKSEMULATIONENABLED: u32 = 15025u32;
pub const IDM_GETPARTIALLAYOUTSTATUS: u32 = 15022u32;
pub const IDM_GETPRINTMANAGERDOCSOURCE: u32 = 15038u32;
pub const IDM_GETPRINTMANAGERDOCSOURCEASYNC: u32 = 15047u32;
pub const IDM_GETPRINTTEMPLATE: u32 = 2295u32;
pub const IDM_GETPROFILINGONSTART: u32 = 15011u32;
pub const IDM_GETSCRIPTENGINE: u32 = 3803u32;
pub const IDM_GETSESSIONDOCUMENTMODE: u32 = 15009u32;
pub const IDM_GETUSERACTIONTIME: u32 = 2431u32;
pub const IDM_GETUSERINITFLAGS: u32 = 15004u32;
pub const IDM_GETZOOM: u32 = 68u32;
pub const IDM_GETZOOMDENOMINATOR: u32 = 2346u32;
pub const IDM_GETZOOMNUMERATOR: u32 = 2345u32;
pub const IDM_GOBACKWARD: u32 = 2282u32;
pub const IDM_GOFORWARD: u32 = 2283u32;
pub const IDM_GOTO: u32 = 2239u32;
pub const IDM_GOTOCLIPBOARDADDRESS: u32 = 2285u32;
pub const IDM_GOTOCLIPBOARDTEXT: u32 = 2286u32;
pub const IDM_GREENHIGHLIGHT: u32 = 15217u32;
pub const IDM_GROUP: u32 = 20u32;
pub const IDM_HELP_ABOUT: u32 = 2221u32;
pub const IDM_HELP_CONTENT: u32 = 2220u32;
pub const IDM_HELP_README: u32 = 2222u32;
pub const IDM_HORIZONTALLINE: u32 = 2150u32;
pub const IDM_HORIZSPACECONCATENATE: u32 = 21u32;
pub const IDM_HORIZSPACEDECREASE: u32 = 22u32;
pub const IDM_HORIZSPACEINCREASE: u32 = 23u32;
pub const IDM_HORIZSPACEMAKEEQUAL: u32 = 24u32;
pub const IDM_HTMLAREA: u32 = 2178u32;
pub const IDM_HTMLCONTAIN: u32 = 2159u32;
pub const IDM_HTMLEDITMODE: u32 = 2316u32;
pub const IDM_HTMLSOURCE: u32 = 2157u32;
pub const IDM_HWND: u32 = 2u32;
pub const IDM_HYPERLINK: u32 = 2124u32;
pub const IDM_IE50_PASTE: u32 = 2401u32;
pub const IDM_IE50_PASTE_MODE: u32 = 2402u32;
pub const IDM_IFRAME: u32 = 2158u32;
pub const IDM_IMAGE: u32 = 2168u32;
pub const IDM_IMAGEMAP: u32 = 2171u32;
pub const IDM_IME_ENABLE_RECONVERSION: u32 = 2409u32;
pub const IDM_IMGARTPLAY: u32 = 2274u32;
pub const IDM_IMGARTREWIND: u32 = 2276u32;
pub const IDM_IMGARTSTOP: u32 = 2275u32;
pub const IDM_IMPORT: u32 = 86u32;
pub const IDM_INDENT: u32 = 2186u32;
pub const IDM_INLINEDIRLTR: u32 = 2354u32;
pub const IDM_INLINEDIRRTL: u32 = 2355u32;
pub const IDM_INSERTHTML: u32 = 2502u32;
pub const IDM_INSERTOBJECT: u32 = 25u32;
pub const IDM_INSERTSPAN: u32 = 2357u32;
pub const IDM_INSERTTEXT: u32 = 3907u32;
pub const IDM_INSFIELDSET: u32 = 2119u32;
pub const IDM_INSINPUTBUTTON: u32 = 2115u32;
pub const IDM_INSINPUTHIDDEN: u32 = 2312u32;
pub const IDM_INSINPUTIMAGE: u32 = 2114u32;
pub const IDM_INSINPUTPASSWORD: u32 = 2313u32;
pub const IDM_INSINPUTRESET: u32 = 2116u32;
pub const IDM_INSINPUTSUBMIT: u32 = 2117u32;
pub const IDM_INSINPUTUPLOAD: u32 = 2118u32;
pub const IDM_INSPECTELEMENT: u32 = 3904u32;
pub const IDM_INVOKEFLIPAHEADTARGET: u32 = 15200u32;
pub const IDM_ISTRUSTEDDLG: u32 = 2356u32;
pub const IDM_ITALIC: u32 = 56u32;
pub const IDM_JAVAAPPLET: u32 = 2175u32;
pub const IDM_JUSTIFYCENTER: u32 = 57u32;
pub const IDM_JUSTIFYFULL: u32 = 50u32;
pub const IDM_JUSTIFYGENERAL: u32 = 58u32;
pub const IDM_JUSTIFYLEFT: u32 = 59u32;
pub const IDM_JUSTIFYNONE: u32 = 94u32;
pub const IDM_JUSTIFYRIGHT: u32 = 60u32;
pub const IDM_KEEPSELECTION: u32 = 2410u32;
pub const IDM_LANGUAGE: u32 = 2292u32;
pub const IDM_LAUNCHDEBUGGER: u32 = 2310u32;
pub const IDM_LAUNCHURICALLBACK: u32 = 3908u32;
pub const IDM_LEFTALIGNPARA: u32 = 2251u32;
pub const IDM_LINEBREAKBOTH: u32 = 2154u32;
pub const IDM_LINEBREAKLEFT: u32 = 2152u32;
pub const IDM_LINEBREAKNORMAL: u32 = 2151u32;
pub const IDM_LINEBREAKRIGHT: u32 = 2153u32;
pub const IDM_LIST: u32 = 2183u32;
pub const IDM_LISTBOX: u32 = 2166u32;
pub const IDM_LIVERESIZE: u32 = 2398u32;
pub const IDM_LOCALIZEEDITOR: u32 = 2358u32;
pub const IDM_MARQUEE: u32 = 2182u32;
pub const IDM_MEDIA_FRAMESTEP_BACK: u32 = 2461u32;
pub const IDM_MEDIA_FRAMESTEP_FWD: u32 = 2460u32;
pub const IDM_MEDIA_FULLSCREEN_EXIT: u32 = 2447u32;
pub const IDM_MEDIA_FULLSCREEN_TOGGLE: u32 = 2446u32;
pub const IDM_MEDIA_MUTE: u32 = 2462u32;
pub const IDM_MEDIA_MUTEUNMUTE: u32 = 2442u32;
pub const IDM_MEDIA_PAUSE: u32 = 2444u32;
pub const IDM_MEDIA_PLAY: u32 = 2443u32;
pub const IDM_MEDIA_PLAYPAUSE: u32 = 2441u32;
pub const IDM_MEDIA_PLAYRATE0: u32 = 2480u32;
pub const IDM_MEDIA_PLAYRATE1: u32 = 2481u32;
pub const IDM_MEDIA_PLAYRATE2: u32 = 2482u32;
pub const IDM_MEDIA_PLAYRATE3: u32 = 2483u32;
pub const IDM_MEDIA_PLAYRATE4: u32 = 2484u32;
pub const IDM_MEDIA_PLAYRATE5: u32 = 2485u32;
pub const IDM_MEDIA_PLAYRATE6: u32 = 2486u32;
pub const IDM_MEDIA_PLAYRATE7: u32 = 2487u32;
pub const IDM_MEDIA_PLAYRATE8: u32 = 2488u32;
pub const IDM_MEDIA_PLAYRATE9: u32 = 2489u32;
pub const IDM_MEDIA_RATE_FASTER: u32 = 2456u32;
pub const IDM_MEDIA_RATE_SLOWER: u32 = 2457u32;
pub const IDM_MEDIA_SEEK_BACK_LARGE: u32 = 2455u32;
pub const IDM_MEDIA_SEEK_BACK_SMALL: u32 = 2453u32;
pub const IDM_MEDIA_SEEK_FWD_LARGE: u32 = 2454u32;
pub const IDM_MEDIA_SEEK_FWD_SMALL: u32 = 2452u32;
pub const IDM_MEDIA_SEEK_TO_END: u32 = 2451u32;
pub const IDM_MEDIA_SEEK_TO_START: u32 = 2450u32;
pub const IDM_MEDIA_SHOWCONTROLS_TOGGLE: u32 = 2458u32;
pub const IDM_MEDIA_SHOW_AUDIO_ACCESS: u32 = 2464u32;
pub const IDM_MEDIA_SHOW_SUBTITLE_ACCESS: u32 = 2465u32;
pub const IDM_MEDIA_STOP: u32 = 2445u32;
pub const IDM_MEDIA_UNMUTE: u32 = 2463u32;
pub const IDM_MEDIA_VOLUME_DOWN: u32 = 2449u32;
pub const IDM_MEDIA_VOLUME_UP: u32 = 2448u32;
pub const IDM_MEDIA_ZOOMMODE_TOGGLE: u32 = 2459u32;
pub const IDM_MENUEXT_COUNT: u32 = 3733u32;
pub const IDM_MENUEXT_FIRST__: u32 = 3700u32;
pub const IDM_MENUEXT_LAST__: u32 = 3732u32;
pub const IDM_MENUEXT_PLACEHOLDER: u32 = 6047u32;
pub const IDM_MIMECSET__FIRST__: u32 = 3609u32;
pub const IDM_MIMECSET__LAST__: u32 = 3699u32;
pub const IDM_MOVE: u32 = 88u32;
pub const IDM_MP_EMAILPICTURE: u32 = 2288u32;
pub const IDM_MP_MYPICS: u32 = 2287u32;
pub const IDM_MP_PRINTPICTURE: u32 = 2289u32;
pub const IDM_MULTILEVELREDO: u32 = 30u32;
pub const IDM_MULTILEVELUNDO: u32 = 44u32;
pub const IDM_MULTIPLESELECTION: u32 = 2393u32;
pub const IDM_NEW: u32 = 2001u32;
pub const IDM_NEWPAGE: u32 = 87u32;
pub const IDM_NEW_TOPLEVELWINDOW: u32 = 7050u32;
pub const IDM_NOACTIVATEDESIGNTIMECONTROLS: u32 = 2333u32;
pub const IDM_NOACTIVATEJAVAAPPLETS: u32 = 2334u32;
pub const IDM_NOACTIVATENORMALOLECONTROLS: u32 = 2332u32;
pub const IDM_NOFIXUPURLSONPASTE: u32 = 2335u32;
pub const IDM_NONBREAK: u32 = 2155u32;
pub const IDM_NONEHIGHLIGHT: u32 = 15219u32;
pub const IDM_NOTIFYCONTEXTMENUDISMISSED: u32 = 15046u32;
pub const IDM_NOTIFYZOOMANDSCROLLANIMATIONEND: u32 = 15045u32;
pub const IDM_OBJECT: u32 = 2169u32;
pub const IDM_OBJECTVERBLIST0: u32 = 72u32;
pub const IDM_OBJECTVERBLIST1: u32 = 73u32;
pub const IDM_OBJECTVERBLIST2: u32 = 74u32;
pub const IDM_OBJECTVERBLIST3: u32 = 75u32;
pub const IDM_OBJECTVERBLIST4: u32 = 76u32;
pub const IDM_OBJECTVERBLIST5: u32 = 77u32;
pub const IDM_OBJECTVERBLIST6: u32 = 78u32;
pub const IDM_OBJECTVERBLIST7: u32 = 79u32;
pub const IDM_OBJECTVERBLIST8: u32 = 80u32;
pub const IDM_OBJECTVERBLIST9: u32 = 81u32;
pub const IDM_OBJECTVERBLISTLAST: u32 = 81u32;
pub const IDM_OLEWINDOWSTATECHANGED: u32 = 15006u32;
pub const IDM_OPEN: u32 = 2000u32;
pub const IDM_OPENPDFNOTE: u32 = 15213u32;
pub const IDM_OPTIONS: u32 = 2135u32;
pub const IDM_ORDERLIST: u32 = 2184u32;
pub const IDM_OUTDENT: u32 = 2187u32;
pub const IDM_OVERRIDE_CURSOR: u32 = 2420u32;
pub const IDM_OVERWRITE: u32 = 2314u32;
pub const IDM_PAGE: u32 = 2267u32;
pub const IDM_PAGEBREAK: u32 = 2177u32;
pub const IDM_PAGEINFO: u32 = 2231u32;
pub const IDM_PAGESETUP: u32 = 2004u32;
pub const IDM_PARAGRAPH: u32 = 2180u32;
pub const IDM_PARSECOMPLETE: u32 = 2315u32;
pub const IDM_PASTE: u32 = 26u32;
pub const IDM_PASTECONTENTONLY: u32 = 2500u32;
pub const IDM_PASTEFORMAT: u32 = 2238u32;
pub const IDM_PASTEINSERT: u32 = 2120u32;
pub const IDM_PASTESPECIAL: u32 = 2006u32;
pub const IDM_PASTETEXTONLY: u32 = 2501u32;
pub const IDM_PDFDEFINE: u32 = 15222u32;
pub const IDM_PDFREADALOUD: u32 = 15220u32;
pub const IDM_PEERHITTESTSAMEINEDIT: u32 = 2423u32;
pub const IDM_PERFORMEDITACTIVATION: u32 = 15042u32;
pub const IDM_PERSISTDEFAULTVALUES: u32 = 7100u32;
pub const IDM_PERSISTSTREAMSYNC: u32 = 2341u32;
pub const IDM_PINKHIGHLIGHT: u32 = 15215u32;
pub const IDM_PLUGIN: u32 = 2176u32;
pub const IDM_POPSTATEEVENT: u32 = 15017u32;
pub const IDM_PREFORMATTED: u32 = 2188u32;
pub const IDM_PRESERVEUNDOALWAYS: u32 = 6049u32;
pub const IDM_PRESTOP: u32 = 2284u32;
pub const IDM_PRINT: u32 = 27u32;
pub const IDM_PRINTPREVIEW: u32 = 2003u32;
pub const IDM_PRINTQUERYJOBSPENDING: u32 = 2277u32;
pub const IDM_PRINTTARGET: u32 = 2273u32;
pub const IDM_PROPERTIES: u32 = 28u32;
pub const IDM_PROTECTMETATAGS: u32 = 7101u32;
pub const IDM_RADIOBUTTON: u32 = 2164u32;
pub const IDM_RAISED: u32 = 61u32;
pub const IDM_RCINSERT: u32 = 2201u32;
pub const IDM_REDO: u32 = 29u32;
pub const IDM_REFRESH: u32 = 2300u32;
pub const IDM_REFRESH_THIS: u32 = 6042u32;
pub const IDM_REGISTRYREFRESH: u32 = 2317u32;
pub const IDM_REMOVECONSOLEMESSAGERECEIVER: u32 = 3801u32;
pub const IDM_REMOVEDEBUGCALLBACKRECEIVER: u32 = 3805u32;
pub const IDM_REMOVEFORMAT: u32 = 2230u32;
pub const IDM_REMOVEFROMGLYPHTABLE: u32 = 2338u32;
pub const IDM_REMOVEPARAFORMAT: u32 = 2253u32;
pub const IDM_RENAME: u32 = 85u32;
pub const IDM_REPLACE: u32 = 2121u32;
pub const IDM_REPLACEGLYPHCONTENTS: u32 = 2339u32;
pub const IDM_RESPECTVISIBILITY_INDESIGN: u32 = 2405u32;
pub const IDM_RIGHTALIGNPARA: u32 = 2252u32;
pub const IDM_ROWINSERT: u32 = 2212u32;
pub const IDM_ROWSELECT: u32 = 2207u32;
pub const IDM_RUNFLASH: u32 = 15208u32;
pub const IDM_RUNURLSCRIPT: u32 = 2343u32;
pub const IDM_SAVE: u32 = 70u32;
pub const IDM_SAVEAS: u32 = 71u32;
pub const IDM_SAVEBACKGROUND: u32 = 2263u32;
pub const IDM_SAVECOPYAS: u32 = 2002u32;
pub const IDM_SAVEPDF: u32 = 99u32;
pub const IDM_SAVEPICTURE: u32 = 2270u32;
pub const IDM_SAVEPRETRANSFORMSOURCE: u32 = 2370u32;
pub const IDM_SAVETARGET: u32 = 2268u32;
pub const IDM_SCRIPT: u32 = 2174u32;
pub const IDM_SCRIPTDEBUGGER: u32 = 2330u32;
pub const IDM_SCROLL_BOTTOM: u32 = 2382u32;
pub const IDM_SCROLL_DOWN: u32 = 2386u32;
pub const IDM_SCROLL_HERE: u32 = 2380u32;
pub const IDM_SCROLL_LEFT: u32 = 2391u32;
pub const IDM_SCROLL_LEFTEDGE: u32 = 2387u32;
pub const IDM_SCROLL_PAGEDOWN: u32 = 2384u32;
pub const IDM_SCROLL_PAGELEFT: u32 = 2389u32;
pub const IDM_SCROLL_PAGERIGHT: u32 = 2390u32;
pub const IDM_SCROLL_PAGEUP: u32 = 2383u32;
pub const IDM_SCROLL_RIGHT: u32 = 2392u32;
pub const IDM_SCROLL_RIGHTEDGE: u32 = 2388u32;
pub const IDM_SCROLL_TOP: u32 = 2381u32;
pub const IDM_SCROLL_UP: u32 = 2385u32;
pub const IDM_SELECTALL: u32 = 31u32;
pub const IDM_SELECTIONSEARCH: u32 = 15206u32;
pub const IDM_SENDBACKWARD: u32 = 32u32;
pub const IDM_SENDTOBACK: u32 = 33u32;
pub const IDM_SETACCESSIBILITYNAME: u32 = 15040u32;
pub const IDM_SETCUSTOMCURSOR: u32 = 2434u32;
pub const IDM_SETDEFAULTBACKGROUNDCOLOR: u32 = 15043u32;
pub const IDM_SETDESKTOPITEM: u32 = 2278u32;
pub const IDM_SETDEVTOOLBARCONSOLE: u32 = 15016u32;
pub const IDM_SETDIRTY: u32 = 2342u32;
pub const IDM_SETEXTRAHEADERS: u32 = 15039u32;
pub const IDM_SETGEOLOCATIONCONSENT: u32 = 15029u32;
pub const IDM_SETL9QUIRKSEMULATIONENABLED: u32 = 15024u32;
pub const IDM_SETNAVIGATEEVENTSINK: u32 = 15013u32;
pub const IDM_SETPAGEACTIONALLOWEDFLAGS: u32 = 15100u32;
pub const IDM_SETPARTIALLAYOUTSTATUS: u32 = 15021u32;
pub const IDM_SETPOINTERLOCKCONSENT: u32 = 15026u32;
pub const IDM_SETPRINTHANDLES: u32 = 15002u32;
pub const IDM_SETPRINTTEMPLATE: u32 = 2296u32;
pub const IDM_SETPROFILINGONSTART: u32 = 15010u32;
pub const IDM_SETSCRIPTCONSOLE: u32 = 15012u32;
pub const IDM_SETSESSIONDOCUMENTMODE: u32 = 15008u32;
pub const IDM_SETWALLPAPER: u32 = 2264u32;
pub const IDM_SHADOWED: u32 = 66u32;
pub const IDM_SHARE: u32 = 15031u32;
pub const IDM_SHAREAPPCACHEEVENT: u32 = 15033u32;
pub const IDM_SHAREPDF: u32 = 15221u32;
pub const IDM_SHAREPICTURE: u32 = 3905u32;
pub const IDM_SHOWALIGNEDSITETAGS: u32 = 2321u32;
pub const IDM_SHOWALLTAGS: u32 = 2327u32;
pub const IDM_SHOWAREATAGS: u32 = 2325u32;
pub const IDM_SHOWCOMMENTTAGS: u32 = 2324u32;
pub const IDM_SHOWGRID: u32 = 69u32;
pub const IDM_SHOWHIDE_CODE: u32 = 2235u32;
pub const IDM_SHOWMISCTAGS: u32 = 2320u32;
pub const IDM_SHOWPAGESETUP: u32 = 2011u32;
pub const IDM_SHOWPICTURE: u32 = 2269u32;
pub const IDM_SHOWPRINT: u32 = 2010u32;
pub const IDM_SHOWSCRIPTTAGS: u32 = 2322u32;
pub const IDM_SHOWSHAREUI: u32 = 15207u32;
pub const IDM_SHOWSPECIALCHAR: u32 = 2249u32;
pub const IDM_SHOWSTYLETAGS: u32 = 2323u32;
pub const IDM_SHOWTABLE: u32 = 34u32;
pub const IDM_SHOWUNKNOWNTAGS: u32 = 2326u32;
pub const IDM_SHOWWBRTAGS: u32 = 2340u32;
pub const IDM_SHOWZEROBORDERATDESIGNTIME: u32 = 2328u32;
pub const IDM_SIZETOCONTROL: u32 = 35u32;
pub const IDM_SIZETOCONTROLHEIGHT: u32 = 36u32;
pub const IDM_SIZETOCONTROLWIDTH: u32 = 37u32;
pub const IDM_SIZETOFIT: u32 = 38u32;
pub const IDM_SIZETOGRID: u32 = 39u32;
pub const IDM_SNAPTOGRID: u32 = 40u32;
pub const IDM_SPECIALCHAR: u32 = 2156u32;
pub const IDM_SPELL: u32 = 2005u32;
pub const IDM_STARTDIAGNOSTICSMODE: u32 = 3802u32;
pub const IDM_STATUSBAR: u32 = 2131u32;
pub const IDM_STOP: u32 = 2138u32;
pub const IDM_STOPDOWNLOAD: u32 = 2301u32;
pub const IDM_STRIKETHROUGH: u32 = 91u32;
pub const IDM_STYLEMENU_CHANGESELECTEDSTYLE: u32 = 2440u32;
pub const IDM_STYLEMENU_GETNOSTYLE: u32 = 2438u32;
pub const IDM_STYLEMENU_GETPREFSTYLE: u32 = 2439u32;
pub const IDM_STYLEMENU_SETNOSTYLE: u32 = 2437u32;
pub const IDM_SUBSCRIPT: u32 = 2247u32;
pub const IDM_SUNKEN: u32 = 62u32;
pub const IDM_SUPERSCRIPT: u32 = 2248u32;
pub const IDM_TABLE: u32 = 2236u32;
pub const IDM_TABLEINSERT: u32 = 2200u32;
pub const IDM_TABLEPROPERTIES: u32 = 2210u32;
pub const IDM_TABLESELECT: u32 = 2209u32;
pub const IDM_TABORDER: u32 = 41u32;
pub const IDM_TELETYPE: u32 = 2232u32;
pub const IDM_TEMPLATE_PAGESETUP: u32 = 2298u32;
pub const IDM_TEXTAREA: u32 = 2162u32;
pub const IDM_TEXTBOX: u32 = 2161u32;
pub const IDM_TEXTONLY: u32 = 2133u32;
pub const IDM_TOGGLEREADINGBAR: u32 = 15209u32;
pub const IDM_TOOLBARS: u32 = 2130u32;
pub const IDM_TOOLBOX: u32 = 42u32;
pub const IDM_TRISTATEBOLD: u32 = 95u32;
pub const IDM_TRISTATEITALIC: u32 = 96u32;
pub const IDM_TRISTATEUNDERLINE: u32 = 97u32;
pub const IDM_TRUSTAPPCACHE: u32 = 2425u32;
pub const IDM_UI_OUTDENT: u32 = 2407u32;
pub const IDM_UNBOOKMARK: u32 = 2128u32;
pub const IDM_UNDERLINE: u32 = 63u32;
pub const IDM_UNDO: u32 = 43u32;
pub const IDM_UNGROUP: u32 = 45u32;
pub const IDM_UNKNOWN: u32 = 0u32;
pub const IDM_UNLINK: u32 = 2125u32;
pub const IDM_UNLOADDOCUMENT: u32 = 2411u32;
pub const IDM_UNORDERLIST: u32 = 2185u32;
pub const IDM_UPDATEPAGESTATUS: u32 = 2408u32;
pub const IDM_UPDATESETTINGSFROMREGISTRY: u32 = 15041u32;
pub const IDM_VERTSPACECONCATENATE: u32 = 46u32;
pub const IDM_VERTSPACEDECREASE: u32 = 47u32;
pub const IDM_VERTSPACEINCREASE: u32 = 48u32;
pub const IDM_VERTSPACEMAKEEQUAL: u32 = 49u32;
pub const IDM_VIEWPRETRANSFORMSOURCE: u32 = 2371u32;
pub const IDM_VIEWSOURCE: u32 = 2139u32;
pub const IDM_YELLOWHIGHLIGHT: u32 = 15218u32;
pub const IDM_ZOOMPERCENT: u32 = 50u32;
pub const IDM_ZOOMPOPUP: u32 = 2140u32;
pub const IDM_ZOOMRATIO: u32 = 2344u32;
#[repr(transparent)]
pub struct IDOMBeforeUnloadEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMCloseEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMCompositionEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMCustomEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMDocumentType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMDragEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMEventRegistrationCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMFocusEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMKeyboardEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMMSAnimationEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMMSManipulationEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMMSTransitionEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMMessageEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMMouseEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMMouseWheelEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMMutationEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMNodeIterator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMParser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMParserFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMProcessingInstruction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMProgressEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMSiteModeEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMStorageEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMTextEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMTreeWalker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMUIEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMWheelEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMXmlSerializer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDOMXmlSerializerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugCallbackNotificationHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeveloperConsoleMessageReceiver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceRect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiagnosticsScriptEngine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiagnosticsScriptEngineProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiagnosticsScriptEngineSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayPointer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDithererImpl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDocHostShowUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDocHostUIHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDocHostUIHandler2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDocObjectService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDocumentEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDocumentRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDocumentSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDocumentTraversal(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadManager(pub *mut ::core::ffi::c_void);
pub const IECMDID_ARG_CLEAR_FORMS_ALL: u32 = 0u32;
pub const IECMDID_ARG_CLEAR_FORMS_ALL_BUT_PASSWORDS: u32 = 1u32;
pub const IECMDID_ARG_CLEAR_FORMS_PASSWORDS_ONLY: u32 = 2u32;
pub const IECMDID_BEFORENAVIGATE_DOEXTERNALBROWSE: u32 = 3u32;
pub const IECMDID_BEFORENAVIGATE_GETIDLIST: u32 = 4u32;
pub const IECMDID_BEFORENAVIGATE_GETSHELLBROWSE: u32 = 2u32;
pub const IECMDID_CLEAR_AUTOCOMPLETE_FOR_FORMS: u32 = 0u32;
pub const IECMDID_GET_INVOKE_DEFAULT_BROWSER_ON_NEW_WINDOW: u32 = 6u32;
pub const IECMDID_SETID_AUTOCOMPLETE_FOR_FORMS: u32 = 1u32;
pub const IECMDID_SET_INVOKE_DEFAULT_BROWSER_ON_NEW_WINDOW: u32 = 5u32;
#[cfg(feature = "Win32_Foundation")]
pub type IEISXMLNSREGISTEREDFN = unsafe extern "system" fn(lpszuri: super::super::Foundation::PWSTR, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
#[repr(transparent)]
pub struct IELAUNCHOPTION_FLAGS(pub i32);
pub const IELAUNCHOPTION_SCRIPTDEBUG: IELAUNCHOPTION_FLAGS = IELAUNCHOPTION_FLAGS(1i32);
pub const IELAUNCHOPTION_FORCE_COMPAT: IELAUNCHOPTION_FLAGS = IELAUNCHOPTION_FLAGS(2i32);
pub const IELAUNCHOPTION_FORCE_EDGE: IELAUNCHOPTION_FLAGS = IELAUNCHOPTION_FLAGS(4i32);
pub const IELAUNCHOPTION_LOCK_ENGINE: IELAUNCHOPTION_FLAGS = IELAUNCHOPTION_FLAGS(8i32);
impl ::core::marker::Copy for IELAUNCHOPTION_FLAGS {}
impl ::core::clone::Clone for IELAUNCHOPTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IELAUNCHURLINFO {
    pub cbSize: u32,
    pub dwCreationFlags: u32,
    pub dwLaunchOptionFlags: u32,
}
impl ::core::marker::Copy for IELAUNCHURLINFO {}
impl ::core::clone::Clone for IELAUNCHURLINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type IEREGISTERXMLNSFN = unsafe extern "system" fn(lpszuri: super::super::Foundation::PWSTR, clsid: ::windows_sys::core::GUID, fmachine: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
pub const IEWebDriverManager: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2419149554,
    data2: 21072,
    data3: 18355,
    data4: [137, 216, 98, 149, 252, 35, 188, 34],
};
pub const IE_USE_OE_MAIL_HKEY: i32 = -2147483647i32;
pub const IE_USE_OE_NEWS_HKEY: i32 = -2147483647i32;
pub const IE_USE_OE_PRESENT_HKEY: i32 = -2147483646i32;
#[repr(transparent)]
pub struct IElementBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementBehaviorCategory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementBehaviorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementBehaviorFocus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementBehaviorLayout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementBehaviorLayout2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementBehaviorRender(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementBehaviorSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementBehaviorSiteCategory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementBehaviorSiteLayout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementBehaviorSiteLayout2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementBehaviorSiteOM(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementBehaviorSiteOM2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementBehaviorSiteRender(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementBehaviorSubmit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementNamespace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementNamespaceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementNamespaceFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementNamespaceFactoryCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementNamespaceTable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementTraversal(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumManagerFrames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumOpenServiceActivity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumOpenServiceActivityCategory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumPrivacyRecords(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumSTATURL(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEventException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEventTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEventTarget2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtensionValidation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontNames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGetSVGDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTCAttachBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTCAttachBehavior2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTCDefaultDispatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTCDescBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTCEventBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTCMethodBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTCPropertyBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAnchorElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAnchorElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAnchorElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAppBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAppBehavior2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAppBehavior3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLApplicationCache(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAreaElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAreaElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAreasCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAreasCollection2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAreasCollection3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAreasCollection4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAttributeCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAttributeCollection2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAttributeCollection3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAttributeCollection4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAudioElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLAudioElementFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLBGsound(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLBRElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLBaseElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLBaseElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLBaseFontElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLBlockElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLBlockElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLBlockElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLBodyElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLBodyElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLBodyElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLBodyElement4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLBodyElement5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLBookmarkCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLButtonElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLButtonElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCSSImportRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCSSMediaList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCSSMediaRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCSSNamespaceRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCSSRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCSSStyleDeclaration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCSSStyleDeclaration2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCSSStyleDeclaration3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCSSStyleDeclaration4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCanvasElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCaret(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLChangeLog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLChangePlayback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLChangeSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCommentElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCommentElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCommentElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLComputedStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLControlElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLControlRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLControlRange2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCurrentStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCurrentStyle2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCurrentStyle3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCurrentStyle4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLCurrentStyle5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDDElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDListElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMAttribute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMAttribute2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMAttribute3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMAttribute4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMChildrenCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMChildrenCollection2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMConstructor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMConstructorCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMImplementation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMImplementation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMNode2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMNode3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMTextNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMTextNode2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDOMTextNode3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDTElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDataTransfer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDatabinding(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDialog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDialog2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDialog3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDivElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDivPosition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDocument2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDocument3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDocument4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDocument5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDocument6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDocument7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDocument8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDocumentCompatibleInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLDocumentCompatibleInfoCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLEditDesigner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLEditHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLEditHost2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLEditServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLEditServices2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLElement4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLElement5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLElement6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLElement7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLElementAppliedStyles(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLElementCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLElementCollection2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLElementCollection3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLElementCollection4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLElementDefaults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLElementRender(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLEmbedElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLEmbedElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLEventObj(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLEventObj2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLEventObj3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLEventObj4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLEventObj5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLEventObj6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFieldSetElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFieldSetElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFiltersCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFontElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFontNamesCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFontSizesCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFormElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFormElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFormElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFormElement4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFrameBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFrameBase2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFrameBase3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFrameElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFrameElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFrameElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFrameSetElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFrameSetElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFrameSetElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLFramesCollection2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLGenericElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLHRElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLHeadElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLHeadElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLHeaderElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLHtmlElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLIFrameElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLIFrameElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLIFrameElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLIPrintCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLImageElementFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLImgElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLImgElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLImgElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLImgElement4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLInputButtonElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLInputElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLInputElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLInputElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLInputFileElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLInputHiddenElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLInputImage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLInputRangeElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLInputTextElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLInputTextElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLIsIndexElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLIsIndexElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLLIElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLLabelElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLLabelElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLLegendElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLLegendElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLLinkElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLLinkElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLLinkElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLLinkElement4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLLinkElement5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLListElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLListElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLMSCSSKeyframeRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLMSCSSKeyframesRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLMSImgElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLMSMediaElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLMapElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLMarqueeElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLMediaElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLMediaElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLMediaError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLMetaElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLMetaElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLMetaElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLMimeTypesCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLModelessInit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLNamespace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLNamespaceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLNextIdElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLNoShowElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLOListElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLOMWindowServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLObjectElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLObjectElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLObjectElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLObjectElement4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLObjectElement5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLOpsProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLOptionButtonElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLOptionElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLOptionElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLOptionElement4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLOptionElementFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLOptionsHolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLPaintSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLPainter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLPainterEventInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLPainterOverlay(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLParaElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLParamElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLParamElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLPerformance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLPerformanceNavigation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLPerformanceTiming(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLPersistData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLPersistDataOM(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLPhraseElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLPhraseElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLPhraseElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLPluginsCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLPopup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLProgressElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLRect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLRect2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLRectCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLRenderStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLRuleStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLRuleStyle2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLRuleStyle3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLRuleStyle4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLRuleStyle5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLRuleStyle6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLScreen(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLScreen2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLScreen3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLScreen4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLScriptElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLScriptElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLScriptElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLScriptElement4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLSelectElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLSelectElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLSelectElement4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLSelectElement5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLSelectElement6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLSelectElementEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLSelection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLSelectionObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLSelectionObject2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLSourceElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLSpanElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLSpanFlow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStorage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStorage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyle2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyle3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyle4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyle5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyle6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleEnabled(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleFontFace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleFontFace2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleMedia(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleSheet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleSheet2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleSheet3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleSheet4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleSheetPage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleSheetPage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleSheetPagesCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleSheetRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleSheetRule2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleSheetRuleApplied(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleSheetRulesAppliedCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleSheetRulesCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleSheetRulesCollection2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleSheetsCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLStyleSheetsCollection2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLSubmitData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTable2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTable3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTable4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTableCaption(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTableCell(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTableCell2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTableCell3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTableCol(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTableCol2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTableCol3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTableRow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTableRow2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTableRow3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTableRow4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTableRowMetrics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTableSection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTableSection2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTableSection3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTableSection4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTextAreaElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTextAreaElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTextContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTextElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTextRangeMetrics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTextRangeMetrics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTimeRanges(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTimeRanges2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTitleElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTxtRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLTxtRangeCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLUListElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLUniqueName(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLUnknownElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLUrnCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLUserDataOM(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLVideoElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLWindow2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLWindow3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLWindow4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLWindow5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLWindow6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLWindow7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLWindow8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLXDomainRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLXDomainRequestFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLXMLHttpRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLXMLHttpRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHTMLXMLHttpRequestFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHeaderFooter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHeaderFooter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHighlightRenderingServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHighlightSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHomePage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHomePageSetting(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHostBehaviorInit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHostDialogHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHtmlDlgSafeHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IICCSVGColor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIE70DispatchEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIE80DispatchEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIEWebDriverManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIEWebDriverSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIMEServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageDecodeEventSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageDecodeEventSink2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageDecodeFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIntelliForms(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInternetExplorerManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInternetExplorerManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILayoutRect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineInfo(pub *mut ::core::ffi::c_void);
pub const IMGDECODE_EVENT_BEGINBITS: u32 = 4u32;
pub const IMGDECODE_EVENT_BITSCOMPLETE: u32 = 8u32;
pub const IMGDECODE_EVENT_PALETTE: u32 = 2u32;
pub const IMGDECODE_EVENT_PROGRESS: u32 = 1u32;
pub const IMGDECODE_EVENT_USEDDRAW: u32 = 16u32;
pub const IMGDECODE_HINT_BOTTOMUP: u32 = 2u32;
pub const IMGDECODE_HINT_FULLWIDTH: u32 = 4u32;
pub const IMGDECODE_HINT_TOPDOWN: u32 = 1u32;
#[repr(transparent)]
pub struct IMapMIMEToCLSID(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMarkupContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMarkupContainer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMarkupPointer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMarkupPointer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMarkupServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMarkupServices2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMarkupTextFrags(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaActivityNotifySite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INTERNETEXPLORERCONFIGURATION(pub i32);
pub const INTERNETEXPLORERCONFIGURATION_HOST: INTERNETEXPLORERCONFIGURATION = INTERNETEXPLORERCONFIGURATION(1i32);
pub const INTERNETEXPLORERCONFIGURATION_WEB_DRIVER: INTERNETEXPLORERCONFIGURATION = INTERNETEXPLORERCONFIGURATION(2i32);
pub const INTERNETEXPLORERCONFIGURATION_WEB_DRIVER_EDGE: INTERNETEXPLORERCONFIGURATION = INTERNETEXPLORERCONFIGURATION(4i32);
impl ::core::marker::Copy for INTERNETEXPLORERCONFIGURATION {}
impl ::core::clone::Clone for INTERNETEXPLORERCONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigatorDoNotTrack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigatorGeolocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOmHistory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOmNavigator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOpenService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOpenServiceActivity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOpenServiceActivityCategory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOpenServiceActivityInput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOpenServiceActivityManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOpenServiceActivityOutputContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOpenServiceManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersistHistory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintManagerTemplatePrinter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintManagerTemplatePrinter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskRequestFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskRequestHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRangeException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRulesApplied(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRulesAppliedCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGAElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGAngle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGAnimatedAngle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGAnimatedBoolean(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGAnimatedEnumeration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGAnimatedInteger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGAnimatedLength(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGAnimatedLengthList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGAnimatedNumber(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGAnimatedNumberList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGAnimatedPathData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGAnimatedPoints(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGAnimatedPreserveAspectRatio(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGAnimatedRect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGAnimatedString(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGAnimatedTransformList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGCircleElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGClipPathElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGDefsElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGDescElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGElementInstance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGElementInstanceList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGEllipseElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGExternalResourcesRequired(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGFitToViewBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGGElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGGradientElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGImageElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGLangSpace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGLength(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGLengthList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGLineElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGLinearGradientElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGLocatable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGMarkerElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGMaskElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGMatrix(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGMetadataElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGNumber(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGNumberList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPaint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSeg(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegArcAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegArcRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegClosePath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegCurvetoCubicAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegCurvetoCubicRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegCurvetoCubicSmoothAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegCurvetoCubicSmoothRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegCurvetoQuadraticAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegCurvetoQuadraticRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegCurvetoQuadraticSmoothAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegCurvetoQuadraticSmoothRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegLinetoAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegLinetoHorizontalAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegLinetoHorizontalRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegLinetoRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegLinetoVerticalAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegLinetoVerticalRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegMovetoAbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPathSegMovetoRel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPatternElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPoint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPointList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPolygonElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPolylineElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGPreserveAspectRatio(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGRadialGradientElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGRect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGRectElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGSVGElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGScriptElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGStopElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGStringList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGStylable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGStyleElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGSwitchElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGSymbolElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGTSpanElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGTests(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGTextContentElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGTextElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGTextPathElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGTextPositioningElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGTitleElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGTransformList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGTransformable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGURIReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGUseElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGViewElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGViewSpec(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGZoomAndPan(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISVGZoomEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScriptEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScriptEventHandlerSourceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollableContextMenu(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollableContextMenu2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecureUrlHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISegmentList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISegmentListIterator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectionServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectionServicesListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISequenceNumber(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISniffStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISurfacePresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISurfacePresenterFlip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISurfacePresenterFlip2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISurfacePresenterFlipBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetEmbedding(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetFrame2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetFramePriv(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetFramePriv2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetNotify2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITemplatePrinter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITemplatePrinter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITemplatePrinter3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimerEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimerService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimerSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITrackingProtection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITridentTouchInput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITridentTouchInputSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUrlHistoryNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUrlHistoryStg(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUrlHistoryStg2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewObjectPresentFlip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewObjectPresentFlip2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewObjectPresentFlipSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewObjectPresentFlipSite2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewObjectPresentNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewObjectPresentNotifySite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewObjectPresentSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewObjectPrint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWBScriptControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWPCBlockedUrls(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebBridge(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebBrowserEventsService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebBrowserEventsUrlService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebGeocoordinates(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebGeolocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebGeoposition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebGeopositionError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXMLGenericParse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXMLHttpRequestEventTarget(pub *mut ::core::ffi::c_void);
pub const IntelliForms: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1631238446, data2: 5823, data3: 4562, data4: [188, 165, 0, 192, 79, 217, 41, 219] };
pub const InternetExplorerManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3746548788, data2: 1658, data3: 19978, data4: [131, 82, 74, 26, 80, 149, 52, 110] };
#[repr(transparent)]
pub struct Iwfolders(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LINE_DIRECTION(pub i32);
pub const LINE_DIRECTION_RightToLeft: LINE_DIRECTION = LINE_DIRECTION(1i32);
pub const LINE_DIRECTION_LeftToRight: LINE_DIRECTION = LINE_DIRECTION(2i32);
pub const LINE_DIRECTION_Max: LINE_DIRECTION = LINE_DIRECTION(2147483647i32);
impl ::core::marker::Copy for LINE_DIRECTION {}
impl ::core::clone::Clone for LINE_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const LINKSBAND: u32 = 4u32;
#[repr(C)]
pub struct LayoutRectEvents(pub u8);
pub const MAPMIME_CLSID: u32 = 1u32;
pub const MAPMIME_DEFAULT: u32 = 0u32;
pub const MAPMIME_DEFAULT_ALWAYS: u32 = 3u32;
pub const MAPMIME_DISABLE: u32 = 2u32;
#[repr(transparent)]
pub struct MARKUP_CONTEXT_TYPE(pub i32);
pub const CONTEXT_TYPE_None: MARKUP_CONTEXT_TYPE = MARKUP_CONTEXT_TYPE(0i32);
pub const CONTEXT_TYPE_Text: MARKUP_CONTEXT_TYPE = MARKUP_CONTEXT_TYPE(1i32);
pub const CONTEXT_TYPE_EnterScope: MARKUP_CONTEXT_TYPE = MARKUP_CONTEXT_TYPE(2i32);
pub const CONTEXT_TYPE_ExitScope: MARKUP_CONTEXT_TYPE = MARKUP_CONTEXT_TYPE(3i32);
pub const CONTEXT_TYPE_NoScope: MARKUP_CONTEXT_TYPE = MARKUP_CONTEXT_TYPE(4i32);
pub const MARKUP_CONTEXT_TYPE_Max: MARKUP_CONTEXT_TYPE = MARKUP_CONTEXT_TYPE(2147483647i32);
impl ::core::marker::Copy for MARKUP_CONTEXT_TYPE {}
impl ::core::clone::Clone for MARKUP_CONTEXT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MAX_SEARCH_FORMAT_STRING: u32 = 255u32;
pub const MENUEXT_SHOWDIALOG: u32 = 1u32;
#[repr(transparent)]
pub struct MOVEUNIT_ACTION(pub i32);
pub const MOVEUNIT_PREVCHAR: MOVEUNIT_ACTION = MOVEUNIT_ACTION(0i32);
pub const MOVEUNIT_NEXTCHAR: MOVEUNIT_ACTION = MOVEUNIT_ACTION(1i32);
pub const MOVEUNIT_PREVCLUSTERBEGIN: MOVEUNIT_ACTION = MOVEUNIT_ACTION(2i32);
pub const MOVEUNIT_NEXTCLUSTERBEGIN: MOVEUNIT_ACTION = MOVEUNIT_ACTION(3i32);
pub const MOVEUNIT_PREVCLUSTEREND: MOVEUNIT_ACTION = MOVEUNIT_ACTION(4i32);
pub const MOVEUNIT_NEXTCLUSTEREND: MOVEUNIT_ACTION = MOVEUNIT_ACTION(5i32);
pub const MOVEUNIT_PREVWORDBEGIN: MOVEUNIT_ACTION = MOVEUNIT_ACTION(6i32);
pub const MOVEUNIT_NEXTWORDBEGIN: MOVEUNIT_ACTION = MOVEUNIT_ACTION(7i32);
pub const MOVEUNIT_PREVWORDEND: MOVEUNIT_ACTION = MOVEUNIT_ACTION(8i32);
pub const MOVEUNIT_NEXTWORDEND: MOVEUNIT_ACTION = MOVEUNIT_ACTION(9i32);
pub const MOVEUNIT_PREVPROOFWORD: MOVEUNIT_ACTION = MOVEUNIT_ACTION(10i32);
pub const MOVEUNIT_NEXTPROOFWORD: MOVEUNIT_ACTION = MOVEUNIT_ACTION(11i32);
pub const MOVEUNIT_NEXTURLBEGIN: MOVEUNIT_ACTION = MOVEUNIT_ACTION(12i32);
pub const MOVEUNIT_PREVURLBEGIN: MOVEUNIT_ACTION = MOVEUNIT_ACTION(13i32);
pub const MOVEUNIT_NEXTURLEND: MOVEUNIT_ACTION = MOVEUNIT_ACTION(14i32);
pub const MOVEUNIT_PREVURLEND: MOVEUNIT_ACTION = MOVEUNIT_ACTION(15i32);
pub const MOVEUNIT_PREVSENTENCE: MOVEUNIT_ACTION = MOVEUNIT_ACTION(16i32);
pub const MOVEUNIT_NEXTSENTENCE: MOVEUNIT_ACTION = MOVEUNIT_ACTION(17i32);
pub const MOVEUNIT_PREVBLOCK: MOVEUNIT_ACTION = MOVEUNIT_ACTION(18i32);
pub const MOVEUNIT_NEXTBLOCK: MOVEUNIT_ACTION = MOVEUNIT_ACTION(19i32);
pub const MOVEUNIT_ACTION_Max: MOVEUNIT_ACTION = MOVEUNIT_ACTION(2147483647i32);
impl ::core::marker::Copy for MOVEUNIT_ACTION {}
impl ::core::clone::Clone for MOVEUNIT_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaActivityNotifyType(pub i32);
pub const MediaPlayback: MediaActivityNotifyType = MediaActivityNotifyType(0i32);
pub const MediaRecording: MediaActivityNotifyType = MediaActivityNotifyType(1i32);
pub const MediaCasting: MediaActivityNotifyType = MediaActivityNotifyType(2i32);
impl ::core::marker::Copy for MediaActivityNotifyType {}
impl ::core::clone::Clone for MediaActivityNotifyType {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NodeIterator: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616645, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const OldHTMLDocument: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3565842121, data2: 27210, data3: 4559, data4: [148, 167, 68, 69, 83, 84, 0, 0] };
pub const OldHTMLFormElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 218419845, data2: 27628, data3: 4559, data4: [139, 151, 0, 170, 0, 71, 109, 166] };
#[repr(transparent)]
pub struct OpenServiceActivityContentType(pub i32);
pub const ActivityContentNone: OpenServiceActivityContentType = OpenServiceActivityContentType(-1i32);
pub const ActivityContentDocument: OpenServiceActivityContentType = OpenServiceActivityContentType(0i32);
pub const ActivityContentSelection: OpenServiceActivityContentType = OpenServiceActivityContentType(1i32);
pub const ActivityContentLink: OpenServiceActivityContentType = OpenServiceActivityContentType(2i32);
pub const ActivityContentCount: OpenServiceActivityContentType = OpenServiceActivityContentType(3i32);
impl ::core::marker::Copy for OpenServiceActivityContentType {}
impl ::core::clone::Clone for OpenServiceActivityContentType {
    fn clone(&self) -> Self {
        *self
    }
}
pub const OpenServiceActivityManager: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3320829955,
    data2: 20728,
    data3: 17357,
    data4: [154, 184, 170, 252, 19, 148, 201, 224],
};
#[repr(transparent)]
pub struct OpenServiceErrors(pub i32);
pub const OS_E_NOTFOUND: OpenServiceErrors = OpenServiceErrors(-2147287038i32);
pub const OS_E_NOTSUPPORTED: OpenServiceErrors = OpenServiceErrors(-2147467231i32);
pub const OS_E_CANCELLED: OpenServiceErrors = OpenServiceErrors(-2147471631i32);
pub const OS_E_GPDISABLED: OpenServiceErrors = OpenServiceErrors(-1072886820i32);
impl ::core::marker::Copy for OpenServiceErrors {}
impl ::core::clone::Clone for OpenServiceErrors {
    fn clone(&self) -> Self {
        *self
    }
}
pub const OpenServiceManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 159936694, data2: 14826, data3: 18443, data4: [184, 181, 221, 1, 103, 196, 219, 89] };
#[repr(transparent)]
pub struct PARSE_FLAGS(pub i32);
pub const PARSE_ABSOLUTIFYIE40URLS: PARSE_FLAGS = PARSE_FLAGS(1i32);
pub const PARSE_DISABLEVML: PARSE_FLAGS = PARSE_FLAGS(2i32);
pub const PARSE_FLAGS_Max: PARSE_FLAGS = PARSE_FLAGS(2147483647i32);
impl ::core::marker::Copy for PARSE_FLAGS {}
impl ::core::clone::Clone for PARSE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct POINTER_GRAVITY(pub i32);
pub const POINTER_GRAVITY_Left: POINTER_GRAVITY = POINTER_GRAVITY(0i32);
pub const POINTER_GRAVITY_Right: POINTER_GRAVITY = POINTER_GRAVITY(1i32);
pub const POINTER_GRAVITY_Max: POINTER_GRAVITY = POINTER_GRAVITY(2147483647i32);
impl ::core::marker::Copy for POINTER_GRAVITY {}
impl ::core::clone::Clone for POINTER_GRAVITY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PRINT_DONTBOTHERUSER: u32 = 1u32;
pub const PRINT_WAITFORCOMPLETION: u32 = 2u32;
pub const PRIVACY_URLHASCOMPACTPOLICY: u32 = 131072u32;
pub const PRIVACY_URLHASP3PHEADER: u32 = 4194304u32;
pub const PRIVACY_URLHASPOLICYREFHEADER: u32 = 2097152u32;
pub const PRIVACY_URLHASPOLICYREFLINK: u32 = 1048576u32;
pub const PRIVACY_URLHASPOSTDATA: u32 = 524288u32;
pub const PRIVACY_URLISTOPLEVEL: u32 = 65536u32;
pub const PeerFactory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611919, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const REGSTR_VAL_FONT_SIZE_DEF: u32 = 2u32;
pub const REGSTR_VAL_JAVAJIT_DEF: u32 = 0u32;
pub const REGSTR_VAL_JAVALOGGING_DEF: u32 = 0u32;
pub const REGSTR_VAL_SCHANNELENABLEPROTOCOL_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYACTICEXSCRIPTS_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYACTIVEX_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYALLOWCOOKIES_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYDISABLECACHINGOFSSLPAGES_DEF: u32 = 0u32;
pub const REGSTR_VAL_SECURITYJAVA_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYWARNONBADCERTSENDING_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYWARNONBADCERTVIEWING_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYWARNONSENDALWAYS_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYWARNONSEND_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYWARNONVIEW_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYWARNONZONECROSSING_DEF: u32 = 1u32;
pub const REGSTR_VAL_SMOOTHSCROLL_DEF: u32 = 1u32;
pub const REGSTR_VAL_USEICM_DEF: u32 = 0u32;
pub const REGSTR_VAL_VISIBLEBANDS_DEF: u32 = 7u32;
pub const RangeException: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616622, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const RulesApplied: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2088778016,
    data2: 31315,
    data3: 19750,
    data4: [152, 172, 253, 210, 62, 107, 158, 1],
};
pub const RulesAppliedCollection: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1729701614,
    data2: 50127,
    data3: 16559,
    data4: [190, 143, 28, 186, 238, 100, 134, 232],
};
#[repr(transparent)]
pub struct SAVE_SEGMENTS_FLAGS(pub i32);
pub const SAVE_SEGMENTS_NoIE4SelectionCompat: SAVE_SEGMENTS_FLAGS = SAVE_SEGMENTS_FLAGS(1i32);
pub const SAVE_SEGMENTS_FLAGS_Max: SAVE_SEGMENTS_FLAGS = SAVE_SEGMENTS_FLAGS(2147483647i32);
impl ::core::marker::Copy for SAVE_SEGMENTS_FLAGS {}
impl ::core::clone::Clone for SAVE_SEGMENTS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SCRIPT_TIMER_TYPE(pub i32);
pub const STT_TIMEOUT: SCRIPT_TIMER_TYPE = SCRIPT_TIMER_TYPE(0i32);
pub const STT_INTERVAL: SCRIPT_TIMER_TYPE = SCRIPT_TIMER_TYPE(1i32);
pub const STT_IMMEDIATE: SCRIPT_TIMER_TYPE = SCRIPT_TIMER_TYPE(2i32);
pub const STT_ANIMATION_FRAME: SCRIPT_TIMER_TYPE = SCRIPT_TIMER_TYPE(3i32);
pub const SCRIPT_TIMER_TYPE_Max: SCRIPT_TIMER_TYPE = SCRIPT_TIMER_TYPE(2147483647i32);
impl ::core::marker::Copy for SCRIPT_TIMER_TYPE {}
impl ::core::clone::Clone for SCRIPT_TIMER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SCROLLABLECONTEXTMENU_PLACEMENT(pub i32);
pub const SCMP_TOP: SCROLLABLECONTEXTMENU_PLACEMENT = SCROLLABLECONTEXTMENU_PLACEMENT(0i32);
pub const SCMP_BOTTOM: SCROLLABLECONTEXTMENU_PLACEMENT = SCROLLABLECONTEXTMENU_PLACEMENT(1i32);
pub const SCMP_LEFT: SCROLLABLECONTEXTMENU_PLACEMENT = SCROLLABLECONTEXTMENU_PLACEMENT(2i32);
pub const SCMP_RIGHT: SCROLLABLECONTEXTMENU_PLACEMENT = SCROLLABLECONTEXTMENU_PLACEMENT(3i32);
pub const SCMP_FULL: SCROLLABLECONTEXTMENU_PLACEMENT = SCROLLABLECONTEXTMENU_PLACEMENT(4i32);
impl ::core::marker::Copy for SCROLLABLECONTEXTMENU_PLACEMENT {}
impl ::core::clone::Clone for SCROLLABLECONTEXTMENU_PLACEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SECUREURLHOSTVALIDATE_FLAGS(pub i32);
pub const SUHV_PROMPTBEFORENO: SECUREURLHOSTVALIDATE_FLAGS = SECUREURLHOSTVALIDATE_FLAGS(1i32);
pub const SUHV_SILENTYES: SECUREURLHOSTVALIDATE_FLAGS = SECUREURLHOSTVALIDATE_FLAGS(2i32);
pub const SUHV_UNSECURESOURCE: SECUREURLHOSTVALIDATE_FLAGS = SECUREURLHOSTVALIDATE_FLAGS(4i32);
pub const SECUREURLHOSTVALIDATE_FLAGS_Max: SECUREURLHOSTVALIDATE_FLAGS = SECUREURLHOSTVALIDATE_FLAGS(2147483647i32);
impl ::core::marker::Copy for SECUREURLHOSTVALIDATE_FLAGS {}
impl ::core::clone::Clone for SECUREURLHOSTVALIDATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SELECTION_TYPE(pub i32);
pub const SELECTION_TYPE_None: SELECTION_TYPE = SELECTION_TYPE(0i32);
pub const SELECTION_TYPE_Caret: SELECTION_TYPE = SELECTION_TYPE(1i32);
pub const SELECTION_TYPE_Text: SELECTION_TYPE = SELECTION_TYPE(2i32);
pub const SELECTION_TYPE_Control: SELECTION_TYPE = SELECTION_TYPE(3i32);
pub const SELECTION_TYPE_Max: SELECTION_TYPE = SELECTION_TYPE(2147483647i32);
impl ::core::marker::Copy for SELECTION_TYPE {}
impl ::core::clone::Clone for SELECTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type SHOWHTMLDIALOGEXFN = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, pmk: super::super::System::Com::IMoniker, dwdialogflags: u32, pvarargin: *mut super::super::System::Com::VARIANT, pchoptions: super::super::Foundation::PWSTR, pvargout: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type SHOWHTMLDIALOGFN = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, pmk: super::super::System::Com::IMoniker, pvarargin: *mut super::super::System::Com::VARIANT, pchoptions: super::super::Foundation::PWSTR, pvargout: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type SHOWMODELESSHTMLDIALOGFN = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, pmk: super::super::System::Com::IMoniker, pvarargin: *mut super::super::System::Com::VARIANT, pvaroptions: *mut super::super::System::Com::VARIANT, ppwindow: *mut IHTMLWindow2) -> ::windows_sys::core::HRESULT;
pub const SID_SEditCommandTarget: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611893, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SID_SHTMLEditHost: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612384, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SID_SHTMLEditServices: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612729, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STATURL {
    pub cbSize: u32,
    pub pwcsUrl: super::super::Foundation::PWSTR,
    pub pwcsTitle: super::super::Foundation::PWSTR,
    pub ftLastVisited: super::super::Foundation::FILETIME,
    pub ftLastUpdated: super::super::Foundation::FILETIME,
    pub ftExpires: super::super::Foundation::FILETIME,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STATURL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STATURL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const STATURLFLAG_ISCACHED: u32 = 1u32;
pub const STATURLFLAG_ISTOPLEVEL: u32 = 2u32;
pub const STATURL_QUERYFLAG_ISCACHED: u32 = 65536u32;
pub const STATURL_QUERYFLAG_NOTITLE: u32 = 262144u32;
pub const STATURL_QUERYFLAG_NOURL: u32 = 131072u32;
pub const STATURL_QUERYFLAG_TOPLEVEL: u32 = 524288u32;
pub const STDDISPID_XOBJ_AFTERUPDATE: u32 = 65541u32;
pub const STDDISPID_XOBJ_BEFOREUPDATE: u32 = 65540u32;
pub const STDDISPID_XOBJ_ERRORUPDATE: u32 = 65549u32;
pub const STDDISPID_XOBJ_ONBEFORECOPY: u32 = 65566u32;
pub const STDDISPID_XOBJ_ONBEFORECUT: u32 = 65565u32;
pub const STDDISPID_XOBJ_ONBEFOREPASTE: u32 = 65567u32;
pub const STDDISPID_XOBJ_ONCELLCHANGE: u32 = 65570u32;
pub const STDDISPID_XOBJ_ONCOPY: u32 = 65563u32;
pub const STDDISPID_XOBJ_ONCUT: u32 = 65562u32;
pub const STDDISPID_XOBJ_ONDATAAVAILABLE: u32 = 65551u32;
pub const STDDISPID_XOBJ_ONDATASETCHANGED: u32 = 65550u32;
pub const STDDISPID_XOBJ_ONDATASETCOMPLETE: u32 = 65552u32;
pub const STDDISPID_XOBJ_ONDRAG: u32 = 65556u32;
pub const STDDISPID_XOBJ_ONDRAGEND: u32 = 65557u32;
pub const STDDISPID_XOBJ_ONDRAGENTER: u32 = 65558u32;
pub const STDDISPID_XOBJ_ONDRAGLEAVE: u32 = 65560u32;
pub const STDDISPID_XOBJ_ONDRAGOVER: u32 = 65559u32;
pub const STDDISPID_XOBJ_ONDRAGSTART: u32 = 65547u32;
pub const STDDISPID_XOBJ_ONDROP: u32 = 65561u32;
pub const STDDISPID_XOBJ_ONFILTER: u32 = 65553u32;
pub const STDDISPID_XOBJ_ONFOCUS: u32 = 65537u32;
pub const STDDISPID_XOBJ_ONHELP: u32 = 65546u32;
pub const STDDISPID_XOBJ_ONLOSECAPTURE: u32 = 65554u32;
pub const STDDISPID_XOBJ_ONMOUSEOUT: u32 = 65545u32;
pub const STDDISPID_XOBJ_ONMOUSEOVER: u32 = 65544u32;
pub const STDDISPID_XOBJ_ONPASTE: u32 = 65564u32;
pub const STDDISPID_XOBJ_ONPROPERTYCHANGE: u32 = 65555u32;
pub const STDDISPID_XOBJ_ONROWENTER: u32 = 65543u32;
pub const STDDISPID_XOBJ_ONROWEXIT: u32 = 65542u32;
pub const STDDISPID_XOBJ_ONROWSDELETE: u32 = 65568u32;
pub const STDDISPID_XOBJ_ONROWSINSERTED: u32 = 65569u32;
pub const STDDISPID_XOBJ_ONSELECTSTART: u32 = 65548u32;
pub const STDPROPID_IE3XOBJ_OBJECTALIGN: u32 = 65537u32;
pub const STDPROPID_XOBJ_ALIGNPERSIST: u32 = 65596u32;
pub const STDPROPID_XOBJ_APPLICATION: u32 = 65607u32;
pub const STDPROPID_XOBJ_BASEHREF: u32 = 65538u32;
pub const STDPROPID_XOBJ_BLOCKALIGN: u32 = 65608u32;
pub const STDPROPID_XOBJ_BOTTOM: u32 = 65614u32;
pub const STDPROPID_XOBJ_CANCEL: u32 = 65592u32;
pub const STDPROPID_XOBJ_CONTROLALIGN: u32 = 65609u32;
pub const STDPROPID_XOBJ_CONTROLTIPTEXT: u32 = 65605u32;
pub const STDPROPID_XOBJ_COUNT: u32 = 65611u32;
pub const STDPROPID_XOBJ_DATACHANGED: u32 = 65601u32;
pub const STDPROPID_XOBJ_DATAFIELD: u32 = 65602u32;
pub const STDPROPID_XOBJ_DATASOURCE: u32 = 65603u32;
pub const STDPROPID_XOBJ_DEFAULT: u32 = 65591u32;
pub const STDPROPID_XOBJ_DISABLED: u32 = 65612u32;
pub const STDPROPID_XOBJ_DRAGICON: u32 = 65546u32;
pub const STDPROPID_XOBJ_DRAGMODE: u32 = 65545u32;
pub const STDPROPID_XOBJ_GETSVGDOCUMENT: u32 = 65615u32;
pub const STDPROPID_XOBJ_HEIGHT: u32 = 65542u32;
pub const STDPROPID_XOBJ_HELPCONTEXTID: u32 = 65586u32;
pub const STDPROPID_XOBJ_INDEX: u32 = 65537u32;
pub const STDPROPID_XOBJ_LEFT: u32 = 65539u32;
pub const STDPROPID_XOBJ_LEFTNORUN: u32 = 65593u32;
pub const STDPROPID_XOBJ_LINKITEM: u32 = 65599u32;
pub const STDPROPID_XOBJ_LINKMODE: u32 = 65600u32;
pub const STDPROPID_XOBJ_LINKTIMEOUT: u32 = 65597u32;
pub const STDPROPID_XOBJ_LINKTOPIC: u32 = 65598u32;
pub const STDPROPID_XOBJ_NAME: u32 = 65536u32;
pub const STDPROPID_XOBJ_PARENT: u32 = 65544u32;
pub const STDPROPID_XOBJ_RIGHT: u32 = 65613u32;
pub const STDPROPID_XOBJ_STATUSBARTEXT: u32 = 65606u32;
pub const STDPROPID_XOBJ_STYLE: u32 = 65610u32;
pub const STDPROPID_XOBJ_TABINDEX: u32 = 65551u32;
pub const STDPROPID_XOBJ_TABSTOP: u32 = 65550u32;
pub const STDPROPID_XOBJ_TAG: u32 = 65547u32;
pub const STDPROPID_XOBJ_TOP: u32 = 65540u32;
pub const STDPROPID_XOBJ_TOPNORUN: u32 = 65594u32;
pub const STDPROPID_XOBJ_VISIBLE: u32 = 65543u32;
pub const STDPROPID_XOBJ_WHATSTHISHELPID: u32 = 65604u32;
pub const STDPROPID_XOBJ_WIDTH: u32 = 65541u32;
pub const SURFACE_LOCK_ALLOW_DISCARD: u32 = 2u32;
pub const SURFACE_LOCK_EXCLUSIVE: u32 = 1u32;
pub const SURFACE_LOCK_WAIT: u32 = 4u32;
pub const SVGAElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616283, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGAngle: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616196, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGAnimatedAngle: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616292, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGAnimatedBoolean: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616203, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGAnimatedEnumeration: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616206, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGAnimatedInteger: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616207, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGAnimatedLength: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616193, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGAnimatedLengthList: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616194, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGAnimatedNumber: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616200, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGAnimatedNumberList: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616202, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGAnimatedPreserveAspectRatio: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616270, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGAnimatedRect: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616198, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGAnimatedString: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616204, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGAnimatedTransformList: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616241, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGCircleElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616184, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGClipPathElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616294, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGDefsElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616176, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGDescElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616178, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616164, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGElementInstance: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616181, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGElementInstanceList: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616182, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGEllipseElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616185, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGException: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616624, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGGElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616175, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGGradientElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616278, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGImageElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616271, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGLength: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616190, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGLengthList: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616192, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGLineElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616186, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGLinearGradientElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616274, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGMarkerElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616286, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGMaskElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616295, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGMatrix: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616238, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGMetadataElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616279, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGNumber: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616199, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGNumberList: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616201, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616242, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSeg: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616243, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegArcAbs: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616251, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegArcRel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616252, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegClosePath: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616253, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegCurvetoCubicAbs: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616254, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegCurvetoCubicRel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616255, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegCurvetoCubicSmoothAbs: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616256, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegCurvetoCubicSmoothRel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616257, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegCurvetoQuadraticAbs: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616258, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegCurvetoQuadraticRel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616259, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegCurvetoQuadraticSmoothAbs: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616260, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegCurvetoQuadraticSmoothRel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616261, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegLinetoAbs: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616262, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegLinetoHorizontalAbs: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616263, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegLinetoHorizontalRel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616264, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegLinetoRel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616265, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegLinetoVerticalAbs: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616266, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegLinetoVerticalRel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616267, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegList: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616244, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegMovetoAbs: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616268, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPathSegMovetoRel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616269, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPatternElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616276, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPoint: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616250, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPointList: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616249, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPolygonElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616187, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPolylineElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616188, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGPreserveAspectRatio: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616272, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGRadialGradientElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616275, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGRect: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616195, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGRectElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616183, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGSVGElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616180, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGScriptElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616289, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGStopElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616277, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGStringList: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616205, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGStyleElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616273, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGSwitchElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616280, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGSymbolElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616177, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGTSpanElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616290, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGTextContentElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616285, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGTextElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616287, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGTextPathElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616299, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGTextPositioningElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616288, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGTitleElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616179, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGTransform: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616239, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGTransformList: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616240, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGUseElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616208, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGViewElement: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616284, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SVGZoomEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616281, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const S_SURFACE_DISCARDED: i32 = 49155i32;
pub const Scriptlet: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2921659822, data2: 966, data3: 4561, data4: [139, 118, 0, 128, 199, 68, 243, 137] };
pub const StaticNodeList: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810615911, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const TF_NAVIGATE: u32 = 2142153644u32;
pub const TIMERMODE_NORMAL: u32 = 0u32;
pub const TIMERMODE_VISIBILITYAWARE: u32 = 1u32;
pub const TOOLSBAND: u32 = 1u32;
pub const ThreadDialogProcParam: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612203, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const TreeWalker: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616647, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct VIEW_OBJECT_ALPHA_MODE(pub i32);
pub const VIEW_OBJECT_ALPHA_MODE_IGNORE: VIEW_OBJECT_ALPHA_MODE = VIEW_OBJECT_ALPHA_MODE(0i32);
pub const VIEW_OBJECT_ALPHA_MODE_PREMULTIPLIED: VIEW_OBJECT_ALPHA_MODE = VIEW_OBJECT_ALPHA_MODE(1i32);
pub const VIEW_OBJECT_ALPHA_MODE_Max: VIEW_OBJECT_ALPHA_MODE = VIEW_OBJECT_ALPHA_MODE(2147483647i32);
impl ::core::marker::Copy for VIEW_OBJECT_ALPHA_MODE {}
impl ::core::clone::Clone for VIEW_OBJECT_ALPHA_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VIEW_OBJECT_COMPOSITION_MODE(pub i32);
pub const VIEW_OBJECT_COMPOSITION_MODE_LEGACY: VIEW_OBJECT_COMPOSITION_MODE = VIEW_OBJECT_COMPOSITION_MODE(0i32);
pub const VIEW_OBJECT_COMPOSITION_MODE_SURFACEPRESENTER: VIEW_OBJECT_COMPOSITION_MODE = VIEW_OBJECT_COMPOSITION_MODE(1i32);
pub const VIEW_OBJECT_COMPOSITION_MODE_Max: VIEW_OBJECT_COMPOSITION_MODE = VIEW_OBJECT_COMPOSITION_MODE(2147483647i32);
impl ::core::marker::Copy for VIEW_OBJECT_COMPOSITION_MODE {}
impl ::core::clone::Clone for VIEW_OBJECT_COMPOSITION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WEBOC_DISPIDBASE: u32 = 70536u32;
pub const WEBOC_DISPIDMAX: u32 = 70636u32;
pub const WebGeocoordinates: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616776, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const WebGeolocation: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616774, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const WebGeoposition: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616782, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const WebGeopositionError: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616778, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const XDomainRequest: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810615893, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const XDomainRequestFactory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810615895, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const XMLHttpRequestEventTarget: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616881, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const XMLSerializer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810616702, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[repr(transparent)]
pub struct __MIDL_ITargetFrame2_0001(pub i32);
pub const FINDFRAME_NONE: __MIDL_ITargetFrame2_0001 = __MIDL_ITargetFrame2_0001(0i32);
pub const FINDFRAME_JUSTTESTEXISTENCE: __MIDL_ITargetFrame2_0001 = __MIDL_ITargetFrame2_0001(1i32);
pub const FINDFRAME_INTERNAL: __MIDL_ITargetFrame2_0001 = __MIDL_ITargetFrame2_0001(-2147483648i32);
impl ::core::marker::Copy for __MIDL_ITargetFrame2_0001 {}
impl ::core::clone::Clone for __MIDL_ITargetFrame2_0001 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct __MIDL_ITargetFrame2_0002(pub i32);
pub const FRAMEOPTIONS_SCROLL_YES: __MIDL_ITargetFrame2_0002 = __MIDL_ITargetFrame2_0002(1i32);
pub const FRAMEOPTIONS_SCROLL_NO: __MIDL_ITargetFrame2_0002 = __MIDL_ITargetFrame2_0002(2i32);
pub const FRAMEOPTIONS_SCROLL_AUTO: __MIDL_ITargetFrame2_0002 = __MIDL_ITargetFrame2_0002(4i32);
pub const FRAMEOPTIONS_NORESIZE: __MIDL_ITargetFrame2_0002 = __MIDL_ITargetFrame2_0002(8i32);
pub const FRAMEOPTIONS_NO3DBORDER: __MIDL_ITargetFrame2_0002 = __MIDL_ITargetFrame2_0002(16i32);
pub const FRAMEOPTIONS_DESKTOP: __MIDL_ITargetFrame2_0002 = __MIDL_ITargetFrame2_0002(32i32);
pub const FRAMEOPTIONS_BROWSERBAND: __MIDL_ITargetFrame2_0002 = __MIDL_ITargetFrame2_0002(64i32);
impl ::core::marker::Copy for __MIDL_ITargetFrame2_0002 {}
impl ::core::clone::Clone for __MIDL_ITargetFrame2_0002 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct __MIDL_ITargetFrame_0001(pub i32);
pub const NAVIGATEFRAME_FL_RECORD: __MIDL_ITargetFrame_0001 = __MIDL_ITargetFrame_0001(1i32);
pub const NAVIGATEFRAME_FL_POST: __MIDL_ITargetFrame_0001 = __MIDL_ITargetFrame_0001(2i32);
pub const NAVIGATEFRAME_FL_NO_DOC_CACHE: __MIDL_ITargetFrame_0001 = __MIDL_ITargetFrame_0001(4i32);
pub const NAVIGATEFRAME_FL_NO_IMAGE_CACHE: __MIDL_ITargetFrame_0001 = __MIDL_ITargetFrame_0001(8i32);
pub const NAVIGATEFRAME_FL_AUTH_FAIL_CACHE_OK: __MIDL_ITargetFrame_0001 = __MIDL_ITargetFrame_0001(16i32);
pub const NAVIGATEFRAME_FL_SENDING_FROM_FORM: __MIDL_ITargetFrame_0001 = __MIDL_ITargetFrame_0001(32i32);
pub const NAVIGATEFRAME_FL_REALLY_SENDING_FROM_FORM: __MIDL_ITargetFrame_0001 = __MIDL_ITargetFrame_0001(64i32);
impl ::core::marker::Copy for __MIDL_ITargetFrame_0001 {}
impl ::core::clone::Clone for __MIDL_ITargetFrame_0001 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct bodyScroll(pub i32);
pub const bodyScrollyes: bodyScroll = bodyScroll(1i32);
pub const bodyScrollno: bodyScroll = bodyScroll(2i32);
pub const bodyScrollauto: bodyScroll = bodyScroll(4i32);
pub const bodyScrolldefault: bodyScroll = bodyScroll(3i32);
pub const bodyScroll_Max: bodyScroll = bodyScroll(2147483647i32);
impl ::core::marker::Copy for bodyScroll {}
impl ::core::clone::Clone for bodyScroll {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct frameScrolling(pub i32);
pub const frameScrollingyes: frameScrolling = frameScrolling(1i32);
pub const frameScrollingno: frameScrolling = frameScrolling(2i32);
pub const frameScrollingauto: frameScrolling = frameScrolling(4i32);
pub const frameScrolling_Max: frameScrolling = frameScrolling(2147483647i32);
impl ::core::marker::Copy for frameScrolling {}
impl ::core::clone::Clone for frameScrolling {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlAdjacency(pub i32);
pub const htmlAdjacencyBeforeBegin: htmlAdjacency = htmlAdjacency(1i32);
pub const htmlAdjacencyAfterBegin: htmlAdjacency = htmlAdjacency(2i32);
pub const htmlAdjacencyBeforeEnd: htmlAdjacency = htmlAdjacency(3i32);
pub const htmlAdjacencyAfterEnd: htmlAdjacency = htmlAdjacency(4i32);
pub const htmlAdjacency_Max: htmlAdjacency = htmlAdjacency(2147483647i32);
impl ::core::marker::Copy for htmlAdjacency {}
impl ::core::clone::Clone for htmlAdjacency {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlApplyLocation(pub i32);
pub const htmlApplyLocationInside: htmlApplyLocation = htmlApplyLocation(0i32);
pub const htmlApplyLocationOutside: htmlApplyLocation = htmlApplyLocation(1i32);
pub const htmlApplyLocation_Max: htmlApplyLocation = htmlApplyLocation(2147483647i32);
impl ::core::marker::Copy for htmlApplyLocation {}
impl ::core::clone::Clone for htmlApplyLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlBlockAlign(pub i32);
pub const htmlBlockAlignNotSet: htmlBlockAlign = htmlBlockAlign(0i32);
pub const htmlBlockAlignLeft: htmlBlockAlign = htmlBlockAlign(1i32);
pub const htmlBlockAlignCenter: htmlBlockAlign = htmlBlockAlign(2i32);
pub const htmlBlockAlignRight: htmlBlockAlign = htmlBlockAlign(3i32);
pub const htmlBlockAlignJustify: htmlBlockAlign = htmlBlockAlign(4i32);
pub const htmlBlockAlign_Max: htmlBlockAlign = htmlBlockAlign(2147483647i32);
impl ::core::marker::Copy for htmlBlockAlign {}
impl ::core::clone::Clone for htmlBlockAlign {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlCaptionAlign(pub i32);
pub const htmlCaptionAlignNotSet: htmlCaptionAlign = htmlCaptionAlign(0i32);
pub const htmlCaptionAlignLeft: htmlCaptionAlign = htmlCaptionAlign(1i32);
pub const htmlCaptionAlignCenter: htmlCaptionAlign = htmlCaptionAlign(2i32);
pub const htmlCaptionAlignRight: htmlCaptionAlign = htmlCaptionAlign(3i32);
pub const htmlCaptionAlignJustify: htmlCaptionAlign = htmlCaptionAlign(4i32);
pub const htmlCaptionAlignTop: htmlCaptionAlign = htmlCaptionAlign(5i32);
pub const htmlCaptionAlignBottom: htmlCaptionAlign = htmlCaptionAlign(6i32);
pub const htmlCaptionAlign_Max: htmlCaptionAlign = htmlCaptionAlign(2147483647i32);
impl ::core::marker::Copy for htmlCaptionAlign {}
impl ::core::clone::Clone for htmlCaptionAlign {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlCaptionVAlign(pub i32);
pub const htmlCaptionVAlignNotSet: htmlCaptionVAlign = htmlCaptionVAlign(0i32);
pub const htmlCaptionVAlignTop: htmlCaptionVAlign = htmlCaptionVAlign(1i32);
pub const htmlCaptionVAlignBottom: htmlCaptionVAlign = htmlCaptionVAlign(2i32);
pub const htmlCaptionVAlign_Max: htmlCaptionVAlign = htmlCaptionVAlign(2147483647i32);
impl ::core::marker::Copy for htmlCaptionVAlign {}
impl ::core::clone::Clone for htmlCaptionVAlign {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlCellAlign(pub i32);
pub const htmlCellAlignNotSet: htmlCellAlign = htmlCellAlign(0i32);
pub const htmlCellAlignLeft: htmlCellAlign = htmlCellAlign(1i32);
pub const htmlCellAlignCenter: htmlCellAlign = htmlCellAlign(2i32);
pub const htmlCellAlignRight: htmlCellAlign = htmlCellAlign(3i32);
pub const htmlCellAlignMiddle: htmlCellAlign = htmlCellAlign(2i32);
pub const htmlCellAlign_Max: htmlCellAlign = htmlCellAlign(2147483647i32);
impl ::core::marker::Copy for htmlCellAlign {}
impl ::core::clone::Clone for htmlCellAlign {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlCellVAlign(pub i32);
pub const htmlCellVAlignNotSet: htmlCellVAlign = htmlCellVAlign(0i32);
pub const htmlCellVAlignTop: htmlCellVAlign = htmlCellVAlign(1i32);
pub const htmlCellVAlignMiddle: htmlCellVAlign = htmlCellVAlign(2i32);
pub const htmlCellVAlignBottom: htmlCellVAlign = htmlCellVAlign(3i32);
pub const htmlCellVAlignBaseline: htmlCellVAlign = htmlCellVAlign(4i32);
pub const htmlCellVAlignCenter: htmlCellVAlign = htmlCellVAlign(2i32);
pub const htmlCellVAlign_Max: htmlCellVAlign = htmlCellVAlign(2147483647i32);
impl ::core::marker::Copy for htmlCellVAlign {}
impl ::core::clone::Clone for htmlCellVAlign {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlClear(pub i32);
pub const htmlClearNotSet: htmlClear = htmlClear(0i32);
pub const htmlClearAll: htmlClear = htmlClear(1i32);
pub const htmlClearLeft: htmlClear = htmlClear(2i32);
pub const htmlClearRight: htmlClear = htmlClear(3i32);
pub const htmlClearBoth: htmlClear = htmlClear(4i32);
pub const htmlClearNone: htmlClear = htmlClear(5i32);
pub const htmlClear_Max: htmlClear = htmlClear(2147483647i32);
impl ::core::marker::Copy for htmlClear {}
impl ::core::clone::Clone for htmlClear {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlCompatMode(pub i32);
pub const htmlCompatModeBackCompat: htmlCompatMode = htmlCompatMode(0i32);
pub const htmlCompatModeCSS1Compat: htmlCompatMode = htmlCompatMode(1i32);
pub const htmlCompatMode_Max: htmlCompatMode = htmlCompatMode(2147483647i32);
impl ::core::marker::Copy for htmlCompatMode {}
impl ::core::clone::Clone for htmlCompatMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlComponent(pub i32);
pub const htmlComponentClient: htmlComponent = htmlComponent(0i32);
pub const htmlComponentSbLeft: htmlComponent = htmlComponent(1i32);
pub const htmlComponentSbPageLeft: htmlComponent = htmlComponent(2i32);
pub const htmlComponentSbHThumb: htmlComponent = htmlComponent(3i32);
pub const htmlComponentSbPageRight: htmlComponent = htmlComponent(4i32);
pub const htmlComponentSbRight: htmlComponent = htmlComponent(5i32);
pub const htmlComponentSbUp: htmlComponent = htmlComponent(6i32);
pub const htmlComponentSbPageUp: htmlComponent = htmlComponent(7i32);
pub const htmlComponentSbVThumb: htmlComponent = htmlComponent(8i32);
pub const htmlComponentSbPageDown: htmlComponent = htmlComponent(9i32);
pub const htmlComponentSbDown: htmlComponent = htmlComponent(10i32);
pub const htmlComponentSbLeft2: htmlComponent = htmlComponent(11i32);
pub const htmlComponentSbPageLeft2: htmlComponent = htmlComponent(12i32);
pub const htmlComponentSbRight2: htmlComponent = htmlComponent(13i32);
pub const htmlComponentSbPageRight2: htmlComponent = htmlComponent(14i32);
pub const htmlComponentSbUp2: htmlComponent = htmlComponent(15i32);
pub const htmlComponentSbPageUp2: htmlComponent = htmlComponent(16i32);
pub const htmlComponentSbDown2: htmlComponent = htmlComponent(17i32);
pub const htmlComponentSbPageDown2: htmlComponent = htmlComponent(18i32);
pub const htmlComponentSbTop: htmlComponent = htmlComponent(19i32);
pub const htmlComponentSbBottom: htmlComponent = htmlComponent(20i32);
pub const htmlComponentOutside: htmlComponent = htmlComponent(21i32);
pub const htmlComponentGHTopLeft: htmlComponent = htmlComponent(22i32);
pub const htmlComponentGHLeft: htmlComponent = htmlComponent(23i32);
pub const htmlComponentGHTop: htmlComponent = htmlComponent(24i32);
pub const htmlComponentGHBottomLeft: htmlComponent = htmlComponent(25i32);
pub const htmlComponentGHTopRight: htmlComponent = htmlComponent(26i32);
pub const htmlComponentGHBottom: htmlComponent = htmlComponent(27i32);
pub const htmlComponentGHRight: htmlComponent = htmlComponent(28i32);
pub const htmlComponentGHBottomRight: htmlComponent = htmlComponent(29i32);
pub const htmlComponent_Max: htmlComponent = htmlComponent(2147483647i32);
impl ::core::marker::Copy for htmlComponent {}
impl ::core::clone::Clone for htmlComponent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlControlAlign(pub i32);
pub const htmlControlAlignNotSet: htmlControlAlign = htmlControlAlign(0i32);
pub const htmlControlAlignLeft: htmlControlAlign = htmlControlAlign(1i32);
pub const htmlControlAlignCenter: htmlControlAlign = htmlControlAlign(2i32);
pub const htmlControlAlignRight: htmlControlAlign = htmlControlAlign(3i32);
pub const htmlControlAlignTextTop: htmlControlAlign = htmlControlAlign(4i32);
pub const htmlControlAlignAbsMiddle: htmlControlAlign = htmlControlAlign(5i32);
pub const htmlControlAlignBaseline: htmlControlAlign = htmlControlAlign(6i32);
pub const htmlControlAlignAbsBottom: htmlControlAlign = htmlControlAlign(7i32);
pub const htmlControlAlignBottom: htmlControlAlign = htmlControlAlign(8i32);
pub const htmlControlAlignMiddle: htmlControlAlign = htmlControlAlign(9i32);
pub const htmlControlAlignTop: htmlControlAlign = htmlControlAlign(10i32);
pub const htmlControlAlign_Max: htmlControlAlign = htmlControlAlign(2147483647i32);
impl ::core::marker::Copy for htmlControlAlign {}
impl ::core::clone::Clone for htmlControlAlign {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlDesignMode(pub i32);
pub const htmlDesignModeInherit: htmlDesignMode = htmlDesignMode(-2i32);
pub const htmlDesignModeOn: htmlDesignMode = htmlDesignMode(-1i32);
pub const htmlDesignModeOff: htmlDesignMode = htmlDesignMode(0i32);
pub const htmlDesignMode_Max: htmlDesignMode = htmlDesignMode(2147483647i32);
impl ::core::marker::Copy for htmlDesignMode {}
impl ::core::clone::Clone for htmlDesignMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlDir(pub i32);
pub const htmlDirNotSet: htmlDir = htmlDir(0i32);
pub const htmlDirLeftToRight: htmlDir = htmlDir(1i32);
pub const htmlDirRightToLeft: htmlDir = htmlDir(2i32);
pub const htmlDir_Max: htmlDir = htmlDir(2147483647i32);
impl ::core::marker::Copy for htmlDir {}
impl ::core::clone::Clone for htmlDir {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlDirection(pub i32);
pub const htmlDirectionForward: htmlDirection = htmlDirection(99999i32);
pub const htmlDirectionBackward: htmlDirection = htmlDirection(-99999i32);
pub const htmlDirection_Max: htmlDirection = htmlDirection(2147483647i32);
impl ::core::marker::Copy for htmlDirection {}
impl ::core::clone::Clone for htmlDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlDraggable(pub i32);
pub const htmlDraggableAuto: htmlDraggable = htmlDraggable(0i32);
pub const htmlDraggableTrue: htmlDraggable = htmlDraggable(1i32);
pub const htmlDraggableFalse: htmlDraggable = htmlDraggable(2i32);
pub const htmlDraggable_Max: htmlDraggable = htmlDraggable(2147483647i32);
impl ::core::marker::Copy for htmlDraggable {}
impl ::core::clone::Clone for htmlDraggable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlDropEffect(pub i32);
pub const htmlDropEffectCopy: htmlDropEffect = htmlDropEffect(0i32);
pub const htmlDropEffectLink: htmlDropEffect = htmlDropEffect(1i32);
pub const htmlDropEffectMove: htmlDropEffect = htmlDropEffect(2i32);
pub const htmlDropEffectNone: htmlDropEffect = htmlDropEffect(3i32);
pub const htmlDropEffect_Max: htmlDropEffect = htmlDropEffect(2147483647i32);
impl ::core::marker::Copy for htmlDropEffect {}
impl ::core::clone::Clone for htmlDropEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlEditable(pub i32);
pub const htmlEditableInherit: htmlEditable = htmlEditable(0i32);
pub const htmlEditableTrue: htmlEditable = htmlEditable(1i32);
pub const htmlEditableFalse: htmlEditable = htmlEditable(2i32);
pub const htmlEditable_Max: htmlEditable = htmlEditable(2147483647i32);
impl ::core::marker::Copy for htmlEditable {}
impl ::core::clone::Clone for htmlEditable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlEffectAllowed(pub i32);
pub const htmlEffectAllowedCopy: htmlEffectAllowed = htmlEffectAllowed(0i32);
pub const htmlEffectAllowedLink: htmlEffectAllowed = htmlEffectAllowed(1i32);
pub const htmlEffectAllowedMove: htmlEffectAllowed = htmlEffectAllowed(2i32);
pub const htmlEffectAllowedCopyLink: htmlEffectAllowed = htmlEffectAllowed(3i32);
pub const htmlEffectAllowedCopyMove: htmlEffectAllowed = htmlEffectAllowed(4i32);
pub const htmlEffectAllowedLinkMove: htmlEffectAllowed = htmlEffectAllowed(5i32);
pub const htmlEffectAllowedAll: htmlEffectAllowed = htmlEffectAllowed(6i32);
pub const htmlEffectAllowedNone: htmlEffectAllowed = htmlEffectAllowed(7i32);
pub const htmlEffectAllowedUninitialized: htmlEffectAllowed = htmlEffectAllowed(8i32);
pub const htmlEffectAllowed_Max: htmlEffectAllowed = htmlEffectAllowed(2147483647i32);
impl ::core::marker::Copy for htmlEffectAllowed {}
impl ::core::clone::Clone for htmlEffectAllowed {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlEncoding(pub i32);
pub const htmlEncodingURL: htmlEncoding = htmlEncoding(0i32);
pub const htmlEncodingMultipart: htmlEncoding = htmlEncoding(1i32);
pub const htmlEncodingText: htmlEncoding = htmlEncoding(2i32);
pub const htmlEncoding_Max: htmlEncoding = htmlEncoding(2147483647i32);
impl ::core::marker::Copy for htmlEncoding {}
impl ::core::clone::Clone for htmlEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlEndPoints(pub i32);
pub const htmlEndPointsStartToStart: htmlEndPoints = htmlEndPoints(1i32);
pub const htmlEndPointsStartToEnd: htmlEndPoints = htmlEndPoints(2i32);
pub const htmlEndPointsEndToStart: htmlEndPoints = htmlEndPoints(3i32);
pub const htmlEndPointsEndToEnd: htmlEndPoints = htmlEndPoints(4i32);
pub const htmlEndPoints_Max: htmlEndPoints = htmlEndPoints(2147483647i32);
impl ::core::marker::Copy for htmlEndPoints {}
impl ::core::clone::Clone for htmlEndPoints {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlFrame(pub i32);
pub const htmlFrameNotSet: htmlFrame = htmlFrame(0i32);
pub const htmlFramevoid: htmlFrame = htmlFrame(1i32);
pub const htmlFrameabove: htmlFrame = htmlFrame(2i32);
pub const htmlFramebelow: htmlFrame = htmlFrame(3i32);
pub const htmlFramehsides: htmlFrame = htmlFrame(4i32);
pub const htmlFramelhs: htmlFrame = htmlFrame(5i32);
pub const htmlFramerhs: htmlFrame = htmlFrame(6i32);
pub const htmlFramevsides: htmlFrame = htmlFrame(7i32);
pub const htmlFramebox: htmlFrame = htmlFrame(8i32);
pub const htmlFrameborder: htmlFrame = htmlFrame(9i32);
pub const htmlFrame_Max: htmlFrame = htmlFrame(2147483647i32);
impl ::core::marker::Copy for htmlFrame {}
impl ::core::clone::Clone for htmlFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlGlyphMode(pub i32);
pub const htmlGlyphModeNone: htmlGlyphMode = htmlGlyphMode(0i32);
pub const htmlGlyphModeBegin: htmlGlyphMode = htmlGlyphMode(1i32);
pub const htmlGlyphModeEnd: htmlGlyphMode = htmlGlyphMode(2i32);
pub const htmlGlyphModeBoth: htmlGlyphMode = htmlGlyphMode(3i32);
pub const htmlGlyphMode_Max: htmlGlyphMode = htmlGlyphMode(2147483647i32);
impl ::core::marker::Copy for htmlGlyphMode {}
impl ::core::clone::Clone for htmlGlyphMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlInput(pub i32);
pub const htmlInputNotSet: htmlInput = htmlInput(0i32);
pub const htmlInputButton: htmlInput = htmlInput(1i32);
pub const htmlInputCheckbox: htmlInput = htmlInput(2i32);
pub const htmlInputFile: htmlInput = htmlInput(3i32);
pub const htmlInputHidden: htmlInput = htmlInput(4i32);
pub const htmlInputImage: htmlInput = htmlInput(5i32);
pub const htmlInputPassword: htmlInput = htmlInput(6i32);
pub const htmlInputRadio: htmlInput = htmlInput(7i32);
pub const htmlInputReset: htmlInput = htmlInput(8i32);
pub const htmlInputSelectOne: htmlInput = htmlInput(9i32);
pub const htmlInputSelectMultiple: htmlInput = htmlInput(10i32);
pub const htmlInputSubmit: htmlInput = htmlInput(11i32);
pub const htmlInputText: htmlInput = htmlInput(12i32);
pub const htmlInputTextarea: htmlInput = htmlInput(13i32);
pub const htmlInputRichtext: htmlInput = htmlInput(14i32);
pub const htmlInputRange: htmlInput = htmlInput(15i32);
pub const htmlInputUrl: htmlInput = htmlInput(16i32);
pub const htmlInputEmail: htmlInput = htmlInput(17i32);
pub const htmlInputNumber: htmlInput = htmlInput(18i32);
pub const htmlInputTel: htmlInput = htmlInput(19i32);
pub const htmlInputSearch: htmlInput = htmlInput(20i32);
pub const htmlInput_Max: htmlInput = htmlInput(2147483647i32);
impl ::core::marker::Copy for htmlInput {}
impl ::core::clone::Clone for htmlInput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlListType(pub i32);
pub const htmlListTypeNotSet: htmlListType = htmlListType(0i32);
pub const htmlListTypeLargeAlpha: htmlListType = htmlListType(1i32);
pub const htmlListTypeSmallAlpha: htmlListType = htmlListType(2i32);
pub const htmlListTypeLargeRoman: htmlListType = htmlListType(3i32);
pub const htmlListTypeSmallRoman: htmlListType = htmlListType(4i32);
pub const htmlListTypeNumbers: htmlListType = htmlListType(5i32);
pub const htmlListTypeDisc: htmlListType = htmlListType(6i32);
pub const htmlListTypeCircle: htmlListType = htmlListType(7i32);
pub const htmlListTypeSquare: htmlListType = htmlListType(8i32);
pub const htmlListType_Max: htmlListType = htmlListType(2147483647i32);
impl ::core::marker::Copy for htmlListType {}
impl ::core::clone::Clone for htmlListType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlLoop(pub i32);
pub const htmlLoopLoopInfinite: htmlLoop = htmlLoop(-1i32);
pub const htmlLoop_Max: htmlLoop = htmlLoop(2147483647i32);
impl ::core::marker::Copy for htmlLoop {}
impl ::core::clone::Clone for htmlLoop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlMarqueeBehavior(pub i32);
pub const htmlMarqueeBehaviorscroll: htmlMarqueeBehavior = htmlMarqueeBehavior(1i32);
pub const htmlMarqueeBehaviorslide: htmlMarqueeBehavior = htmlMarqueeBehavior(2i32);
pub const htmlMarqueeBehavioralternate: htmlMarqueeBehavior = htmlMarqueeBehavior(3i32);
pub const htmlMarqueeBehavior_Max: htmlMarqueeBehavior = htmlMarqueeBehavior(2147483647i32);
impl ::core::marker::Copy for htmlMarqueeBehavior {}
impl ::core::clone::Clone for htmlMarqueeBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlMarqueeDirection(pub i32);
pub const htmlMarqueeDirectionleft: htmlMarqueeDirection = htmlMarqueeDirection(1i32);
pub const htmlMarqueeDirectionright: htmlMarqueeDirection = htmlMarqueeDirection(3i32);
pub const htmlMarqueeDirectionup: htmlMarqueeDirection = htmlMarqueeDirection(5i32);
pub const htmlMarqueeDirectiondown: htmlMarqueeDirection = htmlMarqueeDirection(7i32);
pub const htmlMarqueeDirection_Max: htmlMarqueeDirection = htmlMarqueeDirection(2147483647i32);
impl ::core::marker::Copy for htmlMarqueeDirection {}
impl ::core::clone::Clone for htmlMarqueeDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlMediaErr(pub i32);
pub const htmlMediaErrAborted: htmlMediaErr = htmlMediaErr(0i32);
pub const htmlMediaErrNetwork: htmlMediaErr = htmlMediaErr(1i32);
pub const htmlMediaErrDecode: htmlMediaErr = htmlMediaErr(2i32);
pub const htmlMediaErrSrcNotSupported: htmlMediaErr = htmlMediaErr(3i32);
pub const htmlMediaErr_Max: htmlMediaErr = htmlMediaErr(2147483647i32);
impl ::core::marker::Copy for htmlMediaErr {}
impl ::core::clone::Clone for htmlMediaErr {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlMediaNetworkState(pub i32);
pub const htmlMediaNetworkStateEmpty: htmlMediaNetworkState = htmlMediaNetworkState(0i32);
pub const htmlMediaNetworkStateIdle: htmlMediaNetworkState = htmlMediaNetworkState(1i32);
pub const htmlMediaNetworkStateLoading: htmlMediaNetworkState = htmlMediaNetworkState(2i32);
pub const htmlMediaNetworkStateNoSource: htmlMediaNetworkState = htmlMediaNetworkState(3i32);
pub const htmlMediaNetworkState_Max: htmlMediaNetworkState = htmlMediaNetworkState(2147483647i32);
impl ::core::marker::Copy for htmlMediaNetworkState {}
impl ::core::clone::Clone for htmlMediaNetworkState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlMediaReadyState(pub i32);
pub const htmlMediaReadyStateHaveNothing: htmlMediaReadyState = htmlMediaReadyState(0i32);
pub const htmlMediaReadyStateHaveMetadata: htmlMediaReadyState = htmlMediaReadyState(1i32);
pub const htmlMediaReadyStateHaveCurrentData: htmlMediaReadyState = htmlMediaReadyState(2i32);
pub const htmlMediaReadyStateHaveFutureData: htmlMediaReadyState = htmlMediaReadyState(3i32);
pub const htmlMediaReadyStateHaveEnoughData: htmlMediaReadyState = htmlMediaReadyState(4i32);
pub const htmlMediaReadyState_Max: htmlMediaReadyState = htmlMediaReadyState(2147483647i32);
impl ::core::marker::Copy for htmlMediaReadyState {}
impl ::core::clone::Clone for htmlMediaReadyState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlMethod(pub i32);
pub const htmlMethodNotSet: htmlMethod = htmlMethod(0i32);
pub const htmlMethodGet: htmlMethod = htmlMethod(1i32);
pub const htmlMethodPost: htmlMethod = htmlMethod(2i32);
pub const htmlMethod_Max: htmlMethod = htmlMethod(2147483647i32);
impl ::core::marker::Copy for htmlMethod {}
impl ::core::clone::Clone for htmlMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlPersistState(pub i32);
pub const htmlPersistStateNormal: htmlPersistState = htmlPersistState(0i32);
pub const htmlPersistStateFavorite: htmlPersistState = htmlPersistState(1i32);
pub const htmlPersistStateHistory: htmlPersistState = htmlPersistState(2i32);
pub const htmlPersistStateSnapshot: htmlPersistState = htmlPersistState(3i32);
pub const htmlPersistStateUserData: htmlPersistState = htmlPersistState(4i32);
pub const htmlPersistState_Max: htmlPersistState = htmlPersistState(2147483647i32);
impl ::core::marker::Copy for htmlPersistState {}
impl ::core::clone::Clone for htmlPersistState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlReadyState(pub i32);
pub const htmlReadyStateuninitialized: htmlReadyState = htmlReadyState(0i32);
pub const htmlReadyStateloading: htmlReadyState = htmlReadyState(1i32);
pub const htmlReadyStateloaded: htmlReadyState = htmlReadyState(2i32);
pub const htmlReadyStateinteractive: htmlReadyState = htmlReadyState(3i32);
pub const htmlReadyStatecomplete: htmlReadyState = htmlReadyState(4i32);
pub const htmlReadyState_Max: htmlReadyState = htmlReadyState(2147483647i32);
impl ::core::marker::Copy for htmlReadyState {}
impl ::core::clone::Clone for htmlReadyState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlRules(pub i32);
pub const htmlRulesNotSet: htmlRules = htmlRules(0i32);
pub const htmlRulesnone: htmlRules = htmlRules(1i32);
pub const htmlRulesgroups: htmlRules = htmlRules(2i32);
pub const htmlRulesrows: htmlRules = htmlRules(3i32);
pub const htmlRulescols: htmlRules = htmlRules(4i32);
pub const htmlRulesall: htmlRules = htmlRules(5i32);
pub const htmlRules_Max: htmlRules = htmlRules(2147483647i32);
impl ::core::marker::Copy for htmlRules {}
impl ::core::clone::Clone for htmlRules {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlSelectExFlag(pub i32);
pub const htmlSelectExFlagNone: htmlSelectExFlag = htmlSelectExFlag(0i32);
pub const htmlSelectExFlagHideSelectionInDesign: htmlSelectExFlag = htmlSelectExFlag(1i32);
pub const htmlSelectExFlag_Max: htmlSelectExFlag = htmlSelectExFlag(2147483647i32);
impl ::core::marker::Copy for htmlSelectExFlag {}
impl ::core::clone::Clone for htmlSelectExFlag {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlSelectType(pub i32);
pub const htmlSelectTypeSelectOne: htmlSelectType = htmlSelectType(1i32);
pub const htmlSelectTypeSelectMultiple: htmlSelectType = htmlSelectType(2i32);
pub const htmlSelectType_Max: htmlSelectType = htmlSelectType(2147483647i32);
impl ::core::marker::Copy for htmlSelectType {}
impl ::core::clone::Clone for htmlSelectType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlSelection(pub i32);
pub const htmlSelectionNone: htmlSelection = htmlSelection(0i32);
pub const htmlSelectionText: htmlSelection = htmlSelection(1i32);
pub const htmlSelectionControl: htmlSelection = htmlSelection(2i32);
pub const htmlSelectionTable: htmlSelection = htmlSelection(3i32);
pub const htmlSelection_Max: htmlSelection = htmlSelection(2147483647i32);
impl ::core::marker::Copy for htmlSelection {}
impl ::core::clone::Clone for htmlSelection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlSpellCheck(pub i32);
pub const htmlSpellCheckNotSet: htmlSpellCheck = htmlSpellCheck(0i32);
pub const htmlSpellCheckTrue: htmlSpellCheck = htmlSpellCheck(1i32);
pub const htmlSpellCheckFalse: htmlSpellCheck = htmlSpellCheck(2i32);
pub const htmlSpellCheckDefault: htmlSpellCheck = htmlSpellCheck(3i32);
pub const htmlSpellCheck_Max: htmlSpellCheck = htmlSpellCheck(2147483647i32);
impl ::core::marker::Copy for htmlSpellCheck {}
impl ::core::clone::Clone for htmlSpellCheck {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlStart(pub i32);
pub const htmlStartfileopen: htmlStart = htmlStart(0i32);
pub const htmlStartmouseover: htmlStart = htmlStart(1i32);
pub const htmlStart_Max: htmlStart = htmlStart(2147483647i32);
impl ::core::marker::Copy for htmlStart {}
impl ::core::clone::Clone for htmlStart {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlTabIndex(pub i32);
pub const htmlTabIndexNotSet: htmlTabIndex = htmlTabIndex(-32768i32);
pub const htmlTabIndex_Max: htmlTabIndex = htmlTabIndex(2147483647i32);
impl ::core::marker::Copy for htmlTabIndex {}
impl ::core::clone::Clone for htmlTabIndex {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlUnit(pub i32);
pub const htmlUnitCharacter: htmlUnit = htmlUnit(1i32);
pub const htmlUnitWord: htmlUnit = htmlUnit(2i32);
pub const htmlUnitSentence: htmlUnit = htmlUnit(3i32);
pub const htmlUnitTextEdit: htmlUnit = htmlUnit(6i32);
pub const htmlUnit_Max: htmlUnit = htmlUnit(2147483647i32);
impl ::core::marker::Copy for htmlUnit {}
impl ::core::clone::Clone for htmlUnit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlWrap(pub i32);
pub const htmlWrapOff: htmlWrap = htmlWrap(1i32);
pub const htmlWrapSoft: htmlWrap = htmlWrap(2i32);
pub const htmlWrapHard: htmlWrap = htmlWrap(3i32);
pub const htmlWrap_Max: htmlWrap = htmlWrap(2147483647i32);
impl ::core::marker::Copy for htmlWrap {}
impl ::core::clone::Clone for htmlWrap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct htmlZOrder(pub i32);
pub const htmlZOrderFront: htmlZOrder = htmlZOrder(0i32);
pub const htmlZOrderBack: htmlZOrder = htmlZOrder(1i32);
pub const htmlZOrder_Max: htmlZOrder = htmlZOrder(2147483647i32);
impl ::core::marker::Copy for htmlZOrder {}
impl ::core::clone::Clone for htmlZOrder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct lengthAdjust(pub i32);
pub const LENGTHADJUST_UNKNOWN: lengthAdjust = lengthAdjust(0i32);
pub const LENGTHADJUST_SPACING: lengthAdjust = lengthAdjust(1i32);
pub const LENGTHADJUST_SPACINGANDGLYPHS: lengthAdjust = lengthAdjust(2i32);
pub const lengthAdjust_Max: lengthAdjust = lengthAdjust(2147483647i32);
impl ::core::marker::Copy for lengthAdjust {}
impl ::core::clone::Clone for lengthAdjust {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct mediaType(pub i32);
pub const mediaTypeNotSet: mediaType = mediaType(0i32);
pub const mediaTypeAll: mediaType = mediaType(511i32);
pub const mediaTypeAural: mediaType = mediaType(1i32);
pub const mediaTypeBraille: mediaType = mediaType(2i32);
pub const mediaTypeEmbossed: mediaType = mediaType(4i32);
pub const mediaTypeHandheld: mediaType = mediaType(8i32);
pub const mediaTypePrint: mediaType = mediaType(16i32);
pub const mediaTypeProjection: mediaType = mediaType(32i32);
pub const mediaTypeScreen: mediaType = mediaType(64i32);
pub const mediaTypeTty: mediaType = mediaType(128i32);
pub const mediaTypeTv: mediaType = mediaType(256i32);
pub const mediaType_Max: mediaType = mediaType(2147483647i32);
impl ::core::marker::Copy for mediaType {}
impl ::core::clone::Clone for mediaType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct sandboxAllow(pub i32);
pub const sandboxAllowScripts: sandboxAllow = sandboxAllow(0i32);
pub const sandboxAllowSameOrigin: sandboxAllow = sandboxAllow(1i32);
pub const sandboxAllowTopNavigation: sandboxAllow = sandboxAllow(2i32);
pub const sandboxAllowForms: sandboxAllow = sandboxAllow(3i32);
pub const sandboxAllowPopups: sandboxAllow = sandboxAllow(4i32);
pub const sandboxAllow_Max: sandboxAllow = sandboxAllow(2147483647i32);
impl ::core::marker::Copy for sandboxAllow {}
impl ::core::clone::Clone for sandboxAllow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleAccelerator(pub i32);
pub const styleAcceleratorFalse: styleAccelerator = styleAccelerator(0i32);
pub const styleAcceleratorTrue: styleAccelerator = styleAccelerator(1i32);
pub const styleAccelerator_Max: styleAccelerator = styleAccelerator(2147483647i32);
impl ::core::marker::Copy for styleAccelerator {}
impl ::core::clone::Clone for styleAccelerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleAlignContent(pub i32);
pub const styleAlignContentFlexStart: styleAlignContent = styleAlignContent(0i32);
pub const styleAlignContentFlexEnd: styleAlignContent = styleAlignContent(1i32);
pub const styleAlignContentCenter: styleAlignContent = styleAlignContent(2i32);
pub const styleAlignContentSpaceBetween: styleAlignContent = styleAlignContent(3i32);
pub const styleAlignContentSpaceAround: styleAlignContent = styleAlignContent(4i32);
pub const styleAlignContentStretch: styleAlignContent = styleAlignContent(5i32);
pub const styleAlignContentNotSet: styleAlignContent = styleAlignContent(6i32);
pub const styleAlignContent_Max: styleAlignContent = styleAlignContent(2147483647i32);
impl ::core::marker::Copy for styleAlignContent {}
impl ::core::clone::Clone for styleAlignContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleAlignItems(pub i32);
pub const styleAlignItemsFlexStart: styleAlignItems = styleAlignItems(0i32);
pub const styleAlignItemsFlexEnd: styleAlignItems = styleAlignItems(1i32);
pub const styleAlignItemsCenter: styleAlignItems = styleAlignItems(2i32);
pub const styleAlignItemsBaseline: styleAlignItems = styleAlignItems(3i32);
pub const styleAlignItemsStretch: styleAlignItems = styleAlignItems(4i32);
pub const styleAlignItemsNotSet: styleAlignItems = styleAlignItems(5i32);
pub const styleAlignItems_Max: styleAlignItems = styleAlignItems(2147483647i32);
impl ::core::marker::Copy for styleAlignItems {}
impl ::core::clone::Clone for styleAlignItems {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleAlignSelf(pub i32);
pub const styleAlignSelfFlexStart: styleAlignSelf = styleAlignSelf(0i32);
pub const styleAlignSelfFlexEnd: styleAlignSelf = styleAlignSelf(1i32);
pub const styleAlignSelfCenter: styleAlignSelf = styleAlignSelf(2i32);
pub const styleAlignSelfBaseline: styleAlignSelf = styleAlignSelf(3i32);
pub const styleAlignSelfStretch: styleAlignSelf = styleAlignSelf(4i32);
pub const styleAlignSelfAuto: styleAlignSelf = styleAlignSelf(5i32);
pub const styleAlignSelfNotSet: styleAlignSelf = styleAlignSelf(6i32);
pub const styleAlignSelf_Max: styleAlignSelf = styleAlignSelf(2147483647i32);
impl ::core::marker::Copy for styleAlignSelf {}
impl ::core::clone::Clone for styleAlignSelf {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleAlignmentBaseline(pub i32);
pub const styleAlignmentBaselineNotSet: styleAlignmentBaseline = styleAlignmentBaseline(0i32);
pub const styleAlignmentBaselineAfterEdge: styleAlignmentBaseline = styleAlignmentBaseline(1i32);
pub const styleAlignmentBaselineAlphabetic: styleAlignmentBaseline = styleAlignmentBaseline(2i32);
pub const styleAlignmentBaselineAuto: styleAlignmentBaseline = styleAlignmentBaseline(3i32);
pub const styleAlignmentBaselineBaseline: styleAlignmentBaseline = styleAlignmentBaseline(4i32);
pub const styleAlignmentBaselineBeforeEdge: styleAlignmentBaseline = styleAlignmentBaseline(5i32);
pub const styleAlignmentBaselineCentral: styleAlignmentBaseline = styleAlignmentBaseline(6i32);
pub const styleAlignmentBaselineHanging: styleAlignmentBaseline = styleAlignmentBaseline(7i32);
pub const styleAlignmentBaselineMathematical: styleAlignmentBaseline = styleAlignmentBaseline(8i32);
pub const styleAlignmentBaselineMiddle: styleAlignmentBaseline = styleAlignmentBaseline(9i32);
pub const styleAlignmentBaselineTextAfterEdge: styleAlignmentBaseline = styleAlignmentBaseline(10i32);
pub const styleAlignmentBaselineTextBeforeEdge: styleAlignmentBaseline = styleAlignmentBaseline(11i32);
pub const styleAlignmentBaselineIdeographic: styleAlignmentBaseline = styleAlignmentBaseline(12i32);
pub const styleAlignmentBaseline_Max: styleAlignmentBaseline = styleAlignmentBaseline(2147483647i32);
impl ::core::marker::Copy for styleAlignmentBaseline {}
impl ::core::clone::Clone for styleAlignmentBaseline {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleAttrType(pub i32);
pub const styleAttrTypeString: styleAttrType = styleAttrType(0i32);
pub const styleAttrTypeColor: styleAttrType = styleAttrType(1i32);
pub const styleAttrTypeUrl: styleAttrType = styleAttrType(2i32);
pub const styleAttrTypeInteger: styleAttrType = styleAttrType(3i32);
pub const styleAttrTypeNumber: styleAttrType = styleAttrType(4i32);
pub const styleAttrTypeLength: styleAttrType = styleAttrType(5i32);
pub const styleAttrTypePx: styleAttrType = styleAttrType(6i32);
pub const styleAttrTypeEm: styleAttrType = styleAttrType(7i32);
pub const styleAttrTypeEx: styleAttrType = styleAttrType(8i32);
pub const styleAttrTypeIn: styleAttrType = styleAttrType(9i32);
pub const styleAttrTypeCm: styleAttrType = styleAttrType(10i32);
pub const styleAttrTypeMm: styleAttrType = styleAttrType(11i32);
pub const styleAttrTypePt: styleAttrType = styleAttrType(12i32);
pub const styleAttrTypePc: styleAttrType = styleAttrType(13i32);
pub const styleAttrTypeRem: styleAttrType = styleAttrType(14i32);
pub const styleAttrTypeCh: styleAttrType = styleAttrType(15i32);
pub const styleAttrTypeVh: styleAttrType = styleAttrType(16i32);
pub const styleAttrTypeVw: styleAttrType = styleAttrType(17i32);
pub const styleAttrTypeVmin: styleAttrType = styleAttrType(18i32);
pub const styleAttrTypePercentage: styleAttrType = styleAttrType(19i32);
pub const styleAttrTypeAngle: styleAttrType = styleAttrType(20i32);
pub const styleAttrTypeDeg: styleAttrType = styleAttrType(21i32);
pub const styleAttrTypeRad: styleAttrType = styleAttrType(22i32);
pub const styleAttrTypeGrad: styleAttrType = styleAttrType(23i32);
pub const styleAttrTypeTime: styleAttrType = styleAttrType(24i32);
pub const styleAttrTypeS: styleAttrType = styleAttrType(25i32);
pub const styleAttrTypeMs: styleAttrType = styleAttrType(26i32);
pub const styleAttrType_Max: styleAttrType = styleAttrType(2147483647i32);
impl ::core::marker::Copy for styleAttrType {}
impl ::core::clone::Clone for styleAttrType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleAuto(pub i32);
pub const styleAutoAuto: styleAuto = styleAuto(0i32);
pub const styleAuto_Max: styleAuto = styleAuto(2147483647i32);
impl ::core::marker::Copy for styleAuto {}
impl ::core::clone::Clone for styleAuto {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBackfaceVisibility(pub i32);
pub const styleBackfaceVisibilityVisible: styleBackfaceVisibility = styleBackfaceVisibility(0i32);
pub const styleBackfaceVisibilityHidden: styleBackfaceVisibility = styleBackfaceVisibility(1i32);
pub const styleBackfaceVisibilityNotSet: styleBackfaceVisibility = styleBackfaceVisibility(2i32);
pub const styleBackfaceVisibility_Max: styleBackfaceVisibility = styleBackfaceVisibility(2147483647i32);
impl ::core::marker::Copy for styleBackfaceVisibility {}
impl ::core::clone::Clone for styleBackfaceVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBackgroundAttachment(pub i32);
pub const styleBackgroundAttachmentFixed: styleBackgroundAttachment = styleBackgroundAttachment(0i32);
pub const styleBackgroundAttachmentScroll: styleBackgroundAttachment = styleBackgroundAttachment(1i32);
pub const styleBackgroundAttachmentNotSet: styleBackgroundAttachment = styleBackgroundAttachment(2i32);
pub const styleBackgroundAttachment_Max: styleBackgroundAttachment = styleBackgroundAttachment(2147483647i32);
impl ::core::marker::Copy for styleBackgroundAttachment {}
impl ::core::clone::Clone for styleBackgroundAttachment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBackgroundAttachment3(pub i32);
pub const styleBackgroundAttachment3Fixed: styleBackgroundAttachment3 = styleBackgroundAttachment3(0i32);
pub const styleBackgroundAttachment3Scroll: styleBackgroundAttachment3 = styleBackgroundAttachment3(1i32);
pub const styleBackgroundAttachment3Local: styleBackgroundAttachment3 = styleBackgroundAttachment3(2i32);
pub const styleBackgroundAttachment3NotSet: styleBackgroundAttachment3 = styleBackgroundAttachment3(3i32);
pub const styleBackgroundAttachment3_Max: styleBackgroundAttachment3 = styleBackgroundAttachment3(2147483647i32);
impl ::core::marker::Copy for styleBackgroundAttachment3 {}
impl ::core::clone::Clone for styleBackgroundAttachment3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBackgroundClip(pub i32);
pub const styleBackgroundClipBorderBox: styleBackgroundClip = styleBackgroundClip(0i32);
pub const styleBackgroundClipPaddingBox: styleBackgroundClip = styleBackgroundClip(1i32);
pub const styleBackgroundClipContentBox: styleBackgroundClip = styleBackgroundClip(2i32);
pub const styleBackgroundClipNotSet: styleBackgroundClip = styleBackgroundClip(3i32);
pub const styleBackgroundClip_Max: styleBackgroundClip = styleBackgroundClip(2147483647i32);
impl ::core::marker::Copy for styleBackgroundClip {}
impl ::core::clone::Clone for styleBackgroundClip {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBackgroundOrigin(pub i32);
pub const styleBackgroundOriginBorderBox: styleBackgroundOrigin = styleBackgroundOrigin(0i32);
pub const styleBackgroundOriginPaddingBox: styleBackgroundOrigin = styleBackgroundOrigin(1i32);
pub const styleBackgroundOriginContentBox: styleBackgroundOrigin = styleBackgroundOrigin(2i32);
pub const styleBackgroundOriginNotSet: styleBackgroundOrigin = styleBackgroundOrigin(3i32);
pub const styleBackgroundOrigin_Max: styleBackgroundOrigin = styleBackgroundOrigin(2147483647i32);
impl ::core::marker::Copy for styleBackgroundOrigin {}
impl ::core::clone::Clone for styleBackgroundOrigin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBackgroundRepeat(pub i32);
pub const styleBackgroundRepeatRepeat: styleBackgroundRepeat = styleBackgroundRepeat(0i32);
pub const styleBackgroundRepeatRepeatX: styleBackgroundRepeat = styleBackgroundRepeat(1i32);
pub const styleBackgroundRepeatRepeatY: styleBackgroundRepeat = styleBackgroundRepeat(2i32);
pub const styleBackgroundRepeatNoRepeat: styleBackgroundRepeat = styleBackgroundRepeat(3i32);
pub const styleBackgroundRepeatNotSet: styleBackgroundRepeat = styleBackgroundRepeat(4i32);
pub const styleBackgroundRepeat_Max: styleBackgroundRepeat = styleBackgroundRepeat(2147483647i32);
impl ::core::marker::Copy for styleBackgroundRepeat {}
impl ::core::clone::Clone for styleBackgroundRepeat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBaselineShift(pub i32);
pub const styleBaselineShiftBaseline: styleBaselineShift = styleBaselineShift(0i32);
pub const styleBaselineShiftSub: styleBaselineShift = styleBaselineShift(1i32);
pub const styleBaselineShiftSuper: styleBaselineShift = styleBaselineShift(2i32);
pub const styleBaselineShift_Max: styleBaselineShift = styleBaselineShift(2147483647i32);
impl ::core::marker::Copy for styleBaselineShift {}
impl ::core::clone::Clone for styleBaselineShift {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBidi(pub i32);
pub const styleBidiNotSet: styleBidi = styleBidi(0i32);
pub const styleBidiNormal: styleBidi = styleBidi(1i32);
pub const styleBidiEmbed: styleBidi = styleBidi(2i32);
pub const styleBidiOverride: styleBidi = styleBidi(3i32);
pub const styleBidiInherit: styleBidi = styleBidi(4i32);
pub const styleBidi_Max: styleBidi = styleBidi(2147483647i32);
impl ::core::marker::Copy for styleBidi {}
impl ::core::clone::Clone for styleBidi {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBlockProgression(pub i32);
pub const styleBlockProgressionTb: styleBlockProgression = styleBlockProgression(0i32);
pub const styleBlockProgressionRl: styleBlockProgression = styleBlockProgression(1i32);
pub const styleBlockProgressionBt: styleBlockProgression = styleBlockProgression(2i32);
pub const styleBlockProgressionLr: styleBlockProgression = styleBlockProgression(3i32);
pub const styleBlockProgressionNotSet: styleBlockProgression = styleBlockProgression(4i32);
pub const styleBlockProgression_Max: styleBlockProgression = styleBlockProgression(2147483647i32);
impl ::core::marker::Copy for styleBlockProgression {}
impl ::core::clone::Clone for styleBlockProgression {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBool(pub i32);
pub const styleBoolFalse: styleBool = styleBool(0i32);
pub const styleBoolTrue: styleBool = styleBool(1i32);
pub const styleBool_Max: styleBool = styleBool(2147483647i32);
impl ::core::marker::Copy for styleBool {}
impl ::core::clone::Clone for styleBool {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBorderCollapse(pub i32);
pub const styleBorderCollapseNotSet: styleBorderCollapse = styleBorderCollapse(0i32);
pub const styleBorderCollapseSeparate: styleBorderCollapse = styleBorderCollapse(1i32);
pub const styleBorderCollapseCollapse: styleBorderCollapse = styleBorderCollapse(2i32);
pub const styleBorderCollapse_Max: styleBorderCollapse = styleBorderCollapse(2147483647i32);
impl ::core::marker::Copy for styleBorderCollapse {}
impl ::core::clone::Clone for styleBorderCollapse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBorderImageRepeat(pub i32);
pub const styleBorderImageRepeatStretch: styleBorderImageRepeat = styleBorderImageRepeat(0i32);
pub const styleBorderImageRepeatRepeat: styleBorderImageRepeat = styleBorderImageRepeat(1i32);
pub const styleBorderImageRepeatRound: styleBorderImageRepeat = styleBorderImageRepeat(2i32);
pub const styleBorderImageRepeatSpace: styleBorderImageRepeat = styleBorderImageRepeat(3i32);
pub const styleBorderImageRepeatNotSet: styleBorderImageRepeat = styleBorderImageRepeat(4i32);
pub const styleBorderImageRepeat_Max: styleBorderImageRepeat = styleBorderImageRepeat(2147483647i32);
impl ::core::marker::Copy for styleBorderImageRepeat {}
impl ::core::clone::Clone for styleBorderImageRepeat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBorderImageSliceFill(pub i32);
pub const styleBorderImageSliceFillNotSet: styleBorderImageSliceFill = styleBorderImageSliceFill(0i32);
pub const styleBorderImageSliceFillFill: styleBorderImageSliceFill = styleBorderImageSliceFill(1i32);
pub const styleBorderImageSliceFill_Max: styleBorderImageSliceFill = styleBorderImageSliceFill(2147483647i32);
impl ::core::marker::Copy for styleBorderImageSliceFill {}
impl ::core::clone::Clone for styleBorderImageSliceFill {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBorderStyle(pub i32);
pub const styleBorderStyleNotSet: styleBorderStyle = styleBorderStyle(0i32);
pub const styleBorderStyleDotted: styleBorderStyle = styleBorderStyle(1i32);
pub const styleBorderStyleDashed: styleBorderStyle = styleBorderStyle(2i32);
pub const styleBorderStyleSolid: styleBorderStyle = styleBorderStyle(3i32);
pub const styleBorderStyleDouble: styleBorderStyle = styleBorderStyle(4i32);
pub const styleBorderStyleGroove: styleBorderStyle = styleBorderStyle(5i32);
pub const styleBorderStyleRidge: styleBorderStyle = styleBorderStyle(6i32);
pub const styleBorderStyleInset: styleBorderStyle = styleBorderStyle(7i32);
pub const styleBorderStyleOutset: styleBorderStyle = styleBorderStyle(8i32);
pub const styleBorderStyleWindowInset: styleBorderStyle = styleBorderStyle(9i32);
pub const styleBorderStyleNone: styleBorderStyle = styleBorderStyle(10i32);
pub const styleBorderStyleHidden: styleBorderStyle = styleBorderStyle(11i32);
pub const styleBorderStyle_Max: styleBorderStyle = styleBorderStyle(2147483647i32);
impl ::core::marker::Copy for styleBorderStyle {}
impl ::core::clone::Clone for styleBorderStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBorderWidth(pub i32);
pub const styleBorderWidthThin: styleBorderWidth = styleBorderWidth(0i32);
pub const styleBorderWidthMedium: styleBorderWidth = styleBorderWidth(1i32);
pub const styleBorderWidthThick: styleBorderWidth = styleBorderWidth(2i32);
pub const styleBorderWidth_Max: styleBorderWidth = styleBorderWidth(2147483647i32);
impl ::core::marker::Copy for styleBorderWidth {}
impl ::core::clone::Clone for styleBorderWidth {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBoxSizing(pub i32);
pub const styleBoxSizingNotSet: styleBoxSizing = styleBoxSizing(0i32);
pub const styleBoxSizingContentBox: styleBoxSizing = styleBoxSizing(1i32);
pub const styleBoxSizingBorderBox: styleBoxSizing = styleBoxSizing(2i32);
pub const styleBoxSizing_Max: styleBoxSizing = styleBoxSizing(2147483647i32);
impl ::core::marker::Copy for styleBoxSizing {}
impl ::core::clone::Clone for styleBoxSizing {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBreak(pub i32);
pub const styleBreakNotSet: styleBreak = styleBreak(0i32);
pub const styleBreakAuto: styleBreak = styleBreak(1i32);
pub const styleBreakAlways: styleBreak = styleBreak(2i32);
pub const styleBreakAvoid: styleBreak = styleBreak(3i32);
pub const styleBreakLeft: styleBreak = styleBreak(4i32);
pub const styleBreakRight: styleBreak = styleBreak(5i32);
pub const styleBreakPage: styleBreak = styleBreak(6i32);
pub const styleBreakColumn: styleBreak = styleBreak(7i32);
pub const styleBreakAvoidPage: styleBreak = styleBreak(8i32);
pub const styleBreakAvoidColumn: styleBreak = styleBreak(9i32);
pub const styleBreak_Max: styleBreak = styleBreak(2147483647i32);
impl ::core::marker::Copy for styleBreak {}
impl ::core::clone::Clone for styleBreak {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleBreakInside(pub i32);
pub const styleBreakInsideNotSet: styleBreakInside = styleBreakInside(0i32);
pub const styleBreakInsideAuto: styleBreakInside = styleBreakInside(1i32);
pub const styleBreakInsideAvoid: styleBreakInside = styleBreakInside(2i32);
pub const styleBreakInsideAvoidPage: styleBreakInside = styleBreakInside(3i32);
pub const styleBreakInsideAvoidColumn: styleBreakInside = styleBreakInside(4i32);
pub const styleBreakInside_Max: styleBreakInside = styleBreakInside(2147483647i32);
impl ::core::marker::Copy for styleBreakInside {}
impl ::core::clone::Clone for styleBreakInside {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleCaptionSide(pub i32);
pub const styleCaptionSideNotSet: styleCaptionSide = styleCaptionSide(0i32);
pub const styleCaptionSideTop: styleCaptionSide = styleCaptionSide(1i32);
pub const styleCaptionSideBottom: styleCaptionSide = styleCaptionSide(2i32);
pub const styleCaptionSideLeft: styleCaptionSide = styleCaptionSide(3i32);
pub const styleCaptionSideRight: styleCaptionSide = styleCaptionSide(4i32);
pub const styleCaptionSide_Max: styleCaptionSide = styleCaptionSide(2147483647i32);
impl ::core::marker::Copy for styleCaptionSide {}
impl ::core::clone::Clone for styleCaptionSide {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleClipRule(pub i32);
pub const styleClipRuleNotSet: styleClipRule = styleClipRule(0i32);
pub const styleClipRuleNonZero: styleClipRule = styleClipRule(1i32);
pub const styleClipRuleEvenOdd: styleClipRule = styleClipRule(2i32);
pub const styleClipRule_Max: styleClipRule = styleClipRule(2147483647i32);
impl ::core::marker::Copy for styleClipRule {}
impl ::core::clone::Clone for styleClipRule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleColorInterpolationFilters(pub i32);
pub const styleColorInterpolationFiltersAuto: styleColorInterpolationFilters = styleColorInterpolationFilters(0i32);
pub const styleColorInterpolationFiltersSRgb: styleColorInterpolationFilters = styleColorInterpolationFilters(1i32);
pub const styleColorInterpolationFiltersLinearRgb: styleColorInterpolationFilters = styleColorInterpolationFilters(2i32);
pub const styleColorInterpolationFiltersNotSet: styleColorInterpolationFilters = styleColorInterpolationFilters(3i32);
pub const styleColorInterpolationFilters_Max: styleColorInterpolationFilters = styleColorInterpolationFilters(2147483647i32);
impl ::core::marker::Copy for styleColorInterpolationFilters {}
impl ::core::clone::Clone for styleColorInterpolationFilters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleColumnFill(pub i32);
pub const styleColumnFillAuto: styleColumnFill = styleColumnFill(0i32);
pub const styleColumnFillBalance: styleColumnFill = styleColumnFill(1i32);
pub const styleColumnFillNotSet: styleColumnFill = styleColumnFill(2i32);
pub const styleColumnFill_Max: styleColumnFill = styleColumnFill(2147483647i32);
impl ::core::marker::Copy for styleColumnFill {}
impl ::core::clone::Clone for styleColumnFill {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleColumnSpan(pub i32);
pub const styleColumnSpanNone: styleColumnSpan = styleColumnSpan(0i32);
pub const styleColumnSpanAll: styleColumnSpan = styleColumnSpan(1i32);
pub const styleColumnSpanOne: styleColumnSpan = styleColumnSpan(2i32);
pub const styleColumnSpanNotSet: styleColumnSpan = styleColumnSpan(3i32);
pub const styleColumnSpan_Max: styleColumnSpan = styleColumnSpan(2147483647i32);
impl ::core::marker::Copy for styleColumnSpan {}
impl ::core::clone::Clone for styleColumnSpan {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleCursor(pub i32);
pub const styleCursorAuto: styleCursor = styleCursor(0i32);
pub const styleCursorCrosshair: styleCursor = styleCursor(1i32);
pub const styleCursorDefault: styleCursor = styleCursor(2i32);
pub const styleCursorHand: styleCursor = styleCursor(3i32);
pub const styleCursorMove: styleCursor = styleCursor(4i32);
pub const styleCursorE_resize: styleCursor = styleCursor(5i32);
pub const styleCursorNe_resize: styleCursor = styleCursor(6i32);
pub const styleCursorNw_resize: styleCursor = styleCursor(7i32);
pub const styleCursorN_resize: styleCursor = styleCursor(8i32);
pub const styleCursorSe_resize: styleCursor = styleCursor(9i32);
pub const styleCursorSw_resize: styleCursor = styleCursor(10i32);
pub const styleCursorS_resize: styleCursor = styleCursor(11i32);
pub const styleCursorW_resize: styleCursor = styleCursor(12i32);
pub const styleCursorText: styleCursor = styleCursor(13i32);
pub const styleCursorWait: styleCursor = styleCursor(14i32);
pub const styleCursorHelp: styleCursor = styleCursor(15i32);
pub const styleCursorPointer: styleCursor = styleCursor(16i32);
pub const styleCursorProgress: styleCursor = styleCursor(17i32);
pub const styleCursorNot_allowed: styleCursor = styleCursor(18i32);
pub const styleCursorNo_drop: styleCursor = styleCursor(19i32);
pub const styleCursorVertical_text: styleCursor = styleCursor(20i32);
pub const styleCursorall_scroll: styleCursor = styleCursor(21i32);
pub const styleCursorcol_resize: styleCursor = styleCursor(22i32);
pub const styleCursorrow_resize: styleCursor = styleCursor(23i32);
pub const styleCursorNone: styleCursor = styleCursor(24i32);
pub const styleCursorContext_menu: styleCursor = styleCursor(25i32);
pub const styleCursorEw_resize: styleCursor = styleCursor(26i32);
pub const styleCursorNs_resize: styleCursor = styleCursor(27i32);
pub const styleCursorNesw_resize: styleCursor = styleCursor(28i32);
pub const styleCursorNwse_resize: styleCursor = styleCursor(29i32);
pub const styleCursorCell: styleCursor = styleCursor(30i32);
pub const styleCursorCopy: styleCursor = styleCursor(31i32);
pub const styleCursorAlias: styleCursor = styleCursor(32i32);
pub const styleCursorcustom: styleCursor = styleCursor(33i32);
pub const styleCursorNotSet: styleCursor = styleCursor(34i32);
pub const styleCursor_Max: styleCursor = styleCursor(2147483647i32);
impl ::core::marker::Copy for styleCursor {}
impl ::core::clone::Clone for styleCursor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleDataRepeat(pub i32);
pub const styleDataRepeatNone: styleDataRepeat = styleDataRepeat(0i32);
pub const styleDataRepeatInner: styleDataRepeat = styleDataRepeat(1i32);
pub const styleDataRepeat_Max: styleDataRepeat = styleDataRepeat(2147483647i32);
impl ::core::marker::Copy for styleDataRepeat {}
impl ::core::clone::Clone for styleDataRepeat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleDefaultTextSelection(pub i32);
pub const styleDefaultTextSelectionFalse: styleDefaultTextSelection = styleDefaultTextSelection(0i32);
pub const styleDefaultTextSelectionTrue: styleDefaultTextSelection = styleDefaultTextSelection(1i32);
pub const styleDefaultTextSelection_Max: styleDefaultTextSelection = styleDefaultTextSelection(2147483647i32);
impl ::core::marker::Copy for styleDefaultTextSelection {}
impl ::core::clone::Clone for styleDefaultTextSelection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleDir(pub i32);
pub const styleDirNotSet: styleDir = styleDir(0i32);
pub const styleDirLeftToRight: styleDir = styleDir(1i32);
pub const styleDirRightToLeft: styleDir = styleDir(2i32);
pub const styleDirInherit: styleDir = styleDir(3i32);
pub const styleDir_Max: styleDir = styleDir(2147483647i32);
impl ::core::marker::Copy for styleDir {}
impl ::core::clone::Clone for styleDir {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleDisplay(pub i32);
pub const styleDisplayNotSet: styleDisplay = styleDisplay(0i32);
pub const styleDisplayBlock: styleDisplay = styleDisplay(1i32);
pub const styleDisplayInline: styleDisplay = styleDisplay(2i32);
pub const styleDisplayListItem: styleDisplay = styleDisplay(3i32);
pub const styleDisplayNone: styleDisplay = styleDisplay(4i32);
pub const styleDisplayTableHeaderGroup: styleDisplay = styleDisplay(5i32);
pub const styleDisplayTableFooterGroup: styleDisplay = styleDisplay(6i32);
pub const styleDisplayInlineBlock: styleDisplay = styleDisplay(7i32);
pub const styleDisplayTable: styleDisplay = styleDisplay(8i32);
pub const styleDisplayInlineTable: styleDisplay = styleDisplay(9i32);
pub const styleDisplayTableRow: styleDisplay = styleDisplay(10i32);
pub const styleDisplayTableRowGroup: styleDisplay = styleDisplay(11i32);
pub const styleDisplayTableColumn: styleDisplay = styleDisplay(12i32);
pub const styleDisplayTableColumnGroup: styleDisplay = styleDisplay(13i32);
pub const styleDisplayTableCell: styleDisplay = styleDisplay(14i32);
pub const styleDisplayTableCaption: styleDisplay = styleDisplay(15i32);
pub const styleDisplayRunIn: styleDisplay = styleDisplay(16i32);
pub const styleDisplayRuby: styleDisplay = styleDisplay(17i32);
pub const styleDisplayRubyBase: styleDisplay = styleDisplay(18i32);
pub const styleDisplayRubyText: styleDisplay = styleDisplay(19i32);
pub const styleDisplayRubyBaseContainer: styleDisplay = styleDisplay(20i32);
pub const styleDisplayRubyTextContainer: styleDisplay = styleDisplay(21i32);
pub const styleDisplayMsFlexbox: styleDisplay = styleDisplay(22i32);
pub const styleDisplayMsInlineFlexbox: styleDisplay = styleDisplay(23i32);
pub const styleDisplayMsGrid: styleDisplay = styleDisplay(24i32);
pub const styleDisplayMsInlineGrid: styleDisplay = styleDisplay(25i32);
pub const styleDisplayFlex: styleDisplay = styleDisplay(26i32);
pub const styleDisplayInlineFlex: styleDisplay = styleDisplay(27i32);
pub const styleDisplayWebkitBox: styleDisplay = styleDisplay(28i32);
pub const styleDisplayWebkitInlineBox: styleDisplay = styleDisplay(29i32);
pub const styleDisplay_Max: styleDisplay = styleDisplay(2147483647i32);
impl ::core::marker::Copy for styleDisplay {}
impl ::core::clone::Clone for styleDisplay {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleDominantBaseline(pub i32);
pub const styleDominantBaselineNotSet: styleDominantBaseline = styleDominantBaseline(0i32);
pub const styleDominantBaselineAlphabetic: styleDominantBaseline = styleDominantBaseline(1i32);
pub const styleDominantBaselineAuto: styleDominantBaseline = styleDominantBaseline(2i32);
pub const styleDominantBaselineCentral: styleDominantBaseline = styleDominantBaseline(3i32);
pub const styleDominantBaselineHanging: styleDominantBaseline = styleDominantBaseline(4i32);
pub const styleDominantBaselineIdeographic: styleDominantBaseline = styleDominantBaseline(5i32);
pub const styleDominantBaselineMathematical: styleDominantBaseline = styleDominantBaseline(6i32);
pub const styleDominantBaselineMiddle: styleDominantBaseline = styleDominantBaseline(7i32);
pub const styleDominantBaselineNoChange: styleDominantBaseline = styleDominantBaseline(8i32);
pub const styleDominantBaselineResetSize: styleDominantBaseline = styleDominantBaseline(9i32);
pub const styleDominantBaselineTextAfterEdge: styleDominantBaseline = styleDominantBaseline(10i32);
pub const styleDominantBaselineTextBeforeEdge: styleDominantBaseline = styleDominantBaseline(11i32);
pub const styleDominantBaselineUseScript: styleDominantBaseline = styleDominantBaseline(12i32);
pub const styleDominantBaseline_Max: styleDominantBaseline = styleDominantBaseline(2147483647i32);
impl ::core::marker::Copy for styleDominantBaseline {}
impl ::core::clone::Clone for styleDominantBaseline {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleEmptyCells(pub i32);
pub const styleEmptyCellsNotSet: styleEmptyCells = styleEmptyCells(0i32);
pub const styleEmptyCellsShow: styleEmptyCells = styleEmptyCells(1i32);
pub const styleEmptyCellsHide: styleEmptyCells = styleEmptyCells(2i32);
pub const styleEmptyCells_Max: styleEmptyCells = styleEmptyCells(2147483647i32);
impl ::core::marker::Copy for styleEmptyCells {}
impl ::core::clone::Clone for styleEmptyCells {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleEnableBackground(pub i32);
pub const styleEnableBackgroundNotSet: styleEnableBackground = styleEnableBackground(0i32);
pub const styleEnableBackgroundAccumulate: styleEnableBackground = styleEnableBackground(1i32);
pub const styleEnableBackgroundNew: styleEnableBackground = styleEnableBackground(2i32);
pub const styleEnableBackgroundInherit: styleEnableBackground = styleEnableBackground(3i32);
pub const styleEnableBackground_Max: styleEnableBackground = styleEnableBackground(2147483647i32);
impl ::core::marker::Copy for styleEnableBackground {}
impl ::core::clone::Clone for styleEnableBackground {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleFillRule(pub i32);
pub const styleFillRuleNotSet: styleFillRule = styleFillRule(0i32);
pub const styleFillRuleNonZero: styleFillRule = styleFillRule(1i32);
pub const styleFillRuleEvenOdd: styleFillRule = styleFillRule(2i32);
pub const styleFillRule_Max: styleFillRule = styleFillRule(2147483647i32);
impl ::core::marker::Copy for styleFillRule {}
impl ::core::clone::Clone for styleFillRule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleFlex(pub i32);
pub const styleFlexNone: styleFlex = styleFlex(0i32);
pub const styleFlexNotSet: styleFlex = styleFlex(1i32);
pub const styleFlex_Max: styleFlex = styleFlex(2147483647i32);
impl ::core::marker::Copy for styleFlex {}
impl ::core::clone::Clone for styleFlex {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleFlexBasis(pub i32);
pub const styleFlexBasisAuto: styleFlexBasis = styleFlexBasis(0i32);
pub const styleFlexBasisNotSet: styleFlexBasis = styleFlexBasis(1i32);
pub const styleFlexBasis_Max: styleFlexBasis = styleFlexBasis(2147483647i32);
impl ::core::marker::Copy for styleFlexBasis {}
impl ::core::clone::Clone for styleFlexBasis {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleFlexDirection(pub i32);
pub const styleFlexDirectionRow: styleFlexDirection = styleFlexDirection(0i32);
pub const styleFlexDirectionRowReverse: styleFlexDirection = styleFlexDirection(1i32);
pub const styleFlexDirectionColumn: styleFlexDirection = styleFlexDirection(2i32);
pub const styleFlexDirectionColumnReverse: styleFlexDirection = styleFlexDirection(3i32);
pub const styleFlexDirectionNotSet: styleFlexDirection = styleFlexDirection(4i32);
pub const styleFlexDirection_Max: styleFlexDirection = styleFlexDirection(2147483647i32);
impl ::core::marker::Copy for styleFlexDirection {}
impl ::core::clone::Clone for styleFlexDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleFlexWrap(pub i32);
pub const styleFlexWrapNowrap: styleFlexWrap = styleFlexWrap(0i32);
pub const styleFlexWrapWrap: styleFlexWrap = styleFlexWrap(1i32);
pub const styleFlexWrapWrapReverse: styleFlexWrap = styleFlexWrap(2i32);
pub const styleFlexWrapNotSet: styleFlexWrap = styleFlexWrap(3i32);
pub const styleFlexWrap_Max: styleFlexWrap = styleFlexWrap(2147483647i32);
impl ::core::marker::Copy for styleFlexWrap {}
impl ::core::clone::Clone for styleFlexWrap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleFontSize(pub i32);
pub const styleFontSizeXXSmall: styleFontSize = styleFontSize(0i32);
pub const styleFontSizeXSmall: styleFontSize = styleFontSize(1i32);
pub const styleFontSizeSmall: styleFontSize = styleFontSize(2i32);
pub const styleFontSizeMedium: styleFontSize = styleFontSize(3i32);
pub const styleFontSizeLarge: styleFontSize = styleFontSize(4i32);
pub const styleFontSizeXLarge: styleFontSize = styleFontSize(5i32);
pub const styleFontSizeXXLarge: styleFontSize = styleFontSize(6i32);
pub const styleFontSizeSmaller: styleFontSize = styleFontSize(7i32);
pub const styleFontSizeLarger: styleFontSize = styleFontSize(8i32);
pub const styleFontSize_Max: styleFontSize = styleFontSize(2147483647i32);
impl ::core::marker::Copy for styleFontSize {}
impl ::core::clone::Clone for styleFontSize {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleFontStretch(pub i32);
pub const styleFontStretchNotSet: styleFontStretch = styleFontStretch(0i32);
pub const styleFontStretchWider: styleFontStretch = styleFontStretch(1i32);
pub const styleFontStretchNarrower: styleFontStretch = styleFontStretch(2i32);
pub const styleFontStretchUltraCondensed: styleFontStretch = styleFontStretch(3i32);
pub const styleFontStretchExtraCondensed: styleFontStretch = styleFontStretch(4i32);
pub const styleFontStretchCondensed: styleFontStretch = styleFontStretch(5i32);
pub const styleFontStretchSemiCondensed: styleFontStretch = styleFontStretch(6i32);
pub const styleFontStretchNormal: styleFontStretch = styleFontStretch(7i32);
pub const styleFontStretchSemiExpanded: styleFontStretch = styleFontStretch(8i32);
pub const styleFontStretchExpanded: styleFontStretch = styleFontStretch(9i32);
pub const styleFontStretchExtraExpanded: styleFontStretch = styleFontStretch(10i32);
pub const styleFontStretchUltraExpanded: styleFontStretch = styleFontStretch(11i32);
pub const styleFontStretch_Max: styleFontStretch = styleFontStretch(2147483647i32);
impl ::core::marker::Copy for styleFontStretch {}
impl ::core::clone::Clone for styleFontStretch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleFontStyle(pub i32);
pub const styleFontStyleNotSet: styleFontStyle = styleFontStyle(0i32);
pub const styleFontStyleItalic: styleFontStyle = styleFontStyle(1i32);
pub const styleFontStyleOblique: styleFontStyle = styleFontStyle(2i32);
pub const styleFontStyleNormal: styleFontStyle = styleFontStyle(3i32);
pub const styleFontStyle_Max: styleFontStyle = styleFontStyle(2147483647i32);
impl ::core::marker::Copy for styleFontStyle {}
impl ::core::clone::Clone for styleFontStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleFontVariant(pub i32);
pub const styleFontVariantNotSet: styleFontVariant = styleFontVariant(0i32);
pub const styleFontVariantSmallCaps: styleFontVariant = styleFontVariant(1i32);
pub const styleFontVariantNormal: styleFontVariant = styleFontVariant(2i32);
pub const styleFontVariant_Max: styleFontVariant = styleFontVariant(2147483647i32);
impl ::core::marker::Copy for styleFontVariant {}
impl ::core::clone::Clone for styleFontVariant {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleFontWeight(pub i32);
pub const styleFontWeightNotSet: styleFontWeight = styleFontWeight(0i32);
pub const styleFontWeight100: styleFontWeight = styleFontWeight(1i32);
pub const styleFontWeight200: styleFontWeight = styleFontWeight(2i32);
pub const styleFontWeight300: styleFontWeight = styleFontWeight(3i32);
pub const styleFontWeight400: styleFontWeight = styleFontWeight(4i32);
pub const styleFontWeight500: styleFontWeight = styleFontWeight(5i32);
pub const styleFontWeight600: styleFontWeight = styleFontWeight(6i32);
pub const styleFontWeight700: styleFontWeight = styleFontWeight(7i32);
pub const styleFontWeight800: styleFontWeight = styleFontWeight(8i32);
pub const styleFontWeight900: styleFontWeight = styleFontWeight(9i32);
pub const styleFontWeightNormal: styleFontWeight = styleFontWeight(10i32);
pub const styleFontWeightBold: styleFontWeight = styleFontWeight(11i32);
pub const styleFontWeightBolder: styleFontWeight = styleFontWeight(12i32);
pub const styleFontWeightLighter: styleFontWeight = styleFontWeight(13i32);
pub const styleFontWeight_Max: styleFontWeight = styleFontWeight(2147483647i32);
impl ::core::marker::Copy for styleFontWeight {}
impl ::core::clone::Clone for styleFontWeight {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleGridColumn(pub i32);
pub const styleGridColumnNotSet: styleGridColumn = styleGridColumn(0i32);
pub const styleGridColumn_Max: styleGridColumn = styleGridColumn(2147483647i32);
impl ::core::marker::Copy for styleGridColumn {}
impl ::core::clone::Clone for styleGridColumn {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleGridColumnAlign(pub i32);
pub const styleGridColumnAlignCenter: styleGridColumnAlign = styleGridColumnAlign(0i32);
pub const styleGridColumnAlignEnd: styleGridColumnAlign = styleGridColumnAlign(1i32);
pub const styleGridColumnAlignStart: styleGridColumnAlign = styleGridColumnAlign(2i32);
pub const styleGridColumnAlignStretch: styleGridColumnAlign = styleGridColumnAlign(3i32);
pub const styleGridColumnAlignNotSet: styleGridColumnAlign = styleGridColumnAlign(4i32);
pub const styleGridColumnAlign_Max: styleGridColumnAlign = styleGridColumnAlign(2147483647i32);
impl ::core::marker::Copy for styleGridColumnAlign {}
impl ::core::clone::Clone for styleGridColumnAlign {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleGridColumnSpan(pub i32);
pub const styleGridColumnSpanNotSet: styleGridColumnSpan = styleGridColumnSpan(0i32);
pub const styleGridColumnSpan_Max: styleGridColumnSpan = styleGridColumnSpan(2147483647i32);
impl ::core::marker::Copy for styleGridColumnSpan {}
impl ::core::clone::Clone for styleGridColumnSpan {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleGridRow(pub i32);
pub const styleGridRowNotSet: styleGridRow = styleGridRow(0i32);
pub const styleGridRow_Max: styleGridRow = styleGridRow(2147483647i32);
impl ::core::marker::Copy for styleGridRow {}
impl ::core::clone::Clone for styleGridRow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleGridRowAlign(pub i32);
pub const styleGridRowAlignCenter: styleGridRowAlign = styleGridRowAlign(0i32);
pub const styleGridRowAlignEnd: styleGridRowAlign = styleGridRowAlign(1i32);
pub const styleGridRowAlignStart: styleGridRowAlign = styleGridRowAlign(2i32);
pub const styleGridRowAlignStretch: styleGridRowAlign = styleGridRowAlign(3i32);
pub const styleGridRowAlignNotSet: styleGridRowAlign = styleGridRowAlign(4i32);
pub const styleGridRowAlign_Max: styleGridRowAlign = styleGridRowAlign(2147483647i32);
impl ::core::marker::Copy for styleGridRowAlign {}
impl ::core::clone::Clone for styleGridRowAlign {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleGridRowSpan(pub i32);
pub const styleGridRowSpanNotSet: styleGridRowSpan = styleGridRowSpan(0i32);
pub const styleGridRowSpan_Max: styleGridRowSpan = styleGridRowSpan(2147483647i32);
impl ::core::marker::Copy for styleGridRowSpan {}
impl ::core::clone::Clone for styleGridRowSpan {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleHyphenateLimitLines(pub i32);
pub const styleHyphenateLimitLinesNoLimit: styleHyphenateLimitLines = styleHyphenateLimitLines(0i32);
pub const styleHyphenateLimitLines_Max: styleHyphenateLimitLines = styleHyphenateLimitLines(2147483647i32);
impl ::core::marker::Copy for styleHyphenateLimitLines {}
impl ::core::clone::Clone for styleHyphenateLimitLines {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleHyphens(pub i32);
pub const styleHyphensNone: styleHyphens = styleHyphens(0i32);
pub const styleHyphensManual: styleHyphens = styleHyphens(1i32);
pub const styleHyphensAuto: styleHyphens = styleHyphens(2i32);
pub const styleHyphensNotSet: styleHyphens = styleHyphens(3i32);
pub const styleHyphens_Max: styleHyphens = styleHyphens(2147483647i32);
impl ::core::marker::Copy for styleHyphens {}
impl ::core::clone::Clone for styleHyphens {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleImeMode(pub i32);
pub const styleImeModeAuto: styleImeMode = styleImeMode(0i32);
pub const styleImeModeActive: styleImeMode = styleImeMode(1i32);
pub const styleImeModeInactive: styleImeMode = styleImeMode(2i32);
pub const styleImeModeDisabled: styleImeMode = styleImeMode(3i32);
pub const styleImeModeNotSet: styleImeMode = styleImeMode(4i32);
pub const styleImeMode_Max: styleImeMode = styleImeMode(2147483647i32);
impl ::core::marker::Copy for styleImeMode {}
impl ::core::clone::Clone for styleImeMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleInitialColor(pub i32);
pub const styleInitialColorNoInitial: styleInitialColor = styleInitialColor(0i32);
pub const styleInitialColorColorProperty: styleInitialColor = styleInitialColor(1i32);
pub const styleInitialColorTransparent: styleInitialColor = styleInitialColor(2i32);
pub const styleInitialColorInvert: styleInitialColor = styleInitialColor(3i32);
pub const styleInitialColor_Max: styleInitialColor = styleInitialColor(2147483647i32);
impl ::core::marker::Copy for styleInitialColor {}
impl ::core::clone::Clone for styleInitialColor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleInitialString(pub i32);
pub const styleInitialStringNoInitial: styleInitialString = styleInitialString(0i32);
pub const styleInitialStringNone: styleInitialString = styleInitialString(1i32);
pub const styleInitialStringAuto: styleInitialString = styleInitialString(2i32);
pub const styleInitialStringNormal: styleInitialString = styleInitialString(3i32);
pub const styleInitialString_Max: styleInitialString = styleInitialString(2147483647i32);
impl ::core::marker::Copy for styleInitialString {}
impl ::core::clone::Clone for styleInitialString {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleInterpolation(pub i32);
pub const styleInterpolationNotSet: styleInterpolation = styleInterpolation(0i32);
pub const styleInterpolationNN: styleInterpolation = styleInterpolation(1i32);
pub const styleInterpolationBCH: styleInterpolation = styleInterpolation(2i32);
pub const styleInterpolation_Max: styleInterpolation = styleInterpolation(2147483647i32);
impl ::core::marker::Copy for styleInterpolation {}
impl ::core::clone::Clone for styleInterpolation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleJustifyContent(pub i32);
pub const styleJustifyContentFlexStart: styleJustifyContent = styleJustifyContent(0i32);
pub const styleJustifyContentFlexEnd: styleJustifyContent = styleJustifyContent(1i32);
pub const styleJustifyContentCenter: styleJustifyContent = styleJustifyContent(2i32);
pub const styleJustifyContentSpaceBetween: styleJustifyContent = styleJustifyContent(3i32);
pub const styleJustifyContentSpaceAround: styleJustifyContent = styleJustifyContent(4i32);
pub const styleJustifyContentNotSet: styleJustifyContent = styleJustifyContent(5i32);
pub const styleJustifyContent_Max: styleJustifyContent = styleJustifyContent(2147483647i32);
impl ::core::marker::Copy for styleJustifyContent {}
impl ::core::clone::Clone for styleJustifyContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleLayoutFlow(pub i32);
pub const styleLayoutFlowHorizontal: styleLayoutFlow = styleLayoutFlow(0i32);
pub const styleLayoutFlowVerticalIdeographic: styleLayoutFlow = styleLayoutFlow(1i32);
pub const styleLayoutFlowNotSet: styleLayoutFlow = styleLayoutFlow(2i32);
pub const styleLayoutFlow_Max: styleLayoutFlow = styleLayoutFlow(2147483647i32);
impl ::core::marker::Copy for styleLayoutFlow {}
impl ::core::clone::Clone for styleLayoutFlow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleLayoutGridChar(pub i32);
pub const styleLayoutGridCharNotSet: styleLayoutGridChar = styleLayoutGridChar(0i32);
pub const styleLayoutGridCharAuto: styleLayoutGridChar = styleLayoutGridChar(1i32);
pub const styleLayoutGridCharNone: styleLayoutGridChar = styleLayoutGridChar(2i32);
pub const styleLayoutGridChar_Max: styleLayoutGridChar = styleLayoutGridChar(2147483647i32);
impl ::core::marker::Copy for styleLayoutGridChar {}
impl ::core::clone::Clone for styleLayoutGridChar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleLayoutGridLine(pub i32);
pub const styleLayoutGridLineNotSet: styleLayoutGridLine = styleLayoutGridLine(0i32);
pub const styleLayoutGridLineAuto: styleLayoutGridLine = styleLayoutGridLine(1i32);
pub const styleLayoutGridLineNone: styleLayoutGridLine = styleLayoutGridLine(2i32);
pub const styleLayoutGridLine_Max: styleLayoutGridLine = styleLayoutGridLine(2147483647i32);
impl ::core::marker::Copy for styleLayoutGridLine {}
impl ::core::clone::Clone for styleLayoutGridLine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleLayoutGridMode(pub i32);
pub const styleLayoutGridModeNotSet: styleLayoutGridMode = styleLayoutGridMode(0i32);
pub const styleLayoutGridModeChar: styleLayoutGridMode = styleLayoutGridMode(1i32);
pub const styleLayoutGridModeLine: styleLayoutGridMode = styleLayoutGridMode(2i32);
pub const styleLayoutGridModeBoth: styleLayoutGridMode = styleLayoutGridMode(3i32);
pub const styleLayoutGridModeNone: styleLayoutGridMode = styleLayoutGridMode(4i32);
pub const styleLayoutGridMode_Max: styleLayoutGridMode = styleLayoutGridMode(2147483647i32);
impl ::core::marker::Copy for styleLayoutGridMode {}
impl ::core::clone::Clone for styleLayoutGridMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleLayoutGridType(pub i32);
pub const styleLayoutGridTypeNotSet: styleLayoutGridType = styleLayoutGridType(0i32);
pub const styleLayoutGridTypeLoose: styleLayoutGridType = styleLayoutGridType(1i32);
pub const styleLayoutGridTypeStrict: styleLayoutGridType = styleLayoutGridType(2i32);
pub const styleLayoutGridTypeFixed: styleLayoutGridType = styleLayoutGridType(3i32);
pub const styleLayoutGridType_Max: styleLayoutGridType = styleLayoutGridType(2147483647i32);
impl ::core::marker::Copy for styleLayoutGridType {}
impl ::core::clone::Clone for styleLayoutGridType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleLineBreak(pub i32);
pub const styleLineBreakNotSet: styleLineBreak = styleLineBreak(0i32);
pub const styleLineBreakNormal: styleLineBreak = styleLineBreak(1i32);
pub const styleLineBreakStrict: styleLineBreak = styleLineBreak(2i32);
pub const styleLineBreak_Max: styleLineBreak = styleLineBreak(2147483647i32);
impl ::core::marker::Copy for styleLineBreak {}
impl ::core::clone::Clone for styleLineBreak {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleListStylePosition(pub i32);
pub const styleListStylePositionNotSet: styleListStylePosition = styleListStylePosition(0i32);
pub const styleListStylePositionInside: styleListStylePosition = styleListStylePosition(1i32);
pub const styleListStylePositionOutSide: styleListStylePosition = styleListStylePosition(2i32);
pub const styleListStylePosition_Max: styleListStylePosition = styleListStylePosition(2147483647i32);
impl ::core::marker::Copy for styleListStylePosition {}
impl ::core::clone::Clone for styleListStylePosition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleListStyleType(pub i32);
pub const styleListStyleTypeNotSet: styleListStyleType = styleListStyleType(0i32);
pub const styleListStyleTypeDisc: styleListStyleType = styleListStyleType(1i32);
pub const styleListStyleTypeCircle: styleListStyleType = styleListStyleType(2i32);
pub const styleListStyleTypeSquare: styleListStyleType = styleListStyleType(3i32);
pub const styleListStyleTypeDecimal: styleListStyleType = styleListStyleType(4i32);
pub const styleListStyleTypeLowerRoman: styleListStyleType = styleListStyleType(5i32);
pub const styleListStyleTypeUpperRoman: styleListStyleType = styleListStyleType(6i32);
pub const styleListStyleTypeLowerAlpha: styleListStyleType = styleListStyleType(7i32);
pub const styleListStyleTypeUpperAlpha: styleListStyleType = styleListStyleType(8i32);
pub const styleListStyleTypeNone: styleListStyleType = styleListStyleType(9i32);
pub const styleListStyleTypeDecimalLeadingZero: styleListStyleType = styleListStyleType(10i32);
pub const styleListStyleTypeGeorgian: styleListStyleType = styleListStyleType(11i32);
pub const styleListStyleTypeArmenian: styleListStyleType = styleListStyleType(12i32);
pub const styleListStyleTypeUpperLatin: styleListStyleType = styleListStyleType(13i32);
pub const styleListStyleTypeLowerLatin: styleListStyleType = styleListStyleType(14i32);
pub const styleListStyleTypeUpperGreek: styleListStyleType = styleListStyleType(15i32);
pub const styleListStyleTypeLowerGreek: styleListStyleType = styleListStyleType(16i32);
pub const styleListStyleType_Max: styleListStyleType = styleListStyleType(2147483647i32);
impl ::core::marker::Copy for styleListStyleType {}
impl ::core::clone::Clone for styleListStyleType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsAnimationDirection(pub i32);
pub const styleMsAnimationDirectionNormal: styleMsAnimationDirection = styleMsAnimationDirection(0i32);
pub const styleMsAnimationDirectionAlternate: styleMsAnimationDirection = styleMsAnimationDirection(1i32);
pub const styleMsAnimationDirectionReverse: styleMsAnimationDirection = styleMsAnimationDirection(2i32);
pub const styleMsAnimationDirectionAlternateReverse: styleMsAnimationDirection = styleMsAnimationDirection(3i32);
pub const styleMsAnimationDirectionNotSet: styleMsAnimationDirection = styleMsAnimationDirection(4i32);
pub const styleMsAnimationDirection_Max: styleMsAnimationDirection = styleMsAnimationDirection(2147483647i32);
impl ::core::marker::Copy for styleMsAnimationDirection {}
impl ::core::clone::Clone for styleMsAnimationDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsAnimationFillMode(pub i32);
pub const styleMsAnimationFillModeNone: styleMsAnimationFillMode = styleMsAnimationFillMode(0i32);
pub const styleMsAnimationFillModeForwards: styleMsAnimationFillMode = styleMsAnimationFillMode(1i32);
pub const styleMsAnimationFillModeBackwards: styleMsAnimationFillMode = styleMsAnimationFillMode(2i32);
pub const styleMsAnimationFillModeBoth: styleMsAnimationFillMode = styleMsAnimationFillMode(3i32);
pub const styleMsAnimationFillModeNotSet: styleMsAnimationFillMode = styleMsAnimationFillMode(4i32);
pub const styleMsAnimationFillMode_Max: styleMsAnimationFillMode = styleMsAnimationFillMode(2147483647i32);
impl ::core::marker::Copy for styleMsAnimationFillMode {}
impl ::core::clone::Clone for styleMsAnimationFillMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsAnimationPlayState(pub i32);
pub const styleMsAnimationPlayStateRunning: styleMsAnimationPlayState = styleMsAnimationPlayState(0i32);
pub const styleMsAnimationPlayStatePaused: styleMsAnimationPlayState = styleMsAnimationPlayState(1i32);
pub const styleMsAnimationPlayStateNotSet: styleMsAnimationPlayState = styleMsAnimationPlayState(2i32);
pub const styleMsAnimationPlayState_Max: styleMsAnimationPlayState = styleMsAnimationPlayState(2147483647i32);
impl ::core::marker::Copy for styleMsAnimationPlayState {}
impl ::core::clone::Clone for styleMsAnimationPlayState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsContentZoomChaining(pub i32);
pub const styleMsContentZoomChainingNotSet: styleMsContentZoomChaining = styleMsContentZoomChaining(0i32);
pub const styleMsContentZoomChainingNone: styleMsContentZoomChaining = styleMsContentZoomChaining(1i32);
pub const styleMsContentZoomChainingChained: styleMsContentZoomChaining = styleMsContentZoomChaining(2i32);
pub const styleMsContentZoomChaining_Max: styleMsContentZoomChaining = styleMsContentZoomChaining(2147483647i32);
impl ::core::marker::Copy for styleMsContentZoomChaining {}
impl ::core::clone::Clone for styleMsContentZoomChaining {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsContentZoomSnapType(pub i32);
pub const styleMsContentZoomSnapTypeNotSet: styleMsContentZoomSnapType = styleMsContentZoomSnapType(0i32);
pub const styleMsContentZoomSnapTypeNone: styleMsContentZoomSnapType = styleMsContentZoomSnapType(1i32);
pub const styleMsContentZoomSnapTypeMandatory: styleMsContentZoomSnapType = styleMsContentZoomSnapType(2i32);
pub const styleMsContentZoomSnapTypeProximity: styleMsContentZoomSnapType = styleMsContentZoomSnapType(3i32);
pub const styleMsContentZoomSnapType_Max: styleMsContentZoomSnapType = styleMsContentZoomSnapType(2147483647i32);
impl ::core::marker::Copy for styleMsContentZoomSnapType {}
impl ::core::clone::Clone for styleMsContentZoomSnapType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsContentZooming(pub i32);
pub const styleMsContentZoomingNotSet: styleMsContentZooming = styleMsContentZooming(0i32);
pub const styleMsContentZoomingNone: styleMsContentZooming = styleMsContentZooming(1i32);
pub const styleMsContentZoomingZoom: styleMsContentZooming = styleMsContentZooming(2i32);
pub const styleMsContentZooming_Max: styleMsContentZooming = styleMsContentZooming(2147483647i32);
impl ::core::marker::Copy for styleMsContentZooming {}
impl ::core::clone::Clone for styleMsContentZooming {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsFlexAlign(pub i32);
pub const styleMsFlexAlignStart: styleMsFlexAlign = styleMsFlexAlign(0i32);
pub const styleMsFlexAlignEnd: styleMsFlexAlign = styleMsFlexAlign(1i32);
pub const styleMsFlexAlignCenter: styleMsFlexAlign = styleMsFlexAlign(2i32);
pub const styleMsFlexAlignBaseline: styleMsFlexAlign = styleMsFlexAlign(3i32);
pub const styleMsFlexAlignStretch: styleMsFlexAlign = styleMsFlexAlign(4i32);
pub const styleMsFlexAlignNotSet: styleMsFlexAlign = styleMsFlexAlign(5i32);
pub const styleMsFlexAlign_Max: styleMsFlexAlign = styleMsFlexAlign(2147483647i32);
impl ::core::marker::Copy for styleMsFlexAlign {}
impl ::core::clone::Clone for styleMsFlexAlign {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsFlexItemAlign(pub i32);
pub const styleMsFlexItemAlignStart: styleMsFlexItemAlign = styleMsFlexItemAlign(0i32);
pub const styleMsFlexItemAlignEnd: styleMsFlexItemAlign = styleMsFlexItemAlign(1i32);
pub const styleMsFlexItemAlignCenter: styleMsFlexItemAlign = styleMsFlexItemAlign(2i32);
pub const styleMsFlexItemAlignBaseline: styleMsFlexItemAlign = styleMsFlexItemAlign(3i32);
pub const styleMsFlexItemAlignStretch: styleMsFlexItemAlign = styleMsFlexItemAlign(4i32);
pub const styleMsFlexItemAlignAuto: styleMsFlexItemAlign = styleMsFlexItemAlign(5i32);
pub const styleMsFlexItemAlignNotSet: styleMsFlexItemAlign = styleMsFlexItemAlign(6i32);
pub const styleMsFlexItemAlign_Max: styleMsFlexItemAlign = styleMsFlexItemAlign(2147483647i32);
impl ::core::marker::Copy for styleMsFlexItemAlign {}
impl ::core::clone::Clone for styleMsFlexItemAlign {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsFlexLinePack(pub i32);
pub const styleMsFlexLinePackStart: styleMsFlexLinePack = styleMsFlexLinePack(0i32);
pub const styleMsFlexLinePackEnd: styleMsFlexLinePack = styleMsFlexLinePack(1i32);
pub const styleMsFlexLinePackCenter: styleMsFlexLinePack = styleMsFlexLinePack(2i32);
pub const styleMsFlexLinePackJustify: styleMsFlexLinePack = styleMsFlexLinePack(3i32);
pub const styleMsFlexLinePackDistribute: styleMsFlexLinePack = styleMsFlexLinePack(4i32);
pub const styleMsFlexLinePackStretch: styleMsFlexLinePack = styleMsFlexLinePack(5i32);
pub const styleMsFlexLinePackNotSet: styleMsFlexLinePack = styleMsFlexLinePack(6i32);
pub const styleMsFlexLinePack_Max: styleMsFlexLinePack = styleMsFlexLinePack(2147483647i32);
impl ::core::marker::Copy for styleMsFlexLinePack {}
impl ::core::clone::Clone for styleMsFlexLinePack {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsFlexPack(pub i32);
pub const styleMsFlexPackStart: styleMsFlexPack = styleMsFlexPack(0i32);
pub const styleMsFlexPackEnd: styleMsFlexPack = styleMsFlexPack(1i32);
pub const styleMsFlexPackCenter: styleMsFlexPack = styleMsFlexPack(2i32);
pub const styleMsFlexPackJustify: styleMsFlexPack = styleMsFlexPack(3i32);
pub const styleMsFlexPackDistribute: styleMsFlexPack = styleMsFlexPack(4i32);
pub const styleMsFlexPackNotSet: styleMsFlexPack = styleMsFlexPack(5i32);
pub const styleMsFlexPack_Max: styleMsFlexPack = styleMsFlexPack(2147483647i32);
impl ::core::marker::Copy for styleMsFlexPack {}
impl ::core::clone::Clone for styleMsFlexPack {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsHighContrastAdjust(pub i32);
pub const styleMsHighContrastAdjustNotSet: styleMsHighContrastAdjust = styleMsHighContrastAdjust(0i32);
pub const styleMsHighContrastAdjustAuto: styleMsHighContrastAdjust = styleMsHighContrastAdjust(1i32);
pub const styleMsHighContrastAdjustNone: styleMsHighContrastAdjust = styleMsHighContrastAdjust(2i32);
pub const styleMsHighContrastAdjust_Max: styleMsHighContrastAdjust = styleMsHighContrastAdjust(2147483647i32);
impl ::core::marker::Copy for styleMsHighContrastAdjust {}
impl ::core::clone::Clone for styleMsHighContrastAdjust {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsImeAlign(pub i32);
pub const styleMsImeAlignAuto: styleMsImeAlign = styleMsImeAlign(0i32);
pub const styleMsImeAlignAfter: styleMsImeAlign = styleMsImeAlign(1i32);
pub const styleMsImeAlignNotSet: styleMsImeAlign = styleMsImeAlign(2i32);
pub const styleMsImeAlign_Max: styleMsImeAlign = styleMsImeAlign(2147483647i32);
impl ::core::marker::Copy for styleMsImeAlign {}
impl ::core::clone::Clone for styleMsImeAlign {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsOverflowStyle(pub i32);
pub const styleMsOverflowStyleNotSet: styleMsOverflowStyle = styleMsOverflowStyle(0i32);
pub const styleMsOverflowStyleAuto: styleMsOverflowStyle = styleMsOverflowStyle(1i32);
pub const styleMsOverflowStyleNone: styleMsOverflowStyle = styleMsOverflowStyle(2i32);
pub const styleMsOverflowStyleScrollbar: styleMsOverflowStyle = styleMsOverflowStyle(3i32);
pub const styleMsOverflowStyleMsAutoHidingScrollbar: styleMsOverflowStyle = styleMsOverflowStyle(4i32);
pub const styleMsOverflowStyle_Max: styleMsOverflowStyle = styleMsOverflowStyle(2147483647i32);
impl ::core::marker::Copy for styleMsOverflowStyle {}
impl ::core::clone::Clone for styleMsOverflowStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsScrollChaining(pub i32);
pub const styleMsScrollChainingNotSet: styleMsScrollChaining = styleMsScrollChaining(0i32);
pub const styleMsScrollChainingNone: styleMsScrollChaining = styleMsScrollChaining(1i32);
pub const styleMsScrollChainingChained: styleMsScrollChaining = styleMsScrollChaining(2i32);
pub const styleMsScrollChaining_Max: styleMsScrollChaining = styleMsScrollChaining(2147483647i32);
impl ::core::marker::Copy for styleMsScrollChaining {}
impl ::core::clone::Clone for styleMsScrollChaining {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsScrollRails(pub i32);
pub const styleMsScrollRailsNotSet: styleMsScrollRails = styleMsScrollRails(0i32);
pub const styleMsScrollRailsNone: styleMsScrollRails = styleMsScrollRails(1i32);
pub const styleMsScrollRailsRailed: styleMsScrollRails = styleMsScrollRails(2i32);
pub const styleMsScrollRails_Max: styleMsScrollRails = styleMsScrollRails(2147483647i32);
impl ::core::marker::Copy for styleMsScrollRails {}
impl ::core::clone::Clone for styleMsScrollRails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsScrollSnapType(pub i32);
pub const styleMsScrollSnapTypeNotSet: styleMsScrollSnapType = styleMsScrollSnapType(0i32);
pub const styleMsScrollSnapTypeNone: styleMsScrollSnapType = styleMsScrollSnapType(1i32);
pub const styleMsScrollSnapTypeMandatory: styleMsScrollSnapType = styleMsScrollSnapType(2i32);
pub const styleMsScrollSnapTypeProximity: styleMsScrollSnapType = styleMsScrollSnapType(3i32);
pub const styleMsScrollSnapType_Max: styleMsScrollSnapType = styleMsScrollSnapType(2147483647i32);
impl ::core::marker::Copy for styleMsScrollSnapType {}
impl ::core::clone::Clone for styleMsScrollSnapType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsScrollTranslation(pub i32);
pub const styleMsScrollTranslationNotSet: styleMsScrollTranslation = styleMsScrollTranslation(0i32);
pub const styleMsScrollTranslationNone: styleMsScrollTranslation = styleMsScrollTranslation(1i32);
pub const styleMsScrollTranslationVtoH: styleMsScrollTranslation = styleMsScrollTranslation(2i32);
pub const styleMsScrollTranslation_Max: styleMsScrollTranslation = styleMsScrollTranslation(2147483647i32);
impl ::core::marker::Copy for styleMsScrollTranslation {}
impl ::core::clone::Clone for styleMsScrollTranslation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsTextCombineHorizontal(pub i32);
pub const styleMsTextCombineHorizontalNone: styleMsTextCombineHorizontal = styleMsTextCombineHorizontal(0i32);
pub const styleMsTextCombineHorizontalAll: styleMsTextCombineHorizontal = styleMsTextCombineHorizontal(1i32);
pub const styleMsTextCombineHorizontalDigits: styleMsTextCombineHorizontal = styleMsTextCombineHorizontal(2i32);
pub const styleMsTextCombineHorizontalNotSet: styleMsTextCombineHorizontal = styleMsTextCombineHorizontal(3i32);
pub const styleMsTextCombineHorizontal_Max: styleMsTextCombineHorizontal = styleMsTextCombineHorizontal(2147483647i32);
impl ::core::marker::Copy for styleMsTextCombineHorizontal {}
impl ::core::clone::Clone for styleMsTextCombineHorizontal {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsTouchAction(pub i32);
pub const styleMsTouchActionNotSet: styleMsTouchAction = styleMsTouchAction(-1i32);
pub const styleMsTouchActionNone: styleMsTouchAction = styleMsTouchAction(0i32);
pub const styleMsTouchActionAuto: styleMsTouchAction = styleMsTouchAction(1i32);
pub const styleMsTouchActionManipulation: styleMsTouchAction = styleMsTouchAction(2i32);
pub const styleMsTouchActionDoubleTapZoom: styleMsTouchAction = styleMsTouchAction(4i32);
pub const styleMsTouchActionPanX: styleMsTouchAction = styleMsTouchAction(8i32);
pub const styleMsTouchActionPanY: styleMsTouchAction = styleMsTouchAction(16i32);
pub const styleMsTouchActionPinchZoom: styleMsTouchAction = styleMsTouchAction(32i32);
pub const styleMsTouchActionCrossSlideX: styleMsTouchAction = styleMsTouchAction(64i32);
pub const styleMsTouchActionCrossSlideY: styleMsTouchAction = styleMsTouchAction(128i32);
pub const styleMsTouchAction_Max: styleMsTouchAction = styleMsTouchAction(2147483647i32);
impl ::core::marker::Copy for styleMsTouchAction {}
impl ::core::clone::Clone for styleMsTouchAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsTouchSelect(pub i32);
pub const styleMsTouchSelectGrippers: styleMsTouchSelect = styleMsTouchSelect(0i32);
pub const styleMsTouchSelectNone: styleMsTouchSelect = styleMsTouchSelect(1i32);
pub const styleMsTouchSelectNotSet: styleMsTouchSelect = styleMsTouchSelect(2i32);
pub const styleMsTouchSelect_Max: styleMsTouchSelect = styleMsTouchSelect(2147483647i32);
impl ::core::marker::Copy for styleMsTouchSelect {}
impl ::core::clone::Clone for styleMsTouchSelect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleMsUserSelect(pub i32);
pub const styleMsUserSelectAuto: styleMsUserSelect = styleMsUserSelect(0i32);
pub const styleMsUserSelectText: styleMsUserSelect = styleMsUserSelect(1i32);
pub const styleMsUserSelectElement: styleMsUserSelect = styleMsUserSelect(2i32);
pub const styleMsUserSelectNone: styleMsUserSelect = styleMsUserSelect(3i32);
pub const styleMsUserSelectNotSet: styleMsUserSelect = styleMsUserSelect(4i32);
pub const styleMsUserSelect_Max: styleMsUserSelect = styleMsUserSelect(2147483647i32);
impl ::core::marker::Copy for styleMsUserSelect {}
impl ::core::clone::Clone for styleMsUserSelect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleNone(pub i32);
pub const styleNoneNone: styleNone = styleNone(0i32);
pub const styleNone_Max: styleNone = styleNone(2147483647i32);
impl ::core::marker::Copy for styleNone {}
impl ::core::clone::Clone for styleNone {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleNormal(pub i32);
pub const styleNormalNormal: styleNormal = styleNormal(0i32);
pub const styleNormal_Max: styleNormal = styleNormal(2147483647i32);
impl ::core::marker::Copy for styleNormal {}
impl ::core::clone::Clone for styleNormal {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleOutlineStyle(pub i32);
pub const styleOutlineStyleNotSet: styleOutlineStyle = styleOutlineStyle(0i32);
pub const styleOutlineStyleDotted: styleOutlineStyle = styleOutlineStyle(1i32);
pub const styleOutlineStyleDashed: styleOutlineStyle = styleOutlineStyle(2i32);
pub const styleOutlineStyleSolid: styleOutlineStyle = styleOutlineStyle(3i32);
pub const styleOutlineStyleDouble: styleOutlineStyle = styleOutlineStyle(4i32);
pub const styleOutlineStyleGroove: styleOutlineStyle = styleOutlineStyle(5i32);
pub const styleOutlineStyleRidge: styleOutlineStyle = styleOutlineStyle(6i32);
pub const styleOutlineStyleInset: styleOutlineStyle = styleOutlineStyle(7i32);
pub const styleOutlineStyleOutset: styleOutlineStyle = styleOutlineStyle(8i32);
pub const styleOutlineStyleWindowInset: styleOutlineStyle = styleOutlineStyle(9i32);
pub const styleOutlineStyleNone: styleOutlineStyle = styleOutlineStyle(10i32);
pub const styleOutlineStyle_Max: styleOutlineStyle = styleOutlineStyle(2147483647i32);
impl ::core::marker::Copy for styleOutlineStyle {}
impl ::core::clone::Clone for styleOutlineStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleOverflow(pub i32);
pub const styleOverflowNotSet: styleOverflow = styleOverflow(0i32);
pub const styleOverflowAuto: styleOverflow = styleOverflow(1i32);
pub const styleOverflowHidden: styleOverflow = styleOverflow(2i32);
pub const styleOverflowVisible: styleOverflow = styleOverflow(3i32);
pub const styleOverflowScroll: styleOverflow = styleOverflow(4i32);
pub const styleOverflow_Max: styleOverflow = styleOverflow(2147483647i32);
impl ::core::marker::Copy for styleOverflow {}
impl ::core::clone::Clone for styleOverflow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct stylePageBreak(pub i32);
pub const stylePageBreakNotSet: stylePageBreak = stylePageBreak(0i32);
pub const stylePageBreakAuto: stylePageBreak = stylePageBreak(1i32);
pub const stylePageBreakAlways: stylePageBreak = stylePageBreak(2i32);
pub const stylePageBreakLeft: stylePageBreak = stylePageBreak(3i32);
pub const stylePageBreakRight: stylePageBreak = stylePageBreak(4i32);
pub const stylePageBreakAvoid: stylePageBreak = stylePageBreak(5i32);
pub const stylePageBreak_Max: stylePageBreak = stylePageBreak(2147483647i32);
impl ::core::marker::Copy for stylePageBreak {}
impl ::core::clone::Clone for stylePageBreak {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct stylePageBreakInside(pub i32);
pub const stylePageBreakInsideNotSet: stylePageBreakInside = stylePageBreakInside(0i32);
pub const stylePageBreakInsideAuto: stylePageBreakInside = stylePageBreakInside(1i32);
pub const stylePageBreakInsideAvoid: stylePageBreakInside = stylePageBreakInside(2i32);
pub const stylePageBreakInside_Max: stylePageBreakInside = stylePageBreakInside(2147483647i32);
impl ::core::marker::Copy for stylePageBreakInside {}
impl ::core::clone::Clone for stylePageBreakInside {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct stylePerspectiveOriginX(pub i32);
pub const stylePerspectiveOriginXNotSet: stylePerspectiveOriginX = stylePerspectiveOriginX(0i32);
pub const stylePerspectiveOriginXLeft: stylePerspectiveOriginX = stylePerspectiveOriginX(1i32);
pub const stylePerspectiveOriginXCenter: stylePerspectiveOriginX = stylePerspectiveOriginX(2i32);
pub const stylePerspectiveOriginXRight: stylePerspectiveOriginX = stylePerspectiveOriginX(3i32);
pub const stylePerspectiveOriginX_Max: stylePerspectiveOriginX = stylePerspectiveOriginX(2147483647i32);
impl ::core::marker::Copy for stylePerspectiveOriginX {}
impl ::core::clone::Clone for stylePerspectiveOriginX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct stylePerspectiveOriginY(pub i32);
pub const stylePerspectiveOriginYNotSet: stylePerspectiveOriginY = stylePerspectiveOriginY(0i32);
pub const stylePerspectiveOriginYTop: stylePerspectiveOriginY = stylePerspectiveOriginY(1i32);
pub const stylePerspectiveOriginYCenter: stylePerspectiveOriginY = stylePerspectiveOriginY(2i32);
pub const stylePerspectiveOriginYBottom: stylePerspectiveOriginY = stylePerspectiveOriginY(3i32);
pub const stylePerspectiveOriginY_Max: stylePerspectiveOriginY = stylePerspectiveOriginY(2147483647i32);
impl ::core::marker::Copy for stylePerspectiveOriginY {}
impl ::core::clone::Clone for stylePerspectiveOriginY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct stylePointerEvents(pub i32);
pub const stylePointerEventsNotSet: stylePointerEvents = stylePointerEvents(0i32);
pub const stylePointerEventsVisiblePainted: stylePointerEvents = stylePointerEvents(1i32);
pub const stylePointerEventsVisibleFill: stylePointerEvents = stylePointerEvents(2i32);
pub const stylePointerEventsVisibleStroke: stylePointerEvents = stylePointerEvents(3i32);
pub const stylePointerEventsVisible: stylePointerEvents = stylePointerEvents(4i32);
pub const stylePointerEventsPainted: stylePointerEvents = stylePointerEvents(5i32);
pub const stylePointerEventsFill: stylePointerEvents = stylePointerEvents(6i32);
pub const stylePointerEventsStroke: stylePointerEvents = stylePointerEvents(7i32);
pub const stylePointerEventsAll: stylePointerEvents = stylePointerEvents(8i32);
pub const stylePointerEventsNone: stylePointerEvents = stylePointerEvents(9i32);
pub const stylePointerEventsInitial: stylePointerEvents = stylePointerEvents(10i32);
pub const stylePointerEventsAuto: stylePointerEvents = stylePointerEvents(11i32);
pub const stylePointerEvents_Max: stylePointerEvents = stylePointerEvents(2147483647i32);
impl ::core::marker::Copy for stylePointerEvents {}
impl ::core::clone::Clone for stylePointerEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct stylePosition(pub i32);
pub const stylePositionNotSet: stylePosition = stylePosition(0i32);
pub const stylePositionstatic: stylePosition = stylePosition(1i32);
pub const stylePositionrelative: stylePosition = stylePosition(2i32);
pub const stylePositionabsolute: stylePosition = stylePosition(3i32);
pub const stylePositionfixed: stylePosition = stylePosition(4i32);
pub const stylePositionMsPage: stylePosition = stylePosition(5i32);
pub const stylePositionMsDeviceFixed: stylePosition = stylePosition(6i32);
pub const stylePosition_Max: stylePosition = stylePosition(2147483647i32);
impl ::core::marker::Copy for stylePosition {}
impl ::core::clone::Clone for stylePosition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleRubyAlign(pub i32);
pub const styleRubyAlignNotSet: styleRubyAlign = styleRubyAlign(0i32);
pub const styleRubyAlignAuto: styleRubyAlign = styleRubyAlign(1i32);
pub const styleRubyAlignLeft: styleRubyAlign = styleRubyAlign(2i32);
pub const styleRubyAlignCenter: styleRubyAlign = styleRubyAlign(3i32);
pub const styleRubyAlignRight: styleRubyAlign = styleRubyAlign(4i32);
pub const styleRubyAlignDistributeLetter: styleRubyAlign = styleRubyAlign(5i32);
pub const styleRubyAlignDistributeSpace: styleRubyAlign = styleRubyAlign(6i32);
pub const styleRubyAlignLineEdge: styleRubyAlign = styleRubyAlign(7i32);
pub const styleRubyAlign_Max: styleRubyAlign = styleRubyAlign(2147483647i32);
impl ::core::marker::Copy for styleRubyAlign {}
impl ::core::clone::Clone for styleRubyAlign {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleRubyOverhang(pub i32);
pub const styleRubyOverhangNotSet: styleRubyOverhang = styleRubyOverhang(0i32);
pub const styleRubyOverhangAuto: styleRubyOverhang = styleRubyOverhang(1i32);
pub const styleRubyOverhangWhitespace: styleRubyOverhang = styleRubyOverhang(2i32);
pub const styleRubyOverhangNone: styleRubyOverhang = styleRubyOverhang(3i32);
pub const styleRubyOverhang_Max: styleRubyOverhang = styleRubyOverhang(2147483647i32);
impl ::core::marker::Copy for styleRubyOverhang {}
impl ::core::clone::Clone for styleRubyOverhang {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleRubyPosition(pub i32);
pub const styleRubyPositionNotSet: styleRubyPosition = styleRubyPosition(0i32);
pub const styleRubyPositionAbove: styleRubyPosition = styleRubyPosition(1i32);
pub const styleRubyPositionInline: styleRubyPosition = styleRubyPosition(2i32);
pub const styleRubyPosition_Max: styleRubyPosition = styleRubyPosition(2147483647i32);
impl ::core::marker::Copy for styleRubyPosition {}
impl ::core::clone::Clone for styleRubyPosition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleStrokeLinecap(pub i32);
pub const styleStrokeLinecapNotSet: styleStrokeLinecap = styleStrokeLinecap(0i32);
pub const styleStrokeLinecapButt: styleStrokeLinecap = styleStrokeLinecap(1i32);
pub const styleStrokeLinecapRound: styleStrokeLinecap = styleStrokeLinecap(2i32);
pub const styleStrokeLinecapSquare: styleStrokeLinecap = styleStrokeLinecap(3i32);
pub const styleStrokeLinecap_Max: styleStrokeLinecap = styleStrokeLinecap(2147483647i32);
impl ::core::marker::Copy for styleStrokeLinecap {}
impl ::core::clone::Clone for styleStrokeLinecap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleStrokeLinejoin(pub i32);
pub const styleStrokeLinejoinNotSet: styleStrokeLinejoin = styleStrokeLinejoin(0i32);
pub const styleStrokeLinejoinMiter: styleStrokeLinejoin = styleStrokeLinejoin(1i32);
pub const styleStrokeLinejoinRound: styleStrokeLinejoin = styleStrokeLinejoin(2i32);
pub const styleStrokeLinejoinBevel: styleStrokeLinejoin = styleStrokeLinejoin(3i32);
pub const styleStrokeLinejoin_Max: styleStrokeLinejoin = styleStrokeLinejoin(2147483647i32);
impl ::core::marker::Copy for styleStrokeLinejoin {}
impl ::core::clone::Clone for styleStrokeLinejoin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleStyleFloat(pub i32);
pub const styleStyleFloatNotSet: styleStyleFloat = styleStyleFloat(0i32);
pub const styleStyleFloatLeft: styleStyleFloat = styleStyleFloat(1i32);
pub const styleStyleFloatRight: styleStyleFloat = styleStyleFloat(2i32);
pub const styleStyleFloatNone: styleStyleFloat = styleStyleFloat(3i32);
pub const styleStyleFloat_Max: styleStyleFloat = styleStyleFloat(2147483647i32);
impl ::core::marker::Copy for styleStyleFloat {}
impl ::core::clone::Clone for styleStyleFloat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleTableLayout(pub i32);
pub const styleTableLayoutNotSet: styleTableLayout = styleTableLayout(0i32);
pub const styleTableLayoutAuto: styleTableLayout = styleTableLayout(1i32);
pub const styleTableLayoutFixed: styleTableLayout = styleTableLayout(2i32);
pub const styleTableLayout_Max: styleTableLayout = styleTableLayout(2147483647i32);
impl ::core::marker::Copy for styleTableLayout {}
impl ::core::clone::Clone for styleTableLayout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleTextAlignLast(pub i32);
pub const styleTextAlignLastNotSet: styleTextAlignLast = styleTextAlignLast(0i32);
pub const styleTextAlignLastLeft: styleTextAlignLast = styleTextAlignLast(1i32);
pub const styleTextAlignLastCenter: styleTextAlignLast = styleTextAlignLast(2i32);
pub const styleTextAlignLastRight: styleTextAlignLast = styleTextAlignLast(3i32);
pub const styleTextAlignLastJustify: styleTextAlignLast = styleTextAlignLast(4i32);
pub const styleTextAlignLastAuto: styleTextAlignLast = styleTextAlignLast(5i32);
pub const styleTextAlignLast_Max: styleTextAlignLast = styleTextAlignLast(2147483647i32);
impl ::core::marker::Copy for styleTextAlignLast {}
impl ::core::clone::Clone for styleTextAlignLast {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleTextAnchor(pub i32);
pub const styleTextAnchorNotSet: styleTextAnchor = styleTextAnchor(0i32);
pub const styleTextAnchorStart: styleTextAnchor = styleTextAnchor(1i32);
pub const styleTextAnchorMiddle: styleTextAnchor = styleTextAnchor(2i32);
pub const styleTextAnchorEnd: styleTextAnchor = styleTextAnchor(3i32);
pub const styleTextAnchor_Max: styleTextAnchor = styleTextAnchor(2147483647i32);
impl ::core::marker::Copy for styleTextAnchor {}
impl ::core::clone::Clone for styleTextAnchor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleTextDecoration(pub i32);
pub const styleTextDecorationNone: styleTextDecoration = styleTextDecoration(0i32);
pub const styleTextDecorationUnderline: styleTextDecoration = styleTextDecoration(1i32);
pub const styleTextDecorationOverline: styleTextDecoration = styleTextDecoration(2i32);
pub const styleTextDecorationLineThrough: styleTextDecoration = styleTextDecoration(3i32);
pub const styleTextDecorationBlink: styleTextDecoration = styleTextDecoration(4i32);
pub const styleTextDecoration_Max: styleTextDecoration = styleTextDecoration(2147483647i32);
impl ::core::marker::Copy for styleTextDecoration {}
impl ::core::clone::Clone for styleTextDecoration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleTextEffect(pub i32);
pub const styleTextEffectNone: styleTextEffect = styleTextEffect(0i32);
pub const styleTextEffectEmboss: styleTextEffect = styleTextEffect(1i32);
pub const styleTextEffectEngrave: styleTextEffect = styleTextEffect(2i32);
pub const styleTextEffectOutline: styleTextEffect = styleTextEffect(3i32);
pub const styleTextEffect_Max: styleTextEffect = styleTextEffect(2147483647i32);
impl ::core::marker::Copy for styleTextEffect {}
impl ::core::clone::Clone for styleTextEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleTextJustify(pub i32);
pub const styleTextJustifyNotSet: styleTextJustify = styleTextJustify(0i32);
pub const styleTextJustifyInterWord: styleTextJustify = styleTextJustify(1i32);
pub const styleTextJustifyNewspaper: styleTextJustify = styleTextJustify(2i32);
pub const styleTextJustifyDistribute: styleTextJustify = styleTextJustify(3i32);
pub const styleTextJustifyDistributeAllLines: styleTextJustify = styleTextJustify(4i32);
pub const styleTextJustifyInterIdeograph: styleTextJustify = styleTextJustify(5i32);
pub const styleTextJustifyInterCluster: styleTextJustify = styleTextJustify(6i32);
pub const styleTextJustifyKashida: styleTextJustify = styleTextJustify(7i32);
pub const styleTextJustifyAuto: styleTextJustify = styleTextJustify(8i32);
pub const styleTextJustify_Max: styleTextJustify = styleTextJustify(2147483647i32);
impl ::core::marker::Copy for styleTextJustify {}
impl ::core::clone::Clone for styleTextJustify {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleTextJustifyTrim(pub i32);
pub const styleTextJustifyTrimNotSet: styleTextJustifyTrim = styleTextJustifyTrim(0i32);
pub const styleTextJustifyTrimNone: styleTextJustifyTrim = styleTextJustifyTrim(1i32);
pub const styleTextJustifyTrimPunctuation: styleTextJustifyTrim = styleTextJustifyTrim(2i32);
pub const styleTextJustifyTrimPunctAndKana: styleTextJustifyTrim = styleTextJustifyTrim(3i32);
pub const styleTextJustifyTrim_Max: styleTextJustifyTrim = styleTextJustifyTrim(2147483647i32);
impl ::core::marker::Copy for styleTextJustifyTrim {}
impl ::core::clone::Clone for styleTextJustifyTrim {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleTextLineThroughStyle(pub i32);
pub const styleTextLineThroughStyleUndefined: styleTextLineThroughStyle = styleTextLineThroughStyle(0i32);
pub const styleTextLineThroughStyleSingle: styleTextLineThroughStyle = styleTextLineThroughStyle(1i32);
pub const styleTextLineThroughStyleDouble: styleTextLineThroughStyle = styleTextLineThroughStyle(2i32);
pub const styleTextLineThroughStyle_Max: styleTextLineThroughStyle = styleTextLineThroughStyle(2147483647i32);
impl ::core::marker::Copy for styleTextLineThroughStyle {}
impl ::core::clone::Clone for styleTextLineThroughStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleTextOverflow(pub i32);
pub const styleTextOverflowClip: styleTextOverflow = styleTextOverflow(0i32);
pub const styleTextOverflowEllipsis: styleTextOverflow = styleTextOverflow(1i32);
pub const styleTextOverflowNotSet: styleTextOverflow = styleTextOverflow(2i32);
pub const styleTextOverflow_Max: styleTextOverflow = styleTextOverflow(2147483647i32);
impl ::core::marker::Copy for styleTextOverflow {}
impl ::core::clone::Clone for styleTextOverflow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleTextSizeAdjust(pub i32);
pub const styleTextSizeAdjustNone: styleTextSizeAdjust = styleTextSizeAdjust(0i32);
pub const styleTextSizeAdjustAuto: styleTextSizeAdjust = styleTextSizeAdjust(1i32);
pub const styleTextSizeAdjust_Max: styleTextSizeAdjust = styleTextSizeAdjust(2147483647i32);
impl ::core::marker::Copy for styleTextSizeAdjust {}
impl ::core::clone::Clone for styleTextSizeAdjust {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleTextTransform(pub i32);
pub const styleTextTransformNotSet: styleTextTransform = styleTextTransform(0i32);
pub const styleTextTransformCapitalize: styleTextTransform = styleTextTransform(1i32);
pub const styleTextTransformLowercase: styleTextTransform = styleTextTransform(2i32);
pub const styleTextTransformUppercase: styleTextTransform = styleTextTransform(3i32);
pub const styleTextTransformNone: styleTextTransform = styleTextTransform(4i32);
pub const styleTextTransform_Max: styleTextTransform = styleTextTransform(2147483647i32);
impl ::core::marker::Copy for styleTextTransform {}
impl ::core::clone::Clone for styleTextTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleTextUnderlinePosition(pub i32);
pub const styleTextUnderlinePositionBelow: styleTextUnderlinePosition = styleTextUnderlinePosition(0i32);
pub const styleTextUnderlinePositionAbove: styleTextUnderlinePosition = styleTextUnderlinePosition(1i32);
pub const styleTextUnderlinePositionAuto: styleTextUnderlinePosition = styleTextUnderlinePosition(2i32);
pub const styleTextUnderlinePositionNotSet: styleTextUnderlinePosition = styleTextUnderlinePosition(3i32);
pub const styleTextUnderlinePosition_Max: styleTextUnderlinePosition = styleTextUnderlinePosition(2147483647i32);
impl ::core::marker::Copy for styleTextUnderlinePosition {}
impl ::core::clone::Clone for styleTextUnderlinePosition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleTextUnderlineStyle(pub i32);
pub const styleTextUnderlineStyleUndefined: styleTextUnderlineStyle = styleTextUnderlineStyle(0i32);
pub const styleTextUnderlineStyleSingle: styleTextUnderlineStyle = styleTextUnderlineStyle(1i32);
pub const styleTextUnderlineStyleDouble: styleTextUnderlineStyle = styleTextUnderlineStyle(2i32);
pub const styleTextUnderlineStyleWords: styleTextUnderlineStyle = styleTextUnderlineStyle(3i32);
pub const styleTextUnderlineStyleDotted: styleTextUnderlineStyle = styleTextUnderlineStyle(4i32);
pub const styleTextUnderlineStyleThick: styleTextUnderlineStyle = styleTextUnderlineStyle(5i32);
pub const styleTextUnderlineStyleDash: styleTextUnderlineStyle = styleTextUnderlineStyle(6i32);
pub const styleTextUnderlineStyleDotDash: styleTextUnderlineStyle = styleTextUnderlineStyle(7i32);
pub const styleTextUnderlineStyleDotDotDash: styleTextUnderlineStyle = styleTextUnderlineStyle(8i32);
pub const styleTextUnderlineStyleWave: styleTextUnderlineStyle = styleTextUnderlineStyle(9i32);
pub const styleTextUnderlineStyleSingleAccounting: styleTextUnderlineStyle = styleTextUnderlineStyle(10i32);
pub const styleTextUnderlineStyleDoubleAccounting: styleTextUnderlineStyle = styleTextUnderlineStyle(11i32);
pub const styleTextUnderlineStyleThickDash: styleTextUnderlineStyle = styleTextUnderlineStyle(12i32);
pub const styleTextUnderlineStyle_Max: styleTextUnderlineStyle = styleTextUnderlineStyle(2147483647i32);
impl ::core::marker::Copy for styleTextUnderlineStyle {}
impl ::core::clone::Clone for styleTextUnderlineStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleTransformOriginX(pub i32);
pub const styleTransformOriginXNotSet: styleTransformOriginX = styleTransformOriginX(0i32);
pub const styleTransformOriginXLeft: styleTransformOriginX = styleTransformOriginX(1i32);
pub const styleTransformOriginXCenter: styleTransformOriginX = styleTransformOriginX(2i32);
pub const styleTransformOriginXRight: styleTransformOriginX = styleTransformOriginX(3i32);
pub const styleTransformOriginX_Max: styleTransformOriginX = styleTransformOriginX(2147483647i32);
impl ::core::marker::Copy for styleTransformOriginX {}
impl ::core::clone::Clone for styleTransformOriginX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleTransformOriginY(pub i32);
pub const styleTransformOriginYNotSet: styleTransformOriginY = styleTransformOriginY(0i32);
pub const styleTransformOriginYTop: styleTransformOriginY = styleTransformOriginY(1i32);
pub const styleTransformOriginYCenter: styleTransformOriginY = styleTransformOriginY(2i32);
pub const styleTransformOriginYBottom: styleTransformOriginY = styleTransformOriginY(3i32);
pub const styleTransformOriginY_Max: styleTransformOriginY = styleTransformOriginY(2147483647i32);
impl ::core::marker::Copy for styleTransformOriginY {}
impl ::core::clone::Clone for styleTransformOriginY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleTransformStyle(pub i32);
pub const styleTransformStyleFlat: styleTransformStyle = styleTransformStyle(0i32);
pub const styleTransformStylePreserve3D: styleTransformStyle = styleTransformStyle(1i32);
pub const styleTransformStyleNotSet: styleTransformStyle = styleTransformStyle(2i32);
pub const styleTransformStyle_Max: styleTransformStyle = styleTransformStyle(2147483647i32);
impl ::core::marker::Copy for styleTransformStyle {}
impl ::core::clone::Clone for styleTransformStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleUserZoom(pub i32);
pub const styleUserZoomNotSet: styleUserZoom = styleUserZoom(0i32);
pub const styleUserZoomZoom: styleUserZoom = styleUserZoom(1i32);
pub const styleUserZoomFixed: styleUserZoom = styleUserZoom(2i32);
pub const styleUserZoom_Max: styleUserZoom = styleUserZoom(2147483647i32);
impl ::core::marker::Copy for styleUserZoom {}
impl ::core::clone::Clone for styleUserZoom {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleVerticalAlign(pub i32);
pub const styleVerticalAlignAuto: styleVerticalAlign = styleVerticalAlign(0i32);
pub const styleVerticalAlignBaseline: styleVerticalAlign = styleVerticalAlign(1i32);
pub const styleVerticalAlignSub: styleVerticalAlign = styleVerticalAlign(2i32);
pub const styleVerticalAlignSuper: styleVerticalAlign = styleVerticalAlign(3i32);
pub const styleVerticalAlignTop: styleVerticalAlign = styleVerticalAlign(4i32);
pub const styleVerticalAlignTextTop: styleVerticalAlign = styleVerticalAlign(5i32);
pub const styleVerticalAlignMiddle: styleVerticalAlign = styleVerticalAlign(6i32);
pub const styleVerticalAlignBottom: styleVerticalAlign = styleVerticalAlign(7i32);
pub const styleVerticalAlignTextBottom: styleVerticalAlign = styleVerticalAlign(8i32);
pub const styleVerticalAlignInherit: styleVerticalAlign = styleVerticalAlign(9i32);
pub const styleVerticalAlignNotSet: styleVerticalAlign = styleVerticalAlign(10i32);
pub const styleVerticalAlign_Max: styleVerticalAlign = styleVerticalAlign(2147483647i32);
impl ::core::marker::Copy for styleVerticalAlign {}
impl ::core::clone::Clone for styleVerticalAlign {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleViewportSize(pub i32);
pub const styleViewportSizeAuto: styleViewportSize = styleViewportSize(0i32);
pub const styleViewportSizeDeviceWidth: styleViewportSize = styleViewportSize(1i32);
pub const styleViewportSizeDeviceHeight: styleViewportSize = styleViewportSize(2i32);
pub const styleViewportSize_Max: styleViewportSize = styleViewportSize(2147483647i32);
impl ::core::marker::Copy for styleViewportSize {}
impl ::core::clone::Clone for styleViewportSize {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleVisibility(pub i32);
pub const styleVisibilityNotSet: styleVisibility = styleVisibility(0i32);
pub const styleVisibilityInherit: styleVisibility = styleVisibility(1i32);
pub const styleVisibilityVisible: styleVisibility = styleVisibility(2i32);
pub const styleVisibilityHidden: styleVisibility = styleVisibility(3i32);
pub const styleVisibilityCollapse: styleVisibility = styleVisibility(4i32);
pub const styleVisibility_Max: styleVisibility = styleVisibility(2147483647i32);
impl ::core::marker::Copy for styleVisibility {}
impl ::core::clone::Clone for styleVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleWebkitAppearance(pub i32);
pub const styleWebkitAppearanceNone: styleWebkitAppearance = styleWebkitAppearance(0i32);
pub const styleWebkitAppearanceCapsLockIndicator: styleWebkitAppearance = styleWebkitAppearance(1i32);
pub const styleWebkitAppearanceButton: styleWebkitAppearance = styleWebkitAppearance(2i32);
pub const styleWebkitAppearanceButtonBevel: styleWebkitAppearance = styleWebkitAppearance(3i32);
pub const styleWebkitAppearanceCaret: styleWebkitAppearance = styleWebkitAppearance(4i32);
pub const styleWebkitAppearanceCheckbox: styleWebkitAppearance = styleWebkitAppearance(5i32);
pub const styleWebkitAppearanceDefaultButton: styleWebkitAppearance = styleWebkitAppearance(6i32);
pub const styleWebkitAppearanceListbox: styleWebkitAppearance = styleWebkitAppearance(7i32);
pub const styleWebkitAppearanceListitem: styleWebkitAppearance = styleWebkitAppearance(8i32);
pub const styleWebkitAppearanceMediaFullscreenButton: styleWebkitAppearance = styleWebkitAppearance(9i32);
pub const styleWebkitAppearanceMediaMuteButton: styleWebkitAppearance = styleWebkitAppearance(10i32);
pub const styleWebkitAppearanceMediaPlayButton: styleWebkitAppearance = styleWebkitAppearance(11i32);
pub const styleWebkitAppearanceMediaSeekBackButton: styleWebkitAppearance = styleWebkitAppearance(12i32);
pub const styleWebkitAppearanceMediaSeekForwardButton: styleWebkitAppearance = styleWebkitAppearance(13i32);
pub const styleWebkitAppearanceMediaSlider: styleWebkitAppearance = styleWebkitAppearance(14i32);
pub const styleWebkitAppearanceMediaSliderthumb: styleWebkitAppearance = styleWebkitAppearance(15i32);
pub const styleWebkitAppearanceMenulist: styleWebkitAppearance = styleWebkitAppearance(16i32);
pub const styleWebkitAppearanceMenulistButton: styleWebkitAppearance = styleWebkitAppearance(17i32);
pub const styleWebkitAppearanceMenulistText: styleWebkitAppearance = styleWebkitAppearance(18i32);
pub const styleWebkitAppearanceMenulistTextfield: styleWebkitAppearance = styleWebkitAppearance(19i32);
pub const styleWebkitAppearancePushButton: styleWebkitAppearance = styleWebkitAppearance(20i32);
pub const styleWebkitAppearanceRadio: styleWebkitAppearance = styleWebkitAppearance(21i32);
pub const styleWebkitAppearanceSearchfield: styleWebkitAppearance = styleWebkitAppearance(22i32);
pub const styleWebkitAppearanceSearchfieldCancelButton: styleWebkitAppearance = styleWebkitAppearance(23i32);
pub const styleWebkitAppearanceSearchfieldDecoration: styleWebkitAppearance = styleWebkitAppearance(24i32);
pub const styleWebkitAppearanceSearchfieldResultsButton: styleWebkitAppearance = styleWebkitAppearance(25i32);
pub const styleWebkitAppearanceSearchfieldResultsDecoration: styleWebkitAppearance = styleWebkitAppearance(26i32);
pub const styleWebkitAppearanceSliderHorizontal: styleWebkitAppearance = styleWebkitAppearance(27i32);
pub const styleWebkitAppearanceSliderVertical: styleWebkitAppearance = styleWebkitAppearance(28i32);
pub const styleWebkitAppearanceSliderthumbHorizontal: styleWebkitAppearance = styleWebkitAppearance(29i32);
pub const styleWebkitAppearanceSliderthumbVertical: styleWebkitAppearance = styleWebkitAppearance(30i32);
pub const styleWebkitAppearanceSquareButton: styleWebkitAppearance = styleWebkitAppearance(31i32);
pub const styleWebkitAppearanceTextarea: styleWebkitAppearance = styleWebkitAppearance(32i32);
pub const styleWebkitAppearanceTextfield: styleWebkitAppearance = styleWebkitAppearance(33i32);
pub const styleWebkitAppearanceNotSet: styleWebkitAppearance = styleWebkitAppearance(34i32);
pub const styleWebkitAppearance_Max: styleWebkitAppearance = styleWebkitAppearance(2147483647i32);
impl ::core::marker::Copy for styleWebkitAppearance {}
impl ::core::clone::Clone for styleWebkitAppearance {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleWebkitBoxDirection(pub i32);
pub const styleWebkitBoxDirectionNormal: styleWebkitBoxDirection = styleWebkitBoxDirection(0i32);
pub const styleWebkitBoxDirectionReverse: styleWebkitBoxDirection = styleWebkitBoxDirection(1i32);
pub const styleWebkitBoxDirectionNotSet: styleWebkitBoxDirection = styleWebkitBoxDirection(2i32);
pub const styleWebkitBoxDirection_Max: styleWebkitBoxDirection = styleWebkitBoxDirection(2147483647i32);
impl ::core::marker::Copy for styleWebkitBoxDirection {}
impl ::core::clone::Clone for styleWebkitBoxDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleWebkitBoxOrient(pub i32);
pub const styleWebkitBoxOrientHorizontal: styleWebkitBoxOrient = styleWebkitBoxOrient(0i32);
pub const styleWebkitBoxOrientInlineAxis: styleWebkitBoxOrient = styleWebkitBoxOrient(1i32);
pub const styleWebkitBoxOrientVertical: styleWebkitBoxOrient = styleWebkitBoxOrient(2i32);
pub const styleWebkitBoxOrientBlockAxis: styleWebkitBoxOrient = styleWebkitBoxOrient(3i32);
pub const styleWebkitBoxOrientNotSet: styleWebkitBoxOrient = styleWebkitBoxOrient(4i32);
pub const styleWebkitBoxOrient_Max: styleWebkitBoxOrient = styleWebkitBoxOrient(2147483647i32);
impl ::core::marker::Copy for styleWebkitBoxOrient {}
impl ::core::clone::Clone for styleWebkitBoxOrient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleWebkitBoxPack(pub i32);
pub const styleWebkitBoxPackStart: styleWebkitBoxPack = styleWebkitBoxPack(0i32);
pub const styleWebkitBoxPackEnd: styleWebkitBoxPack = styleWebkitBoxPack(1i32);
pub const styleWebkitBoxPackCenter: styleWebkitBoxPack = styleWebkitBoxPack(2i32);
pub const styleWebkitBoxPackJustify: styleWebkitBoxPack = styleWebkitBoxPack(3i32);
pub const styleWebkitBoxPackNotSet: styleWebkitBoxPack = styleWebkitBoxPack(5i32);
pub const styleWebkitBoxPack_Max: styleWebkitBoxPack = styleWebkitBoxPack(2147483647i32);
impl ::core::marker::Copy for styleWebkitBoxPack {}
impl ::core::clone::Clone for styleWebkitBoxPack {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleWhiteSpace(pub i32);
pub const styleWhiteSpaceNotSet: styleWhiteSpace = styleWhiteSpace(0i32);
pub const styleWhiteSpaceNormal: styleWhiteSpace = styleWhiteSpace(1i32);
pub const styleWhiteSpacePre: styleWhiteSpace = styleWhiteSpace(2i32);
pub const styleWhiteSpaceNowrap: styleWhiteSpace = styleWhiteSpace(3i32);
pub const styleWhiteSpacePreline: styleWhiteSpace = styleWhiteSpace(4i32);
pub const styleWhiteSpacePrewrap: styleWhiteSpace = styleWhiteSpace(5i32);
pub const styleWhiteSpace_Max: styleWhiteSpace = styleWhiteSpace(2147483647i32);
impl ::core::marker::Copy for styleWhiteSpace {}
impl ::core::clone::Clone for styleWhiteSpace {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleWidowsOrphans(pub i32);
pub const styleWidowsOrphansNotSet: styleWidowsOrphans = styleWidowsOrphans(-2147483647i32);
pub const styleWidowsOrphans_Max: styleWidowsOrphans = styleWidowsOrphans(2147483647i32);
impl ::core::marker::Copy for styleWidowsOrphans {}
impl ::core::clone::Clone for styleWidowsOrphans {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleWordBreak(pub i32);
pub const styleWordBreakNotSet: styleWordBreak = styleWordBreak(0i32);
pub const styleWordBreakNormal: styleWordBreak = styleWordBreak(1i32);
pub const styleWordBreakBreakAll: styleWordBreak = styleWordBreak(2i32);
pub const styleWordBreakKeepAll: styleWordBreak = styleWordBreak(3i32);
pub const styleWordBreak_Max: styleWordBreak = styleWordBreak(2147483647i32);
impl ::core::marker::Copy for styleWordBreak {}
impl ::core::clone::Clone for styleWordBreak {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleWordWrap(pub i32);
pub const styleWordWrapNotSet: styleWordWrap = styleWordWrap(0i32);
pub const styleWordWrapOff: styleWordWrap = styleWordWrap(1i32);
pub const styleWordWrapOn: styleWordWrap = styleWordWrap(2i32);
pub const styleWordWrap_Max: styleWordWrap = styleWordWrap(2147483647i32);
impl ::core::marker::Copy for styleWordWrap {}
impl ::core::clone::Clone for styleWordWrap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleWrapFlow(pub i32);
pub const styleWrapFlowNotSet: styleWrapFlow = styleWrapFlow(0i32);
pub const styleWrapFlowAuto: styleWrapFlow = styleWrapFlow(1i32);
pub const styleWrapFlowBoth: styleWrapFlow = styleWrapFlow(2i32);
pub const styleWrapFlowStart: styleWrapFlow = styleWrapFlow(3i32);
pub const styleWrapFlowEnd: styleWrapFlow = styleWrapFlow(4i32);
pub const styleWrapFlowClear: styleWrapFlow = styleWrapFlow(5i32);
pub const styleWrapFlowMinimum: styleWrapFlow = styleWrapFlow(6i32);
pub const styleWrapFlowMaximum: styleWrapFlow = styleWrapFlow(7i32);
pub const styleWrapFlow_Max: styleWrapFlow = styleWrapFlow(2147483647i32);
impl ::core::marker::Copy for styleWrapFlow {}
impl ::core::clone::Clone for styleWrapFlow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleWrapThrough(pub i32);
pub const styleWrapThroughNotSet: styleWrapThrough = styleWrapThrough(0i32);
pub const styleWrapThroughWrap: styleWrapThrough = styleWrapThrough(1i32);
pub const styleWrapThroughNone: styleWrapThrough = styleWrapThrough(2i32);
pub const styleWrapThrough_Max: styleWrapThrough = styleWrapThrough(2147483647i32);
impl ::core::marker::Copy for styleWrapThrough {}
impl ::core::clone::Clone for styleWrapThrough {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleWritingMode(pub i32);
pub const styleWritingModeLrtb: styleWritingMode = styleWritingMode(0i32);
pub const styleWritingModeTbrl: styleWritingMode = styleWritingMode(1i32);
pub const styleWritingModeRltb: styleWritingMode = styleWritingMode(2i32);
pub const styleWritingModeBtrl: styleWritingMode = styleWritingMode(3i32);
pub const styleWritingModeNotSet: styleWritingMode = styleWritingMode(4i32);
pub const styleWritingModeTblr: styleWritingMode = styleWritingMode(5i32);
pub const styleWritingModeBtlr: styleWritingMode = styleWritingMode(6i32);
pub const styleWritingModeLrbt: styleWritingMode = styleWritingMode(7i32);
pub const styleWritingModeRlbt: styleWritingMode = styleWritingMode(8i32);
pub const styleWritingModeLr: styleWritingMode = styleWritingMode(9i32);
pub const styleWritingModeRl: styleWritingMode = styleWritingMode(10i32);
pub const styleWritingModeTb: styleWritingMode = styleWritingMode(11i32);
pub const styleWritingMode_Max: styleWritingMode = styleWritingMode(2147483647i32);
impl ::core::marker::Copy for styleWritingMode {}
impl ::core::clone::Clone for styleWritingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct styleZIndex(pub i32);
pub const styleZIndexAuto: styleZIndex = styleZIndex(-2147483647i32);
pub const styleZIndex_Max: styleZIndex = styleZIndex(2147483647i32);
impl ::core::marker::Copy for styleZIndex {}
impl ::core::clone::Clone for styleZIndex {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgAngleType(pub i32);
pub const SVG_ANGLETYPE_UNKNOWN: svgAngleType = svgAngleType(0i32);
pub const SVG_ANGLETYPE_UNSPECIFIED: svgAngleType = svgAngleType(1i32);
pub const SVG_ANGLETYPE_DEG: svgAngleType = svgAngleType(2i32);
pub const SVG_ANGLETYPE_RAD: svgAngleType = svgAngleType(3i32);
pub const SVG_ANGLETYPE_GRAD: svgAngleType = svgAngleType(4i32);
pub const svgAngleType_Max: svgAngleType = svgAngleType(2147483647i32);
impl ::core::marker::Copy for svgAngleType {}
impl ::core::clone::Clone for svgAngleType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgChannel(pub i32);
pub const SVG_CHANNEL_UNKNOWN: svgChannel = svgChannel(0i32);
pub const SVG_CHANNEL_R: svgChannel = svgChannel(1i32);
pub const SVG_CHANNEL_G: svgChannel = svgChannel(2i32);
pub const SVG_CHANNEL_B: svgChannel = svgChannel(3i32);
pub const SVG_CHANNEL_A: svgChannel = svgChannel(4i32);
pub const svgChannel_Max: svgChannel = svgChannel(2147483647i32);
impl ::core::marker::Copy for svgChannel {}
impl ::core::clone::Clone for svgChannel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgEdgemode(pub i32);
pub const SVG_EDGEMODE_UNKNOWN: svgEdgemode = svgEdgemode(0i32);
pub const SVG_EDGEMODE_DUPLICATE: svgEdgemode = svgEdgemode(1i32);
pub const SVG_EDGEMODE_WRAP: svgEdgemode = svgEdgemode(2i32);
pub const SVG_EDGEMODE_NONE: svgEdgemode = svgEdgemode(3i32);
pub const svgEdgemode_Max: svgEdgemode = svgEdgemode(2147483647i32);
impl ::core::marker::Copy for svgEdgemode {}
impl ::core::clone::Clone for svgEdgemode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgExternalResourcesRequired(pub i32);
pub const svgExternalResourcesRequiredFalse: svgExternalResourcesRequired = svgExternalResourcesRequired(0i32);
pub const svgExternalResourcesRequiredTrue: svgExternalResourcesRequired = svgExternalResourcesRequired(1i32);
pub const svgExternalResourcesRequired_Max: svgExternalResourcesRequired = svgExternalResourcesRequired(2147483647i32);
impl ::core::marker::Copy for svgExternalResourcesRequired {}
impl ::core::clone::Clone for svgExternalResourcesRequired {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgFeblendMode(pub i32);
pub const SVG_FEBLEND_MODE_UNKNOWN: svgFeblendMode = svgFeblendMode(0i32);
pub const SVG_FEBLEND_MODE_NORMAL: svgFeblendMode = svgFeblendMode(1i32);
pub const SVG_FEBLEND_MODE_MULTIPLY: svgFeblendMode = svgFeblendMode(2i32);
pub const SVG_FEBLEND_MODE_SCREEN: svgFeblendMode = svgFeblendMode(3i32);
pub const SVG_FEBLEND_MODE_DARKEN: svgFeblendMode = svgFeblendMode(4i32);
pub const SVG_FEBLEND_MODE_LIGHTEN: svgFeblendMode = svgFeblendMode(5i32);
pub const svgFeblendMode_Max: svgFeblendMode = svgFeblendMode(2147483647i32);
impl ::core::marker::Copy for svgFeblendMode {}
impl ::core::clone::Clone for svgFeblendMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgFecolormatrixType(pub i32);
pub const SVG_FECOLORMATRIX_TYPE_UNKNOWN: svgFecolormatrixType = svgFecolormatrixType(0i32);
pub const SVG_FECOLORMATRIX_TYPE_MATRIX: svgFecolormatrixType = svgFecolormatrixType(1i32);
pub const SVG_FECOLORMATRIX_TYPE_SATURATE: svgFecolormatrixType = svgFecolormatrixType(2i32);
pub const SVG_FECOLORMATRIX_TYPE_HUEROTATE: svgFecolormatrixType = svgFecolormatrixType(3i32);
pub const SVG_FECOLORMATRIX_TYPE_LUMINANCETOALPHA: svgFecolormatrixType = svgFecolormatrixType(4i32);
pub const svgFecolormatrixType_Max: svgFecolormatrixType = svgFecolormatrixType(2147483647i32);
impl ::core::marker::Copy for svgFecolormatrixType {}
impl ::core::clone::Clone for svgFecolormatrixType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgFecomponenttransferType(pub i32);
pub const SVG_FECOMPONENTTRANSFER_TYPE_UNKNOWN: svgFecomponenttransferType = svgFecomponenttransferType(0i32);
pub const SVG_FECOMPONENTTRANSFER_TYPE_IDENTITY: svgFecomponenttransferType = svgFecomponenttransferType(1i32);
pub const SVG_FECOMPONENTTRANSFER_TYPE_TABLE: svgFecomponenttransferType = svgFecomponenttransferType(2i32);
pub const SVG_FECOMPONENTTRANSFER_TYPE_DISCRETE: svgFecomponenttransferType = svgFecomponenttransferType(3i32);
pub const SVG_FECOMPONENTTRANSFER_TYPE_LINEAR: svgFecomponenttransferType = svgFecomponenttransferType(4i32);
pub const SVG_FECOMPONENTTRANSFER_TYPE_GAMMA: svgFecomponenttransferType = svgFecomponenttransferType(5i32);
pub const svgFecomponenttransferType_Max: svgFecomponenttransferType = svgFecomponenttransferType(2147483647i32);
impl ::core::marker::Copy for svgFecomponenttransferType {}
impl ::core::clone::Clone for svgFecomponenttransferType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgFecompositeOperator(pub i32);
pub const SVG_FECOMPOSITE_OPERATOR_UNKNOWN: svgFecompositeOperator = svgFecompositeOperator(0i32);
pub const SVG_FECOMPOSITE_OPERATOR_OVER: svgFecompositeOperator = svgFecompositeOperator(1i32);
pub const SVG_FECOMPOSITE_OPERATOR_IN: svgFecompositeOperator = svgFecompositeOperator(2i32);
pub const SVG_FECOMPOSITE_OPERATOR_OUT: svgFecompositeOperator = svgFecompositeOperator(3i32);
pub const SVG_FECOMPOSITE_OPERATOR_ATOP: svgFecompositeOperator = svgFecompositeOperator(4i32);
pub const SVG_FECOMPOSITE_OPERATOR_XOR: svgFecompositeOperator = svgFecompositeOperator(5i32);
pub const SVG_FECOMPOSITE_OPERATOR_ARITHMETIC: svgFecompositeOperator = svgFecompositeOperator(6i32);
pub const svgFecompositeOperator_Max: svgFecompositeOperator = svgFecompositeOperator(2147483647i32);
impl ::core::marker::Copy for svgFecompositeOperator {}
impl ::core::clone::Clone for svgFecompositeOperator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgFocusable(pub i32);
pub const svgFocusableNotSet: svgFocusable = svgFocusable(0i32);
pub const svgFocusableAuto: svgFocusable = svgFocusable(1i32);
pub const svgFocusableTrue: svgFocusable = svgFocusable(2i32);
pub const svgFocusableFalse: svgFocusable = svgFocusable(3i32);
pub const svgFocusable_Max: svgFocusable = svgFocusable(2147483647i32);
impl ::core::marker::Copy for svgFocusable {}
impl ::core::clone::Clone for svgFocusable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgLengthType(pub i32);
pub const SVG_LENGTHTYPE_UNKNOWN: svgLengthType = svgLengthType(0i32);
pub const SVG_LENGTHTYPE_NUMBER: svgLengthType = svgLengthType(1i32);
pub const SVG_LENGTHTYPE_PERCENTAGE: svgLengthType = svgLengthType(2i32);
pub const SVG_LENGTHTYPE_EMS: svgLengthType = svgLengthType(3i32);
pub const SVG_LENGTHTYPE_EXS: svgLengthType = svgLengthType(4i32);
pub const SVG_LENGTHTYPE_PX: svgLengthType = svgLengthType(5i32);
pub const SVG_LENGTHTYPE_CM: svgLengthType = svgLengthType(6i32);
pub const SVG_LENGTHTYPE_MM: svgLengthType = svgLengthType(7i32);
pub const SVG_LENGTHTYPE_IN: svgLengthType = svgLengthType(8i32);
pub const SVG_LENGTHTYPE_PT: svgLengthType = svgLengthType(9i32);
pub const SVG_LENGTHTYPE_PC: svgLengthType = svgLengthType(10i32);
pub const svgLengthType_Max: svgLengthType = svgLengthType(2147483647i32);
impl ::core::marker::Copy for svgLengthType {}
impl ::core::clone::Clone for svgLengthType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgMarkerOrient(pub i32);
pub const SVG_MARKER_ORIENT_UNKNOWN: svgMarkerOrient = svgMarkerOrient(0i32);
pub const SVG_MARKER_ORIENT_AUTO: svgMarkerOrient = svgMarkerOrient(1i32);
pub const SVG_MARKER_ORIENT_ANGLE: svgMarkerOrient = svgMarkerOrient(2i32);
pub const svgMarkerOrient_Max: svgMarkerOrient = svgMarkerOrient(2147483647i32);
impl ::core::marker::Copy for svgMarkerOrient {}
impl ::core::clone::Clone for svgMarkerOrient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgMarkerOrientAttribute(pub i32);
pub const svgMarkerOrientAttributeAuto: svgMarkerOrientAttribute = svgMarkerOrientAttribute(0i32);
pub const svgMarkerOrientAttribute_Max: svgMarkerOrientAttribute = svgMarkerOrientAttribute(2147483647i32);
impl ::core::marker::Copy for svgMarkerOrientAttribute {}
impl ::core::clone::Clone for svgMarkerOrientAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgMarkerUnits(pub i32);
pub const SVG_MARKERUNITS_UNKNOWN: svgMarkerUnits = svgMarkerUnits(0i32);
pub const SVG_MARKERUNITS_USERSPACEONUSE: svgMarkerUnits = svgMarkerUnits(1i32);
pub const SVG_MARKERUNITS_STROKEWIDTH: svgMarkerUnits = svgMarkerUnits(2i32);
pub const svgMarkerUnits_Max: svgMarkerUnits = svgMarkerUnits(2147483647i32);
impl ::core::marker::Copy for svgMarkerUnits {}
impl ::core::clone::Clone for svgMarkerUnits {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgMorphologyOperator(pub i32);
pub const SVG_MORPHOLOGY_OPERATOR_UNKNOWN: svgMorphologyOperator = svgMorphologyOperator(0i32);
pub const SVG_MORPHOLOGY_OPERATOR_ERODE: svgMorphologyOperator = svgMorphologyOperator(1i32);
pub const SVG_MORPHOLOGY_OPERATOR_DILATE: svgMorphologyOperator = svgMorphologyOperator(2i32);
pub const svgMorphologyOperator_Max: svgMorphologyOperator = svgMorphologyOperator(2147483647i32);
impl ::core::marker::Copy for svgMorphologyOperator {}
impl ::core::clone::Clone for svgMorphologyOperator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgPathSegType(pub i32);
pub const PATHSEG_UNKNOWN: svgPathSegType = svgPathSegType(0i32);
pub const PATHSEG_CLOSEPATH: svgPathSegType = svgPathSegType(1i32);
pub const PATHSEG_MOVETO_ABS: svgPathSegType = svgPathSegType(2i32);
pub const PATHSEG_MOVETO_REL: svgPathSegType = svgPathSegType(3i32);
pub const PATHSEG_LINETO_ABS: svgPathSegType = svgPathSegType(4i32);
pub const PATHSEG_LINETO_REL: svgPathSegType = svgPathSegType(5i32);
pub const PATHSEG_CURVETO_CUBIC_ABS: svgPathSegType = svgPathSegType(6i32);
pub const PATHSEG_CURVETO_CUBIC_REL: svgPathSegType = svgPathSegType(7i32);
pub const PATHSEG_CURVETO_QUADRATIC_ABS: svgPathSegType = svgPathSegType(8i32);
pub const PATHSEG_CURVETO_QUADRATIC_REL: svgPathSegType = svgPathSegType(9i32);
pub const PATHSEG_ARC_ABS: svgPathSegType = svgPathSegType(10i32);
pub const PATHSEG_ARC_REL: svgPathSegType = svgPathSegType(11i32);
pub const PATHSEG_LINETO_HORIZONTAL_ABS: svgPathSegType = svgPathSegType(12i32);
pub const PATHSEG_LINETO_HORIZONTAL_REL: svgPathSegType = svgPathSegType(13i32);
pub const PATHSEG_LINETO_VERTICAL_ABS: svgPathSegType = svgPathSegType(14i32);
pub const PATHSEG_LINETO_VERTICAL_REL: svgPathSegType = svgPathSegType(15i32);
pub const PATHSEG_CURVETO_CUBIC_SMOOTH_ABS: svgPathSegType = svgPathSegType(16i32);
pub const PATHSEG_CURVETO_CUBIC_SMOOTH_REL: svgPathSegType = svgPathSegType(17i32);
pub const PATHSEG_CURVETO_QUADRATIC_SMOOTH_ABS: svgPathSegType = svgPathSegType(18i32);
pub const PATHSEG_CURVETO_QUADRATIC_SMOOTH_REL: svgPathSegType = svgPathSegType(19i32);
pub const svgPathSegType_Max: svgPathSegType = svgPathSegType(2147483647i32);
impl ::core::marker::Copy for svgPathSegType {}
impl ::core::clone::Clone for svgPathSegType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgPreserveAlpha(pub i32);
pub const SVG_PRESERVEALPHA_FALSE: svgPreserveAlpha = svgPreserveAlpha(0i32);
pub const SVG_PRESERVEALPHA_TRUE: svgPreserveAlpha = svgPreserveAlpha(1i32);
pub const svgPreserveAlpha_Max: svgPreserveAlpha = svgPreserveAlpha(2147483647i32);
impl ::core::marker::Copy for svgPreserveAlpha {}
impl ::core::clone::Clone for svgPreserveAlpha {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgPreserveAspectMeetOrSliceType(pub i32);
pub const SVG_MEETORSLICE_UNKNOWN: svgPreserveAspectMeetOrSliceType = svgPreserveAspectMeetOrSliceType(0i32);
pub const SVG_MEETORSLICE_MEET: svgPreserveAspectMeetOrSliceType = svgPreserveAspectMeetOrSliceType(1i32);
pub const SVG_MEETORSLICE_SLICE: svgPreserveAspectMeetOrSliceType = svgPreserveAspectMeetOrSliceType(2i32);
pub const svgPreserveAspectMeetOrSliceType_Max: svgPreserveAspectMeetOrSliceType = svgPreserveAspectMeetOrSliceType(2147483647i32);
impl ::core::marker::Copy for svgPreserveAspectMeetOrSliceType {}
impl ::core::clone::Clone for svgPreserveAspectMeetOrSliceType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgPreserveAspectRatioAlignType(pub i32);
pub const SVG_PRESERVEASPECTRATIO_UNKNOWN: svgPreserveAspectRatioAlignType = svgPreserveAspectRatioAlignType(0i32);
pub const SVG_PRESERVEASPECTRATIO_NONE: svgPreserveAspectRatioAlignType = svgPreserveAspectRatioAlignType(1i32);
pub const SVG_PRESERVEASPECTRATIO_XMINYMIN: svgPreserveAspectRatioAlignType = svgPreserveAspectRatioAlignType(2i32);
pub const SVG_PRESERVEASPECTRATIO_XMIDYMIN: svgPreserveAspectRatioAlignType = svgPreserveAspectRatioAlignType(3i32);
pub const SVG_PRESERVEASPECTRATIO_XMAXYMIN: svgPreserveAspectRatioAlignType = svgPreserveAspectRatioAlignType(4i32);
pub const SVG_PRESERVEASPECTRATIO_XMINYMID: svgPreserveAspectRatioAlignType = svgPreserveAspectRatioAlignType(5i32);
pub const SVG_PRESERVEASPECTRATIO_XMIDYMID: svgPreserveAspectRatioAlignType = svgPreserveAspectRatioAlignType(6i32);
pub const SVG_PRESERVEASPECTRATIO_XMAXYMID: svgPreserveAspectRatioAlignType = svgPreserveAspectRatioAlignType(7i32);
pub const SVG_PRESERVEASPECTRATIO_XMINYMAX: svgPreserveAspectRatioAlignType = svgPreserveAspectRatioAlignType(8i32);
pub const SVG_PRESERVEASPECTRATIO_XMIDYMAX: svgPreserveAspectRatioAlignType = svgPreserveAspectRatioAlignType(9i32);
pub const SVG_PRESERVEASPECTRATIO_XMAXYMAX: svgPreserveAspectRatioAlignType = svgPreserveAspectRatioAlignType(10i32);
pub const svgPreserveAspectRatioAlignType_Max: svgPreserveAspectRatioAlignType = svgPreserveAspectRatioAlignType(2147483647i32);
impl ::core::marker::Copy for svgPreserveAspectRatioAlignType {}
impl ::core::clone::Clone for svgPreserveAspectRatioAlignType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgSpreadMethod(pub i32);
pub const SVG_SPREADMETHOD_UNKNOWN: svgSpreadMethod = svgSpreadMethod(0i32);
pub const SVG_SPREADMETHOD_PAD: svgSpreadMethod = svgSpreadMethod(1i32);
pub const SVG_SPREADMETHOD_REFLECT: svgSpreadMethod = svgSpreadMethod(2i32);
pub const SVG_SPREADMETHOD_REPEAT: svgSpreadMethod = svgSpreadMethod(3i32);
pub const svgSpreadMethod_Max: svgSpreadMethod = svgSpreadMethod(2147483647i32);
impl ::core::marker::Copy for svgSpreadMethod {}
impl ::core::clone::Clone for svgSpreadMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgStitchtype(pub i32);
pub const SVG_STITCHTYPE_UNKNOWN: svgStitchtype = svgStitchtype(0i32);
pub const SVG_STITCHTYPE_STITCH: svgStitchtype = svgStitchtype(1i32);
pub const SVG_STITCHTYPE_NOSTITCH: svgStitchtype = svgStitchtype(2i32);
pub const svgStitchtype_Max: svgStitchtype = svgStitchtype(2147483647i32);
impl ::core::marker::Copy for svgStitchtype {}
impl ::core::clone::Clone for svgStitchtype {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgTransformType(pub i32);
pub const SVG_TRANSFORM_UNKNOWN: svgTransformType = svgTransformType(0i32);
pub const SVG_TRANSFORM_MATRIX: svgTransformType = svgTransformType(1i32);
pub const SVG_TRANSFORM_TRANSLATE: svgTransformType = svgTransformType(2i32);
pub const SVG_TRANSFORM_SCALE: svgTransformType = svgTransformType(3i32);
pub const SVG_TRANSFORM_ROTATE: svgTransformType = svgTransformType(4i32);
pub const SVG_TRANSFORM_SKEWX: svgTransformType = svgTransformType(5i32);
pub const SVG_TRANSFORM_SKEWY: svgTransformType = svgTransformType(6i32);
pub const svgTransformType_Max: svgTransformType = svgTransformType(2147483647i32);
impl ::core::marker::Copy for svgTransformType {}
impl ::core::clone::Clone for svgTransformType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgTurbulenceType(pub i32);
pub const SVG_TURBULENCE_TYPE_UNKNOWN: svgTurbulenceType = svgTurbulenceType(0i32);
pub const SVG_TURBULENCE_TYPE_FACTALNOISE: svgTurbulenceType = svgTurbulenceType(1i32);
pub const SVG_TURBULENCE_TYPE_TURBULENCE: svgTurbulenceType = svgTurbulenceType(2i32);
pub const svgTurbulenceType_Max: svgTurbulenceType = svgTurbulenceType(2147483647i32);
impl ::core::marker::Copy for svgTurbulenceType {}
impl ::core::clone::Clone for svgTurbulenceType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct svgUnitTypes(pub i32);
pub const SVG_UNITTYPE_UNKNOWN: svgUnitTypes = svgUnitTypes(0i32);
pub const SVG_UNITTYPE_USERSPACEONUSE: svgUnitTypes = svgUnitTypes(1i32);
pub const SVG_UNITTYPE_OBJECTBOUNDINGBOX: svgUnitTypes = svgUnitTypes(2i32);
pub const svgUnitTypes_Max: svgUnitTypes = svgUnitTypes(2147483647i32);
impl ::core::marker::Copy for svgUnitTypes {}
impl ::core::clone::Clone for svgUnitTypes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct tagNavigateData {
    pub ulTarget: u32,
    pub ulURL: u32,
    pub ulRefURL: u32,
    pub ulPostData: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for tagNavigateData {}
impl ::core::clone::Clone for tagNavigateData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct textDecoration(pub i32);
pub const textDecorationNone: textDecoration = textDecoration(0i32);
pub const textDecorationUnderline: textDecoration = textDecoration(1i32);
pub const textDecorationOverline: textDecoration = textDecoration(2i32);
pub const textDecorationLineThrough: textDecoration = textDecoration(3i32);
pub const textDecorationBlink: textDecoration = textDecoration(4i32);
pub const textDecoration_Max: textDecoration = textDecoration(2147483647i32);
impl ::core::marker::Copy for textDecoration {}
impl ::core::clone::Clone for textDecoration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct textpathMethodtype(pub i32);
pub const TEXTPATH_METHODTYPE_UNKNOWN: textpathMethodtype = textpathMethodtype(0i32);
pub const TEXTPATH_METHODTYPE_ALIGN: textpathMethodtype = textpathMethodtype(1i32);
pub const TEXTPATH_METHODTYPE_STRETCH: textpathMethodtype = textpathMethodtype(2i32);
pub const textpathMethodtype_Max: textpathMethodtype = textpathMethodtype(2147483647i32);
impl ::core::marker::Copy for textpathMethodtype {}
impl ::core::clone::Clone for textpathMethodtype {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct textpathSpacingtype(pub i32);
pub const TEXTPATH_SPACINGTYPE_UNKNOWN: textpathSpacingtype = textpathSpacingtype(0i32);
pub const TEXTPATH_SPACINGTYPE_AUTO: textpathSpacingtype = textpathSpacingtype(1i32);
pub const TEXTPATH_SPACINGTYPE_EXACT: textpathSpacingtype = textpathSpacingtype(2i32);
pub const textpathSpacingtype_Max: textpathSpacingtype = textpathSpacingtype(2147483647i32);
impl ::core::marker::Copy for textpathSpacingtype {}
impl ::core::clone::Clone for textpathSpacingtype {
    fn clone(&self) -> Self {
        *self
    }
}
pub const wfolders: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3135446938, data2: 7041, data3: 4562, data4: [169, 122, 0, 192, 79, 142, 203, 2] };
