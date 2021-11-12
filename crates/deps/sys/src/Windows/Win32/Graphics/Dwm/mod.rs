#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmAttachMilContent(hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmDefWindowProc(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmDetachMilContent(hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DwmEnableBlurBehindWindow(hwnd: super::super::Foundation::HWND, pblurbehind: *const DWM_BLURBEHIND) -> ::windows_sys::core::HRESULT;
    pub fn DwmEnableComposition(ucompositionaction: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmEnableMMCSS(fenablemmcss: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn DwmExtendFrameIntoClientArea(hwnd: super::super::Foundation::HWND, pmarinset: *const super::super::UI::Controls::MARGINS) -> ::windows_sys::core::HRESULT;
    pub fn DwmFlush() -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmGetColorizationColor(pcrcolorization: *mut u32, pfopaqueblend: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmGetCompositionTimingInfo(hwnd: super::super::Foundation::HWND, ptiminginfo: *mut DWM_TIMING_INFO) -> ::windows_sys::core::HRESULT;
    pub fn DwmGetGraphicsStreamClient(uindex: u32, pclientuuid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    pub fn DwmGetGraphicsStreamTransformHint(uindex: u32, ptransform: *mut MilMatrix3x2D) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmGetTransportAttributes(pfisremoting: *mut super::super::Foundation::BOOL, pfisconnected: *mut super::super::Foundation::BOOL, pdwgeneration: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmGetUnmetTabRequirements(appwindow: super::super::Foundation::HWND, value: *mut DWM_TAB_WINDOW_REQUIREMENTS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmGetWindowAttribute(hwnd: super::super::Foundation::HWND, dwattribute: DWMWINDOWATTRIBUTE, pvattribute: *mut ::core::ffi::c_void, cbattribute: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmInvalidateIconicBitmaps(hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmIsCompositionEnabled(pfenabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmModifyPreviousDxFrameDuration(hwnd: super::super::Foundation::HWND, crefreshes: i32, frelative: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmQueryThumbnailSourceSize(hthumbnail: isize, psize: *mut super::super::Foundation::SIZE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmRegisterThumbnail(hwnddestination: super::super::Foundation::HWND, hwndsource: super::super::Foundation::HWND, phthumbnailid: *mut isize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmRenderGesture(gt: GESTURE_TYPE, ccontacts: u32, pdwpointerid: *const u32, ppoints: *const super::super::Foundation::POINT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmSetDxFrameDuration(hwnd: super::super::Foundation::HWND, crefreshes: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DwmSetIconicLivePreviewBitmap(hwnd: super::super::Foundation::HWND, hbmp: super::Gdi::HBITMAP, pptclient: *const super::super::Foundation::POINT, dwsitflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DwmSetIconicThumbnail(hwnd: super::super::Foundation::HWND, hbmp: super::Gdi::HBITMAP, dwsitflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmSetPresentParameters(hwnd: super::super::Foundation::HWND, ppresentparams: *mut DWM_PRESENT_PARAMETERS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmSetWindowAttribute(hwnd: super::super::Foundation::HWND, dwattribute: DWMWINDOWATTRIBUTE, pvattribute: *const ::core::ffi::c_void, cbattribute: u32) -> ::windows_sys::core::HRESULT;
    pub fn DwmShowContact(dwpointerid: u32, eshowcontact: DWM_SHOWCONTACT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmTetherContact(dwpointerid: u32, fenable: super::super::Foundation::BOOL, pttether: super::super::Foundation::POINT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmTransitionOwnedWindow(hwnd: super::super::Foundation::HWND, target: DWMTRANSITION_OWNEDWINDOW_TARGET) -> ::windows_sys::core::HRESULT;
    pub fn DwmUnregisterThumbnail(hthumbnailid: isize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmUpdateThumbnailProperties(hthumbnailid: isize, ptnproperties: *const DWM_THUMBNAIL_PROPERTIES) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
pub struct DWMFLIP3DWINDOWPOLICY(i32);
#[repr(C)]
pub struct DWMNCRENDERINGPOLICY(i32);
#[repr(C)]
pub struct DWMTRANSITION_OWNEDWINDOW_TARGET(i32);
pub const DWMWA_COLOR_DEFAULT: u32 = 4294967295u32;
pub const DWMWA_COLOR_NONE: u32 = 4294967294u32;
#[repr(C)]
pub struct DWMWINDOWATTRIBUTE(i32);
pub const DWM_BB_BLURREGION: u32 = 2u32;
pub const DWM_BB_ENABLE: u32 = 1u32;
pub const DWM_BB_TRANSITIONONMAXIMIZED: u32 = 4u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DWM_BLURBEHIND(i32);
pub const DWM_CLOAKED_APP: u32 = 1u32;
pub const DWM_CLOAKED_INHERITED: u32 = 4u32;
pub const DWM_CLOAKED_SHELL: u32 = 2u32;
pub const DWM_EC_DISABLECOMPOSITION: u32 = 0u32;
pub const DWM_EC_ENABLECOMPOSITION: u32 = 1u32;
pub const DWM_FRAME_DURATION_DEFAULT: i32 = -1i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DWM_PRESENT_PARAMETERS(i32);
#[repr(C)]
pub struct DWM_SHOWCONTACT(i32);
pub const DWM_SIT_DISPLAYFRAME: u32 = 1u32;
#[repr(C)]
pub struct DWM_SOURCE_FRAME_SAMPLING(i32);
#[repr(C)]
pub struct DWM_TAB_WINDOW_REQUIREMENTS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DWM_THUMBNAIL_PROPERTIES(i32);
#[repr(C)]
pub struct DWM_TIMING_INFO(i32);
pub const DWM_TNP_OPACITY: u32 = 4u32;
pub const DWM_TNP_RECTDESTINATION: u32 = 1u32;
pub const DWM_TNP_RECTSOURCE: u32 = 2u32;
pub const DWM_TNP_SOURCECLIENTAREAONLY: u32 = 16u32;
pub const DWM_TNP_VISIBLE: u32 = 8u32;
#[repr(C)]
pub struct DWM_WINDOW_CORNER_PREFERENCE(i32);
#[repr(C)]
pub struct GESTURE_TYPE(i32);
#[repr(C)]
pub struct MilMatrix3x2D(i32);
#[repr(C)]
pub struct UNSIGNED_RATIO(i32);
pub const c_DwmMaxAdapters: u32 = 16u32;
pub const c_DwmMaxMonitors: u32 = 16u32;
pub const c_DwmMaxQueuedBuffers: u32 = 8u32;
