#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmAttachMilContent(hwnd: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmDefWindowProc(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmDetachMilContent(hwnd: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DwmEnableBlurBehindWindow(hwnd: super::super::Foundation::HWND, pblurbehind: *const DWM_BLURBEHIND) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`*"]
    pub fn DwmEnableComposition(ucompositionaction: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmEnableMMCSS(fenablemmcss: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn DwmExtendFrameIntoClientArea(hwnd: super::super::Foundation::HWND, pmarinset: *const super::super::UI::Controls::MARGINS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`*"]
    pub fn DwmFlush() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmGetColorizationColor(pcrcolorization: *mut u32, pfopaqueblend: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmGetCompositionTimingInfo(hwnd: super::super::Foundation::HWND, ptiminginfo: *mut DWM_TIMING_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`*"]
    pub fn DwmGetGraphicsStreamClient(uindex: u32, pclientuuid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`*"]
    pub fn DwmGetGraphicsStreamTransformHint(uindex: u32, ptransform: *mut MilMatrix3x2D) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmGetTransportAttributes(pfisremoting: *mut super::super::Foundation::BOOL, pfisconnected: *mut super::super::Foundation::BOOL, pdwgeneration: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmGetUnmetTabRequirements(appwindow: super::super::Foundation::HWND, value: *mut DWM_TAB_WINDOW_REQUIREMENTS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmGetWindowAttribute(hwnd: super::super::Foundation::HWND, dwattribute: DWMWINDOWATTRIBUTE, pvattribute: *mut ::core::ffi::c_void, cbattribute: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmInvalidateIconicBitmaps(hwnd: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmIsCompositionEnabled(pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmModifyPreviousDxFrameDuration(hwnd: super::super::Foundation::HWND, crefreshes: i32, frelative: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmQueryThumbnailSourceSize(hthumbnail: isize, psize: *mut super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmRegisterThumbnail(hwnddestination: super::super::Foundation::HWND, hwndsource: super::super::Foundation::HWND, phthumbnailid: *mut isize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmRenderGesture(gt: GESTURE_TYPE, ccontacts: u32, pdwpointerid: *const u32, ppoints: *const super::super::Foundation::POINT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmSetDxFrameDuration(hwnd: super::super::Foundation::HWND, crefreshes: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DwmSetIconicLivePreviewBitmap(hwnd: super::super::Foundation::HWND, hbmp: super::Gdi::HBITMAP, pptclient: *const super::super::Foundation::POINT, dwsitflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DwmSetIconicThumbnail(hwnd: super::super::Foundation::HWND, hbmp: super::Gdi::HBITMAP, dwsitflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmSetPresentParameters(hwnd: super::super::Foundation::HWND, ppresentparams: *mut DWM_PRESENT_PARAMETERS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmSetWindowAttribute(hwnd: super::super::Foundation::HWND, dwattribute: DWMWINDOWATTRIBUTE, pvattribute: *const ::core::ffi::c_void, cbattribute: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`*"]
    pub fn DwmShowContact(dwpointerid: u32, eshowcontact: DWM_SHOWCONTACT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmTetherContact(dwpointerid: u32, fenable: super::super::Foundation::BOOL, pttether: super::super::Foundation::POINT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmTransitionOwnedWindow(hwnd: super::super::Foundation::HWND, target: DWMTRANSITION_OWNEDWINDOW_TARGET) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`*"]
    pub fn DwmUnregisterThumbnail(hthumbnailid: isize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmUpdateThumbnailProperties(hthumbnailid: isize, ptnproperties: *const DWM_THUMBNAIL_PROPERTIES) -> ::windows::runtime::HRESULT;
}
