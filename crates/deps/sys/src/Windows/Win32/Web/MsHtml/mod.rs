#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ComputeInvCMAP(prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD, ncolors: u32, pinvtable: *mut u8, cbtable: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateDDrawSurfaceOnDIB(hbmdib: super::super::Graphics::Gdi::HBITMAP, ppsurface: *mut super::super::Graphics::DirectDraw::IDirectDrawSurface) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn CreateMIMEMap(ppmap: *mut IMapMIMEToCLSID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn DecodeImage(pstream: super::super::System::Com::IStream, pmap: IMapMIMEToCLSID, peventsink: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DecodeImageEx(pstream: super::super::System::Com::IStream, pmap: IMapMIMEToCLSID, peventsink: ::windows_sys::core::IUnknown, pszmimetypeparam: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DitherTo8(pdestbits: *mut u8, ndestpitch: i32, psrcbits: *mut u8, nsrcpitch: i32, bfidsrc: *const ::windows_sys::core::GUID, prgbdestcolors: *mut super::super::Graphics::Gdi::RGBQUAD, prgbsrccolors: *mut super::super::Graphics::Gdi::RGBQUAD, pbdestinvmap: *mut u8, x: i32, y: i32, cx: i32, cy: i32, ldesttrans: i32, lsrctrans: i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DoPrivacyDlg(hwndowner: super::super::Foundation::HWND, pszurl: super::super::Foundation::PWSTR, pprivacyenum: IEnumPrivacyRecords, freportallsites: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn GetMaxMIMEIDBytes(pnmaxbytes: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn IdentifyMIMEType(pbbytes: *const u8, nbytes: u32, pnformat: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAccessDeniedDialog(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PSTR, pszcontentdescription: super::super::Foundation::PSTR, pratingdetails: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAccessDeniedDialog2(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PSTR, pratingdetails: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAccessDeniedDialog2W(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PWSTR, pratingdetails: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAccessDeniedDialogW(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PWSTR, pszcontentdescription: super::super::Foundation::PWSTR, pratingdetails: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAddToApprovedSites(hdlg: super::super::Foundation::HWND, cbpasswordblob: u32, pbpasswordblob: *mut u8, lpszurl: super::super::Foundation::PWSTR, falwaysnever: super::super::Foundation::BOOL, fsitepage: super::super::Foundation::BOOL, fapprovedsitesenforced: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingCheckUserAccess(pszusername: super::super::Foundation::PSTR, pszurl: super::super::Foundation::PSTR, pszratinginfo: super::super::Foundation::PSTR, pdata: *const u8, cbdata: u32, ppratingdetails: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingCheckUserAccessW(pszusername: super::super::Foundation::PWSTR, pszurl: super::super::Foundation::PWSTR, pszratinginfo: super::super::Foundation::PWSTR, pdata: *const u8, cbdata: u32, ppratingdetails: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingClickedOnPRFInternal(hwndowner: super::super::Foundation::HWND, param1: super::super::Foundation::HINSTANCE, lpszfilename: super::super::Foundation::PSTR, nshow: i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingClickedOnRATInternal(hwndowner: super::super::Foundation::HWND, param1: super::super::Foundation::HINSTANCE, lpszfilename: super::super::Foundation::PSTR, nshow: i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingEnable(hwndparent: super::super::Foundation::HWND, pszusername: super::super::Foundation::PSTR, fenable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingEnableW(hwndparent: super::super::Foundation::HWND, pszusername: super::super::Foundation::PWSTR, fenable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn RatingEnabledQuery() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn RatingFreeDetails(pratingdetails: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn RatingInit() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingObtainCancel(hratingobtainquery: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingObtainQuery(psztargeturl: super::super::Foundation::PSTR, dwuserdata: u32, fcallback: isize, phratingobtainquery: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingObtainQueryW(psztargeturl: super::super::Foundation::PWSTR, dwuserdata: u32, fcallback: isize, phratingobtainquery: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingSetupUI(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingSetupUIW(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SniffStream(pinstream: super::super::System::Com::IStream, pnformat: *mut u32, ppoutstream: *mut super::super::System::Com::IStream) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const ADDRESSBAND: u32 = 2u32;
pub struct ADDURL_FLAG(i32);
pub struct AnchorClick(i32);
pub struct ApplicationCache(i32);
pub struct BEHAVIOR_EVENT(i32);
pub struct BEHAVIOR_EVENT_FLAGS(i32);
pub struct BEHAVIOR_LAYOUT_INFO(i32);
pub struct BEHAVIOR_LAYOUT_MODE(i32);
pub struct BEHAVIOR_RELATION(i32);
pub struct BEHAVIOR_RENDER_INFO(i32);
pub struct BlockFormats(i32);
pub struct BoolValue(i32);
pub struct CARET_DIRECTION(i32);
pub const CATID_MSOfficeAntiVirus: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1459604528, data2: 54168, data3: 4560, data4: [178, 174, 0, 160, 201, 8, 250, 73] };
pub struct CClientCaps(i32);
pub struct CDeviceRect(i32);
pub struct CDownloadBehavior(i32);
pub struct CEventObj(i32);
pub const CGID_DocHostCommandHandler: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4086022722, data2: 47440, data3: 4561, data4: [137, 24, 0, 192, 79, 194, 200, 54] };
pub const CGID_EditStateCommands: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611894, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub struct CHeaderFooter(i32);
pub struct CLayoutRect(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CMDID_HOSTCONTEXT_URL: u32 = 8u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CMDID_SCRIPTSITE_ALLOWRECOVERY: u32 = 9u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CMDID_SCRIPTSITE_BASEIURI: u32 = 10u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CMDID_SCRIPTSITE_HTMLDLGTRUST: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CMDID_SCRIPTSITE_IURI: u32 = 7u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CMDID_SCRIPTSITE_NAMESPACE: u32 = 6u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CMDID_SCRIPTSITE_SECSTATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CMDID_SCRIPTSITE_SECURITY_WINDOW: u32 = 5u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CMDID_SCRIPTSITE_SID: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CMDID_SCRIPTSITE_TRUSTEDDOC: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CMDID_SCRIPTSITE_URL: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CMD_ZOOM_FIT: i32 = -5i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CMD_ZOOM_ONEPAGE: i32 = -2i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CMD_ZOOM_PAGEWIDTH: i32 = -1i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CMD_ZOOM_SELECTION: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CMD_ZOOM_TWOPAGES: i32 = -3i32;
pub struct CMimeTypes(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const COLOR_NO_TRANSPARENT: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CONTEXT_MENU_ANCHOR: u32 = 5u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CONTEXT_MENU_CONTROL: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CONTEXT_MENU_DEBUG: u32 = 8u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CONTEXT_MENU_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CONTEXT_MENU_DISABLEDFLASH: u32 = 14u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CONTEXT_MENU_ENTITY: u32 = 12u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CONTEXT_MENU_HSCROLL: u32 = 10u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CONTEXT_MENU_IMAGE: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CONTEXT_MENU_IMGDYNSRC: u32 = 7u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CONTEXT_MENU_MEDIA: u32 = 11u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CONTEXT_MENU_PDF: u32 = 13u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CONTEXT_MENU_TABLE: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CONTEXT_MENU_TEXTSELECT: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CONTEXT_MENU_UNKNOWN: u32 = 6u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const CONTEXT_MENU_VSCROLL: u32 = 9u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const COOKIEACTION_ACCEPT: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const COOKIEACTION_DOWNGRADE: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const COOKIEACTION_LEASH: u32 = 8u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const COOKIEACTION_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const COOKIEACTION_READ: u32 = 32u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const COOKIEACTION_REJECT: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const COOKIEACTION_SUPPRESS: u32 = 16u32;
pub struct COORD_SYSTEM(i32);
pub struct COpsProfile(i32);
pub struct CPersistDataPeer(i32);
pub struct CPersistHistory(i32);
pub struct CPersistShortcut(i32);
pub struct CPersistSnapshot(i32);
pub struct CPersistUserData(i32);
pub struct CPlugins(i32);
pub struct CPrintManagerTemplatePrinter(i32);
pub struct CTemplatePrinter(i32);
pub struct CanvasGradient(i32);
pub struct CanvasImageData(i32);
pub struct CanvasPattern(i32);
pub struct CanvasRenderingContext2D(i32);
pub struct CanvasTextMetrics(i32);
pub struct CoDitherToRGB8(i32);
pub struct CoMapMIMEToCLSID(i32);
pub struct CoSniffStream(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DEBUGCALLBACKNOTIFICATION_ANIMATIONFRAME: u32 = 8u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DEBUGCALLBACKNOTIFICATION_DOMEVENT: u32 = 16u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DEBUGCALLBACKNOTIFICATION_IMMEDIATE: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DEBUGCALLBACKNOTIFICATION_INTERVAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DEBUGCALLBACKNOTIFICATION_TIMEOUT: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DEBUGDOMEVENTPROPAGATIONSTATUS_DEFAULTCANCELED: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DEBUGDOMEVENTPROPAGATIONSTATUS_STOPIMMEDIATEPROPAGATION: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DEBUGDOMEVENTPROPAGATIONSTATUS_STOPPROPAGATION: u32 = 4u32;
pub struct DEV_CONSOLE_MESSAGE_LEVEL(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISP10_IE10_XMSARIAFLOWFROM: u32 = 66835u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_1D: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_2D: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_AAHEADER: u32 = 70793u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ACTIVEXFILTERINGENABLED: u32 = 61u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ACTIVEX_EXPANDO_BASE: u32 = 72536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ACTIVEX_EXPANDO_MAX: u32 = 73535u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ADDCHANNEL: u32 = 5u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ADDDESKTOPCOMPONENT: u32 = 6u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ADDFAVORITE: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ADDSEARCHPROVIDER: u32 = 14u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ADDSERVICE: u32 = 30u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ADDSITEMODE: u32 = 49u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ADDTHUMBNAILBUTTONS: u32 = 48u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ADDTOFAVORITESBAR: u32 = 32u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ADDTRACKINGPROTECTIONLIST: u32 = 57u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ADVANCEERROR: u32 = 10u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ADVISEDATASRCCHANGEEVENT: i32 = -3901i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_AMBIENT_DLCONTROL: i32 = -5512i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_AMBIENT_OFFLINEIFNOTCONNECTED: i32 = -5501i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_AMBIENT_SILENT: i32 = -5502i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_AMBIENT_USERAGENT: i32 = -5513i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ANCHOR: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_APPLICATIONCACHE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_AREA: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ATTRS: u32 = 70536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_AUDIO: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_AUTOCOMPLETEATTACH: u32 = 12u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_AUTOCOMPLETESAVEFORM: u32 = 10u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_AUTOSCAN: u32 = 11u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ACCELERATOR: u32 = 70683u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ACCEVENTRECORDID_END: u32 = 71209u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ACCEVENTRECORDID_START: u32 = 71190u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ALIGNCONTENT: u32 = 71009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ALIGNITEMS: u32 = 71007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ALIGNMENTBASELINE: u32 = 70814u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ALIGNSELF: u32 = 71008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ALLOWTRANSPARENCY: u32 = 70742u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ANIMATION: u32 = 70985u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ANIMATIONDELAY: u32 = 70981u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ANIMATIONDIRECTION: u32 = 70982u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ANIMATIONDURATION: u32 = 70979u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ANIMATIONFILLMODE: u32 = 70986u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ANIMATIONITERATIONCOUNT: u32 = 70984u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ANIMATIONNAME: u32 = 70978u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ANIMATIONPLAYSTATE: u32 = 70983u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ANIMATIONTIMINGFUNCTION: u32 = 70980u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BACKFACEVISIBILITY: u32 = 70977u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BACKGROUND: u32 = 70568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BACKGROUNDATTACHMENT: u32 = 70581u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BACKGROUNDIMAGE: u32 = 70537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BACKGROUNDPOSITION: u32 = 70582u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BACKGROUNDPOSX: u32 = 70569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BACKGROUNDPOSY: u32 = 70570u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BACKGROUNDREPEAT: u32 = 70580u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BASEFONT: u32 = 70562u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BASELINESHIFT: u32 = 70815u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BDURLIMGCTXCACHEINDEX: u32 = 71214u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BEHAVIOR: u32 = 70651u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BGURLIMGCTXCACHEINDEX: u32 = 70646u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BGURLIMGCTXCACHEINDEX_FLETTER: u32 = 70738u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BGURLIMGCTXCACHEINDEX_FLINE: u32 = 70737u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDER: u32 = 70585u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERBOTTOM: u32 = 70588u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERBOTTOMCOLOR: u32 = 70593u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERBOTTOMLEFTRADIUS: u32 = 70850u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERBOTTOMRIGHTRADIUS: u32 = 70849u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERBOTTOMSTYLE: u32 = 70603u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERBOTTOMWIDTH: u32 = 70598u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERCOLLAPSE: u32 = 70620u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERCOLOR: u32 = 70590u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERIMAGE: u32 = 71010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERIMAGEOUTSET: u32 = 71014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERIMAGEREPEAT: u32 = 71015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERIMAGESLICE: u32 = 71012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERIMAGESOURCE: u32 = 71011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERIMAGEWIDTH: u32 = 71013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERLEFT: u32 = 70589u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERLEFTCOLOR: u32 = 70594u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERLEFTSTYLE: u32 = 70604u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERLEFTWIDTH: u32 = 70599u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERRADIUS: u32 = 70846u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERRIGHT: u32 = 70587u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERRIGHTCOLOR: u32 = 70592u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERRIGHTSTYLE: u32 = 70602u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERRIGHTWIDTH: u32 = 70597u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERSPACING: u32 = 70763u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERSTYLE: u32 = 70600u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERTOP: u32 = 70586u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERTOPCOLOR: u32 = 70591u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERTOPLEFTRADIUS: u32 = 70847u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERTOPRIGHTRADIUS: u32 = 70848u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERTOPSTYLE: u32 = 70601u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERTOPWIDTH: u32 = 70596u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BORDERWIDTH: u32 = 70595u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BOXSIZING: u32 = 70762u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BREAKAFTER: u32 = 70882u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BREAKBEFORE: u32 = 70881u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_BREAKINSIDE: u32 = 70883u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_CAPTIONSIDE: u32 = 70755u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_CLASSLIST: u32 = 70953u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_CLEAR: u32 = 70552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_CLIP: u32 = 70628u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_CLIPPATH: u32 = 70820u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_CLIPRECTBOTTOM: u32 = 70631u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_CLIPRECTLEFT: u32 = 70632u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_CLIPRECTRIGHT: u32 = 70630u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_CLIPRECTTOP: u32 = 70629u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_CLIPRULE: u32 = 70821u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_COLOR: u32 = 70538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_COLORINTERPOLATIONFILTERS: u32 = 70928u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_COLUMNCOUNT: u32 = 70872u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_COLUMNFILL: u32 = 70875u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_COLUMNGAP: u32 = 70874u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_COLUMNRULE: u32 = 70877u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_COLUMNRULECOLOR: u32 = 70880u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_COLUMNRULESTYLE: u32 = 70878u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_COLUMNRULEWIDTH: u32 = 70879u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_COLUMNS: u32 = 70871u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_COLUMNSPAN: u32 = 70876u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_COLUMNWIDTH: u32 = 70873u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_CONTENT: u32 = 70754u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_COUNTERINCREMENT: u32 = 70756u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_COUNTERRESET: u32 = 70757u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_CSSFLOAT: u32 = 70845u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_CURSOR: u32 = 70638u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_DATASET: u32 = 71016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_DEFAULTTEXTSELECTION: u32 = 70724u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_DIR: u32 = 70653u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_DIRECTION: u32 = 70655u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_DISPLAY: u32 = 70607u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_DOCFRAGMENT: u32 = 70678u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_DOMINANTBASELINE: u32 = 70816u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_DRAGGABLE: u32 = 70944u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_EDITABLE: u32 = 70698u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_EMPTYCELLS: u32 = 70786u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ENABLEBACKGROUND: u32 = 70946u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_EVENTSINK: u32 = 70639u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_EXTENDEDTAGDESC: u32 = 70687u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FILL: u32 = 70822u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FILLOPACITY: u32 = 70823u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FILLRULE: u32 = 70824u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FILTER: u32 = 70618u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FIRST: u32 = 70536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FLEX: u32 = 71002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FLEXBASIS: u32 = 71005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FLEXDIRECTION: u32 = 70998u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FLEXFLOW: u32 = 71000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FLEXGROW: u32 = 71003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FLEXSHRINK: u32 = 71004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FLEXWRAP: u32 = 70999u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FLOAT: u32 = 70606u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FLOODCOLOR: u32 = 70926u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FLOODOPACITY: u32 = 70927u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FONT: u32 = 70577u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FONTFACE: u32 = 70554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FONTFACESRC: u32 = 70633u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FONTFEATURESETTINGS: u32 = 70987u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FONTSIZE: u32 = 70555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FONTSIZEADJUST: u32 = 70817u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FONTSIZECOMBINE: u32 = 70579u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FONTSIZEKEYWORD: u32 = 70578u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FONTSTRETCH: u32 = 70818u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FONTSTYLE: u32 = 70560u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FONTVARIANT: u32 = 70561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FONTWEIGHT: u32 = 70563u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_FROZEN: u32 = 70734u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_GLYPHORIENTATIONHORIZONTAL: u32 = 70843u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_GLYPHORIENTATIONVERTICAL: u32 = 70844u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_HASLAYOUT: u32 = 70696u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_HIDDEN: u32 = 70617u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_HIDEFOCUS: u32 = 70699u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_HTCDD_CREATEEVENTOBJECT: u32 = 70680u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_HTCDD_DEFAULTS: u32 = 70701u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_HTCDD_ELEMENT: u32 = 70679u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_HTCDD_ISMARKUPSHARED: u32 = 70693u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_HTCDD_PROTECTEDELEMENT: u32 = 70690u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_HTCDISPATCHITEM_VALUE: u32 = 70677u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_HTCDISPATCHITEM_VALUE_SCRIPTSONLY: u32 = 70686u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_IE9_BACKGROUNDCLIP: u32 = 70852u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_IE9_BACKGROUNDORIGIN: u32 = 70853u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_IE9_BACKGROUNDSIZE: u32 = 70854u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_IE9_BOXSHADOW: u32 = 70855u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_IMEMODE: u32 = 70656u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_INTERPOLATION: u32 = 70749u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ISBLOCK: u32 = 70744u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_JUSTIFYCONTENT: u32 = 71006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_KERNING: u32 = 70825u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LANG: u32 = 70545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LANGUAGE: u32 = 70636u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LAYOUTFLOW: u32 = 70691u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LAYOUTGRID: u32 = 70667u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LAYOUTGRIDCHAR: u32 = 70663u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LAYOUTGRIDLINE: u32 = 70664u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LAYOUTGRIDMODE: u32 = 70665u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LAYOUTGRIDTYPE: u32 = 70666u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LETTERSPACING: u32 = 70544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LIGHTINGCOLOR: u32 = 70929u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LINEBREAK: u32 = 70669u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LINEHEIGHT: u32 = 70542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LISTSTYLE: u32 = 70611u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LISTSTYLEIMAGE: u32 = 70610u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LISTSTYLEPOSITION: u32 = 70609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LISTSTYLETYPE: u32 = 70608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LISTTYPE: u32 = 70553u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_LIURLIMGCTXCACHEINDEX: u32 = 70647u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MARGIN: u32 = 70572u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MARGINBOTTOM: u32 = 70575u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MARGINLEFT: u32 = 70576u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MARGINRIGHT: u32 = 70574u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MARGINTOP: u32 = 70573u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MARKER: u32 = 70826u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MARKEREND: u32 = 70827u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MARKERMID: u32 = 70828u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MARKERSTART: u32 = 70829u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MASK: u32 = 70830u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MAX: u32 = 71535u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MAXHEIGHT: u32 = 70750u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MAXWIDTH: u32 = 70752u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIA: u32 = 70697u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAASPECTRATIO: u32 = 71153u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIACOLOR: u32 = 71159u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIACOLORINDEX: u32 = 71162u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIADEVICEASPECTRATIO: u32 = 71156u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIADEVICEHEIGHT: u32 = 71150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIADEVICEWIDTH: u32 = 71147u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAGRID: u32 = 71188u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAHEIGHT: u32 = 71144u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMAXASPECTRATIO: u32 = 71151u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMAXCOLOR: u32 = 71157u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMAXCOLORINDEX: u32 = 71160u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMAXDEVICEASPECTRATIO: u32 = 71154u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMAXDEVICEHEIGHT: u32 = 71148u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMAXDEVICEWIDTH: u32 = 71145u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMAXHEIGHT: u32 = 71142u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMAXMONOCHROME: u32 = 71163u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMAXRESOLUTION: u32 = 71166u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMAXWIDTH: u32 = 71139u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMINASPECTRATIO: u32 = 71152u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMINCOLOR: u32 = 71158u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMINCOLORINDEX: u32 = 71161u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMINDEVICEASPECTRATIO: u32 = 71155u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMINDEVICEHEIGHT: u32 = 71149u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMINDEVICEWIDTH: u32 = 71146u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMINHEIGHT: u32 = 71143u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMINMONOCHROME: u32 = 71164u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMINRESOLUTION: u32 = 71167u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMINWIDTH: u32 = 71140u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMONOCHROME: u32 = 71165u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMSHIGHCONTRAST: u32 = 71174u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAMSVIEWSTATE: u32 = 71178u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAORIENTATION: u32 = 71138u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIARESOLUTION: u32 = 71168u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIASCAN: u32 = 71189u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAWEBKITDEVICEPIXELRATIO: u32 = 71215u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAWEBKITMAXDEVICEPIXELRATIO: u32 = 71216u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAWEBKITMINDEVICEPIXELRATIO: u32 = 71217u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MEDIAWIDTH: u32 = 71141u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MIN: u32 = 70536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MINHEIGHT: u32 = 70747u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MINWIDTH: u32 = 70751u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_ACCELERATOR: u32 = 70783u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_ANIMATION: u32 = 70924u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_ANIMATIONDELAY: u32 = 70920u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_ANIMATIONDIRECTION: u32 = 70921u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_ANIMATIONDURATION: u32 = 70918u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_ANIMATIONFILLMODE: u32 = 70925u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_ANIMATIONITERATIONCOUNT: u32 = 70923u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_ANIMATIONNAME: u32 = 70917u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_ANIMATIONPLAYSTATE: u32 = 70922u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_ANIMATIONTIMINGFUNCTION: u32 = 70919u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_BACKFACEVISIBILITY: u32 = 70890u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_BACKGROUNDPOSX: u32 = 70781u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_BACKGROUNDPOSY: u32 = 70782u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_BEHAVIOR: u32 = 70767u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_BLOCKPROGRESSION: u32 = 70787u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_CONTENTZOOMCHAINING: u32 = 70895u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_CONTENTZOOMFACTOR: u32 = 70900u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_CONTENTZOOMING: u32 = 70892u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_CONTENTZOOMLIMIT: u32 = 70897u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_CONTENTZOOMLIMITMAX: u32 = 70902u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_CONTENTZOOMLIMITMIN: u32 = 70901u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_CONTENTZOOMSNAP: u32 = 70898u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_CONTENTZOOMSNAPPOINTS: u32 = 70899u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_CONTENTZOOMSNAPTYPE: u32 = 70893u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_FILTER: u32 = 70801u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_FLEX: u32 = 70955u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_FLEXALIGN: u32 = 70962u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_FLEXDIRECTION: u32 = 70960u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_FLEXFLOW: u32 = 70959u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_FLEXITEMALIGN: u32 = 70963u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_FLEXLINEPACK: u32 = 70965u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_FLEXNEGATIVE: u32 = 70957u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_FLEXORDER: u32 = 70966u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_FLEXPACK: u32 = 70964u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_FLEXPOSITIVE: u32 = 70956u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_FLEXPREFERREDSIZE: u32 = 70958u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_FLEXWRAP: u32 = 70961u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_FLOWFROM: u32 = 70938u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_FLOWINTO: u32 = 70939u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_FONTFEATURESETTINGS: u32 = 70950u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_GRIDCOLUMN: u32 = 70908u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_GRIDCOLUMNALIGN: u32 = 70909u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_GRIDCOLUMNS: u32 = 70910u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_GRIDCOLUMNSPAN: u32 = 70911u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_GRIDROW: u32 = 70913u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_GRIDROWALIGN: u32 = 70914u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_GRIDROWS: u32 = 70915u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_GRIDROWSPAN: u32 = 70916u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_HIGHCONTRASTADJUST: u32 = 70945u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_HYPHENATE_LIMIT_CHARS: u32 = 70942u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_HYPHENATE_LIMIT_LINES: u32 = 70943u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_HYPHENATE_LIMIT_ZONE: u32 = 70941u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_HYPHENS: u32 = 70940u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_IMEALIGN: u32 = 71017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_IMEMODE: u32 = 70780u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_LAYOUTFLOW: u32 = 70784u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_LAYOUTGRID: u32 = 70799u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_LAYOUTGRIDCHAR: u32 = 70795u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_LAYOUTGRIDLINE: u32 = 70796u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_LAYOUTGRIDMODE: u32 = 70797u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_LAYOUTGRIDTYPE: u32 = 70798u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_LINEBREAK: u32 = 70800u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_OVERFLOWSTYLE: u32 = 70935u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_OVERFLOWX: u32 = 70802u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_OVERFLOWY: u32 = 70803u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_PERSPECTIVE: u32 = 70885u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_PERSPECTIVEORIGIN: u32 = 70886u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_PERSPECTIVEORIGINX: u32 = 70887u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_PERSPECTIVEORIGINY: u32 = 70888u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLBAR3DLIGHTCOLOR: u32 = 70770u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLBARARROWCOLOR: u32 = 70774u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLBARBASECOLOR: u32 = 70768u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLBARDARKSHADOWCOLOR: u32 = 70773u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLBARFACECOLOR: u32 = 70769u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLBARHIGHLIGHTCOLOR: u32 = 70772u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLBARSHADOWCOLOR: u32 = 70771u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLBARTRACKCOLOR: u32 = 70775u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLCHAINING: u32 = 70891u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLLIMIT: u32 = 70934u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLLIMITXMAX: u32 = 70932u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLLIMITXMIN: u32 = 70930u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLLIMITYMAX: u32 = 70933u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLLIMITYMIN: u32 = 70931u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLRAILS: u32 = 70894u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLSNAPPOINTSX: u32 = 70905u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLSNAPPOINTSY: u32 = 70906u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLSNAPTYPE: u32 = 70896u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLSNAPX: u32 = 70903u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLSNAPY: u32 = 70904u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_SCROLLTRANSLATION: u32 = 70954u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TEXTALIGNLAST: u32 = 70776u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TEXTAUTOSPACE: u32 = 70804u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TEXTCOMBINEHORIZONTAL: u32 = 71018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TEXTJUSTIFY: u32 = 70805u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TEXTKASHIDASPACE: u32 = 70806u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TEXTOVERFLOW: u32 = 70777u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TEXTSIZEADJUST: u32 = 70864u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TEXTUNDERLINEPOSITION: u32 = 70778u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TOUCHACTION: u32 = 70952u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TOUCHSELECT: u32 = 70994u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TRANSFORM: u32 = 70851u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TRANSFORMORIGIN: u32 = 70861u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TRANSFORMORIGINX: u32 = 70862u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TRANSFORMORIGINY: u32 = 70863u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TRANSFORMORIGINZ: u32 = 70884u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TRANSFORMSTYLE: u32 = 70889u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TRANSITION: u32 = 70870u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TRANSITIONDELAY: u32 = 70869u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TRANSITIONDURATION: u32 = 70867u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TRANSITIONPROPERTY: u32 = 70866u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_TRANSITIONTIMINGFUNCTION: u32 = 70868u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_USERSELECT: u32 = 70951u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_WORDBREAK: u32 = 70807u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_WORDWRAP: u32 = 70808u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_WRAPFLOW: u32 = 70949u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_WRAPMARGIN: u32 = 70947u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_WRAPTHROUGH: u32 = 70937u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_WRITINGMODE: u32 = 70779u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_MS_ZOOM: u32 = 70785u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_NOWRAP: u32 = 70541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_OPACITY: u32 = 70819u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ORDER: u32 = 71001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ORPHANS: u32 = 70764u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_OUTLINE: u32 = 70758u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_OUTLINECOLOR: u32 = 70761u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_OUTLINESTYLE: u32 = 70760u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_OUTLINEWIDTH: u32 = 70759u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_OVERFLOW: u32 = 70546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_OVERFLOWX: u32 = 70675u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_OVERFLOWY: u32 = 70676u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_PADDING: u32 = 70547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_PADDINGBOTTOM: u32 = 70550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_PADDINGLEFT: u32 = 70551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_PADDINGRIGHT: u32 = 70549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_PADDINGTOP: u32 = 70548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_PAGEBREAKAFTER: u32 = 70614u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_PAGEBREAKBEFORE: u32 = 70613u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_PAGEBREAKINSIDE: u32 = 70766u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_PERSPECTIVE: u32 = 70974u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_PERSPECTIVEORIGIN: u32 = 70975u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_PERSPECTIVEORIGINX: u32 = 70992u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_PERSPECTIVEORIGINY: u32 = 70993u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_POINTEREVENTS: u32 = 70831u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_POSITION: u32 = 70626u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_PROPNOTIFYSINK: u32 = 70640u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_QUOTES: u32 = 70788u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_READYSTATE: u32 = 70652u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_RENDERINGPRIORITY: u32 = 70706u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ROTATE: u32 = 70688u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ROWPOSITIONCHANGESINK: u32 = 70650u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ROWSETASYNCHNOTIFYSINK: u32 = 70648u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ROWSETNOTIFYSINK: u32 = 70641u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_RUBYALIGN: u32 = 70657u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_RUBYOVERHANG: u32 = 70659u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_RUBYPOSITION: u32 = 70658u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_SCROLL: u32 = 70615u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_SCROLLBAR3DLIGHTCOLOR: u32 = 70718u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_SCROLLBARARROWCOLOR: u32 = 70722u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_SCROLLBARBASECOLOR: u32 = 70716u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_SCROLLBARDARKSHADOWCOLOR: u32 = 70721u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_SCROLLBARFACECOLOR: u32 = 70717u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_SCROLLBARHIGHLIGHTCOLOR: u32 = 70720u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_SCROLLBARSHADOWCOLOR: u32 = 70719u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_SCROLLBARTRACKCOLOR: u32 = 70732u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_SPELLCHECK: u32 = 70907u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_STOPCOLOR: u32 = 70832u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_STOPOPACITY: u32 = 70833u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_STROKE: u32 = 70834u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_STROKEDASHARRAY: u32 = 70835u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_STROKEDASHOFFSET: u32 = 70836u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_STROKELINECAP: u32 = 70837u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_STROKELINEJOIN: u32 = 70838u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_STROKEMITERLIMIT: u32 = 70839u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_STROKEOPACITY: u32 = 70840u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_STROKEWIDTH: u32 = 70841u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_STYLETEXT: u32 = 70635u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_STYLETEXTDECORATION: u32 = 70727u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TABLEBORDERCOLOR: u32 = 70564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TABLEBORDERCOLORDARK: u32 = 70566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TABLEBORDERCOLORLIGHT: u32 = 70565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TABLELAYOUT: u32 = 70634u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TABLEVALIGN: u32 = 70567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTALIGNLAST: u32 = 70739u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTANCHOR: u32 = 70842u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTAUTOSPACE: u32 = 70668u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTBACKGROUNDCOLOR: u32 = 70705u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTCOLOR: u32 = 70726u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTDECORATION: u32 = 70571u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTDECORATIONBLINK: u32 = 70558u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTDECORATIONCOLOR: u32 = 70725u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTDECORATIONLINETHROUGH: u32 = 70556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTDECORATIONNONE: u32 = 70559u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTDECORATIONOVERLINE: u32 = 70605u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTDECORATIONUNDERLINE: u32 = 70557u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTEFFECT: u32 = 70704u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTINDENT: u32 = 70543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTJUSTIFY: u32 = 70671u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTJUSTIFYTRIM: u32 = 70672u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTKASHIDA: u32 = 70673u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTKASHIDASPACE: u32 = 70740u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTLINETHROUGHSTYLE: u32 = 70702u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTOVERFLOW: u32 = 70745u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTSHADOW: u32 = 70936u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTTRANSFORM: u32 = 70540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTUNDERLINEPOSITION: u32 = 70695u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TEXTUNDERLINESTYLE: u32 = 70703u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TOUCHACTION: u32 = 71019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TRANSFORM: u32 = 70967u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TRANSFORMORIGIN: u32 = 70968u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TRANSFORMORIGINX: u32 = 70988u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TRANSFORMORIGINY: u32 = 70989u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TRANSFORMORIGINZ: u32 = 70990u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TRANSFORMSTYLE: u32 = 70976u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TRANSITION: u32 = 70973u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TRANSITIONDELAY: u32 = 70972u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TRANSITIONDURATION: u32 = 70970u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TRANSITIONPROPERTY: u32 = 70969u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_TRANSITIONTIMINGFUNCTION: u32 = 70971u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_UNICODEBIDI: u32 = 70654u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_UNIQUEPEERNUMBER: u32 = 70682u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_URNATOM: u32 = 70681u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_VALUE: u32 = 70637u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_VERTICALALIGN: u32 = 70584u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_VIEWINHERITSTYLE: u32 = 70735u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_VISIBILITY: u32 = 70616u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_ANIMATION: u32 = 71033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_ANIMATIONDELAY: u32 = 71038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_ANIMATIONDIRECTION: u32 = 71040u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_ANIMATIONDURATION: u32 = 71036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_ANIMATIONFILLMODE: u32 = 71027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_ANIMATIONITERATIONCOUNT: u32 = 71039u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_ANIMATIONNAME: u32 = 71035u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_ANIMATIONPLAYSTATE: u32 = 71041u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_ANIMATIONTIMINGFUNCTION: u32 = 71037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_APPEARANCE: u32 = 71020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BACKFACEVISIBILITY: u32 = 71030u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BACKGROUND: u32 = 71055u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BACKGROUNDATTACHMENT: u32 = 71046u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BACKGROUNDCLIP: u32 = 71048u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BACKGROUNDCOLOR: u32 = 71047u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BACKGROUNDIMAGE: u32 = 71049u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BACKGROUNDORIGIN: u32 = 71051u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BACKGROUNDPOSITION: u32 = 71052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BACKGROUNDPOSITIONX: u32 = 71053u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BACKGROUNDPOSITIONY: u32 = 71054u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BACKGROUNDREPEAT: u32 = 71050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BACKGROUNDSIZE: u32 = 71029u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BORDERIMAGE: u32 = 71061u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BORDERIMAGEOUTSET: u32 = 71065u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BORDERIMAGEREPEAT: u32 = 71066u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BORDERIMAGESLICE: u32 = 71063u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BORDERIMAGESOURCE: u32 = 71062u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BORDERIMAGEWIDTH: u32 = 71064u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BOXALIGN: u32 = 71021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BOXDIRECTION: u32 = 71026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BOXFLEX: u32 = 71024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BOXORDINALGROUP: u32 = 71022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BOXORIENT: u32 = 71025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BOXPACK: u32 = 71023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_BOXSIZING: u32 = 71031u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_TEXTSIZEADJUST: u32 = 71060u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_TRANSFORM: u32 = 71028u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_TRANSFORMORIGIN: u32 = 71056u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_TRANSFORMORIGINX: u32 = 71057u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_TRANSFORMORIGINY: u32 = 71058u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_TRANSFORMORIGINZ: u32 = 71059u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_TRANSITION: u32 = 71034u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_TRANSITIONDELAY: u32 = 71045u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_TRANSITIONDURATION: u32 = 71043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_TRANSITIONPROPERTY: u32 = 71042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_TRANSITIONTIMINGFUNCTION: u32 = 71044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WEBKIT_USERSELECT: u32 = 71032u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WHITESPACE: u32 = 70612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WIDOWS: u32 = 70765u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WORDBREAK: u32 = 70670u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WORDSPACING: u32 = 70583u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WORDWRAP: u32 = 70694u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_WRITINGMODE: u32 = 70728u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ZINDEX: u32 = 70627u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_A_ZOOM: u32 = 70689u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_BASE_STYLE: u32 = 70036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_BASE_STYLERULE: u32 = 1100u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_BEFORENAVIGATE: u32 = 100u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_BEFORENAVIGATE2: u32 = 250u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_BEFORESCRIPTEXECUTE: u32 = 290u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_BGSOUND: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_BLOCK: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_BODY: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_BR: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_BRANDIMAGEURI: u32 = 20u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_BUILDNEWTABPAGE: u32 = 33u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_BUTTON: u32 = 8000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CANADVANCEERROR: u32 = 12u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CANRETREATERROR: u32 = 13u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CANVASELEMENT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CANVASGRADIENT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CANVASIMAGEDATA: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CANVASPIXELARRAY: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CANVASPIXELARRAY_BASE: u32 = 5000000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CANVASPIXELARRAY_MAX: u32 = 2000000000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CANVASRENDERCONTEXT2D: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CANVASTEXTMETRICS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CHANGEDEFAULTBROWSER: u32 = 68u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CHECKBOX: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CLEARNOTIFICATION: u32 = 71u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CLEARSITEMODEICONOVERLAY: u32 = 45u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CLIENTCAPS: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CLIENTTOHOSTWINDOW: u32 = 268u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_COLLECTION: u32 = 1500u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_COLLECTION_MAX: u32 = 2999999u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_COLLECTION_MIN: u32 = 1000000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_COMMANDSTATECHANGE: u32 = 105u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_COMMENTPDL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CONTENTDISCOVERYRESET: u32 = 36u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_COUNTVIEWTYPES: u32 = 22u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CREATESUBSCRIPTION: u32 = 11u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CUSTOMIZECLEARTYPE: u32 = 23u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CUSTOMIZESETTINGS: u32 = 17u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CommonCtrl_FONTBOLD: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CommonCtrl_FONTCHARSET: u32 = 8u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CommonCtrl_FONTITAL: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CommonCtrl_FONTNAME: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CommonCtrl_FONTSIZE: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CommonCtrl_FONTSTRIKE: u32 = 6u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CommonCtrl_FONTSUBSCRIPT: u32 = 10u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CommonCtrl_FONTSUPERSCRIPT: u32 = 9u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CommonCtrl_FONTUNDER: u32 = 5u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_CommonCtrl_FONTWEIGHT: u32 = 7u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DATALIST: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DATATRANSFER: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DD: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DEBUG_ENABLESECUREPROXYASSERTS: i32 = -5518i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DEBUG_INTERNALWINDOW: i32 = -5517i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DEBUG_ISSECUREPROXY: i32 = -5515i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DEBUG_TRUSTEDPROXY: i32 = -5516i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DEFAULTS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DEFAULTSEARCHPROVIDER: u32 = 26u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DEFAULTVALUE: u32 = 70619u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DELETESUBSCRIPTION: u32 = 12u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DEPTH: u32 = 17u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DIAGNOSECONNECTION: u32 = 22u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DIAGNOSECONNECTIONUILESS: u32 = 66u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DIR: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DIV: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOCFRAG: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOCUMENTCOMPATIBLEINFO: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOCUMENTCOMPATIBLEINFO_COLLECTION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOCUMENTCOMPLETE: u32 = 259u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOCUMENTTYPE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMATTRIBUTE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMBEFOREUNLOADEVENT: u32 = 1375u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMCLOSEEVENT: u32 = 1525u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMCOMPOSITIONEVENT: u32 = 1175u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMCUSTOMEVENT: u32 = 1200u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMDRAGEVENT: u32 = 1400u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMEVENT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMEXCEPTION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMFOCUSEVENT: u32 = 1250u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMIMPLEMENTATION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMKEYBOARDEVENT: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMMESSAGEEVENT: u32 = 1325u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMMOUSEEVENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMMOUSEWHEELEVENT: u32 = 1075u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMMSANIMATIONEVENT: u32 = 1500u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMMSGESTUREEVENT: u32 = 1450u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMMSMANIPULATIONEVENT: u32 = 1525u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMMSPOINTEREVENT: u32 = 1425u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMMSTRANSITIONEVENT: u32 = 1475u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMMUTATIONEVENT: u32 = 1225u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMPARSER: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMPROGRESSEVENT: u32 = 1550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMRANGE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMSITEMODEEVENT: u32 = 1300u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMSTORAGE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMSTORAGEEVENT: u32 = 1350u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMSTORAGEITEM: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMSTORAGELIST: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMTEXTEVENT: u32 = 1125u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMTEXTNODE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMTRAVERSAL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMUIEVENT: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOMWHEELEVENT: u32 = 1100u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOUBLECLICK: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOWNLOADBEGIN: u32 = 106u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DOWNLOADCOMPLETE: u32 = 104u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DWEBBRIDGEEVENTS_ONCLICK: i32 = -600i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DWEBBRIDGEEVENTS_ONDBLCLICK: i32 = -601i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DWEBBRIDGEEVENTS_ONKEYDOWN: i32 = -602i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DWEBBRIDGEEVENTS_ONKEYPRESS: i32 = -603i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DWEBBRIDGEEVENTS_ONKEYUP: i32 = -604i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DWEBBRIDGEEVENTS_ONMOUSEDOWN: i32 = -605i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DWEBBRIDGEEVENTS_ONMOUSEMOVE: i32 = -606i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DWEBBRIDGEEVENTS_ONMOUSEUP: i32 = -607i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DWEBBRIDGEEVENTS_ONREADYSTATECHANGE: i32 = -609i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_DWEBBRIDGEEVENTS_ONSCRIPTLETEVENT: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EFONT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ELEMENT: u32 = 66536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ENABLENOTIFICATIONQUEUE: u32 = 72u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ENABLENOTIFICATIONQUEUELARGE: u32 = 78u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ENABLENOTIFICATIONQUEUESQUARE: u32 = 76u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ENABLENOTIFICATIONQUEUEWIDE: u32 = 77u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ENABLESUGGESTEDSITES: u32 = 39u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ENUMOPTIONS: u32 = 14u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVENTEXCEPTION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVENTHOOK_INSENSITIVE_BASE: u32 = 4500000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVENTHOOK_INSENSITIVE_MAX: u32 = 4999999u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVENTHOOK_SENSITIVE_BASE: u32 = 4000000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVENTHOOK_SENSITIVE_MAX: u32 = 4499999u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVENTOBJ: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVENTS: u32 = 71536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONABORT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONACTIVATE: u32 = 1044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONAFTERPRINT: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONAFTERUPDATE: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONALERT: u32 = 1061u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONBEFOREACTIVATE: u32 = 1047u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONBEFORECOPY: u32 = 65566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONBEFORECUT: u32 = 65565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONBEFOREDEACTIVATE: u32 = 1034u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONBEFOREEDITFOCUS: u32 = 1027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONBEFOREPASTE: u32 = 65567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONBEFOREPRINT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONBEFOREUNLOAD: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONBEFOREUPDATE: u32 = 65540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONBOUNCE: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONCELLCHANGE: u32 = 65570u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONCHANGE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONCHANGEBLUR: u32 = 1019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONCHANGEFOCUS: u32 = 1018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONCLICK: i32 = -600i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONCONTENTREADY: u32 = 1029u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONCONTEXTMENU: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONCONTROLSELECT: u32 = 1036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONCOPY: u32 = 65563u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONCUT: u32 = 65562u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONDATAAVAILABLE: u32 = 65551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONDATASETCHANGED: u32 = 65550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONDATASETCOMPLETE: u32 = 65552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONDBLCLICK: i32 = -601i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONDEACTIVATE: u32 = 1045u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONDOMMUTATION: u32 = 1068u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONDRAG: u32 = 65556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONDRAGEND: u32 = 65557u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONDRAGENTER: u32 = 65558u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONDRAGLEAVE: u32 = 65560u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONDRAGOVER: u32 = 65559u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONDRAGSTART: u32 = 65547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONDROP: u32 = 65561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONERRORUPDATE: u32 = 65549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONFILTER: u32 = 65553u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONFINISH: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONFOCUS: u32 = 65537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONFOCUSIN: u32 = 1048u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONFOCUSOUT: u32 = 1049u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONHASHCHANGE: u32 = 1066u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONHELP: u32 = 65546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONHIDE: u32 = 1060u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONKEYDOWN: i32 = -602i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONKEYPRESS: i32 = -603i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONKEYUP: i32 = -604i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONLAYOUT: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONLAYOUTCOMPLETE: u32 = 1030u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONLINKEDOVERFLOW: u32 = 1032u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONLOSECAPTURE: u32 = 65554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONMESSAGE: u32 = 1067u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONMOUSEDOWN: i32 = -605i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONMOUSEENTER: u32 = 1042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONMOUSEHOVER: u32 = 1028u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONMOUSELEAVE: u32 = 1043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONMOUSEMOVE: i32 = -606i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONMOUSEOUT: u32 = 65545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONMOUSEOVER: u32 = 65544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONMOUSEUP: i32 = -607i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONMOUSEWHEEL: u32 = 1033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONMOVE: u32 = 1035u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONMOVEEND: u32 = 1039u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONMOVESTART: u32 = 1038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONMULTILAYOUTCLEANUP: u32 = 1046u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONOBJECTCONTENTSCROLLED: u32 = 1056u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONOFFLINE: u32 = 1065u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONONLINE: u32 = 1064u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONPAGE: u32 = 1031u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONPASTE: u32 = 65564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONPERSISTLOAD: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONPERSISTSAVE: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONPOPUPMENUEND: u32 = 1063u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONPOPUPMENUSTART: u32 = 1062u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONPROPERTYCHANGE: u32 = 65555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONREADYSTATECHANGE: i32 = -609i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONRESET: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONRESIZE: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONRESIZEEND: u32 = 1041u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONRESIZESTART: u32 = 1040u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONROWENTER: u32 = 65543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONROWEXIT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONROWSDELETE: u32 = 65568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONROWSINSERTED: u32 = 65569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONSCROLL: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONSELECT: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONSELECTADD: u32 = 1051u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONSELECTIONCHANGE: u32 = 1037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONSELECTREMOVE: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONSELECTSTART: u32 = 65548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONSELECTWITHIN: u32 = 1053u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONSHOW: u32 = 1059u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONSTART: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONSTOP: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONSTORAGE: u32 = 1057u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONSTORAGECOMMIT: u32 = 1058u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONSUBMIT: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONSYSTEMSCROLLINGEND: u32 = 1055u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONSYSTEMSCROLLINGSTART: u32 = 1054u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONUNLOAD: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVMETH_ONVALUECHANGE: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROPS_COUNT: u32 = 260u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ADDTRACK: u32 = 71736u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_CACHED: u32 = 71721u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_CANPLAY: u32 = 71670u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_CANPLAYTHROUGH: u32 = 71671u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_CHECKING: u32 = 71717u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_COMPOSITIONEND: u32 = 71660u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_COMPOSITIONSTART: u32 = 71658u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_COMPOSITIONUPDATE: u32 = 71659u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_DOMATTRMODIFIED: u32 = 71661u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_DOMCHARDATAMODIFIED: u32 = 71664u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_DOMCONTENTLOADED: u32 = 71662u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_DOMNODEINSERTED: u32 = 71667u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_DOMNODEREMOVED: u32 = 71668u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_DOMSUBTREEMODIFIED: u32 = 71669u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_DOWNLOADING: u32 = 71719u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_DURATIONCHANGE: u32 = 71672u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_EMPTIED: u32 = 71673u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ENDED: u32 = 71674u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_HTML5ONREADYSTATECHANGE: u32 = 71780u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_INPUT: u32 = 71663u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_INVALID: u32 = 71724u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_LOADEDDATA: u32 = 71675u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_LOADEDMETADATA: u32 = 71676u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_LOADEND: u32 = 71723u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_LOADSTART: u32 = 71677u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSCONNECT: u32 = 71697u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSDISCONNECT: u32 = 71698u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSELEMENTRESIZE: u32 = 71742u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONCONTAINSFULLSCREENELEMENTCHANGED: u32 = 71783u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONCONTENTLOADING: u32 = 71753u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONDOMCONTENTLOADED: u32 = 71752u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONFRAMECONTENTLOADING: u32 = 71757u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONFRAMEDOMCONTENTLOADED: u32 = 71756u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONFRAMENAVIGATIONCOMPLETED: u32 = 71759u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONFRAMENAVIGATIONSTARTING: u32 = 71758u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONLONGRUNNINGSCRIPTDETECTED: u32 = 71763u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONNAVIGATIONCOMPLETED: u32 = 71755u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONNAVIGATIONSTARTING: u32 = 71754u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONSCRIPTNOTIFY: u32 = 71760u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONUNSAFECONTENTWARNINGDISPLAYING: u32 = 71762u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSHTMLWEBVIEW_ONUNVIEWABLECONTENT: u32 = 71761u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_MSORIENTATIONCHANGE: u32 = 71772u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_NOUPDATE: u32 = 71718u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_OBSOLETE: u32 = 71722u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONABORT: u32 = 71564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONACTIVATE: u32 = 71623u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONADDSOURCEBUFFER: u32 = 71746u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONAFTERPRINT: u32 = 71603u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONAFTERUPDATE: u32 = 71558u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONALERT: u32 = 71640u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONANIMATIONEND: u32 = 71712u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONANIMATIONITERATION: u32 = 71713u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONANIMATIONSTART: u32 = 71711u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONATTACHEVENT: u32 = 71606u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONBEFOREACTIVATE: u32 = 71626u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONBEFORECOPY: u32 = 71595u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONBEFORECUT: u32 = 71594u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONBEFOREDEACTIVATE: u32 = 71613u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONBEFOREDRAGOVER: u32 = 71559u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONBEFOREDROPORPASTE: u32 = 71560u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONBEFOREEDITFOCUS: u32 = 71605u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONBEFOREPASTE: u32 = 71596u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONBEFOREPRINT: u32 = 71602u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONBEFOREUNLOAD: u32 = 71575u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONBEFOREUPDATE: u32 = 71557u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONBLOCKED: u32 = 71726u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONBLUR: u32 = 71551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONBOUNCE: u32 = 71556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONCELLCHANGE: u32 = 71600u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONCHANGE: u32 = 71566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONCHANGEBLUR: u32 = 71581u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONCHANGEFOCUS: u32 = 71580u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONCLICK: u32 = 71544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONCLOSE: u32 = 71716u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONCOMPASSNEEDSCALIBRATION: u32 = 71782u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONCOMPLETE: u32 = 71727u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONCONTENTREADY: u32 = 71608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONCONTEXTMENU: u32 = 71601u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONCONTROLSELECT: u32 = 71615u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONCOPY: u32 = 71592u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONCUECHANGE: u32 = 71729u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONCUT: u32 = 71591u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDATAAVAILABLE: u32 = 71577u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDATASETCHANGED: u32 = 71576u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDATASETCOMPLETE: u32 = 71578u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDBLCLICK: u32 = 71545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDEACTIVATE: u32 = 71624u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDEVICEMOTION: u32 = 71774u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDEVICEORIENTATION: u32 = 71773u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDOMFOCUSIN: u32 = 71793u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDOMFOCUSOUT: u32 = 71794u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDOMMUTATION: u32 = 71647u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDRAG: u32 = 71585u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDRAGEND: u32 = 71586u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDRAGENTER: u32 = 71587u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDRAGLEAVE: u32 = 71589u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDRAGOVER: u32 = 71588u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDRAGSTART: u32 = 71571u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONDROP: u32 = 71590u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONENTER: u32 = 71730u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONERROR: u32 = 71565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONERRORUPDATE: u32 = 71574u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONEXIT: u32 = 71731u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONFILTER: u32 = 71579u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONFINISH: u32 = 71562u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONFOCUS: u32 = 71550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONFOCUSIN: u32 = 71627u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONFOCUSOUT: u32 = 71628u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONHASHCHANGE: u32 = 71645u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONHELP: u32 = 71549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONHIDE: u32 = 71639u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONKEYDOWN: u32 = 71541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONKEYPRESS: u32 = 71543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONKEYUP: u32 = 71542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONLAYOUT: u32 = 71570u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONLAYOUTCOMPLETE: u32 = 71609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONLINKEDOVERFLOW: u32 = 71611u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONLOAD: u32 = 71568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONLOSECAPTURE: u32 = 71582u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMESSAGE: u32 = 71646u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMOUSEDOWN: u32 = 71538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMOUSEENTER: u32 = 71621u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMOUSEHOVER: u32 = 71607u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMOUSELEAVE: u32 = 71622u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMOUSEMOVE: u32 = 71540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMOUSEOUT: u32 = 71537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMOUSEOVER: u32 = 71536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMOUSEUP: u32 = 71539u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMOUSEWHEEL: u32 = 71612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMOVE: u32 = 71614u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMOVEEND: u32 = 71618u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMOVESTART: u32 = 71617u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSCANDIDATEWINDOWHIDE: u32 = 71779u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSCANDIDATEWINDOWSHOW: u32 = 71777u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSCANDIDATEWINDOWUPDATE: u32 = 71778u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSCONTENTZOOM: u32 = 71708u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSFULLSCREENCHANGE: u32 = 71740u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSFULLSCREENERROR: u32 = 71741u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSGESTURECHANGE: u32 = 71700u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSGESTUREDOUBLETAP: u32 = 71704u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSGESTUREEND: u32 = 71701u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSGESTUREHOLD: u32 = 71702u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSGESTURESTART: u32 = 71699u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSGESTURETAP: u32 = 71703u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSGOTPOINTERCAPTURE: u32 = 71707u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSHOLDVISUAL: u32 = 71738u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSINERTIASTART: u32 = 71705u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSKEYADDED: u32 = 71751u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSKEYERROR: u32 = 71750u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSKEYMESSAGE: u32 = 71749u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSLOSTPOINTERCAPTURE: u32 = 71706u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSMANIPULATIONSTATECHANGED: u32 = 71714u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSNEEDKEY: u32 = 71748u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSPOINTERCANCEL: u32 = 71695u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSPOINTERDOWN: u32 = 71690u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSPOINTERENTER: u32 = 71769u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSPOINTERHOVER: u32 = 71696u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSPOINTERLEAVE: u32 = 71770u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSPOINTERMOVE: u32 = 71691u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSPOINTEROUT: u32 = 71694u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSPOINTEROVER: u32 = 71693u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSPOINTERUP: u32 = 71692u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSREGIONUPDATE: u32 = 71733u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSSITEMODEJUMPLISTITEMREMOVED: u32 = 71666u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSSITEPINNED: u32 = 71771u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSTHUMBNAILCLICK: u32 = 71657u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSVIDEOFORMATCHANGED: u32 = 71735u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSVIDEOFRAMESTEPCOMPLETED: u32 = 71737u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMSVIDEOOPTIMALLAYOUTCHANGED: u32 = 71739u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONMULTILAYOUTCLEANUP: u32 = 71625u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONOBJECTCONTENTSCROLLED: u32 = 71635u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONOFFLINE: u32 = 71644u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONONLINE: u32 = 71643u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONOPEN: u32 = 71715u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONPAGE: u32 = 71610u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONPAGEHIDE: u32 = 71776u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONPAGESHOW: u32 = 71775u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONPASTE: u32 = 71593u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONPERSISTLOAD: u32 = 71597u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONPERSISTSAVE: u32 = 71584u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONPOPSTATE: u32 = 71728u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONPOPUPMENUEND: u32 = 71642u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONPOPUPMENUSTART: u32 = 71641u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONPROPERTYCHANGE: u32 = 71583u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONREADYSTATECHANGE: u32 = 71561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONREMOVESOURCEBUFFER: u32 = 71747u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONRESET: u32 = 71548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONRESIZE: u32 = 71572u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONRESIZEEND: u32 = 71620u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONRESIZESTART: u32 = 71619u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONROWENTER: u32 = 71555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONROWEXIT: u32 = 71554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONROWSDELETE: u32 = 71598u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONROWSINSERTED: u32 = 71599u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSCROLL: u32 = 71567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSELECT: u32 = 71546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSELECTADD: u32 = 71630u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSELECTIONCHANGE: u32 = 71616u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSELECTREMOVE: u32 = 71631u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSELECTSTART: u32 = 71573u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSELECTWITHIN: u32 = 71632u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSHOW: u32 = 71638u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSOURCECLOSE: u32 = 71744u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSOURCEENDED: u32 = 71745u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSOURCEOPEN: u32 = 71743u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSTART: u32 = 71563u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSTOP: u32 = 71604u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSTORAGE: u32 = 71636u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSTORAGECOMMIT: u32 = 71637u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSUBMIT: u32 = 71547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSUCCESS: u32 = 71725u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSYSTEMSCROLLINGEND: u32 = 71634u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONSYSTEMSCROLLINGSTART: u32 = 71633u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONTOUCHCANCEL: u32 = 71787u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONTOUCHEND: u32 = 71785u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONTOUCHMOVE: u32 = 71786u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONTOUCHSTART: u32 = 71784u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONTRANSITIONEND: u32 = 71710u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONTRANSITIONSTART: u32 = 71709u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONUNLOAD: u32 = 71569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONUPDATE: u32 = 71767u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONUPDATEEND: u32 = 71768u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONUPDATESTART: u32 = 71766u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONUPGRADENEEDED: u32 = 71734u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONVALUECHANGE: u32 = 71629u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONWEBKITANIMATIONEND: u32 = 71790u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONWEBKITANIMATIONITERATION: u32 = 71791u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONWEBKITANIMATIONSTART: u32 = 71789u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ONWEBKITTRANSITIONEND: u32 = 71788u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_ORIENTATIONCHANGE: u32 = 71795u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_PAUSE: u32 = 71678u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_PLAY: u32 = 71679u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_PLAYING: u32 = 71680u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_PROGRESS: u32 = 71681u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_RATECHANGE: u32 = 71682u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_REMOVETRACK: u32 = 71781u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_SEEKED: u32 = 71683u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_SEEKING: u32 = 71684u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_SINKLIMIT: u32 = 71647u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_STALLED: u32 = 71685u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_SUSPEND: u32 = 71686u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_SVGABORT: u32 = 71652u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_SVGERROR: u32 = 71653u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_SVGLOAD: u32 = 71650u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_SVGRESIZE: u32 = 71654u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_SVGSCROLL: u32 = 71655u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_SVGUNLOAD: u32 = 71651u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_SVGZOOM: u32 = 71656u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_TEXTINPUT: u32 = 71665u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_TIMEOUT: u32 = 71648u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_TIMEUPDATE: u32 = 71687u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_UPDATEREADY: u32 = 71720u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_VISIBILITYCHANGE: u32 = 71732u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_VOLUMECHANGE: u32 = 71688u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_WAITING: u32 = 71689u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_WEBGLCONTEXTCREATIONERROR: u32 = 71792u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_WEBGLCONTEXTLOST: u32 = 71764u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_WEBGLCONTEXTRESTORED: u32 = 71765u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EVPROP_WHEEL: u32 = 71649u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EXPAND: u32 = 25u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EXPANDO_BASE: u32 = 3000000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EXPANDO_MAX: u32 = 3999999u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_EXPORT: u32 = 7u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_FAVSELECTIONCHANGE: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_FILEDOWNLOAD: u32 = 270u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_FILTERS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_FLAGS: u32 = 19u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_FORM: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_FRAME: u32 = 69536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_FRAMEBEFORENAVIGATE: u32 = 200u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_FRAMENAVIGATECOMPLETE: u32 = 201u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_FRAMENEWWINDOW: u32 = 204u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_FRAMESCOLLECTION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_FRAMESET: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_FRAMESITE: u32 = 68536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GENERIC: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETALWAYSSHOWLOCKSTATE: u32 = 23u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETCVLISTDATA: u32 = 93u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETCVLISTLOCALDATA: u32 = 94u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETDETAILSSTATE: u32 = 19u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETEMIELISTDATA: u32 = 95u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETEMIELISTLOCALDATA: u32 = 96u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETERRORCHAR: u32 = 15u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETERRORCODE: u32 = 16u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETERRORLINE: u32 = 14u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETERRORMSG: u32 = 17u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETERRORURL: u32 = 18u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETEXPERIMENTALFLAG: u32 = 85u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETEXPERIMENTALVALUE: u32 = 87u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETNEEDHVSIAUTOLAUNCHFLAG: u32 = 100u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETNEEDIEAUTOLAUNCHFLAG: u32 = 89u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETOSSKU: u32 = 103u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_GETPERERRSTATE: u32 = 21u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HASNEEDHVSIAUTOLAUNCHFLAG: u32 = 102u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HASNEEDIEAUTOLAUNCHFLAG: u32 = 88u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HEADER: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HEDELEMS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HISTORY: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HISTORYOBJECT: i32 = -5507i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HR: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTML5ATTRIBUTESELECTORCI: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLAPP: u32 = 5000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDLG: u32 = 25000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDLGMODEL: u32 = 26000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONACTIVATE: u32 = 1044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONAFTERUPDATE: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONBEFOREACTIVATE: u32 = 1047u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONBEFOREDEACTIVATE: u32 = 1034u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONBEFOREEDITFOCUS: u32 = 1027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONBEFOREUPDATE: u32 = 65540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONCELLCHANGE: u32 = 65570u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONCLICK: i32 = -600i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONCONTEXTMENU: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONCONTROLSELECT: u32 = 1036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONDATAAVAILABLE: u32 = 65551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONDATASETCHANGED: u32 = 65550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONDATASETCOMPLETE: u32 = 65552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONDBLCLICK: i32 = -601i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONDEACTIVATE: u32 = 1045u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONDRAGSTART: u32 = 65547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONERRORUPDATE: u32 = 65549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONFOCUSIN: u32 = 1048u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONFOCUSOUT: u32 = 1049u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONHELP: u32 = 65546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONKEYDOWN: i32 = -602i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONKEYPRESS: i32 = -603i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONKEYUP: i32 = -604i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONMOUSEDOWN: i32 = -605i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONMOUSEMOVE: i32 = -606i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONMOUSEOUT: u32 = 65545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONMOUSEOVER: u32 = 65544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONMOUSEUP: i32 = -607i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONMOUSEWHEEL: u32 = 1033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONPROPERTYCHANGE: u32 = 65555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONREADYSTATECHANGE: i32 = -609i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONROWENTER: u32 = 65543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONROWEXIT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONROWSDELETE: u32 = 65568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONROWSINSERTED: u32 = 65569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONSELECTIONCHANGE: u32 = 1037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONSELECTSTART: u32 = 65548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS2_ONSTOP: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS3_ONSTORAGE: u32 = 1057u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS3_ONSTORAGECOMMIT: u32 = 1058u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS4_ONMSSITEMODEJUMPLISTITEMREMOVED: u32 = 71666u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS4_ONMSTHUMBNAILCLICK: u32 = 71657u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONACTIVATE: u32 = 1044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONAFTERUPDATE: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONBEFOREACTIVATE: u32 = 1047u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONBEFOREDEACTIVATE: u32 = 1034u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONBEFOREEDITFOCUS: u32 = 1027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONBEFOREUPDATE: u32 = 65540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONCELLCHANGE: u32 = 65570u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONCLICK: i32 = -600i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONCONTEXTMENU: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONCONTROLSELECT: u32 = 1036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONDATAAVAILABLE: u32 = 65551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONDATASETCHANGED: u32 = 65550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONDATASETCOMPLETE: u32 = 65552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONDBLCLICK: i32 = -601i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONDEACTIVATE: u32 = 1045u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONDRAGSTART: u32 = 65547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONERRORUPDATE: u32 = 65549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONFOCUSIN: u32 = 1048u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONFOCUSOUT: u32 = 1049u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONHELP: u32 = 65546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONKEYDOWN: i32 = -602i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONKEYPRESS: i32 = -603i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONKEYUP: i32 = -604i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONMOUSEDOWN: i32 = -605i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONMOUSEMOVE: i32 = -606i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONMOUSEOUT: u32 = 65545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONMOUSEOVER: u32 = 65544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONMOUSEUP: i32 = -607i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONMOUSEWHEEL: u32 = 1033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONPROPERTYCHANGE: u32 = 65555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONREADYSTATECHANGE: i32 = -609i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONROWENTER: u32 = 65543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONROWEXIT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONROWSDELETE: u32 = 65568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONROWSINSERTED: u32 = 65569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONSELECTIONCHANGE: u32 = 1037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONSELECTSTART: u32 = 65548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLDOCUMENTEVENTS_ONSTOP: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONACTIVATE: u32 = 1044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONAFTERUPDATE: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONBEFOREACTIVATE: u32 = 1047u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONBEFORECOPY: u32 = 65566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONBEFORECUT: u32 = 65565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONBEFOREDEACTIVATE: u32 = 1034u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONBEFOREPASTE: u32 = 65567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONBEFOREUPDATE: u32 = 65540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONCELLCHANGE: u32 = 65570u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONCLICK: i32 = -600i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONCONTEXTMENU: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONCONTROLSELECT: u32 = 1036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONCOPY: u32 = 65563u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONCUT: u32 = 65562u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONDATAAVAILABLE: u32 = 65551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONDATASETCHANGED: u32 = 65550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONDATASETCOMPLETE: u32 = 65552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONDBLCLICK: i32 = -601i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONDEACTIVATE: u32 = 1045u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONDRAG: u32 = 65556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONDRAGEND: u32 = 65557u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONDRAGENTER: u32 = 65558u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONDRAGLEAVE: u32 = 65560u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONDRAGOVER: u32 = 65559u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONDRAGSTART: u32 = 65547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONDROP: u32 = 65561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONERRORUPDATE: u32 = 65549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONFILTERCHANGE: u32 = 65553u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONFOCUS: u32 = 65537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONFOCUSIN: u32 = 1048u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONFOCUSOUT: u32 = 1049u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONHELP: u32 = 65546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONKEYDOWN: i32 = -602i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONKEYPRESS: i32 = -603i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONKEYUP: i32 = -604i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONLAYOUTCOMPLETE: u32 = 1030u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONLOSECAPTURE: u32 = 65554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONMOUSEDOWN: i32 = -605i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONMOUSEENTER: u32 = 1042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONMOUSELEAVE: u32 = 1043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONMOUSEMOVE: i32 = -606i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONMOUSEOUT: u32 = 65545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONMOUSEOVER: u32 = 65544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONMOUSEUP: i32 = -607i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONMOUSEWHEEL: u32 = 1033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONMOVE: u32 = 1035u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONMOVEEND: u32 = 1039u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONMOVESTART: u32 = 1038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONPAGE: u32 = 1031u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONPASTE: u32 = 65564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONPROPERTYCHANGE: u32 = 65555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONREADYSTATECHANGE: i32 = -609i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONRESIZE: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONRESIZEEND: u32 = 1041u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONRESIZESTART: u32 = 1040u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONROWENTER: u32 = 65543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONROWEXIT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONROWSDELETE: u32 = 65568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONROWSINSERTED: u32 = 65569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONSCROLL: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS2_ONSELECTSTART: u32 = 65548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS3_ONOFFLINE: u32 = 1065u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS3_ONONLINE: u32 = 1064u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS4_ONABORT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS4_ONCHANGE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS4_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS4_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS4_ONMSCONTENTZOOM: u32 = 71708u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS4_ONRESET: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS4_ONSELECT: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS4_ONSUBMIT: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONACTIVATE: u32 = 1044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONAFTERUPDATE: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONBEFOREACTIVATE: u32 = 1047u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONBEFORECOPY: u32 = 65566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONBEFORECUT: u32 = 65565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONBEFOREDEACTIVATE: u32 = 1034u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONBEFOREEDITFOCUS: u32 = 1027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONBEFOREPASTE: u32 = 65567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONBEFOREUPDATE: u32 = 65540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONCELLCHANGE: u32 = 65570u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONCLICK: i32 = -600i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONCONTEXTMENU: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONCONTROLSELECT: u32 = 1036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONCOPY: u32 = 65563u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONCUT: u32 = 65562u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONDATAAVAILABLE: u32 = 65551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONDATASETCHANGED: u32 = 65550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONDATASETCOMPLETE: u32 = 65552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONDBLCLICK: i32 = -601i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONDEACTIVATE: u32 = 1045u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONDRAG: u32 = 65556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONDRAGEND: u32 = 65557u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONDRAGENTER: u32 = 65558u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONDRAGLEAVE: u32 = 65560u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONDRAGOVER: u32 = 65559u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONDRAGSTART: u32 = 65547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONDROP: u32 = 65561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONERRORUPDATE: u32 = 65549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONFILTERCHANGE: u32 = 65553u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONFOCUS: u32 = 65537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONFOCUSIN: u32 = 1048u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONFOCUSOUT: u32 = 1049u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONHELP: u32 = 65546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONKEYDOWN: i32 = -602i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONKEYPRESS: i32 = -603i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONKEYUP: i32 = -604i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONLAYOUTCOMPLETE: u32 = 1030u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONLOSECAPTURE: u32 = 65554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONMOUSEDOWN: i32 = -605i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONMOUSEENTER: u32 = 1042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONMOUSELEAVE: u32 = 1043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONMOUSEMOVE: i32 = -606i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONMOUSEOUT: u32 = 65545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONMOUSEOVER: u32 = 65544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONMOUSEUP: i32 = -607i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONMOUSEWHEEL: u32 = 1033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONMOVE: u32 = 1035u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONMOVEEND: u32 = 1039u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONMOVESTART: u32 = 1038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONPAGE: u32 = 1031u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONPASTE: u32 = 65564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONPROPERTYCHANGE: u32 = 65555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONREADYSTATECHANGE: i32 = -609i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONRESIZE: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONRESIZEEND: u32 = 1041u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONRESIZESTART: u32 = 1040u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONROWENTER: u32 = 65543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONROWEXIT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONROWSDELETE: u32 = 65568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONROWSINSERTED: u32 = 65569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONSCROLL: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLELEMENTEVENTS_ONSELECTSTART: u32 = 65548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLFORMELEMENTEVENTS2_ONRESET: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLFORMELEMENTEVENTS2_ONSUBMIT: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLFORMELEMENTEVENTS_ONRESET: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLFORMELEMENTEVENTS_ONSUBMIT: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLFRAMESITEEVENTS2_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLFRAMESITEEVENTS_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLIMGEVENTS2_ONABORT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLIMGEVENTS2_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLIMGEVENTS2_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLIMGEVENTS_ONABORT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLIMGEVENTS_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLIMGEVENTS_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLINPUTIMAGEEVENTS2_ONABORT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLINPUTIMAGEEVENTS2_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLINPUTIMAGEEVENTS2_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLINPUTIMAGEEVENTS_ONABORT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLINPUTIMAGEEVENTS_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLINPUTIMAGEEVENTS_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS2_ONABORT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS2_ONCHANGE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS2_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS2_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS2_ONSELECT: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS_ONABORT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS_ONCHANGE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLINPUTTEXTELEMENTEVENTS_ONSELECT: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLLINKELEMENTEVENTS2_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLLINKELEMENTEVENTS2_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLLINKELEMENTEVENTS_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLLINKELEMENTEVENTS_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLMARQUEEELEMENTEVENTS2_ONBOUNCE: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLMARQUEEELEMENTEVENTS2_ONFINISH: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLMARQUEEELEMENTEVENTS2_ONSTART: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLMARQUEEELEMENTEVENTS_ONBOUNCE: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLMARQUEEELEMENTEVENTS_ONFINISH: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLMARQUEEELEMENTEVENTS_ONSTART: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLNAMESPACEEVENTS_ONREADYSTATECHANGE: i32 = -609i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECT: u32 = 66036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONAFTERUPDATE: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONBEFOREUPDATE: u32 = 65540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONCELLCHANGE: u32 = 65570u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONDATAAVAILABLE: u32 = 65551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONDATASETCHANGED: u32 = 65550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONDATASETCOMPLETE: u32 = 65552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONERROR: u32 = 65555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONERRORUPDATE: u32 = 65549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONREADYSTATECHANGE: u32 = 65556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONROWENTER: u32 = 65543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONROWEXIT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONROWSDELETE: u32 = 65568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS2_ONROWSINSERTED: u32 = 65569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONAFTERUPDATE: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONBEFOREUPDATE: u32 = 65540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONCELLCHANGE: u32 = 65570u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONDATAAVAILABLE: u32 = 65551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONDATASETCHANGED: u32 = 65550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONDATASETCOMPLETE: u32 = 65552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONERROR: u32 = 65555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONERRORUPDATE: u32 = 65549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONREADYSTATECHANGE: u32 = 65556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONROWENTER: u32 = 65543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONROWEXIT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONROWSDELETE: u32 = 65568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLOBJECTELEMENTEVENTS_ONROWSINSERTED: u32 = 65569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLPOPUP: u32 = 27000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLSCRIPTEVENTS2_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLSCRIPTEVENTS_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLSELECTELEMENTEVENTS2_ONCHANGE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLSELECTELEMENTEVENTS_ONCHANGE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLSELECTION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLSTYLEELEMENTEVENTS2_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLSTYLEELEMENTEVENTS2_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLSTYLEELEMENTEVENTS_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLSTYLEELEMENTEVENTS_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLTEXTCONTAINEREVENTS2_ONCHANGE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLTEXTCONTAINEREVENTS2_ONSELECT: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLTEXTCONTAINEREVENTS_ONCHANGE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLTEXTCONTAINEREVENTS_ONSELECT: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS2_ONAFTERPRINT: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS2_ONBEFOREPRINT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS2_ONBEFOREUNLOAD: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS2_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS2_ONFOCUS: u32 = 65537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS2_ONHELP: u32 = 65546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS2_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS2_ONRESIZE: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS2_ONSCROLL: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS2_ONUNLOAD: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS3_ONHASHCHANGE: u32 = 1066u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS3_ONMESSAGE: u32 = 1067u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS_ONAFTERPRINT: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS_ONBEFOREPRINT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS_ONBEFOREUNLOAD: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS_ONFOCUS: u32 = 65537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS_ONHELP: u32 = 65546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS_ONRESIZE: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS_ONSCROLL: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLWINDOWEVENTS_ONUNLOAD: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLXMLHTTPREQUESTEVENTS_ONREADYSTATECHANGE: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_HTMLXMLHTTPREQUESTEVENTS_ONTIMEOUT: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IBLOCKFORMATS_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IBLOCKFORMATS_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IBLOCKFORMATS__NEWENUM: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASGRADIENT_ADDCOLORSTOP: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASIMAGEDATA_DATA: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASIMAGEDATA_HEIGHT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASIMAGEDATA_WIDTH: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASPIXELARRAY_LENGTH: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_ARC: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_ARCTO: u32 = 1027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_BEGINPATH: u32 = 1028u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_BEZIERCURVETO: u32 = 1029u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_CANVAS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_CLEARRECT: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_CLIP: u32 = 1030u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_CLOSEPATH: u32 = 1031u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_CREATEIMAGEDATA: u32 = 1046u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_CREATELINEARGRADIENT: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_CREATEPATTERN: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_CREATERADIALGRADIENT: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_DRAWIMAGE: u32 = 1045u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_FILL: u32 = 1032u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_FILLRECT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_FILLSTYLE: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_FILLTEXT: u32 = 1042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_FONT: u32 = 1039u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_GETIMAGEDATA: u32 = 1047u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_GLOBALALPHA: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_GLOBALCOMPOSITEOPERATION: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_ISPOINTINPATH: u32 = 1038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_LINECAP: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_LINEJOIN: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_LINETO: u32 = 1033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_LINEWIDTH: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_MEASURETEXT: u32 = 1043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_MITERLIMIT: u32 = 1018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_MOVETO: u32 = 1034u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_PUTIMAGEDATA: u32 = 1048u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_QUADRATICCURVETO: u32 = 1035u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_RECT: u32 = 1036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_RESTORE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_ROTATE: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_SAVE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_SCALE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_SETTRANSFORM: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_SHADOWBLUR: u32 = 1019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_SHADOWCOLOR: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_SHADOWOFFSETX: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_SHADOWOFFSETY: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_STROKE: u32 = 1037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_STROKERECT: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_STROKESTYLE: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_STROKETEXT: u32 = 1044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_TEXTALIGN: u32 = 1040u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_TEXTBASELINE: u32 = 1041u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_TRANSFORM: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASRENDERINGCONTEXT2D_TRANSLATE: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICANVASTEXTMETRICS_WIDTH: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_ADDCOMPONENTREQUEST: u32 = 19u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_AVAILHEIGHT: u32 = 13u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_AVAILWIDTH: u32 = 14u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_BUFFERDEPTH: u32 = 10u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_CLEARCOMPONENTREQUEST: u32 = 21u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_COLORDEPTH: u32 = 9u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_COMPAREVERSIONS: u32 = 18u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_CONNECTIONSPEED: u32 = 7u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_CONNECTIONTYPE: u32 = 15u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_COOKIEENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_CPUCLASS: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_DOCOMPONENTREQUEST: u32 = 20u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_GETCOMPONENTVERSION: u32 = 17u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_HEIGHT: u32 = 12u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_ISCOMPONENTINSTALLED: u32 = 16u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_JAVAENABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_ONLINE: u32 = 8u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_PLATFORM: u32 = 6u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_SYSTEMLANGUAGE: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_USERLANGUAGE: u32 = 5u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ICLIENTCAPS_WIDTH: u32 = 11u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOCUMENTEVENT_CREATEEVENT: u32 = 1108u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOCUMENTRANGE_CREATERANGE: u32 = 1111u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOCUMENTSELECTOR_QUERYSELECTOR: u32 = 1105u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOCUMENTSELECTOR_QUERYSELECTORALL: u32 = 1106u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOCUMENTTRAVERSAL_CREATENODEITERATOR: u32 = 1121u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOCUMENTTRAVERSAL_CREATETREEWALKER: u32 = 1122u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMBEFOREUNLOADEVENT_RETURNVALUE: u32 = 1376u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMCLOSEEVENT_INITCLOSEEVENT: u32 = 1529u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMCLOSEEVENT_WASCLEAN: u32 = 1526u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMCOMPOSITIONEVENT_DATA: u32 = 1176u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMCOMPOSITIONEVENT_INITCOMPOSITIONEVENT: u32 = 1177u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMCOMPOSITIONEVENT_LOCALE: u32 = 1178u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMCUSTOMEVENT_DETAIL: u32 = 1201u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMCUSTOMEVENT_INITCUSTOMEVENT: u32 = 1202u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMDOCUMENTTYPE_ENTITIES: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMDOCUMENTTYPE_INTERNALSUBSET: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMDOCUMENTTYPE_NAME: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMDOCUMENTTYPE_NOTATIONS: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMDOCUMENTTYPE_PUBLICID: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMDOCUMENTTYPE_SYSTEMID: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMDRAGEVENT_DATATRANSFER: u32 = 1401u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMDRAGEVENT_INITDRAGEVENT: u32 = 1402u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEVENT_BUBBLES: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEVENT_CANCELABLE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEVENT_CANCELBUBBLE: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEVENT_CURRENTTARGET: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEVENT_DEFAULTPREVENTED: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEVENT_EVENTPHASE: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEVENT_INITEVENT: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEVENT_ISTRUSTED: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEVENT_PREVENTDEFAULT: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEVENT_SRCELEMENT: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEVENT_STOPIMMEDIATEPROPAGATION: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEVENT_STOPPROPAGATION: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEVENT_TARGET: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEVENT_TIMESTAMP: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEVENT_TYPE: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEXCEPTION_CODE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMEXCEPTION_MESSAGE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMFOCUSEVENT_INITFOCUSEVENT: u32 = 1252u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMFOCUSEVENT_RELATEDTARGET: u32 = 1251u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMKEYBOARDEVENT_ALTKEY: u32 = 1155u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMKEYBOARDEVENT_CHARCODE: u32 = 1161u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMKEYBOARDEVENT_CTRLKEY: u32 = 1153u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMKEYBOARDEVENT_GETMODIFIERSTATE: u32 = 1158u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMKEYBOARDEVENT_IE9_CHAR: u32 = 1163u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMKEYBOARDEVENT_INITKEYBOARDEVENT: u32 = 1159u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMKEYBOARDEVENT_KEY: u32 = 1151u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMKEYBOARDEVENT_KEYCODE: u32 = 1160u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMKEYBOARDEVENT_LOCALE: u32 = 1164u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMKEYBOARDEVENT_LOCATION: u32 = 1152u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMKEYBOARDEVENT_METAKEY: u32 = 1156u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMKEYBOARDEVENT_REPEAT: u32 = 1157u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMKEYBOARDEVENT_SHIFTKEY: u32 = 1154u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMKEYBOARDEVENT_WHICH: u32 = 1162u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMESSAGEEVENT_DATA: u32 = 1326u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMESSAGEEVENT_INITMESSAGEEVENT: u32 = 1329u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMESSAGEEVENT_ORIGIN: u32 = 1327u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMESSAGEEVENT_SOURCE: u32 = 1328u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_ALTKEY: u32 = 1057u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_BUTTON: u32 = 1059u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_BUTTONS: u32 = 1063u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_CLIENTX: u32 = 1053u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_CLIENTY: u32 = 1054u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_CTRLKEY: u32 = 1055u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_FROMELEMENT: u32 = 1064u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_GETMODIFIERSTATE: u32 = 1062u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_INITMOUSEEVENT: u32 = 1061u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_LAYERX: u32 = 1072u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_LAYERY: u32 = 1073u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_METAKEY: u32 = 1058u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_OFFSETX: u32 = 1068u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_OFFSETY: u32 = 1069u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_PAGEX: u32 = 1070u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_PAGEY: u32 = 1071u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_RELATEDTARGET: u32 = 1060u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_SCREENX: u32 = 1051u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_SCREENY: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_SHIFTKEY: u32 = 1056u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_TOELEMENT: u32 = 1065u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_WHICH: u32 = 1074u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_X: u32 = 1066u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEEVENT_Y: u32 = 1067u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEWHEELEVENT_INITMOUSEWHEELEVENT: u32 = 1077u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMOUSEWHEELEVENT_WHEELDELTA: u32 = 1076u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMSANIMATIONEVENT_ANIMATIONNAME: u32 = 1501u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMSANIMATIONEVENT_ELAPSEDTIME: u32 = 1502u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMSANIMATIONEVENT_INITMSANIMATIONEVENT: u32 = 1503u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMSMANIPULATIONEVENT_CURRENTSTATE: u32 = 1527u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMSMANIPULATIONEVENT_INITMSMANIPULATIONEVENT: u32 = 1528u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMSMANIPULATIONEVENT_LASTSTATE: u32 = 1526u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMSTRANSITIONEVENT_ELAPSEDTIME: u32 = 1477u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMSTRANSITIONEVENT_INITMSTRANSITIONEVENT: u32 = 1478u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMSTRANSITIONEVENT_PROPERTYNAME: u32 = 1476u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMUTATIONEVENT_ATTRCHANGE: u32 = 1230u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMUTATIONEVENT_ATTRNAME: u32 = 1229u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMUTATIONEVENT_INITMUTATIONEVENT: u32 = 1231u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMUTATIONEVENT_NEWVALUE: u32 = 1228u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMUTATIONEVENT_PREVVALUE: u32 = 1227u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMMUTATIONEVENT_RELATEDNODE: u32 = 1226u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMNODEITERATOR_DETACH: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMNODEITERATOR_EXPANDENTITYREFERENCES: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMNODEITERATOR_FILTER: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMNODEITERATOR_NEXTNODE: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMNODEITERATOR_PREVIOUSNODE: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMNODEITERATOR_ROOT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMNODEITERATOR_WHATTOSHOW: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMPARSERFACTORY_CREATE: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMPARSER_PARSEFROMSTRING: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMPROCESSINGINSTRUCTION_DATA: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMPROCESSINGINSTRUCTION_TARGET: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMPROGRESSEVENT_INITPROGRESSEVENT: u32 = 1554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMPROGRESSEVENT_LENGTHCOMPUTABLE: u32 = 1551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMPROGRESSEVENT_LOADED: u32 = 1552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMPROGRESSEVENT_TOTAL: u32 = 1553u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMSITEMODEEVENT_ACTIONURL: u32 = 1302u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMSITEMODEEVENT_BUTTONID: u32 = 1301u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMSTORAGEEVENT_INITSTORAGEEVENT: u32 = 1356u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMSTORAGEEVENT_KEY: u32 = 1351u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMSTORAGEEVENT_NEWVALUE: u32 = 1353u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMSTORAGEEVENT_OLDVALUE: u32 = 1352u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMSTORAGEEVENT_STORAGEAREA: u32 = 1355u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMSTORAGEEVENT_URL: u32 = 1354u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMTEXTEVENT_DATA: u32 = 1126u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMTEXTEVENT_INITTEXTEVENT: u32 = 1128u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMTEXTEVENT_INPUTMETHOD: u32 = 1127u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMTEXTEVENT_LOCALE: u32 = 1129u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMTREEWALKER_CURRENTNODE: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMTREEWALKER_EXPANDENTITYREFERENCES: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMTREEWALKER_FILTER: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMTREEWALKER_FIRSTCHILD: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMTREEWALKER_LASTCHILD: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMTREEWALKER_NEXTNODE: u32 = 1027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMTREEWALKER_NEXTSIBLING: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMTREEWALKER_PARENTNODE: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMTREEWALKER_PREVIOUSNODE: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMTREEWALKER_PREVIOUSSIBLING: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMTREEWALKER_ROOT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMTREEWALKER_WHATTOSHOW: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMUIEVENT_DETAIL: u32 = 1027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMUIEVENT_INITUIEVENT: u32 = 1028u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMUIEVENT_VIEW: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMWHEELEVENT_DELTAMODE: u32 = 1104u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMWHEELEVENT_DELTAX: u32 = 1101u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMWHEELEVENT_DELTAY: u32 = 1102u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMWHEELEVENT_DELTAZ: u32 = 1103u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMWHEELEVENT_INITWHEELEVENT: u32 = 1105u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMXMLSERIALIZERFACTORY_CREATE: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IDOMXMLSERIALIZER_SERIALIZETOSTRING: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE10_ELEMENT: u32 = 66822u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE10_ELEMENTBASE: u32 = 66822u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_ANCHOR: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_AREA: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_ATTR: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_BASE: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_BLOCK: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_BODY: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_COLLECTION: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_ELEMENT: u32 = 66736u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_ELEMENTBASE: u32 = 66736u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_ELEMENTMAX: u32 = 66776u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_EMBED: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_FORM: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_FRAME: u32 = 69656u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_FRAMESITEBASE: u32 = 69656u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_HEAD: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_IFRAME: u32 = 69656u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_IMG: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_INPUT: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_LINK: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_META: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_MOD: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_NAMEDNODEMAP: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_NORMAL_FIRST: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_OBJECT: u32 = 68566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_OBJECTBASE: u32 = 68566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_PARAM: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_SCRIPT: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_SELECT: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE8_STYLE: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_ABORT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_ACTIVATE: u32 = 1044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_ADDSOURCEBUFFER: u32 = 71746u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_ADDTRACK: u32 = 71736u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_AFTERPRINT: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_ANIMATIONEND: u32 = 71712u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_ANIMATIONITERATION: u32 = 71713u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_ANIMATIONSTART: u32 = 71711u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_BEFOREACTIVATE: u32 = 1047u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_BEFORECOPY: u32 = 65566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_BEFORECUT: u32 = 65565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_BEFOREDEACTIVATE: u32 = 1034u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_BEFOREPASTE: u32 = 65567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_BEFOREPRINT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_BEFOREUNLOAD: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_BLOCKED: u32 = 71726u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_BOUNCE: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_CACHED: u32 = 71721u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_CANPLAY: u32 = 71670u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_CANPLAYTHROUGH: u32 = 71671u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_CHANGE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_CHECKING: u32 = 71717u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_CLICK: i32 = -600i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_CLOSE: u32 = 71716u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_COMPASSNEEDSCALIBRATION: u32 = 71782u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_COMPLETE: u32 = 71727u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_COMPOSITIONEND: u32 = 71660u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_COMPOSITIONSTART: u32 = 71658u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_COMPOSITIONUPDATE: u32 = 71659u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_CONTEXTMENU: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_COPY: u32 = 65563u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_CUECHANGE: u32 = 71729u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_CUT: u32 = 65562u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DBLCLICK: i32 = -601i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DEACTIVATE: u32 = 1045u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DEVICEMOTION: u32 = 71774u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DEVICEORIENTATION: u32 = 71773u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DOMATTRMODIFIED: u32 = 71661u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DOMCHARACTERDATAMODIFIED: u32 = 71664u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DOMCONTENTLOADED: u32 = 71662u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DOMNODEINSERTED: u32 = 71667u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DOMNODEREMOVED: u32 = 71668u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DOMSUBTREEMODIFIED: u32 = 71669u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DOWNLOADING: u32 = 71719u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DRAG: u32 = 65556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DRAGEND: u32 = 65557u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DRAGENTER: u32 = 65558u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DRAGLEAVE: u32 = 65560u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DRAGOVER: u32 = 65559u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DRAGSTART: u32 = 65547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DROP: u32 = 65561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_DURATIONCHANGE: u32 = 71672u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_EMPTIED: u32 = 71673u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_ENDED: u32 = 71674u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_ENTER: u32 = 71730u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_ERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_EXIT: u32 = 71731u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_FINISH: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_FOCUS: u32 = 65537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_FOCUSIN: u32 = 1048u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_FOCUSOUT: u32 = 1049u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_HASHCHANGE: u32 = 1066u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_HELP: u32 = 65546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_INPUT: u32 = 71663u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_INVALID: u32 = 71724u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_KEYDOWN: i32 = -602i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_KEYPRESS: i32 = -603i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_KEYUP: i32 = -604i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_LOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_LOADEDDATA: u32 = 71675u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_LOADEDMETADATA: u32 = 71676u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_LOADEND: u32 = 71723u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_LOADSTART: u32 = 71677u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MESSAGE: u32 = 1067u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MOUSEDOWN: i32 = -605i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MOUSEENTER: u32 = 1042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MOUSELEAVE: u32 = 1043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MOUSEMOVE: i32 = -606i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MOUSEOUT: u32 = 65545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MOUSEOVER: u32 = 65544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MOUSEUP: i32 = -607i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MOUSEWHEEL: u32 = 1033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSBEFOREEDITFOCUS: u32 = 1027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSCANDIDATEWINDOWHIDE: u32 = 71779u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSCANDIDATEWINDOWSHOW: u32 = 71777u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSCANDIDATEWINDOWUPDATE: u32 = 71778u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSCONTENTZOOM: u32 = 71708u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSCONTROLRESIZEEND: u32 = 1041u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSCONTROLRESIZESTART: u32 = 1040u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSCONTROLSELECT: u32 = 1036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSELEMENTRESIZE: u32 = 71742u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSFULLSCREENCHANGE: u32 = 71740u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSFULLSCREENERROR: u32 = 71741u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSGESTURECHANGE: u32 = 71700u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSGESTUREDOUBLETAP: u32 = 71704u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSGESTUREEND: u32 = 71701u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSGESTUREHOLD: u32 = 71702u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSGESTURESTART: u32 = 71699u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSGESTURETAP: u32 = 71703u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSGOTPOINTERCAPTURE: u32 = 71707u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSHOLDVISUAL: u32 = 71738u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSINERTIASTART: u32 = 71705u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSKEYADDED: u32 = 71751u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSKEYERROR: u32 = 71750u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSKEYMESSAGE: u32 = 71749u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSLOSTPOINTERCAPTURE: u32 = 71706u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSMANIPULATIONSTATECHANGED: u32 = 71714u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSNEEDKEY: u32 = 71748u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSORIENTATIONCHANGE: u32 = 71772u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSPOINTERCANCEL: u32 = 71695u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSPOINTERDOWN: u32 = 71690u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSPOINTERENTER: u32 = 71769u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSPOINTERHOVER: u32 = 71696u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSPOINTERLEAVE: u32 = 71770u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSPOINTERMOVE: u32 = 71691u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSPOINTEROUT: u32 = 71694u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSPOINTEROVER: u32 = 71693u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSPOINTERUP: u32 = 71692u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSREGIONUPDATE: u32 = 71733u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSSITEMODEJUMPLISTITEMREMOVED: u32 = 71666u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSSITEPINNED: u32 = 71771u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSTHUMBNAILCLICK: u32 = 71657u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSVIDEOFORMATCHANGED: u32 = 71735u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSVIDEOFRAMESTEPCOMPLETED: u32 = 71737u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSVIDEOOPTIMALLAYOUTCHANGED: u32 = 71739u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSWEBVIEWCONTAINSFULLSCREENELEMENTCHANGED: u32 = 71783u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSWEBVIEWCONTENTLOADING: u32 = 71753u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSWEBVIEWDOMCONTENTLOADED: u32 = 71752u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSWEBVIEWFRAMECONTENTLOADING: u32 = 71757u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSWEBVIEWFRAMEDOMCONTENTLOADED: u32 = 71756u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSWEBVIEWFRAMENAVIGATIONCOMPLETED: u32 = 71759u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSWEBVIEWFRAMENAVIGATIONSTARTING: u32 = 71758u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSWEBVIEWLONGRUNNINGSCRIPTDETECTED: u32 = 71763u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSWEBVIEWNAVIGATIONCOMPLETED: u32 = 71755u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSWEBVIEWNAVIGATIONSTARTING: u32 = 71754u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSWEBVIEWSCRIPTNOTIFY: u32 = 71760u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSWEBVIEWUNSAFECONTENTWARNINGDISPLAYING: u32 = 71762u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_MSWEBVIEWUNVIEWABLECONTENTIDENTIFIED: u32 = 71761u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_NOUPDATE: u32 = 71718u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_OBSOLETE: u32 = 71722u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_OFFLINE: u32 = 1065u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_ONLINE: u32 = 1064u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_OPEN: u32 = 71715u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_ORIENTATIONCHANGE: u32 = 71795u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_PAGEHIDE: u32 = 71776u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_PAGESHOW: u32 = 71775u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_PASTE: u32 = 65564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_PAUSE: u32 = 71678u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_PLAY: u32 = 71679u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_PLAYING: u32 = 71680u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_POPSTATE: u32 = 71728u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_PROGRESS: u32 = 71681u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_RATECHANGE: u32 = 71682u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_READYSTATECHANGE: i32 = -609i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_REMOVESOURCEBUFFER: u32 = 71747u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_REMOVETRACK: u32 = 71781u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_RESET: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_RESIZE: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SCROLL: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SEEKED: u32 = 71683u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SEEKING: u32 = 71684u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SELECT: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SELECTIONCHANGE: u32 = 1037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SELECTSTART: u32 = 65548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SOURCECLOSE: u32 = 71744u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SOURCEENDED: u32 = 71745u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SOURCEOPEN: u32 = 71743u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_STALLED: u32 = 71685u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_START: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_STOP: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_STORAGE: u32 = 1057u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_STORAGECOMMIT: u32 = 1058u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SUBMIT: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SUCCESS: u32 = 71725u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SUSPEND: u32 = 71686u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SVGABORT: u32 = 71652u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SVGERROR: u32 = 71653u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SVGLOAD: u32 = 71650u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SVGRESIZE: u32 = 71654u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SVGSCROLL: u32 = 71655u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SVGUNLOAD: u32 = 71651u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_SVGZOOM: u32 = 71656u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_TEXTINPUT: u32 = 71665u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_TIMEOUT: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_TIMEUPDATE: u32 = 71687u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_TOUCHCANCEL: u32 = 71787u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_TOUCHEND: u32 = 71785u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_TOUCHMOVE: u32 = 71786u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_TOUCHSTART: u32 = 71784u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_TRANSITIONEND: u32 = 71710u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_TRANSITIONSTART: u32 = 71709u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_UNLOAD: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_UPDATE: u32 = 71767u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_UPDATEEND: u32 = 71768u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_UPDATEREADY: u32 = 71720u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_UPDATESTART: u32 = 71766u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_UPGRADENEEDED: u32 = 71734u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_VISIBILITYCHANGE: u32 = 71732u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_VOLUMECHANGE: u32 = 71688u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_WAITING: u32 = 71689u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_WEBGLCONTEXTCREATIONERROR: u32 = 71792u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_WEBGLCONTEXTLOST: u32 = 71764u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_WEBGLCONTEXTRESTORED: u32 = 71765u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9EVENTS_WHEEL: u32 = 71649u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9_ELEMENT: u32 = 66786u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9_ELEMENTBASE: u32 = 66786u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IE9_ELEMENTMAX: u32 = 66821u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IELEMENTSELECTOR_QUERYSELECTOR: u32 = 66650u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IELEMENTSELECTOR_QUERYSELECTORALL: u32 = 66651u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IELEMENTTRAVERSAL_CHILDELEMENTCOUNT: u32 = 66812u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IELEMENTTRAVERSAL_FIRSTELEMENTCHILD: u32 = 66808u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IELEMENTTRAVERSAL_LASTELEMENTCHILD: u32 = 66809u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IELEMENTTRAVERSAL_NEXTELEMENTSIBLING: u32 = 66811u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IELEMENTTRAVERSAL_PREVIOUSELEMENTSIBLING: u32 = 66810u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IEVENTEXCEPTION_CODE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IEVENTEXCEPTION_MESSAGE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IEVENTTARGET_ADDEVENTLISTENER: u32 = 66046u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IEVENTTARGET_DISPATCHEVENT: u32 = 66048u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IEVENTTARGET_REMOVEEVENTLISTENER: u32 = 66047u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IFONTNAMES_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IFONTNAMES_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IFONTNAMES__NEWENUM: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IFRAME: u32 = 69536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IGETSVGDOCUMENT_GETSVGDOCUMENT: u32 = 65615u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTCATTACHBEHAVIOR2_FIREEVENT: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTCATTACHBEHAVIOR_DETACHEVENT: u32 = 66036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTCATTACHBEHAVIOR_FIREEVENT: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTCDEFAULTDISPATCH_CREATEEVENTOBJECT: u32 = 70680u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTCDEFAULTDISPATCH_DEFAULTS: u32 = 70701u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTCDEFAULTDISPATCH_DOCUMENT: u32 = 70678u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTCDEFAULTDISPATCH_ELEMENT: u32 = 70679u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTCDESCBEHAVIOR_NAME: u32 = 66037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTCDESCBEHAVIOR_URN: u32 = 66036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTCEVENTBEHAVIOR_FIRE: u32 = 66036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTCPROPERTYBEHAVIOR_FIRECHANGE: u32 = 66036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTCPROPERTYBEHAVIOR_VALUE: u32 = 70677u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT2_CHARSET: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT2_COORDS: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT2_HREFLANG: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT2_SHAPE: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT2_TYPE: u32 = 1027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT3_IE8_COORDS: u32 = 1152u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT3_IE8_HREF: u32 = 1153u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT3_IE8_SHAPE: u32 = 1151u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_ACCESSKEY: u32 = 67541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_BLUR: u32 = 67538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_FOCUS: u32 = 67536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_HASH: u32 = 1018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_HOST: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_HOSTNAME: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_HREF: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_METHODS: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_MIMETYPE: u32 = 1030u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_NAMEPROP: u32 = 1032u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_ONBLUR: u32 = 71551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_ONFOCUS: u32 = 71550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_PATHNAME: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_PORT: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_PROTOCOL: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_PROTOCOLLONG: u32 = 1031u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_REL: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_REV: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_SEARCH: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_TABINDEX: u32 = 65551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_TARGET: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLANCHORELEMENT_URN: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR2_CONTEXTMENU: u32 = 5014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR2_INNERBORDER: u32 = 5015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR2_SCROLL: u32 = 5016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR2_SCROLLFLAT: u32 = 5017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR2_SELECTION: u32 = 5018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR3_NAVIGABLE: u32 = 5019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR_APPLICATIONNAME: u32 = 5000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR_BORDER: u32 = 5007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR_BORDERSTYLE: u32 = 5008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR_CAPTION: u32 = 5010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR_COMMANDLINE: u32 = 5013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR_ICON: u32 = 5002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR_MAXIMIZEBUTTON: u32 = 5006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR_MINIMIZEBUTTON: u32 = 5005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR_SHOWINTASKBAR: u32 = 5012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR_SINGLEINSTANCE: u32 = 5003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR_SYSMENU: u32 = 5009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR_VERSION: u32 = 5001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPBEHAVIOR_WINDOWSTATE: u32 = 5011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPLICATIONCACHE_ABORT: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPLICATIONCACHE_ONCACHED: u32 = 71721u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPLICATIONCACHE_ONCHECKING: u32 = 71717u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPLICATIONCACHE_ONDOWNLOADING: u32 = 71719u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPLICATIONCACHE_ONERROR: u32 = 71565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPLICATIONCACHE_ONNOUPDATE: u32 = 71718u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPLICATIONCACHE_ONOBSOLETE: u32 = 71722u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPLICATIONCACHE_ONPROGRESS: u32 = 71681u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPLICATIONCACHE_ONUPDATEREADY: u32 = 71720u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPLICATIONCACHE_STATUS: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPLICATIONCACHE_SWAPCACHE: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAPPLICATIONCACHE_UPDATE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT2_IE8_COORDS: u32 = 1152u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT2_IE8_HREF: u32 = 1153u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT2_IE8_SHAPE: u32 = 1151u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_ALT: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_BLUR: u32 = 67538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_COORDS: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_FOCUS: u32 = 67536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_HASH: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_HOST: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_HOSTNAME: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_HREF: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_NOHREF: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_ONBLUR: u32 = 71551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_ONFOCUS: u32 = 71550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_PATHNAME: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_PORT: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_PROTOCOL: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_SEARCH: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_SHAPE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_TABINDEX: u32 = 65551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREAELEMENT_TARGET: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREASCOLLECTION2_URNS: u32 = 1505u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREASCOLLECTION3_NAMEDITEM: u32 = 1506u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREASCOLLECTION4_IE8_ITEM: u32 = 1152u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREASCOLLECTION4_IE8_LENGTH: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREASCOLLECTION4_IE8_NAMEDITEM: u32 = 1153u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREASCOLLECTION_ADD: u32 = 1503u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREASCOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREASCOLLECTION_LENGTH: u32 = 1500u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREASCOLLECTION_REMOVE: u32 = 1504u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREASCOLLECTION_TAGS: u32 = 1502u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAREASCOLLECTION__NEWENUM: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION2_GETNAMEDITEM: u32 = 1501u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION2_REMOVENAMEDITEM: u32 = 1503u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION2_SETNAMEDITEM: u32 = 1502u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION3_IE8_GETNAMEDITEM: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION3_IE8_ITEM: u32 = 1154u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION3_IE8_LENGTH: u32 = 1153u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION3_IE8_REMOVENAMEDITEM: u32 = 1152u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION3_IE8_SETNAMEDITEM: u32 = 1151u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION4_GETNAMEDITEMNS: u32 = 1155u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION4_IE9_GETNAMEDITEM: u32 = 1158u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION4_IE9_ITEM: u32 = 1161u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION4_IE9_LENGTH: u32 = 1162u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION4_IE9_REMOVENAMEDITEM: u32 = 1160u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION4_IE9_SETNAMEDITEM: u32 = 1159u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION4_REMOVENAMEDITEMNS: u32 = 1157u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION4_SETNAMEDITEMNS: u32 = 1156u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION_LENGTH: u32 = 1500u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLATTRIBUTECOLLECTION__NEWENUM: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLAUDIOELEMENTFACTORY_CREATE: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBASEELEMENT2_IE8_HREF: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBASEELEMENT_HREF: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBASEELEMENT_TARGET: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBASEFONTELEMENT_COLOR: u32 = 70538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBASEFONTELEMENT_FACE: u32 = 70554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBASEFONTELEMENT_SIZE: u32 = 70562u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBGSOUND_BALANCE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBGSOUND_LOOP: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBGSOUND_SRC: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBGSOUND_VOLUME: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBLOCKELEMENT2_CITE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBLOCKELEMENT2_WIDTH: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBLOCKELEMENT3_IE8_CITE: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBLOCKELEMENT_CLEAR: u32 = 70552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT2_ONAFTERPRINT: u32 = 71603u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT2_ONBEFOREPRINT: u32 = 71602u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT3_IE8_BACKGROUND: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT3_ONHASHCHANGE: u32 = 71645u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT3_ONOFFLINE: u32 = 71644u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT3_ONONLINE: u32 = 71643u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT4_ONMESSAGE: u32 = 71646u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT4_ONSTORAGE: u32 = 71636u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT5_ONPOPSTATE: u32 = 71728u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_ALINK: u32 = 2011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_BACKGROUND: u32 = 70537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_BGCOLOR: i32 = -501i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_BGPROPERTIES: u32 = 70581u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_BOTTOMMARGIN: u32 = 70575u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_CREATETEXTRANGE: u32 = 2013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_LEFTMARGIN: u32 = 70576u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_LINK: u32 = 2010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_NOWRAP: u32 = 70541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_ONBEFOREUNLOAD: u32 = 71575u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_ONLOAD: u32 = 71568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_ONSELECT: u32 = 71546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_ONUNLOAD: u32 = 71569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_RIGHTMARGIN: u32 = 70574u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_SCROLL: u32 = 70615u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_TEXT: u32 = 70538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_TOPMARGIN: u32 = 70573u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBODYELEMENT_VLINK: u32 = 2012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBOOKMARKCOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBOOKMARKCOLLECTION_LENGTH: u32 = 1501u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBOOKMARKCOLLECTION__NEWENUM: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBRELEMENT_CLEAR: u32 = 70552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBUTTONELEMENT2_IE9_TYPE: u32 = 8003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBUTTONELEMENT_CREATETEXTRANGE: u32 = 8002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBUTTONELEMENT_DISABLED: u32 = 65612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBUTTONELEMENT_FORM: u32 = 67540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBUTTONELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBUTTONELEMENT_STATUS: u32 = 8001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBUTTONELEMENT_TYPE: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLBUTTONELEMENT_VALUE: u32 = 70637u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCANVASELEMENT_GETCONTEXT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCANVASELEMENT_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCANVASELEMENT_TODATAURL: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCANVASELEMENT_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMMENTELEMENT2_APPENDDATA: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMMENTELEMENT2_DATA: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMMENTELEMENT2_DELETEDATA: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMMENTELEMENT2_INSERTDATA: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMMENTELEMENT2_LENGTH: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMMENTELEMENT2_REPLACEDATA: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMMENTELEMENT2_SUBSTRINGDATA: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMMENTELEMENT3_IE9_DELETEDATA: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMMENTELEMENT3_IE9_INSERTDATA: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMMENTELEMENT3_IE9_REPLACEDATA: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMMENTELEMENT3_IE9_SUBSTRINGDATA: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMMENTELEMENT_ATOMIC: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMMENTELEMENT_TEXT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_BACKGROUNDCOLOR: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_BLOCKDIRECTION: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_BOLD: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_DIRECTION: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_EXPLICITFACE: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_FONTNAME: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_FONTSIZE: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_FONTWEIGHT: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_HASBGCOLOR: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_ITALIC: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_OL: u32 = 1018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_OVERLINE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_PREFORMATTED: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_STRIKEOUT: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_SUBSCRIPT: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_SUPERSCRIPT: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_TEXTCOLOR: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCOMPUTEDSTYLE_UNDERLINE: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLELEMENT_ACCESSKEY: u32 = 67541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLELEMENT_ADDFILTER: u32 = 67553u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLELEMENT_BLUR: u32 = 67538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLELEMENT_CLIENTHEIGHT: u32 = 67555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLELEMENT_CLIENTLEFT: u32 = 67558u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLELEMENT_CLIENTTOP: u32 = 67557u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLELEMENT_CLIENTWIDTH: u32 = 67556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLELEMENT_FOCUS: u32 = 67536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLELEMENT_ONBLUR: u32 = 71551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLELEMENT_ONFOCUS: u32 = 71550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLELEMENT_ONRESIZE: u32 = 71572u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLELEMENT_REMOVEFILTER: u32 = 67554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLELEMENT_TABINDEX: u32 = 65551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLRANGE2_ADDELEMENT: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLRANGE_ADD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLRANGE_COMMONPARENTELEMENT: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLRANGE_EXECCOMMAND: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLRANGE_EXECCOMMANDSHOWHELP: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLRANGE_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLRANGE_LENGTH: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLRANGE_QUERYCOMMANDENABLED: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLRANGE_QUERYCOMMANDINDETERM: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLRANGE_QUERYCOMMANDSTATE: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLRANGE_QUERYCOMMANDSUPPORTED: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLRANGE_QUERYCOMMANDTEXT: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLRANGE_QUERYCOMMANDVALUE: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLRANGE_REMOVE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLRANGE_SCROLLINTOVIEW: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCONTROLRANGE_SELECT: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSIMPORTRULE_HREF: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSIMPORTRULE_MEDIA: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSIMPORTRULE_STYLESHEET: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSMEDIALIST_APPENDMEDIUM: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSMEDIALIST_DELETEMEDIUM: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSMEDIALIST_ITEM: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSMEDIALIST_LENGTH: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSMEDIALIST_MEDIATEXT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSMEDIARULE_CSSRULES: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSMEDIARULE_DELETERULE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSMEDIARULE_INSERTRULE: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSMEDIARULE_MEDIA: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSNAMESPACERULE_NAMESPACEURI: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSNAMESPACERULE_PREFIX: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSRULE_CSSTEXT: u32 = 1102u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSRULE_PARENTRULE: u32 = 1103u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSRULE_PARENTSTYLESHEET: u32 = 1104u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSRULE_TYPE: u32 = 1101u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATION: u32 = 70985u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATIONDELAY: u32 = 70981u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATIONDIRECTION: u32 = 70982u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATIONDURATION: u32 = 70979u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATIONFILLMODE: u32 = 70986u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATIONITERATIONCOUNT: u32 = 70984u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATIONNAME: u32 = 70978u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATIONPLAYSTATE: u32 = 70983u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ANIMATIONTIMINGFUNCTION: u32 = 70980u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_BACKFACEVISIBILITY: u32 = 70977u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_BREAKAFTER: u32 = 70882u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_BREAKBEFORE: u32 = 70881u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_BREAKINSIDE: u32 = 70883u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLORINTERPOLATIONFILTERS: u32 = 70928u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNCOUNT: u32 = 70872u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNFILL: u32 = 70875u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNGAP: u32 = 70874u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNRULE: u32 = 70877u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNRULECOLOR: u32 = 70880u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNRULESTYLE: u32 = 70878u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNRULEWIDTH: u32 = 70879u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNS: u32 = 70871u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNSPAN: u32 = 70876u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_COLUMNWIDTH: u32 = 70873u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_ENABLEBACKGROUND: u32 = 70946u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_FLOODCOLOR: u32 = 70926u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_FLOODOPACITY: u32 = 70927u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_FONTFEATURESETTINGS: u32 = 70987u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_LIGHTINGCOLOR: u32 = 70929u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATION: u32 = 70924u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATIONDELAY: u32 = 70920u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATIONDIRECTION: u32 = 70921u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATIONDURATION: u32 = 70918u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATIONFILLMODE: u32 = 70925u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATIONITERATIONCOUNT: u32 = 70923u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATIONNAME: u32 = 70917u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATIONPLAYSTATE: u32 = 70922u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSANIMATIONTIMINGFUNCTION: u32 = 70919u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSBACKFACEVISIBILITY: u32 = 70890u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSCONTENTZOOMCHAINING: u32 = 70895u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSCONTENTZOOMING: u32 = 70892u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSCONTENTZOOMLIMIT: u32 = 70897u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSCONTENTZOOMLIMITMAX: u32 = 70902u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSCONTENTZOOMLIMITMIN: u32 = 70901u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSCONTENTZOOMSNAP: u32 = 70898u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSCONTENTZOOMSNAPPOINTS: u32 = 70899u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSCONTENTZOOMSNAPTYPE: u32 = 70893u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEX: u32 = 70955u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXALIGN: u32 = 70962u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXDIRECTION: u32 = 70960u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXFLOW: u32 = 70959u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXITEMALIGN: u32 = 70963u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXLINEPACK: u32 = 70965u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXNEGATIVE: u32 = 70957u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXORDER: u32 = 70966u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXPACK: u32 = 70964u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXPOSITIVE: u32 = 70956u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXPREFERREDSIZE: u32 = 70958u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLEXWRAP: u32 = 70961u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLOWFROM: u32 = 70938u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFLOWINTO: u32 = 70939u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSFONTFEATURESETTINGS: u32 = 70950u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSGRIDCOLUMN: u32 = 70908u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSGRIDCOLUMNALIGN: u32 = 70909u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSGRIDCOLUMNS: u32 = 70910u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSGRIDCOLUMNSPAN: u32 = 70911u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSGRIDROW: u32 = 70913u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSGRIDROWALIGN: u32 = 70914u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSGRIDROWS: u32 = 70915u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSGRIDROWSPAN: u32 = 70916u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSHIGHCONTRASTADJUST: u32 = 70945u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSHYPHENATELIMITCHARS: u32 = 70942u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSHYPHENATELIMITLINES: u32 = 70943u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSHYPHENATELIMITZONE: u32 = 70941u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSHYPHENS: u32 = 70940u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSOVERFLOWSTYLE: u32 = 70935u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSPERSPECTIVE: u32 = 70885u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSPERSPECTIVEORIGIN: u32 = 70886u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLCHAINING: u32 = 70891u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLLIMIT: u32 = 70934u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLLIMITXMAX: u32 = 70932u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLLIMITXMIN: u32 = 70930u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLLIMITYMAX: u32 = 70933u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLLIMITYMIN: u32 = 70931u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLRAILS: u32 = 70894u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLSNAPPOINTSX: u32 = 70905u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLSNAPPOINTSY: u32 = 70906u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLSNAPTYPE: u32 = 70896u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLSNAPX: u32 = 70903u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLSNAPY: u32 = 70904u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSSCROLLTRANSLATION: u32 = 70954u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSTOUCHACTION: u32 = 70952u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSTOUCHSELECT: u32 = 70994u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSTRANSFORMSTYLE: u32 = 70889u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSTRANSITION: u32 = 70870u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSTRANSITIONDELAY: u32 = 70869u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSTRANSITIONDURATION: u32 = 70867u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSTRANSITIONPROPERTY: u32 = 70866u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSTRANSITIONTIMINGFUNCTION: u32 = 70868u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSUSERSELECT: u32 = 70951u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSWRAPFLOW: u32 = 70949u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSWRAPMARGIN: u32 = 70947u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_MSWRAPTHROUGH: u32 = 70937u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_PERSPECTIVE: u32 = 70974u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_PERSPECTIVEORIGIN: u32 = 70975u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TEXTSHADOW: u32 = 70936u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TRANSFORM: u32 = 70967u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TRANSFORMORIGIN: u32 = 70968u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TRANSFORMSTYLE: u32 = 70976u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TRANSITION: u32 = 70973u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TRANSITIONDELAY: u32 = 70972u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TRANSITIONDURATION: u32 = 70970u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TRANSITIONPROPERTY: u32 = 70969u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION2_TRANSITIONTIMINGFUNCTION: u32 = 70971u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_ALIGNCONTENT: u32 = 71009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_ALIGNITEMS: u32 = 71007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_ALIGNSELF: u32 = 71008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_BORDERIMAGE: u32 = 71010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_BORDERIMAGEOUTSET: u32 = 71014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_BORDERIMAGEREPEAT: u32 = 71015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_BORDERIMAGESLICE: u32 = 71012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_BORDERIMAGESOURCE: u32 = 71011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_BORDERIMAGEWIDTH: u32 = 71013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_FLEX: u32 = 71002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_FLEXBASIS: u32 = 71005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_FLEXDIRECTION: u32 = 70998u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_FLEXFLOW: u32 = 71000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_FLEXGROW: u32 = 71003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_FLEXSHRINK: u32 = 71004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_FLEXWRAP: u32 = 70999u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_JUSTIFYCONTENT: u32 = 71006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_MSIMEALIGN: u32 = 71017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_MSTEXTCOMBINEHORIZONTAL: u32 = 71018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION3_TOUCHACTION: u32 = 71019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_MSTEXTSIZEADJUST: u32 = 70864u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATION: u32 = 71033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATIONDELAY: u32 = 71038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATIONDIRECTION: u32 = 71040u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATIONDURATION: u32 = 71036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATIONFILLMODE: u32 = 71027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATIONITERATIONCOUNT: u32 = 71039u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATIONNAME: u32 = 71035u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATIONPLAYSTATE: u32 = 71041u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITANIMATIONTIMINGFUNCTION: u32 = 71037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITAPPEARANCE: u32 = 71020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKFACEVISIBILITY: u32 = 71030u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUND: u32 = 71055u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDATTACHMENT: u32 = 71046u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDCLIP: u32 = 71048u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDCOLOR: u32 = 71047u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDIMAGE: u32 = 71049u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDORIGIN: u32 = 71051u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDPOSITION: u32 = 71052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDPOSITIONX: u32 = 71053u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDPOSITIONY: u32 = 71054u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDREPEAT: u32 = 71050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBACKGROUNDSIZE: u32 = 71029u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBORDERIMAGE: u32 = 71061u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBORDERIMAGEOUTSET: u32 = 71065u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBORDERIMAGEREPEAT: u32 = 71066u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBORDERIMAGESLICE: u32 = 71063u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBORDERIMAGESOURCE: u32 = 71062u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBORDERIMAGEWIDTH: u32 = 71064u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBOXALIGN: u32 = 71021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBOXDIRECTION: u32 = 71026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBOXFLEX: u32 = 71024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBOXORDINALGROUP: u32 = 71022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBOXORIENT: u32 = 71025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBOXPACK: u32 = 71023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITBOXSIZING: u32 = 71031u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITTEXTSIZEADJUST: u32 = 71060u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITTRANSFORM: u32 = 71028u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITTRANSFORMORIGIN: u32 = 71056u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITTRANSITION: u32 = 71034u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITTRANSITIONDELAY: u32 = 71045u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITTRANSITIONDURATION: u32 = 71043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITTRANSITIONPROPERTY: u32 = 71042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITTRANSITIONTIMINGFUNCTION: u32 = 71044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION4_WEBKITUSERSELECT: u32 = 71032u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_ACCELERATOR: u32 = 70683u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_ALIGNMENTBASELINE: u32 = 70814u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUND: u32 = 70568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDATTACHMENT: u32 = 70581u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDCLIP: u32 = 70852u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDCOLOR: i32 = -501i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDIMAGE: u32 = 70537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDORIGIN: u32 = 70853u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDPOSITION: u32 = 70582u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDPOSITIONX: u32 = 70569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDPOSITIONY: u32 = 70570u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDREPEAT: u32 = 70580u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BACKGROUNDSIZE: u32 = 70854u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BASELINESHIFT: u32 = 70815u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BEHAVIOR: u32 = 70651u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDER: u32 = 70585u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERBOTTOM: u32 = 70588u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERBOTTOMCOLOR: u32 = 70593u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERBOTTOMLEFTRADIUS: u32 = 70850u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERBOTTOMRIGHTRADIUS: u32 = 70849u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERBOTTOMSTYLE: u32 = 70603u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERBOTTOMWIDTH: u32 = 70598u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERCOLLAPSE: u32 = 70620u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERCOLOR: u32 = 70590u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERLEFT: u32 = 70589u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERLEFTCOLOR: u32 = 70594u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERLEFTSTYLE: u32 = 70604u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERLEFTWIDTH: u32 = 70599u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERRADIUS: u32 = 70846u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERRIGHT: u32 = 70587u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERRIGHTCOLOR: u32 = 70592u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERRIGHTSTYLE: u32 = 70602u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERRIGHTWIDTH: u32 = 70597u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERSPACING: u32 = 70763u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERSTYLE: u32 = 70600u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERTOP: u32 = 70586u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERTOPCOLOR: u32 = 70591u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERTOPLEFTRADIUS: u32 = 70847u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERTOPRIGHTRADIUS: u32 = 70848u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERTOPSTYLE: u32 = 70601u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERTOPWIDTH: u32 = 70596u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BORDERWIDTH: u32 = 70595u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BOTTOM: u32 = 65614u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BOXSHADOW: u32 = 70855u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_BOXSIZING: u32 = 70762u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CAPTIONSIDE: u32 = 70755u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CLEAR: u32 = 70552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CLIP: u32 = 70628u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CLIPBOTTOM: u32 = 70631u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CLIPLEFT: u32 = 70632u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CLIPPATH: u32 = 70820u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CLIPRIGHT: u32 = 70630u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CLIPRULE: u32 = 70821u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CLIPTOP: u32 = 70629u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_COLOR: u32 = 70538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CONTENT: u32 = 70754u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_COUNTERINCREMENT: u32 = 70756u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_COUNTERRESET: u32 = 70757u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CSSFLOAT: u32 = 70845u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CSSTEXT: u32 = 70635u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_CURSOR: u32 = 70638u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_DIRECTION: u32 = 70655u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_DISPLAY: u32 = 70607u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_DOMINANTBASELINE: u32 = 70816u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_EMPTYCELLS: u32 = 70786u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FILL: u32 = 70822u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FILLOPACITY: u32 = 70823u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FILLRULE: u32 = 70824u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FILTER: u32 = 70618u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FONT: u32 = 70577u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FONTFAMILY: u32 = 70554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FONTSIZE: u32 = 70555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FONTSIZEADJUST: u32 = 70817u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FONTSTRETCH: u32 = 70818u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FONTSTYLE: u32 = 70560u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FONTVARIANT: u32 = 70561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_FONTWEIGHT: u32 = 70563u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_GETPROPERTYPRIORITY: u32 = 70040u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_GETPROPERTYVALUE: u32 = 70039u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_GLYPHORIENTATIONHORIZONTAL: u32 = 70843u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_GLYPHORIENTATIONVERTICAL: u32 = 70844u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_IMEMODE: u32 = 70656u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_KERNING: u32 = 70825u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LAYOUTFLOW: u32 = 70691u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LAYOUTGRID: u32 = 70667u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LAYOUTGRIDCHAR: u32 = 70663u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LAYOUTGRIDLINE: u32 = 70664u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LAYOUTGRIDMODE: u32 = 70665u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LAYOUTGRIDTYPE: u32 = 70666u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LEFT: u32 = 65539u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LENGTH: u32 = 70037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LETTERSPACING: u32 = 70544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LINEBREAK: u32 = 70669u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LINEHEIGHT: u32 = 70542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LISTSTYLE: u32 = 70611u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LISTSTYLEIMAGE: u32 = 70610u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LISTSTYLEPOSITION: u32 = 70609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_LISTSTYLETYPE: u32 = 70608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARGIN: u32 = 70572u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARGINBOTTOM: u32 = 70575u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARGINLEFT: u32 = 70576u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARGINRIGHT: u32 = 70574u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARGINTOP: u32 = 70573u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARKER: u32 = 70826u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARKEREND: u32 = 70827u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARKERMID: u32 = 70828u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MARKERSTART: u32 = 70829u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MASK: u32 = 70830u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MAXHEIGHT: u32 = 70750u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MAXWIDTH: u32 = 70752u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MINHEIGHT: u32 = 70747u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MINWIDTH: u32 = 70751u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MSBLOCKPROGRESSION: u32 = 70787u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MSINTERPOLATIONMODE: u32 = 70749u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MSTRANSFORM: u32 = 70851u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_MSTRANSFORMORIGIN: u32 = 70861u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_OPACITY: u32 = 70819u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_ORPHANS: u32 = 70764u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_OUTLINE: u32 = 70758u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_OUTLINECOLOR: u32 = 70761u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_OUTLINESTYLE: u32 = 70760u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_OUTLINEWIDTH: u32 = 70759u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_OVERFLOW: u32 = 70546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_OVERFLOWX: u32 = 70675u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_OVERFLOWY: u32 = 70676u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PADDING: u32 = 70547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PADDINGBOTTOM: u32 = 70550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PADDINGLEFT: u32 = 70551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PADDINGRIGHT: u32 = 70549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PADDINGTOP: u32 = 70548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PAGEBREAKAFTER: u32 = 70614u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PAGEBREAKBEFORE: u32 = 70613u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PAGEBREAKINSIDE: u32 = 70766u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_PARENTRULE: u32 = 70038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_POINTEREVENTS: u32 = 70831u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_POSITION: u32 = 70626u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_QUOTES: u32 = 70788u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_REMOVEPROPERTY: u32 = 70041u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_RIGHT: u32 = 65613u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_RUBYALIGN: u32 = 70657u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_RUBYOVERHANG: u32 = 70659u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_RUBYPOSITION: u32 = 70658u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SCROLLBAR3DLIGHTCOLOR: u32 = 70718u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SCROLLBARARROWCOLOR: u32 = 70722u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SCROLLBARBASECOLOR: u32 = 70716u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SCROLLBARDARKSHADOWCOLOR: u32 = 70721u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SCROLLBARFACECOLOR: u32 = 70717u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SCROLLBARHIGHLIGHTCOLOR: u32 = 70720u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SCROLLBARSHADOWCOLOR: u32 = 70719u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SCROLLBARTRACKCOLOR: u32 = 70732u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_SETPROPERTY: u32 = 70042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STOPCOLOR: u32 = 70832u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STOPOPACITY: u32 = 70833u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STROKE: u32 = 70834u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STROKEDASHARRAY: u32 = 70835u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STROKEDASHOFFSET: u32 = 70836u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STROKELINECAP: u32 = 70837u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STROKELINEJOIN: u32 = 70838u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STROKEMITERLIMIT: u32 = 70839u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STROKEOPACITY: u32 = 70840u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STROKEWIDTH: u32 = 70841u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_STYLEFLOAT: u32 = 70606u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TABLELAYOUT: u32 = 70634u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTALIGN: u32 = 65608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTALIGNLAST: u32 = 70739u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTANCHOR: u32 = 70842u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTAUTOSPACE: u32 = 70668u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTDECORATION: u32 = 70571u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTINDENT: u32 = 70543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTJUSTIFY: u32 = 70671u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTJUSTIFYTRIM: u32 = 70672u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTKASHIDA: u32 = 70673u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTKASHIDASPACE: u32 = 70740u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTOVERFLOW: u32 = 70745u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTTRANSFORM: u32 = 70540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TEXTUNDERLINEPOSITION: u32 = 70695u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_TOP: u32 = 65540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_UNICODEBIDI: u32 = 70654u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_VERTICALALIGN: u32 = 70584u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_VISIBILITY: u32 = 70616u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_WHITESPACE: u32 = 70612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_WIDOWS: u32 = 70765u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_WORDBREAK: u32 = 70670u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_WORDSPACING: u32 = 70583u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_WORDWRAP: u32 = 70694u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_WRITINGMODE: u32 = 70728u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_ZINDEX: u32 = 70627u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCSSSTYLEDECLARATION_ZOOM: u32 = 70689u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_FILTER: u32 = 70618u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_HASLAYOUT: u32 = 70696u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_ISBLOCK: u32 = 70744u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_LAYOUTFLOW: u32 = 70691u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_SCROLLBAR3DLIGHTCOLOR: u32 = 70718u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_SCROLLBARARROWCOLOR: u32 = 70722u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_SCROLLBARBASECOLOR: u32 = 70716u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_SCROLLBARDARKSHADOWCOLOR: u32 = 70721u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_SCROLLBARFACECOLOR: u32 = 70717u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_SCROLLBARHIGHLIGHTCOLOR: u32 = 70720u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_SCROLLBARSHADOWCOLOR: u32 = 70719u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_SCROLLBARTRACKCOLOR: u32 = 70732u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_TEXTALIGNLAST: u32 = 70739u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_TEXTKASHIDASPACE: u32 = 70740u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_TEXTUNDERLINEPOSITION: u32 = 70695u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_WORDWRAP: u32 = 70694u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_WRITINGMODE: u32 = 70728u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE2_ZOOM: u32 = 70689u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE3_MINHEIGHT: u32 = 70747u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE3_TEXTOVERFLOW: u32 = 70745u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE3_WHITESPACE: u32 = 70612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE3_WORDSPACING: u32 = 70583u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE4_MAXHEIGHT: u32 = 70750u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE4_MAXWIDTH: u32 = 70752u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE4_MINWIDTH: u32 = 70751u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE4_MSINTERPOLATIONMODE: u32 = 70749u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE5_BORDERSPACING: u32 = 70763u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE5_BOXSIZING: u32 = 70762u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE5_CAPTIONSIDE: u32 = 70755u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE5_EMPTYCELLS: u32 = 70786u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE5_MSBLOCKPROGRESSION: u32 = 70787u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE5_ORPHANS: u32 = 70764u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE5_OUTLINE: u32 = 70758u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE5_OUTLINECOLOR: u32 = 70761u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE5_OUTLINESTYLE: u32 = 70760u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE5_OUTLINEWIDTH: u32 = 70759u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE5_PAGEBREAKINSIDE: u32 = 70766u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE5_QUOTES: u32 = 70788u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE5_WIDOWS: u32 = 70765u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_ACCELERATOR: u32 = 70683u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BACKGROUNDATTACHMENT: u32 = 70581u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BACKGROUNDCOLOR: i32 = -501i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BACKGROUNDIMAGE: u32 = 70537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BACKGROUNDPOSITIONX: u32 = 70569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BACKGROUNDPOSITIONY: u32 = 70570u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BACKGROUNDREPEAT: u32 = 70580u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BEHAVIOR: u32 = 70651u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BLOCKDIRECTION: u32 = 70653u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BORDERBOTTOMCOLOR: u32 = 70593u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BORDERBOTTOMSTYLE: u32 = 70603u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BORDERBOTTOMWIDTH: u32 = 70598u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BORDERCOLLAPSE: u32 = 70620u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BORDERCOLOR: u32 = 70590u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BORDERLEFTCOLOR: u32 = 70594u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BORDERLEFTSTYLE: u32 = 70604u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BORDERLEFTWIDTH: u32 = 70599u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BORDERRIGHTCOLOR: u32 = 70592u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BORDERRIGHTSTYLE: u32 = 70602u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BORDERRIGHTWIDTH: u32 = 70597u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BORDERSTYLE: u32 = 70600u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BORDERTOPCOLOR: u32 = 70591u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BORDERTOPSTYLE: u32 = 70601u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BORDERTOPWIDTH: u32 = 70596u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BORDERWIDTH: u32 = 70595u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_BOTTOM: u32 = 65614u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_CLEAR: u32 = 70552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_CLIPBOTTOM: u32 = 70631u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_CLIPLEFT: u32 = 70632u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_CLIPRIGHT: u32 = 70630u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_CLIPTOP: u32 = 70629u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_COLOR: u32 = 70538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_CURSOR: u32 = 70638u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_DIRECTION: u32 = 70655u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_DISPLAY: u32 = 70607u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_FONTFAMILY: u32 = 70554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_FONTSIZE: u32 = 70555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_FONTSTYLE: u32 = 70560u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_FONTVARIANT: u32 = 70561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_FONTWEIGHT: u32 = 70563u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_GETATTRIBUTE: u32 = 66038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_IMEMODE: u32 = 70656u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_LAYOUTGRIDCHAR: u32 = 70663u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_LAYOUTGRIDLINE: u32 = 70664u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_LAYOUTGRIDMODE: u32 = 70665u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_LAYOUTGRIDTYPE: u32 = 70666u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_LEFT: u32 = 65539u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_LETTERSPACING: u32 = 70544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_LINEBREAK: u32 = 70669u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_LINEHEIGHT: u32 = 70542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_LISTSTYLEIMAGE: u32 = 70610u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_LISTSTYLEPOSITION: u32 = 70609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_LISTSTYLETYPE: u32 = 70608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_MARGIN: u32 = 70572u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_MARGINBOTTOM: u32 = 70575u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_MARGINLEFT: u32 = 70576u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_MARGINRIGHT: u32 = 70574u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_MARGINTOP: u32 = 70573u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_OVERFLOW: u32 = 70546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_OVERFLOWX: u32 = 70675u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_OVERFLOWY: u32 = 70676u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_PADDING: u32 = 70547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_PADDINGBOTTOM: u32 = 70550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_PADDINGLEFT: u32 = 70551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_PADDINGRIGHT: u32 = 70549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_PADDINGTOP: u32 = 70548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_PAGEBREAKAFTER: u32 = 70614u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_PAGEBREAKBEFORE: u32 = 70613u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_POSITION: u32 = 70626u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_RIGHT: u32 = 65613u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_RUBYALIGN: u32 = 70657u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_RUBYOVERHANG: u32 = 70659u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_RUBYPOSITION: u32 = 70658u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_STYLEFLOAT: u32 = 70606u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_TABLELAYOUT: u32 = 70634u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_TEXTALIGN: u32 = 65608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_TEXTAUTOSPACE: u32 = 70668u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_TEXTDECORATION: u32 = 70571u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_TEXTINDENT: u32 = 70543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_TEXTJUSTIFY: u32 = 70671u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_TEXTJUSTIFYTRIM: u32 = 70672u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_TEXTKASHIDA: u32 = 70673u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_TEXTTRANSFORM: u32 = 70540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_TOP: u32 = 65540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_UNICODEBIDI: u32 = 70654u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_VERTICALALIGN: u32 = 70584u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_VISIBILITY: u32 = 70616u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_WORDBREAK: u32 = 70670u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLCURRENTSTYLE_ZINDEX: u32 = 70627u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDATABINDING_DATAFLD: u32 = 66557u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDATABINDING_DATAFORMATAS: u32 = 66559u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDATABINDING_DATASRC: u32 = 66558u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDATATRANSFER_CLEARDATA: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDATATRANSFER_DROPEFFECT: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDATATRANSFER_EFFECTALLOWED: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDATATRANSFER_GETDATA: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDATATRANSFER_SETDATA: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDDELEMENT_NOWRAP: u32 = 70541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDIALOG2_RESIZABLE: u32 = 25015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDIALOG2_STATUS: u32 = 25014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDIALOG3_DIALOGHIDE: u32 = 25007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDIALOG3_UNADORNED: u32 = 25016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDIALOG_CLOSE: u32 = 25011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDIALOG_DIALOGARGUMENTS: u32 = 25000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDIALOG_DIALOGHEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDIALOG_DIALOGLEFT: u32 = 65539u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDIALOG_DIALOGTOP: u32 = 65540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDIALOG_DIALOGWIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDIALOG_MENUARGUMENTS: u32 = 25013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDIALOG_RETURNVALUE: u32 = 25001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDIALOG_TOSTRING: u32 = 25012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDIVELEMENT_ALIGN: u32 = 65608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDIVELEMENT_NOWRAP: u32 = 70541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDIVPOSITION_ALIGN: u32 = 65609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDLGSAFEHELPER_BLOCKFORMATS: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDLGSAFEHELPER_CHOOSECOLORDLG: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDLGSAFEHELPER_FONTS: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDLGSAFEHELPER_GETCHARSET: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDLISTELEMENT_COMPACT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ACTIVEELEMENT: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ALINKCOLOR: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ALL: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ANCHORS: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_APPLETS: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_BGCOLOR: i32 = -501i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_BODY: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_CHARSET: u32 = 1032u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_CLEAR: u32 = 1058u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_CLOSE: u32 = 1057u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_COOKIE: u32 = 1030u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_CREATEELEMENT: u32 = 1067u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_CREATESTYLESHEET: u32 = 1071u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_DEFAULTCHARSET: u32 = 1033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_DESIGNMODE: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_DOMAIN: u32 = 1029u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ELEMENTFROMPOINT: u32 = 1068u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_EMBEDS: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_EXECCOMMAND: u32 = 1065u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_EXECCOMMANDSHOWHELP: u32 = 1066u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_EXPANDO: u32 = 1031u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_FGCOLOR: u32 = 70538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_FILECREATEDDATE: u32 = 1043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_FILEMODIFIEDDATE: u32 = 1044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_FILESIZE: u32 = 1042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_FILEUPDATEDDATE: u32 = 1045u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_FORMS: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_FRAMES: u32 = 1019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_IMAGES: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_LASTMODIFIED: u32 = 1028u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_LINKCOLOR: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_LINKS: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_LOCATION: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_MIMETYPE: u32 = 1041u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_NAMEPROP: u32 = 1048u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONAFTERUPDATE: u32 = 71558u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONBEFOREUPDATE: u32 = 71557u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONCLICK: u32 = 71544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONDBLCLICK: u32 = 71545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONDRAGSTART: u32 = 71571u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONERRORUPDATE: u32 = 71574u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONHELP: u32 = 71549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONKEYDOWN: u32 = 71541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONKEYPRESS: u32 = 71543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONKEYUP: u32 = 71542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONMOUSEDOWN: u32 = 71538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONMOUSEMOVE: u32 = 71540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONMOUSEOUT: u32 = 71537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONMOUSEOVER: u32 = 71536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONMOUSEUP: u32 = 71539u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONREADYSTATECHANGE: u32 = 71561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONROWENTER: u32 = 71555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONROWEXIT: u32 = 71554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_ONSELECTSTART: u32 = 71573u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_OPEN: u32 = 1056u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_PARENTWINDOW: u32 = 1034u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_PLUGINS: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_PROTOCOL: u32 = 1047u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_QUERYCOMMANDENABLED: u32 = 1060u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_QUERYCOMMANDINDETERM: u32 = 1062u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_QUERYCOMMANDSTATE: u32 = 1061u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_QUERYCOMMANDSUPPORTED: u32 = 1059u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_QUERYCOMMANDTEXT: u32 = 1063u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_QUERYCOMMANDVALUE: u32 = 1064u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_READYSTATE: u32 = 1018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_REFERRER: u32 = 1027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_SCRIPTS: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_SECURITY: u32 = 1046u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_SELECTION: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_STYLESHEETS: u32 = 1069u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_TITLE: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_TOSTRING: u32 = 1070u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_URL: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_VLINKCOLOR: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_WRITE: u32 = 1054u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT2_WRITELN: u32 = 1055u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_ATTACHEVENT: u32 = 66043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_BASEURL: u32 = 1080u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_CHILDNODES: u32 = 66585u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_CREATEDOCUMENTFRAGMENT: u32 = 1076u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_CREATETEXTNODE: u32 = 1074u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_DETACHEVENT: u32 = 66044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_DIR: u32 = 70653u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_DOCUMENTELEMENT: u32 = 1075u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_ENABLEDOWNLOAD: u32 = 1079u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_GETELEMENTBYID: u32 = 1088u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_GETELEMENTSBYNAME: u32 = 1086u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_GETELEMENTSBYTAGNAME: u32 = 1087u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_INHERITSTYLESHEETS: u32 = 1082u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_ONBEFOREEDITFOCUS: u32 = 71605u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_ONCELLCHANGE: u32 = 71600u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_ONCONTEXTMENU: u32 = 71601u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_ONDATAAVAILABLE: u32 = 71577u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_ONDATASETCHANGED: u32 = 71576u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_ONDATASETCOMPLETE: u32 = 71578u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_ONPROPERTYCHANGE: u32 = 71583u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_ONROWSDELETE: u32 = 71598u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_ONROWSINSERTED: u32 = 71599u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_ONSTOP: u32 = 71604u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_PARENTDOCUMENT: u32 = 1078u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_RECALC: u32 = 1073u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_RELEASECAPTURE: u32 = 1072u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT3_UNIQUEID: u32 = 1077u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT4_CREATEDOCUMENTFROMURL: u32 = 1092u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT4_CREATEEVENTOBJECT: u32 = 1094u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT4_CREATERENDERSTYLE: u32 = 1096u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT4_FIREEVENT: u32 = 1095u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT4_FOCUS: u32 = 1089u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT4_HASFOCUS: u32 = 1090u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT4_MEDIA: u32 = 1093u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT4_NAMESPACES: u32 = 1091u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT4_ONCONTROLSELECT: u32 = 71615u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT4_ONSELECTIONCHANGE: u32 = 71616u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT4_URLUNENCODED: u32 = 1097u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT5_COMPATMODE: u32 = 1102u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT5_CREATEATTRIBUTE: u32 = 1100u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT5_CREATECOMMENT: u32 = 1101u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT5_DOCTYPE: u32 = 1098u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT5_IMPLEMENTATION: u32 = 1099u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT5_ONACTIVATE: u32 = 71623u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT5_ONBEFOREACTIVATE: u32 = 71626u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT5_ONBEFOREDEACTIVATE: u32 = 71613u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT5_ONDEACTIVATE: u32 = 71624u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT5_ONFOCUSIN: u32 = 71627u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT5_ONFOCUSOUT: u32 = 71628u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT5_ONMOUSEWHEEL: u32 = 71612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT6_COMPATIBLE: u32 = 1103u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT6_DOCUMENTMODE: u32 = 1104u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT6_IE8_GETELEMENTBYID: u32 = 1107u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT6_ONSTORAGE: u32 = 71636u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT6_ONSTORAGECOMMIT: u32 = 71637u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT6_UPDATESETTINGS: u32 = 1109u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ADOPTNODE: u32 = 1125u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_CHARACTERSET: u32 = 1117u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_CREATEATTRIBUTENS: u32 = 1115u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_CREATECDATASECTION: u32 = 1123u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_CREATEELEMENTNS: u32 = 1114u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_CREATEPROCESSINGINSTRUCTION: u32 = 1124u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_DEFAULTVIEW: u32 = 1110u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_GETELEMENTSBYCLASSNAME: u32 = 1120u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_GETELEMENTSBYTAGNAMENS: u32 = 1113u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_GETSELECTION: u32 = 1112u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_HASATTRIBUTES: u32 = 1132u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_HEAD: u32 = 1138u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_IE9_ALL: u32 = 1126u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_IE9_BODY: u32 = 1137u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_IE9_CREATEATTRIBUTE: u32 = 1119u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_IE9_CREATEELEMENT: u32 = 1118u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_IE9_PARENTWINDOW: u32 = 1136u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_IMPORTNODE: u32 = 1135u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_INPUTENCODING: u32 = 1127u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_NORMALIZE: u32 = 1134u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONABORT: u32 = 71564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONBLUR: u32 = 71551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONCANPLAY: u32 = 71670u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONCANPLAYTHROUGH: u32 = 71671u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONCHANGE: u32 = 71566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONDRAG: u32 = 71585u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONDRAGEND: u32 = 71586u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONDRAGENTER: u32 = 71587u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONDRAGLEAVE: u32 = 71589u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONDRAGOVER: u32 = 71588u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONDROP: u32 = 71590u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONDURATIONCHANGE: u32 = 71672u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONEMPTIED: u32 = 71673u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONENDED: u32 = 71674u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONERROR: u32 = 71565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONFOCUS: u32 = 71550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONINPUT: u32 = 71663u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONLOAD: u32 = 71568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONLOADEDDATA: u32 = 71675u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONLOADEDMETADATA: u32 = 71676u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONLOADSTART: u32 = 71677u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONMSSITEMODEJUMPLISTITEMREMOVED: u32 = 71666u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONMSTHUMBNAILCLICK: u32 = 71657u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONPAUSE: u32 = 71678u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONPLAY: u32 = 71679u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONPLAYING: u32 = 71680u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONPROGRESS: u32 = 71681u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONRATECHANGE: u32 = 71682u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONRESET: u32 = 71548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONSCROLL: u32 = 71567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONSEEKED: u32 = 71683u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONSEEKING: u32 = 71684u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONSELECT: u32 = 71546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONSTALLED: u32 = 71685u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONSUBMIT: u32 = 71547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONSUSPEND: u32 = 71686u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONTIMEUPDATE: u32 = 71687u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONVOLUMECHANGE: u32 = 71688u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_ONWAITING: u32 = 71689u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_XMLENCODING: u32 = 1128u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_XMLSTANDALONE: u32 = 1129u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT7_XMLVERSION: u32 = 1130u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ELEMENTSFROMPOINT: u32 = 1139u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ELEMENTSFROMRECT: u32 = 1140u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_MSCAPSLOCKWARNINGOFF: u32 = 1141u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ONMSCONTENTZOOM: u32 = 71708u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ONMSGESTURECHANGE: u32 = 71700u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ONMSGESTUREDOUBLETAP: u32 = 71704u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ONMSGESTUREEND: u32 = 71701u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ONMSGESTUREHOLD: u32 = 71702u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ONMSGESTURESTART: u32 = 71699u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ONMSGESTURETAP: u32 = 71703u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ONMSINERTIASTART: u32 = 71705u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ONMSMANIPULATIONSTATECHANGED: u32 = 71714u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ONMSPOINTERCANCEL: u32 = 71695u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ONMSPOINTERDOWN: u32 = 71690u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ONMSPOINTERHOVER: u32 = 71696u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ONMSPOINTERMOVE: u32 = 71691u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ONMSPOINTEROUT: u32 = 71694u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ONMSPOINTEROVER: u32 = 71693u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT8_ONMSPOINTERUP: u32 = 71692u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENTCOMPATIBLEINFOCOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENTCOMPATIBLEINFOCOLLECTION_LENGTH: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENTCOMPATIBLEINFO_USERAGENT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENTCOMPATIBLEINFO_VERSION: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOCUMENT_SCRIPT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_APPENDCHILD: u32 = 1018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_ATTRIBUTES: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_CHILDNODES: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_CLONENODE: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_EXPANDO: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_FIRSTCHILD: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_HASCHILDNODES: u32 = 1019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_INSERTBEFORE: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_LASTCHILD: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_NAME: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_NEXTSIBLING: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_NODETYPE: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_OWNERDOCUMENT: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_PARENTNODE: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_PREVIOUSSIBLING: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_REMOVECHILD: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_REPLACECHILD: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE2_VALUE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE3_IE8_NODEVALUE: u32 = 1153u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE3_IE8_SPECIFIED: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE3_IE8_VALUE: u32 = 1154u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE3_OWNERELEMENT: u32 = 1151u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE4_HASATTRIBUTES: u32 = 1166u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_CHILDNODES: u32 = 1165u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_FIRSTCHILD: u32 = 1163u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_HASCHILDNODES: u32 = 1167u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_LASTCHILD: u32 = 1164u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_NAME: u32 = 1161u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_NODENAME: u32 = 1160u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_NODEVALUE: u32 = 1159u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_SPECIFIED: u32 = 1171u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE4_IE9_VALUE: u32 = 1162u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE4_NORMALIZE: u32 = 1170u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE_NODENAME: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE_NODEVALUE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMATTRIBUTE_SPECIFIED: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMCHILDRENCOLLECTION2_IE9_ITEM: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMCHILDRENCOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMCHILDRENCOLLECTION_LENGTH: u32 = 1500u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMCHILDRENCOLLECTION__NEWENUM: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMCONSTRUCTOR_CONSTRUCTOR: u32 = 66045u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMIMPLEMENTATION2_CREATEDOCUMENT: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMIMPLEMENTATION2_CREATEDOCUMENTTYPE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMIMPLEMENTATION2_CREATEHTMLDOCUMENT: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMIMPLEMENTATION2_IE9_HASFEATURE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMIMPLEMENTATION_HASFEATURE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE2_OWNERDOCUMENT: u32 = 66649u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE3_COMPAREDOCUMENTPOSITION: u32 = 66662u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE3_IE9_APPENDCHILD: u32 = 66804u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE3_IE9_INSERTBEFORE: u32 = 66805u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE3_IE9_REMOVECHILD: u32 = 66806u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE3_IE9_REPLACECHILD: u32 = 66807u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE3_ISDEFAULTNAMESPACE: u32 = 66660u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE3_ISEQUALNODE: u32 = 66657u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE3_ISSAMENODE: u32 = 66661u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE3_ISSUPPORTED: u32 = 66813u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE3_LOCALNAME: u32 = 66654u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE3_LOOKUPNAMESPACEURI: u32 = 66658u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE3_LOOKUPPREFIX: u32 = 66659u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE3_NAMESPACEURI: u32 = 66655u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE3_PREFIX: u32 = 66656u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE3_TEXTCONTENT: u32 = 66663u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_APPENDCHILD: u32 = 66609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_ATTRIBUTES: u32 = 66586u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_CHILDNODES: u32 = 66585u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_CLONENODE: u32 = 66597u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_FIRSTCHILD: u32 = 66612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_HASCHILDNODES: u32 = 66584u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_INSERTBEFORE: u32 = 66587u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_LASTCHILD: u32 = 66613u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_NEXTSIBLING: u32 = 66615u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_NODENAME: u32 = 66610u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_NODETYPE: u32 = 66582u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_NODEVALUE: u32 = 66611u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_PARENTNODE: u32 = 66583u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_PREVIOUSSIBLING: u32 = 66614u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_REMOVECHILD: u32 = 66588u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_REMOVENODE: u32 = 66602u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_REPLACECHILD: u32 = 66589u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_REPLACENODE: u32 = 66603u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMNODE_SWAPNODE: u32 = 66604u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_CLONECONTENTS: u32 = 1019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_CLONERANGE: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_COLLAPSE: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_COLLAPSED: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_COMMONANCESTORCONTAINER: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_COMPAREBOUNDARYPOINTS: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_DELETECONTENTS: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_DETACH: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_ENDCONTAINER: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_ENDOFFSET: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_EXTRACTCONTENTS: u32 = 1018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_GETBOUNDINGCLIENTRECT: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_GETCLIENTRECTS: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_INSERTNODE: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_SELECTNODE: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_SELECTNODECONTENTS: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_SETEND: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_SETENDAFTER: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_SETENDBEFORE: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_SETSTART: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_SETSTARTAFTER: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_SETSTARTBEFORE: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_STARTCONTAINER: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_STARTOFFSET: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_SURROUNDCONTENTS: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMRANGE_TOSTRING: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE2_APPENDDATA: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE2_DELETEDATA: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE2_INSERTDATA: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE2_REPLACEDATA: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE2_SUBSTRINGDATA: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE3_HASATTRIBUTES: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE3_IE9_DELETEDATA: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE3_IE9_INSERTDATA: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE3_IE9_REPLACEDATA: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE3_IE9_SPLITTEXT: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE3_IE9_SUBSTRINGDATA: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE3_NORMALIZE: u32 = 1019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE3_REPLACEWHOLETEXT: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE3_WHOLETEXT: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE_DATA: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE_LENGTH: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE_SPLITTEXT: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDOMTEXTNODE_TOSTRING: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLDTELEMENT_NOWRAP: u32 = 70541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ACCESSKEY: u32 = 67541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ADDBEHAVIOR: u32 = 66616u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ADDFILTER: u32 = 67553u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_APPLYELEMENT: u32 = 66601u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ATTACHEVENT: u32 = 66043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_BEHAVIORURNS: u32 = 66618u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_BLUR: u32 = 67538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_CANHAVECHILDREN: u32 = 66608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_CLEARATTRIBUTES: u32 = 66598u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_CLIENTHEIGHT: u32 = 67555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_CLIENTLEFT: u32 = 67558u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_CLIENTTOP: u32 = 67557u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_CLIENTWIDTH: u32 = 67556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_COMPONENTFROMPOINT: u32 = 66578u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_CREATECONTROLRANGE: u32 = 66592u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_CURRENTSTYLE: u32 = 66543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_DETACHEVENT: u32 = 66044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_DIR: u32 = 70653u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_DOSCROLL: u32 = 66579u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_FOCUS: u32 = 67536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_GETADJACENTTEXT: u32 = 66606u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_GETBOUNDINGCLIENTRECT: u32 = 66581u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_GETCLIENTRECTS: u32 = 66580u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_GETELEMENTSBYTAGNAME: u32 = 66621u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_GETEXPRESSION: u32 = 66041u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_INSERTADJACENTELEMENT: u32 = 66605u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_MERGEATTRIBUTES: u32 = 66599u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONBEFORECOPY: u32 = 71595u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONBEFORECUT: u32 = 71594u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONBEFOREEDITFOCUS: u32 = 71605u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONBEFOREPASTE: u32 = 71596u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONBLUR: u32 = 71551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONCELLCHANGE: u32 = 71600u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONCONTEXTMENU: u32 = 71601u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONCOPY: u32 = 71592u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONCUT: u32 = 71591u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONDRAG: u32 = 71585u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONDRAGEND: u32 = 71586u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONDRAGENTER: u32 = 71587u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONDRAGLEAVE: u32 = 71589u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONDRAGOVER: u32 = 71588u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONDROP: u32 = 71590u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONFOCUS: u32 = 71550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONLOSECAPTURE: u32 = 71582u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONPASTE: u32 = 71593u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONPROPERTYCHANGE: u32 = 71583u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONREADYSTATECHANGE: u32 = 71561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONRESIZE: u32 = 71572u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONROWSDELETE: u32 = 71598u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONROWSINSERTED: u32 = 71599u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_ONSCROLL: u32 = 71567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_READYSTATE: u32 = 70652u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_READYSTATEVALUE: u32 = 66620u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_RELEASECAPTURE: u32 = 66577u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_REMOVEBEHAVIOR: u32 = 66617u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_REMOVEEXPRESSION: u32 = 66042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_REMOVEFILTER: u32 = 67554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_REPLACEADJACENTTEXT: u32 = 66607u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_RUNTIMESTYLE: u32 = 66600u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_SCOPENAME: u32 = 66575u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_SCROLLHEIGHT: u32 = 66593u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_SCROLLLEFT: u32 = 66596u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_SCROLLTOP: u32 = 66595u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_SCROLLWIDTH: u32 = 66594u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_SETCAPTURE: u32 = 66576u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_SETEXPRESSION: u32 = 66040u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_TABINDEX: u32 = 65551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT2_TAGURN: u32 = 66619u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_CANHAVEHTML: u32 = 66634u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_CONTENTEDITABLE: u32 = 70698u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_DISABLED: u32 = 65612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_DRAGDROP: u32 = 66643u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_FIREEVENT: u32 = 66642u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_GLYPHMODE: u32 = 66644u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_HIDEFOCUS: u32 = 70699u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_INFLATEBLOCK: u32 = 66636u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_ISCONTENTEDITABLE: u32 = 66638u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_ISDISABLED: u32 = 66641u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_ISMULTILINE: u32 = 66633u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_MERGEATTRIBUTES: u32 = 66632u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_ONACTIVATE: u32 = 71623u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_ONBEFOREDEACTIVATE: u32 = 71613u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_ONCONTROLSELECT: u32 = 71615u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_ONDEACTIVATE: u32 = 71624u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_ONLAYOUTCOMPLETE: u32 = 71609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_ONMOUSEENTER: u32 = 71621u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_ONMOUSELEAVE: u32 = 71622u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_ONMOVE: u32 = 71614u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_ONMOVEEND: u32 = 71618u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_ONMOVESTART: u32 = 71617u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_ONPAGE: u32 = 71610u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_ONRESIZEEND: u32 = 71620u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_ONRESIZESTART: u32 = 71619u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT3_SETACTIVE: u32 = 66637u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT4_GETATTRIBUTENODE: u32 = 66645u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT4_NORMALIZE: u32 = 66648u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT4_ONBEFOREACTIVATE: u32 = 71626u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT4_ONFOCUSIN: u32 = 71627u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT4_ONFOCUSOUT: u32 = 71628u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT4_ONMOUSEWHEEL: u32 = 71612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT4_REMOVEATTRIBUTENODE: u32 = 66647u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT4_SETATTRIBUTENODE: u32 = 66646u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIAACTIVEDESCENDANT: u32 = 66768u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIABUSY: u32 = 66741u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIACHECKED: u32 = 66742u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIACONTROLS: u32 = 66764u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIADESCRIBEDBY: u32 = 66765u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIADISABLED: u32 = 66743u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIAEXPANDED: u32 = 66744u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIAFLOWTO: u32 = 66766u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIAHASPOPUP: u32 = 66745u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIAHIDDEN: u32 = 66746u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIAINVALID: u32 = 66747u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIALABELLEDBY: u32 = 66767u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIALEVEL: u32 = 66761u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIALIVE: u32 = 66771u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIAMULTISELECTABLE: u32 = 66748u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIAOWNS: u32 = 66769u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIAPOSINSET: u32 = 66759u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIAPRESSED: u32 = 66749u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIAREADONLY: u32 = 66750u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIARELEVANT: u32 = 66772u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIAREQUIRED: u32 = 66751u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIASECRET: u32 = 66752u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIASELECTED: u32 = 66753u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIASETSIZE: u32 = 66760u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIAVALUEMAX: u32 = 66763u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIAVALUEMIN: u32 = 66762u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ARIAVALUENOW: u32 = 66758u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_HASATTRIBUTE: u32 = 66739u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_HASATTRIBUTES: u32 = 66770u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_IE8_ATTRIBUTES: u32 = 66757u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_IE8_GETATTRIBUTE: u32 = 66754u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_IE8_GETATTRIBUTENODE: u32 = 66736u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_IE8_REMOVEATTRIBUTE: u32 = 66756u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_IE8_REMOVEATTRIBUTENODE: u32 = 66738u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_IE8_SETATTRIBUTE: u32 = 66755u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_IE8_SETATTRIBUTENODE: u32 = 66737u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT5_ROLE: u32 = 66740u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_GETATTRIBUTENODENS: u32 = 66786u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_GETATTRIBUTENS: u32 = 66789u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_GETELEMENTSBYCLASSNAME: u32 = 66803u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_GETELEMENTSBYTAGNAMENS: u32 = 66799u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_HASATTRIBUTENS: u32 = 66788u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_IE9_GETATTRIBUTE: u32 = 66796u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_IE9_GETATTRIBUTENODE: u32 = 66792u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_IE9_HASATTRIBUTE: u32 = 66795u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_IE9_HASATTRIBUTES: u32 = 66815u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_IE9_NODENAME: u32 = 66802u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_IE9_REMOVEATTRIBUTE: u32 = 66798u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_IE9_REMOVEATTRIBUTENODE: u32 = 66794u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_IE9_SETATTRIBUTE: u32 = 66797u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_IE9_SETATTRIBUTENODE: u32 = 66793u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_IE9_TAGNAME: u32 = 66801u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_MSMATCHESSELECTOR: u32 = 66814u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONABORT: u32 = 71564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONCANPLAY: u32 = 71670u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONCANPLAYTHROUGH: u32 = 71671u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONCHANGE: u32 = 71566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONDURATIONCHANGE: u32 = 71672u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONEMPTIED: u32 = 71673u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONENDED: u32 = 71674u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONERROR: u32 = 71565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONINPUT: u32 = 71663u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONLOAD: u32 = 71568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONLOADEDDATA: u32 = 71675u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONLOADEDMETADATA: u32 = 71676u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONLOADSTART: u32 = 71677u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONPAUSE: u32 = 71678u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONPLAY: u32 = 71679u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONPLAYING: u32 = 71680u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONPROGRESS: u32 = 71681u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONRATECHANGE: u32 = 71682u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONRESET: u32 = 71548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONSEEKED: u32 = 71683u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONSEEKING: u32 = 71684u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONSELECT: u32 = 71546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONSTALLED: u32 = 71685u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONSUBMIT: u32 = 71547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONSUSPEND: u32 = 71686u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONTIMEUPDATE: u32 = 71687u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONVOLUMECHANGE: u32 = 71688u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_ONWAITING: u32 = 71689u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_REMOVEATTRIBUTENS: u32 = 66791u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_SETATTRIBUTENODENS: u32 = 66787u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT6_SETATTRIBUTENS: u32 = 66790u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_MSRELEASEPOINTERCAPTURE: u32 = 66823u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_MSSETPOINTERCAPTURE: u32 = 66822u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONCUECHANGE: u32 = 71729u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONINVALID: u32 = 71724u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSANIMATIONEND: u32 = 71712u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSANIMATIONITERATION: u32 = 71713u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSANIMATIONSTART: u32 = 71711u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSGESTURECHANGE: u32 = 71700u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSGESTUREDOUBLETAP: u32 = 71704u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSGESTUREEND: u32 = 71701u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSGESTUREHOLD: u32 = 71702u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSGESTURESTART: u32 = 71699u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSGESTURETAP: u32 = 71703u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSGOTPOINTERCAPTURE: u32 = 71707u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSINERTIASTART: u32 = 71705u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSLOSTPOINTERCAPTURE: u32 = 71706u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSMANIPULATIONSTATECHANGED: u32 = 71714u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSPOINTERCANCEL: u32 = 71695u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSPOINTERDOWN: u32 = 71690u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSPOINTERHOVER: u32 = 71696u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSPOINTERMOVE: u32 = 71691u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSPOINTEROUT: u32 = 71694u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSPOINTEROVER: u32 = 71693u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSPOINTERUP: u32 = 71692u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSTRANSITIONEND: u32 = 71710u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_ONMSTRANSITIONSTART: u32 = 71709u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_SPELLCHECK: u32 = 70907u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT7_XMSACCELERATORKEY: u32 = 66834u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTAPPLIEDSTYLES_MSGETRULESAPPLIED: u32 = 66652u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTAPPLIEDSTYLES_MSGETRULESAPPLIEDWITHANCESTOR: u32 = 66653u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTCOLLECTION2_URNS: u32 = 1505u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTCOLLECTION3_NAMEDITEM: u32 = 1506u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTCOLLECTION4_IE8_ITEM: u32 = 1152u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTCOLLECTION4_IE8_LENGTH: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTCOLLECTION4_IE8_NAMEDITEM: u32 = 1153u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTCOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTCOLLECTION_LENGTH: u32 = 1500u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTCOLLECTION_TAGS: u32 = 1502u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTCOLLECTION_TOSTRING: u32 = 1501u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTCOLLECTION__NEWENUM: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTDEFAULTS_CANHAVEHTML: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTDEFAULTS_CONTENTEDITABLE: u32 = 70698u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTDEFAULTS_FROZEN: u32 = 70734u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTDEFAULTS_ISMULTILINE: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTDEFAULTS_SCROLLSEGMENTX: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTDEFAULTS_SCROLLSEGMENTY: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTDEFAULTS_STYLE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTDEFAULTS_TABSTOP: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTDEFAULTS_VIEWINHERITSTYLE: u32 = 70735u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTDEFAULTS_VIEWLINK: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENTDEFAULTS_VIEWMASTERTAB: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ALL: u32 = 66574u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_CHILDREN: u32 = 66573u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_CLASSNAME: u32 = 66537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_CLICK: u32 = 66569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_CONTAINS: u32 = 66556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_DOCUMENT: u32 = 66554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_FILTERS: u32 = 66571u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_GETATTRIBUTE: u32 = 66038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ID: u32 = 66538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_INNERHTML: u32 = 66562u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_INNERTEXT: u32 = 66563u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_INSERTADJACENTHTML: u32 = 66566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_INSERTADJACENTTEXT: u32 = 66567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ISTEXTEDIT: u32 = 66570u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_LANG: u32 = 70545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_LANGUAGE: u32 = 70636u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_OFFSETHEIGHT: u32 = 66547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_OFFSETLEFT: u32 = 66544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_OFFSETPARENT: u32 = 66548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_OFFSETTOP: u32 = 66545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_OFFSETWIDTH: u32 = 66546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONAFTERUPDATE: u32 = 71558u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONBEFOREUPDATE: u32 = 71557u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONCLICK: u32 = 71544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONDATAAVAILABLE: u32 = 71577u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONDATASETCHANGED: u32 = 71576u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONDATASETCOMPLETE: u32 = 71578u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONDBLCLICK: u32 = 71545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONDRAGSTART: u32 = 71571u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONERRORUPDATE: u32 = 71574u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONFILTERCHANGE: u32 = 71579u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONHELP: u32 = 71549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONKEYDOWN: u32 = 71541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONKEYPRESS: u32 = 71543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONKEYUP: u32 = 71542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONMOUSEDOWN: u32 = 71538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONMOUSEMOVE: u32 = 71540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONMOUSEOUT: u32 = 71537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONMOUSEOVER: u32 = 71536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONMOUSEUP: u32 = 71539u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONROWENTER: u32 = 71555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONROWEXIT: u32 = 71554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_ONSELECTSTART: u32 = 71573u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_OUTERHTML: u32 = 66564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_OUTERTEXT: u32 = 66565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_PARENTELEMENT: u32 = 65544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_PARENTTEXTEDIT: u32 = 66568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_RECORDNUMBER: u32 = 66561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_REMOVEATTRIBUTE: u32 = 66039u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_SCROLLINTOVIEW: u32 = 66555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_SETATTRIBUTE: u32 = 66037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_SOURCEINDEX: u32 = 66560u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_STYLE: u32 = 65610u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_TAGNAME: u32 = 66540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_TITLE: u32 = 65605u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLELEMENT_TOSTRING: u32 = 66572u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEMBEDELEMENT2_IE8_PLUGINSPAGE: u32 = 1151u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEMBEDELEMENT2_IE8_SRC: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEMBEDELEMENT_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEMBEDELEMENT_HIDDEN: u32 = 68546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEMBEDELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEMBEDELEMENT_PALETTE: u32 = 68540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEMBEDELEMENT_PLUGINSPAGE: u32 = 68541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEMBEDELEMENT_SRC: u32 = 68542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEMBEDELEMENT_UNITS: u32 = 68544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEMBEDELEMENT_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_ALTKEY: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_BOOKMARKS: u32 = 1031u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_BOUNDELEMENTS: u32 = 1034u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_BUTTON: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_CLIENTX: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_CLIENTY: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_CTRLKEY: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_DATAFLD: u32 = 1033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_DATATRANSFER: u32 = 1037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_FROMELEMENT: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_GETATTRIBUTE: u32 = 66038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_OFFSETX: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_OFFSETY: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_PROPERTYNAME: u32 = 1027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_QUALIFIER: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_REASON: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_RECORDSET: u32 = 1032u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_REMOVEATTRIBUTE: u32 = 66039u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_REPEAT: u32 = 1035u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_SCREENX: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_SCREENY: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_SETATTRIBUTE: u32 = 66037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_SHIFTKEY: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_SRCELEMENT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_SRCFILTER: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_SRCURN: u32 = 1036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_TOELEMENT: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_TYPE: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_X: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ2_Y: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ3_ALTLEFT: u32 = 1040u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ3_BEHAVIORCOOKIE: u32 = 1048u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ3_BEHAVIORPART: u32 = 1049u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ3_CONTENTOVERFLOW: u32 = 1038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ3_CTRLLEFT: u32 = 1041u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ3_IMECOMPOSITIONCHANGE: u32 = 1042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ3_IMENOTIFYCOMMAND: u32 = 1043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ3_IMENOTIFYDATA: u32 = 1044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ3_IMEREQUEST: u32 = 1046u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ3_IMEREQUESTDATA: u32 = 1047u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ3_KEYBOARDLAYOUT: u32 = 1045u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ3_NEXTPAGE: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ3_SHIFTLEFT: u32 = 1039u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ4_WHEELDELTA: u32 = 1051u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ5_DATA: u32 = 1054u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ5_ISSESSION: u32 = 1056u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ5_ORIGIN: u32 = 1053u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ5_SOURCE: u32 = 1055u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ5_URL: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ6_ACTIONURL: u32 = 1058u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ6_BUTTONID: u32 = 1057u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_ALTKEY: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_BUTTON: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_CANCELBUBBLE: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_CLIENTX: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_CLIENTY: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_CTRLKEY: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_FROMELEMENT: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_KEYCODE: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_OFFSETX: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_OFFSETY: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_QUALIFIER: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_REASON: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_RETURNVALUE: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_SCREENX: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_SCREENY: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_SHIFTKEY: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_SRCELEMENT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_SRCFILTER: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_TOELEMENT: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_TYPE: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_X: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLEVENTOBJ_Y: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFIELDSETELEMENT2_FORM: u32 = 67540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFIELDSETELEMENT_ALIGN: u32 = 65609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFILTERSCOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFILTERSCOLLECTION_LENGTH: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFILTERSCOLLECTION__NEWENUM: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFONTELEMENT_COLOR: u32 = 70538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFONTELEMENT_FACE: u32 = 70554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFONTELEMENT_SIZE: u32 = 70555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFONTNAMESCOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFONTNAMESCOLLECTION_LENGTH: u32 = 1501u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFONTNAMESCOLLECTION__NEWENUM: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFONTSIZESCOLLECTION_FORFONT: u32 = 1503u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFONTSIZESCOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFONTSIZESCOLLECTION_LENGTH: u32 = 1502u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFONTSIZESCOLLECTION__NEWENUM: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT2_ACCEPTCHARSET: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT2_URNS: u32 = 1505u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT3_NAMEDITEM: u32 = 1506u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT4_IE8_ACTION: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT_ACTION: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT_DIR: u32 = 70653u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT_ELEMENTS: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT_ENCODING: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT_LENGTH: u32 = 1500u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT_METHOD: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT_ONRESET: u32 = 71548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT_ONSUBMIT: u32 = 71547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT_RESET: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT_SUBMIT: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT_TAGS: u32 = 1502u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT_TARGET: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFORMELEMENT__NEWENUM: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEBASE2_ALLOWTRANSPARENCY: u32 = 70742u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEBASE2_CONTENTWINDOW: u32 = 68545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEBASE2_ONLOAD: u32 = 71568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEBASE2_ONREADYSTATECHANGE: u32 = 71561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEBASE2_READYSTATE: u32 = 70652u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEBASE3_LONGDESC: u32 = 68546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEBASE_BORDER: u32 = 68538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEBASE_FRAMEBORDER: u32 = 68539u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEBASE_FRAMESPACING: u32 = 68540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEBASE_MARGINHEIGHT: u32 = 68542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEBASE_MARGINWIDTH: u32 = 68541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEBASE_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEBASE_NORESIZE: u32 = 68543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEBASE_SCROLLING: u32 = 68544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEBASE_SRC: u32 = 68536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEELEMENT2_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEELEMENT2_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEELEMENT3_CONTENTDOCUMENT: u32 = 69656u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEELEMENT3_IE8_FRAMEBORDER: u32 = 69659u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEELEMENT3_IE8_LONGDESC: u32 = 69658u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEELEMENT3_IE8_SRC: u32 = 69657u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMEELEMENT_BORDERCOLOR: u32 = 69537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESCOLLECTION2_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESCOLLECTION2_LENGTH: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT2_ONAFTERPRINT: u32 = 71603u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT2_ONBEFOREPRINT: u32 = 71602u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT3_ONHASHCHANGE: u32 = 71645u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT3_ONMESSAGE: u32 = 71646u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT3_ONOFFLINE: u32 = 71644u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT3_ONONLINE: u32 = 71643u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT3_ONSTORAGE: u32 = 71636u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT_BORDER: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT_BORDERCOLOR: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT_COLS: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT_FRAMEBORDER: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT_FRAMESPACING: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT_ONBEFOREUNLOAD: u32 = 71575u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT_ONLOAD: u32 = 71568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT_ONUNLOAD: u32 = 71569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLFRAMESETELEMENT_ROWS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLGENERICELEMENT_NAMEDRECORDSET: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLGENERICELEMENT_RECORDSET: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLHEADELEMENT2_IE8_PROFILE: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLHEADELEMENT_PROFILE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLHEADERELEMENT_ALIGN: u32 = 65608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLHRELEMENT_ALIGN: u32 = 65608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLHRELEMENT_COLOR: u32 = 70538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLHRELEMENT_NOSHADE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLHRELEMENT_SIZE: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLHRELEMENT_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLHTMLELEMENT_VERSION: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIFRAMEELEMENT2_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIFRAMEELEMENT2_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIFRAMEELEMENT3_CONTENTDOCUMENT: u32 = 69656u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIFRAMEELEMENT3_IE8_FRAMEBORDER: u32 = 69659u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIFRAMEELEMENT3_IE8_LONGDESC: u32 = 69658u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIFRAMEELEMENT3_IE8_SRC: u32 = 69657u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIFRAMEELEMENT_ALIGN: u32 = 65609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIFRAMEELEMENT_HSPACE: u32 = 69538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIFRAMEELEMENT_VSPACE: u32 = 69537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMAGEELEMENTFACTORY_CREATE: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT2_LONGDESC: u32 = 2019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT3_IE8_DYNSRC: u32 = 1154u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT3_IE8_LONGDESC: u32 = 1151u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT3_IE8_LOWSRC: u32 = 1153u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT3_IE8_VRML: u32 = 1152u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT4_NATURALHEIGHT: u32 = 1156u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT4_NATURALWIDTH: u32 = 1155u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_ALIGN: u32 = 65609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_ALT: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_BORDER: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_COMPLETE: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_DYNSRC: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_FILECREATEDDATE: u32 = 2012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_FILEMODIFIEDDATE: u32 = 2013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_FILESIZE: u32 = 2011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_FILEUPDATEDDATE: u32 = 2014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_HREF: u32 = 2016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_HSPACE: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_ISMAP: u32 = 2002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_LOOP: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_LOWSRC: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_MIMETYPE: u32 = 2010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_NAMEPROP: u32 = 2017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_ONABORT: u32 = 71564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_ONERROR: u32 = 71565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_ONLOAD: u32 = 71568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_PROTOCOL: u32 = 2015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_READYSTATE: u32 = 70652u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_SRC: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_START: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_USEMAP: u32 = 2008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_VRML: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_VSPACE: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIMGELEMENT_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTBUTTONELEMENT_CREATETEXTRANGE: u32 = 2006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTBUTTONELEMENT_DISABLED: u32 = 65612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTBUTTONELEMENT_FORM: u32 = 67540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTBUTTONELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTBUTTONELEMENT_STATUS: u32 = 2021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTBUTTONELEMENT_TYPE: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTBUTTONELEMENT_VALUE: u32 = 70637u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT2_ACCEPT: u32 = 2022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT2_USEMAP: u32 = 2023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT3_IE8_DYNSRC: u32 = 1153u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT3_IE8_LOWSRC: u32 = 1151u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT3_IE8_SRC: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT3_IE8_VRML: u32 = 1152u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_ALIGN: u32 = 65609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_ALT: u32 = 2010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_BORDER: u32 = 2012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_CHECKED: u32 = 2009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_COMPLETE: u32 = 2018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_CREATETEXTRANGE: u32 = 2006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_DEFAULTCHECKED: u32 = 2008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_DEFAULTVALUE: u32 = 70619u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_DISABLED: u32 = 65612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_DYNSRC: u32 = 2017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_FORM: u32 = 67540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_HSPACE: u32 = 2014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_INDETERMINATE: u32 = 2007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_LOOP: u32 = 2019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_LOWSRC: u32 = 2015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_MAXLENGTH: u32 = 2003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_ONABORT: u32 = 71564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_ONCHANGE: u32 = 71566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_ONERROR: u32 = 71565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_ONLOAD: u32 = 71568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_ONSELECT: u32 = 71546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_READONLY: u32 = 2005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_READYSTATE: u32 = 70652u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_SELECT: u32 = 2004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_SIZE: u32 = 2002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_SRC: u32 = 2011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_START: u32 = 2020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_STATUS: u32 = 2001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_TYPE: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_VALUE: u32 = 70637u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_VRML: u32 = 2016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_VSPACE: u32 = 2013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTELEMENT_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTFILEELEMENT_DISABLED: u32 = 65612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTFILEELEMENT_FORM: u32 = 67540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTFILEELEMENT_MAXLENGTH: u32 = 2003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTFILEELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTFILEELEMENT_ONCHANGE: u32 = 71566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTFILEELEMENT_ONSELECT: u32 = 71546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTFILEELEMENT_SELECT: u32 = 2004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTFILEELEMENT_SIZE: u32 = 2002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTFILEELEMENT_STATUS: u32 = 2021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTFILEELEMENT_TYPE: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTFILEELEMENT_VALUE: u32 = 70637u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTHIDDENELEMENT_CREATETEXTRANGE: u32 = 2006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTHIDDENELEMENT_DISABLED: u32 = 65612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTHIDDENELEMENT_FORM: u32 = 67540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTHIDDENELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTHIDDENELEMENT_STATUS: u32 = 2021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTHIDDENELEMENT_TYPE: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTHIDDENELEMENT_VALUE: u32 = 70637u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_ALIGN: u32 = 65609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_ALT: u32 = 2010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_BORDER: u32 = 2012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_COMPLETE: u32 = 2018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_DISABLED: u32 = 65612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_DYNSRC: u32 = 2017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_HSPACE: u32 = 2014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_LOOP: u32 = 2019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_LOWSRC: u32 = 2015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_ONABORT: u32 = 71564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_ONERROR: u32 = 71565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_ONLOAD: u32 = 71568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_READYSTATE: u32 = 70652u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_SRC: u32 = 2011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_START: u32 = 2020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_TYPE: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_VRML: u32 = 2016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_VSPACE: u32 = 2013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTIMAGE_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTRANGEELEMENT_ALT: u32 = 2010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTRANGEELEMENT_DISABLED: u32 = 65612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTRANGEELEMENT_MAX: u32 = 2029u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTRANGEELEMENT_MIN: u32 = 2028u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTRANGEELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTRANGEELEMENT_STEP: u32 = 2030u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTRANGEELEMENT_STEPDOWN: u32 = 2032u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTRANGEELEMENT_STEPUP: u32 = 2033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTRANGEELEMENT_TYPE: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTRANGEELEMENT_VALUE: u32 = 70637u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTRANGEELEMENT_VALUEASNUMBER: u32 = 2031u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT2_SELECTIONEND: u32 = 2026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT2_SELECTIONSTART: u32 = 2025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT2_SETSELECTIONRANGE: u32 = 2027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT_CREATETEXTRANGE: u32 = 2006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT_DEFAULTVALUE: u32 = 70619u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT_DISABLED: u32 = 65612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT_FORM: u32 = 67540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT_MAXLENGTH: u32 = 2003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT_ONCHANGE: u32 = 71566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT_ONSELECT: u32 = 71546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT_READONLY: u32 = 2005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT_SELECT: u32 = 2004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT_SIZE: u32 = 2002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT_STATUS: u32 = 2021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT_TYPE: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLINPUTTEXTELEMENT_VALUE: u32 = 70637u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIPRINTCOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIPRINTCOLLECTION_LENGTH: u32 = 1501u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLIPRINTCOLLECTION__NEWENUM: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLISINDEXELEMENT2_FORM: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLISINDEXELEMENT_ACTION: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLISINDEXELEMENT_PROMPT: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLABELELEMENT2_FORM: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLABELELEMENT_ACCESSKEY: u32 = 67541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLABELELEMENT_HTMLFOR: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLEGENDELEMENT2_FORM: u32 = 67540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLEGENDELEMENT_ALIGN: u32 = 65609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLIELEMENT_TYPE: u32 = 70553u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLIELEMENT_VALUE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLINKELEMENT2_TARGET: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLINKELEMENT3_CHARSET: u32 = 1018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLINKELEMENT3_HREFLANG: u32 = 1019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLINKELEMENT4_IE8_HREF: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLINKELEMENT5_SHEET: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLINKELEMENT_DISABLED: u32 = 65612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLINKELEMENT_HREF: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLINKELEMENT_MEDIA: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLINKELEMENT_ONERROR: u32 = 71565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLINKELEMENT_ONLOAD: u32 = 71568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLINKELEMENT_ONREADYSTATECHANGE: u32 = 71561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLINKELEMENT_READYSTATE: u32 = 70652u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLINKELEMENT_REL: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLINKELEMENT_REV: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLINKELEMENT_STYLESHEET: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLINKELEMENT_TYPE: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLISTELEMENT2_COMPACT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLOCATION_ASSIGN: u32 = 10u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLOCATION_HASH: u32 = 7u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLOCATION_HOST: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLOCATION_HOSTNAME: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLOCATION_HREF: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLOCATION_PATHNAME: u32 = 5u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLOCATION_PORT: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLOCATION_PROTOCOL: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLOCATION_RELOAD: u32 = 8u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLOCATION_REPLACE: u32 = 9u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLOCATION_SEARCH: u32 = 6u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLLOCATION_TOSTRING: u32 = 11u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMAPELEMENT_AREAS: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMAPELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMARQUEEELEMENT_BEHAVIOR: u32 = 6002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMARQUEEELEMENT_BGCOLOR: i32 = -501i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMARQUEEELEMENT_DIRECTION: u32 = 6001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMARQUEEELEMENT_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMARQUEEELEMENT_HSPACE: u32 = 6006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMARQUEEELEMENT_LOOP: u32 = 6004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMARQUEEELEMENT_ONBOUNCE: u32 = 71556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMARQUEEELEMENT_ONFINISH: u32 = 71562u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMARQUEEELEMENT_ONSTART: u32 = 71563u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMARQUEEELEMENT_SCROLLAMOUNT: u32 = 6003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMARQUEEELEMENT_SCROLLDELAY: u32 = 6000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMARQUEEELEMENT_START: u32 = 6010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMARQUEEELEMENT_STOP: u32 = 6011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMARQUEEELEMENT_TRUESPEED: u32 = 6007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMARQUEEELEMENT_VSPACE: u32 = 6005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMARQUEEELEMENT_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT2_CURRENTTIMEDOUBLE: u32 = 1027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT2_DEFAULTPLAYBACKRATEDOUBLE: u32 = 1030u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT2_DURATIONDOUBLE: u32 = 1029u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT2_INITIALTIMEDOUBLE: u32 = 1028u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT2_PLAYBACKRATEDOUBLE: u32 = 1031u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT2_VOLUMEDOUBLE: u32 = 1032u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_AUTOBUFFER: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_AUTOPLAY: u32 = 1019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_BUFFERED: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_CANPLAYTYPE: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_CONTROLS: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_CURRENTSRC: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_CURRENTTIME: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_DEFAULTPLAYBACKRATE: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_DURATION: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_ENDED: u32 = 1018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_ERROR: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_INITIALTIME: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_LOAD: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_LOOP: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_MUTED: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_NETWORKSTATE: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_PAUSE: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_PAUSED: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_PLAY: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_PLAYBACKRATE: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_PLAYED: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_PRELOAD: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_SEEKABLE: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_SEEKING: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_SRC: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAELEMENT_VOLUME: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMEDIAERROR_CODE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMETAELEMENT2_SCHEME: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMETAELEMENT3_IE8_URL: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMETAELEMENT_CHARSET: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMETAELEMENT_CONTENT: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMETAELEMENT_HTTPEQUIV: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMETAELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMETAELEMENT_URL: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMIMETYPESCOLLECTION_LENGTH: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMODELESSINIT_DOCUMENT: u32 = 25007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMODELESSINIT_MONIKER: u32 = 25006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMODELESSINIT_OPTIONSTRING: u32 = 25001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMODELESSINIT_PARAMETERS: u32 = 25000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMSCSSKEYFRAMERULE_KEYTEXT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMSCSSKEYFRAMERULE_STYLE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMSCSSKEYFRAMESRULE_APPENDRULE: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMSCSSKEYFRAMESRULE_CSSRULES: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMSCSSKEYFRAMESRULE_DELETERULE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMSCSSKEYFRAMESRULE_FINDRULE: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMSCSSKEYFRAMESRULE_NAME: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMSIMGELEMENT_MSPLAYTODISABLED: u32 = 1157u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMSIMGELEMENT_MSPLAYTOPRIMARY: u32 = 1158u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMSMEDIAELEMENT_MSPLAYTODISABLED: u32 = 1033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLMSMEDIAELEMENT_MSPLAYTOPRIMARY: u32 = 1034u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLNAMESPACECOLLECTION_ADD: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLNAMESPACECOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLNAMESPACECOLLECTION_LENGTH: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLNAMESPACE_ATTACHEVENT: u32 = 66043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLNAMESPACE_DETACHEVENT: u32 = 66044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLNAMESPACE_DOIMPORT: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLNAMESPACE_NAME: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLNAMESPACE_ONREADYSTATECHANGE: u32 = 71561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLNAMESPACE_READYSTATE: u32 = 70652u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLNAMESPACE_TAGNAMES: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLNAMESPACE_URN: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLNEXTIDELEMENT_N: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT2_CLASSID: u32 = 68538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT2_DATA: u32 = 68539u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT2_NAMEDRECORDSET: u32 = 68550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT3_ALT: u32 = 68552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT3_ARCHIVE: u32 = 68551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT3_BORDER: u32 = 68555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT3_DECLARE: u32 = 68553u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT3_STANDBY: u32 = 68554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT3_USEMAP: u32 = 68556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT4_CONTENTDOCUMENT: u32 = 68566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT4_IE8_CODEBASE: u32 = 68567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT4_IE8_DATA: u32 = 68568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT5_IE9_OBJECT: u32 = 68569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_ALIGN: u32 = 65609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_ALTHTML: u32 = 68547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_BASEHREF: u32 = 65538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_CLASSID: u32 = 68538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_CODE: u32 = 68544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_CODEBASE: u32 = 68542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_CODETYPE: u32 = 68543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_DATA: u32 = 68539u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_FORM: u32 = 67540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_HSPACE: u32 = 68549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_OBJECT: u32 = 68537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_ONERROR: u32 = 71565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_ONREADYSTATECHANGE: u32 = 71561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_READYSTATE: u32 = 68546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_RECORDSET: u32 = 68541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_TYPE: u32 = 68545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_VSPACE: u32 = 68548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOBJECTELEMENT_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOLISTELEMENT_COMPACT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOLISTELEMENT_START: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOLISTELEMENT_TYPE: u32 = 70553u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPSPROFILE_ADDREADREQUEST: u32 = 7u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPSPROFILE_ADDREQUEST: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPSPROFILE_CLEARREQUEST: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPSPROFILE_COMMITCHANGES: u32 = 6u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPSPROFILE_DOREADREQUEST: u32 = 8u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPSPROFILE_DOREQUEST: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPSPROFILE_DOWRITEREQUEST: u32 = 9u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPSPROFILE_GETATTRIBUTE: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPSPROFILE_SETATTRIBUTE: u32 = 5u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_CHECKED: u32 = 2009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_DEFAULTCHECKED: u32 = 2008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_DISABLED: u32 = 65612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_FORM: u32 = 67540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_INDETERMINATE: u32 = 2007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_ONCHANGE: u32 = 71566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_STATUS: u32 = 2001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_TYPE: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONBUTTONELEMENT_VALUE: u32 = 70637u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONELEMENT3_LABEL: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONELEMENT4_IE9_VALUE: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONELEMENTFACTORY_CREATE: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONELEMENT_DEFAULTSELECTED: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONELEMENT_FORM: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONELEMENT_INDEX: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONELEMENT_SELECTED: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONELEMENT_TEXT: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONELEMENT_VALUE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_ANYTHINGAFTERFRAMESET: u32 = 1513u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_CHOOSECOLORDLG: u32 = 1517u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_DOCUMENT: u32 = 1503u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_ERRORCHARACTER: u32 = 1507u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_ERRORCODE: u32 = 1508u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_ERRORDEBUG: u32 = 1510u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_ERRORLINE: u32 = 1506u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_ERRORMESSAGE: u32 = 1509u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_EXECARG: u32 = 1505u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_FINDTEXT: u32 = 1512u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_FONTS: u32 = 1504u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_GETCHARSET: u32 = 1520u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_ISAPARTMENTMODEL: u32 = 1519u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_OPENFILEDLG: u32 = 1515u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_SAVEFILEDLG: u32 = 1516u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_SECURECONNECTIONINFO: u32 = 1521u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_SHOWSECURITYINFO: u32 = 1518u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_SIZES: u32 = 1514u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLOPTIONSHOLDER_UNSECUREDWINDOWOFDOCUMENT: u32 = 1511u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPARAELEMENT_ALIGN: u32 = 65608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPARAMELEMENT2_IE8_VALUETYPE: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPARAMELEMENT2_NAME: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPARAMELEMENT2_TYPE: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPARAMELEMENT2_VALUE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPARAMELEMENT_NAME: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPARAMELEMENT_TYPE: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPARAMELEMENT_VALUE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPARAMELEMENT_VALUETYPE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCENAVIGATION_REDIRECTCOUNT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCENAVIGATION_TOJSON: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCENAVIGATION_TOSTRING: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCENAVIGATION_TYPE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_CONNECTEND: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_CONNECTSTART: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_DOMAINLOOKUPEND: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_DOMAINLOOKUPSTART: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_DOMCOMPLETE: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_DOMCONTENTLOADEDEVENTEND: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_DOMCONTENTLOADEDEVENTSTART: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_DOMINTERACTIVE: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_DOMLOADING: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_FETCHSTART: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_LOADEVENTEND: u32 = 1019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_LOADEVENTSTART: u32 = 1018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_MSFIRSTPAINT: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_NAVIGATIONSTART: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_REDIRECTEND: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_REDIRECTSTART: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_REQUESTSTART: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_RESPONSEEND: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_RESPONSESTART: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_TOJSON: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_TOSTRING: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_UNLOADEVENTEND: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCETIMING_UNLOADEVENTSTART: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCE_NAVIGATION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCE_TIMING: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCE_TOJSON: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPERFORMANCE_TOSTRING: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPHRASEELEMENT2_CITE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPHRASEELEMENT2_DATETIME: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPHRASEELEMENT3_IE8_CITE: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPLUGINSCOLLECTION_LENGTH: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPLUGINSCOLLECTION_REFRESH: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPOPUP_DOCUMENT: u32 = 27003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPOPUP_HIDE: u32 = 27002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPOPUP_ISOPEN: u32 = 27004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPOPUP_SHOW: u32 = 27001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPROGRESSELEMENT_FORM: u32 = 67540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPROGRESSELEMENT_MAX: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPROGRESSELEMENT_POSITION: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLPROGRESSELEMENT_VALUE: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRECT2_HEIGHT: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRECT2_WIDTH: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRECTCOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRECTCOLLECTION_LENGTH: u32 = 1500u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRECTCOLLECTION__NEWENUM: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRECT_BOTTOM: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRECT_LEFT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRECT_RIGHT: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRECT_TOP: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRENDERSTYLE_DEFAULTTEXTSELECTION: u32 = 70724u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRENDERSTYLE_RENDERINGPRIORITY: u32 = 70706u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRENDERSTYLE_TEXTBACKGROUNDCOLOR: u32 = 70705u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRENDERSTYLE_TEXTCOLOR: u32 = 70726u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRENDERSTYLE_TEXTDECORATION: u32 = 70727u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRENDERSTYLE_TEXTDECORATIONCOLOR: u32 = 70725u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRENDERSTYLE_TEXTEFFECT: u32 = 70704u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRENDERSTYLE_TEXTLINETHROUGHSTYLE: u32 = 70702u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRENDERSTYLE_TEXTUNDERLINESTYLE: u32 = 70703u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_ACCELERATOR: u32 = 70683u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_BEHAVIOR: u32 = 70651u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_BORDERCOLLAPSE: u32 = 70620u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_BOTTOM: u32 = 65614u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_DIRECTION: u32 = 70655u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_IMEMODE: u32 = 70656u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_LAYOUTGRID: u32 = 70667u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_LAYOUTGRIDCHAR: u32 = 70663u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_LAYOUTGRIDLINE: u32 = 70664u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_LAYOUTGRIDMODE: u32 = 70665u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_LAYOUTGRIDTYPE: u32 = 70666u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_LINEBREAK: u32 = 70669u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_OVERFLOWX: u32 = 70675u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_OVERFLOWY: u32 = 70676u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_PIXELBOTTOM: u32 = 69545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_PIXELRIGHT: u32 = 69546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_POSBOTTOM: u32 = 69547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_POSITION: u32 = 70626u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_POSRIGHT: u32 = 69548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_RIGHT: u32 = 65613u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_RUBYALIGN: u32 = 70657u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_RUBYOVERHANG: u32 = 70659u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_RUBYPOSITION: u32 = 70658u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_TABLELAYOUT: u32 = 70634u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_TEXTAUTOSPACE: u32 = 70668u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_TEXTJUSTIFY: u32 = 70671u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_TEXTJUSTIFYTRIM: u32 = 70672u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_TEXTKASHIDA: u32 = 70673u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_UNICODEBIDI: u32 = 70654u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE2_WORDBREAK: u32 = 70670u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE3_LAYOUTFLOW: u32 = 70691u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE3_SCROLLBAR3DLIGHTCOLOR: u32 = 70718u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE3_SCROLLBARARROWCOLOR: u32 = 70722u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE3_SCROLLBARBASECOLOR: u32 = 70716u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE3_SCROLLBARDARKSHADOWCOLOR: u32 = 70721u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE3_SCROLLBARFACECOLOR: u32 = 70717u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE3_SCROLLBARHIGHLIGHTCOLOR: u32 = 70720u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE3_SCROLLBARSHADOWCOLOR: u32 = 70719u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE3_SCROLLBARTRACKCOLOR: u32 = 70732u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE3_TEXTALIGNLAST: u32 = 70739u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE3_TEXTKASHIDASPACE: u32 = 70740u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE3_TEXTUNDERLINEPOSITION: u32 = 70695u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE3_WORDWRAP: u32 = 70694u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE3_WRITINGMODE: u32 = 70728u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE3_ZOOM: u32 = 70689u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE4_MINHEIGHT: u32 = 70747u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE4_TEXTOVERFLOW: u32 = 70745u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE5_MAXHEIGHT: u32 = 70750u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE5_MAXWIDTH: u32 = 70752u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE5_MINWIDTH: u32 = 70751u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE5_MSINTERPOLATIONMODE: u32 = 70749u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE6_BORDERSPACING: u32 = 70763u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE6_BOXSIZING: u32 = 70762u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE6_CAPTIONSIDE: u32 = 70755u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE6_CONTENT: u32 = 70754u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE6_COUNTERINCREMENT: u32 = 70756u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE6_COUNTERRESET: u32 = 70757u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE6_EMPTYCELLS: u32 = 70786u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE6_MSBLOCKPROGRESSION: u32 = 70787u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE6_ORPHANS: u32 = 70764u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE6_OUTLINE: u32 = 70758u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE6_OUTLINECOLOR: u32 = 70761u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE6_OUTLINESTYLE: u32 = 70760u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE6_OUTLINEWIDTH: u32 = 70759u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE6_PAGEBREAKINSIDE: u32 = 70766u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE6_QUOTES: u32 = 70788u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE6_WIDOWS: u32 = 70765u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BACKGROUND: u32 = 70568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BACKGROUNDATTACHMENT: u32 = 70581u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BACKGROUNDCOLOR: i32 = -501i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BACKGROUNDIMAGE: u32 = 70537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BACKGROUNDPOSITION: u32 = 70582u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BACKGROUNDPOSITIONX: u32 = 70569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BACKGROUNDPOSITIONY: u32 = 70570u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BACKGROUNDREPEAT: u32 = 70580u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDER: u32 = 70585u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERBOTTOM: u32 = 70588u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERBOTTOMCOLOR: u32 = 70593u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERBOTTOMSTYLE: u32 = 70603u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERBOTTOMWIDTH: u32 = 70598u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERCOLOR: u32 = 70590u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERLEFT: u32 = 70589u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERLEFTCOLOR: u32 = 70594u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERLEFTSTYLE: u32 = 70604u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERLEFTWIDTH: u32 = 70599u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERRIGHT: u32 = 70587u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERRIGHTCOLOR: u32 = 70592u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERRIGHTSTYLE: u32 = 70602u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERRIGHTWIDTH: u32 = 70597u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERSTYLE: u32 = 70600u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERTOP: u32 = 70586u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERTOPCOLOR: u32 = 70591u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERTOPSTYLE: u32 = 70601u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERTOPWIDTH: u32 = 70596u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_BORDERWIDTH: u32 = 70595u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_CLEAR: u32 = 70552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_CLIP: u32 = 70628u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_COLOR: u32 = 70538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_CSSTEXT: u32 = 70635u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_CURSOR: u32 = 70638u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_DISPLAY: u32 = 70607u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_FILTER: u32 = 70618u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_FONT: u32 = 70577u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_FONTFAMILY: u32 = 70554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_FONTSIZE: u32 = 70555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_FONTSTYLE: u32 = 70560u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_FONTVARIANT: u32 = 70561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_FONTWEIGHT: u32 = 70563u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_GETATTRIBUTE: u32 = 66038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_LEFT: u32 = 65539u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_LETTERSPACING: u32 = 70544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_LINEHEIGHT: u32 = 70542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_LISTSTYLE: u32 = 70611u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_LISTSTYLEIMAGE: u32 = 70610u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_LISTSTYLEPOSITION: u32 = 70609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_LISTSTYLETYPE: u32 = 70608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_MARGIN: u32 = 70572u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_MARGINBOTTOM: u32 = 70575u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_MARGINLEFT: u32 = 70576u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_MARGINRIGHT: u32 = 70574u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_MARGINTOP: u32 = 70573u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_OVERFLOW: u32 = 70546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_PADDING: u32 = 70547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_PADDINGBOTTOM: u32 = 70550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_PADDINGLEFT: u32 = 70551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_PADDINGRIGHT: u32 = 70549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_PADDINGTOP: u32 = 70548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_PAGEBREAKAFTER: u32 = 70614u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_PAGEBREAKBEFORE: u32 = 70613u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_POSITION: u32 = 70626u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_REMOVEATTRIBUTE: u32 = 66039u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_SETATTRIBUTE: u32 = 66037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_STYLEFLOAT: u32 = 70606u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_TEXTALIGN: u32 = 65608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_TEXTDECORATION: u32 = 70571u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_TEXTDECORATIONBLINK: u32 = 70558u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_TEXTDECORATIONLINETHROUGH: u32 = 70556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_TEXTDECORATIONNONE: u32 = 70559u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_TEXTDECORATIONOVERLINE: u32 = 70605u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_TEXTDECORATIONUNDERLINE: u32 = 70557u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_TEXTINDENT: u32 = 70543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_TEXTTRANSFORM: u32 = 70540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_TOP: u32 = 65540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_VERTICALALIGN: u32 = 70584u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_VISIBILITY: u32 = 70616u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_WHITESPACE: u32 = 70612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_WORDSPACING: u32 = 70583u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLRULESTYLE_ZINDEX: u32 = 70627u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCREEN2_DEVICEXDPI: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCREEN2_DEVICEYDPI: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCREEN2_LOGICALXDPI: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCREEN2_LOGICALYDPI: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCREEN3_SYSTEMXDPI: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCREEN3_SYSTEMYDPI: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCREEN4_PIXELDEPTH: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCREEN_AVAILHEIGHT: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCREEN_AVAILWIDTH: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCREEN_BUFFERDEPTH: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCREEN_COLORDEPTH: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCREEN_FONTSMOOTHINGENABLED: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCREEN_HEIGHT: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCREEN_UPDATEINTERVAL: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCREEN_WIDTH: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCRIPTELEMENT2_CHARSET: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCRIPTELEMENT3_IE8_SRC: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCRIPTELEMENT4_USEDCHARSET: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCRIPTELEMENT_DEFER: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCRIPTELEMENT_EVENT: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCRIPTELEMENT_HTMLFOR: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCRIPTELEMENT_ONERROR: u32 = 71565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCRIPTELEMENT_READYSTATE: u32 = 70652u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCRIPTELEMENT_SRC: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCRIPTELEMENT_TEXT: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSCRIPTELEMENT_TYPE: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT2_URNS: u32 = 1505u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT4_NAMEDITEM: u32 = 1506u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT5_IE8_ADD: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT6_IE9_ADD: u32 = 1151u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT6_IE9_VALUE: u32 = 1152u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT_ADD: u32 = 1503u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT_DISABLED: u32 = 65612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT_FORM: u32 = 67540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT_LENGTH: u32 = 1500u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT_MULTIPLE: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT_ONCHANGE: u32 = 71566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT_OPTIONS: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT_REMOVE: u32 = 1504u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT_SELECTEDINDEX: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT_SIZE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT_TAGS: u32 = 1502u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT_TYPE: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT_VALUE: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTELEMENT__NEWENUM: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTIONOBJECT2_CREATERANGECOLLECTION: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTIONOBJECT2_TYPEDETAIL: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTIONOBJECT_CLEAR: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTIONOBJECT_CREATERANGE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTIONOBJECT_EMPTY: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTIONOBJECT_TYPE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTION_ADDRANGE: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTION_ANCHORNODE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTION_ANCHOROFFSET: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTION_COLLAPSE: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTION_COLLAPSETOEND: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTION_COLLAPSETOSTART: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTION_DELETEFROMDOCUMENT: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTION_FOCUSNODE: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTION_FOCUSOFFSET: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTION_GETRANGEAT: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTION_ISCOLLAPSED: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTION_RANGECOUNT: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTION_REMOVEALLRANGES: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTION_REMOVERANGE: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTION_SELECTALLCHILDREN: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSELECTION_TOSTRING: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSOURCEELEMENT_MEDIA: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSOURCEELEMENT_SRC: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSOURCEELEMENT_TYPE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSPANFLOW_ALIGN: u32 = 65609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTORAGE2_IE9_SETITEM: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTORAGE_CLEAR: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTORAGE_GETITEM: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTORAGE_KEY: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTORAGE_LENGTH: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTORAGE_REMAININGSPACE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTORAGE_REMOVEITEM: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTORAGE_SETITEM: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_ACCELERATOR: u32 = 70683u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_BEHAVIOR: u32 = 70651u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_BORDERCOLLAPSE: u32 = 70620u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_BOTTOM: u32 = 65614u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_DIRECTION: u32 = 70655u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_GETEXPRESSION: u32 = 66041u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_IMEMODE: u32 = 70656u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_LAYOUTGRID: u32 = 70667u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_LAYOUTGRIDCHAR: u32 = 70663u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_LAYOUTGRIDLINE: u32 = 70664u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_LAYOUTGRIDMODE: u32 = 70665u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_LAYOUTGRIDTYPE: u32 = 70666u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_LINEBREAK: u32 = 70669u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_OVERFLOWX: u32 = 70675u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_OVERFLOWY: u32 = 70676u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_PIXELBOTTOM: u32 = 69545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_PIXELRIGHT: u32 = 69546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_POSBOTTOM: u32 = 69547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_POSITION: u32 = 70626u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_POSRIGHT: u32 = 69548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_REMOVEEXPRESSION: u32 = 66042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_RIGHT: u32 = 65613u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_RUBYALIGN: u32 = 70657u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_RUBYOVERHANG: u32 = 70659u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_RUBYPOSITION: u32 = 70658u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_SETEXPRESSION: u32 = 66040u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_TABLELAYOUT: u32 = 70634u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_TEXTAUTOSPACE: u32 = 70668u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_TEXTJUSTIFY: u32 = 70671u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_TEXTJUSTIFYTRIM: u32 = 70672u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_TEXTKASHIDA: u32 = 70673u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_UNICODEBIDI: u32 = 70654u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE2_WORDBREAK: u32 = 70670u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE3_LAYOUTFLOW: u32 = 70691u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE3_SCROLLBAR3DLIGHTCOLOR: u32 = 70718u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE3_SCROLLBARARROWCOLOR: u32 = 70722u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE3_SCROLLBARBASECOLOR: u32 = 70716u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE3_SCROLLBARDARKSHADOWCOLOR: u32 = 70721u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE3_SCROLLBARFACECOLOR: u32 = 70717u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE3_SCROLLBARHIGHLIGHTCOLOR: u32 = 70720u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE3_SCROLLBARSHADOWCOLOR: u32 = 70719u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE3_SCROLLBARTRACKCOLOR: u32 = 70732u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE3_TEXTALIGNLAST: u32 = 70739u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE3_TEXTKASHIDASPACE: u32 = 70740u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE3_TEXTUNDERLINEPOSITION: u32 = 70695u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE3_WORDWRAP: u32 = 70694u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE3_WRITINGMODE: u32 = 70728u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE3_ZOOM: u32 = 70689u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE4_MINHEIGHT: u32 = 70747u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE4_TEXTOVERFLOW: u32 = 70745u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE5_MAXHEIGHT: u32 = 70750u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE5_MAXWIDTH: u32 = 70752u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE5_MINWIDTH: u32 = 70751u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE5_MSINTERPOLATIONMODE: u32 = 70749u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE6_BORDERSPACING: u32 = 70763u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE6_BOXSIZING: u32 = 70762u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE6_CAPTIONSIDE: u32 = 70755u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE6_CONTENT: u32 = 70754u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE6_COUNTERINCREMENT: u32 = 70756u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE6_COUNTERRESET: u32 = 70757u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE6_EMPTYCELLS: u32 = 70786u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE6_MSBLOCKPROGRESSION: u32 = 70787u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE6_ORPHANS: u32 = 70764u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE6_OUTLINE: u32 = 70758u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE6_OUTLINECOLOR: u32 = 70761u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE6_OUTLINESTYLE: u32 = 70760u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE6_OUTLINEWIDTH: u32 = 70759u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE6_PAGEBREAKINSIDE: u32 = 70766u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE6_QUOTES: u32 = 70788u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE6_WIDOWS: u32 = 70765u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLEELEMENT2_SHEET: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLEELEMENT_DISABLED: u32 = 65612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLEELEMENT_MEDIA: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLEELEMENT_ONERROR: u32 = 71565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLEELEMENT_ONLOAD: u32 = 71568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLEELEMENT_ONREADYSTATECHANGE: u32 = 71561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLEELEMENT_READYSTATE: u32 = 70652u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLEELEMENT_STYLESHEET: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLEELEMENT_TYPE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLEENABLED_MSGETPROPERTYENABLED: u32 = 70043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLEENABLED_MSPUTPROPERTYENABLED: u32 = 70044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLEFONTFACE2_STYLE: u32 = 65610u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLEFONTFACE_FONTSRC: u32 = 70633u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLEMEDIA_MATCHMEDIUM: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLEMEDIA_TYPE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET2_ADDPAGERULE: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET2_PAGES: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET3_IE8_HREF: u32 = 1150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET3_ISALTERNATE: u32 = 1151u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET3_ISPREFALTERNATE: u32 = 1152u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET4_CSSRULES: u32 = 1158u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET4_DELETERULE: u32 = 1161u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET4_IE9_HREF: u32 = 1154u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET4_IE9_MEDIA: u32 = 1159u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET4_IE9_TITLE: u32 = 1155u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET4_IE9_TYPE: u32 = 1153u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET4_INSERTRULE: u32 = 1160u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET4_OWNERNODE: u32 = 1156u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET4_OWNERRULE: u32 = 1157u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETPAGE2_SELECTORTEXT: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETPAGE2_STYLE: u32 = 65610u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETPAGESCOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETPAGESCOLLECTION_LENGTH: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETPAGE_PSEUDOCLASS: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETPAGE_SELECTOR: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETRULE2_IE9_SELECTORTEXT: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETRULEAPPLIED_MSGETSPECIFICITY: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETRULEAPPLIED_MSSPECIFICITY: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETRULESAPPLIEDCOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETRULESAPPLIEDCOLLECTION_LENGTH: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETRULESAPPLIEDCOLLECTION_PROPERTYAPPLIEDBY: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETRULESAPPLIEDCOLLECTION_PROPERTYAPPLIEDTRACE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETRULESAPPLIEDCOLLECTION_PROPERTYAPPLIEDTRACELENGTH: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETRULESCOLLECTION2_IE9_ITEM: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETRULESCOLLECTION2_IE9_LENGTH: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETRULESCOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETRULESCOLLECTION_LENGTH: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETRULE_READONLY: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETRULE_SELECTORTEXT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETRULE_STYLE: u32 = 65610u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETSCOLLECTION2_IE9_ITEM: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETSCOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETSCOLLECTION_LENGTH: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEETSCOLLECTION__NEWENUM: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET_ADDIMPORT: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET_ADDRULE: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET_CSSTEXT: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET_DISABLED: u32 = 65612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET_HREF: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET_ID: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET_IMPORTS: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET_MEDIA: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET_OWNINGELEMENT: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET_PARENTSTYLESHEET: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET_READONLY: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET_REMOVEIMPORT: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET_REMOVERULE: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET_RULES: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET_TITLE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLESHEET_TYPE: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BACKGROUND: u32 = 70568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BACKGROUNDATTACHMENT: u32 = 70581u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BACKGROUNDCOLOR: i32 = -501i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BACKGROUNDIMAGE: u32 = 70537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BACKGROUNDPOSITION: u32 = 70582u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BACKGROUNDPOSITIONX: u32 = 70569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BACKGROUNDPOSITIONY: u32 = 70570u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BACKGROUNDREPEAT: u32 = 70580u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDER: u32 = 70585u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERBOTTOM: u32 = 70588u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERBOTTOMCOLOR: u32 = 70593u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERBOTTOMSTYLE: u32 = 70603u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERBOTTOMWIDTH: u32 = 70598u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERCOLOR: u32 = 70590u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERLEFT: u32 = 70589u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERLEFTCOLOR: u32 = 70594u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERLEFTSTYLE: u32 = 70604u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERLEFTWIDTH: u32 = 70599u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERRIGHT: u32 = 70587u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERRIGHTCOLOR: u32 = 70592u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERRIGHTSTYLE: u32 = 70602u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERRIGHTWIDTH: u32 = 70597u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERSTYLE: u32 = 70600u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERTOP: u32 = 70586u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERTOPCOLOR: u32 = 70591u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERTOPSTYLE: u32 = 70601u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERTOPWIDTH: u32 = 70596u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_BORDERWIDTH: u32 = 70595u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_CLEAR: u32 = 70552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_CLIP: u32 = 70628u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_COLOR: u32 = 70538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_CSSTEXT: u32 = 70635u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_CURSOR: u32 = 70638u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_DISPLAY: u32 = 70607u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_FILTER: u32 = 70618u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_FONT: u32 = 70577u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_FONTFAMILY: u32 = 70554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_FONTSIZE: u32 = 70555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_FONTSTYLE: u32 = 70560u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_FONTVARIANT: u32 = 70561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_FONTWEIGHT: u32 = 70563u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_GETATTRIBUTE: u32 = 66038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_LEFT: u32 = 65539u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_LETTERSPACING: u32 = 70544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_LINEHEIGHT: u32 = 70542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_LISTSTYLE: u32 = 70611u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_LISTSTYLEIMAGE: u32 = 70610u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_LISTSTYLEPOSITION: u32 = 70609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_LISTSTYLETYPE: u32 = 70608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_MARGIN: u32 = 70572u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_MARGINBOTTOM: u32 = 70575u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_MARGINLEFT: u32 = 70576u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_MARGINRIGHT: u32 = 70574u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_MARGINTOP: u32 = 70573u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_OVERFLOW: u32 = 70546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_PADDING: u32 = 70547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_PADDINGBOTTOM: u32 = 70550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_PADDINGLEFT: u32 = 70551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_PADDINGRIGHT: u32 = 70549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_PADDINGTOP: u32 = 70548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_PAGEBREAKAFTER: u32 = 70614u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_PAGEBREAKBEFORE: u32 = 70613u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_PIXELHEIGHT: u32 = 69539u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_PIXELLEFT: u32 = 69537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_PIXELTOP: u32 = 69536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_PIXELWIDTH: u32 = 69538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_POSHEIGHT: u32 = 69543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_POSITION: u32 = 70626u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_POSLEFT: u32 = 69541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_POSTOP: u32 = 69540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_POSWIDTH: u32 = 69542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_REMOVEATTRIBUTE: u32 = 66039u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_SETATTRIBUTE: u32 = 66037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_STYLEFLOAT: u32 = 70606u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_TEXTALIGN: u32 = 65608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_TEXTDECORATION: u32 = 70571u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_TEXTDECORATIONBLINK: u32 = 70558u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_TEXTDECORATIONLINETHROUGH: u32 = 70556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_TEXTDECORATIONNONE: u32 = 70559u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_TEXTDECORATIONOVERLINE: u32 = 70605u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_TEXTDECORATIONUNDERLINE: u32 = 70557u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_TEXTINDENT: u32 = 70543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_TEXTTRANSFORM: u32 = 70540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_TOP: u32 = 65540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_TOSTRING: u32 = 69544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_VERTICALALIGN: u32 = 70584u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_VISIBILITY: u32 = 70616u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_WHITESPACE: u32 = 70612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_WORDSPACING: u32 = 70583u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSTYLE_ZINDEX: u32 = 70627u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSUBMITDATA_APPENDITEMSEPARATOR: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSUBMITDATA_APPENDNAMEFILEPAIR: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLSUBMITDATA_APPENDNAMEVALUEPAIR: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE2_CELLS: u32 = 1037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE2_FIRSTPAGE: u32 = 1035u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE2_LASTPAGE: u32 = 1036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE2_MOVEROW: u32 = 1038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE3_SUMMARY: u32 = 1039u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE4_CREATETBODY: u32 = 1045u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE4_IE9_CAPTION: u32 = 1042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE4_IE9_DELETEROW: u32 = 1044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE4_IE9_INSERTROW: u32 = 1043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE4_IE9_TFOOT: u32 = 1041u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE4_IE9_THEAD: u32 = 1040u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECAPTION_ALIGN: u32 = 65608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECAPTION_VALIGN: u32 = 70567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL2_ABBR: u32 = 2004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL2_AXIS: u32 = 2005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL2_CH: u32 = 2006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL2_CHOFF: u32 = 2007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL2_HEADERS: u32 = 2008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL2_SCOPE: u32 = 2009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL3_IE9_CH: u32 = 2010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL3_IE9_CHOFF: u32 = 2011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL_ALIGN: u32 = 65608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL_BACKGROUND: u32 = 70537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL_BGCOLOR: i32 = -501i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL_BORDERCOLOR: u32 = 70564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL_BORDERCOLORDARK: u32 = 70566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL_BORDERCOLORLIGHT: u32 = 70565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL_CELLINDEX: u32 = 2003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL_COLSPAN: u32 = 2002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL_NOWRAP: u32 = 70541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL_ROWSPAN: u32 = 2001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL_VALIGN: u32 = 70567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECELL_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECOL2_CH: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECOL2_CHOFF: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECOL3_IE9_CH: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECOL3_IE9_CHOFF: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECOL_ALIGN: u32 = 65608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECOL_SPAN: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECOL_VALIGN: u32 = 70567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLECOL_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW2_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW3_CH: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW3_CHOFF: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW4_IE9_CH: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW4_IE9_CHOFF: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW4_IE9_DELETECELL: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW4_IE9_INSERTCELL: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROWMETRICS_CLIENTHEIGHT: u32 = 67555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROWMETRICS_CLIENTLEFT: u32 = 67558u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROWMETRICS_CLIENTTOP: u32 = 67557u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROWMETRICS_CLIENTWIDTH: u32 = 67556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW_ALIGN: u32 = 65608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW_BGCOLOR: i32 = -501i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW_BORDERCOLOR: u32 = 70564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW_BORDERCOLORDARK: u32 = 70566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW_BORDERCOLORLIGHT: u32 = 70565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW_CELLS: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW_DELETECELL: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW_INSERTCELL: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW_ROWINDEX: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW_SECTIONROWINDEX: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLEROW_VALIGN: u32 = 70567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLESECTION2_MOVEROW: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLESECTION3_CH: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLESECTION3_CHOFF: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLESECTION4_IE9_CH: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLESECTION4_IE9_CHOFF: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLESECTION4_IE9_DELETEROW: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLESECTION4_IE9_INSERTROW: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLESECTION_ALIGN: u32 = 65608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLESECTION_BGCOLOR: i32 = -501i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLESECTION_DELETEROW: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLESECTION_INSERTROW: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLESECTION_ROWS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLESECTION_VALIGN: u32 = 70567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_ALIGN: u32 = 65609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_BACKGROUND: u32 = 70537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_BGCOLOR: i32 = -501i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_BORDER: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_BORDERCOLOR: u32 = 70564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_BORDERCOLORDARK: u32 = 70566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_BORDERCOLORLIGHT: u32 = 70565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_CAPTION: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_CELLPADDING: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_CELLSPACING: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_COLS: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_CREATECAPTION: u32 = 1030u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_CREATETFOOT: u32 = 1028u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_CREATETHEAD: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_DATAPAGESIZE: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_DELETECAPTION: u32 = 1031u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_DELETEROW: u32 = 1033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_DELETETFOOT: u32 = 1029u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_DELETETHEAD: u32 = 1027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_FRAME: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_INSERTROW: u32 = 1032u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_NEXTPAGE: u32 = 1018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_ONREADYSTATECHANGE: u32 = 71561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_PREVIOUSPAGE: u32 = 1019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_READYSTATE: u32 = 70652u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_REFRESH: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_ROWS: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_RULES: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_TBODIES: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_TFOOT: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_THEAD: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTABLE_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT2_SELECTIONEND: u32 = 7008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT2_SELECTIONSTART: u32 = 7007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT2_SETSELECTIONRANGE: u32 = 7009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT_COLS: u32 = 7002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT_CREATETEXTRANGE: u32 = 7006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT_DEFAULTVALUE: u32 = 70619u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT_DISABLED: u32 = 65612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT_FORM: u32 = 67540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT_ONCHANGE: u32 = 71566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT_ONSELECT: u32 = 71546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT_READONLY: u32 = 7004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT_ROWS: u32 = 7001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT_SELECT: u32 = 7005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT_STATUS: u32 = 2001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT_TYPE: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT_VALUE: u32 = 70637u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTAREAELEMENT_WRAP: u32 = 7003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTCONTAINER_CREATECONTROLRANGE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTCONTAINER_ONSCROLL: u32 = 71567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTCONTAINER_SCROLLHEIGHT: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTCONTAINER_SCROLLLEFT: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTCONTAINER_SCROLLTOP: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTCONTAINER_SCROLLWIDTH: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTRANGEMETRICS2_GETBOUNDINGCLIENTRECT: u32 = 1042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTRANGEMETRICS2_GETCLIENTRECTS: u32 = 1041u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTRANGEMETRICS_BOUNDINGHEIGHT: u32 = 1040u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTRANGEMETRICS_BOUNDINGLEFT: u32 = 1038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTRANGEMETRICS_BOUNDINGTOP: u32 = 1037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTRANGEMETRICS_BOUNDINGWIDTH: u32 = 1039u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTRANGEMETRICS_OFFSETLEFT: u32 = 1036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTEXTRANGEMETRICS_OFFSETTOP: u32 = 1035u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTIMERANGES2_ENDDOUBLE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTIMERANGES2_STARTDOUBLE: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTIMERANGES_END: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTIMERANGES_LENGTH: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTIMERANGES_START: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTITLEELEMENT_TEXT: u32 = 70637u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGECOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGECOLLECTION_LENGTH: u32 = 1500u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGECOLLECTION__NEWENUM: i32 = -4i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGEINTERNAL_GET_VISIBLETEXT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_COLLAPSE: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_COMPAREENDPOINTS: u32 = 1018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_DUPLICATE: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_EXECCOMMAND: u32 = 1033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_EXECCOMMANDSHOWHELP: u32 = 1034u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_EXPAND: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_FINDTEXT: u32 = 1019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_GETBOOKMARK: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_HTMLTEXT: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_INRANGE: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_ISEQUAL: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_MOVE: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_MOVEEND: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_MOVESTART: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_MOVETOBOOKMARK: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_MOVETOELEMENTTEXT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_MOVETOPOINT: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_PARENTELEMENT: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_PASTEHTML: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_QUERYCOMMANDENABLED: u32 = 1028u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_QUERYCOMMANDINDETERM: u32 = 1030u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_QUERYCOMMANDSTATE: u32 = 1029u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_QUERYCOMMANDSUPPORTED: u32 = 1027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_QUERYCOMMANDTEXT: u32 = 1031u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_QUERYCOMMANDVALUE: u32 = 1032u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_SCROLLINTOVIEW: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_SELECT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_SETENDPOINT: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLTXTRANGE_TEXT: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLULISTELEMENT_COMPACT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLULISTELEMENT_TYPE: u32 = 70553u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLUNIQUENAME_UNIQUEID: u32 = 66591u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLUNIQUENAME_UNIQUENUMBER: u32 = 66590u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLURNCOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLURNCOLLECTION_LENGTH: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLVIDEOELEMENT_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLVIDEOELEMENT_POSTER: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLVIDEOELEMENT_VIDEOHEIGHT: u32 = 1051u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLVIDEOELEMENT_VIDEOWIDTH: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLVIDEOELEMENT_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_ALERT: u32 = 1105u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_BLUR: u32 = 1159u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_CLEARINTERVAL: u32 = 1163u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_CLEARTIMEOUT: u32 = 1104u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_CLIENTINFORMATION: u32 = 1161u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_CLOSE: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_CLOSED: u32 = 23u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_CONFIRM: u32 = 1110u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_DEFAULTSTATUS: u32 = 1101u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_DOCUMENT: u32 = 1151u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_EVENT: u32 = 1152u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_EXECSCRIPT: u32 = 1165u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_EXTERNAL: u32 = 1169u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_FOCUS: u32 = 1158u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_FRAMES: u32 = 1100u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_HISTORY: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_IMAGE: u32 = 1125u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_LOCATION: u32 = 14u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_MOVEBY: u32 = 7u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_MOVETO: u32 = 6u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_NAME: u32 = 11u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_NAVIGATE: u32 = 25u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_NAVIGATOR: u32 = 5u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_OFFSCREENBUFFERING: u32 = 1164u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_ONBEFOREUNLOAD: u32 = 71575u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_ONBLUR: u32 = 71551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_ONERROR: u32 = 71565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_ONFOCUS: u32 = 71550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_ONHELP: u32 = 71549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_ONLOAD: u32 = 71568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_ONRESIZE: u32 = 71572u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_ONSCROLL: u32 = 71567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_ONUNLOAD: u32 = 71569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_OPEN: u32 = 13u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_OPENER: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_OPTION: u32 = 1157u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_PARENT: u32 = 12u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_PROMPT: u32 = 1111u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_RESIZEBY: u32 = 8u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_RESIZETO: u32 = 9u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_SCREEN: u32 = 1156u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_SCROLL: u32 = 1160u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_SCROLLBY: u32 = 1167u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_SCROLLTO: u32 = 1168u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_SELF: u32 = 20u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_SETINTERVAL: u32 = 1173u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_SETTIMEOUT: u32 = 1172u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_SHOWHELP: u32 = 1155u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_SHOWMODALDIALOG: u32 = 1154u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_STATUS: u32 = 1102u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_TOP: u32 = 21u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_TOSTRING: u32 = 1166u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2_WINDOW: u32 = 22u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW2__NEWENUM: u32 = 1153u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW3_ATTACHEVENT: u32 = 66043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW3_CLIPBOARDDATA: u32 = 1175u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW3_DETACHEVENT: u32 = 66044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW3_ONAFTERPRINT: u32 = 71603u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW3_ONBEFOREPRINT: u32 = 71602u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW3_PRINT: u32 = 1174u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW3_SCREENLEFT: u32 = 1170u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW3_SCREENTOP: u32 = 1171u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW3_SETINTERVAL: u32 = 1162u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW3_SETTIMEOUT: u32 = 1103u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW3_SHOWMODELESSDIALOG: u32 = 1176u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW4_CREATEPOPUP: u32 = 1180u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW4_FRAMEELEMENT: u32 = 1181u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW5_XMLHTTPREQUEST: u32 = 1190u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW6_LOCALSTORAGE: u32 = 1193u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW6_MAXCONNECTIONSPERSERVER: u32 = 1194u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW6_MSWRITEPROFILERMARK: u32 = 1198u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW6_ONHASHCHANGE: u32 = 71645u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW6_ONMESSAGE: u32 = 71646u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW6_POSTMESSAGE: u32 = 1196u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW6_SESSIONSTORAGE: u32 = 1192u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW6_TOSTATICHTML: u32 = 1197u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW6_XDOMAINREQUEST: u32 = 1191u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_GETCOMPUTEDSTYLE: u32 = 1200u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_GETSELECTION: u32 = 1199u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_INNERHEIGHT: u32 = 1205u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_INNERWIDTH: u32 = 1204u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONABORT: u32 = 71564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONCANPLAY: u32 = 71670u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONCANPLAYTHROUGH: u32 = 71671u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONCHANGE: u32 = 71566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONCLICK: u32 = 71544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONCONTEXTMENU: u32 = 71601u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONDBLCLICK: u32 = 71545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONDRAG: u32 = 71585u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONDRAGEND: u32 = 71586u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONDRAGENTER: u32 = 71587u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONDRAGLEAVE: u32 = 71589u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONDRAGOVER: u32 = 71588u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONDRAGSTART: u32 = 71571u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONDROP: u32 = 71590u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONDURATIONCHANGE: u32 = 71672u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONEMPTIED: u32 = 71673u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONENDED: u32 = 71674u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONFOCUSIN: u32 = 71627u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONFOCUSOUT: u32 = 71628u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONINPUT: u32 = 71663u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONKEYDOWN: u32 = 71541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONKEYPRESS: u32 = 71543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONKEYUP: u32 = 71542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONLOADEDDATA: u32 = 71675u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONLOADEDMETADATA: u32 = 71676u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONLOADSTART: u32 = 71677u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONMOUSEDOWN: u32 = 71538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONMOUSEENTER: u32 = 71621u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONMOUSELEAVE: u32 = 71622u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONMOUSEMOVE: u32 = 71540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONMOUSEOUT: u32 = 71537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONMOUSEOVER: u32 = 71536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONMOUSEUP: u32 = 71539u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONMOUSEWHEEL: u32 = 71612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONOFFLINE: u32 = 71644u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONONLINE: u32 = 71643u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONPAUSE: u32 = 71678u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONPLAY: u32 = 71679u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONPLAYING: u32 = 71680u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONPROGRESS: u32 = 71681u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONRATECHANGE: u32 = 71682u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONREADYSTATECHANGE: u32 = 71561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONRESET: u32 = 71548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONSEEKED: u32 = 71683u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONSEEKING: u32 = 71684u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONSELECT: u32 = 71546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONSTALLED: u32 = 71685u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONSTORAGE: u32 = 71636u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONSUBMIT: u32 = 71547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONSUSPEND: u32 = 71686u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONTIMEUPDATE: u32 = 71687u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONVOLUMECHANGE: u32 = 71688u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_ONWAITING: u32 = 71689u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_OUTERHEIGHT: u32 = 1211u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_OUTERWIDTH: u32 = 1210u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_PAGEXOFFSET: u32 = 1206u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_PAGEYOFFSET: u32 = 1207u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_PERFORMANCE: u32 = 1203u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_SCREENX: u32 = 1208u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_SCREENY: u32 = 1209u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW7_STYLEMEDIA: u32 = 1202u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW8_APPLICATIONCACHE: u32 = 1213u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW8_ONMSGESTURECHANGE: u32 = 71700u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW8_ONMSGESTUREDOUBLETAP: u32 = 71704u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW8_ONMSGESTUREEND: u32 = 71701u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW8_ONMSGESTUREHOLD: u32 = 71702u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW8_ONMSGESTURESTART: u32 = 71699u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW8_ONMSGESTURETAP: u32 = 71703u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW8_ONMSINERTIASTART: u32 = 71705u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW8_ONMSPOINTERCANCEL: u32 = 71695u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW8_ONMSPOINTERDOWN: u32 = 71690u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW8_ONMSPOINTERHOVER: u32 = 71696u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW8_ONMSPOINTERMOVE: u32 = 71691u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW8_ONMSPOINTEROUT: u32 = 71694u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW8_ONMSPOINTEROVER: u32 = 71693u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW8_ONMSPOINTERUP: u32 = 71692u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLWINDOW8_ONPOPSTATE: u32 = 71728u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXDOMAINREQUESTFACTORY_CREATE: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXDOMAINREQUEST_ABORT: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXDOMAINREQUEST_CONTENTTYPE: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXDOMAINREQUEST_ONERROR: u32 = 71565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXDOMAINREQUEST_ONLOAD: u32 = 71568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXDOMAINREQUEST_ONPROGRESS: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXDOMAINREQUEST_ONTIMEOUT: u32 = 71648u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXDOMAINREQUEST_OPEN: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXDOMAINREQUEST_RESPONSETEXT: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXDOMAINREQUEST_SEND: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXDOMAINREQUEST_TIMEOUT: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXMLHTTPREQUEST2_ONTIMEOUT: u32 = 71648u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXMLHTTPREQUEST2_TIMEOUT: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXMLHTTPREQUESTFACTORY_CREATE: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXMLHTTPREQUEST_ABORT: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXMLHTTPREQUEST_GETALLRESPONSEHEADERS: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXMLHTTPREQUEST_GETRESPONSEHEADER: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXMLHTTPREQUEST_ONREADYSTATECHANGE: u32 = 71561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXMLHTTPREQUEST_OPEN: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXMLHTTPREQUEST_READYSTATE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXMLHTTPREQUEST_RESPONSEBODY: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXMLHTTPREQUEST_RESPONSETEXT: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXMLHTTPREQUEST_RESPONSEXML: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXMLHTTPREQUEST_SEND: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXMLHTTPREQUEST_SETREQUESTHEADER: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXMLHTTPREQUEST_STATUS: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IHTMLXMLHTTPREQUEST_STATUSTEXT: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ILINEINFO: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ILINEINFO_BASELINE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ILINEINFO_LINEDIRECTION: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ILINEINFO_TEXTDESCENT: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ILINEINFO_TEXTHEIGHT: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ILINEINFO_X: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IMG: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IMGBASE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IMPORT: u32 = 6u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IMPORTEXPORTFAVORITES: u32 = 9u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INAVIGATORDONOTTRACK_MSDONOTTRACK: u32 = 22u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INAVIGATORGEOLOCATION_GEOLOCATION: u32 = 21u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INITIALIZED: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INPRIVATEFILTERINGENABLED: u32 = 37u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INPUT: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INPUTIMAGE: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INPUTTEXT: u32 = 4000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INPUTTEXTBASE: u32 = 3000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ARIAATOMIC: u32 = 71179u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ARIAAUTOCOMPLETE: u32 = 71180u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ARIADROPEFFECT: u32 = 71181u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ARIAGRABBED: u32 = 71182u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ARIALABEL: u32 = 71183u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ARIAMULTILINE: u32 = 71184u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ARIAORIENTATION: u32 = 71185u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ARIASORT: u32 = 71186u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ARIAVALUETEXT: u32 = 71187u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ARYELEMENTRELEASENOTIFYPTRCACHE: u32 = 70712u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ARYOBJECTRELEASECLEANUPPTRCACHE: u32 = 70753u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_A_MS_HYPHENATE_LIMIT_AFTER: u32 = 71177u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_A_MS_HYPHENATE_LIMIT_BEFORE: u32 = 71176u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_A_MS_HYPHENATE_LIMIT_WORDS: u32 = 71175u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_BACKGROUNDDEFINITION: u32 = 71137u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_BGURLIMGCTXCACHEINDEX_GCAFTER: u32 = 70790u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_BGURLIMGCTXCACHEINDEX_GCBEFORE: u32 = 70789u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_BGURLIMGCTXCACHEINDEX_URLAFTER: u32 = 70792u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_BGURLIMGCTXCACHEINDEX_URLBEFORE: u32 = 70791u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_CATTRIBUTECOLLPTRCACHE: u32 = 70746u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_CATTRIBUTEPTRCACHE: u32 = 71169u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_CDOMCHILDRENPTRCACHE: u32 = 70662u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_CELEMENTCLASSCACHE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_CODEPAGESETTINGSPTRCACHE: u32 = 70708u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_COMPUTEFORMATSTATECACHE: u32 = 70715u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_CRUNTIMESTYLEPTRCACHE: u32 = 70644u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_CSS_PARSEDARY: u32 = 71211u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_CSS_TRACEDSTYLES: u32 = 71213u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_CSTYLEPTRCACHE: u32 = 70643u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_DATABINDTASKPTRCACHE: u32 = 70710u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_DWNDOCPTRCACHE: u32 = 70709u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_DWNHEADERCACHE: u32 = 70733u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_DWNPOSTPTRCACHE: u32 = 70707u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ERRORPAGEDWNPOST: u32 = 70997u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ERRORPAGEREASON: u32 = 70996u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ERRORPAGEREFRESHURL: u32 = 70995u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_FILTERNATIVEINFOPTRCACHE: u32 = 70692u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_FILTERPTRCACHE: u32 = 70649u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_FIRST: u32 = 71136u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_FOCUSITEMS: u32 = 70731u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_FONTFACEUNICODERANGE: u32 = 71170u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_FONTHISTORYINDEX: u32 = 70741u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_FRAMESCOLLECTION: u32 = 70736u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_GENERICCOMPLUSREF: u32 = 70730u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_GETTERSETTERCOLLECTION: u32 = 70794u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_INLINESTYLEAA: u32 = 70642u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_INVOKECONTEXT: u32 = 70645u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_INVOKECONTEXTDOCUMENT: u32 = 70748u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_LAYOUTRECTREGISTRYPTRCACHE: u32 = 70700u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_MEDIA_REFERENCE: u32 = 70729u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_NAMEDFLOWCOLLECTION: u32 = 71173u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ONBEHAVIOR_APPLYSTYLE: u32 = 70684u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ONBEHAVIOR_CONTENTREADY: u32 = 70660u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ONBEHAVIOR_CONTENTSAVE: u32 = 70723u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_ONBEHAVIOR_DOCUMENTREADY: u32 = 70661u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_PAGEFLOWCOLLECTION: u32 = 71172u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_PEERFACTORYURLMAPPTRCACHE: u32 = 70713u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_REQUIRED: u32 = 71210u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_RUNTIMESTYLEAA: u32 = 70685u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_SOURCELOCATION: u32 = 71212u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_STMDIRTYPTRCACHE: u32 = 70714u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_TOUCHTARGETHANDLER: u32 = 71171u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_URIBEFOREREDIRECT: u32 = 70809u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_URLLOCATIONCACHE: u32 = 70711u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INTERNAL_URLSEARCHCACHE: u32 = 70743u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_INVOKECONTEXTMENU: u32 = 8u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMHISTORY_BACK: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMHISTORY_FORWARD: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMHISTORY_GO: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMHISTORY_LENGTH: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_APPCODENAME: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_APPMINORVERSION: u32 = 17u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_APPNAME: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_APPVERSION: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_BROWSERLANGUAGE: u32 = 14u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_CONNECTIONSPEED: u32 = 18u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_COOKIEENABLED: u32 = 9u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_CPUCLASS: u32 = 12u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_JAVAENABLED: u32 = 5u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_MIMETYPES: u32 = 7u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_ONLINE: u32 = 19u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_OPSPROFILE: u32 = 10u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_PLATFORM: u32 = 16u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_PLUGINS: u32 = 8u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_SYSTEMLANGUAGE: u32 = 13u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_TAINTENABLED: u32 = 6u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_TOSTRING: u32 = 11u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_USERAGENT: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_USERLANGUAGE: u32 = 15u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IOMNAVIGATOR_USERPROFILE: u32 = 20u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER2_PERCENTSCALE: u32 = 509u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER2_SHOWHEADERFOOTER: u32 = 507u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER2_SHRINKTOFIT: u32 = 508u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER_DRAWPREVIEWPAGE: u32 = 502u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER_ENDPRINT: u32 = 506u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER_GETPRINTTASKOPTIONVALUE: u32 = 505u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER_INVALIDATEPREVIEW: u32 = 504u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER_SETPAGECOUNT: u32 = 503u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IPRINTMANAGERTEMPLATEPRINTER_STARTPRINT: u32 = 501u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IRANGEEXCEPTION_CODE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IRANGEEXCEPTION_MESSAGE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IRULESAPPLIEDCOLLECTION_ELEMENT: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IRULESAPPLIEDCOLLECTION_ITEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IRULESAPPLIEDCOLLECTION_LENGTH: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IRULESAPPLIEDCOLLECTION_PROPERTY: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IRULESAPPLIEDCOLLECTION_PROPERTYCOUNT: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IRULESAPPLIEDCOLLECTION_PROPERTYINHERITEDFROM: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IRULESAPPLIEDCOLLECTION_PROPERTYINHERITEDTRACE: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IRULESAPPLIEDCOLLECTION_PROPERTYINHERITEDTRACELENGTH: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IRULESAPPLIED_APPLIEDRULES: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IRULESAPPLIED_ELEMENT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IRULESAPPLIED_HASINHERITABLEPROPERTY: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IRULESAPPLIED_INLINESTYLES: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IRULESAPPLIED_PROPERTYISINHERITABLE: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IRULESAPPLIED_PROPERTYISINLINE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISMETAREFERRERAVAILABLE: u32 = 83u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISSEARCHMIGRATED: u32 = 25u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISSEARCHPROVIDERINSTALLED: u32 = 24u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISSERVICEINSTALLED: u32 = 31u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISSITEMODE: u32 = 43u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISSITEMODEFIRSTRUN: u32 = 59u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISSUBSCRIBED: u32 = 7u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGAELEMENT_TARGET: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANGLE_CONVERTTOSPECIFIEDUNITS: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANGLE_NEWVALUESPECIFIEDUNITS: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANGLE_UNITTYPE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANGLE_VALUE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANGLE_VALUEASSTRING: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANGLE_VALUEINSPECIFIEDUNITS: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDANGLE_ANIMVAL: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDANGLE_BASEVAL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDBOOLEAN_ANIMVAL: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDBOOLEAN_BASEVAL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDENUMERATION_ANIMVAL: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDENUMERATION_BASEVAL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDINTEGER_ANIMVAL: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDINTEGER_BASEVAL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDLENGTHLIST_ANIMVAL: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDLENGTHLIST_BASEVAL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDLENGTH_ANIMVAL: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDLENGTH_BASEVAL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDNUMBERLIST_ANIMVAL: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDNUMBERLIST_BASEVAL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDNUMBER_ANIMVAL: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDNUMBER_BASEVAL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDPATHDATA_ANIMATEDNORMALIZEDPATHSEGLIST: u32 = 1078u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDPATHDATA_ANIMATEDPATHSEGLIST: u32 = 1077u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDPATHDATA_NORMALIZEDPATHSEGLIST: u32 = 1076u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDPATHDATA_PATHSEGLIST: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDPOINTS_ANIMATEDPOINTS: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDPOINTS_POINTS: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDPRESERVEASPECTRATIO_ANIMVAL: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDPRESERVEASPECTRATIO_BASEVAL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDRECT_ANIMVAL: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDRECT_BASEVAL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDSTRING_ANIMVAL: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDSTRING_BASEVAL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDTRANSFORMLIST_ANIMVAL: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGANIMATEDTRANSFORMLIST_BASEVAL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGCIRCLEELEMENT_CX: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGCIRCLEELEMENT_CY: u32 = 1054u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGCIRCLEELEMENT_R: u32 = 1056u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGCLIPPATHELEMENT_CLIPPATHUNITS: u32 = 1051u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGDOCUMENT_ROOTELEMENT: u32 = 1116u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELEMENTINSTANCELIST_ITEM: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELEMENTINSTANCELIST_LENGTH: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELEMENTINSTANCE_CHILDNODES: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELEMENTINSTANCE_CORRESPONDINGELEMENT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELEMENTINSTANCE_CORRESPONDINGUSEELEMENT: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELEMENTINSTANCE_FIRSTCHILD: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELEMENTINSTANCE_LASTCHILD: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELEMENTINSTANCE_NEXTSIBLING: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELEMENTINSTANCE_PARENTNODE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELEMENTINSTANCE_PREVIOUSSIBLING: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELEMENT_FOCUSABLE: u32 = 1036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELEMENT_OWNERSVGELEMENT: u32 = 1033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELEMENT_VIEWPORTELEMENT: u32 = 1034u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELEMENT_XMLBASE: u32 = 1032u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELLIPSEELEMENT_CX: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELLIPSEELEMENT_CY: u32 = 1054u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELLIPSEELEMENT_RX: u32 = 1056u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGELLIPSEELEMENT_RY: u32 = 1058u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGEXCEPTION_CODE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGEXCEPTION_MESSAGE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGEXTERNALRESOURCESREQUIRED_EXTERNALRESOURCESREQUIRED: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGFITTOVIEWBOX_PRESERVEASPECTRATIO: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGFITTOVIEWBOX_VIEWBOX: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGGRADIENTELEMENT_GRADIENTTRANSFORM: u32 = 1053u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGGRADIENTELEMENT_GRADIENTUNITS: u32 = 1051u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGGRADIENTELEMENT_SPREADMETHOD: u32 = 1055u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGIMAGEELEMENT_HEIGHT: u32 = 1057u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGIMAGEELEMENT_WIDTH: u32 = 1055u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGIMAGEELEMENT_X: u32 = 1051u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGIMAGEELEMENT_Y: u32 = 1053u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLANGSPACE_XMLLANG: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLANGSPACE_XMLSPACE: u32 = 1018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLENGTHLIST_APPENDITEM: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLENGTHLIST_CLEAR: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLENGTHLIST_GETITEM: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLENGTHLIST_INITIALIZE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLENGTHLIST_INSERTITEMBEFORE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLENGTHLIST_NUMBEROFITEMS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLENGTHLIST_REMOVEITEM: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLENGTHLIST_REPLACEITEM: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLENGTH_CONVERTTOSPECIFIEDUNITS: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLENGTH_NEWVALUESPECIFIEDUNITS: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLENGTH_UNITTYPE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLENGTH_VALUE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLENGTH_VALUEASSTRING: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLENGTH_VALUEINSPECIFIEDUNITS: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLINEARGRADIENTELEMENT_X1: u32 = 1071u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLINEARGRADIENTELEMENT_X2: u32 = 1075u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLINEARGRADIENTELEMENT_Y1: u32 = 1073u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLINEARGRADIENTELEMENT_Y2: u32 = 1077u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLINEELEMENT_X1: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLINEELEMENT_X2: u32 = 1056u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLINEELEMENT_Y1: u32 = 1054u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLINEELEMENT_Y2: u32 = 1058u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLOCATABLE_FARTHESTVIEWPORTELEMENT: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLOCATABLE_GETBBOX: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLOCATABLE_GETCTM: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLOCATABLE_GETSCREENCTM: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLOCATABLE_GETTRANSFORMTOELEMENT: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGLOCATABLE_NEARESTVIEWPORTELEMENT: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMARKERELEMENT_MARKERHEIGHT: u32 = 1059u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMARKERELEMENT_MARKERUNITS: u32 = 1055u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMARKERELEMENT_MARKERWIDTH: u32 = 1057u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMARKERELEMENT_ORIENTANGLE: u32 = 1062u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMARKERELEMENT_ORIENTTYPE: u32 = 1061u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMARKERELEMENT_REFX: u32 = 1051u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMARKERELEMENT_REFY: u32 = 1053u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMARKERELEMENT_SETORIENTTOANGLE: u32 = 1064u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMARKERELEMENT_SETORIENTTOAUTO: u32 = 1063u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMASKELEMENT_HEIGHT: u32 = 1061u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMASKELEMENT_MASKCONTENTUNITS: u32 = 1053u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMASKELEMENT_MASKUNITS: u32 = 1051u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMASKELEMENT_WIDTH: u32 = 1059u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMASKELEMENT_X: u32 = 1055u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMASKELEMENT_Y: u32 = 1057u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_A: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_B: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_C: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_D: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_E: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_F: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_FLIPX: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_FLIPY: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_INVERSE: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_MULTIPLY: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_ROTATE: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_ROTATEFROMVECTOR: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_SCALE: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_SCALENONUNIFORM: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_SKEWX: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_SKEWY: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGMATRIX_TRANSLATE: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGNUMBERLIST_APPENDITEM: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGNUMBERLIST_CLEAR: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGNUMBERLIST_GETITEM: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGNUMBERLIST_INITIALIZE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGNUMBERLIST_INSERTITEMBEFORE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGNUMBERLIST_NUMBEROFITEMS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGNUMBERLIST_REMOVEITEM: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGNUMBERLIST_REPLACEITEM: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGNUMBER_VALUE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGARCABS: u32 = 1063u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGARCREL: u32 = 1064u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCLOSEPATH: u32 = 1054u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCURVETOCUBICABS: u32 = 1059u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCURVETOCUBICREL: u32 = 1060u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCURVETOCUBICSMOOTHABS: u32 = 1069u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCURVETOCUBICSMOOTHREL: u32 = 1070u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCURVETOQUADRATICABS: u32 = 1061u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCURVETOQUADRATICREL: u32 = 1062u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCURVETOQUADRATICSMOOTHABS: u32 = 1071u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGCURVETOQUADRATICSMOOTHREL: u32 = 1072u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGLINETOABS: u32 = 1057u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGLINETOHORIZONTALABS: u32 = 1065u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGLINETOHORIZONTALREL: u32 = 1066u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGLINETOREL: u32 = 1058u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGLINETOVERTICALABS: u32 = 1067u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGLINETOVERTICALREL: u32 = 1068u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGMOVETOABS: u32 = 1055u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_CREATESVGPATHSEGMOVETOREL: u32 = 1056u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_GETPATHSEGATLENGTH: u32 = 1075u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_GETPOINTATLENGTH: u32 = 1074u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_GETTOTALLENGTH: u32 = 1073u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHELEMENT_PATHLENGTH: u32 = 1053u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGARCABS_ANGLE: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGARCABS_LARGEARCFLAG: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGARCABS_R1: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGARCABS_R2: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGARCABS_SWEEPFLAG: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGARCABS_X: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGARCABS_Y: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGARCREL_ANGLE: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGARCREL_LARGEARCFLAG: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGARCREL_R1: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGARCREL_R2: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGARCREL_SWEEPFLAG: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGARCREL_X: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGARCREL_Y: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICABS_X: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICABS_X1: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICABS_X2: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICABS_Y: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICABS_Y1: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICABS_Y2: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICREL_X: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICREL_X1: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICREL_X2: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICREL_Y: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICREL_Y1: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICREL_Y2: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICSMOOTHABS_X: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICSMOOTHABS_X2: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICSMOOTHABS_Y: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICSMOOTHABS_Y2: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICSMOOTHREL_X: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICSMOOTHREL_X2: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICSMOOTHREL_Y: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOCUBICSMOOTHREL_Y2: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICABS_X: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICABS_X1: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICABS_Y: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICABS_Y1: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICREL_X: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICREL_X1: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICREL_Y: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICREL_Y1: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICSMOOTHABS_X: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICSMOOTHABS_Y: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICSMOOTHREL_X: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGCURVETOQUADRATICSMOOTHREL_Y: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGLINETOABS_X: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGLINETOABS_Y: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGLINETOHORIZONTALABS_X: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGLINETOHORIZONTALREL_X: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGLINETOREL_X: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGLINETOREL_Y: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGLINETOVERTICALABS_Y: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGLINETOVERTICALREL_Y: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGLIST_APPENDITEM: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGLIST_CLEAR: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGLIST_GETITEM: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGLIST_INITIALIZE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGLIST_INSERTITEMBEFORE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGLIST_NUMBEROFITEMS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGLIST_REMOVEITEM: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGLIST_REPLACEITEM: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGMOVETOABS_X: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGMOVETOABS_Y: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGMOVETOREL_X: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEGMOVETOREL_Y: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEG_PATHSEGTYPE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATHSEG_PATHSEGTYPEASLETTER: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATTERNELEMENT_HEIGHT: u32 = 1063u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATTERNELEMENT_PATTERNCONTENTUNITS: u32 = 1053u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATTERNELEMENT_PATTERNTRANSFORM: u32 = 1055u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATTERNELEMENT_PATTERNUNITS: u32 = 1051u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATTERNELEMENT_WIDTH: u32 = 1061u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATTERNELEMENT_X: u32 = 1057u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPATTERNELEMENT_Y: u32 = 1059u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPOINTLIST_APPENDITEM: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPOINTLIST_CLEAR: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPOINTLIST_GETITEM: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPOINTLIST_INITIALIZE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPOINTLIST_INSERTITEMBEFORE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPOINTLIST_NUMBEROFITEMS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPOINTLIST_REMOVEITEM: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPOINTLIST_REPLACEITEM: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPOINT_MATRIXTRANSFORM: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPOINT_X: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPOINT_Y: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPRESERVEASPECTRATIO_ALIGN: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGPRESERVEASPECTRATIO_MEETORSLICE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGRADIALGRADIENTELEMENT_CX: u32 = 1071u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGRADIALGRADIENTELEMENT_CY: u32 = 1073u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGRADIALGRADIENTELEMENT_FX: u32 = 1077u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGRADIALGRADIENTELEMENT_FY: u32 = 1079u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGRADIALGRADIENTELEMENT_R: u32 = 1075u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGRECTELEMENT_HEIGHT: u32 = 1058u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGRECTELEMENT_RX: u32 = 1060u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGRECTELEMENT_RY: u32 = 1062u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGRECTELEMENT_WIDTH: u32 = 1056u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGRECTELEMENT_X: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGRECTELEMENT_Y: u32 = 1054u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGRECT_HEIGHT: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGRECT_WIDTH: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGRECT_X: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGRECT_Y: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSCRIPTELEMENT_TYPE: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSTOPELEMENT_OFFSET: u32 = 1051u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSTRINGLIST_APPENDITEM: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSTRINGLIST_CLEAR: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSTRINGLIST_GETITEM: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSTRINGLIST_INITIALIZE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSTRINGLIST_INSERTITEMBEFORE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSTRINGLIST_NUMBEROFITEMS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSTRINGLIST_REMOVEITEM: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSTRINGLIST_REPLACEITEM: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSTYLABLE_CLASSNAME: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSTYLEELEMENT_MEDIA: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSTYLEELEMENT_TYPE: u32 = 1051u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_ANIMATIONSPAUSED: u32 = 1076u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_CHECKENCLOSURE: u32 = 1082u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_CHECKINTERSECTION: u32 = 1081u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_CONTENTSCRIPTTYPE: u32 = 1059u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_CONTENTSTYLETYPE: u32 = 1060u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_CREATESVGANGLE: u32 = 1086u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_CREATESVGLENGTH: u32 = 1085u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_CREATESVGMATRIX: u32 = 1088u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_CREATESVGNUMBER: u32 = 1084u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_CREATESVGPOINT: u32 = 1087u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_CREATESVGRECT: u32 = 1089u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_CREATESVGTRANSFORM: u32 = 1090u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_CREATESVGTRANSFORMFROMMATRIX: u32 = 1091u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_CURRENTSCALE: u32 = 1068u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_CURRENTTRANSLATE: u32 = 1069u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_CURRENTVIEW: u32 = 1067u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_DESELECTALL: u32 = 1083u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_FORCEREDRAW: u32 = 1073u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_GETCURRENTTIME: u32 = 1077u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_GETELEMENTBYID: u32 = 1092u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_GETENCLOSURELIST: u32 = 1080u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_GETINTERSECTIONLIST: u32 = 1079u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_HEIGHT: u32 = 1058u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_PAUSEANIMATIONS: u32 = 1074u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_PIXELUNITTOMILLIMETERX: u32 = 1062u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_PIXELUNITTOMILLIMETERY: u32 = 1063u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_SCREENPIXELTOMILLIMETERX: u32 = 1064u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_SCREENPIXELTOMILLIMETERY: u32 = 1065u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_SETCURRENTTIME: u32 = 1078u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_SUSPENDREDRAW: u32 = 1070u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_UNPAUSEANIMATIONS: u32 = 1075u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_UNSUSPENDREDRAW: u32 = 1071u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_UNSUSPENDREDRAWALL: u32 = 1072u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_USECURRENTVIEW: u32 = 1066u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_VIEWPORT: u32 = 1061u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_WIDTH: u32 = 1056u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_X: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGSVGELEMENT_Y: u32 = 1054u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTESTS_HASEXTENSION: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTESTS_REQUIREDEXTENSIONS: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTESTS_REQUIREDFEATURES: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTESTS_SYSTEMLANGUAGE: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTCONTENTELEMENT_GETCHARNUMATPOSITION: u32 = 1061u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTCONTENTELEMENT_GETCOMPUTEDTEXTLENGTH: u32 = 1055u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTCONTENTELEMENT_GETENDPOSITIONOFCHAR: u32 = 1058u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTCONTENTELEMENT_GETEXTENTOFCHAR: u32 = 1059u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTCONTENTELEMENT_GETNUMBEROFCHARS: u32 = 1054u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTCONTENTELEMENT_GETROTATIONOFCHAR: u32 = 1060u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTCONTENTELEMENT_GETSTARTPOSITIONOFCHAR: u32 = 1057u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTCONTENTELEMENT_GETSUBSTRINGLENGTH: u32 = 1056u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTCONTENTELEMENT_LENGTHADJUST: u32 = 1051u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTCONTENTELEMENT_SELECTSUBSTRING: u32 = 1062u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTCONTENTELEMENT_TEXTLENGTH: u32 = 1053u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTPATHELEMENT_METHOD: u32 = 1073u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTPATHELEMENT_SPACING: u32 = 1075u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTPATHELEMENT_STARTOFFSET: u32 = 1071u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTPOSITIONINGELEMENT_DX: u32 = 1075u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTPOSITIONINGELEMENT_DY: u32 = 1077u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTPOSITIONINGELEMENT_ROTATE: u32 = 1079u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTPOSITIONINGELEMENT_X: u32 = 1071u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTEXTPOSITIONINGELEMENT_Y: u32 = 1073u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORMABLE_TRANSFORM: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORMLIST_APPENDITEM: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORMLIST_CLEAR: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORMLIST_CONSOLIDATE: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORMLIST_CREATESVGTRANSFORMFROMMATRIX: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORMLIST_GETITEM: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORMLIST_INITIALIZE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORMLIST_INSERTITEMBEFORE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORMLIST_NUMBEROFITEMS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORMLIST_REMOVEITEM: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORMLIST_REPLACEITEM: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORM_ANGLE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORM_MATRIX: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORM_SETMATRIX: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORM_SETROTATE: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORM_SETSCALE: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORM_SETSKEWX: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORM_SETSKEWY: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORM_SETTRANSLATE: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGTRANSFORM_TYPE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGURIREFERENCE_HREF: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGUSEELEMENT_ANIMATEDINSTANCEROOT: u32 = 1060u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGUSEELEMENT_HEIGHT: u32 = 1058u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGUSEELEMENT_INSTANCEROOT: u32 = 1059u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGUSEELEMENT_WIDTH: u32 = 1056u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGUSEELEMENT_X: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGUSEELEMENT_Y: u32 = 1054u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGVIEWELEMENT_VIEWTARGET: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGZOOMANDPAN_ZOOMANDPAN: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGZOOMEVENT_NEWSCALE: u32 = 1279u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGZOOMEVENT_NEWTRANSLATE: u32 = 1280u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGZOOMEVENT_PREVIOUSSCALE: u32 = 1277u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGZOOMEVENT_PREVIOUSTRANSLATE: u32 = 1278u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ISVGZOOMEVENT_ZOOMRECTSCREEN: u32 = 1276u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER2_DEVICESUPPORTS: u32 = 41u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER2_FRAMEACTIVEENABLED: u32 = 38u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER2_ORIENTATION: u32 = 39u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER2_SELECTIONENABLED: u32 = 37u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER2_USEPRINTERCOPYCOLLATE: u32 = 40u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER3_GETPAGEMARGINBOTTOM: u32 = 45u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER3_GETPAGEMARGINBOTTOMIMPORTANT: u32 = 49u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER3_GETPAGEMARGINLEFT: u32 = 46u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER3_GETPAGEMARGINLEFTIMPORTANT: u32 = 50u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER3_GETPAGEMARGINRIGHT: u32 = 44u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER3_GETPAGEMARGINRIGHTIMPORTANT: u32 = 48u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER3_GETPAGEMARGINTOP: u32 = 43u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER3_GETPAGEMARGINTOPIMPORTANT: u32 = 47u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER3_HEADERFOOTERFONT: u32 = 42u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_ALLLINKEDDOCUMENTS: u32 = 23u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_COLLATE: u32 = 17u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_COPIES: u32 = 19u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_CURRENTPAGE: u32 = 15u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_CURRENTPAGEAVAIL: u32 = 16u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_DUPLEX: u32 = 18u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_ENSUREPRINTDIALOGDEFAULTS: u32 = 5u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_FOOTER: u32 = 25u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_FRAMEACTIVE: u32 = 11u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_FRAMEASSHOWN: u32 = 12u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_FRAMESETDOCUMENT: u32 = 10u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_HEADER: u32 = 24u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_MARGINBOTTOM: u32 = 29u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_MARGINLEFT: u32 = 26u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_MARGINRIGHT: u32 = 27u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_MARGINTOP: u32 = 28u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_PAGEFROM: u32 = 20u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_PAGEHEIGHT: u32 = 31u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_PAGETO: u32 = 21u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_PAGEWIDTH: u32 = 30u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_PRINTBLANKPAGE: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_PRINTNONNATIVE: u32 = 8u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_PRINTNONNATIVEFRAMES: u32 = 9u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_PRINTPAGE: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_SELECTEDPAGES: u32 = 14u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_SELECTION: u32 = 13u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_SHOWPAGESETUPDIALOG: u32 = 7u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_SHOWPRINTDIALOG: u32 = 6u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_STARTDOC: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_STOPDOC: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_TABLEOFLINKS: u32 = 22u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_UNPRINTABLEBOTTOM: u32 = 35u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_UNPRINTABLELEFT: u32 = 32u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_UNPRINTABLERIGHT: u32 = 34u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_UNPRINTABLETOP: u32 = 33u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ITEMPLATEPRINTER_UPDATEPAGESTATUS: u32 = 36u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWBSCRIPTCONTROL_BUBBLEEVENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWBSCRIPTCONTROL_FROZEN: u32 = 5u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWBSCRIPTCONTROL_ONVISIBILITYCHANGE: u32 = 10u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWBSCRIPTCONTROL_RAISEEVENT: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWBSCRIPTCONTROL_SCROLLBAR: u32 = 7u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWBSCRIPTCONTROL_SELECTABLECONTENT: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWBSCRIPTCONTROL_SETCONTEXTMENU: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWBSCRIPTCONTROL_VERSION: u32 = 8u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWBSCRIPTCONTROL_VISIBILITY: u32 = 9u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBBRIDGE_ABOUTBOX: i32 = -552i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBBRIDGE_EMBED: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBBRIDGE_EVENT: u32 = 1152u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBBRIDGE_READYSTATE: i32 = -525i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBBRIDGE_SCROLLBAR: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBBRIDGE_URL: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBGEOCOORDINATES_ACCURACY: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBGEOCOORDINATES_ALTITUDE: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBGEOCOORDINATES_ALTITUDEACCURACY: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBGEOCOORDINATES_HEADING: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBGEOCOORDINATES_LATITUDE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBGEOCOORDINATES_LONGITUDE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBGEOCOORDINATES_SPEED: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBGEOLOCATION_CLEARWATCH: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBGEOLOCATION_GETCURRENTPOSITION: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBGEOLOCATION_WATCHPOSITION: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBGEOPOSITIONERROR_CODE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBGEOPOSITIONERROR_MESSAGE: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBGEOPOSITION_COORDS: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_IWEBGEOPOSITION_TIMESTAMP: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_LABEL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_LAUNCHIE: u32 = 91u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_LAUNCHINHVSI: u32 = 99u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_LAUNCHINTERNETOPTIONS: u32 = 74u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_LAUNCHNETWORKCLIENTHELP: u32 = 67u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_LI: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_LINK: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_LOCATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_LOCATIONOBJECT: i32 = -5506i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MAP: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MARKUP: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MARQUEE: u32 = 6000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MEDIA: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MEDIAERROR: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MEDIALIST: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MEDIAQUERY: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MENU: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MIMETYPES_COL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MODE: u32 = 18u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MOVESELECTIONDOWN: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MOVESELECTIONTO: u32 = 9u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MOVESELECTIONUP: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSANIMATIONEND: u32 = 1095u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSANIMATIONITERATION: u32 = 1096u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSANIMATIONSTART: u32 = 1094u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSDATASRCINTERFACE: i32 = -3900i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSGESTURECHANGE: u32 = 1084u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSGESTUREDOUBLETAP: u32 = 1088u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSGESTUREEND: u32 = 1085u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSGESTUREHOLD: u32 = 1086u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSGESTUREINIT: u32 = 1097u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSGESTURESTART: u32 = 1083u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSGESTURETAP: u32 = 1087u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSGOTPOINTERCAPTURE: u32 = 1091u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSHTMLWEBVIEWELEMENT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSINERTIASTART: u32 = 1089u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSLOSTPOINTERCAPTURE: u32 = 1090u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSMANIPULATIONSTATECHANGED: u32 = 1098u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSORIENTATIONCHANGE: u32 = 1103u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSPOINTERCANCEL: u32 = 1081u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSPOINTERDOWN: u32 = 1076u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSPOINTERENTER: u32 = 1101u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSPOINTERHOVER: u32 = 1082u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSPOINTERLEAVE: u32 = 1102u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSPOINTERMOVE: u32 = 1077u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSPOINTEROUT: u32 = 1080u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSPOINTEROVER: u32 = 1079u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSPOINTERPOINT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSPOINTERUP: u32 = 1078u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSTRANSITIONEND: u32 = 1093u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_MSTRANSITIONSTART: u32 = 1092u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_NAMESPACE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_NAMESPACE_COLLECTION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_NAVIGATEANDFIND: u32 = 8u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_NAVIGATECOMPLETE: u32 = 101u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_NAVIGATECOMPLETE2: u32 = 252u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_NAVIGATEERROR: u32 = 271u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_NAVIGATETOSUGGESTEDSITES: u32 = 40u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_NAVIGATOR: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_NAVIGATOROBJECT: i32 = -5508i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_NEWFOLDER: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_NEWPROCESS: u32 = 284u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_NEWWINDOW: u32 = 107u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_NEWWINDOW2: u32 = 251u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_NEWWINDOW3: u32 = 273u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_NORMAL_FIRST: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_NSCOLUMNS: u32 = 21u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_OBJECT: u32 = 68536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_OBJECT_ORDINAL_BASE: u32 = 73536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_OBJECT_ORDINAL_MAX: u32 = 74535u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_OL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_OLESITE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_OMDOCUMENT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_OMRECT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_OMWINDOW: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONABORT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONACTIVATE: u32 = 1044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONADDRESSBAR: u32 = 261u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONAFTERPRINT: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONALERT: u32 = 1061u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONBEFOREACTIVATE: u32 = 1047u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONBEFOREDEACTIVATE: u32 = 1034u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONBEFOREEDITFOCUS: u32 = 1027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONBEFOREPRINT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONBEFOREUNLOAD: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONBOUNCE: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONCHANGE: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONCHANGEBLUR: u32 = 1019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONCHANGEFOCUS: u32 = 1018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONCLOSE: u32 = 1100u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONCOMPASSNEEDSCALIBRATION: u32 = 1108u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONCONTENTREADY: u32 = 1029u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONCONTEXTMENU: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONCONTROLSELECT: u32 = 1036u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONDEACTIVATE: u32 = 1045u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONDEVICEMOTION: u32 = 1105u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONDEVICEORIENTATION: u32 = 1104u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONDOMMUTATION: u32 = 1068u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONERROR: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONFINISH: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONFOCUSIN: u32 = 1048u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONFOCUSOUT: u32 = 1049u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONFULLSCREEN: u32 = 258u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONHASHCHANGE: u32 = 1066u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONHIDE: u32 = 1060u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONLAYOUT: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONLAYOUTCOMPLETE: u32 = 1030u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONLINKEDOVERFLOW: u32 = 1032u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONLOAD: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONMENUBAR: u32 = 256u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONMESSAGE: u32 = 1067u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONMOUSEENTER: u32 = 1042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONMOUSEHOVER: u32 = 1028u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONMOUSELEAVE: u32 = 1043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONMOUSEWHEEL: u32 = 1033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONMOVE: u32 = 1035u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONMOVEEND: u32 = 1039u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONMOVESTART: u32 = 1038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONMULTILAYOUTCLEANUP: u32 = 1046u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONOBJECTCONTENTSCROLLED: u32 = 1056u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONOFFLINE: u32 = 1065u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONONLINE: u32 = 1064u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONOPEN: u32 = 1099u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONPAGE: u32 = 1031u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONPAGEHIDE: u32 = 1107u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONPAGESHOW: u32 = 1106u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONPERSIST: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONPERSISTLOAD: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONPERSISTSAVE: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONPOPUPMENUEND: u32 = 1063u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONPOPUPMENUSTART: u32 = 1062u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONQUIT: u32 = 253u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONRESET: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONRESIZE: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONRESIZEEND: u32 = 1041u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONRESIZESTART: u32 = 1040u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONSCROLL: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONSELECT: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONSELECTADD: u32 = 1051u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONSELECTIONCHANGE: u32 = 1037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONSELECTREMOVE: u32 = 1052u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONSELECTWITHIN: u32 = 1053u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONSHOW: u32 = 1059u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONSTART: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONSTATUSBAR: u32 = 257u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONSTOP: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONSTORAGE: u32 = 1057u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONSTORAGECOMMIT: u32 = 1058u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONSUBMIT: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONSYSTEMSCROLLINGEND: u32 = 1055u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONSYSTEMSCROLLINGSTART: u32 = 1054u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONTHEATERMODE: u32 = 260u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONTOOLBAR: u32 = 255u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONUNLOAD: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONVALUECHANGE: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ONVISIBLE: u32 = 254u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_OPENFAVORITESPANE: u32 = 97u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_OPENFAVORITESSETTINGS: u32 = 98u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_OPTION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_OPTIONS_COL: u32 = 1500u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PARA: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PARAM: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PEER_HOLDER_BASE: u32 = 5000000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PERFORMANCE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PERFORMANCENAVIGATION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PERFORMANCEOBJECT: i32 = -5505i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PERFORMANCETIMING: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PERSISTDATA: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PHISHINGENABLED: u32 = 19u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PHRASE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PINNEDSITESTATE: u32 = 73u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PLAYTO: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PLAYTODEVICE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PLUGINS_COL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PRINTMANAGER_TEMPLATE_PRINTER: u32 = 501u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PRINTTEMPLATEINSTANTIATION: u32 = 225u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PRINTTEMPLATETEARDOWN: u32 = 226u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PRIVACYIMPACTEDSTATECHANGE: u32 = 272u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PROCESSINGINSTRUCTION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PROGRESS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PROGRESSCHANGE: u32 = 108u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PROPERTYCHANGE: u32 = 112u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PROTECTEDELEMENT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_PROVISIONNETWORKS: u32 = 62u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_QUIT: u32 = 103u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_RADIO: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_RANGE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_RANGEEXCEPTION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_REDIRECTXDOMAINBLOCKED: u32 = 286u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_REFRESHOFFLINEDESKTOP: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_REMOVESCHEDULEDTILENOTIFICATION: u32 = 80u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_REPORTSAFEURL: u32 = 63u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_RESETEXPERIMENTALFLAGS: u32 = 92u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_RESETFIRSTBOOTMODE: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_RESETSAFEMODE: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_RESETSORT: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_RETREATERROR: u32 = 11u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_RICHTEXT: u32 = 7000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_ROOT: u32 = 16u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_RULESAPPLIED: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_RULESAPPLIED_COLLECTION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_RUNONCEHASSHOWN: u32 = 28u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_RUNONCEREQUIREDSETTINGSCOMPLETE: u32 = 27u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_RUNONCESHOWN: u32 = 15u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SCHEDULEDTILENOTIFICATION: u32 = 79u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SCREEN: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SCRIPT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SEARCHGUIDEURL: u32 = 29u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SECURITYCTX: i32 = -5511i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SECURITYDOMAIN: i32 = -5514i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SELECT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SELECTEDITEM: u32 = 15u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SELECTEDITEMS: u32 = 24u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SELECTION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SELECTIONCHANGE: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SELECTOBJ: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SETACTIVITIESVISIBLE: u32 = 35u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SETDETAILSSTATE: u32 = 20u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SETEXPERIMENTALFLAG: u32 = 84u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SETEXPERIMENTALVALUE: u32 = 86u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SETMSDEFAULTS: u32 = 104u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SETNEEDHVSIAUTOLAUNCHFLAG: u32 = 101u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SETNEEDIEAUTOLAUNCHFLAG: u32 = 90u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SETPERERRSTATE: u32 = 22u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SETPHISHINGFILTERSTATUS: u32 = 282u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SETRECENTLYCLOSEDVISIBLE: u32 = 34u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SETROOT: u32 = 13u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SETSECURELOCKICON: u32 = 269u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SETSITEMODEICONOVERLAY: u32 = 44u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SETSITEMODEPROPERTIES: u32 = 50u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SETTHUMBNAILBUTTONS: u32 = 47u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SETVIEWTYPE: u32 = 23u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SHELLUIHELPERLAST: u32 = 105u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SHOWBROWSERUI: u32 = 13u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SHOWINPRIVATEHELP: u32 = 42u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SHOWTABSHELP: u32 = 41u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SITE: u32 = 67536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SITEMODEACTIVATE: u32 = 58u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SITEMODEADDBUTTONSTYLE: u32 = 54u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SITEMODEADDJUMPLISTITEM: u32 = 52u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SITEMODECLEARBADGE: u32 = 65u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SITEMODECLEARJUMPLIST: u32 = 53u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SITEMODECREATEJUMPLIST: u32 = 51u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SITEMODEREFRESHBADGE: u32 = 64u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SITEMODESHOWBUTTONSTYLE: u32 = 55u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SITEMODESHOWJUMPLIST: u32 = 56u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SKIPRUNONCE: u32 = 16u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SKIPTABSWELCOME: u32 = 21u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SOURCE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SQMENABLED: u32 = 18u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STARTBADGEUPDATE: u32 = 81u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STARTPERIODICUPDATE: u32 = 70u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STARTPERIODICUPDATEBATCH: u32 = 75u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STATUSTEXTCHANGE: u32 = 102u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STOPBADGEUPDATE: u32 = 82u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STOPPERIODICUPDATE: u32 = 69u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STYLE: u32 = 69536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STYLEELEMENT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STYLEMEDIA: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STYLEPAGE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STYLEPAGES_COL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STYLERULE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STYLERULES_COL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STYLESHEET: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STYLESHEETRULESAPPLIED_COLLECTION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STYLESHEETSCOLLECTION_NAMED_BASE: u32 = 1000000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STYLESHEETSCOLLECTION_NAMED_MAX: u32 = 1999999u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STYLESHEETSCOLLECTION_ORDINAL_BASE: u32 = 2000000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STYLESHEETSCOLLECTION_ORDINAL_MAX: u32 = 2999999u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_STYLESHEETS_COL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SUBSCRIPTIONSENABLED: u32 = 10u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SUGGESTEDSITESENABLED: u32 = 38u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGABORT: u32 = 1071u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGAELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGALTGLYPHDEFELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGALTGLYPHELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGALTGLYPHITEMELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANGLE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATECOLORELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATEDANGLE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATEDBOOLEAN: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATEDENUMERATION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATEDINTEGER: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATEDLENGTH: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATEDLENGTHLIST: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATEDNUMBER: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATEDNUMBERLIST: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATEDPOINTS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATEDPRESERVEASPECTRATIO: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATEDRECT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATEDSTRING: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATEDTRANSFORMLIST: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATEMOTIONELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGANIMATETRANSFORMELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGCIRCLEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGCLIPPATHELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGCOLOR_PROFILEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGCOMPONENTTRANSFERFUNCTIONELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGCURSORELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGDEFINITION_SRCELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGDEFSELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGDESCELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGELEMENT: u32 = 1030u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGELEMENTINSTANCE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGELEMENTINSTANCELIST: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGELEMENT_BASE: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGELLIPSEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGERROR: u32 = 1072u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGEXCEPTION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGEXTERNALRESOURCESREQUIRED_EXTERNALRESOURCESREQUIRED_ATTR: u32 = 1019u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGEXTERNALRESOURCESREQUIRED_EXTERNALRESOURCESREQUIRED_PROP: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFEBLENDELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFECOLORMATRIXELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFECOMPONENTTRANSFERELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFECOMPOSITEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFECONVOLVEMATRIXELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFEDIFFUSELIGHTINGELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFEDISPLACEMENTMAPELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFEDISTANTLIGHTELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFEFLOODELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFEFUNCAELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFEFUNCBELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFEFUNCGELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFEFUNCRELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFEGAUSSIANBLURELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFEIMAGEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFEMERGEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFEMERGENODEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFEMORPHOLOGYELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFEOFFSETELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFEPOINTLIGHTELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFESPECULARLIGHTINGELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFESPOTLIGHTELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFETILEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFETURBULENCEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFILTERELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFITTOVIEWBOX_PRESERVEASPECTRATIO_ATTR: u32 = 1023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFITTOVIEWBOX_PRESERVEASPECTRATIO_PROP: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFITTOVIEWBOX_VIEWBOX_ATTR: u32 = 1021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFITTOVIEWBOX_VIEWBOX_PROP: u32 = 1022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFONTELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFONT_FACEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFONT_FACE_FORMATELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFONT_FACE_NAMEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFONT_FACE_SRCELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFONT_FACE_URIELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGFOREIGNOBJECTELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGGELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGGLYPHELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGGLYPHREFELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGGRADIENTELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGHKERNELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGIMAGEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGLANGSPACE_XMLLANG: u32 = 1017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGLANGSPACE_XMLSPACE: u32 = 1018u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGLENGTH: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGLENGTHLIST: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGLINEARGRADIENTELEMENT: u32 = 1070u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGLINEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGLOAD: u32 = 1069u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGLOCATABLE_FARTHESTVIEWPORTELEMENT: u32 = 1003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGLOCATABLE_GETBBOX: u32 = 1004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGLOCATABLE_GETCTM: u32 = 1005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGLOCATABLE_GETSCREENCTM: u32 = 1006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGLOCATABLE_GETTRANSFORMTOELEMENT: u32 = 1007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGLOCATABLE_NEARESTVIEWPORTELEMENT: u32 = 1002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGMARKERELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGMASKELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGMATRIX: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGMETADATAELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGMISSING_GLYPHELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGMIXINS: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGMPATHELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGNUMBER: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGNUMBERLIST: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEG: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGARCABS: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGARCREL: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGCLOSEPATH: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGCURVETOCUBICABS: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGCURVETOCUBICREL: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGCURVETOCUBICSMOOTHABS: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGCURVETOCUBICSMOOTHREL: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGCURVETOQUADRATICABS: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGCURVETOQUADRATICREL: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGCURVETOQUADRATICSMOOTHABS: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGCURVETOQUADRATICSMOOTHREL: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGLINETOABS: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGLINETOHORIZONTALABS: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGLINETOHORIZONTALREL: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGLINETOREL: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGLINETOVERTICALABS: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGLINETOVERTICALREL: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGLIST: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGMOVETOABS: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEGMOVETOREL: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATHSEG_BASE: u32 = 1020u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPATTERNELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPOINT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPOINTLIST: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPOLYGONELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPOLYLINEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGPRESERVEASPECTRATIO: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGRADIALGRADIENTELEMENT: u32 = 1070u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGRECT: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGRECTELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGRESIZE: u32 = 1073u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGSCRIPTELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGSCROLL: u32 = 1074u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGSETELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGSTOPELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGSTRINGLIST: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGSTYLABLE_CLASSNAME_PROP: u32 = 1001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGSTYLEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGSVGELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGSWITCHELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGSYMBOLELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTESTS_HASEXTENSION: u32 = 1016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTESTS_REQUIREDEXTENSIONS_ATTR: u32 = 1012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTESTS_REQUIREDEXTENSIONS_PROP: u32 = 1013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTESTS_REQUIREDFEATURES_ATTR: u32 = 1010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTESTS_REQUIREDFEATURES_PROP: u32 = 1011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTESTS_SYSTEMLANGUAGE_ATTR: u32 = 1014u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTESTS_SYSTEMLANGUAGE_PROP: u32 = 1015u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTEXTCONTENTELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTEXTCONTENTELEMENT_BASE: u32 = 1070u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTEXTELEMENT: u32 = 1090u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTEXTPATHELEMENT: u32 = 1070u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTEXTPOSITIONINGELEMENT: u32 = 1070u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTEXTPOSITIONINGELEMENT_BASE: u32 = 1090u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTITLEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTRANSFORM: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTRANSFORMABLE_TRANSFORM_ATTR: u32 = 1008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTRANSFORMABLE_TRANSFORM_PROP: u32 = 1009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTRANSFORMLIST: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTREFELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGTSPANELEMENT: u32 = 1090u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGUNLOAD: u32 = 1070u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGURIREFERENCE_HREF: u32 = 1026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGUSEELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGVIEWELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGVKERNELEMENT: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGZOOM: u32 = 1075u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGZOOMANDPAN_ZOOMANDPAN: u32 = 1025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SVGZOOMEVENT: u32 = 1275u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_SYNCHRONIZE: u32 = 5u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_TABLE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_TABLECELL: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_TABLECOL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_TABLEROW: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_TABLESECTION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_TAGNAMES_COLLECTION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_TEMPLATE_PRINTER: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_TEXTAREA: u32 = 5000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_TEXTSITE: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_THIRDPARTYURLBLOCKED: u32 = 285u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_TIMERANGES: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_TITLECHANGE: u32 = 113u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_TITLEICONCHANGE: u32 = 114u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_TRACK: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_TRACKINGPROTECTIONENABLED: u32 = 60u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_TVFLAGS: u32 = 20u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_UL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_UNKNOWNPDL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_UNSELECTALL: u32 = 26u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_UPDATEPAGESTATUS: u32 = 227u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_UPDATETHUMBNAILBUTTON: u32 = 46u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_URN_COLL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_VIDEO: u32 = 1050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_VIEWUPDATE: u32 = 281u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WEBGEOCOORDINATES: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WEBGEOLOCATION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WEBGEOPOSITION: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WEBGEOPOSITION_ERROR: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WEBSOCKET: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WEBWORKERFINISHED: u32 = 289u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WEBWORKERSTARTED: u32 = 288u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WINDOW: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WINDOWACTIVATE: u32 = 111u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WINDOWCLOSING: u32 = 263u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WINDOWMOVE: u32 = 109u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WINDOWOBJECT: i32 = -5500i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WINDOWREGISTERED: u32 = 200u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WINDOWRESIZE: u32 = 110u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WINDOWREVOKED: u32 = 201u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WINDOWSETHEIGHT: u32 = 267u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WINDOWSETLEFT: u32 = 264u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WINDOWSETRESIZABLE: u32 = 262u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WINDOWSETTOP: u32 = 265u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WINDOWSETWIDTH: u32 = 266u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_WINDOWSTATECHANGED: u32 = 283u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_XDOMAINREQUEST: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_XMLDECL: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_XMLHTTPREQUEST: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_XMLSERIALIZER: u32 = 1000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_XOBJ_EXPANDO: u32 = 72536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DISPID_XOBJ_ORDINAL: u32 = 73536u32;
pub struct DISPLAY_BREAK(i32);
pub struct DISPLAY_GRAVITY(i32);
pub struct DISPLAY_MOVEUNIT(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_BGSOUNDS: u32 = 64u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_DLIMAGES: u32 = 16u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_DOWNLOADONLY: u32 = 2048u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_FORCEOFFLINE: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_NOFRAMES: u32 = 524288u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_NO_BEHAVIORS: u32 = 32768u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_NO_CLIENTPULL: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_NO_DLACTIVEXCTLS: u32 = 1024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_NO_FRAMEDOWNLOAD: u32 = 4096u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_NO_JAVA: u32 = 256u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_NO_METACHARSET: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_NO_RUNACTIVEXCTLS: u32 = 512u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_NO_SCRIPTS: u32 = 128u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_OFFLINE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_OFFLINEIFNOTCONNECTED: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_PRAGMA_NO_CACHE: u32 = 16384u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_RESYNCHRONIZE: u32 = 8192u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_SILENT: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_URL_ENCODING_DISABLE_UTF8: u32 = 131072u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_URL_ENCODING_ENABLE_UTF8: u32 = 262144u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const DLCTL_VIDEOS: u32 = 32u32;
pub struct DOCHOSTUIDBLCLK(i32);
pub struct DOCHOSTUIFLAG(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DOCHOSTUIINFO(i32);
pub struct DOCHOSTUITYPE(i32);
pub struct DOMBeforeUnloadEvent(i32);
pub struct DOMChildrenCollection(i32);
pub struct DOMCloseEvent(i32);
pub struct DOMCompositionEvent(i32);
pub struct DOMCustomEvent(i32);
pub struct DOMDocumentType(i32);
pub struct DOMDragEvent(i32);
pub struct DOMEvent(i32);
pub struct DOMException(i32);
pub struct DOMFocusEvent(i32);
pub struct DOMKeyboardEvent(i32);
pub struct DOMMSAnimationEvent(i32);
pub struct DOMMSManipulationEvent(i32);
pub struct DOMMSTransitionEvent(i32);
pub struct DOMMessageEvent(i32);
pub struct DOMMouseEvent(i32);
pub struct DOMMouseWheelEvent(i32);
pub struct DOMMutationEvent(i32);
pub struct DOMParser(i32);
pub struct DOMParserFactory(i32);
pub struct DOMProcessingInstruction(i32);
pub struct DOMProgressEvent(i32);
pub struct DOMSiteModeEvent(i32);
pub struct DOMStorageEvent(i32);
pub struct DOMTextEvent(i32);
pub struct DOMUIEvent(i32);
pub struct DOMWheelEvent(i32);
pub struct DOM_EVENT_PHASE(i32);
pub struct DWebBridgeEvents(pub *mut ::core::ffi::c_void);
pub struct DispApplicationCache(pub *mut ::core::ffi::c_void);
pub struct DispCEventObj(pub *mut ::core::ffi::c_void);
pub struct DispCPlugins(pub *mut ::core::ffi::c_void);
pub struct DispCPrintManagerTemplatePrinter(pub *mut ::core::ffi::c_void);
pub struct DispCanvasGradient(pub *mut ::core::ffi::c_void);
pub struct DispCanvasImageData(pub *mut ::core::ffi::c_void);
pub struct DispCanvasPattern(pub *mut ::core::ffi::c_void);
pub struct DispCanvasRenderingContext2D(pub *mut ::core::ffi::c_void);
pub struct DispCanvasTextMetrics(pub *mut ::core::ffi::c_void);
pub struct DispDOMBeforeUnloadEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMChildrenCollection(pub *mut ::core::ffi::c_void);
pub struct DispDOMCloseEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMCompositionEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMCustomEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMDocumentType(pub *mut ::core::ffi::c_void);
pub struct DispDOMDragEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMException(pub *mut ::core::ffi::c_void);
pub struct DispDOMFocusEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMKeyboardEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMMSAnimationEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMMSManipulationEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMMSTransitionEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMMessageEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMMouseEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMMouseWheelEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMMutationEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMParser(pub *mut ::core::ffi::c_void);
pub struct DispDOMProcessingInstruction(pub *mut ::core::ffi::c_void);
pub struct DispDOMProgressEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMSiteModeEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMStorageEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMTextEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMUIEvent(pub *mut ::core::ffi::c_void);
pub struct DispDOMWheelEvent(pub *mut ::core::ffi::c_void);
pub struct DispEventException(pub *mut ::core::ffi::c_void);
pub struct DispHTCAttachBehavior(pub *mut ::core::ffi::c_void);
pub struct DispHTCDefaultDispatch(pub *mut ::core::ffi::c_void);
pub struct DispHTCDescBehavior(pub *mut ::core::ffi::c_void);
pub struct DispHTCEventBehavior(pub *mut ::core::ffi::c_void);
pub struct DispHTCMethodBehavior(pub *mut ::core::ffi::c_void);
pub struct DispHTCPropertyBehavior(pub *mut ::core::ffi::c_void);
pub struct DispHTMLAnchorElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLAppBehavior(pub *mut ::core::ffi::c_void);
pub struct DispHTMLAreaElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLAreasCollection(pub *mut ::core::ffi::c_void);
pub struct DispHTMLAttributeCollection(pub *mut ::core::ffi::c_void);
pub struct DispHTMLAudioElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLBGsound(pub *mut ::core::ffi::c_void);
pub struct DispHTMLBRElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLBaseElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLBaseFontElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLBlockElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLBody(pub *mut ::core::ffi::c_void);
pub struct DispHTMLButtonElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLCSSImportRule(pub *mut ::core::ffi::c_void);
pub struct DispHTMLCSSMediaList(pub *mut ::core::ffi::c_void);
pub struct DispHTMLCSSMediaRule(pub *mut ::core::ffi::c_void);
pub struct DispHTMLCSSNamespaceRule(pub *mut ::core::ffi::c_void);
pub struct DispHTMLCSSRule(pub *mut ::core::ffi::c_void);
pub struct DispHTMLCSSStyleDeclaration(pub *mut ::core::ffi::c_void);
pub struct DispHTMLCanvasElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLCommentElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLCurrentStyle(pub *mut ::core::ffi::c_void);
pub struct DispHTMLDDElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLDListElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLDOMAttribute(pub *mut ::core::ffi::c_void);
pub struct DispHTMLDOMImplementation(pub *mut ::core::ffi::c_void);
pub struct DispHTMLDOMRange(pub *mut ::core::ffi::c_void);
pub struct DispHTMLDOMTextNode(pub *mut ::core::ffi::c_void);
pub struct DispHTMLDTElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLDefaults(pub *mut ::core::ffi::c_void);
pub struct DispHTMLDivElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLDivPosition(pub *mut ::core::ffi::c_void);
pub struct DispHTMLDocument(pub *mut ::core::ffi::c_void);
pub struct DispHTMLDocumentCompatibleInfo(pub *mut ::core::ffi::c_void);
pub struct DispHTMLDocumentCompatibleInfoCollection(pub *mut ::core::ffi::c_void);
pub struct DispHTMLElementCollection(pub *mut ::core::ffi::c_void);
pub struct DispHTMLEmbed(pub *mut ::core::ffi::c_void);
pub struct DispHTMLFieldSetElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLFontElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLFormElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLFrameBase(pub *mut ::core::ffi::c_void);
pub struct DispHTMLFrameElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLFrameSetSite(pub *mut ::core::ffi::c_void);
pub struct DispHTMLGenericElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLHRElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLHeadElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLHeaderElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLHistory(pub *mut ::core::ffi::c_void);
pub struct DispHTMLHtmlElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLIFrame(pub *mut ::core::ffi::c_void);
pub struct DispHTMLImg(pub *mut ::core::ffi::c_void);
pub struct DispHTMLInputElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLIsIndexElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLLIElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLLabelElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLLegendElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLLinkElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLListElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLLocation(pub *mut ::core::ffi::c_void);
pub struct DispHTMLMSCSSKeyframeRule(pub *mut ::core::ffi::c_void);
pub struct DispHTMLMSCSSKeyframesRule(pub *mut ::core::ffi::c_void);
pub struct DispHTMLMapElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLMarqueeElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLMediaElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLMediaError(pub *mut ::core::ffi::c_void);
pub struct DispHTMLMetaElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLNamespace(pub *mut ::core::ffi::c_void);
pub struct DispHTMLNamespaceCollection(pub *mut ::core::ffi::c_void);
pub struct DispHTMLNavigator(pub *mut ::core::ffi::c_void);
pub struct DispHTMLNextIdElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLNoShowElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLOListElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLObjectElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLOptionElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLParaElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLParamElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLPerformance(pub *mut ::core::ffi::c_void);
pub struct DispHTMLPerformanceNavigation(pub *mut ::core::ffi::c_void);
pub struct DispHTMLPerformanceTiming(pub *mut ::core::ffi::c_void);
pub struct DispHTMLPhraseElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLPopup(pub *mut ::core::ffi::c_void);
pub struct DispHTMLProgressElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLRenderStyle(pub *mut ::core::ffi::c_void);
pub struct DispHTMLRichtextElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLRuleStyle(pub *mut ::core::ffi::c_void);
pub struct DispHTMLScreen(pub *mut ::core::ffi::c_void);
pub struct DispHTMLScriptElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLSelectElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLSemanticElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLSourceElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLSpanElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLSpanFlow(pub *mut ::core::ffi::c_void);
pub struct DispHTMLStorage(pub *mut ::core::ffi::c_void);
pub struct DispHTMLStyle(pub *mut ::core::ffi::c_void);
pub struct DispHTMLStyleElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLStyleFontFace(pub *mut ::core::ffi::c_void);
pub struct DispHTMLStyleMedia(pub *mut ::core::ffi::c_void);
pub struct DispHTMLStyleSheet(pub *mut ::core::ffi::c_void);
pub struct DispHTMLStyleSheetPage(pub *mut ::core::ffi::c_void);
pub struct DispHTMLStyleSheetPagesCollection(pub *mut ::core::ffi::c_void);
pub struct DispHTMLStyleSheetRule(pub *mut ::core::ffi::c_void);
pub struct DispHTMLStyleSheetRulesAppliedCollection(pub *mut ::core::ffi::c_void);
pub struct DispHTMLStyleSheetRulesCollection(pub *mut ::core::ffi::c_void);
pub struct DispHTMLStyleSheetsCollection(pub *mut ::core::ffi::c_void);
pub struct DispHTMLTable(pub *mut ::core::ffi::c_void);
pub struct DispHTMLTableCaption(pub *mut ::core::ffi::c_void);
pub struct DispHTMLTableCell(pub *mut ::core::ffi::c_void);
pub struct DispHTMLTableCol(pub *mut ::core::ffi::c_void);
pub struct DispHTMLTableRow(pub *mut ::core::ffi::c_void);
pub struct DispHTMLTableSection(pub *mut ::core::ffi::c_void);
pub struct DispHTMLTextAreaElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLTextElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLTimeRanges(pub *mut ::core::ffi::c_void);
pub struct DispHTMLTitleElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLUListElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLUnknownElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLUrnCollection(pub *mut ::core::ffi::c_void);
pub struct DispHTMLVideoElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLW3CComputedStyle(pub *mut ::core::ffi::c_void);
pub struct DispHTMLWindow2(pub *mut ::core::ffi::c_void);
pub struct DispHTMLWindowProxy(pub *mut ::core::ffi::c_void);
pub struct DispHTMLWndOptionElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLWndSelectElement(pub *mut ::core::ffi::c_void);
pub struct DispHTMLXMLHttpRequest(pub *mut ::core::ffi::c_void);
pub struct DispIHTMLInputButtonElement(pub *mut ::core::ffi::c_void);
pub struct DispIHTMLInputFileElement(pub *mut ::core::ffi::c_void);
pub struct DispIHTMLInputImage(pub *mut ::core::ffi::c_void);
pub struct DispIHTMLInputTextElement(pub *mut ::core::ffi::c_void);
pub struct DispIHTMLOptionButtonElement(pub *mut ::core::ffi::c_void);
pub struct DispNodeIterator(pub *mut ::core::ffi::c_void);
pub struct DispRangeException(pub *mut ::core::ffi::c_void);
pub struct DispRulesApplied(pub *mut ::core::ffi::c_void);
pub struct DispRulesAppliedCollection(pub *mut ::core::ffi::c_void);
pub struct DispSVGAElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGCircleElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGClipPathElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGDefsElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGDescElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGElementInstance(pub *mut ::core::ffi::c_void);
pub struct DispSVGElementInstanceList(pub *mut ::core::ffi::c_void);
pub struct DispSVGEllipseElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGException(pub *mut ::core::ffi::c_void);
pub struct DispSVGGElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGGradientElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGImageElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGLineElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGLinearGradientElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGMarkerElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGMaskElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGMetadataElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegArcAbs(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegArcRel(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegClosePath(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegCurvetoCubicAbs(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegCurvetoCubicRel(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegCurvetoCubicSmoothAbs(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegCurvetoCubicSmoothRel(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegCurvetoQuadraticAbs(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegCurvetoQuadraticRel(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegCurvetoQuadraticSmoothAbs(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegCurvetoQuadraticSmoothRel(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegLinetoAbs(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegLinetoHorizontalAbs(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegLinetoHorizontalRel(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegLinetoRel(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegLinetoVerticalAbs(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegLinetoVerticalRel(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegMovetoAbs(pub *mut ::core::ffi::c_void);
pub struct DispSVGPathSegMovetoRel(pub *mut ::core::ffi::c_void);
pub struct DispSVGPatternElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGPolygonElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGPolylineElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGRadialGradientElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGRectElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGSVGElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGScriptElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGStopElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGStyleElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGSwitchElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGSymbolElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGTSpanElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGTextContentElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGTextElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGTextPathElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGTextPositioningElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGTitleElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGUseElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGViewElement(pub *mut ::core::ffi::c_void);
pub struct DispSVGZoomEvent(pub *mut ::core::ffi::c_void);
pub struct DispStaticNodeList(pub *mut ::core::ffi::c_void);
pub struct DispTreeWalker(pub *mut ::core::ffi::c_void);
pub struct DispWebGeocoordinates(pub *mut ::core::ffi::c_void);
pub struct DispWebGeolocation(pub *mut ::core::ffi::c_void);
pub struct DispWebGeoposition(pub *mut ::core::ffi::c_void);
pub struct DispWebGeopositionError(pub *mut ::core::ffi::c_void);
pub struct DispXDomainRequest(pub *mut ::core::ffi::c_void);
pub struct DispXMLHttpRequestEventTarget(pub *mut ::core::ffi::c_void);
pub struct DispXMLSerializer(pub *mut ::core::ffi::c_void);
pub struct DomConstructor(i32);
pub struct ELEMENTDESCRIPTOR_FLAGS(i32);
pub struct ELEMENTNAMESPACE_FLAGS(i32);
pub struct ELEMENT_ADJACENCY(i32);
pub struct ELEMENT_CORNER(i32);
pub struct ELEMENT_TAG_ID(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const E_SURFACE_DISCARDED: i32 = -2147434493i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const E_SURFACE_NODC: i32 = -2147434492i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const E_SURFACE_NOSURFACE: i32 = -2147434496i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const E_SURFACE_NOTMYDC: i32 = -2147434491i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const E_SURFACE_NOTMYPOINTER: i32 = -2147434494i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const E_SURFACE_UNKNOWN_FORMAT: i32 = -2147434495i32;
pub struct EventException(i32);
pub struct ExtensionValidationContexts(i32);
pub struct ExtensionValidationResults(i32);
pub struct FINDTEXT_FLAGS(i32);
pub struct FontNames(i32);
pub struct FramesCollection(i32);
pub struct HTCAttachBehavior(i32);
pub struct HTCDefaultDispatch(i32);
pub struct HTCDescBehavior(i32);
pub struct HTCEventBehavior(i32);
pub struct HTCMethodBehavior(i32);
pub struct HTCPropertyBehavior(i32);
pub struct HTMLAnchorElement(i32);
pub struct HTMLAnchorEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLAnchorEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLAppBehavior(i32);
pub struct HTMLAppFlag(i32);
pub struct HTMLAreaElement(i32);
pub struct HTMLAreaEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLAreaEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLAreasCollection(i32);
pub struct HTMLAttributeCollection(i32);
pub struct HTMLAudioElement(i32);
pub struct HTMLAudioElementFactory(i32);
pub struct HTMLBGsound(i32);
pub struct HTMLBRElement(i32);
pub struct HTMLBaseElement(i32);
pub struct HTMLBaseFontElement(i32);
pub struct HTMLBlockElement(i32);
pub struct HTMLBody(i32);
pub struct HTMLBorder(i32);
pub struct HTMLBorderStyle(i32);
pub struct HTMLButtonElement(i32);
pub struct HTMLButtonElementEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLButtonElementEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLCSSImportRule(i32);
pub struct HTMLCSSMediaList(i32);
pub struct HTMLCSSMediaRule(i32);
pub struct HTMLCSSNamespaceRule(i32);
pub struct HTMLCSSRule(i32);
pub struct HTMLCSSStyleDeclaration(i32);
pub struct HTMLCanvasElement(i32);
pub struct HTMLCaptionFlag(i32);
pub struct HTMLCommentElement(i32);
pub struct HTMLControlElementEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLControlElementEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLCurrentStyle(i32);
pub struct HTMLDDElement(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const HTMLDLG_ALLOW_UNKNOWN_THREAD: u32 = 512u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const HTMLDLG_MODAL: u32 = 32u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const HTMLDLG_MODELESS: u32 = 64u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const HTMLDLG_NOUI: u32 = 16u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const HTMLDLG_PRINT_TEMPLATE: u32 = 128u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const HTMLDLG_VERIFY: u32 = 256u32;
pub struct HTMLDListElement(i32);
pub struct HTMLDOMAttribute(i32);
pub struct HTMLDOMImplementation(i32);
pub struct HTMLDOMRange(i32);
pub struct HTMLDOMTextNode(i32);
pub struct HTMLDOMXmlSerializerFactory(i32);
pub struct HTMLDTElement(i32);
pub struct HTMLDefaults(i32);
pub struct HTMLDialog(i32);
pub struct HTMLDivElement(i32);
pub struct HTMLDivPosition(i32);
pub struct HTMLDlgBorder(i32);
pub struct HTMLDlgCenter(i32);
pub struct HTMLDlgEdge(i32);
pub struct HTMLDlgFlag(i32);
pub struct HTMLDocument(i32);
pub struct HTMLDocumentCompatibleInfo(i32);
pub struct HTMLDocumentCompatibleInfoCollection(i32);
pub struct HTMLDocumentEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLDocumentEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLDocumentEvents3(pub *mut ::core::ffi::c_void);
pub struct HTMLDocumentEvents4(pub *mut ::core::ffi::c_void);
pub struct HTMLElementCollection(i32);
pub struct HTMLElementEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLElementEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLElementEvents3(pub *mut ::core::ffi::c_void);
pub struct HTMLElementEvents4(pub *mut ::core::ffi::c_void);
pub struct HTMLEmbed(i32);
pub struct HTMLFieldSetElement(i32);
pub struct HTMLFontElement(i32);
pub struct HTMLFormElement(i32);
pub struct HTMLFormElementEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLFormElementEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLFrameBase(i32);
pub struct HTMLFrameElement(i32);
pub struct HTMLFrameSetSite(i32);
pub struct HTMLFrameSiteEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLFrameSiteEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLGenericElement(i32);
pub struct HTMLHRElement(i32);
pub struct HTMLHeadElement(i32);
pub struct HTMLHeaderElement(i32);
pub struct HTMLHistory(i32);
pub struct HTMLHtmlElement(i32);
pub struct HTMLIFrame(i32);
pub struct HTMLImageElementFactory(i32);
pub struct HTMLImg(i32);
pub struct HTMLImgEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLImgEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLInputButtonElement(i32);
pub struct HTMLInputElement(i32);
pub struct HTMLInputFileElement(i32);
pub struct HTMLInputFileElementEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLInputFileElementEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLInputImage(i32);
pub struct HTMLInputImageEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLInputImageEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLInputTextElement(i32);
pub struct HTMLInputTextElementEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLInputTextElementEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLIsIndexElement(i32);
pub struct HTMLLIElement(i32);
pub struct HTMLLabelElement(i32);
pub struct HTMLLabelEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLLabelEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLLegendElement(i32);
pub struct HTMLLinkElement(i32);
pub struct HTMLLinkElementEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLLinkElementEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLListElement(i32);
pub struct HTMLLocation(i32);
pub struct HTMLMSCSSKeyframeRule(i32);
pub struct HTMLMSCSSKeyframesRule(i32);
pub struct HTMLMapElement(i32);
pub struct HTMLMapEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLMapEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLMarqueeElement(i32);
pub struct HTMLMarqueeElementEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLMarqueeElementEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLMaximizeFlag(i32);
pub struct HTMLMediaElement(i32);
pub struct HTMLMediaError(i32);
pub struct HTMLMetaElement(i32);
pub struct HTMLMinimizeFlag(i32);
pub struct HTMLNamespace(i32);
pub struct HTMLNamespaceCollection(i32);
pub struct HTMLNamespaceEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLNavigator(i32);
pub struct HTMLNextIdElement(i32);
pub struct HTMLNoShowElement(i32);
pub struct HTMLOListElement(i32);
pub struct HTMLObjectElement(i32);
pub struct HTMLObjectElementEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLObjectElementEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLOptionButtonElement(i32);
pub struct HTMLOptionButtonElementEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLOptionButtonElementEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLOptionElement(i32);
pub struct HTMLOptionElementFactory(i32);
pub struct HTMLParaElement(i32);
pub struct HTMLParamElement(i32);
pub struct HTMLPerformance(i32);
pub struct HTMLPerformanceNavigation(i32);
pub struct HTMLPerformanceTiming(i32);
pub struct HTMLPersistEvents(i32);
pub struct HTMLPhraseElement(i32);
pub struct HTMLPopup(i32);
pub struct HTMLProgressElement(i32);
pub struct HTMLRenderStyle(i32);
pub struct HTMLRichtextElement(i32);
pub struct HTMLRuleStyle(i32);
pub struct HTMLScreen(i32);
pub struct HTMLScriptElement(i32);
pub struct HTMLScriptEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLScriptEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLSelectElement(i32);
pub struct HTMLSelectElementEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLSelectElementEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLSemanticElement(i32);
pub struct HTMLSourceElement(i32);
pub struct HTMLSpanElement(i32);
pub struct HTMLSpanFlow(i32);
pub struct HTMLStorage(i32);
pub struct HTMLStyle(i32);
pub struct HTMLStyleElement(i32);
pub struct HTMLStyleElementEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLStyleElementEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLStyleFontFace(i32);
pub struct HTMLStyleMedia(i32);
pub struct HTMLStyleSheet(i32);
pub struct HTMLStyleSheetPage(i32);
pub struct HTMLStyleSheetPagesCollection(i32);
pub struct HTMLStyleSheetRule(i32);
pub struct HTMLStyleSheetRulesAppliedCollection(i32);
pub struct HTMLStyleSheetRulesCollection(i32);
pub struct HTMLStyleSheetsCollection(i32);
pub struct HTMLSysMenuFlag(i32);
pub struct HTMLTable(i32);
pub struct HTMLTableCaption(i32);
pub struct HTMLTableCell(i32);
pub struct HTMLTableCol(i32);
pub struct HTMLTableEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLTableEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLTableRow(i32);
pub struct HTMLTableSection(i32);
pub struct HTMLTextAreaElement(i32);
pub struct HTMLTextContainerEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLTextContainerEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLTextElement(i32);
pub struct HTMLTimeRanges(i32);
pub struct HTMLTitleElement(i32);
pub struct HTMLUListElement(i32);
pub struct HTMLUnknownElement(i32);
pub struct HTMLUrnCollection(i32);
pub struct HTMLVideoElement(i32);
pub struct HTMLW3CComputedStyle(i32);
pub struct HTMLWindow2(i32);
pub struct HTMLWindowEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLWindowEvents2(pub *mut ::core::ffi::c_void);
pub struct HTMLWindowEvents3(pub *mut ::core::ffi::c_void);
pub struct HTMLWindowProxy(i32);
pub struct HTMLWindowState(i32);
pub struct HTMLWndOptionElement(i32);
pub struct HTMLWndSelectElement(i32);
pub struct HTMLXMLHttpRequest(i32);
pub struct HTMLXMLHttpRequestEvents(pub *mut ::core::ffi::c_void);
pub struct HTMLXMLHttpRequestFactory(i32);
pub struct HTML_PAINTER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct HTML_PAINTER_INFO(i32);
pub struct HTML_PAINT_DRAW_FLAGS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct HTML_PAINT_DRAW_INFO(i32);
pub struct HTML_PAINT_DRAW_INFO_FLAGS(i32);
pub struct HTML_PAINT_EVENT_FLAGS(i32);
pub struct HTML_PAINT_XFORM(i32);
pub struct HTML_PAINT_ZORDER(i32);
pub struct HT_OPTIONS(i32);
pub struct HT_RESULTS(i32);
pub struct HomePage(i32);
pub struct HomePageSetting(i32);
pub struct HostDialogHelper(i32);
pub struct HtmlDlgSafeHelper(i32);
pub struct IActiveXUIHandlerSite(pub *mut ::core::ffi::c_void);
pub struct IActiveXUIHandlerSite2(pub *mut ::core::ffi::c_void);
pub struct IActiveXUIHandlerSite3(pub *mut ::core::ffi::c_void);
pub struct IAnchorClick(pub *mut ::core::ffi::c_void);
pub struct IAudioSessionSite(pub *mut ::core::ffi::c_void);
pub struct IBFCacheable(pub *mut ::core::ffi::c_void);
pub struct IBlockFormats(pub *mut ::core::ffi::c_void);
pub struct ICSSFilter(pub *mut ::core::ffi::c_void);
pub struct ICSSFilterSite(pub *mut ::core::ffi::c_void);
pub struct ICanvasGradient(pub *mut ::core::ffi::c_void);
pub struct ICanvasImageData(pub *mut ::core::ffi::c_void);
pub struct ICanvasPattern(pub *mut ::core::ffi::c_void);
pub struct ICanvasPixelArray(pub *mut ::core::ffi::c_void);
pub struct ICanvasPixelArrayData(pub *mut ::core::ffi::c_void);
pub struct ICanvasRenderingContext2D(pub *mut ::core::ffi::c_void);
pub struct ICanvasTextMetrics(pub *mut ::core::ffi::c_void);
pub struct ICaretPositionProvider(pub *mut ::core::ffi::c_void);
pub struct IClassFactoryEx(pub *mut ::core::ffi::c_void);
pub struct IClientCaps(pub *mut ::core::ffi::c_void);
pub struct ICustomDoc(pub *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_1D: u32 = 2170u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_1D_ELEMENT: u32 = 2396u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_2D_ELEMENT: u32 = 2395u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_2D_POSITION: u32 = 2394u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ABSOLUTE_POSITION: u32 = 2397u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ACTIVEXFILTERINGENABLED: u32 = 15030u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ACTIVEXINSTALLSCOPE: u32 = 15007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ADDCONSOLEMESSAGERECEIVER: u32 = 3800u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ADDDEBUGCALLBACKRECEIVER: u32 = 3804u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ADDFAVORITES: u32 = 2261u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ADDPARTIALTESTSTEPCOUNT: u32 = 15023u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ADDPDFHIGHLIGHT: u32 = 15210u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ADDPDFNOTE: u32 = 15212u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ADDRESS: u32 = 2189u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ADDTOGLYPHTABLE: u32 = 2337u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ALIGNBOTTOM: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ALIGNHORIZONTALCENTERS: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ALIGNLEFT: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ALIGNRIGHT: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ALIGNTOGRID: u32 = 5u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ALIGNTOP: u32 = 6u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ALIGNVERTICALCENTERS: u32 = 7u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_APPLYHEADING1: u32 = 2255u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_APPLYHEADING2: u32 = 2256u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_APPLYHEADING3: u32 = 2257u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_APPLYNORMAL: u32 = 2254u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ARRANGEBOTTOM: u32 = 8u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ARRANGERIGHT: u32 = 9u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ATOMICSELECTION: u32 = 2399u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_AUTODETECT: u32 = 2329u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_AUTOURLDETECT_MODE: u32 = 2400u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BACKCOLOR: u32 = 51u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BACKGROUNDIMAGECACHE: u32 = 2430u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BASELINEFONT1: u32 = 2141u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BASELINEFONT2: u32 = 2142u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BASELINEFONT3: u32 = 2143u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BASELINEFONT4: u32 = 2144u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BASELINEFONT5: u32 = 2145u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BEGINUNDOUNIT: u32 = 3901u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BEGINUSERACTION: u32 = 2432u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BLINK: u32 = 2190u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BLOCKDIRLTR: u32 = 2352u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BLOCKDIRRTL: u32 = 2353u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BLOCKFMT: u32 = 2234u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BLUEHIGHLIGHT: u32 = 15216u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BOLD: u32 = 52u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BOOKMARK: u32 = 2123u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BORDERCOLOR: u32 = 53u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BREAKATNEXT: u32 = 2311u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BRINGFORWARD: u32 = 10u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BRINGTOFRONT: u32 = 11u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BROWSEMODE: u32 = 2126u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_BUTTON: u32 = 2167u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CANCEL: u32 = 89u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CAPTIONINSERT: u32 = 2203u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CARETBROWSINGMODE: u32 = 2436u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CELLINSERT: u32 = 2202u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CELLMERGE: u32 = 2204u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CELLPROPERTIES: u32 = 2211u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CELLSELECT: u32 = 2206u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CELLSPLIT: u32 = 2205u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CENTERALIGNPARA: u32 = 2250u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CENTERHORIZONTALLY: u32 = 12u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CENTERVERTICALLY: u32 = 13u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CHANGECASE: u32 = 2246u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CHANGEFONT: u32 = 2240u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CHANGEFONTSIZE: u32 = 2241u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CHECKBOX: u32 = 2163u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CHISELED: u32 = 64u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CLEARAUTHENTICATIONCACHE: u32 = 15003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CLEARSELECTION: u32 = 2007u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CLEARUNDO: u32 = 3903u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CODE: u32 = 14u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_COLUMNINSERT: u32 = 2213u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_COLUMNSELECT: u32 = 2208u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_COMMENT: u32 = 2173u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_COMPOSESETTINGS: u32 = 2318u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CONTEXT: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CONTEXTMENU: u32 = 2280u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CONVERTOBJECT: u32 = 82u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_COPY: u32 = 15u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_COPYBACKGROUND: u32 = 2265u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_COPYCONTENT: u32 = 2291u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_COPYFORMAT: u32 = 2237u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_COPYSHORTCUT: u32 = 2262u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CREATELINK: u32 = 2290u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CREATESHORTCUT: u32 = 2266u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CSSEDITING_LEVEL: u32 = 2406u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CUSTOMCONTROL: u32 = 83u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CUSTOMIZEITEM: u32 = 84u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_CUT: u32 = 16u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_DEBUGGERDYNAMICATTACH: u32 = 15202u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_DEBUGGERDYNAMICATTACHSOURCERUNDOWN: u32 = 15204u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_DEBUGGERDYNAMICDETACH: u32 = 15203u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_DEFAULTBLOCK: u32 = 6046u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_DEFAULTPARAGRAPHSEPARATOR: u32 = 3900u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_DELETE: u32 = 17u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_DELETEPDFHIGHLIGHT: u32 = 15211u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_DELETEWORD: u32 = 92u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_DIRLTR: u32 = 2350u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_DIRRTL: u32 = 2351u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_DISABLE_EDITFOCUS_UI: u32 = 2404u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_DIV: u32 = 2191u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_DOCPROPERTIES: u32 = 2260u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_DROPDOWNBOX: u32 = 2165u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_DYNSRCPLAY: u32 = 2271u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_DYNSRCSTOP: u32 = 2272u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_EDITMODE: u32 = 2127u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_EDITPDFHIGHLIGHT: u32 = 15214u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_EDITSOURCE: u32 = 2122u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_EMPTYGLYPHTABLE: u32 = 2336u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ENABLEFLIPAHEADTARGET: u32 = 15201u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ENABLE_INTERACTION: u32 = 2302u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ENABLE_OBJECT_RESIZING: u32 = 3906u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ENDUNDOUNIT: u32 = 3902u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ENDUSERACTION: u32 = 2433u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ETCHED: u32 = 65u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_EXECPRINT: u32 = 93u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FILE: u32 = 2172u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FIND: u32 = 67u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FIRE_PRINTTEMPLATEDOWN: u32 = 15001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FIRE_PRINTTEMPLATEUP: u32 = 15000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FLAT: u32 = 54u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FOLLOWLINKC: u32 = 2136u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FOLLOWLINKEDGE: u32 = 3911u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FOLLOWLINKN: u32 = 2137u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FOLLOWLINKN_INPRIVATE: u32 = 3909u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FOLLOWLINKT: u32 = 2435u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FOLLOWLINKT_INPRIVATE: u32 = 3910u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FOLLOW_ANCHOR: u32 = 2008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FONT: u32 = 90u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FONTNAME: u32 = 18u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FONTSIZE: u32 = 19u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FORECOLOR: u32 = 55u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FORM: u32 = 2181u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FORMATMARK: u32 = 2132u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_FORWARDDELETE: u32 = 98u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETBLOCKFMTS: u32 = 2233u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETBYTESDOWNLOADED: u32 = 2331u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETDEBUGGERSTATE: u32 = 15205u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETDEFAULTBACKGROUNDCOLOR: u32 = 15044u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETDEFAULTZOOMLEVEL: u32 = 15027u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETDOCDLGFLAGS: u32 = 15005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETELEMENTBOUNDINGBOX: u32 = 15028u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETFRAMEZONE: u32 = 6037u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETIPRINT: u32 = 2403u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETL9QUIRKSEMULATIONENABLED: u32 = 15025u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETPARTIALLAYOUTSTATUS: u32 = 15022u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETPRINTMANAGERDOCSOURCE: u32 = 15038u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETPRINTMANAGERDOCSOURCEASYNC: u32 = 15047u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETPRINTTEMPLATE: u32 = 2295u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETPROFILINGONSTART: u32 = 15011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETSCRIPTENGINE: u32 = 3803u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETSESSIONDOCUMENTMODE: u32 = 15009u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETUSERACTIONTIME: u32 = 2431u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETUSERINITFLAGS: u32 = 15004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETZOOM: u32 = 68u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETZOOMDENOMINATOR: u32 = 2346u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GETZOOMNUMERATOR: u32 = 2345u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GOBACKWARD: u32 = 2282u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GOFORWARD: u32 = 2283u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GOTO: u32 = 2239u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GOTOCLIPBOARDADDRESS: u32 = 2285u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GOTOCLIPBOARDTEXT: u32 = 2286u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GREENHIGHLIGHT: u32 = 15217u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_GROUP: u32 = 20u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_HELP_ABOUT: u32 = 2221u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_HELP_CONTENT: u32 = 2220u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_HELP_README: u32 = 2222u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_HORIZONTALLINE: u32 = 2150u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_HORIZSPACECONCATENATE: u32 = 21u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_HORIZSPACEDECREASE: u32 = 22u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_HORIZSPACEINCREASE: u32 = 23u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_HORIZSPACEMAKEEQUAL: u32 = 24u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_HTMLAREA: u32 = 2178u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_HTMLCONTAIN: u32 = 2159u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_HTMLEDITMODE: u32 = 2316u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_HTMLSOURCE: u32 = 2157u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_HWND: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_HYPERLINK: u32 = 2124u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_IE50_PASTE: u32 = 2401u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_IE50_PASTE_MODE: u32 = 2402u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_IFRAME: u32 = 2158u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_IMAGE: u32 = 2168u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_IMAGEMAP: u32 = 2171u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_IME_ENABLE_RECONVERSION: u32 = 2409u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_IMGARTPLAY: u32 = 2274u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_IMGARTREWIND: u32 = 2276u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_IMGARTSTOP: u32 = 2275u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_IMPORT: u32 = 86u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INDENT: u32 = 2186u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INLINEDIRLTR: u32 = 2354u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INLINEDIRRTL: u32 = 2355u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INSERTHTML: u32 = 2502u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INSERTOBJECT: u32 = 25u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INSERTSPAN: u32 = 2357u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INSERTTEXT: u32 = 3907u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INSFIELDSET: u32 = 2119u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INSINPUTBUTTON: u32 = 2115u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INSINPUTHIDDEN: u32 = 2312u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INSINPUTIMAGE: u32 = 2114u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INSINPUTPASSWORD: u32 = 2313u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INSINPUTRESET: u32 = 2116u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INSINPUTSUBMIT: u32 = 2117u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INSINPUTUPLOAD: u32 = 2118u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INSPECTELEMENT: u32 = 3904u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_INVOKEFLIPAHEADTARGET: u32 = 15200u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ISTRUSTEDDLG: u32 = 2356u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ITALIC: u32 = 56u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_JAVAAPPLET: u32 = 2175u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_JUSTIFYCENTER: u32 = 57u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_JUSTIFYFULL: u32 = 50u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_JUSTIFYGENERAL: u32 = 58u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_JUSTIFYLEFT: u32 = 59u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_JUSTIFYNONE: u32 = 94u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_JUSTIFYRIGHT: u32 = 60u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_KEEPSELECTION: u32 = 2410u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_LANGUAGE: u32 = 2292u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_LAUNCHDEBUGGER: u32 = 2310u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_LAUNCHURICALLBACK: u32 = 3908u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_LEFTALIGNPARA: u32 = 2251u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_LINEBREAKBOTH: u32 = 2154u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_LINEBREAKLEFT: u32 = 2152u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_LINEBREAKNORMAL: u32 = 2151u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_LINEBREAKRIGHT: u32 = 2153u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_LIST: u32 = 2183u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_LISTBOX: u32 = 2166u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_LIVERESIZE: u32 = 2398u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_LOCALIZEEDITOR: u32 = 2358u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MARQUEE: u32 = 2182u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_FRAMESTEP_BACK: u32 = 2461u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_FRAMESTEP_FWD: u32 = 2460u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_FULLSCREEN_EXIT: u32 = 2447u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_FULLSCREEN_TOGGLE: u32 = 2446u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_MUTE: u32 = 2462u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_MUTEUNMUTE: u32 = 2442u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_PAUSE: u32 = 2444u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_PLAY: u32 = 2443u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_PLAYPAUSE: u32 = 2441u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_PLAYRATE0: u32 = 2480u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_PLAYRATE1: u32 = 2481u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_PLAYRATE2: u32 = 2482u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_PLAYRATE3: u32 = 2483u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_PLAYRATE4: u32 = 2484u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_PLAYRATE5: u32 = 2485u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_PLAYRATE6: u32 = 2486u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_PLAYRATE7: u32 = 2487u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_PLAYRATE8: u32 = 2488u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_PLAYRATE9: u32 = 2489u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_RATE_FASTER: u32 = 2456u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_RATE_SLOWER: u32 = 2457u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_SEEK_BACK_LARGE: u32 = 2455u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_SEEK_BACK_SMALL: u32 = 2453u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_SEEK_FWD_LARGE: u32 = 2454u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_SEEK_FWD_SMALL: u32 = 2452u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_SEEK_TO_END: u32 = 2451u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_SEEK_TO_START: u32 = 2450u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_SHOWCONTROLS_TOGGLE: u32 = 2458u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_SHOW_AUDIO_ACCESS: u32 = 2464u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_SHOW_SUBTITLE_ACCESS: u32 = 2465u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_STOP: u32 = 2445u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_UNMUTE: u32 = 2463u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_VOLUME_DOWN: u32 = 2449u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_VOLUME_UP: u32 = 2448u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MEDIA_ZOOMMODE_TOGGLE: u32 = 2459u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MENUEXT_COUNT: u32 = 3733u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MENUEXT_FIRST__: u32 = 3700u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MENUEXT_LAST__: u32 = 3732u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MENUEXT_PLACEHOLDER: u32 = 6047u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MIMECSET__FIRST__: u32 = 3609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MIMECSET__LAST__: u32 = 3699u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MOVE: u32 = 88u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MP_EMAILPICTURE: u32 = 2288u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MP_MYPICS: u32 = 2287u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MP_PRINTPICTURE: u32 = 2289u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MULTILEVELREDO: u32 = 30u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MULTILEVELUNDO: u32 = 44u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_MULTIPLESELECTION: u32 = 2393u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_NEW: u32 = 2001u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_NEWPAGE: u32 = 87u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_NEW_TOPLEVELWINDOW: u32 = 7050u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_NOACTIVATEDESIGNTIMECONTROLS: u32 = 2333u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_NOACTIVATEJAVAAPPLETS: u32 = 2334u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_NOACTIVATENORMALOLECONTROLS: u32 = 2332u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_NOFIXUPURLSONPASTE: u32 = 2335u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_NONBREAK: u32 = 2155u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_NONEHIGHLIGHT: u32 = 15219u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_NOTIFYCONTEXTMENUDISMISSED: u32 = 15046u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_NOTIFYZOOMANDSCROLLANIMATIONEND: u32 = 15045u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OBJECT: u32 = 2169u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OBJECTVERBLIST0: u32 = 72u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OBJECTVERBLIST1: u32 = 73u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OBJECTVERBLIST2: u32 = 74u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OBJECTVERBLIST3: u32 = 75u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OBJECTVERBLIST4: u32 = 76u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OBJECTVERBLIST5: u32 = 77u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OBJECTVERBLIST6: u32 = 78u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OBJECTVERBLIST7: u32 = 79u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OBJECTVERBLIST8: u32 = 80u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OBJECTVERBLIST9: u32 = 81u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OBJECTVERBLISTLAST: u32 = 81u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OLEWINDOWSTATECHANGED: u32 = 15006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OPEN: u32 = 2000u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OPENPDFNOTE: u32 = 15213u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OPTIONS: u32 = 2135u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ORDERLIST: u32 = 2184u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OUTDENT: u32 = 2187u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OVERRIDE_CURSOR: u32 = 2420u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_OVERWRITE: u32 = 2314u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PAGE: u32 = 2267u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PAGEBREAK: u32 = 2177u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PAGEINFO: u32 = 2231u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PAGESETUP: u32 = 2004u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PARAGRAPH: u32 = 2180u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PARSECOMPLETE: u32 = 2315u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PASTE: u32 = 26u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PASTECONTENTONLY: u32 = 2500u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PASTEFORMAT: u32 = 2238u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PASTEINSERT: u32 = 2120u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PASTESPECIAL: u32 = 2006u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PASTETEXTONLY: u32 = 2501u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PDFDEFINE: u32 = 15222u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PDFREADALOUD: u32 = 15220u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PEERHITTESTSAMEINEDIT: u32 = 2423u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PERFORMEDITACTIVATION: u32 = 15042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PERSISTDEFAULTVALUES: u32 = 7100u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PERSISTSTREAMSYNC: u32 = 2341u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PINKHIGHLIGHT: u32 = 15215u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PLUGIN: u32 = 2176u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_POPSTATEEVENT: u32 = 15017u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PREFORMATTED: u32 = 2188u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PRESERVEUNDOALWAYS: u32 = 6049u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PRESTOP: u32 = 2284u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PRINT: u32 = 27u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PRINTPREVIEW: u32 = 2003u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PRINTQUERYJOBSPENDING: u32 = 2277u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PRINTTARGET: u32 = 2273u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PROPERTIES: u32 = 28u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_PROTECTMETATAGS: u32 = 7101u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_RADIOBUTTON: u32 = 2164u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_RAISED: u32 = 61u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_RCINSERT: u32 = 2201u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_REDO: u32 = 29u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_REFRESH: u32 = 2300u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_REFRESH_THIS: u32 = 6042u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_REGISTRYREFRESH: u32 = 2317u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_REMOVECONSOLEMESSAGERECEIVER: u32 = 3801u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_REMOVEDEBUGCALLBACKRECEIVER: u32 = 3805u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_REMOVEFORMAT: u32 = 2230u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_REMOVEFROMGLYPHTABLE: u32 = 2338u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_REMOVEPARAFORMAT: u32 = 2253u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_RENAME: u32 = 85u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_REPLACE: u32 = 2121u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_REPLACEGLYPHCONTENTS: u32 = 2339u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_RESPECTVISIBILITY_INDESIGN: u32 = 2405u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_RIGHTALIGNPARA: u32 = 2252u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ROWINSERT: u32 = 2212u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ROWSELECT: u32 = 2207u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_RUNFLASH: u32 = 15208u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_RUNURLSCRIPT: u32 = 2343u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SAVE: u32 = 70u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SAVEAS: u32 = 71u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SAVEBACKGROUND: u32 = 2263u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SAVECOPYAS: u32 = 2002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SAVEPDF: u32 = 99u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SAVEPICTURE: u32 = 2270u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SAVEPRETRANSFORMSOURCE: u32 = 2370u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SAVETARGET: u32 = 2268u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SCRIPT: u32 = 2174u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SCRIPTDEBUGGER: u32 = 2330u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SCROLL_BOTTOM: u32 = 2382u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SCROLL_DOWN: u32 = 2386u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SCROLL_HERE: u32 = 2380u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SCROLL_LEFT: u32 = 2391u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SCROLL_LEFTEDGE: u32 = 2387u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SCROLL_PAGEDOWN: u32 = 2384u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SCROLL_PAGELEFT: u32 = 2389u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SCROLL_PAGERIGHT: u32 = 2390u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SCROLL_PAGEUP: u32 = 2383u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SCROLL_RIGHT: u32 = 2392u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SCROLL_RIGHTEDGE: u32 = 2388u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SCROLL_TOP: u32 = 2381u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SCROLL_UP: u32 = 2385u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SELECTALL: u32 = 31u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SELECTIONSEARCH: u32 = 15206u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SENDBACKWARD: u32 = 32u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SENDTOBACK: u32 = 33u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETACCESSIBILITYNAME: u32 = 15040u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETCUSTOMCURSOR: u32 = 2434u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETDEFAULTBACKGROUNDCOLOR: u32 = 15043u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETDESKTOPITEM: u32 = 2278u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETDEVTOOLBARCONSOLE: u32 = 15016u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETDIRTY: u32 = 2342u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETEXTRAHEADERS: u32 = 15039u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETGEOLOCATIONCONSENT: u32 = 15029u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETL9QUIRKSEMULATIONENABLED: u32 = 15024u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETNAVIGATEEVENTSINK: u32 = 15013u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETPAGEACTIONALLOWEDFLAGS: u32 = 15100u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETPARTIALLAYOUTSTATUS: u32 = 15021u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETPOINTERLOCKCONSENT: u32 = 15026u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETPRINTHANDLES: u32 = 15002u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETPRINTTEMPLATE: u32 = 2296u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETPROFILINGONSTART: u32 = 15010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETSCRIPTCONSOLE: u32 = 15012u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETSESSIONDOCUMENTMODE: u32 = 15008u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SETWALLPAPER: u32 = 2264u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHADOWED: u32 = 66u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHARE: u32 = 15031u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHAREAPPCACHEEVENT: u32 = 15033u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHAREPDF: u32 = 15221u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHAREPICTURE: u32 = 3905u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWALIGNEDSITETAGS: u32 = 2321u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWALLTAGS: u32 = 2327u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWAREATAGS: u32 = 2325u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWCOMMENTTAGS: u32 = 2324u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWGRID: u32 = 69u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWHIDE_CODE: u32 = 2235u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWMISCTAGS: u32 = 2320u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWPAGESETUP: u32 = 2011u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWPICTURE: u32 = 2269u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWPRINT: u32 = 2010u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWSCRIPTTAGS: u32 = 2322u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWSHAREUI: u32 = 15207u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWSPECIALCHAR: u32 = 2249u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWSTYLETAGS: u32 = 2323u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWTABLE: u32 = 34u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWUNKNOWNTAGS: u32 = 2326u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWWBRTAGS: u32 = 2340u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SHOWZEROBORDERATDESIGNTIME: u32 = 2328u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SIZETOCONTROL: u32 = 35u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SIZETOCONTROLHEIGHT: u32 = 36u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SIZETOCONTROLWIDTH: u32 = 37u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SIZETOFIT: u32 = 38u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SIZETOGRID: u32 = 39u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SNAPTOGRID: u32 = 40u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SPECIALCHAR: u32 = 2156u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SPELL: u32 = 2005u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_STARTDIAGNOSTICSMODE: u32 = 3802u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_STATUSBAR: u32 = 2131u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_STOP: u32 = 2138u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_STOPDOWNLOAD: u32 = 2301u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_STRIKETHROUGH: u32 = 91u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_STYLEMENU_CHANGESELECTEDSTYLE: u32 = 2440u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_STYLEMENU_GETNOSTYLE: u32 = 2438u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_STYLEMENU_GETPREFSTYLE: u32 = 2439u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_STYLEMENU_SETNOSTYLE: u32 = 2437u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SUBSCRIPT: u32 = 2247u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SUNKEN: u32 = 62u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_SUPERSCRIPT: u32 = 2248u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TABLE: u32 = 2236u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TABLEINSERT: u32 = 2200u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TABLEPROPERTIES: u32 = 2210u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TABLESELECT: u32 = 2209u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TABORDER: u32 = 41u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TELETYPE: u32 = 2232u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TEMPLATE_PAGESETUP: u32 = 2298u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TEXTAREA: u32 = 2162u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TEXTBOX: u32 = 2161u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TEXTONLY: u32 = 2133u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TOGGLEREADINGBAR: u32 = 15209u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TOOLBARS: u32 = 2130u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TOOLBOX: u32 = 42u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TRISTATEBOLD: u32 = 95u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TRISTATEITALIC: u32 = 96u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TRISTATEUNDERLINE: u32 = 97u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_TRUSTAPPCACHE: u32 = 2425u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_UI_OUTDENT: u32 = 2407u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_UNBOOKMARK: u32 = 2128u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_UNDERLINE: u32 = 63u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_UNDO: u32 = 43u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_UNGROUP: u32 = 45u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_UNLINK: u32 = 2125u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_UNLOADDOCUMENT: u32 = 2411u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_UNORDERLIST: u32 = 2185u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_UPDATEPAGESTATUS: u32 = 2408u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_UPDATESETTINGSFROMREGISTRY: u32 = 15041u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_VERTSPACECONCATENATE: u32 = 46u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_VERTSPACEDECREASE: u32 = 47u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_VERTSPACEINCREASE: u32 = 48u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_VERTSPACEMAKEEQUAL: u32 = 49u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_VIEWPRETRANSFORMSOURCE: u32 = 2371u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_VIEWSOURCE: u32 = 2139u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_YELLOWHIGHLIGHT: u32 = 15218u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ZOOMPERCENT: u32 = 50u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ZOOMPOPUP: u32 = 2140u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IDM_ZOOMRATIO: u32 = 2344u32;
pub struct IDOMBeforeUnloadEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMCloseEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMCompositionEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMCustomEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMDocumentType(pub *mut ::core::ffi::c_void);
pub struct IDOMDragEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMEventRegistrationCallback(pub *mut ::core::ffi::c_void);
pub struct IDOMException(pub *mut ::core::ffi::c_void);
pub struct IDOMFocusEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMKeyboardEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMMSAnimationEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMMSManipulationEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMMSTransitionEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMMessageEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMMouseEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMMouseWheelEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMMutationEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMNodeIterator(pub *mut ::core::ffi::c_void);
pub struct IDOMParser(pub *mut ::core::ffi::c_void);
pub struct IDOMParserFactory(pub *mut ::core::ffi::c_void);
pub struct IDOMProcessingInstruction(pub *mut ::core::ffi::c_void);
pub struct IDOMProgressEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMSiteModeEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMStorageEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMTextEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMTreeWalker(pub *mut ::core::ffi::c_void);
pub struct IDOMUIEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMWheelEvent(pub *mut ::core::ffi::c_void);
pub struct IDOMXmlSerializer(pub *mut ::core::ffi::c_void);
pub struct IDOMXmlSerializerFactory(pub *mut ::core::ffi::c_void);
pub struct IDebugCallbackNotificationHandler(pub *mut ::core::ffi::c_void);
pub struct IDeveloperConsoleMessageReceiver(pub *mut ::core::ffi::c_void);
pub struct IDeviceRect(pub *mut ::core::ffi::c_void);
pub struct IDiagnosticsScriptEngine(pub *mut ::core::ffi::c_void);
pub struct IDiagnosticsScriptEngineProvider(pub *mut ::core::ffi::c_void);
pub struct IDiagnosticsScriptEngineSite(pub *mut ::core::ffi::c_void);
pub struct IDisplayPointer(pub *mut ::core::ffi::c_void);
pub struct IDisplayServices(pub *mut ::core::ffi::c_void);
pub struct IDithererImpl(pub *mut ::core::ffi::c_void);
pub struct IDocHostShowUI(pub *mut ::core::ffi::c_void);
pub struct IDocHostUIHandler(pub *mut ::core::ffi::c_void);
pub struct IDocHostUIHandler2(pub *mut ::core::ffi::c_void);
pub struct IDocObjectService(pub *mut ::core::ffi::c_void);
pub struct IDocumentEvent(pub *mut ::core::ffi::c_void);
pub struct IDocumentRange(pub *mut ::core::ffi::c_void);
pub struct IDocumentSelector(pub *mut ::core::ffi::c_void);
pub struct IDocumentTraversal(pub *mut ::core::ffi::c_void);
pub struct IDownloadBehavior(pub *mut ::core::ffi::c_void);
pub struct IDownloadManager(pub *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IECMDID_ARG_CLEAR_FORMS_ALL: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IECMDID_ARG_CLEAR_FORMS_ALL_BUT_PASSWORDS: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IECMDID_ARG_CLEAR_FORMS_PASSWORDS_ONLY: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IECMDID_BEFORENAVIGATE_DOEXTERNALBROWSE: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IECMDID_BEFORENAVIGATE_GETIDLIST: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IECMDID_BEFORENAVIGATE_GETSHELLBROWSE: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IECMDID_CLEAR_AUTOCOMPLETE_FOR_FORMS: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IECMDID_GET_INVOKE_DEFAULT_BROWSER_ON_NEW_WINDOW: u32 = 6u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IECMDID_SETID_AUTOCOMPLETE_FOR_FORMS: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IECMDID_SET_INVOKE_DEFAULT_BROWSER_ON_NEW_WINDOW: u32 = 5u32;
pub struct IEISXMLNSREGISTEREDFN(i32);
pub struct IELAUNCHOPTION_FLAGS(i32);
pub struct IELAUNCHURLINFO(i32);
pub struct IEREGISTERXMLNSFN(i32);
pub struct IEWebDriverManager(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IE_USE_OE_MAIL_HKEY: i32 = -2147483647i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IE_USE_OE_NEWS_HKEY: i32 = -2147483647i32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IE_USE_OE_PRESENT_HKEY: i32 = -2147483646i32;
pub struct IElementBehavior(pub *mut ::core::ffi::c_void);
pub struct IElementBehaviorCategory(pub *mut ::core::ffi::c_void);
pub struct IElementBehaviorFactory(pub *mut ::core::ffi::c_void);
pub struct IElementBehaviorFocus(pub *mut ::core::ffi::c_void);
pub struct IElementBehaviorLayout(pub *mut ::core::ffi::c_void);
pub struct IElementBehaviorLayout2(pub *mut ::core::ffi::c_void);
pub struct IElementBehaviorRender(pub *mut ::core::ffi::c_void);
pub struct IElementBehaviorSite(pub *mut ::core::ffi::c_void);
pub struct IElementBehaviorSiteCategory(pub *mut ::core::ffi::c_void);
pub struct IElementBehaviorSiteLayout(pub *mut ::core::ffi::c_void);
pub struct IElementBehaviorSiteLayout2(pub *mut ::core::ffi::c_void);
pub struct IElementBehaviorSiteOM(pub *mut ::core::ffi::c_void);
pub struct IElementBehaviorSiteOM2(pub *mut ::core::ffi::c_void);
pub struct IElementBehaviorSiteRender(pub *mut ::core::ffi::c_void);
pub struct IElementBehaviorSubmit(pub *mut ::core::ffi::c_void);
pub struct IElementNamespace(pub *mut ::core::ffi::c_void);
pub struct IElementNamespaceFactory(pub *mut ::core::ffi::c_void);
pub struct IElementNamespaceFactory2(pub *mut ::core::ffi::c_void);
pub struct IElementNamespaceFactoryCallback(pub *mut ::core::ffi::c_void);
pub struct IElementNamespaceTable(pub *mut ::core::ffi::c_void);
pub struct IElementSegment(pub *mut ::core::ffi::c_void);
pub struct IElementSelector(pub *mut ::core::ffi::c_void);
pub struct IElementTraversal(pub *mut ::core::ffi::c_void);
pub struct IEnumManagerFrames(pub *mut ::core::ffi::c_void);
pub struct IEnumOpenServiceActivity(pub *mut ::core::ffi::c_void);
pub struct IEnumOpenServiceActivityCategory(pub *mut ::core::ffi::c_void);
pub struct IEnumPrivacyRecords(pub *mut ::core::ffi::c_void);
pub struct IEnumSTATURL(pub *mut ::core::ffi::c_void);
pub struct IEventException(pub *mut ::core::ffi::c_void);
pub struct IEventTarget(pub *mut ::core::ffi::c_void);
pub struct IEventTarget2(pub *mut ::core::ffi::c_void);
pub struct IExtensionValidation(pub *mut ::core::ffi::c_void);
pub struct IFontNames(pub *mut ::core::ffi::c_void);
pub struct IGetSVGDocument(pub *mut ::core::ffi::c_void);
pub struct IHTCAttachBehavior(pub *mut ::core::ffi::c_void);
pub struct IHTCAttachBehavior2(pub *mut ::core::ffi::c_void);
pub struct IHTCDefaultDispatch(pub *mut ::core::ffi::c_void);
pub struct IHTCDescBehavior(pub *mut ::core::ffi::c_void);
pub struct IHTCEventBehavior(pub *mut ::core::ffi::c_void);
pub struct IHTCMethodBehavior(pub *mut ::core::ffi::c_void);
pub struct IHTCPropertyBehavior(pub *mut ::core::ffi::c_void);
pub struct IHTMLAnchorElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLAnchorElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLAnchorElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLAppBehavior(pub *mut ::core::ffi::c_void);
pub struct IHTMLAppBehavior2(pub *mut ::core::ffi::c_void);
pub struct IHTMLAppBehavior3(pub *mut ::core::ffi::c_void);
pub struct IHTMLApplicationCache(pub *mut ::core::ffi::c_void);
pub struct IHTMLAreaElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLAreaElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLAreasCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLAreasCollection2(pub *mut ::core::ffi::c_void);
pub struct IHTMLAreasCollection3(pub *mut ::core::ffi::c_void);
pub struct IHTMLAreasCollection4(pub *mut ::core::ffi::c_void);
pub struct IHTMLAttributeCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLAttributeCollection2(pub *mut ::core::ffi::c_void);
pub struct IHTMLAttributeCollection3(pub *mut ::core::ffi::c_void);
pub struct IHTMLAttributeCollection4(pub *mut ::core::ffi::c_void);
pub struct IHTMLAudioElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLAudioElementFactory(pub *mut ::core::ffi::c_void);
pub struct IHTMLBGsound(pub *mut ::core::ffi::c_void);
pub struct IHTMLBRElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLBaseElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLBaseElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLBaseFontElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLBlockElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLBlockElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLBlockElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLBodyElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLBodyElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLBodyElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLBodyElement4(pub *mut ::core::ffi::c_void);
pub struct IHTMLBodyElement5(pub *mut ::core::ffi::c_void);
pub struct IHTMLBookmarkCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLButtonElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLButtonElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLCSSImportRule(pub *mut ::core::ffi::c_void);
pub struct IHTMLCSSMediaList(pub *mut ::core::ffi::c_void);
pub struct IHTMLCSSMediaRule(pub *mut ::core::ffi::c_void);
pub struct IHTMLCSSNamespaceRule(pub *mut ::core::ffi::c_void);
pub struct IHTMLCSSRule(pub *mut ::core::ffi::c_void);
pub struct IHTMLCSSStyleDeclaration(pub *mut ::core::ffi::c_void);
pub struct IHTMLCSSStyleDeclaration2(pub *mut ::core::ffi::c_void);
pub struct IHTMLCSSStyleDeclaration3(pub *mut ::core::ffi::c_void);
pub struct IHTMLCSSStyleDeclaration4(pub *mut ::core::ffi::c_void);
pub struct IHTMLCanvasElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLCaret(pub *mut ::core::ffi::c_void);
pub struct IHTMLChangeLog(pub *mut ::core::ffi::c_void);
pub struct IHTMLChangePlayback(pub *mut ::core::ffi::c_void);
pub struct IHTMLChangeSink(pub *mut ::core::ffi::c_void);
pub struct IHTMLCommentElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLCommentElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLCommentElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLComputedStyle(pub *mut ::core::ffi::c_void);
pub struct IHTMLControlElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLControlRange(pub *mut ::core::ffi::c_void);
pub struct IHTMLControlRange2(pub *mut ::core::ffi::c_void);
pub struct IHTMLCurrentStyle(pub *mut ::core::ffi::c_void);
pub struct IHTMLCurrentStyle2(pub *mut ::core::ffi::c_void);
pub struct IHTMLCurrentStyle3(pub *mut ::core::ffi::c_void);
pub struct IHTMLCurrentStyle4(pub *mut ::core::ffi::c_void);
pub struct IHTMLCurrentStyle5(pub *mut ::core::ffi::c_void);
pub struct IHTMLDDElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLDListElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMAttribute(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMAttribute2(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMAttribute3(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMAttribute4(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMChildrenCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMChildrenCollection2(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMConstructor(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMConstructorCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMImplementation(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMImplementation2(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMNode(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMNode2(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMNode3(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMRange(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMTextNode(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMTextNode2(pub *mut ::core::ffi::c_void);
pub struct IHTMLDOMTextNode3(pub *mut ::core::ffi::c_void);
pub struct IHTMLDTElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLDataTransfer(pub *mut ::core::ffi::c_void);
pub struct IHTMLDatabinding(pub *mut ::core::ffi::c_void);
pub struct IHTMLDialog(pub *mut ::core::ffi::c_void);
pub struct IHTMLDialog2(pub *mut ::core::ffi::c_void);
pub struct IHTMLDialog3(pub *mut ::core::ffi::c_void);
pub struct IHTMLDivElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLDivPosition(pub *mut ::core::ffi::c_void);
pub struct IHTMLDocument(pub *mut ::core::ffi::c_void);
pub struct IHTMLDocument2(pub *mut ::core::ffi::c_void);
pub struct IHTMLDocument3(pub *mut ::core::ffi::c_void);
pub struct IHTMLDocument4(pub *mut ::core::ffi::c_void);
pub struct IHTMLDocument5(pub *mut ::core::ffi::c_void);
pub struct IHTMLDocument6(pub *mut ::core::ffi::c_void);
pub struct IHTMLDocument7(pub *mut ::core::ffi::c_void);
pub struct IHTMLDocument8(pub *mut ::core::ffi::c_void);
pub struct IHTMLDocumentCompatibleInfo(pub *mut ::core::ffi::c_void);
pub struct IHTMLDocumentCompatibleInfoCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLEditDesigner(pub *mut ::core::ffi::c_void);
pub struct IHTMLEditHost(pub *mut ::core::ffi::c_void);
pub struct IHTMLEditHost2(pub *mut ::core::ffi::c_void);
pub struct IHTMLEditServices(pub *mut ::core::ffi::c_void);
pub struct IHTMLEditServices2(pub *mut ::core::ffi::c_void);
pub struct IHTMLElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLElement4(pub *mut ::core::ffi::c_void);
pub struct IHTMLElement5(pub *mut ::core::ffi::c_void);
pub struct IHTMLElement6(pub *mut ::core::ffi::c_void);
pub struct IHTMLElement7(pub *mut ::core::ffi::c_void);
pub struct IHTMLElementAppliedStyles(pub *mut ::core::ffi::c_void);
pub struct IHTMLElementCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLElementCollection2(pub *mut ::core::ffi::c_void);
pub struct IHTMLElementCollection3(pub *mut ::core::ffi::c_void);
pub struct IHTMLElementCollection4(pub *mut ::core::ffi::c_void);
pub struct IHTMLElementDefaults(pub *mut ::core::ffi::c_void);
pub struct IHTMLElementRender(pub *mut ::core::ffi::c_void);
pub struct IHTMLEmbedElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLEmbedElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLEventObj(pub *mut ::core::ffi::c_void);
pub struct IHTMLEventObj2(pub *mut ::core::ffi::c_void);
pub struct IHTMLEventObj3(pub *mut ::core::ffi::c_void);
pub struct IHTMLEventObj4(pub *mut ::core::ffi::c_void);
pub struct IHTMLEventObj5(pub *mut ::core::ffi::c_void);
pub struct IHTMLEventObj6(pub *mut ::core::ffi::c_void);
pub struct IHTMLFieldSetElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLFieldSetElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLFiltersCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLFontElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLFontNamesCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLFontSizesCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLFormElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLFormElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLFormElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLFormElement4(pub *mut ::core::ffi::c_void);
pub struct IHTMLFrameBase(pub *mut ::core::ffi::c_void);
pub struct IHTMLFrameBase2(pub *mut ::core::ffi::c_void);
pub struct IHTMLFrameBase3(pub *mut ::core::ffi::c_void);
pub struct IHTMLFrameElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLFrameElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLFrameElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLFrameSetElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLFrameSetElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLFrameSetElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLFramesCollection2(pub *mut ::core::ffi::c_void);
pub struct IHTMLGenericElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLHRElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLHeadElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLHeadElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLHeaderElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLHtmlElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLIFrameElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLIFrameElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLIFrameElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLIPrintCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLImageElementFactory(pub *mut ::core::ffi::c_void);
pub struct IHTMLImgElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLImgElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLImgElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLImgElement4(pub *mut ::core::ffi::c_void);
pub struct IHTMLInputButtonElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLInputElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLInputElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLInputElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLInputFileElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLInputHiddenElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLInputImage(pub *mut ::core::ffi::c_void);
pub struct IHTMLInputRangeElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLInputTextElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLInputTextElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLIsIndexElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLIsIndexElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLLIElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLLabelElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLLabelElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLLegendElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLLegendElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLLinkElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLLinkElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLLinkElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLLinkElement4(pub *mut ::core::ffi::c_void);
pub struct IHTMLLinkElement5(pub *mut ::core::ffi::c_void);
pub struct IHTMLListElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLListElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLLocation(pub *mut ::core::ffi::c_void);
pub struct IHTMLMSCSSKeyframeRule(pub *mut ::core::ffi::c_void);
pub struct IHTMLMSCSSKeyframesRule(pub *mut ::core::ffi::c_void);
pub struct IHTMLMSImgElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLMSMediaElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLMapElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLMarqueeElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLMediaElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLMediaElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLMediaError(pub *mut ::core::ffi::c_void);
pub struct IHTMLMetaElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLMetaElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLMetaElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLMimeTypesCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLModelessInit(pub *mut ::core::ffi::c_void);
pub struct IHTMLNamespace(pub *mut ::core::ffi::c_void);
pub struct IHTMLNamespaceCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLNextIdElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLNoShowElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLOListElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLOMWindowServices(pub *mut ::core::ffi::c_void);
pub struct IHTMLObjectElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLObjectElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLObjectElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLObjectElement4(pub *mut ::core::ffi::c_void);
pub struct IHTMLObjectElement5(pub *mut ::core::ffi::c_void);
pub struct IHTMLOpsProfile(pub *mut ::core::ffi::c_void);
pub struct IHTMLOptionButtonElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLOptionElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLOptionElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLOptionElement4(pub *mut ::core::ffi::c_void);
pub struct IHTMLOptionElementFactory(pub *mut ::core::ffi::c_void);
pub struct IHTMLOptionsHolder(pub *mut ::core::ffi::c_void);
pub struct IHTMLPaintSite(pub *mut ::core::ffi::c_void);
pub struct IHTMLPainter(pub *mut ::core::ffi::c_void);
pub struct IHTMLPainterEventInfo(pub *mut ::core::ffi::c_void);
pub struct IHTMLPainterOverlay(pub *mut ::core::ffi::c_void);
pub struct IHTMLParaElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLParamElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLParamElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLPerformance(pub *mut ::core::ffi::c_void);
pub struct IHTMLPerformanceNavigation(pub *mut ::core::ffi::c_void);
pub struct IHTMLPerformanceTiming(pub *mut ::core::ffi::c_void);
pub struct IHTMLPersistData(pub *mut ::core::ffi::c_void);
pub struct IHTMLPersistDataOM(pub *mut ::core::ffi::c_void);
pub struct IHTMLPhraseElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLPhraseElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLPhraseElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLPluginsCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLPopup(pub *mut ::core::ffi::c_void);
pub struct IHTMLProgressElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLRect(pub *mut ::core::ffi::c_void);
pub struct IHTMLRect2(pub *mut ::core::ffi::c_void);
pub struct IHTMLRectCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLRenderStyle(pub *mut ::core::ffi::c_void);
pub struct IHTMLRuleStyle(pub *mut ::core::ffi::c_void);
pub struct IHTMLRuleStyle2(pub *mut ::core::ffi::c_void);
pub struct IHTMLRuleStyle3(pub *mut ::core::ffi::c_void);
pub struct IHTMLRuleStyle4(pub *mut ::core::ffi::c_void);
pub struct IHTMLRuleStyle5(pub *mut ::core::ffi::c_void);
pub struct IHTMLRuleStyle6(pub *mut ::core::ffi::c_void);
pub struct IHTMLScreen(pub *mut ::core::ffi::c_void);
pub struct IHTMLScreen2(pub *mut ::core::ffi::c_void);
pub struct IHTMLScreen3(pub *mut ::core::ffi::c_void);
pub struct IHTMLScreen4(pub *mut ::core::ffi::c_void);
pub struct IHTMLScriptElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLScriptElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLScriptElement3(pub *mut ::core::ffi::c_void);
pub struct IHTMLScriptElement4(pub *mut ::core::ffi::c_void);
pub struct IHTMLSelectElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLSelectElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLSelectElement4(pub *mut ::core::ffi::c_void);
pub struct IHTMLSelectElement5(pub *mut ::core::ffi::c_void);
pub struct IHTMLSelectElement6(pub *mut ::core::ffi::c_void);
pub struct IHTMLSelectElementEx(pub *mut ::core::ffi::c_void);
pub struct IHTMLSelection(pub *mut ::core::ffi::c_void);
pub struct IHTMLSelectionObject(pub *mut ::core::ffi::c_void);
pub struct IHTMLSelectionObject2(pub *mut ::core::ffi::c_void);
pub struct IHTMLSourceElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLSpanElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLSpanFlow(pub *mut ::core::ffi::c_void);
pub struct IHTMLStorage(pub *mut ::core::ffi::c_void);
pub struct IHTMLStorage2(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyle(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyle2(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyle3(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyle4(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyle5(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyle6(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleEnabled(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleFontFace(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleFontFace2(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleMedia(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleSheet(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleSheet2(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleSheet3(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleSheet4(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleSheetPage(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleSheetPage2(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleSheetPagesCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleSheetRule(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleSheetRule2(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleSheetRuleApplied(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleSheetRulesAppliedCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleSheetRulesCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleSheetRulesCollection2(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleSheetsCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLStyleSheetsCollection2(pub *mut ::core::ffi::c_void);
pub struct IHTMLSubmitData(pub *mut ::core::ffi::c_void);
pub struct IHTMLTable(pub *mut ::core::ffi::c_void);
pub struct IHTMLTable2(pub *mut ::core::ffi::c_void);
pub struct IHTMLTable3(pub *mut ::core::ffi::c_void);
pub struct IHTMLTable4(pub *mut ::core::ffi::c_void);
pub struct IHTMLTableCaption(pub *mut ::core::ffi::c_void);
pub struct IHTMLTableCell(pub *mut ::core::ffi::c_void);
pub struct IHTMLTableCell2(pub *mut ::core::ffi::c_void);
pub struct IHTMLTableCell3(pub *mut ::core::ffi::c_void);
pub struct IHTMLTableCol(pub *mut ::core::ffi::c_void);
pub struct IHTMLTableCol2(pub *mut ::core::ffi::c_void);
pub struct IHTMLTableCol3(pub *mut ::core::ffi::c_void);
pub struct IHTMLTableRow(pub *mut ::core::ffi::c_void);
pub struct IHTMLTableRow2(pub *mut ::core::ffi::c_void);
pub struct IHTMLTableRow3(pub *mut ::core::ffi::c_void);
pub struct IHTMLTableRow4(pub *mut ::core::ffi::c_void);
pub struct IHTMLTableRowMetrics(pub *mut ::core::ffi::c_void);
pub struct IHTMLTableSection(pub *mut ::core::ffi::c_void);
pub struct IHTMLTableSection2(pub *mut ::core::ffi::c_void);
pub struct IHTMLTableSection3(pub *mut ::core::ffi::c_void);
pub struct IHTMLTableSection4(pub *mut ::core::ffi::c_void);
pub struct IHTMLTextAreaElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLTextAreaElement2(pub *mut ::core::ffi::c_void);
pub struct IHTMLTextContainer(pub *mut ::core::ffi::c_void);
pub struct IHTMLTextElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLTextRangeMetrics(pub *mut ::core::ffi::c_void);
pub struct IHTMLTextRangeMetrics2(pub *mut ::core::ffi::c_void);
pub struct IHTMLTimeRanges(pub *mut ::core::ffi::c_void);
pub struct IHTMLTimeRanges2(pub *mut ::core::ffi::c_void);
pub struct IHTMLTitleElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLTxtRange(pub *mut ::core::ffi::c_void);
pub struct IHTMLTxtRangeCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLUListElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLUniqueName(pub *mut ::core::ffi::c_void);
pub struct IHTMLUnknownElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLUrnCollection(pub *mut ::core::ffi::c_void);
pub struct IHTMLUserDataOM(pub *mut ::core::ffi::c_void);
pub struct IHTMLVideoElement(pub *mut ::core::ffi::c_void);
pub struct IHTMLWindow2(pub *mut ::core::ffi::c_void);
pub struct IHTMLWindow3(pub *mut ::core::ffi::c_void);
pub struct IHTMLWindow4(pub *mut ::core::ffi::c_void);
pub struct IHTMLWindow5(pub *mut ::core::ffi::c_void);
pub struct IHTMLWindow6(pub *mut ::core::ffi::c_void);
pub struct IHTMLWindow7(pub *mut ::core::ffi::c_void);
pub struct IHTMLWindow8(pub *mut ::core::ffi::c_void);
pub struct IHTMLXDomainRequest(pub *mut ::core::ffi::c_void);
pub struct IHTMLXDomainRequestFactory(pub *mut ::core::ffi::c_void);
pub struct IHTMLXMLHttpRequest(pub *mut ::core::ffi::c_void);
pub struct IHTMLXMLHttpRequest2(pub *mut ::core::ffi::c_void);
pub struct IHTMLXMLHttpRequestFactory(pub *mut ::core::ffi::c_void);
pub struct IHeaderFooter(pub *mut ::core::ffi::c_void);
pub struct IHeaderFooter2(pub *mut ::core::ffi::c_void);
pub struct IHighlightRenderingServices(pub *mut ::core::ffi::c_void);
pub struct IHighlightSegment(pub *mut ::core::ffi::c_void);
pub struct IHomePage(pub *mut ::core::ffi::c_void);
pub struct IHomePageSetting(pub *mut ::core::ffi::c_void);
pub struct IHostBehaviorInit(pub *mut ::core::ffi::c_void);
pub struct IHostDialogHelper(pub *mut ::core::ffi::c_void);
pub struct IHtmlDlgSafeHelper(pub *mut ::core::ffi::c_void);
pub struct IICCSVGColor(pub *mut ::core::ffi::c_void);
pub struct IIE70DispatchEx(pub *mut ::core::ffi::c_void);
pub struct IIE80DispatchEx(pub *mut ::core::ffi::c_void);
pub struct IIEWebDriverManager(pub *mut ::core::ffi::c_void);
pub struct IIEWebDriverSite(pub *mut ::core::ffi::c_void);
pub struct IIMEServices(pub *mut ::core::ffi::c_void);
pub struct IImageDecodeEventSink(pub *mut ::core::ffi::c_void);
pub struct IImageDecodeEventSink2(pub *mut ::core::ffi::c_void);
pub struct IImageDecodeFilter(pub *mut ::core::ffi::c_void);
pub struct IIntelliForms(pub *mut ::core::ffi::c_void);
pub struct IInternetExplorerManager(pub *mut ::core::ffi::c_void);
pub struct IInternetExplorerManager2(pub *mut ::core::ffi::c_void);
pub struct ILayoutRect(pub *mut ::core::ffi::c_void);
pub struct ILineInfo(pub *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IMGDECODE_EVENT_BEGINBITS: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IMGDECODE_EVENT_BITSCOMPLETE: u32 = 8u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IMGDECODE_EVENT_PALETTE: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IMGDECODE_EVENT_PROGRESS: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IMGDECODE_EVENT_USEDDRAW: u32 = 16u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IMGDECODE_HINT_BOTTOMUP: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IMGDECODE_HINT_FULLWIDTH: u32 = 4u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const IMGDECODE_HINT_TOPDOWN: u32 = 1u32;
pub struct IMapMIMEToCLSID(pub *mut ::core::ffi::c_void);
pub struct IMarkupContainer(pub *mut ::core::ffi::c_void);
pub struct IMarkupContainer2(pub *mut ::core::ffi::c_void);
pub struct IMarkupPointer(pub *mut ::core::ffi::c_void);
pub struct IMarkupPointer2(pub *mut ::core::ffi::c_void);
pub struct IMarkupServices(pub *mut ::core::ffi::c_void);
pub struct IMarkupServices2(pub *mut ::core::ffi::c_void);
pub struct IMarkupTextFrags(pub *mut ::core::ffi::c_void);
pub struct IMediaActivityNotifySite(pub *mut ::core::ffi::c_void);
pub struct INTERNETEXPLORERCONFIGURATION(i32);
pub struct INavigatorDoNotTrack(pub *mut ::core::ffi::c_void);
pub struct INavigatorGeolocation(pub *mut ::core::ffi::c_void);
pub struct IOmHistory(pub *mut ::core::ffi::c_void);
pub struct IOmNavigator(pub *mut ::core::ffi::c_void);
pub struct IOpenService(pub *mut ::core::ffi::c_void);
pub struct IOpenServiceActivity(pub *mut ::core::ffi::c_void);
pub struct IOpenServiceActivityCategory(pub *mut ::core::ffi::c_void);
pub struct IOpenServiceActivityInput(pub *mut ::core::ffi::c_void);
pub struct IOpenServiceActivityManager(pub *mut ::core::ffi::c_void);
pub struct IOpenServiceActivityOutputContext(pub *mut ::core::ffi::c_void);
pub struct IOpenServiceManager(pub *mut ::core::ffi::c_void);
pub struct IPeerFactory(pub *mut ::core::ffi::c_void);
pub struct IPersistHistory(pub *mut ::core::ffi::c_void);
pub struct IPrintManagerTemplatePrinter(pub *mut ::core::ffi::c_void);
pub struct IPrintManagerTemplatePrinter2(pub *mut ::core::ffi::c_void);
pub struct IPrintTaskRequestFactory(pub *mut ::core::ffi::c_void);
pub struct IPrintTaskRequestHandler(pub *mut ::core::ffi::c_void);
pub struct IRangeException(pub *mut ::core::ffi::c_void);
pub struct IRulesApplied(pub *mut ::core::ffi::c_void);
pub struct IRulesAppliedCollection(pub *mut ::core::ffi::c_void);
pub struct ISVGAElement(pub *mut ::core::ffi::c_void);
pub struct ISVGAngle(pub *mut ::core::ffi::c_void);
pub struct ISVGAnimatedAngle(pub *mut ::core::ffi::c_void);
pub struct ISVGAnimatedBoolean(pub *mut ::core::ffi::c_void);
pub struct ISVGAnimatedEnumeration(pub *mut ::core::ffi::c_void);
pub struct ISVGAnimatedInteger(pub *mut ::core::ffi::c_void);
pub struct ISVGAnimatedLength(pub *mut ::core::ffi::c_void);
pub struct ISVGAnimatedLengthList(pub *mut ::core::ffi::c_void);
pub struct ISVGAnimatedNumber(pub *mut ::core::ffi::c_void);
pub struct ISVGAnimatedNumberList(pub *mut ::core::ffi::c_void);
pub struct ISVGAnimatedPathData(pub *mut ::core::ffi::c_void);
pub struct ISVGAnimatedPoints(pub *mut ::core::ffi::c_void);
pub struct ISVGAnimatedPreserveAspectRatio(pub *mut ::core::ffi::c_void);
pub struct ISVGAnimatedRect(pub *mut ::core::ffi::c_void);
pub struct ISVGAnimatedString(pub *mut ::core::ffi::c_void);
pub struct ISVGAnimatedTransformList(pub *mut ::core::ffi::c_void);
pub struct ISVGCircleElement(pub *mut ::core::ffi::c_void);
pub struct ISVGClipPathElement(pub *mut ::core::ffi::c_void);
pub struct ISVGDefsElement(pub *mut ::core::ffi::c_void);
pub struct ISVGDescElement(pub *mut ::core::ffi::c_void);
pub struct ISVGDocument(pub *mut ::core::ffi::c_void);
pub struct ISVGElement(pub *mut ::core::ffi::c_void);
pub struct ISVGElementInstance(pub *mut ::core::ffi::c_void);
pub struct ISVGElementInstanceList(pub *mut ::core::ffi::c_void);
pub struct ISVGEllipseElement(pub *mut ::core::ffi::c_void);
pub struct ISVGException(pub *mut ::core::ffi::c_void);
pub struct ISVGExternalResourcesRequired(pub *mut ::core::ffi::c_void);
pub struct ISVGFitToViewBox(pub *mut ::core::ffi::c_void);
pub struct ISVGGElement(pub *mut ::core::ffi::c_void);
pub struct ISVGGradientElement(pub *mut ::core::ffi::c_void);
pub struct ISVGImageElement(pub *mut ::core::ffi::c_void);
pub struct ISVGLangSpace(pub *mut ::core::ffi::c_void);
pub struct ISVGLength(pub *mut ::core::ffi::c_void);
pub struct ISVGLengthList(pub *mut ::core::ffi::c_void);
pub struct ISVGLineElement(pub *mut ::core::ffi::c_void);
pub struct ISVGLinearGradientElement(pub *mut ::core::ffi::c_void);
pub struct ISVGLocatable(pub *mut ::core::ffi::c_void);
pub struct ISVGMarkerElement(pub *mut ::core::ffi::c_void);
pub struct ISVGMaskElement(pub *mut ::core::ffi::c_void);
pub struct ISVGMatrix(pub *mut ::core::ffi::c_void);
pub struct ISVGMetadataElement(pub *mut ::core::ffi::c_void);
pub struct ISVGNumber(pub *mut ::core::ffi::c_void);
pub struct ISVGNumberList(pub *mut ::core::ffi::c_void);
pub struct ISVGPaint(pub *mut ::core::ffi::c_void);
pub struct ISVGPathElement(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSeg(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegArcAbs(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegArcRel(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegClosePath(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegCurvetoCubicAbs(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegCurvetoCubicRel(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegCurvetoCubicSmoothAbs(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegCurvetoCubicSmoothRel(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegCurvetoQuadraticAbs(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegCurvetoQuadraticRel(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegCurvetoQuadraticSmoothAbs(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegCurvetoQuadraticSmoothRel(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegLinetoAbs(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegLinetoHorizontalAbs(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegLinetoHorizontalRel(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegLinetoRel(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegLinetoVerticalAbs(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegLinetoVerticalRel(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegList(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegMovetoAbs(pub *mut ::core::ffi::c_void);
pub struct ISVGPathSegMovetoRel(pub *mut ::core::ffi::c_void);
pub struct ISVGPatternElement(pub *mut ::core::ffi::c_void);
pub struct ISVGPoint(pub *mut ::core::ffi::c_void);
pub struct ISVGPointList(pub *mut ::core::ffi::c_void);
pub struct ISVGPolygonElement(pub *mut ::core::ffi::c_void);
pub struct ISVGPolylineElement(pub *mut ::core::ffi::c_void);
pub struct ISVGPreserveAspectRatio(pub *mut ::core::ffi::c_void);
pub struct ISVGRadialGradientElement(pub *mut ::core::ffi::c_void);
pub struct ISVGRect(pub *mut ::core::ffi::c_void);
pub struct ISVGRectElement(pub *mut ::core::ffi::c_void);
pub struct ISVGSVGElement(pub *mut ::core::ffi::c_void);
pub struct ISVGScriptElement(pub *mut ::core::ffi::c_void);
pub struct ISVGStopElement(pub *mut ::core::ffi::c_void);
pub struct ISVGStringList(pub *mut ::core::ffi::c_void);
pub struct ISVGStylable(pub *mut ::core::ffi::c_void);
pub struct ISVGStyleElement(pub *mut ::core::ffi::c_void);
pub struct ISVGSwitchElement(pub *mut ::core::ffi::c_void);
pub struct ISVGSymbolElement(pub *mut ::core::ffi::c_void);
pub struct ISVGTSpanElement(pub *mut ::core::ffi::c_void);
pub struct ISVGTests(pub *mut ::core::ffi::c_void);
pub struct ISVGTextContentElement(pub *mut ::core::ffi::c_void);
pub struct ISVGTextElement(pub *mut ::core::ffi::c_void);
pub struct ISVGTextPathElement(pub *mut ::core::ffi::c_void);
pub struct ISVGTextPositioningElement(pub *mut ::core::ffi::c_void);
pub struct ISVGTitleElement(pub *mut ::core::ffi::c_void);
pub struct ISVGTransform(pub *mut ::core::ffi::c_void);
pub struct ISVGTransformList(pub *mut ::core::ffi::c_void);
pub struct ISVGTransformable(pub *mut ::core::ffi::c_void);
pub struct ISVGURIReference(pub *mut ::core::ffi::c_void);
pub struct ISVGUseElement(pub *mut ::core::ffi::c_void);
pub struct ISVGViewElement(pub *mut ::core::ffi::c_void);
pub struct ISVGViewSpec(pub *mut ::core::ffi::c_void);
pub struct ISVGZoomAndPan(pub *mut ::core::ffi::c_void);
pub struct ISVGZoomEvent(pub *mut ::core::ffi::c_void);
pub struct IScriptEventHandler(pub *mut ::core::ffi::c_void);
pub struct IScriptEventHandlerSourceInfo(pub *mut ::core::ffi::c_void);
pub struct IScrollableContextMenu(pub *mut ::core::ffi::c_void);
pub struct IScrollableContextMenu2(pub *mut ::core::ffi::c_void);
pub struct ISecureUrlHost(pub *mut ::core::ffi::c_void);
pub struct ISegment(pub *mut ::core::ffi::c_void);
pub struct ISegmentList(pub *mut ::core::ffi::c_void);
pub struct ISegmentListIterator(pub *mut ::core::ffi::c_void);
pub struct ISelectionServices(pub *mut ::core::ffi::c_void);
pub struct ISelectionServicesListener(pub *mut ::core::ffi::c_void);
pub struct ISequenceNumber(pub *mut ::core::ffi::c_void);
pub struct ISniffStream(pub *mut ::core::ffi::c_void);
pub struct ISurfacePresenter(pub *mut ::core::ffi::c_void);
pub struct ISurfacePresenterFlip(pub *mut ::core::ffi::c_void);
pub struct ISurfacePresenterFlip2(pub *mut ::core::ffi::c_void);
pub struct ISurfacePresenterFlipBuffer(pub *mut ::core::ffi::c_void);
pub struct ITargetContainer(pub *mut ::core::ffi::c_void);
pub struct ITargetEmbedding(pub *mut ::core::ffi::c_void);
pub struct ITargetFrame(pub *mut ::core::ffi::c_void);
pub struct ITargetFrame2(pub *mut ::core::ffi::c_void);
pub struct ITargetFramePriv(pub *mut ::core::ffi::c_void);
pub struct ITargetFramePriv2(pub *mut ::core::ffi::c_void);
pub struct ITargetNotify(pub *mut ::core::ffi::c_void);
pub struct ITargetNotify2(pub *mut ::core::ffi::c_void);
pub struct ITemplatePrinter(pub *mut ::core::ffi::c_void);
pub struct ITemplatePrinter2(pub *mut ::core::ffi::c_void);
pub struct ITemplatePrinter3(pub *mut ::core::ffi::c_void);
pub struct ITimer(pub *mut ::core::ffi::c_void);
pub struct ITimerEx(pub *mut ::core::ffi::c_void);
pub struct ITimerService(pub *mut ::core::ffi::c_void);
pub struct ITimerSink(pub *mut ::core::ffi::c_void);
pub struct ITrackingProtection(pub *mut ::core::ffi::c_void);
pub struct ITridentTouchInput(pub *mut ::core::ffi::c_void);
pub struct ITridentTouchInputSite(pub *mut ::core::ffi::c_void);
pub struct IUrlHistoryNotify(pub *mut ::core::ffi::c_void);
pub struct IUrlHistoryStg(pub *mut ::core::ffi::c_void);
pub struct IUrlHistoryStg2(pub *mut ::core::ffi::c_void);
pub struct IViewObjectPresentFlip(pub *mut ::core::ffi::c_void);
pub struct IViewObjectPresentFlip2(pub *mut ::core::ffi::c_void);
pub struct IViewObjectPresentFlipSite(pub *mut ::core::ffi::c_void);
pub struct IViewObjectPresentFlipSite2(pub *mut ::core::ffi::c_void);
pub struct IViewObjectPresentNotify(pub *mut ::core::ffi::c_void);
pub struct IViewObjectPresentNotifySite(pub *mut ::core::ffi::c_void);
pub struct IViewObjectPresentSite(pub *mut ::core::ffi::c_void);
pub struct IViewObjectPrint(pub *mut ::core::ffi::c_void);
pub struct IWBScriptControl(pub *mut ::core::ffi::c_void);
pub struct IWPCBlockedUrls(pub *mut ::core::ffi::c_void);
pub struct IWebBridge(pub *mut ::core::ffi::c_void);
pub struct IWebBrowserEventsService(pub *mut ::core::ffi::c_void);
pub struct IWebBrowserEventsUrlService(pub *mut ::core::ffi::c_void);
pub struct IWebGeocoordinates(pub *mut ::core::ffi::c_void);
pub struct IWebGeolocation(pub *mut ::core::ffi::c_void);
pub struct IWebGeoposition(pub *mut ::core::ffi::c_void);
pub struct IWebGeopositionError(pub *mut ::core::ffi::c_void);
pub struct IXMLGenericParse(pub *mut ::core::ffi::c_void);
pub struct IXMLHttpRequestEventTarget(pub *mut ::core::ffi::c_void);
pub struct IntelliForms(i32);
pub struct InternetExplorerManager(i32);
pub struct Iwfolders(pub *mut ::core::ffi::c_void);
pub struct LINE_DIRECTION(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const LINKSBAND: u32 = 4u32;
pub struct LayoutRectEvents(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const MAPMIME_CLSID: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const MAPMIME_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const MAPMIME_DEFAULT_ALWAYS: u32 = 3u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const MAPMIME_DISABLE: u32 = 2u32;
pub struct MARKUP_CONTEXT_TYPE(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const MAX_SEARCH_FORMAT_STRING: u32 = 255u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const MENUEXT_SHOWDIALOG: u32 = 1u32;
pub struct MOVEUNIT_ACTION(i32);
pub struct MediaActivityNotifyType(i32);
pub struct NodeIterator(i32);
pub struct OldHTMLDocument(i32);
pub struct OldHTMLFormElement(i32);
pub struct OpenServiceActivityContentType(i32);
pub struct OpenServiceActivityManager(i32);
pub struct OpenServiceErrors(i32);
pub struct OpenServiceManager(i32);
pub struct PARSE_FLAGS(i32);
pub struct POINTER_GRAVITY(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const PRINT_DONTBOTHERUSER: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const PRINT_WAITFORCOMPLETION: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const PRIVACY_URLHASCOMPACTPOLICY: u32 = 131072u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const PRIVACY_URLHASP3PHEADER: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const PRIVACY_URLHASPOLICYREFHEADER: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const PRIVACY_URLHASPOLICYREFLINK: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const PRIVACY_URLHASPOSTDATA: u32 = 524288u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const PRIVACY_URLISTOPLEVEL: u32 = 65536u32;
pub struct PeerFactory(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_FONT_SIZE_DEF: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_JAVAJIT_DEF: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_JAVALOGGING_DEF: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_SCHANNELENABLEPROTOCOL_DEF: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_SECURITYACTICEXSCRIPTS_DEF: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_SECURITYACTIVEX_DEF: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_SECURITYALLOWCOOKIES_DEF: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_SECURITYDISABLECACHINGOFSSLPAGES_DEF: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_SECURITYJAVA_DEF: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_SECURITYWARNONBADCERTSENDING_DEF: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_SECURITYWARNONBADCERTVIEWING_DEF: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_SECURITYWARNONSENDALWAYS_DEF: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_SECURITYWARNONSEND_DEF: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_SECURITYWARNONVIEW_DEF: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_SECURITYWARNONZONECROSSING_DEF: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_SMOOTHSCROLL_DEF: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_USEICM_DEF: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const REGSTR_VAL_VISIBLEBANDS_DEF: u32 = 7u32;
pub struct RangeException(i32);
pub struct RulesApplied(i32);
pub struct RulesAppliedCollection(i32);
pub struct SAVE_SEGMENTS_FLAGS(i32);
pub struct SCRIPT_TIMER_TYPE(i32);
pub struct SCROLLABLECONTEXTMENU_PLACEMENT(i32);
pub struct SECUREURLHOSTVALIDATE_FLAGS(i32);
pub struct SELECTION_TYPE(i32);
pub struct SHOWHTMLDIALOGEXFN(i32);
pub struct SHOWHTMLDIALOGFN(i32);
pub struct SHOWMODELESSHTMLDIALOGFN(i32);
pub const SID_SEditCommandTarget: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611893, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SID_SHTMLEditHost: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612384, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const SID_SHTMLEditServices: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810612729, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
#[cfg(feature = "Win32_Foundation")]
pub struct STATURL(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STATURLFLAG_ISCACHED: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STATURLFLAG_ISTOPLEVEL: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STATURL_QUERYFLAG_ISCACHED: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STATURL_QUERYFLAG_NOTITLE: u32 = 262144u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STATURL_QUERYFLAG_NOURL: u32 = 131072u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STATURL_QUERYFLAG_TOPLEVEL: u32 = 524288u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_AFTERUPDATE: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_BEFOREUPDATE: u32 = 65540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ERRORUPDATE: u32 = 65549u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONBEFORECOPY: u32 = 65566u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONBEFORECUT: u32 = 65565u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONBEFOREPASTE: u32 = 65567u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONCELLCHANGE: u32 = 65570u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONCOPY: u32 = 65563u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONCUT: u32 = 65562u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONDATAAVAILABLE: u32 = 65551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONDATASETCHANGED: u32 = 65550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONDATASETCOMPLETE: u32 = 65552u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONDRAG: u32 = 65556u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONDRAGEND: u32 = 65557u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONDRAGENTER: u32 = 65558u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONDRAGLEAVE: u32 = 65560u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONDRAGOVER: u32 = 65559u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONDRAGSTART: u32 = 65547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONDROP: u32 = 65561u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONFILTER: u32 = 65553u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONFOCUS: u32 = 65537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONHELP: u32 = 65546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONLOSECAPTURE: u32 = 65554u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONMOUSEOUT: u32 = 65545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONMOUSEOVER: u32 = 65544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONPASTE: u32 = 65564u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONPROPERTYCHANGE: u32 = 65555u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONROWENTER: u32 = 65543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONROWEXIT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONROWSDELETE: u32 = 65568u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONROWSINSERTED: u32 = 65569u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDDISPID_XOBJ_ONSELECTSTART: u32 = 65548u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_IE3XOBJ_OBJECTALIGN: u32 = 65537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_ALIGNPERSIST: u32 = 65596u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_APPLICATION: u32 = 65607u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_BASEHREF: u32 = 65538u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_BLOCKALIGN: u32 = 65608u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_BOTTOM: u32 = 65614u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_CANCEL: u32 = 65592u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_CONTROLALIGN: u32 = 65609u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_CONTROLTIPTEXT: u32 = 65605u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_COUNT: u32 = 65611u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_DATACHANGED: u32 = 65601u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_DATAFIELD: u32 = 65602u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_DATASOURCE: u32 = 65603u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_DEFAULT: u32 = 65591u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_DISABLED: u32 = 65612u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_DRAGICON: u32 = 65546u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_DRAGMODE: u32 = 65545u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_GETSVGDOCUMENT: u32 = 65615u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_HEIGHT: u32 = 65542u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_HELPCONTEXTID: u32 = 65586u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_INDEX: u32 = 65537u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_LEFT: u32 = 65539u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_LEFTNORUN: u32 = 65593u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_LINKITEM: u32 = 65599u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_LINKMODE: u32 = 65600u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_LINKTIMEOUT: u32 = 65597u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_LINKTOPIC: u32 = 65598u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_PARENT: u32 = 65544u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_RIGHT: u32 = 65613u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_STATUSBARTEXT: u32 = 65606u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_STYLE: u32 = 65610u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_TABINDEX: u32 = 65551u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_TABSTOP: u32 = 65550u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_TAG: u32 = 65547u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_TOP: u32 = 65540u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_TOPNORUN: u32 = 65594u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_VISIBLE: u32 = 65543u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_WHATSTHISHELPID: u32 = 65604u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const STDPROPID_XOBJ_WIDTH: u32 = 65541u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const SURFACE_LOCK_ALLOW_DISCARD: u32 = 2u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const SURFACE_LOCK_EXCLUSIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const SURFACE_LOCK_WAIT: u32 = 4u32;
pub struct SVGAElement(i32);
pub struct SVGAngle(i32);
pub struct SVGAnimatedAngle(i32);
pub struct SVGAnimatedBoolean(i32);
pub struct SVGAnimatedEnumeration(i32);
pub struct SVGAnimatedInteger(i32);
pub struct SVGAnimatedLength(i32);
pub struct SVGAnimatedLengthList(i32);
pub struct SVGAnimatedNumber(i32);
pub struct SVGAnimatedNumberList(i32);
pub struct SVGAnimatedPreserveAspectRatio(i32);
pub struct SVGAnimatedRect(i32);
pub struct SVGAnimatedString(i32);
pub struct SVGAnimatedTransformList(i32);
pub struct SVGCircleElement(i32);
pub struct SVGClipPathElement(i32);
pub struct SVGDefsElement(i32);
pub struct SVGDescElement(i32);
pub struct SVGElement(i32);
pub struct SVGElementInstance(i32);
pub struct SVGElementInstanceList(i32);
pub struct SVGEllipseElement(i32);
pub struct SVGException(i32);
pub struct SVGGElement(i32);
pub struct SVGGradientElement(i32);
pub struct SVGImageElement(i32);
pub struct SVGLength(i32);
pub struct SVGLengthList(i32);
pub struct SVGLineElement(i32);
pub struct SVGLinearGradientElement(i32);
pub struct SVGMarkerElement(i32);
pub struct SVGMaskElement(i32);
pub struct SVGMatrix(i32);
pub struct SVGMetadataElement(i32);
pub struct SVGNumber(i32);
pub struct SVGNumberList(i32);
pub struct SVGPathElement(i32);
pub struct SVGPathSeg(i32);
pub struct SVGPathSegArcAbs(i32);
pub struct SVGPathSegArcRel(i32);
pub struct SVGPathSegClosePath(i32);
pub struct SVGPathSegCurvetoCubicAbs(i32);
pub struct SVGPathSegCurvetoCubicRel(i32);
pub struct SVGPathSegCurvetoCubicSmoothAbs(i32);
pub struct SVGPathSegCurvetoCubicSmoothRel(i32);
pub struct SVGPathSegCurvetoQuadraticAbs(i32);
pub struct SVGPathSegCurvetoQuadraticRel(i32);
pub struct SVGPathSegCurvetoQuadraticSmoothAbs(i32);
pub struct SVGPathSegCurvetoQuadraticSmoothRel(i32);
pub struct SVGPathSegLinetoAbs(i32);
pub struct SVGPathSegLinetoHorizontalAbs(i32);
pub struct SVGPathSegLinetoHorizontalRel(i32);
pub struct SVGPathSegLinetoRel(i32);
pub struct SVGPathSegLinetoVerticalAbs(i32);
pub struct SVGPathSegLinetoVerticalRel(i32);
pub struct SVGPathSegList(i32);
pub struct SVGPathSegMovetoAbs(i32);
pub struct SVGPathSegMovetoRel(i32);
pub struct SVGPatternElement(i32);
pub struct SVGPoint(i32);
pub struct SVGPointList(i32);
pub struct SVGPolygonElement(i32);
pub struct SVGPolylineElement(i32);
pub struct SVGPreserveAspectRatio(i32);
pub struct SVGRadialGradientElement(i32);
pub struct SVGRect(i32);
pub struct SVGRectElement(i32);
pub struct SVGSVGElement(i32);
pub struct SVGScriptElement(i32);
pub struct SVGStopElement(i32);
pub struct SVGStringList(i32);
pub struct SVGStyleElement(i32);
pub struct SVGSwitchElement(i32);
pub struct SVGSymbolElement(i32);
pub struct SVGTSpanElement(i32);
pub struct SVGTextContentElement(i32);
pub struct SVGTextElement(i32);
pub struct SVGTextPathElement(i32);
pub struct SVGTextPositioningElement(i32);
pub struct SVGTitleElement(i32);
pub struct SVGTransform(i32);
pub struct SVGTransformList(i32);
pub struct SVGUseElement(i32);
pub struct SVGViewElement(i32);
pub struct SVGZoomEvent(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const S_SURFACE_DISCARDED: i32 = 49155i32;
pub struct Scriptlet(i32);
pub struct StaticNodeList(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const TF_NAVIGATE: u32 = 2142153644u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const TIMERMODE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const TIMERMODE_VISIBILITYAWARE: u32 = 1u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const TOOLSBAND: u32 = 1u32;
pub struct ThreadDialogProcParam(i32);
pub struct TreeWalker(i32);
pub struct VIEW_OBJECT_ALPHA_MODE(i32);
pub struct VIEW_OBJECT_COMPOSITION_MODE(i32);
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const WEBOC_DISPIDBASE: u32 = 70536u32;
#[doc = "*Required features: `Win32_Web_MsHtml`*"]
pub const WEBOC_DISPIDMAX: u32 = 70636u32;
pub struct WebGeocoordinates(i32);
pub struct WebGeolocation(i32);
pub struct WebGeoposition(i32);
pub struct WebGeopositionError(i32);
pub struct XDomainRequest(i32);
pub struct XDomainRequestFactory(i32);
pub struct XMLHttpRequestEventTarget(i32);
pub struct XMLSerializer(i32);
pub struct __MIDL_ITargetFrame2_0001(i32);
pub struct __MIDL_ITargetFrame2_0002(i32);
pub struct __MIDL_ITargetFrame_0001(i32);
pub struct bodyScroll(i32);
pub struct frameScrolling(i32);
pub struct htmlAdjacency(i32);
pub struct htmlApplyLocation(i32);
pub struct htmlBlockAlign(i32);
pub struct htmlCaptionAlign(i32);
pub struct htmlCaptionVAlign(i32);
pub struct htmlCellAlign(i32);
pub struct htmlCellVAlign(i32);
pub struct htmlClear(i32);
pub struct htmlCompatMode(i32);
pub struct htmlComponent(i32);
pub struct htmlControlAlign(i32);
pub struct htmlDesignMode(i32);
pub struct htmlDir(i32);
pub struct htmlDirection(i32);
pub struct htmlDraggable(i32);
pub struct htmlDropEffect(i32);
pub struct htmlEditable(i32);
pub struct htmlEffectAllowed(i32);
pub struct htmlEncoding(i32);
pub struct htmlEndPoints(i32);
pub struct htmlFrame(i32);
pub struct htmlGlyphMode(i32);
pub struct htmlInput(i32);
pub struct htmlListType(i32);
pub struct htmlLoop(i32);
pub struct htmlMarqueeBehavior(i32);
pub struct htmlMarqueeDirection(i32);
pub struct htmlMediaErr(i32);
pub struct htmlMediaNetworkState(i32);
pub struct htmlMediaReadyState(i32);
pub struct htmlMethod(i32);
pub struct htmlPersistState(i32);
pub struct htmlReadyState(i32);
pub struct htmlRules(i32);
pub struct htmlSelectExFlag(i32);
pub struct htmlSelectType(i32);
pub struct htmlSelection(i32);
pub struct htmlSpellCheck(i32);
pub struct htmlStart(i32);
pub struct htmlTabIndex(i32);
pub struct htmlUnit(i32);
pub struct htmlWrap(i32);
pub struct htmlZOrder(i32);
pub struct lengthAdjust(i32);
pub struct mediaType(i32);
pub struct sandboxAllow(i32);
pub struct styleAccelerator(i32);
pub struct styleAlignContent(i32);
pub struct styleAlignItems(i32);
pub struct styleAlignSelf(i32);
pub struct styleAlignmentBaseline(i32);
pub struct styleAttrType(i32);
pub struct styleAuto(i32);
pub struct styleBackfaceVisibility(i32);
pub struct styleBackgroundAttachment(i32);
pub struct styleBackgroundAttachment3(i32);
pub struct styleBackgroundClip(i32);
pub struct styleBackgroundOrigin(i32);
pub struct styleBackgroundRepeat(i32);
pub struct styleBaselineShift(i32);
pub struct styleBidi(i32);
pub struct styleBlockProgression(i32);
pub struct styleBool(i32);
pub struct styleBorderCollapse(i32);
pub struct styleBorderImageRepeat(i32);
pub struct styleBorderImageSliceFill(i32);
pub struct styleBorderStyle(i32);
pub struct styleBorderWidth(i32);
pub struct styleBoxSizing(i32);
pub struct styleBreak(i32);
pub struct styleBreakInside(i32);
pub struct styleCaptionSide(i32);
pub struct styleClipRule(i32);
pub struct styleColorInterpolationFilters(i32);
pub struct styleColumnFill(i32);
pub struct styleColumnSpan(i32);
pub struct styleCursor(i32);
pub struct styleDataRepeat(i32);
pub struct styleDefaultTextSelection(i32);
pub struct styleDir(i32);
pub struct styleDisplay(i32);
pub struct styleDominantBaseline(i32);
pub struct styleEmptyCells(i32);
pub struct styleEnableBackground(i32);
pub struct styleFillRule(i32);
pub struct styleFlex(i32);
pub struct styleFlexBasis(i32);
pub struct styleFlexDirection(i32);
pub struct styleFlexWrap(i32);
pub struct styleFontSize(i32);
pub struct styleFontStretch(i32);
pub struct styleFontStyle(i32);
pub struct styleFontVariant(i32);
pub struct styleFontWeight(i32);
pub struct styleGridColumn(i32);
pub struct styleGridColumnAlign(i32);
pub struct styleGridColumnSpan(i32);
pub struct styleGridRow(i32);
pub struct styleGridRowAlign(i32);
pub struct styleGridRowSpan(i32);
pub struct styleHyphenateLimitLines(i32);
pub struct styleHyphens(i32);
pub struct styleImeMode(i32);
pub struct styleInitialColor(i32);
pub struct styleInitialString(i32);
pub struct styleInterpolation(i32);
pub struct styleJustifyContent(i32);
pub struct styleLayoutFlow(i32);
pub struct styleLayoutGridChar(i32);
pub struct styleLayoutGridLine(i32);
pub struct styleLayoutGridMode(i32);
pub struct styleLayoutGridType(i32);
pub struct styleLineBreak(i32);
pub struct styleListStylePosition(i32);
pub struct styleListStyleType(i32);
pub struct styleMsAnimationDirection(i32);
pub struct styleMsAnimationFillMode(i32);
pub struct styleMsAnimationPlayState(i32);
pub struct styleMsContentZoomChaining(i32);
pub struct styleMsContentZoomSnapType(i32);
pub struct styleMsContentZooming(i32);
pub struct styleMsFlexAlign(i32);
pub struct styleMsFlexItemAlign(i32);
pub struct styleMsFlexLinePack(i32);
pub struct styleMsFlexPack(i32);
pub struct styleMsHighContrastAdjust(i32);
pub struct styleMsImeAlign(i32);
pub struct styleMsOverflowStyle(i32);
pub struct styleMsScrollChaining(i32);
pub struct styleMsScrollRails(i32);
pub struct styleMsScrollSnapType(i32);
pub struct styleMsScrollTranslation(i32);
pub struct styleMsTextCombineHorizontal(i32);
pub struct styleMsTouchAction(i32);
pub struct styleMsTouchSelect(i32);
pub struct styleMsUserSelect(i32);
pub struct styleNone(i32);
pub struct styleNormal(i32);
pub struct styleOutlineStyle(i32);
pub struct styleOverflow(i32);
pub struct stylePageBreak(i32);
pub struct stylePageBreakInside(i32);
pub struct stylePerspectiveOriginX(i32);
pub struct stylePerspectiveOriginY(i32);
pub struct stylePointerEvents(i32);
pub struct stylePosition(i32);
pub struct styleRubyAlign(i32);
pub struct styleRubyOverhang(i32);
pub struct styleRubyPosition(i32);
pub struct styleStrokeLinecap(i32);
pub struct styleStrokeLinejoin(i32);
pub struct styleStyleFloat(i32);
pub struct styleTableLayout(i32);
pub struct styleTextAlignLast(i32);
pub struct styleTextAnchor(i32);
pub struct styleTextDecoration(i32);
pub struct styleTextEffect(i32);
pub struct styleTextJustify(i32);
pub struct styleTextJustifyTrim(i32);
pub struct styleTextLineThroughStyle(i32);
pub struct styleTextOverflow(i32);
pub struct styleTextSizeAdjust(i32);
pub struct styleTextTransform(i32);
pub struct styleTextUnderlinePosition(i32);
pub struct styleTextUnderlineStyle(i32);
pub struct styleTransformOriginX(i32);
pub struct styleTransformOriginY(i32);
pub struct styleTransformStyle(i32);
pub struct styleUserZoom(i32);
pub struct styleVerticalAlign(i32);
pub struct styleViewportSize(i32);
pub struct styleVisibility(i32);
pub struct styleWebkitAppearance(i32);
pub struct styleWebkitBoxDirection(i32);
pub struct styleWebkitBoxOrient(i32);
pub struct styleWebkitBoxPack(i32);
pub struct styleWhiteSpace(i32);
pub struct styleWidowsOrphans(i32);
pub struct styleWordBreak(i32);
pub struct styleWordWrap(i32);
pub struct styleWrapFlow(i32);
pub struct styleWrapThrough(i32);
pub struct styleWritingMode(i32);
pub struct styleZIndex(i32);
pub struct svgAngleType(i32);
pub struct svgChannel(i32);
pub struct svgEdgemode(i32);
pub struct svgExternalResourcesRequired(i32);
pub struct svgFeblendMode(i32);
pub struct svgFecolormatrixType(i32);
pub struct svgFecomponenttransferType(i32);
pub struct svgFecompositeOperator(i32);
pub struct svgFocusable(i32);
pub struct svgLengthType(i32);
pub struct svgMarkerOrient(i32);
pub struct svgMarkerOrientAttribute(i32);
pub struct svgMarkerUnits(i32);
pub struct svgMorphologyOperator(i32);
pub struct svgPathSegType(i32);
pub struct svgPreserveAlpha(i32);
pub struct svgPreserveAspectMeetOrSliceType(i32);
pub struct svgPreserveAspectRatioAlignType(i32);
pub struct svgSpreadMethod(i32);
pub struct svgStitchtype(i32);
pub struct svgTransformType(i32);
pub struct svgTurbulenceType(i32);
pub struct svgUnitTypes(i32);
pub struct tagNavigateData(i32);
pub struct textDecoration(i32);
pub struct textpathMethodtype(i32);
pub struct textpathSpacingtype(i32);
pub struct wfolders(i32);
