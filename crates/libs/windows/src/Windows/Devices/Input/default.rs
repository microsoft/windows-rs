impl ::core::cmp::PartialEq for KeyboardCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyboardCapabilities {}
impl ::core::fmt::Debug for KeyboardCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyboardCapabilities").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MouseCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MouseCapabilities {}
impl ::core::fmt::Debug for MouseCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MouseCapabilities").field(&self.0).finish()
    }
}
impl ::core::default::Default for MouseDelta {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MouseDelta {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for MouseDelta {}
impl ::core::fmt::Debug for MouseDelta {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MouseDelta").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::core::cmp::PartialEq for MouseDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MouseDevice {}
impl ::core::fmt::Debug for MouseDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MouseDevice").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MouseEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MouseEventArgs {}
impl ::core::fmt::Debug for MouseEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MouseEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PenButtonListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenButtonListener {}
impl ::core::fmt::Debug for PenButtonListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenButtonListener").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PenDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenDevice {}
impl ::core::fmt::Debug for PenDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenDevice").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PenDockListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenDockListener {}
impl ::core::fmt::Debug for PenDockListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenDockListener").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PenDockedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenDockedEventArgs {}
impl ::core::fmt::Debug for PenDockedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenDockedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PenTailButtonClickedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenTailButtonClickedEventArgs {}
impl ::core::fmt::Debug for PenTailButtonClickedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenTailButtonClickedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PenTailButtonDoubleClickedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenTailButtonDoubleClickedEventArgs {}
impl ::core::fmt::Debug for PenTailButtonDoubleClickedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenTailButtonDoubleClickedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PenTailButtonLongPressedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenTailButtonLongPressedEventArgs {}
impl ::core::fmt::Debug for PenTailButtonLongPressedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenTailButtonLongPressedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PenUndockedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenUndockedEventArgs {}
impl ::core::fmt::Debug for PenUndockedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenUndockedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PointerDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerDevice {}
impl ::core::fmt::Debug for PointerDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerDevice").field(&self.0).finish()
    }
}
impl ::core::default::Default for PointerDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PointerDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerDeviceType").field(&self.0).finish()
    }
}
impl ::core::default::Default for PointerDeviceUsage {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PointerDeviceUsage {
    fn eq(&self, other: &Self) -> bool {
        self.UsagePage == other.UsagePage && self.Usage == other.Usage && self.MinLogical == other.MinLogical && self.MaxLogical == other.MaxLogical && self.MinPhysical == other.MinPhysical && self.MaxPhysical == other.MaxPhysical && self.Unit == other.Unit && self.PhysicalMultiplier == other.PhysicalMultiplier
    }
}
impl ::core::cmp::Eq for PointerDeviceUsage {}
impl ::core::fmt::Debug for PointerDeviceUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PointerDeviceUsage").field("UsagePage", &self.UsagePage).field("Usage", &self.Usage).field("MinLogical", &self.MinLogical).field("MaxLogical", &self.MaxLogical).field("MinPhysical", &self.MinPhysical).field("MaxPhysical", &self.MaxPhysical).field("Unit", &self.Unit).field("PhysicalMultiplier", &self.PhysicalMultiplier).finish()
    }
}
impl ::core::cmp::PartialEq for TouchCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TouchCapabilities {}
impl ::core::fmt::Debug for TouchCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TouchCapabilities").field(&self.0).finish()
    }
}
