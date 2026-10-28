#[cfg(feature = "windef")]
windows_link::link!("dwmapi.dll" "system" fn DwmAttachMilContent(hwnd : super::HWND) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("dwmapi.dll" "system" fn DwmDefWindowProc(hwnd : super::HWND, msg : super::UINT, wparam : super::WPARAM, lparam : super::LPARAM, plresult : *mut super::LRESULT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("dwmapi.dll" "system" fn DwmDetachMilContent(hwnd : super::HWND) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("dwmapi.dll" "system" fn DwmEnableBlurBehindWindow(hwnd : super::HWND, pblurbehind : *const DWM_BLURBEHIND) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("dwmapi.dll" "system" fn DwmEnableComposition(ucompositionaction : super::UINT) -> windows_sys::core::HRESULT);
windows_link::link!("dwmapi.dll" "system" fn DwmEnableMMCSS(fenablemmcss : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "uxtheme", feature = "windef"))]
windows_link::link!("dwmapi.dll" "system" fn DwmExtendFrameIntoClientArea(hwnd : super::HWND, pmarinset : *const super::MARGINS) -> windows_sys::core::HRESULT);
windows_link::link!("dwmapi.dll" "system" fn DwmFlush() -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("dwmapi.dll" "system" fn DwmGetColorizationColor(pcrcolorization : *mut super::DWORD, pfopaqueblend : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("dwmapi.dll" "system" fn DwmGetCompositionTimingInfo(hwnd : super::HWND, ptiminginfo : *mut DWM_TIMING_INFO) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "rpc", feature = "shared"))]
windows_link::link!("dwmapi.dll" "system" fn DwmGetGraphicsStreamClient(uindex : super::UINT, pclientuuid : *mut super::UUID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("dwmapi.dll" "system" fn DwmGetGraphicsStreamTransformHint(uindex : super::UINT, ptransform : *mut MilMatrix3x2D) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("dwmapi.dll" "system" fn DwmGetTransportAttributes(pfisremoting : *mut windows_sys::core::BOOL, pfisconnected : *mut windows_sys::core::BOOL, pdwgeneration : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("dwmapi.dll" "system" fn DwmGetUnmetTabRequirements(appwindow : super::HWND, value : *mut DWM_TAB_WINDOW_REQUIREMENTS) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("dwmapi.dll" "system" fn DwmGetWindowAttribute(hwnd : super::HWND, dwattribute : super::DWORD, pvattribute : super::PVOID, cbattribute : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("dwmapi.dll" "system" fn DwmInvalidateIconicBitmaps(hwnd : super::HWND) -> windows_sys::core::HRESULT);
windows_link::link!("dwmapi.dll" "system" fn DwmIsCompositionEnabled(pfenabled : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("dwmapi.dll" "system" fn DwmModifyPreviousDxFrameDuration(hwnd : super::HWND, crefreshes : super::INT, frelative : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("dwmapi.dll" "system" fn DwmQueryThumbnailSourceSize(hthumbnail : HTHUMBNAIL, psize : super::PSIZE) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("dwmapi.dll" "system" fn DwmRegisterThumbnail(hwnddestination : super::HWND, hwndsource : super::HWND, phthumbnailid : PHTHUMBNAIL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("dwmapi.dll" "system" fn DwmRenderGesture(gt : GESTURE_TYPE, ccontacts : super::UINT, pdwpointerid : *const super::DWORD, ppoints : *const super::POINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("dwmapi.dll" "system" fn DwmSetDxFrameDuration(hwnd : super::HWND, crefreshes : super::INT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("dwmapi.dll" "system" fn DwmSetIconicLivePreviewBitmap(hwnd : super::HWND, hbmp : super::HBITMAP, pptclient : *const super::POINT, dwsitflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("dwmapi.dll" "system" fn DwmSetIconicThumbnail(hwnd : super::HWND, hbmp : super::HBITMAP, dwsitflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("dwmapi.dll" "system" fn DwmSetPresentParameters(hwnd : super::HWND, ppresentparams : *mut DWM_PRESENT_PARAMETERS) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("dwmapi.dll" "system" fn DwmSetWindowAttribute(hwnd : super::HWND, dwattribute : super::DWORD, pvattribute : super::LPCVOID, cbattribute : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("dwmapi.dll" "system" fn DwmShowContact(dwpointerid : super::DWORD, eshowcontact : DWM_SHOWCONTACT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("dwmapi.dll" "system" fn DwmTetherContact(dwpointerid : super::DWORD, fenable : windows_sys::core::BOOL, pttether : super::POINT) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("dwmapi.dll" "system" fn DwmTransitionOwnedWindow(hwnd : super::HWND, target : DWMTRANSITION_OWNEDWINDOW_TARGET) -> windows_sys::core::HRESULT);
windows_link::link!("dwmapi.dll" "system" fn DwmUnregisterThumbnail(hthumbnailid : HTHUMBNAIL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("dwmapi.dll" "system" fn DwmUpdateThumbnailProperties(hthumbnailid : HTHUMBNAIL, ptnproperties : *const DWM_THUMBNAIL_PROPERTIES) -> windows_sys::core::HRESULT);
pub type DWMFLIP3DWINDOWPOLICY = i32;
pub const DWMFLIP3D_DEFAULT: DWMFLIP3DWINDOWPOLICY = 0;
pub const DWMFLIP3D_EXCLUDEABOVE: DWMFLIP3DWINDOWPOLICY = 2;
pub const DWMFLIP3D_EXCLUDEBELOW: DWMFLIP3DWINDOWPOLICY = 1;
pub const DWMFLIP3D_LAST: DWMFLIP3DWINDOWPOLICY = 3;
pub type DWMNCRENDERINGPOLICY = i32;
pub const DWMNCRP_DISABLED: DWMNCRENDERINGPOLICY = 1;
pub const DWMNCRP_ENABLED: DWMNCRENDERINGPOLICY = 2;
pub const DWMNCRP_LAST: DWMNCRENDERINGPOLICY = 3;
pub const DWMNCRP_USEWINDOWSTYLE: DWMNCRENDERINGPOLICY = 0;
pub const DWMSBT_AUTO: DWM_SYSTEMBACKDROP_TYPE = 0;
pub const DWMSBT_MAINWINDOW: DWM_SYSTEMBACKDROP_TYPE = 2;
pub const DWMSBT_NONE: DWM_SYSTEMBACKDROP_TYPE = 1;
pub const DWMSBT_TABBEDWINDOW: DWM_SYSTEMBACKDROP_TYPE = 4;
pub const DWMSBT_TRANSIENTWINDOW: DWM_SYSTEMBACKDROP_TYPE = 3;
pub const DWMSC_ALL: DWM_SHOWCONTACT = -1;
pub const DWMSC_DOWN: DWM_SHOWCONTACT = 1;
pub const DWMSC_DRAG: DWM_SHOWCONTACT = 4;
pub const DWMSC_HOLD: DWM_SHOWCONTACT = 8;
pub const DWMSC_NONE: DWM_SHOWCONTACT = 0;
pub const DWMSC_PENBARREL: DWM_SHOWCONTACT = 16;
pub const DWMSC_UP: DWM_SHOWCONTACT = 2;
pub const DWMTRANSITION_OWNEDWINDOW_NULL: DWMTRANSITION_OWNEDWINDOW_TARGET = -1;
pub const DWMTRANSITION_OWNEDWINDOW_REPOSITION: DWMTRANSITION_OWNEDWINDOW_TARGET = 0;
pub type DWMTRANSITION_OWNEDWINDOW_TARGET = i32;
pub const DWMTWR_APP_COMPAT: DWM_TAB_WINDOW_REQUIREMENTS = 512;
pub const DWMTWR_GROUP_POLICY: DWM_TAB_WINDOW_REQUIREMENTS = 256;
pub const DWMTWR_IMPLEMENTED_BY_SYSTEM: DWM_TAB_WINDOW_REQUIREMENTS = 1;
pub const DWMTWR_NONE: DWM_TAB_WINDOW_REQUIREMENTS = 0;
pub const DWMTWR_TABBING_ENABLED: DWM_TAB_WINDOW_REQUIREMENTS = 64;
pub const DWMTWR_USER_POLICY: DWM_TAB_WINDOW_REQUIREMENTS = 128;
pub const DWMTWR_WINDOW_DWM_ATTRIBUTES: DWM_TAB_WINDOW_REQUIREMENTS = 16;
pub const DWMTWR_WINDOW_MARGINS: DWM_TAB_WINDOW_REQUIREMENTS = 32;
pub const DWMTWR_WINDOW_REGION: DWM_TAB_WINDOW_REQUIREMENTS = 8;
pub const DWMTWR_WINDOW_RELATIONSHIP: DWM_TAB_WINDOW_REQUIREMENTS = 2;
pub const DWMTWR_WINDOW_STYLES: DWM_TAB_WINDOW_REQUIREMENTS = 4;
pub const DWMWA_ALLOW_NCPAINT: DWMWINDOWATTRIBUTE = 4;
pub const DWMWA_BORDER_COLOR: DWMWINDOWATTRIBUTE = 34;
pub const DWMWA_BORDER_MARGINS: DWMWINDOWATTRIBUTE = 40;
pub const DWMWA_CAPTION_BUTTON_BOUNDS: DWMWINDOWATTRIBUTE = 5;
pub const DWMWA_CAPTION_COLOR: DWMWINDOWATTRIBUTE = 35;
pub const DWMWA_CLOAK: DWMWINDOWATTRIBUTE = 13;
pub const DWMWA_CLOAKED: DWMWINDOWATTRIBUTE = 14;
pub const DWMWA_COLOR_DEFAULT: u32 = 4294967295;
pub const DWMWA_COLOR_NONE: u32 = 4294967294;
pub const DWMWA_DISALLOW_PEEK: DWMWINDOWATTRIBUTE = 11;
pub const DWMWA_EXCLUDED_FROM_PEEK: DWMWINDOWATTRIBUTE = 12;
pub const DWMWA_EXTENDED_FRAME_BOUNDS: DWMWINDOWATTRIBUTE = 9;
pub const DWMWA_FLIP3D_POLICY: DWMWINDOWATTRIBUTE = 8;
pub const DWMWA_FORCE_ICONIC_REPRESENTATION: DWMWINDOWATTRIBUTE = 7;
pub const DWMWA_FREEZE_REPRESENTATION: DWMWINDOWATTRIBUTE = 15;
pub const DWMWA_HAS_ICONIC_BITMAP: DWMWINDOWATTRIBUTE = 10;
pub const DWMWA_LAST: DWMWINDOWATTRIBUTE = 41;
pub const DWMWA_NCRENDERING_ENABLED: DWMWINDOWATTRIBUTE = 1;
pub const DWMWA_NCRENDERING_POLICY: DWMWINDOWATTRIBUTE = 2;
pub const DWMWA_NONCLIENT_RTL_LAYOUT: DWMWINDOWATTRIBUTE = 6;
pub const DWMWA_PASSIVE_UPDATE_MODE: DWMWINDOWATTRIBUTE = 16;
pub const DWMWA_REDIRECTIONBITMAP_ALPHA: DWMWINDOWATTRIBUTE = 39;
pub const DWMWA_SYSTEMBACKDROP_TYPE: DWMWINDOWATTRIBUTE = 38;
pub const DWMWA_TEXT_COLOR: DWMWINDOWATTRIBUTE = 36;
pub const DWMWA_TRANSITIONS_FORCEDISABLED: DWMWINDOWATTRIBUTE = 3;
pub const DWMWA_USE_HOSTBACKDROPBRUSH: DWMWINDOWATTRIBUTE = 17;
pub const DWMWA_USE_IMMERSIVE_DARK_MODE: DWMWINDOWATTRIBUTE = 20;
pub const DWMWA_VISIBLE_FRAME_BORDER_THICKNESS: DWMWINDOWATTRIBUTE = 37;
pub const DWMWA_WINDOW_CORNER_PREFERENCE: DWMWINDOWATTRIBUTE = 33;
pub const DWMWCP_DEFAULT: DWM_WINDOW_CORNER_PREFERENCE = 0;
pub const DWMWCP_DONOTROUND: DWM_WINDOW_CORNER_PREFERENCE = 1;
pub const DWMWCP_ROUND: DWM_WINDOW_CORNER_PREFERENCE = 2;
pub const DWMWCP_ROUNDSMALL: DWM_WINDOW_CORNER_PREFERENCE = 3;
pub type DWMWINDOWATTRIBUTE = i32;
pub const DWM_BB_BLURREGION: i32 = 2;
pub const DWM_BB_ENABLE: i32 = 1;
pub const DWM_BB_TRANSITIONONMAXIMIZED: i32 = 4;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct DWM_BLURBEHIND {
    pub dwFlags: super::DWORD,
    pub fEnable: windows_sys::core::BOOL,
    pub hRgnBlur: super::HRGN,
    pub fTransitionOnMaximized: windows_sys::core::BOOL,
}
pub const DWM_CLOAKED_APP: i32 = 1;
pub const DWM_CLOAKED_INHERITED: i32 = 4;
pub const DWM_CLOAKED_SHELL: i32 = 2;
pub const DWM_EC_DISABLECOMPOSITION: i32 = 0;
pub const DWM_EC_ENABLECOMPOSITION: i32 = 1;
pub type DWM_FRAME_COUNT = u64;
pub const DWM_FRAME_DURATION_DEFAULT: i32 = -1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DWM_PRESENT_PARAMETERS {
    pub cbSize: super::UINT32,
    pub fQueue: windows_sys::core::BOOL,
    pub cRefreshStart: DWM_FRAME_COUNT,
    pub cBuffer: super::UINT,
    pub fUseSourceRate: windows_sys::core::BOOL,
    pub rateSource: UNSIGNED_RATIO,
    pub cRefreshesPerFrame: super::UINT,
    pub eSampling: DWM_SOURCE_FRAME_SAMPLING,
}
pub type DWM_SHOWCONTACT = i32;
pub const DWM_SIT_DISPLAYFRAME: i32 = 1;
pub type DWM_SOURCE_FRAME_SAMPLING = i32;
pub const DWM_SOURCE_FRAME_SAMPLING_COVERAGE: DWM_SOURCE_FRAME_SAMPLING = 1;
pub const DWM_SOURCE_FRAME_SAMPLING_LAST: DWM_SOURCE_FRAME_SAMPLING = 2;
pub const DWM_SOURCE_FRAME_SAMPLING_POINT: DWM_SOURCE_FRAME_SAMPLING = 0;
pub type DWM_SYSTEMBACKDROP_TYPE = i32;
pub type DWM_TAB_WINDOW_REQUIREMENTS = i32;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct DWM_THUMBNAIL_PROPERTIES {
    pub dwFlags: super::DWORD,
    pub rcDestination: super::RECT,
    pub rcSource: super::RECT,
    pub opacity: super::BYTE,
    pub fVisible: windows_sys::core::BOOL,
    pub fSourceClientAreaOnly: windows_sys::core::BOOL,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DWM_TIMING_INFO {
    pub cbSize: super::UINT32,
    pub rateRefresh: UNSIGNED_RATIO,
    pub qpcRefreshPeriod: QPC_TIME,
    pub rateCompose: UNSIGNED_RATIO,
    pub qpcVBlank: QPC_TIME,
    pub cRefresh: DWM_FRAME_COUNT,
    pub cDXRefresh: super::UINT,
    pub qpcCompose: QPC_TIME,
    pub cFrame: DWM_FRAME_COUNT,
    pub cDXPresent: super::UINT,
    pub cRefreshFrame: DWM_FRAME_COUNT,
    pub cFrameSubmitted: DWM_FRAME_COUNT,
    pub cDXPresentSubmitted: super::UINT,
    pub cFrameConfirmed: DWM_FRAME_COUNT,
    pub cDXPresentConfirmed: super::UINT,
    pub cRefreshConfirmed: DWM_FRAME_COUNT,
    pub cDXRefreshConfirmed: super::UINT,
    pub cFramesLate: DWM_FRAME_COUNT,
    pub cFramesOutstanding: super::UINT,
    pub cFrameDisplayed: DWM_FRAME_COUNT,
    pub qpcFrameDisplayed: QPC_TIME,
    pub cRefreshFrameDisplayed: DWM_FRAME_COUNT,
    pub cFrameComplete: DWM_FRAME_COUNT,
    pub qpcFrameComplete: QPC_TIME,
    pub cFramePending: DWM_FRAME_COUNT,
    pub qpcFramePending: QPC_TIME,
    pub cFramesDisplayed: DWM_FRAME_COUNT,
    pub cFramesComplete: DWM_FRAME_COUNT,
    pub cFramesPending: DWM_FRAME_COUNT,
    pub cFramesAvailable: DWM_FRAME_COUNT,
    pub cFramesDropped: DWM_FRAME_COUNT,
    pub cFramesMissed: DWM_FRAME_COUNT,
    pub cRefreshNextDisplayed: DWM_FRAME_COUNT,
    pub cRefreshNextPresented: DWM_FRAME_COUNT,
    pub cRefreshesDisplayed: DWM_FRAME_COUNT,
    pub cRefreshesPresented: DWM_FRAME_COUNT,
    pub cRefreshStarted: DWM_FRAME_COUNT,
    pub cPixelsReceived: super::ULONGLONG,
    pub cPixelsDrawn: super::ULONGLONG,
    pub cBuffersEmpty: DWM_FRAME_COUNT,
}
pub const DWM_TNP_OPACITY: i32 = 4;
pub const DWM_TNP_RECTDESTINATION: i32 = 1;
pub const DWM_TNP_RECTSOURCE: i32 = 2;
pub const DWM_TNP_SOURCECLIENTAREAONLY: i32 = 16;
pub const DWM_TNP_VISIBLE: i32 = 8;
pub type DWM_WINDOW_CORNER_PREFERENCE = i32;
pub type GESTURE_TYPE = i32;
pub const GT_PEN_DOUBLETAP: GESTURE_TYPE = 1;
pub const GT_PEN_PRESSANDHOLD: GESTURE_TYPE = 3;
pub const GT_PEN_PRESSANDHOLDABORT: GESTURE_TYPE = 4;
pub const GT_PEN_RIGHTTAP: GESTURE_TYPE = 2;
pub const GT_PEN_TAP: GESTURE_TYPE = 0;
pub const GT_TOUCH_DOUBLETAP: GESTURE_TYPE = 6;
pub const GT_TOUCH_PRESSANDHOLD: GESTURE_TYPE = 8;
pub const GT_TOUCH_PRESSANDHOLDABORT: GESTURE_TYPE = 9;
pub const GT_TOUCH_PRESSANDTAP: GESTURE_TYPE = 10;
pub const GT_TOUCH_RIGHTTAP: GESTURE_TYPE = 7;
pub const GT_TOUCH_TAP: GESTURE_TYPE = 5;
pub type HTHUMBNAIL = *mut core::ffi::c_void;
#[cfg(feature = "wtypesbase")]
pub type MIL_MATRIX3X2D = MilMatrix3x2D;
#[repr(C, packed(1))]
#[cfg(feature = "wtypesbase")]
#[derive(Clone, Copy, Default)]
pub struct MilMatrix3x2D {
    pub S_11: super::DOUBLE,
    pub S_12: super::DOUBLE,
    pub S_21: super::DOUBLE,
    pub S_22: super::DOUBLE,
    pub DX: super::DOUBLE,
    pub DY: super::DOUBLE,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PDWM_BLURBEHIND = *mut DWM_BLURBEHIND;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PDWM_THUMBNAIL_PROPERTIES = *mut DWM_THUMBNAIL_PROPERTIES;
pub type PHTHUMBNAIL = *mut *mut core::ffi::c_void;
pub type QPC_TIME = u64;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct UNSIGNED_RATIO {
    pub uiNumerator: super::UINT32,
    pub uiDenominator: super::UINT32,
}
#[cfg(feature = "shared")]
pub const c_DwmMaxAdapters: super::UINT = 16;
#[cfg(feature = "shared")]
pub const c_DwmMaxMonitors: super::UINT = 16;
#[cfg(feature = "shared")]
pub const c_DwmMaxQueuedBuffers: super::UINT = 8;
