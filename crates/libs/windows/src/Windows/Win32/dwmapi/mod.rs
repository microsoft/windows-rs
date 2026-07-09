#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DwmAttachMilContent(hwnd: super::windef::HWND) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmAttachMilContent(hwnd : super::windef::HWND) -> windows_core::HRESULT);
    unsafe { DwmAttachMilContent(hwnd) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DwmDefWindowProc(hwnd: super::windef::HWND, msg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, plresult: *mut super::minwindef::LRESULT) -> windows_core::BOOL {
    windows_core::link!("dwmapi.dll" "system" fn DwmDefWindowProc(hwnd : super::windef::HWND, msg : u32, wparam : super::minwindef::WPARAM, lparam : super::minwindef::LPARAM, plresult : *mut super::minwindef::LRESULT) -> windows_core::BOOL);
    unsafe { DwmDefWindowProc(hwnd, msg, wparam, lparam, plresult as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DwmDetachMilContent(hwnd: super::windef::HWND) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmDetachMilContent(hwnd : super::windef::HWND) -> windows_core::HRESULT);
    unsafe { DwmDetachMilContent(hwnd) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DwmEnableBlurBehindWindow(hwnd: super::windef::HWND, pblurbehind: *const DWM_BLURBEHIND) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmEnableBlurBehindWindow(hwnd : super::windef::HWND, pblurbehind : *const DWM_BLURBEHIND) -> windows_core::HRESULT);
    unsafe { DwmEnableBlurBehindWindow(hwnd, pblurbehind) }
}
#[inline]
pub unsafe fn DwmEnableComposition(ucompositionaction: u32) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmEnableComposition(ucompositionaction : u32) -> windows_core::HRESULT);
    unsafe { DwmEnableComposition(ucompositionaction) }
}
#[inline]
pub unsafe fn DwmEnableMMCSS(fenablemmcss: bool) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmEnableMMCSS(fenablemmcss : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { DwmEnableMMCSS(fenablemmcss.into()) }
}
#[cfg(all(feature = "uxtheme", feature = "windef"))]
#[inline]
pub unsafe fn DwmExtendFrameIntoClientArea(hwnd: super::windef::HWND, pmarinset: *const super::uxtheme::MARGINS) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmExtendFrameIntoClientArea(hwnd : super::windef::HWND, pmarinset : *const super::uxtheme::MARGINS) -> windows_core::HRESULT);
    unsafe { DwmExtendFrameIntoClientArea(hwnd, pmarinset) }
}
#[inline]
pub unsafe fn DwmFlush() -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmFlush() -> windows_core::HRESULT);
    unsafe { DwmFlush() }
}
#[inline]
pub unsafe fn DwmGetColorizationColor(pcrcolorization: *mut u32, pfopaqueblend: *mut windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmGetColorizationColor(pcrcolorization : *mut u32, pfopaqueblend : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { DwmGetColorizationColor(pcrcolorization as _, pfopaqueblend as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DwmGetCompositionTimingInfo(hwnd: super::windef::HWND, ptiminginfo: *mut DWM_TIMING_INFO) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmGetCompositionTimingInfo(hwnd : super::windef::HWND, ptiminginfo : *mut DWM_TIMING_INFO) -> windows_core::HRESULT);
    unsafe { DwmGetCompositionTimingInfo(hwnd, ptiminginfo as _) }
}
#[inline]
pub unsafe fn DwmGetGraphicsStreamClient(uindex: u32) -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("dwmapi.dll" "system" fn DwmGetGraphicsStreamClient(uindex : u32, pclientuuid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DwmGetGraphicsStreamClient(uindex, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DwmGetGraphicsStreamTransformHint(uindex: u32, ptransform: *mut MilMatrix3x2D) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmGetGraphicsStreamTransformHint(uindex : u32, ptransform : *mut MilMatrix3x2D) -> windows_core::HRESULT);
    unsafe { DwmGetGraphicsStreamTransformHint(uindex, ptransform as _) }
}
#[inline]
pub unsafe fn DwmGetTransportAttributes(pfisremoting: &mut [windows_core::BOOL; 1], pfisconnected: &mut [windows_core::BOOL; 1], pdwgeneration: &mut [u32; 1]) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmGetTransportAttributes(pfisremoting : *mut windows_core::BOOL, pfisconnected : *mut windows_core::BOOL, pdwgeneration : *mut u32) -> windows_core::HRESULT);
    unsafe { DwmGetTransportAttributes(core::mem::transmute(pfisremoting.as_ptr()), core::mem::transmute(pfisconnected.as_ptr()), core::mem::transmute(pdwgeneration.as_ptr())) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DwmGetUnmetTabRequirements(appwindow: Option<super::windef::HWND>) -> windows_core::Result<DWM_TAB_WINDOW_REQUIREMENTS> {
    windows_core::link!("dwmapi.dll" "system" fn DwmGetUnmetTabRequirements(appwindow : super::windef::HWND, value : *mut DWM_TAB_WINDOW_REQUIREMENTS) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DwmGetUnmetTabRequirements(appwindow.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DwmGetWindowAttribute(hwnd: super::windef::HWND, dwattribute: u32, pvattribute: *mut core::ffi::c_void, cbattribute: u32) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmGetWindowAttribute(hwnd : super::windef::HWND, dwattribute : u32, pvattribute : *mut core::ffi::c_void, cbattribute : u32) -> windows_core::HRESULT);
    unsafe { DwmGetWindowAttribute(hwnd, dwattribute, pvattribute as _, cbattribute) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DwmInvalidateIconicBitmaps(hwnd: super::windef::HWND) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmInvalidateIconicBitmaps(hwnd : super::windef::HWND) -> windows_core::HRESULT);
    unsafe { DwmInvalidateIconicBitmaps(hwnd) }
}
#[inline]
pub unsafe fn DwmIsCompositionEnabled() -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("dwmapi.dll" "system" fn DwmIsCompositionEnabled(pfenabled : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DwmIsCompositionEnabled(&mut result__).map(|| result__)
    }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DwmModifyPreviousDxFrameDuration(hwnd: super::windef::HWND, crefreshes: i32, frelative: bool) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmModifyPreviousDxFrameDuration(hwnd : super::windef::HWND, crefreshes : i32, frelative : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { DwmModifyPreviousDxFrameDuration(hwnd, crefreshes, frelative.into()) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn DwmQueryThumbnailSourceSize(hthumbnail: HTHUMBNAIL) -> windows_core::Result<super::windef::SIZE> {
    windows_core::link!("dwmapi.dll" "system" fn DwmQueryThumbnailSourceSize(hthumbnail : HTHUMBNAIL, psize : *mut super::windef::SIZE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DwmQueryThumbnailSourceSize(hthumbnail, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn DwmRegisterThumbnail(hwnddestination: super::windef::HWND, hwndsource: super::windef::HWND) -> windows_core::Result<HTHUMBNAIL> {
    windows_core::link!("dwmapi.dll" "system" fn DwmRegisterThumbnail(hwnddestination : super::windef::HWND, hwndsource : super::windef::HWND, phthumbnailid : *mut HTHUMBNAIL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DwmRegisterThumbnail(hwnddestination, hwndsource, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DwmRenderGesture(gt: GESTURE_TYPE, ccontacts: u32, pdwpointerid: *const u32, ppoints: *const super::windef::POINT) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmRenderGesture(gt : GESTURE_TYPE, ccontacts : u32, pdwpointerid : *const u32, ppoints : *const super::windef::POINT) -> windows_core::HRESULT);
    unsafe { DwmRenderGesture(gt, ccontacts, pdwpointerid, ppoints) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DwmSetDxFrameDuration(hwnd: super::windef::HWND, crefreshes: i32) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmSetDxFrameDuration(hwnd : super::windef::HWND, crefreshes : i32) -> windows_core::HRESULT);
    unsafe { DwmSetDxFrameDuration(hwnd, crefreshes) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DwmSetIconicLivePreviewBitmap(hwnd: super::windef::HWND, hbmp: super::windef::HBITMAP, pptclient: Option<*const super::windef::POINT>, dwsitflags: u32) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmSetIconicLivePreviewBitmap(hwnd : super::windef::HWND, hbmp : super::windef::HBITMAP, pptclient : *const super::windef::POINT, dwsitflags : u32) -> windows_core::HRESULT);
    unsafe { DwmSetIconicLivePreviewBitmap(hwnd, hbmp, pptclient.unwrap_or(core::mem::zeroed()) as _, dwsitflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DwmSetIconicThumbnail(hwnd: super::windef::HWND, hbmp: super::windef::HBITMAP, dwsitflags: u32) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmSetIconicThumbnail(hwnd : super::windef::HWND, hbmp : super::windef::HBITMAP, dwsitflags : u32) -> windows_core::HRESULT);
    unsafe { DwmSetIconicThumbnail(hwnd, hbmp, dwsitflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DwmSetPresentParameters(hwnd: super::windef::HWND, ppresentparams: *mut DWM_PRESENT_PARAMETERS) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmSetPresentParameters(hwnd : super::windef::HWND, ppresentparams : *mut DWM_PRESENT_PARAMETERS) -> windows_core::HRESULT);
    unsafe { DwmSetPresentParameters(hwnd, ppresentparams as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DwmSetWindowAttribute(hwnd: super::windef::HWND, dwattribute: u32, pvattribute: *const core::ffi::c_void, cbattribute: u32) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmSetWindowAttribute(hwnd : super::windef::HWND, dwattribute : u32, pvattribute : *const core::ffi::c_void, cbattribute : u32) -> windows_core::HRESULT);
    unsafe { DwmSetWindowAttribute(hwnd, dwattribute, pvattribute, cbattribute) }
}
#[inline]
pub unsafe fn DwmShowContact(dwpointerid: u32, eshowcontact: DWM_SHOWCONTACT) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmShowContact(dwpointerid : u32, eshowcontact : DWM_SHOWCONTACT) -> windows_core::HRESULT);
    unsafe { DwmShowContact(dwpointerid, eshowcontact) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DwmTetherContact(dwpointerid: u32, fenable: bool, pttether: super::windef::POINT) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmTetherContact(dwpointerid : u32, fenable : windows_core::BOOL, pttether : super::windef::POINT) -> windows_core::HRESULT);
    unsafe { DwmTetherContact(dwpointerid, fenable.into(), core::mem::transmute(pttether)) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DwmTransitionOwnedWindow(hwnd: super::windef::HWND, target: DWMTRANSITION_OWNEDWINDOW_TARGET) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmTransitionOwnedWindow(hwnd : super::windef::HWND, target : DWMTRANSITION_OWNEDWINDOW_TARGET) -> windows_core::HRESULT);
    unsafe { DwmTransitionOwnedWindow(hwnd, target) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DwmUnregisterThumbnail(hthumbnailid: HTHUMBNAIL) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmUnregisterThumbnail(hthumbnailid : HTHUMBNAIL) -> windows_core::HRESULT);
    unsafe { DwmUnregisterThumbnail(hthumbnailid) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn DwmUpdateThumbnailProperties(hthumbnailid: HTHUMBNAIL, ptnproperties: *const DWM_THUMBNAIL_PROPERTIES) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmUpdateThumbnailProperties(hthumbnailid : HTHUMBNAIL, ptnproperties : *const DWM_THUMBNAIL_PROPERTIES) -> windows_core::HRESULT);
    unsafe { DwmUpdateThumbnailProperties(hthumbnailid, ptnproperties) }
}
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
pub const DWMSC_ALL: DWM_SHOWCONTACT = 4294967295;
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
pub const DWM_BB_BLURREGION: u32 = 2;
pub const DWM_BB_ENABLE: u32 = 1;
pub const DWM_BB_TRANSITIONONMAXIMIZED: u32 = 4;
#[repr(C, packed(1))]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct DWM_BLURBEHIND {
    pub dwFlags: u32,
    pub fEnable: windows_core::BOOL,
    pub hRgnBlur: super::minwindef::HRGN,
    pub fTransitionOnMaximized: windows_core::BOOL,
}
pub const DWM_CLOAKED_APP: u32 = 1;
pub const DWM_CLOAKED_INHERITED: u32 = 4;
pub const DWM_CLOAKED_SHELL: u32 = 2;
pub const DWM_EC_DISABLECOMPOSITION: u32 = 0;
pub const DWM_EC_ENABLECOMPOSITION: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DWM_FRAME_COUNT(pub u64);
pub const DWM_FRAME_DURATION_DEFAULT: i32 = -1;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DWM_PRESENT_PARAMETERS {
    pub cbSize: u32,
    pub fQueue: windows_core::BOOL,
    pub cRefreshStart: DWM_FRAME_COUNT,
    pub cBuffer: u32,
    pub fUseSourceRate: windows_core::BOOL,
    pub rateSource: UNSIGNED_RATIO,
    pub cRefreshesPerFrame: u32,
    pub eSampling: DWM_SOURCE_FRAME_SAMPLING,
}
pub type DWM_SHOWCONTACT = u32;
pub const DWM_SIT_DISPLAYFRAME: u32 = 1;
pub type DWM_SOURCE_FRAME_SAMPLING = i32;
pub const DWM_SOURCE_FRAME_SAMPLING_COVERAGE: DWM_SOURCE_FRAME_SAMPLING = 1;
pub const DWM_SOURCE_FRAME_SAMPLING_LAST: DWM_SOURCE_FRAME_SAMPLING = 2;
pub const DWM_SOURCE_FRAME_SAMPLING_POINT: DWM_SOURCE_FRAME_SAMPLING = 0;
pub type DWM_SYSTEMBACKDROP_TYPE = i32;
pub type DWM_TAB_WINDOW_REQUIREMENTS = u32;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DWM_THUMBNAIL_PROPERTIES {
    pub dwFlags: u32,
    pub rcDestination: super::windef::RECT,
    pub rcSource: super::windef::RECT,
    pub opacity: u8,
    pub fVisible: windows_core::BOOL,
    pub fSourceClientAreaOnly: windows_core::BOOL,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DWM_TIMING_INFO {
    pub cbSize: u32,
    pub rateRefresh: UNSIGNED_RATIO,
    pub qpcRefreshPeriod: QPC_TIME,
    pub rateCompose: UNSIGNED_RATIO,
    pub qpcVBlank: QPC_TIME,
    pub cRefresh: DWM_FRAME_COUNT,
    pub cDXRefresh: u32,
    pub qpcCompose: QPC_TIME,
    pub cFrame: DWM_FRAME_COUNT,
    pub cDXPresent: u32,
    pub cRefreshFrame: DWM_FRAME_COUNT,
    pub cFrameSubmitted: DWM_FRAME_COUNT,
    pub cDXPresentSubmitted: u32,
    pub cFrameConfirmed: DWM_FRAME_COUNT,
    pub cDXPresentConfirmed: u32,
    pub cRefreshConfirmed: DWM_FRAME_COUNT,
    pub cDXRefreshConfirmed: u32,
    pub cFramesLate: DWM_FRAME_COUNT,
    pub cFramesOutstanding: u32,
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
    pub cPixelsReceived: u64,
    pub cPixelsDrawn: u64,
    pub cBuffersEmpty: DWM_FRAME_COUNT,
}
pub const DWM_TNP_OPACITY: u32 = 4;
pub const DWM_TNP_RECTDESTINATION: u32 = 1;
pub const DWM_TNP_RECTSOURCE: u32 = 2;
pub const DWM_TNP_SOURCECLIENTAREAONLY: u32 = 16;
pub const DWM_TNP_VISIBLE: u32 = 8;
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
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HTHUMBNAIL(pub super::winnt::HANDLE);
pub type MIL_MATRIX3X2D = MilMatrix3x2D;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MilMatrix3x2D {
    pub S_11: f64,
    pub S_12: f64,
    pub S_21: f64,
    pub S_22: f64,
    pub DX: f64,
    pub DY: f64,
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDWM_BLURBEHIND(pub *mut DWM_BLURBEHIND);
#[cfg(feature = "minwindef")]
impl PDWM_BLURBEHIND {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PDWM_BLURBEHIND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDWM_THUMBNAIL_PROPERTIES(pub *mut DWM_THUMBNAIL_PROPERTIES);
#[cfg(feature = "windef")]
impl PDWM_THUMBNAIL_PROPERTIES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for PDWM_THUMBNAIL_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHTHUMBNAIL(pub *mut HTHUMBNAIL);
#[cfg(feature = "winnt")]
impl PHTHUMBNAIL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PHTHUMBNAIL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct QPC_TIME(pub u64);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct UNSIGNED_RATIO {
    pub uiNumerator: u32,
    pub uiDenominator: u32,
}
