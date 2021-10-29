#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DWMFLIP3DWINDOWPOLICY(pub i32);
pub const DWMFLIP3D_DEFAULT: DWMFLIP3DWINDOWPOLICY = DWMFLIP3DWINDOWPOLICY(0i32);
pub const DWMFLIP3D_EXCLUDEBELOW: DWMFLIP3DWINDOWPOLICY = DWMFLIP3DWINDOWPOLICY(1i32);
pub const DWMFLIP3D_EXCLUDEABOVE: DWMFLIP3DWINDOWPOLICY = DWMFLIP3DWINDOWPOLICY(2i32);
pub const DWMFLIP3D_LAST: DWMFLIP3DWINDOWPOLICY = DWMFLIP3DWINDOWPOLICY(3i32);
impl ::std::convert::From<i32> for DWMFLIP3DWINDOWPOLICY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DWMFLIP3DWINDOWPOLICY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DWMNCRENDERINGPOLICY(pub i32);
pub const DWMNCRP_USEWINDOWSTYLE: DWMNCRENDERINGPOLICY = DWMNCRENDERINGPOLICY(0i32);
pub const DWMNCRP_DISABLED: DWMNCRENDERINGPOLICY = DWMNCRENDERINGPOLICY(1i32);
pub const DWMNCRP_ENABLED: DWMNCRENDERINGPOLICY = DWMNCRENDERINGPOLICY(2i32);
pub const DWMNCRP_LAST: DWMNCRENDERINGPOLICY = DWMNCRENDERINGPOLICY(3i32);
impl ::std::convert::From<i32> for DWMNCRENDERINGPOLICY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DWMNCRENDERINGPOLICY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DWMTRANSITION_OWNEDWINDOW_TARGET(pub i32);
pub const DWMTRANSITION_OWNEDWINDOW_NULL: DWMTRANSITION_OWNEDWINDOW_TARGET =
    DWMTRANSITION_OWNEDWINDOW_TARGET(-1i32);
pub const DWMTRANSITION_OWNEDWINDOW_REPOSITION: DWMTRANSITION_OWNEDWINDOW_TARGET =
    DWMTRANSITION_OWNEDWINDOW_TARGET(0i32);
impl ::std::convert::From<i32> for DWMTRANSITION_OWNEDWINDOW_TARGET {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DWMTRANSITION_OWNEDWINDOW_TARGET {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DWMWA_COLOR_DEFAULT: u32 = 4294967295u32;
pub const DWMWA_COLOR_NONE: u32 = 4294967294u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
impl ::std::convert::From<i32> for DWMWINDOWATTRIBUTE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DWMWINDOWATTRIBUTE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DWM_BB_BLURREGION: u32 = 2u32;
pub const DWM_BB_ENABLE: u32 = 1u32;
pub const DWM_BB_TRANSITIONONMAXIMIZED: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DWM_BLURBEHIND {
    pub dwFlags: u32,
    pub fEnable: super::super::Foundation::BOOL,
    pub hRgnBlur: super::Gdi::HRGN,
    pub fTransitionOnMaximized: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl DWM_BLURBEHIND {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for DWM_BLURBEHIND {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for DWM_BLURBEHIND {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for DWM_BLURBEHIND {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for DWM_BLURBEHIND {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DWM_CLOAKED_APP: u32 = 1u32;
pub const DWM_CLOAKED_INHERITED: u32 = 4u32;
pub const DWM_CLOAKED_SHELL: u32 = 2u32;
pub const DWM_EC_DISABLECOMPOSITION: u32 = 0u32;
pub const DWM_EC_ENABLECOMPOSITION: u32 = 1u32;
pub const DWM_FRAME_DURATION_DEFAULT: i32 = -1i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl DWM_PRESENT_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DWM_PRESENT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DWM_PRESENT_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DWM_PRESENT_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DWM_PRESENT_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DWM_SHOWCONTACT(pub u32);
pub const DWMSC_DOWN: DWM_SHOWCONTACT = DWM_SHOWCONTACT(1u32);
pub const DWMSC_UP: DWM_SHOWCONTACT = DWM_SHOWCONTACT(2u32);
pub const DWMSC_DRAG: DWM_SHOWCONTACT = DWM_SHOWCONTACT(4u32);
pub const DWMSC_HOLD: DWM_SHOWCONTACT = DWM_SHOWCONTACT(8u32);
pub const DWMSC_PENBARREL: DWM_SHOWCONTACT = DWM_SHOWCONTACT(16u32);
pub const DWMSC_NONE: DWM_SHOWCONTACT = DWM_SHOWCONTACT(0u32);
pub const DWMSC_ALL: DWM_SHOWCONTACT = DWM_SHOWCONTACT(4294967295u32);
impl ::std::convert::From<u32> for DWM_SHOWCONTACT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DWM_SHOWCONTACT {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DWM_SHOWCONTACT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DWM_SHOWCONTACT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DWM_SHOWCONTACT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DWM_SHOWCONTACT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DWM_SHOWCONTACT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DWM_SIT_DISPLAYFRAME: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DWM_SOURCE_FRAME_SAMPLING(pub i32);
pub const DWM_SOURCE_FRAME_SAMPLING_POINT: DWM_SOURCE_FRAME_SAMPLING =
    DWM_SOURCE_FRAME_SAMPLING(0i32);
pub const DWM_SOURCE_FRAME_SAMPLING_COVERAGE: DWM_SOURCE_FRAME_SAMPLING =
    DWM_SOURCE_FRAME_SAMPLING(1i32);
pub const DWM_SOURCE_FRAME_SAMPLING_LAST: DWM_SOURCE_FRAME_SAMPLING =
    DWM_SOURCE_FRAME_SAMPLING(2i32);
impl ::std::convert::From<i32> for DWM_SOURCE_FRAME_SAMPLING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DWM_SOURCE_FRAME_SAMPLING {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DWM_TAB_WINDOW_REQUIREMENTS(pub u32);
pub const DWMTWR_NONE: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(0u32);
pub const DWMTWR_IMPLEMENTED_BY_SYSTEM: DWM_TAB_WINDOW_REQUIREMENTS =
    DWM_TAB_WINDOW_REQUIREMENTS(1u32);
pub const DWMTWR_WINDOW_RELATIONSHIP: DWM_TAB_WINDOW_REQUIREMENTS =
    DWM_TAB_WINDOW_REQUIREMENTS(2u32);
pub const DWMTWR_WINDOW_STYLES: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(4u32);
pub const DWMTWR_WINDOW_REGION: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(8u32);
pub const DWMTWR_WINDOW_DWM_ATTRIBUTES: DWM_TAB_WINDOW_REQUIREMENTS =
    DWM_TAB_WINDOW_REQUIREMENTS(16u32);
pub const DWMTWR_WINDOW_MARGINS: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(32u32);
pub const DWMTWR_TABBING_ENABLED: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(64u32);
pub const DWMTWR_USER_POLICY: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(128u32);
pub const DWMTWR_GROUP_POLICY: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(256u32);
pub const DWMTWR_APP_COMPAT: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(512u32);
impl ::std::convert::From<u32> for DWM_TAB_WINDOW_REQUIREMENTS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DWM_TAB_WINDOW_REQUIREMENTS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DWM_TAB_WINDOW_REQUIREMENTS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DWM_TAB_WINDOW_REQUIREMENTS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DWM_TAB_WINDOW_REQUIREMENTS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DWM_TAB_WINDOW_REQUIREMENTS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DWM_TAB_WINDOW_REQUIREMENTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl DWM_THUMBNAIL_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DWM_THUMBNAIL_PROPERTIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DWM_THUMBNAIL_PROPERTIES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DWM_THUMBNAIL_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DWM_THUMBNAIL_PROPERTIES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl DWM_TIMING_INFO {}
impl ::std::default::Default for DWM_TIMING_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DWM_TIMING_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DWM_TIMING_INFO {}
unsafe impl ::windows::runtime::Abi for DWM_TIMING_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DWM_TNP_OPACITY: u32 = 4u32;
pub const DWM_TNP_RECTDESTINATION: u32 = 1u32;
pub const DWM_TNP_RECTSOURCE: u32 = 2u32;
pub const DWM_TNP_SOURCECLIENTAREAONLY: u32 = 16u32;
pub const DWM_TNP_VISIBLE: u32 = 8u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DWM_WINDOW_CORNER_PREFERENCE(pub i32);
pub const DWMWCP_DEFAULT: DWM_WINDOW_CORNER_PREFERENCE = DWM_WINDOW_CORNER_PREFERENCE(0i32);
pub const DWMWCP_DONOTROUND: DWM_WINDOW_CORNER_PREFERENCE = DWM_WINDOW_CORNER_PREFERENCE(1i32);
pub const DWMWCP_ROUND: DWM_WINDOW_CORNER_PREFERENCE = DWM_WINDOW_CORNER_PREFERENCE(2i32);
pub const DWMWCP_ROUNDSMALL: DWM_WINDOW_CORNER_PREFERENCE = DWM_WINDOW_CORNER_PREFERENCE(3i32);
impl ::std::convert::From<i32> for DWM_WINDOW_CORNER_PREFERENCE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DWM_WINDOW_CORNER_PREFERENCE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmAttachMilContent<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmAttachMilContent(
                hwnd: super::super::Foundation::HWND,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmAttachMilContent(hwnd.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmDefWindowProc<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    hwnd: Param0,
    msg: u32,
    wparam: Param2,
    lparam: Param3,
    plresult: *mut super::super::Foundation::LRESULT,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmDefWindowProc(
                hwnd: super::super::Foundation::HWND,
                msg: u32,
                wparam: super::super::Foundation::WPARAM,
                lparam: super::super::Foundation::LPARAM,
                plresult: *mut super::super::Foundation::LRESULT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DwmDefWindowProc(
            hwnd.into_param().abi(),
            ::std::mem::transmute(msg),
            wparam.into_param().abi(),
            lparam.into_param().abi(),
            ::std::mem::transmute(plresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmDetachMilContent<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmDetachMilContent(
                hwnd: super::super::Foundation::HWND,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmDetachMilContent(hwnd.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DwmEnableBlurBehindWindow<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    pblurbehind: *const DWM_BLURBEHIND,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmEnableBlurBehindWindow(
                hwnd: super::super::Foundation::HWND,
                pblurbehind: *const DWM_BLURBEHIND,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmEnableBlurBehindWindow(hwnd.into_param().abi(), ::std::mem::transmute(pblurbehind)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DwmEnableComposition(ucompositionaction: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmEnableComposition(ucompositionaction: u32) -> ::windows::runtime::HRESULT;
        }
        DwmEnableComposition(::std::mem::transmute(ucompositionaction)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmEnableMMCSS<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    fenablemmcss: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmEnableMMCSS(
                fenablemmcss: super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmEnableMMCSS(fenablemmcss.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn DwmExtendFrameIntoClientArea<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    pmarinset: *const super::super::UI::Controls::MARGINS,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmExtendFrameIntoClientArea(
                hwnd: super::super::Foundation::HWND,
                pmarinset: *const super::super::UI::Controls::MARGINS,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmExtendFrameIntoClientArea(hwnd.into_param().abi(), ::std::mem::transmute(pmarinset)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DwmFlush() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmFlush() -> ::windows::runtime::HRESULT;
        }
        DwmFlush().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmGetColorizationColor(
    pcrcolorization: *mut u32,
    pfopaqueblend: *mut super::super::Foundation::BOOL,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmGetColorizationColor(
                pcrcolorization: *mut u32,
                pfopaqueblend: *mut super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmGetColorizationColor(
            ::std::mem::transmute(pcrcolorization),
            ::std::mem::transmute(pfopaqueblend),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmGetCompositionTimingInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
) -> ::windows::runtime::Result<DWM_TIMING_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmGetCompositionTimingInfo(
                hwnd: super::super::Foundation::HWND,
                ptiminginfo: *mut DWM_TIMING_INFO,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DWM_TIMING_INFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DwmGetCompositionTimingInfo(hwnd.into_param().abi(), &mut result__)
            .from_abi::<DWM_TIMING_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DwmGetGraphicsStreamClient(
    uindex: u32,
) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmGetGraphicsStreamClient(
                uindex: u32,
                pclientuuid: *mut ::windows::runtime::GUID,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DwmGetGraphicsStreamClient(::std::mem::transmute(uindex), &mut result__)
            .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DwmGetGraphicsStreamTransformHint(
    uindex: u32,
) -> ::windows::runtime::Result<MilMatrix3x2D> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmGetGraphicsStreamTransformHint(
                uindex: u32,
                ptransform: *mut MilMatrix3x2D,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <MilMatrix3x2D as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DwmGetGraphicsStreamTransformHint(::std::mem::transmute(uindex), &mut result__)
            .from_abi::<MilMatrix3x2D>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmGetTransportAttributes(
    pfisremoting: *mut super::super::Foundation::BOOL,
    pfisconnected: *mut super::super::Foundation::BOOL,
    pdwgeneration: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmGetTransportAttributes(
                pfisremoting: *mut super::super::Foundation::BOOL,
                pfisconnected: *mut super::super::Foundation::BOOL,
                pdwgeneration: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmGetTransportAttributes(
            ::std::mem::transmute(pfisremoting),
            ::std::mem::transmute(pfisconnected),
            ::std::mem::transmute(pdwgeneration),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmGetUnmetTabRequirements<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    appwindow: Param0,
) -> ::windows::runtime::Result<DWM_TAB_WINDOW_REQUIREMENTS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmGetUnmetTabRequirements(
                appwindow: super::super::Foundation::HWND,
                value: *mut DWM_TAB_WINDOW_REQUIREMENTS,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DWM_TAB_WINDOW_REQUIREMENTS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DwmGetUnmetTabRequirements(appwindow.into_param().abi(), &mut result__)
            .from_abi::<DWM_TAB_WINDOW_REQUIREMENTS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmGetWindowAttribute<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    dwattribute: DWMWINDOWATTRIBUTE,
    pvattribute: *mut ::std::ffi::c_void,
    cbattribute: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmGetWindowAttribute(
                hwnd: super::super::Foundation::HWND,
                dwattribute: DWMWINDOWATTRIBUTE,
                pvattribute: *mut ::std::ffi::c_void,
                cbattribute: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmGetWindowAttribute(
            hwnd.into_param().abi(),
            ::std::mem::transmute(dwattribute),
            ::std::mem::transmute(pvattribute),
            ::std::mem::transmute(cbattribute),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmInvalidateIconicBitmaps<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmInvalidateIconicBitmaps(
                hwnd: super::super::Foundation::HWND,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmInvalidateIconicBitmaps(hwnd.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmIsCompositionEnabled() -> ::windows::runtime::Result<super::super::Foundation::BOOL>
{
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmIsCompositionEnabled(
                pfenabled: *mut super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DwmIsCompositionEnabled(&mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmModifyPreviousDxFrameDuration<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hwnd: Param0,
    crefreshes: i32,
    frelative: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmModifyPreviousDxFrameDuration(
                hwnd: super::super::Foundation::HWND,
                crefreshes: i32,
                frelative: super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmModifyPreviousDxFrameDuration(
            hwnd.into_param().abi(),
            ::std::mem::transmute(crefreshes),
            frelative.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmQueryThumbnailSourceSize(
    hthumbnail: isize,
) -> ::windows::runtime::Result<super::super::Foundation::SIZE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmQueryThumbnailSourceSize(
                hthumbnail: isize,
                psize: *mut super::super::Foundation::SIZE,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::SIZE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DwmQueryThumbnailSourceSize(::std::mem::transmute(hthumbnail), &mut result__)
            .from_abi::<super::super::Foundation::SIZE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmRegisterThumbnail<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnddestination: Param0,
    hwndsource: Param1,
) -> ::windows::runtime::Result<isize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmRegisterThumbnail(
                hwnddestination: super::super::Foundation::HWND,
                hwndsource: super::super::Foundation::HWND,
                phthumbnailid: *mut isize,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <isize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DwmRegisterThumbnail(
            hwnddestination.into_param().abi(),
            hwndsource.into_param().abi(),
            &mut result__,
        )
        .from_abi::<isize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmRenderGesture(
    gt: GESTURE_TYPE,
    ccontacts: u32,
    pdwpointerid: *const u32,
    ppoints: *const super::super::Foundation::POINT,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmRenderGesture(
                gt: GESTURE_TYPE,
                ccontacts: u32,
                pdwpointerid: *const u32,
                ppoints: *const super::super::Foundation::POINT,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmRenderGesture(
            ::std::mem::transmute(gt),
            ::std::mem::transmute(ccontacts),
            ::std::mem::transmute(pdwpointerid),
            ::std::mem::transmute(ppoints),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmSetDxFrameDuration<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    crefreshes: i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmSetDxFrameDuration(
                hwnd: super::super::Foundation::HWND,
                crefreshes: i32,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmSetDxFrameDuration(hwnd.into_param().abi(), ::std::mem::transmute(crefreshes)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DwmSetIconicLivePreviewBitmap<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param1: ::windows::runtime::IntoParam<'a, super::Gdi::HBITMAP>,
>(
    hwnd: Param0,
    hbmp: Param1,
    pptclient: *const super::super::Foundation::POINT,
    dwsitflags: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmSetIconicLivePreviewBitmap(
                hwnd: super::super::Foundation::HWND,
                hbmp: super::Gdi::HBITMAP,
                pptclient: *const super::super::Foundation::POINT,
                dwsitflags: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmSetIconicLivePreviewBitmap(
            hwnd.into_param().abi(),
            hbmp.into_param().abi(),
            ::std::mem::transmute(pptclient),
            ::std::mem::transmute(dwsitflags),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DwmSetIconicThumbnail<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param1: ::windows::runtime::IntoParam<'a, super::Gdi::HBITMAP>,
>(
    hwnd: Param0,
    hbmp: Param1,
    dwsitflags: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmSetIconicThumbnail(
                hwnd: super::super::Foundation::HWND,
                hbmp: super::Gdi::HBITMAP,
                dwsitflags: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmSetIconicThumbnail(
            hwnd.into_param().abi(),
            hbmp.into_param().abi(),
            ::std::mem::transmute(dwsitflags),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmSetPresentParameters<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    ppresentparams: *mut DWM_PRESENT_PARAMETERS,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmSetPresentParameters(
                hwnd: super::super::Foundation::HWND,
                ppresentparams: *mut DWM_PRESENT_PARAMETERS,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmSetPresentParameters(
            hwnd.into_param().abi(),
            ::std::mem::transmute(ppresentparams),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmSetWindowAttribute<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    dwattribute: DWMWINDOWATTRIBUTE,
    pvattribute: *const ::std::ffi::c_void,
    cbattribute: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmSetWindowAttribute(
                hwnd: super::super::Foundation::HWND,
                dwattribute: DWMWINDOWATTRIBUTE,
                pvattribute: *const ::std::ffi::c_void,
                cbattribute: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmSetWindowAttribute(
            hwnd.into_param().abi(),
            ::std::mem::transmute(dwattribute),
            ::std::mem::transmute(pvattribute),
            ::std::mem::transmute(cbattribute),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DwmShowContact(
    dwpointerid: u32,
    eshowcontact: DWM_SHOWCONTACT,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmShowContact(
                dwpointerid: u32,
                eshowcontact: DWM_SHOWCONTACT,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmShowContact(
            ::std::mem::transmute(dwpointerid),
            ::std::mem::transmute(eshowcontact),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmTetherContact<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::POINT>,
>(
    dwpointerid: u32,
    fenable: Param1,
    pttether: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmTetherContact(
                dwpointerid: u32,
                fenable: super::super::Foundation::BOOL,
                pttether: super::super::Foundation::POINT,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmTetherContact(
            ::std::mem::transmute(dwpointerid),
            fenable.into_param().abi(),
            pttether.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmTransitionOwnedWindow<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    target: DWMTRANSITION_OWNEDWINDOW_TARGET,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmTransitionOwnedWindow(
                hwnd: super::super::Foundation::HWND,
                target: DWMTRANSITION_OWNEDWINDOW_TARGET,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmTransitionOwnedWindow(hwnd.into_param().abi(), ::std::mem::transmute(target)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DwmUnregisterThumbnail(hthumbnailid: isize) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmUnregisterThumbnail(hthumbnailid: isize) -> ::windows::runtime::HRESULT;
        }
        DwmUnregisterThumbnail(::std::mem::transmute(hthumbnailid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmUpdateThumbnailProperties(
    hthumbnailid: isize,
    ptnproperties: *const DWM_THUMBNAIL_PROPERTIES,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DwmUpdateThumbnailProperties(
                hthumbnailid: isize,
                ptnproperties: *const DWM_THUMBNAIL_PROPERTIES,
            ) -> ::windows::runtime::HRESULT;
        }
        DwmUpdateThumbnailProperties(
            ::std::mem::transmute(hthumbnailid),
            ::std::mem::transmute(ptnproperties),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
impl ::std::convert::From<i32> for GESTURE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GESTURE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MilMatrix3x2D {
    pub S_11: f64,
    pub S_12: f64,
    pub S_21: f64,
    pub S_22: f64,
    pub DX: f64,
    pub DY: f64,
}
impl MilMatrix3x2D {}
impl ::std::default::Default for MilMatrix3x2D {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MilMatrix3x2D {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MilMatrix3x2D {}
unsafe impl ::windows::runtime::Abi for MilMatrix3x2D {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct UNSIGNED_RATIO {
    pub uiNumerator: u32,
    pub uiDenominator: u32,
}
impl UNSIGNED_RATIO {}
impl ::std::default::Default for UNSIGNED_RATIO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for UNSIGNED_RATIO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for UNSIGNED_RATIO {}
unsafe impl ::windows::runtime::Abi for UNSIGNED_RATIO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const c_DwmMaxAdapters: u32 = 16u32;
pub const c_DwmMaxMonitors: u32 = 16u32;
pub const c_DwmMaxQueuedBuffers: u32 = 8u32;
