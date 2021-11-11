#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ComputeInvCMAP(prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD, ncolors: u32, pinvtable: *mut u8, cbtable: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateDDrawSurfaceOnDIB(hbmdib: super::super::Graphics::Gdi::HBITMAP, ppsurface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn CreateMIMEMap(ppmap: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn DecodeImage(pstream: ::windows::runtime::RawPtr, pmap: ::windows::runtime::RawPtr, peventsink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DecodeImageEx(pstream: ::windows::runtime::RawPtr, pmap: ::windows::runtime::RawPtr, peventsink: ::windows::runtime::RawPtr, pszmimetypeparam: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DitherTo8(pdestbits: *mut u8, ndestpitch: i32, psrcbits: *mut u8, nsrcpitch: i32, bfidsrc: *const ::windows::runtime::GUID, prgbdestcolors: *mut super::super::Graphics::Gdi::RGBQUAD, prgbsrccolors: *mut super::super::Graphics::Gdi::RGBQUAD, pbdestinvmap: *mut u8, x: i32, y: i32, cx: i32, cy: i32, ldesttrans: i32, lsrctrans: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DoPrivacyDlg(hwndowner: super::super::Foundation::HWND, pszurl: super::super::Foundation::PWSTR, pprivacyenum: ::windows::runtime::RawPtr, freportallsites: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn GetMaxMIMEIDBytes(pnmaxbytes: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn IdentifyMIMEType(pbbytes: *const u8, nbytes: u32, pnformat: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAccessDeniedDialog(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PSTR, pszcontentdescription: super::super::Foundation::PSTR, pratingdetails: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAccessDeniedDialog2(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PSTR, pratingdetails: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAccessDeniedDialog2W(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PWSTR, pratingdetails: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAccessDeniedDialogW(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PWSTR, pszcontentdescription: super::super::Foundation::PWSTR, pratingdetails: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAddToApprovedSites(hdlg: super::super::Foundation::HWND, cbpasswordblob: u32, pbpasswordblob: *mut u8, lpszurl: super::super::Foundation::PWSTR, falwaysnever: super::super::Foundation::BOOL, fsitepage: super::super::Foundation::BOOL, fapprovedsitesenforced: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingCheckUserAccess(pszusername: super::super::Foundation::PSTR, pszurl: super::super::Foundation::PSTR, pszratinginfo: super::super::Foundation::PSTR, pdata: *const u8, cbdata: u32, ppratingdetails: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingCheckUserAccessW(pszusername: super::super::Foundation::PWSTR, pszurl: super::super::Foundation::PWSTR, pszratinginfo: super::super::Foundation::PWSTR, pdata: *const u8, cbdata: u32, ppratingdetails: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingClickedOnPRFInternal(hwndowner: super::super::Foundation::HWND, param1: super::super::Foundation::HINSTANCE, lpszfilename: super::super::Foundation::PSTR, nshow: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingClickedOnRATInternal(hwndowner: super::super::Foundation::HWND, param1: super::super::Foundation::HINSTANCE, lpszfilename: super::super::Foundation::PSTR, nshow: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingEnable(hwndparent: super::super::Foundation::HWND, pszusername: super::super::Foundation::PSTR, fenable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingEnableW(hwndparent: super::super::Foundation::HWND, pszusername: super::super::Foundation::PWSTR, fenable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn RatingEnabledQuery() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn RatingFreeDetails(pratingdetails: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn RatingInit() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingObtainCancel(hratingobtainquery: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingObtainQuery(psztargeturl: super::super::Foundation::PSTR, dwuserdata: u32, fcallback: isize, phratingobtainquery: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingObtainQueryW(psztargeturl: super::super::Foundation::PWSTR, dwuserdata: u32, fcallback: isize, phratingobtainquery: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingSetupUI(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingSetupUIW(hdlg: super::super::Foundation::HWND, pszusername: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SniffStream(pinstream: ::windows::runtime::RawPtr, pnformat: *mut u32, ppoutstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
}
