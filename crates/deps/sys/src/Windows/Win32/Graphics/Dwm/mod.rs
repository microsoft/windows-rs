#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct DWMFLIP3DWINDOWPOLICY(pub i32);
pub const DWMFLIP3D_DEFAULT: DWMFLIP3DWINDOWPOLICY = DWMFLIP3DWINDOWPOLICY(0i32);
pub const DWMFLIP3D_EXCLUDEBELOW: DWMFLIP3DWINDOWPOLICY = DWMFLIP3DWINDOWPOLICY(1i32);
pub const DWMFLIP3D_EXCLUDEABOVE: DWMFLIP3DWINDOWPOLICY = DWMFLIP3DWINDOWPOLICY(2i32);
pub const DWMFLIP3D_LAST: DWMFLIP3DWINDOWPOLICY = DWMFLIP3DWINDOWPOLICY(3i32);
#[repr(transparent)]
pub struct DWMNCRENDERINGPOLICY(pub i32);
pub const DWMNCRP_USEWINDOWSTYLE: DWMNCRENDERINGPOLICY = DWMNCRENDERINGPOLICY(0i32);
pub const DWMNCRP_DISABLED: DWMNCRENDERINGPOLICY = DWMNCRENDERINGPOLICY(1i32);
pub const DWMNCRP_ENABLED: DWMNCRENDERINGPOLICY = DWMNCRENDERINGPOLICY(2i32);
pub const DWMNCRP_LAST: DWMNCRENDERINGPOLICY = DWMNCRENDERINGPOLICY(3i32);
#[repr(transparent)]
pub struct DWMTRANSITION_OWNEDWINDOW_TARGET(pub i32);
pub const DWMTRANSITION_OWNEDWINDOW_NULL: DWMTRANSITION_OWNEDWINDOW_TARGET = DWMTRANSITION_OWNEDWINDOW_TARGET(-1i32);
pub const DWMTRANSITION_OWNEDWINDOW_REPOSITION: DWMTRANSITION_OWNEDWINDOW_TARGET = DWMTRANSITION_OWNEDWINDOW_TARGET(0i32);
pub const DWMWA_COLOR_DEFAULT: u32 = 4294967295u32;
pub const DWMWA_COLOR_NONE: u32 = 4294967294u32;
#[repr(transparent)]
pub struct DWMWINDOWATTRIBUTE(pub i32);
pub const DWMWA_NCRENDERING_ENABLED: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(1i32);
pub const DWMWA_NCRENDERING_POLICY: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(2i32);
pub const DWMWA_TRANSITIONS_FORCEDISABLED: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(3i32);
pub const DWMWA_ALLOW_NCPAINT: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(4i32);
pub const DWMWA_CAPTION_BUTTON_BOUNDS: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(5i32);
pub const DWMWA_NONCLIENT_RTL_LAYOUT: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(6i32);
pub const DWMWA_FORCE_ICONIC_REPRESENTATION: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(7i32);
pub const DWMWA_FLIP3D_POLICY: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(8i32);
pub const DWMWA_EXTENDED_FRAME_BOUNDS: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(9i32);
pub const DWMWA_HAS_ICONIC_BITMAP: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(10i32);
pub const DWMWA_DISALLOW_PEEK: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(11i32);
pub const DWMWA_EXCLUDED_FROM_PEEK: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(12i32);
pub const DWMWA_CLOAK: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(13i32);
pub const DWMWA_CLOAKED: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(14i32);
pub const DWMWA_FREEZE_REPRESENTATION: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(15i32);
pub const DWMWA_PASSIVE_UPDATE_MODE: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(16i32);
pub const DWMWA_USE_HOSTBACKDROPBRUSH: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(17i32);
pub const DWMWA_USE_IMMERSIVE_DARK_MODE: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(20i32);
pub const DWMWA_WINDOW_CORNER_PREFERENCE: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(33i32);
pub const DWMWA_BORDER_COLOR: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(34i32);
pub const DWMWA_CAPTION_COLOR: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(35i32);
pub const DWMWA_TEXT_COLOR: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(36i32);
pub const DWMWA_VISIBLE_FRAME_BORDER_THICKNESS: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(37i32);
pub const DWMWA_LAST: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(38i32);
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
#[repr(transparent)]
pub struct DWM_SHOWCONTACT(pub u32);
pub const DWMSC_DOWN: DWM_SHOWCONTACT = DWM_SHOWCONTACT(1u32);
pub const DWMSC_UP: DWM_SHOWCONTACT = DWM_SHOWCONTACT(2u32);
pub const DWMSC_DRAG: DWM_SHOWCONTACT = DWM_SHOWCONTACT(4u32);
pub const DWMSC_HOLD: DWM_SHOWCONTACT = DWM_SHOWCONTACT(8u32);
pub const DWMSC_PENBARREL: DWM_SHOWCONTACT = DWM_SHOWCONTACT(16u32);
pub const DWMSC_NONE: DWM_SHOWCONTACT = DWM_SHOWCONTACT(0u32);
pub const DWMSC_ALL: DWM_SHOWCONTACT = DWM_SHOWCONTACT(4294967295u32);
pub const DWM_SIT_DISPLAYFRAME: u32 = 1u32;
#[repr(transparent)]
pub struct DWM_SOURCE_FRAME_SAMPLING(pub i32);
pub const DWM_SOURCE_FRAME_SAMPLING_POINT: DWM_SOURCE_FRAME_SAMPLING = DWM_SOURCE_FRAME_SAMPLING(0i32);
pub const DWM_SOURCE_FRAME_SAMPLING_COVERAGE: DWM_SOURCE_FRAME_SAMPLING = DWM_SOURCE_FRAME_SAMPLING(1i32);
pub const DWM_SOURCE_FRAME_SAMPLING_LAST: DWM_SOURCE_FRAME_SAMPLING = DWM_SOURCE_FRAME_SAMPLING(2i32);
#[repr(transparent)]
pub struct DWM_TAB_WINDOW_REQUIREMENTS(pub u32);
pub const DWMTWR_NONE: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(0u32);
pub const DWMTWR_IMPLEMENTED_BY_SYSTEM: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(1u32);
pub const DWMTWR_WINDOW_RELATIONSHIP: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(2u32);
pub const DWMTWR_WINDOW_STYLES: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(4u32);
pub const DWMTWR_WINDOW_REGION: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(8u32);
pub const DWMTWR_WINDOW_DWM_ATTRIBUTES: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(16u32);
pub const DWMTWR_WINDOW_MARGINS: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(32u32);
pub const DWMTWR_TABBING_ENABLED: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(64u32);
pub const DWMTWR_USER_POLICY: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(128u32);
pub const DWMTWR_GROUP_POLICY: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(256u32);
pub const DWMTWR_APP_COMPAT: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(512u32);
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
#[repr(transparent)]
pub struct DWM_WINDOW_CORNER_PREFERENCE(pub i32);
pub const DWMWCP_DEFAULT: DWM_WINDOW_CORNER_PREFERENCE = DWM_WINDOW_CORNER_PREFERENCE(0i32);
pub const DWMWCP_DONOTROUND: DWM_WINDOW_CORNER_PREFERENCE = DWM_WINDOW_CORNER_PREFERENCE(1i32);
pub const DWMWCP_ROUND: DWM_WINDOW_CORNER_PREFERENCE = DWM_WINDOW_CORNER_PREFERENCE(2i32);
pub const DWMWCP_ROUNDSMALL: DWM_WINDOW_CORNER_PREFERENCE = DWM_WINDOW_CORNER_PREFERENCE(3i32);
#[repr(transparent)]
pub struct GESTURE_TYPE(pub i32);
pub const GT_PEN_TAP: GESTURE_TYPE = GESTURE_TYPE(0i32);
pub const GT_PEN_DOUBLETAP: GESTURE_TYPE = GESTURE_TYPE(1i32);
pub const GT_PEN_RIGHTTAP: GESTURE_TYPE = GESTURE_TYPE(2i32);
pub const GT_PEN_PRESSANDHOLD: GESTURE_TYPE = GESTURE_TYPE(3i32);
pub const GT_PEN_PRESSANDHOLDABORT: GESTURE_TYPE = GESTURE_TYPE(4i32);
pub const GT_TOUCH_TAP: GESTURE_TYPE = GESTURE_TYPE(5i32);
pub const GT_TOUCH_DOUBLETAP: GESTURE_TYPE = GESTURE_TYPE(6i32);
pub const GT_TOUCH_RIGHTTAP: GESTURE_TYPE = GESTURE_TYPE(7i32);
pub const GT_TOUCH_PRESSANDHOLD: GESTURE_TYPE = GESTURE_TYPE(8i32);
pub const GT_TOUCH_PRESSANDHOLDABORT: GESTURE_TYPE = GESTURE_TYPE(9i32);
pub const GT_TOUCH_PRESSANDTAP: GESTURE_TYPE = GESTURE_TYPE(10i32);
#[repr(C)]
pub struct MilMatrix3x2D(i32);
#[repr(C)]
pub struct UNSIGNED_RATIO(i32);
pub const c_DwmMaxAdapters: u32 = 16u32;
pub const c_DwmMaxMonitors: u32 = 16u32;
pub const c_DwmMaxQueuedBuffers: u32 = 8u32;
