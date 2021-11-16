#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
pub const DWMFLIP3D_DEFAULT: i32 = 0i32;
pub const DWMFLIP3D_EXCLUDEBELOW: i32 = 1i32;
pub const DWMFLIP3D_EXCLUDEABOVE: i32 = 2i32;
pub const DWMFLIP3D_LAST: i32 = 3i32;
pub const DWMNCRP_USEWINDOWSTYLE: i32 = 0i32;
pub const DWMNCRP_DISABLED: i32 = 1i32;
pub const DWMNCRP_ENABLED: i32 = 2i32;
pub const DWMNCRP_LAST: i32 = 3i32;
pub const DWMTRANSITION_OWNEDWINDOW_NULL: i32 = -1i32;
pub const DWMTRANSITION_OWNEDWINDOW_REPOSITION: i32 = 0i32;
pub const DWMWA_COLOR_DEFAULT: u32 = 4294967295u32;
pub const DWMWA_COLOR_NONE: u32 = 4294967294u32;
pub const DWMWA_NCRENDERING_ENABLED: i32 = 1i32;
pub const DWMWA_NCRENDERING_POLICY: i32 = 2i32;
pub const DWMWA_TRANSITIONS_FORCEDISABLED: i32 = 3i32;
pub const DWMWA_ALLOW_NCPAINT: i32 = 4i32;
pub const DWMWA_CAPTION_BUTTON_BOUNDS: i32 = 5i32;
pub const DWMWA_NONCLIENT_RTL_LAYOUT: i32 = 6i32;
pub const DWMWA_FORCE_ICONIC_REPRESENTATION: i32 = 7i32;
pub const DWMWA_FLIP3D_POLICY: i32 = 8i32;
pub const DWMWA_EXTENDED_FRAME_BOUNDS: i32 = 9i32;
pub const DWMWA_HAS_ICONIC_BITMAP: i32 = 10i32;
pub const DWMWA_DISALLOW_PEEK: i32 = 11i32;
pub const DWMWA_EXCLUDED_FROM_PEEK: i32 = 12i32;
pub const DWMWA_CLOAK: i32 = 13i32;
pub const DWMWA_CLOAKED: i32 = 14i32;
pub const DWMWA_FREEZE_REPRESENTATION: i32 = 15i32;
pub const DWMWA_PASSIVE_UPDATE_MODE: i32 = 16i32;
pub const DWMWA_USE_HOSTBACKDROPBRUSH: i32 = 17i32;
pub const DWMWA_USE_IMMERSIVE_DARK_MODE: i32 = 20i32;
pub const DWMWA_WINDOW_CORNER_PREFERENCE: i32 = 33i32;
pub const DWMWA_BORDER_COLOR: i32 = 34i32;
pub const DWMWA_CAPTION_COLOR: i32 = 35i32;
pub const DWMWA_TEXT_COLOR: i32 = 36i32;
pub const DWMWA_VISIBLE_FRAME_BORDER_THICKNESS: i32 = 37i32;
pub const DWMWA_LAST: i32 = 38i32;
pub const DWM_BB_BLURREGION: u32 = 2u32;
pub const DWM_BB_ENABLE: u32 = 1u32;
pub const DWM_BB_TRANSITIONONMAXIMIZED: u32 = 4u32;
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DWM_BLURBEHIND {
    pub dwFlags: u32,
    pub fEnable: super::super::Foundation::BOOL,
    pub hRgnBlur: super::Gdi::HRGN,
    pub fTransitionOnMaximized: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DWM_BLURBEHIND {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DWM_BLURBEHIND {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWM_CLOAKED_APP: u32 = 1u32;
pub const DWM_CLOAKED_INHERITED: u32 = 4u32;
pub const DWM_CLOAKED_SHELL: u32 = 2u32;
pub const DWM_EC_DISABLECOMPOSITION: u32 = 0u32;
pub const DWM_EC_ENABLECOMPOSITION: u32 = 1u32;
pub const DWM_FRAME_DURATION_DEFAULT: i32 = -1i32;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct DWM_PRESENT_PARAMETERS {
    pub cbSize: u32,
    pub fQueue: super::super::Foundation::BOOL,
    pub cRefreshStart: u64,
    pub cBuffer: u32,
    pub fUseSourceRate: super::super::Foundation::BOOL,
    pub rateSource: UNSIGNED_RATIO,
    pub cRefreshesPerFrame: u32,
    pub eSampling: DWM_SOURCE_FRAME_SAMPLING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWM_PRESENT_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWM_PRESENT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWMSC_DOWN: u32 = 1u32;
pub const DWMSC_UP: u32 = 2u32;
pub const DWMSC_DRAG: u32 = 4u32;
pub const DWMSC_HOLD: u32 = 8u32;
pub const DWMSC_PENBARREL: u32 = 16u32;
pub const DWMSC_NONE: u32 = 0u32;
pub const DWMSC_ALL: u32 = 4294967295u32;
pub const DWM_SIT_DISPLAYFRAME: u32 = 1u32;
pub const DWM_SOURCE_FRAME_SAMPLING_POINT: i32 = 0i32;
pub const DWM_SOURCE_FRAME_SAMPLING_COVERAGE: i32 = 1i32;
pub const DWM_SOURCE_FRAME_SAMPLING_LAST: i32 = 2i32;
pub const DWMTWR_NONE: u32 = 0u32;
pub const DWMTWR_IMPLEMENTED_BY_SYSTEM: u32 = 1u32;
pub const DWMTWR_WINDOW_RELATIONSHIP: u32 = 2u32;
pub const DWMTWR_WINDOW_STYLES: u32 = 4u32;
pub const DWMTWR_WINDOW_REGION: u32 = 8u32;
pub const DWMTWR_WINDOW_DWM_ATTRIBUTES: u32 = 16u32;
pub const DWMTWR_WINDOW_MARGINS: u32 = 32u32;
pub const DWMTWR_TABBING_ENABLED: u32 = 64u32;
pub const DWMTWR_USER_POLICY: u32 = 128u32;
pub const DWMTWR_GROUP_POLICY: u32 = 256u32;
pub const DWMTWR_APP_COMPAT: u32 = 512u32;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct DWM_THUMBNAIL_PROPERTIES {
    pub dwFlags: u32,
    pub rcDestination: super::super::Foundation::RECT,
    pub rcSource: super::super::Foundation::RECT,
    pub opacity: u8,
    pub fVisible: super::super::Foundation::BOOL,
    pub fSourceClientAreaOnly: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWM_THUMBNAIL_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWM_THUMBNAIL_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct DWM_TIMING_INFO {
    pub cbSize: u32,
    pub rateRefresh: UNSIGNED_RATIO,
    pub qpcRefreshPeriod: u64,
    pub rateCompose: UNSIGNED_RATIO,
    pub qpcVBlank: u64,
    pub cRefresh: u64,
    pub cDXRefresh: u32,
    pub qpcCompose: u64,
    pub cFrame: u64,
    pub cDXPresent: u32,
    pub cRefreshFrame: u64,
    pub cFrameSubmitted: u64,
    pub cDXPresentSubmitted: u32,
    pub cFrameConfirmed: u64,
    pub cDXPresentConfirmed: u32,
    pub cRefreshConfirmed: u64,
    pub cDXRefreshConfirmed: u32,
    pub cFramesLate: u64,
    pub cFramesOutstanding: u32,
    pub cFrameDisplayed: u64,
    pub qpcFrameDisplayed: u64,
    pub cRefreshFrameDisplayed: u64,
    pub cFrameComplete: u64,
    pub qpcFrameComplete: u64,
    pub cFramePending: u64,
    pub qpcFramePending: u64,
    pub cFramesDisplayed: u64,
    pub cFramesComplete: u64,
    pub cFramesPending: u64,
    pub cFramesAvailable: u64,
    pub cFramesDropped: u64,
    pub cFramesMissed: u64,
    pub cRefreshNextDisplayed: u64,
    pub cRefreshNextPresented: u64,
    pub cRefreshesDisplayed: u64,
    pub cRefreshesPresented: u64,
    pub cRefreshStarted: u64,
    pub cPixelsReceived: u64,
    pub cPixelsDrawn: u64,
    pub cBuffersEmpty: u64,
}
impl ::core::marker::Copy for DWM_TIMING_INFO {}
impl ::core::clone::Clone for DWM_TIMING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWM_TNP_OPACITY: u32 = 4u32;
pub const DWM_TNP_RECTDESTINATION: u32 = 1u32;
pub const DWM_TNP_RECTSOURCE: u32 = 2u32;
pub const DWM_TNP_SOURCECLIENTAREAONLY: u32 = 16u32;
pub const DWM_TNP_VISIBLE: u32 = 8u32;
pub const DWMWCP_DEFAULT: i32 = 0i32;
pub const DWMWCP_DONOTROUND: i32 = 1i32;
pub const DWMWCP_ROUND: i32 = 2i32;
pub const DWMWCP_ROUNDSMALL: i32 = 3i32;
pub const GT_PEN_TAP: i32 = 0i32;
pub const GT_PEN_DOUBLETAP: i32 = 1i32;
pub const GT_PEN_RIGHTTAP: i32 = 2i32;
pub const GT_PEN_PRESSANDHOLD: i32 = 3i32;
pub const GT_PEN_PRESSANDHOLDABORT: i32 = 4i32;
pub const GT_TOUCH_TAP: i32 = 5i32;
pub const GT_TOUCH_DOUBLETAP: i32 = 6i32;
pub const GT_TOUCH_RIGHTTAP: i32 = 7i32;
pub const GT_TOUCH_PRESSANDHOLD: i32 = 8i32;
pub const GT_TOUCH_PRESSANDHOLDABORT: i32 = 9i32;
pub const GT_TOUCH_PRESSANDTAP: i32 = 10i32;
#[repr(C, packed(1))]
pub struct MilMatrix3x2D {
    pub S_11: f64,
    pub S_12: f64,
    pub S_21: f64,
    pub S_22: f64,
    pub DX: f64,
    pub DY: f64,
}
impl ::core::marker::Copy for MilMatrix3x2D {}
impl ::core::clone::Clone for MilMatrix3x2D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct UNSIGNED_RATIO {
    pub uiNumerator: u32,
    pub uiDenominator: u32,
}
impl ::core::marker::Copy for UNSIGNED_RATIO {}
impl ::core::clone::Clone for UNSIGNED_RATIO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const c_DwmMaxAdapters: u32 = 16u32;
pub const c_DwmMaxMonitors: u32 = 16u32;
pub const c_DwmMaxQueuedBuffers: u32 = 8u32;
