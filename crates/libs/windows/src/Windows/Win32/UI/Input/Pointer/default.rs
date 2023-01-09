impl ::core::default::Default for INPUT_INJECTION_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INPUT_INJECTION_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.page == other.page && self.usage == other.usage && self.value == other.value && self.index == other.index
    }
}
impl ::core::cmp::Eq for INPUT_INJECTION_VALUE {}
impl ::core::fmt::Debug for INPUT_INJECTION_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INPUT_INJECTION_VALUE").field("page", &self.page).field("usage", &self.usage).field("value", &self.value).field("index", &self.index).finish()
    }
}
impl ::core::default::Default for INPUT_TRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for POINTER_BUTTON_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POINTER_BUTTON_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POINTER_BUTTON_CHANGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for POINTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POINTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POINTER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for POINTER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for POINTER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for POINTER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for POINTER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for POINTER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for POINTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for POINTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pointerType == other.pointerType && self.pointerId == other.pointerId && self.frameId == other.frameId && self.pointerFlags == other.pointerFlags && self.sourceDevice == other.sourceDevice && self.hwndTarget == other.hwndTarget && self.ptPixelLocation == other.ptPixelLocation && self.ptHimetricLocation == other.ptHimetricLocation && self.ptPixelLocationRaw == other.ptPixelLocationRaw && self.ptHimetricLocationRaw == other.ptHimetricLocationRaw && self.dwTime == other.dwTime && self.historyCount == other.historyCount && self.InputData == other.InputData && self.dwKeyStates == other.dwKeyStates && self.PerformanceCount == other.PerformanceCount && self.ButtonChangeType == other.ButtonChangeType
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for POINTER_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for POINTER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTER_INFO")
            .field("pointerType", &self.pointerType)
            .field("pointerId", &self.pointerId)
            .field("frameId", &self.frameId)
            .field("pointerFlags", &self.pointerFlags)
            .field("sourceDevice", &self.sourceDevice)
            .field("hwndTarget", &self.hwndTarget)
            .field("ptPixelLocation", &self.ptPixelLocation)
            .field("ptHimetricLocation", &self.ptHimetricLocation)
            .field("ptPixelLocationRaw", &self.ptPixelLocationRaw)
            .field("ptHimetricLocationRaw", &self.ptHimetricLocationRaw)
            .field("dwTime", &self.dwTime)
            .field("historyCount", &self.historyCount)
            .field("InputData", &self.InputData)
            .field("dwKeyStates", &self.dwKeyStates)
            .field("PerformanceCount", &self.PerformanceCount)
            .field("ButtonChangeType", &self.ButtonChangeType)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for POINTER_PEN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for POINTER_PEN_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pointerInfo == other.pointerInfo && self.penFlags == other.penFlags && self.penMask == other.penMask && self.pressure == other.pressure && self.rotation == other.rotation && self.tiltX == other.tiltX && self.tiltY == other.tiltY
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for POINTER_PEN_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for POINTER_PEN_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTER_PEN_INFO").field("pointerInfo", &self.pointerInfo).field("penFlags", &self.penFlags).field("penMask", &self.penMask).field("pressure", &self.pressure).field("rotation", &self.rotation).field("tiltX", &self.tiltX).field("tiltY", &self.tiltY).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for POINTER_TOUCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for POINTER_TOUCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pointerInfo == other.pointerInfo && self.touchFlags == other.touchFlags && self.touchMask == other.touchMask && self.rcContact == other.rcContact && self.rcContactRaw == other.rcContactRaw && self.orientation == other.orientation && self.pressure == other.pressure
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for POINTER_TOUCH_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for POINTER_TOUCH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTER_TOUCH_INFO").field("pointerInfo", &self.pointerInfo).field("touchFlags", &self.touchFlags).field("touchMask", &self.touchMask).field("rcContact", &self.rcContact).field("rcContactRaw", &self.rcContactRaw).field("orientation", &self.orientation).field("pressure", &self.pressure).finish()
    }
}
impl ::core::default::Default for TOUCH_FEEDBACK_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOUCH_FEEDBACK_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOUCH_FEEDBACK_MODE").field(&self.0).finish()
    }
}
