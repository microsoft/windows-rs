impl ::core::default::Default for DWMFLIP3DWINDOWPOLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWMFLIP3DWINDOWPOLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWMFLIP3DWINDOWPOLICY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWMNCRENDERINGPOLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWMNCRENDERINGPOLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWMNCRENDERINGPOLICY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWMTRANSITION_OWNEDWINDOW_TARGET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWMTRANSITION_OWNEDWINDOW_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWMTRANSITION_OWNEDWINDOW_TARGET").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWMWINDOWATTRIBUTE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWMWINDOWATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWMWINDOWATTRIBUTE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DWM_BLURBEHIND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWM_PRESENT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DWM_SHOWCONTACT {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for DWM_SOURCE_FRAME_SAMPLING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWM_SOURCE_FRAME_SAMPLING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWM_SOURCE_FRAME_SAMPLING").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWM_TAB_WINDOW_REQUIREMENTS {
    fn default() -> Self {
        Self(0)
    }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWM_THUMBNAIL_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DWM_TIMING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DWM_WINDOW_CORNER_PREFERENCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWM_WINDOW_CORNER_PREFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWM_WINDOW_CORNER_PREFERENCE").field(&self.0).finish()
    }
}
impl ::core::default::Default for GESTURE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GESTURE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GESTURE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MilMatrix3x2D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for UNSIGNED_RATIO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
