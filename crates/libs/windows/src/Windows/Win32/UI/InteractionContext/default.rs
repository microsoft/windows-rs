impl ::core::default::Default for CROSS_SLIDE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CROSS_SLIDE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CROSS_SLIDE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CROSS_SLIDE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CROSS_SLIDE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CROSS_SLIDE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CROSS_SLIDE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CROSS_SLIDE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CROSS_SLIDE_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CROSS_SLIDE_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.threshold == other.threshold && self.distance == other.distance
    }
}
impl ::core::cmp::Eq for CROSS_SLIDE_PARAMETER {}
impl ::core::fmt::Debug for CROSS_SLIDE_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CROSS_SLIDE_PARAMETER").field("threshold", &self.threshold).field("distance", &self.distance).finish()
    }
}
impl ::core::default::Default for CROSS_SLIDE_THRESHOLD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CROSS_SLIDE_THRESHOLD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CROSS_SLIDE_THRESHOLD").field(&self.0).finish()
    }
}
impl ::core::default::Default for HOLD_PARAMETER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HOLD_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HOLD_PARAMETER").field(&self.0).finish()
    }
}
impl ::core::default::Default for INERTIA_PARAMETER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INERTIA_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INERTIA_PARAMETER").field(&self.0).finish()
    }
}
impl ::core::default::Default for INTERACTION_ARGUMENTS_CROSS_SLIDE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERACTION_ARGUMENTS_CROSS_SLIDE {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags
    }
}
impl ::core::cmp::Eq for INTERACTION_ARGUMENTS_CROSS_SLIDE {}
impl ::core::fmt::Debug for INTERACTION_ARGUMENTS_CROSS_SLIDE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERACTION_ARGUMENTS_CROSS_SLIDE").field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for INTERACTION_ARGUMENTS_MANIPULATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERACTION_ARGUMENTS_MANIPULATION {
    fn eq(&self, other: &Self) -> bool {
        self.delta == other.delta && self.cumulative == other.cumulative && self.velocity == other.velocity && self.railsState == other.railsState
    }
}
impl ::core::cmp::Eq for INTERACTION_ARGUMENTS_MANIPULATION {}
impl ::core::fmt::Debug for INTERACTION_ARGUMENTS_MANIPULATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERACTION_ARGUMENTS_MANIPULATION").field("delta", &self.delta).field("cumulative", &self.cumulative).field("velocity", &self.velocity).field("railsState", &self.railsState).finish()
    }
}
impl ::core::default::Default for INTERACTION_ARGUMENTS_TAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERACTION_ARGUMENTS_TAP {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}
impl ::core::cmp::Eq for INTERACTION_ARGUMENTS_TAP {}
impl ::core::fmt::Debug for INTERACTION_ARGUMENTS_TAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERACTION_ARGUMENTS_TAP").field("count", &self.count).finish()
    }
}
impl ::core::default::Default for INTERACTION_CONFIGURATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INTERACTION_CONFIGURATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERACTION_CONFIGURATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for INTERACTION_CONFIGURATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for INTERACTION_CONFIGURATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for INTERACTION_CONFIGURATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for INTERACTION_CONFIGURATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for INTERACTION_CONFIGURATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for INTERACTION_CONTEXT_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERACTION_CONTEXT_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        self.interactionId == other.interactionId && self.enable == other.enable
    }
}
impl ::core::cmp::Eq for INTERACTION_CONTEXT_CONFIGURATION {}
impl ::core::fmt::Debug for INTERACTION_CONTEXT_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERACTION_CONTEXT_CONFIGURATION").field("interactionId", &self.interactionId).field("enable", &self.enable).finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for INTERACTION_CONTEXT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for INTERACTION_CONTEXT_OUTPUT2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for INTERACTION_CONTEXT_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INTERACTION_CONTEXT_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERACTION_CONTEXT_PROPERTY").field(&self.0).finish()
    }
}
impl ::core::default::Default for INTERACTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INTERACTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERACTION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for INTERACTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for INTERACTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for INTERACTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for INTERACTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for INTERACTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for INTERACTION_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INTERACTION_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERACTION_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for INTERACTION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INTERACTION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERACTION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MANIPULATION_RAILS_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MANIPULATION_RAILS_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MANIPULATION_RAILS_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MANIPULATION_TRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MANIPULATION_TRANSFORM {
    fn eq(&self, other: &Self) -> bool {
        self.translationX == other.translationX && self.translationY == other.translationY && self.scale == other.scale && self.expansion == other.expansion && self.rotation == other.rotation
    }
}
impl ::core::cmp::Eq for MANIPULATION_TRANSFORM {}
impl ::core::fmt::Debug for MANIPULATION_TRANSFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MANIPULATION_TRANSFORM").field("translationX", &self.translationX).field("translationY", &self.translationY).field("scale", &self.scale).field("expansion", &self.expansion).field("rotation", &self.rotation).finish()
    }
}
impl ::core::default::Default for MANIPULATION_VELOCITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MANIPULATION_VELOCITY {
    fn eq(&self, other: &Self) -> bool {
        self.velocityX == other.velocityX && self.velocityY == other.velocityY && self.velocityExpansion == other.velocityExpansion && self.velocityAngular == other.velocityAngular
    }
}
impl ::core::cmp::Eq for MANIPULATION_VELOCITY {}
impl ::core::fmt::Debug for MANIPULATION_VELOCITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MANIPULATION_VELOCITY").field("velocityX", &self.velocityX).field("velocityY", &self.velocityY).field("velocityExpansion", &self.velocityExpansion).field("velocityAngular", &self.velocityAngular).finish()
    }
}
impl ::core::default::Default for MOUSE_WHEEL_PARAMETER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MOUSE_WHEEL_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MOUSE_WHEEL_PARAMETER").field(&self.0).finish()
    }
}
impl ::core::default::Default for TAP_PARAMETER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAP_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAP_PARAMETER").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRANSLATION_PARAMETER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRANSLATION_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSLATION_PARAMETER").field(&self.0).finish()
    }
}
