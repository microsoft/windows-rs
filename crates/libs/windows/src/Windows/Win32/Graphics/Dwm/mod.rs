#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWMFLIP3DWINDOWPOLICY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMFLIP3D_DEFAULT: DWMFLIP3DWINDOWPOLICY = DWMFLIP3DWINDOWPOLICY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMFLIP3D_EXCLUDEBELOW: DWMFLIP3DWINDOWPOLICY = DWMFLIP3DWINDOWPOLICY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMFLIP3D_EXCLUDEABOVE: DWMFLIP3DWINDOWPOLICY = DWMFLIP3DWINDOWPOLICY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMFLIP3D_LAST: DWMFLIP3DWINDOWPOLICY = DWMFLIP3DWINDOWPOLICY(3i32);
impl ::core::marker::Copy for DWMFLIP3DWINDOWPOLICY {}
impl ::core::clone::Clone for DWMFLIP3DWINDOWPOLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWMFLIP3DWINDOWPOLICY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DWMFLIP3DWINDOWPOLICY {
    type Abi = Self;
}
impl ::core::fmt::Debug for DWMFLIP3DWINDOWPOLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWMFLIP3DWINDOWPOLICY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWMNCRENDERINGPOLICY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMNCRP_USEWINDOWSTYLE: DWMNCRENDERINGPOLICY = DWMNCRENDERINGPOLICY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMNCRP_DISABLED: DWMNCRENDERINGPOLICY = DWMNCRENDERINGPOLICY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMNCRP_ENABLED: DWMNCRENDERINGPOLICY = DWMNCRENDERINGPOLICY(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMNCRP_LAST: DWMNCRENDERINGPOLICY = DWMNCRENDERINGPOLICY(3i32);
impl ::core::marker::Copy for DWMNCRENDERINGPOLICY {}
impl ::core::clone::Clone for DWMNCRENDERINGPOLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWMNCRENDERINGPOLICY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DWMNCRENDERINGPOLICY {
    type Abi = Self;
}
impl ::core::fmt::Debug for DWMNCRENDERINGPOLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWMNCRENDERINGPOLICY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWMTRANSITION_OWNEDWINDOW_TARGET(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMTRANSITION_OWNEDWINDOW_NULL: DWMTRANSITION_OWNEDWINDOW_TARGET = DWMTRANSITION_OWNEDWINDOW_TARGET(-1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMTRANSITION_OWNEDWINDOW_REPOSITION: DWMTRANSITION_OWNEDWINDOW_TARGET = DWMTRANSITION_OWNEDWINDOW_TARGET(0i32);
impl ::core::marker::Copy for DWMTRANSITION_OWNEDWINDOW_TARGET {}
impl ::core::clone::Clone for DWMTRANSITION_OWNEDWINDOW_TARGET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWMTRANSITION_OWNEDWINDOW_TARGET {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DWMTRANSITION_OWNEDWINDOW_TARGET {
    type Abi = Self;
}
impl ::core::fmt::Debug for DWMTRANSITION_OWNEDWINDOW_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWMTRANSITION_OWNEDWINDOW_TARGET").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_COLOR_DEFAULT: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_COLOR_NONE: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWMWINDOWATTRIBUTE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_NCRENDERING_ENABLED: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_NCRENDERING_POLICY: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_TRANSITIONS_FORCEDISABLED: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_ALLOW_NCPAINT: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_CAPTION_BUTTON_BOUNDS: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_NONCLIENT_RTL_LAYOUT: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_FORCE_ICONIC_REPRESENTATION: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_FLIP3D_POLICY: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_EXTENDED_FRAME_BOUNDS: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_HAS_ICONIC_BITMAP: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_DISALLOW_PEEK: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_EXCLUDED_FROM_PEEK: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_CLOAK: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_CLOAKED: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_FREEZE_REPRESENTATION: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_PASSIVE_UPDATE_MODE: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_USE_HOSTBACKDROPBRUSH: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_USE_IMMERSIVE_DARK_MODE: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_WINDOW_CORNER_PREFERENCE: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(33i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_BORDER_COLOR: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(34i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_CAPTION_COLOR: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(35i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_TEXT_COLOR: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(36i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_VISIBLE_FRAME_BORDER_THICKNESS: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(37i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWA_LAST: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(38i32);
impl ::core::marker::Copy for DWMWINDOWATTRIBUTE {}
impl ::core::clone::Clone for DWMWINDOWATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWMWINDOWATTRIBUTE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DWMWINDOWATTRIBUTE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DWMWINDOWATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWMWINDOWATTRIBUTE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_BB_BLURREGION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_BB_ENABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_BB_TRANSITIONONMAXIMIZED: u32 = 4u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for DWM_BLURBEHIND {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DWM_BLURBEHIND {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DWM_BLURBEHIND>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DWM_BLURBEHIND {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DWM_BLURBEHIND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_CLOAKED_APP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_CLOAKED_INHERITED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_CLOAKED_SHELL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_EC_DISABLECOMPOSITION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_EC_ENABLECOMPOSITION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_FRAME_DURATION_DEFAULT: i32 = -1i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DWM_PRESENT_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWM_PRESENT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DWM_PRESENT_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWM_PRESENT_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWM_PRESENT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWM_SHOWCONTACT(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMSC_DOWN: DWM_SHOWCONTACT = DWM_SHOWCONTACT(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMSC_UP: DWM_SHOWCONTACT = DWM_SHOWCONTACT(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMSC_DRAG: DWM_SHOWCONTACT = DWM_SHOWCONTACT(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMSC_HOLD: DWM_SHOWCONTACT = DWM_SHOWCONTACT(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMSC_PENBARREL: DWM_SHOWCONTACT = DWM_SHOWCONTACT(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMSC_NONE: DWM_SHOWCONTACT = DWM_SHOWCONTACT(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMSC_ALL: DWM_SHOWCONTACT = DWM_SHOWCONTACT(4294967295u32);
impl ::core::marker::Copy for DWM_SHOWCONTACT {}
impl ::core::clone::Clone for DWM_SHOWCONTACT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWM_SHOWCONTACT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DWM_SHOWCONTACT {
    type Abi = Self;
}
impl ::core::fmt::Debug for DWM_SHOWCONTACT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWM_SHOWCONTACT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DWM_SHOWCONTACT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DWM_SHOWCONTACT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DWM_SHOWCONTACT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DWM_SHOWCONTACT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DWM_SHOWCONTACT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_SIT_DISPLAYFRAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWM_SOURCE_FRAME_SAMPLING(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_SOURCE_FRAME_SAMPLING_POINT: DWM_SOURCE_FRAME_SAMPLING = DWM_SOURCE_FRAME_SAMPLING(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_SOURCE_FRAME_SAMPLING_COVERAGE: DWM_SOURCE_FRAME_SAMPLING = DWM_SOURCE_FRAME_SAMPLING(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_SOURCE_FRAME_SAMPLING_LAST: DWM_SOURCE_FRAME_SAMPLING = DWM_SOURCE_FRAME_SAMPLING(2i32);
impl ::core::marker::Copy for DWM_SOURCE_FRAME_SAMPLING {}
impl ::core::clone::Clone for DWM_SOURCE_FRAME_SAMPLING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWM_SOURCE_FRAME_SAMPLING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DWM_SOURCE_FRAME_SAMPLING {
    type Abi = Self;
}
impl ::core::fmt::Debug for DWM_SOURCE_FRAME_SAMPLING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWM_SOURCE_FRAME_SAMPLING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWM_TAB_WINDOW_REQUIREMENTS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMTWR_NONE: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMTWR_IMPLEMENTED_BY_SYSTEM: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMTWR_WINDOW_RELATIONSHIP: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMTWR_WINDOW_STYLES: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMTWR_WINDOW_REGION: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMTWR_WINDOW_DWM_ATTRIBUTES: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(16u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMTWR_WINDOW_MARGINS: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(32u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMTWR_TABBING_ENABLED: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(64u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMTWR_USER_POLICY: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(128u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMTWR_GROUP_POLICY: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(256u32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMTWR_APP_COMPAT: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(512u32);
impl ::core::marker::Copy for DWM_TAB_WINDOW_REQUIREMENTS {}
impl ::core::clone::Clone for DWM_TAB_WINDOW_REQUIREMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWM_TAB_WINDOW_REQUIREMENTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DWM_TAB_WINDOW_REQUIREMENTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DWM_TAB_WINDOW_REQUIREMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWM_TAB_WINDOW_REQUIREMENTS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DWM_TAB_WINDOW_REQUIREMENTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DWM_TAB_WINDOW_REQUIREMENTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DWM_TAB_WINDOW_REQUIREMENTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DWM_TAB_WINDOW_REQUIREMENTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DWM_TAB_WINDOW_REQUIREMENTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DWM_THUMBNAIL_PROPERTIES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWM_THUMBNAIL_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DWM_THUMBNAIL_PROPERTIES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWM_THUMBNAIL_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWM_THUMBNAIL_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
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
unsafe impl ::windows::core::Abi for DWM_TIMING_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DWM_TIMING_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DWM_TIMING_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DWM_TIMING_INFO {}
impl ::core::default::Default for DWM_TIMING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_TNP_OPACITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_TNP_RECTDESTINATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_TNP_RECTSOURCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_TNP_SOURCECLIENTAREAONLY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWM_TNP_VISIBLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWM_WINDOW_CORNER_PREFERENCE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWCP_DEFAULT: DWM_WINDOW_CORNER_PREFERENCE = DWM_WINDOW_CORNER_PREFERENCE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWCP_DONOTROUND: DWM_WINDOW_CORNER_PREFERENCE = DWM_WINDOW_CORNER_PREFERENCE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWCP_ROUND: DWM_WINDOW_CORNER_PREFERENCE = DWM_WINDOW_CORNER_PREFERENCE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const DWMWCP_ROUNDSMALL: DWM_WINDOW_CORNER_PREFERENCE = DWM_WINDOW_CORNER_PREFERENCE(3i32);
impl ::core::marker::Copy for DWM_WINDOW_CORNER_PREFERENCE {}
impl ::core::clone::Clone for DWM_WINDOW_CORNER_PREFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWM_WINDOW_CORNER_PREFERENCE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DWM_WINDOW_CORNER_PREFERENCE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DWM_WINDOW_CORNER_PREFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWM_WINDOW_CORNER_PREFERENCE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmAttachMilContent<'a, P0>(hwnd: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmAttachMilContent(hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT;
    }
    DwmAttachMilContent(hwnd.into()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmDefWindowProc<'a, P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2, plresult: *mut super::super::Foundation::LRESULT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmDefWindowProc(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> super::super::Foundation::BOOL;
    }
    DwmDefWindowProc(hwnd.into(), msg, wparam.into(), lparam.into(), ::core::mem::transmute(plresult))
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmDetachMilContent<'a, P0>(hwnd: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmDetachMilContent(hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT;
    }
    DwmDetachMilContent(hwnd.into()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DwmEnableBlurBehindWindow<'a, P0>(hwnd: P0, pblurbehind: *const DWM_BLURBEHIND) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmEnableBlurBehindWindow(hwnd: super::super::Foundation::HWND, pblurbehind: *const DWM_BLURBEHIND) -> ::windows::core::HRESULT;
    }
    DwmEnableBlurBehindWindow(hwnd.into(), ::core::mem::transmute(pblurbehind)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
#[inline]
pub unsafe fn DwmEnableComposition(ucompositionaction: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmEnableComposition(ucompositionaction: u32) -> ::windows::core::HRESULT;
    }
    DwmEnableComposition(ucompositionaction).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmEnableMMCSS<'a, P0>(fenablemmcss: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmEnableMMCSS(fenablemmcss: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    DwmEnableMMCSS(fenablemmcss.into()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn DwmExtendFrameIntoClientArea<'a, P0>(hwnd: P0, pmarinset: *const super::super::UI::Controls::MARGINS) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmExtendFrameIntoClientArea(hwnd: super::super::Foundation::HWND, pmarinset: *const super::super::UI::Controls::MARGINS) -> ::windows::core::HRESULT;
    }
    DwmExtendFrameIntoClientArea(hwnd.into(), ::core::mem::transmute(pmarinset)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
#[inline]
pub unsafe fn DwmFlush() -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmFlush() -> ::windows::core::HRESULT;
    }
    DwmFlush().ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmGetColorizationColor(pcrcolorization: *mut u32, pfopaqueblend: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmGetColorizationColor(pcrcolorization: *mut u32, pfopaqueblend: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    DwmGetColorizationColor(::core::mem::transmute(pcrcolorization), ::core::mem::transmute(pfopaqueblend)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmGetCompositionTimingInfo<'a, P0>(hwnd: P0) -> ::windows::core::Result<DWM_TIMING_INFO>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmGetCompositionTimingInfo(hwnd: super::super::Foundation::HWND, ptiminginfo: *mut DWM_TIMING_INFO) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DwmGetCompositionTimingInfo(hwnd.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DWM_TIMING_INFO>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
#[inline]
pub unsafe fn DwmGetGraphicsStreamClient(uindex: u32) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmGetGraphicsStreamClient(uindex: u32, pclientuuid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DwmGetGraphicsStreamClient(uindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
#[inline]
pub unsafe fn DwmGetGraphicsStreamTransformHint(uindex: u32) -> ::windows::core::Result<MilMatrix3x2D> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmGetGraphicsStreamTransformHint(uindex: u32, ptransform: *mut MilMatrix3x2D) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DwmGetGraphicsStreamTransformHint(uindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<MilMatrix3x2D>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmGetTransportAttributes(pfisremoting: *mut super::super::Foundation::BOOL, pfisconnected: *mut super::super::Foundation::BOOL, pdwgeneration: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmGetTransportAttributes(pfisremoting: *mut super::super::Foundation::BOOL, pfisconnected: *mut super::super::Foundation::BOOL, pdwgeneration: *mut u32) -> ::windows::core::HRESULT;
    }
    DwmGetTransportAttributes(::core::mem::transmute(pfisremoting), ::core::mem::transmute(pfisconnected), ::core::mem::transmute(pdwgeneration)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmGetUnmetTabRequirements<'a, P0>(appwindow: P0) -> ::windows::core::Result<DWM_TAB_WINDOW_REQUIREMENTS>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmGetUnmetTabRequirements(appwindow: super::super::Foundation::HWND, value: *mut DWM_TAB_WINDOW_REQUIREMENTS) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DwmGetUnmetTabRequirements(appwindow.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DWM_TAB_WINDOW_REQUIREMENTS>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmGetWindowAttribute<'a, P0>(hwnd: P0, dwattribute: DWMWINDOWATTRIBUTE, pvattribute: *mut ::core::ffi::c_void, cbattribute: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmGetWindowAttribute(hwnd: super::super::Foundation::HWND, dwattribute: DWMWINDOWATTRIBUTE, pvattribute: *mut ::core::ffi::c_void, cbattribute: u32) -> ::windows::core::HRESULT;
    }
    DwmGetWindowAttribute(hwnd.into(), dwattribute, ::core::mem::transmute(pvattribute), cbattribute).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmInvalidateIconicBitmaps<'a, P0>(hwnd: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmInvalidateIconicBitmaps(hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT;
    }
    DwmInvalidateIconicBitmaps(hwnd.into()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmIsCompositionEnabled() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmIsCompositionEnabled(pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DwmIsCompositionEnabled(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmModifyPreviousDxFrameDuration<'a, P0, P1>(hwnd: P0, crefreshes: i32, frelative: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmModifyPreviousDxFrameDuration(hwnd: super::super::Foundation::HWND, crefreshes: i32, frelative: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    DwmModifyPreviousDxFrameDuration(hwnd.into(), crefreshes, frelative.into()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmQueryThumbnailSourceSize(hthumbnail: isize) -> ::windows::core::Result<super::super::Foundation::SIZE> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmQueryThumbnailSourceSize(hthumbnail: isize, psize: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DwmQueryThumbnailSourceSize(hthumbnail, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SIZE>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmRegisterThumbnail<'a, P0, P1>(hwnddestination: P0, hwndsource: P1) -> ::windows::core::Result<isize>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmRegisterThumbnail(hwnddestination: super::super::Foundation::HWND, hwndsource: super::super::Foundation::HWND, phthumbnailid: *mut isize) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DwmRegisterThumbnail(hwnddestination.into(), hwndsource.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmRenderGesture(gt: GESTURE_TYPE, ccontacts: u32, pdwpointerid: *const u32, ppoints: *const super::super::Foundation::POINT) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmRenderGesture(gt: GESTURE_TYPE, ccontacts: u32, pdwpointerid: *const u32, ppoints: *const super::super::Foundation::POINT) -> ::windows::core::HRESULT;
    }
    DwmRenderGesture(gt, ::core::mem::transmute(ccontacts), ::core::mem::transmute(pdwpointerid), ::core::mem::transmute(ppoints)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmSetDxFrameDuration<'a, P0>(hwnd: P0, crefreshes: i32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmSetDxFrameDuration(hwnd: super::super::Foundation::HWND, crefreshes: i32) -> ::windows::core::HRESULT;
    }
    DwmSetDxFrameDuration(hwnd.into(), crefreshes).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DwmSetIconicLivePreviewBitmap<'a, P0, P1>(hwnd: P0, hbmp: P1, pptclient: *const super::super::Foundation::POINT, dwsitflags: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::Gdi::HBITMAP>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmSetIconicLivePreviewBitmap(hwnd: super::super::Foundation::HWND, hbmp: super::Gdi::HBITMAP, pptclient: *const super::super::Foundation::POINT, dwsitflags: u32) -> ::windows::core::HRESULT;
    }
    DwmSetIconicLivePreviewBitmap(hwnd.into(), hbmp.into(), ::core::mem::transmute(pptclient), dwsitflags).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DwmSetIconicThumbnail<'a, P0, P1>(hwnd: P0, hbmp: P1, dwsitflags: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::Gdi::HBITMAP>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmSetIconicThumbnail(hwnd: super::super::Foundation::HWND, hbmp: super::Gdi::HBITMAP, dwsitflags: u32) -> ::windows::core::HRESULT;
    }
    DwmSetIconicThumbnail(hwnd.into(), hbmp.into(), dwsitflags).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmSetPresentParameters<'a, P0>(hwnd: P0, ppresentparams: *mut DWM_PRESENT_PARAMETERS) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmSetPresentParameters(hwnd: super::super::Foundation::HWND, ppresentparams: *mut DWM_PRESENT_PARAMETERS) -> ::windows::core::HRESULT;
    }
    DwmSetPresentParameters(hwnd.into(), ::core::mem::transmute(ppresentparams)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmSetWindowAttribute<'a, P0>(hwnd: P0, dwattribute: DWMWINDOWATTRIBUTE, pvattribute: *const ::core::ffi::c_void, cbattribute: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmSetWindowAttribute(hwnd: super::super::Foundation::HWND, dwattribute: DWMWINDOWATTRIBUTE, pvattribute: *const ::core::ffi::c_void, cbattribute: u32) -> ::windows::core::HRESULT;
    }
    DwmSetWindowAttribute(hwnd.into(), dwattribute, ::core::mem::transmute(pvattribute), cbattribute).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
#[inline]
pub unsafe fn DwmShowContact(dwpointerid: u32, eshowcontact: DWM_SHOWCONTACT) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmShowContact(dwpointerid: u32, eshowcontact: DWM_SHOWCONTACT) -> ::windows::core::HRESULT;
    }
    DwmShowContact(dwpointerid, eshowcontact).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmTetherContact<'a, P0>(dwpointerid: u32, fenable: P0, pttether: super::super::Foundation::POINT) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmTetherContact(dwpointerid: u32, fenable: super::super::Foundation::BOOL, pttether: super::super::Foundation::POINT) -> ::windows::core::HRESULT;
    }
    DwmTetherContact(dwpointerid, fenable.into(), ::core::mem::transmute(pttether)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmTransitionOwnedWindow<'a, P0>(hwnd: P0, target: DWMTRANSITION_OWNEDWINDOW_TARGET) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmTransitionOwnedWindow(hwnd: super::super::Foundation::HWND, target: DWMTRANSITION_OWNEDWINDOW_TARGET) -> ::windows::core::HRESULT;
    }
    DwmTransitionOwnedWindow(hwnd.into(), target).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
#[inline]
pub unsafe fn DwmUnregisterThumbnail(hthumbnailid: isize) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmUnregisterThumbnail(hthumbnailid: isize) -> ::windows::core::HRESULT;
    }
    DwmUnregisterThumbnail(hthumbnailid).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DwmUpdateThumbnailProperties(hthumbnailid: isize, ptnproperties: *const DWM_THUMBNAIL_PROPERTIES) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DwmUpdateThumbnailProperties(hthumbnailid: isize, ptnproperties: *const DWM_THUMBNAIL_PROPERTIES) -> ::windows::core::HRESULT;
    }
    DwmUpdateThumbnailProperties(hthumbnailid, ::core::mem::transmute(ptnproperties)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GESTURE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const GT_PEN_TAP: GESTURE_TYPE = GESTURE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const GT_PEN_DOUBLETAP: GESTURE_TYPE = GESTURE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const GT_PEN_RIGHTTAP: GESTURE_TYPE = GESTURE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const GT_PEN_PRESSANDHOLD: GESTURE_TYPE = GESTURE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const GT_PEN_PRESSANDHOLDABORT: GESTURE_TYPE = GESTURE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const GT_TOUCH_TAP: GESTURE_TYPE = GESTURE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const GT_TOUCH_DOUBLETAP: GESTURE_TYPE = GESTURE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const GT_TOUCH_RIGHTTAP: GESTURE_TYPE = GESTURE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const GT_TOUCH_PRESSANDHOLD: GESTURE_TYPE = GESTURE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const GT_TOUCH_PRESSANDHOLDABORT: GESTURE_TYPE = GESTURE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const GT_TOUCH_PRESSANDTAP: GESTURE_TYPE = GESTURE_TYPE(10i32);
impl ::core::marker::Copy for GESTURE_TYPE {}
impl ::core::clone::Clone for GESTURE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GESTURE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GESTURE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for GESTURE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GESTURE_TYPE").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
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
unsafe impl ::windows::core::Abi for MilMatrix3x2D {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MilMatrix3x2D {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MilMatrix3x2D>()) == 0 }
    }
}
impl ::core::cmp::Eq for MilMatrix3x2D {}
impl ::core::default::Default for MilMatrix3x2D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
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
unsafe impl ::windows::core::Abi for UNSIGNED_RATIO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UNSIGNED_RATIO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UNSIGNED_RATIO>()) == 0 }
    }
}
impl ::core::cmp::Eq for UNSIGNED_RATIO {}
impl ::core::default::Default for UNSIGNED_RATIO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const c_DwmMaxAdapters: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const c_DwmMaxMonitors: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Dwm\"`*"]
pub const c_DwmMaxQueuedBuffers: u32 = 8u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
