#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const CLSID_AutoScrollBehavior: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26126a51_3c70_4c9a_aec2_948849eeb093);
pub const CLSID_DeferContactService: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7b67cf4_84bb_434e_86ae_6592bbc9abd9);
pub const CLSID_DragDropConfigurationBehavior: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09b01b3e_ba6c_454d_82e8_95e352329f23);
pub const CLSID_HorizontalIndicatorContent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7d18cf5_3ec7_44d5_a76b_3770f3cf903d);
pub const CLSID_VerticalIndicatorContent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa10b5f17_afe0_4aa2_91e9_3e7001d2e6b4);
pub const CLSID_VirtualViewportContent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3206a19a_86f0_4cb4_a7f3_16e3b7e2d852);
pub const DCompManipulationCompositor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79dea627_a08a_43ac_8ef5_6900b9299126);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_STOP: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_FORWARD: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_REVERSE: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(2i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION {}
impl ::core::clone::Clone for DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIRECTMANIPULATION_CONFIGURATION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_NONE: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_INTERACTION: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_X: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_Y: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(4i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(16i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_INERTIA: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(32i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING_INERTIA: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(128i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_X: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(256i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_Y: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(512i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_CONFIGURATION {}
impl ::core::clone::Clone for DIRECTMANIPULATION_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_CONFIGURATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_CONFIGURATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_CONFIGURATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_VERTICAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HORIZONTAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_ONLY: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(16i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(32i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HOLD_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(64i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {}
impl ::core::clone::Clone for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIRECTMANIPULATION_DRAG_DROP_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_READY: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_PRESELECT: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_SELECTING: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_DRAGGING: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CANCELLED: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_COMMITTED: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(5i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_DRAG_DROP_STATUS {}
impl ::core::clone::Clone for DIRECTMANIPULATION_DRAG_DROP_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_DRAG_DROP_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_DRAG_DROP_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_DRAG_DROP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_DRAG_DROP_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIRECTMANIPULATION_GESTURE_CONFIGURATION(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_NONE: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_DEFAULT: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_VERTICAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(8i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_HORIZONTAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(16i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(32i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_GESTURE_CONFIGURATION {}
impl ::core::clone::Clone for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_GESTURE_CONFIGURATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIRECTMANIPULATION_HITTEST_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_ASYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = DIRECTMANIPULATION_HITTEST_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = DIRECTMANIPULATION_HITTEST_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_AUTO_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = DIRECTMANIPULATION_HITTEST_TYPE(2i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_HITTEST_TYPE {}
impl ::core::clone::Clone for DIRECTMANIPULATION_HITTEST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_HITTEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_HITTEST_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_HITTEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_HITTEST_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIRECTMANIPULATION_HORIZONTALALIGNMENT(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_NONE: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_LEFT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_CENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_RIGHT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(4i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(8i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_HORIZONTALALIGNMENT {}
impl ::core::clone::Clone for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_HORIZONTALALIGNMENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIRECTMANIPULATION_INPUT_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INPUT_MODE_AUTOMATIC: DIRECTMANIPULATION_INPUT_MODE = DIRECTMANIPULATION_INPUT_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INPUT_MODE_MANUAL: DIRECTMANIPULATION_INPUT_MODE = DIRECTMANIPULATION_INPUT_MODE(1i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_INPUT_MODE {}
impl ::core::clone::Clone for DIRECTMANIPULATION_INPUT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_INPUT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_INPUT_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_INPUT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_INPUT_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIRECTMANIPULATION_INTERACTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_BEGIN: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_MANIPULATION: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_TAP: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_HOLD: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_CROSS_SLIDE: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_END: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(100i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_INTERACTION_TYPE {}
impl ::core::clone::Clone for DIRECTMANIPULATION_INTERACTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_INTERACTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_INTERACTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_INTERACTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_INTERACTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_KEYBOARDFOCUS: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIRECTMANIPULATION_MOTION_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_NONE: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_TRANSLATEX: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_TRANSLATEY: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_ZOOM: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(4i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_CENTERX: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(16i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_CENTERY: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(32i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_ALL: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(55i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_MOTION_TYPES {}
impl ::core::clone::Clone for DIRECTMANIPULATION_MOTION_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_MOTION_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_MOTION_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_MOTION_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_MOTION_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOUSEFOCUS: u32 = 4294967293u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIRECTMANIPULATION_SNAPPOINT_COORDINATE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_COORDINATE_BOUNDARY: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = DIRECTMANIPULATION_SNAPPOINT_COORDINATE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_COORDINATE_ORIGIN: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = DIRECTMANIPULATION_SNAPPOINT_COORDINATE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_COORDINATE_MIRRORED: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = DIRECTMANIPULATION_SNAPPOINT_COORDINATE(16i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {}
impl ::core::clone::Clone for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_SNAPPOINT_COORDINATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIRECTMANIPULATION_SNAPPOINT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(3i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_SNAPPOINT_TYPE {}
impl ::core::clone::Clone for DIRECTMANIPULATION_SNAPPOINT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_SNAPPOINT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_SNAPPOINT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_SNAPPOINT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_SNAPPOINT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIRECTMANIPULATION_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_BUILDING: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_ENABLED: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DISABLED: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_RUNNING: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INERTIA: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_READY: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(5i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SUSPENDED: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(6i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_STATUS {}
impl ::core::clone::Clone for DIRECTMANIPULATION_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIRECTMANIPULATION_VERTICALALIGNMENT(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_NONE: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_TOP: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_CENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_BOTTOM: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(4i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(8i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_VERTICALALIGNMENT {}
impl ::core::clone::Clone for DIRECTMANIPULATION_VERTICALALIGNMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_VERTICALALIGNMENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_VERTICALALIGNMENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_VERTICALALIGNMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_VERTICALALIGNMENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DIRECTMANIPULATION_VIEWPORT_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DEFAULT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_AUTODISABLE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_MANUALUPDATE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_INPUT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_EXPLICITHITTEST: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(8i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DISABLEPIXELSNAPPING: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(16i32);
impl ::core::marker::Copy for DIRECTMANIPULATION_VIEWPORT_OPTIONS {}
impl ::core::clone::Clone for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_VIEWPORT_OPTIONS").field(&self.0).finish()
    }
}
pub const DirectManipulationManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54e211b6_3650_4f75_8334_fa359598e1c5);
pub const DirectManipulationPrimaryContent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaa02661_d59e_41c7_8393_3ba3bacb6b57);
pub const DirectManipulationSharedManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99793286_77cc_4b57_96db_3b354f6f9fb5);
pub const DirectManipulationUpdateManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fc1bfd5_1835_441a_b3b1_b6cc74b727d0);
pub const DirectManipulationViewport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34e211b6_3650_4f75_8334_fa359598e1c5);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationAutoScrollBehavior(::windows::core::IUnknown);
impl IDirectManipulationAutoScrollBehavior {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetConfiguration(&self, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetConfiguration)(::core::mem::transmute_copy(self), ::core::mem::transmute(motiontypes), ::core::mem::transmute(scrollmotion)).ok()
    }
}
impl ::core::convert::From<IDirectManipulationAutoScrollBehavior> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationAutoScrollBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationAutoScrollBehavior> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationAutoScrollBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationAutoScrollBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationAutoScrollBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationAutoScrollBehavior {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationAutoScrollBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationAutoScrollBehavior {}
impl ::core::fmt::Debug for IDirectManipulationAutoScrollBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationAutoScrollBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationAutoScrollBehavior {
    type Vtable = IDirectManipulationAutoScrollBehavior_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d5954d4_2003_4356_9b31_d051c9ff0af7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationAutoScrollBehavior_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationCompositor(::windows::core::IUnknown);
impl IDirectManipulationCompositor {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn AddContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0, device: Param1, parentvisual: Param2, childvisual: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddContent)(::core::mem::transmute_copy(self), content.into_param().abi(), device.into_param().abi(), parentvisual.into_param().abi(), childvisual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn RemoveContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveContent)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetUpdateManager<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationUpdateManager>>(&self, updatemanager: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUpdateManager)(::core::mem::transmute_copy(self), updatemanager.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Flush)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IDirectManipulationCompositor> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationCompositor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationCompositor> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationCompositor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationCompositor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationCompositor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationCompositor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationCompositor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationCompositor {}
impl ::core::fmt::Debug for IDirectManipulationCompositor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationCompositor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationCompositor {
    type Vtable = IDirectManipulationCompositor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x537a0825_0387_4efa_b62f_71eb1f085a7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationCompositor_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AddContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetUpdateManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatemanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationCompositor2(::windows::core::IUnknown);
impl IDirectManipulationCompositor2 {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn AddContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0, device: Param1, parentvisual: Param2, childvisual: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddContent)(::core::mem::transmute_copy(self), content.into_param().abi(), device.into_param().abi(), parentvisual.into_param().abi(), childvisual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn RemoveContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveContent)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetUpdateManager<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationUpdateManager>>(&self, updatemanager: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetUpdateManager)(::core::mem::transmute_copy(self), updatemanager.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Flush)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn AddContentWithCrossProcessChaining<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationPrimaryContent>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0, device: Param1, parentvisual: Param2, childvisual: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddContentWithCrossProcessChaining)(::core::mem::transmute_copy(self), content.into_param().abi(), device.into_param().abi(), parentvisual.into_param().abi(), childvisual.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDirectManipulationCompositor2> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationCompositor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationCompositor2> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationCompositor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationCompositor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationCompositor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirectManipulationCompositor2> for IDirectManipulationCompositor {
    fn from(value: IDirectManipulationCompositor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationCompositor2> for IDirectManipulationCompositor {
    fn from(value: &IDirectManipulationCompositor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectManipulationCompositor> for IDirectManipulationCompositor2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectManipulationCompositor> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectManipulationCompositor> for &'a IDirectManipulationCompositor2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectManipulationCompositor> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationCompositor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationCompositor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationCompositor2 {}
impl ::core::fmt::Debug for IDirectManipulationCompositor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationCompositor2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationCompositor2 {
    type Vtable = IDirectManipulationCompositor2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd38c7822_f1cb_43cb_b4b9_ac0c767a412e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationCompositor2_Vtbl {
    pub base: IDirectManipulationCompositor_Vtbl,
    pub AddContentWithCrossProcessChaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationContent(::windows::core::IUnknown);
impl IDirectManipulationContent {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetContentRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: super::super::Foundation::RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetContentRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContentRect(&self, contentsize: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContentRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(contentsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetViewport<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetViewport)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetTag(&self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTag)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTag)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetOutputTransform(&self, matrix: &mut [f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutputTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(matrix)), matrix.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetContentTransform(&self, matrix: &mut [f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetContentTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(matrix)), matrix.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SyncContentTransform(&self, matrix: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SyncContentTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(matrix)), matrix.len() as _).ok()
    }
}
impl ::core::convert::From<IDirectManipulationContent> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationContent> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationContent {}
impl ::core::fmt::Debug for IDirectManipulationContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationContent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationContent {
    type Vtable = IDirectManipulationContent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb89962cb_3d89_442b_bb58_5098fa0f9f16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationContent_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetContentRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentsize: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetContentRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetContentRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentsize: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetContentRect: usize,
    pub GetViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT,
    pub GetOutputTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub GetContentTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub SyncContentTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationDeferContactService(::windows::core::IUnknown);
impl IDirectManipulationDeferContactService {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn DeferContact(&self, pointerid: u32, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeferContact)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid), ::core::mem::transmute(timeout)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn CancelContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelContact)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn CancelDeferral(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelDeferral)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
}
impl ::core::convert::From<IDirectManipulationDeferContactService> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationDeferContactService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationDeferContactService> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationDeferContactService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationDeferContactService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationDeferContactService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationDeferContactService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationDeferContactService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationDeferContactService {}
impl ::core::fmt::Debug for IDirectManipulationDeferContactService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationDeferContactService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationDeferContactService {
    type Vtable = IDirectManipulationDeferContactService_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x652d5c71_fe60_4a98_be70_e5f21291e7f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDeferContactService_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub DeferContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, timeout: u32) -> ::windows::core::HRESULT,
    pub CancelContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT,
    pub CancelDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationDragDropBehavior(::windows::core::IUnknown);
impl IDirectManipulationDragDropBehavior {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetConfiguration(&self, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetConfiguration)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<DIRECTMANIPULATION_DRAG_DROP_STATUS> {
        let mut result__: DIRECTMANIPULATION_DRAG_DROP_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DIRECTMANIPULATION_DRAG_DROP_STATUS>(result__)
    }
}
impl ::core::convert::From<IDirectManipulationDragDropBehavior> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationDragDropBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationDragDropBehavior> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationDragDropBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationDragDropBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationDragDropBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationDragDropBehavior {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationDragDropBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationDragDropBehavior {}
impl ::core::fmt::Debug for IDirectManipulationDragDropBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationDragDropBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationDragDropBehavior {
    type Vtable = IDirectManipulationDragDropBehavior_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x814b5af5_c2c8_4270_a9b7_a198ce8d02fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDragDropBehavior_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationDragDropEventHandler(::windows::core::IUnknown);
impl IDirectManipulationDragDropEventHandler {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn OnDragDropStatusChange<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationViewport2>>(&self, viewport: Param0, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDragDropStatusChange)(::core::mem::transmute_copy(self), viewport.into_param().abi(), ::core::mem::transmute(current), ::core::mem::transmute(previous)).ok()
    }
}
impl ::core::convert::From<IDirectManipulationDragDropEventHandler> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationDragDropEventHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationDragDropEventHandler> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationDragDropEventHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationDragDropEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationDragDropEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationDragDropEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationDragDropEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationDragDropEventHandler {}
impl ::core::fmt::Debug for IDirectManipulationDragDropEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationDragDropEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationDragDropEventHandler {
    type Vtable = IDirectManipulationDragDropEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fa11b10_701b_41ae_b5f2_49e36bd595aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDragDropEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnDragDropStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationFrameInfoProvider(::windows::core::IUnknown);
impl IDirectManipulationFrameInfoProvider {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetNextFrameInfo(&self, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNextFrameInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(time), ::core::mem::transmute(processtime), ::core::mem::transmute(compositiontime)).ok()
    }
}
impl ::core::convert::From<IDirectManipulationFrameInfoProvider> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationFrameInfoProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationFrameInfoProvider> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationFrameInfoProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationFrameInfoProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationFrameInfoProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationFrameInfoProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationFrameInfoProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationFrameInfoProvider {}
impl ::core::fmt::Debug for IDirectManipulationFrameInfoProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationFrameInfoProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationFrameInfoProvider {
    type Vtable = IDirectManipulationFrameInfoProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb759dba_6f4c_4c01_874e_19c8a05907f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationFrameInfoProvider_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetNextFrameInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationInteractionEventHandler(::windows::core::IUnknown);
impl IDirectManipulationInteractionEventHandler {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn OnInteraction<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationViewport2>>(&self, viewport: Param0, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnInteraction)(::core::mem::transmute_copy(self), viewport.into_param().abi(), ::core::mem::transmute(interaction)).ok()
    }
}
impl ::core::convert::From<IDirectManipulationInteractionEventHandler> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationInteractionEventHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationInteractionEventHandler> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationInteractionEventHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationInteractionEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationInteractionEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationInteractionEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationInteractionEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationInteractionEventHandler {}
impl ::core::fmt::Debug for IDirectManipulationInteractionEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationInteractionEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationInteractionEventHandler {
    type Vtable = IDirectManipulationInteractionEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe43f45b8_42b4_403e_b1f2_273b8f510830);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationInteractionEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnInteraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationManager(::windows::core::IUnknown);
impl IDirectManipulationManager {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Activate)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deactivate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Deactivate)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHitTestTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0, hittestwindow: Param1, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterHitTestTarget)(::core::mem::transmute_copy(self), window.into_param().abi(), hittestwindow.into_param().abi(), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ProcessInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(message), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetUpdateManager<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetUpdateManager)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateViewport<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::core::Interface>(&self, frameinfo: Param0, window: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateViewport)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), window.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn CreateContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, T: ::windows::core::Interface>(&self, frameinfo: Param0, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateContent)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDirectManipulationManager> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationManager> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationManager {}
impl ::core::fmt::Debug for IDirectManipulationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationManager {
    type Vtable = IDirectManipulationManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbf5d3b4_70c7_4163_9322_5a6f660d6fbc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Activate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Deactivate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterHitTestTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterHitTestTarget: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub ProcessInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    ProcessInput: usize,
    pub GetUpdateManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateViewport: usize,
    pub CreateContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationManager2(::windows::core::IUnknown);
impl IDirectManipulationManager2 {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Activate)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deactivate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Deactivate)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHitTestTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0, hittestwindow: Param1, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RegisterHitTestTarget)(::core::mem::transmute_copy(self), window.into_param().abi(), hittestwindow.into_param().abi(), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ProcessInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(message), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetUpdateManager<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetUpdateManager)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateViewport<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::core::Interface>(&self, frameinfo: Param0, window: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.CreateViewport)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), window.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn CreateContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, T: ::windows::core::Interface>(&self, frameinfo: Param0, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.CreateContent)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn CreateBehavior<T: ::windows::core::Interface>(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateBehavior)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDirectManipulationManager2> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationManager2> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirectManipulationManager2> for IDirectManipulationManager {
    fn from(value: IDirectManipulationManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationManager2> for IDirectManipulationManager {
    fn from(value: &IDirectManipulationManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectManipulationManager> for IDirectManipulationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectManipulationManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectManipulationManager> for &'a IDirectManipulationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectManipulationManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationManager2 {}
impl ::core::fmt::Debug for IDirectManipulationManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationManager2 {
    type Vtable = IDirectManipulationManager2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa1005e9_3d16_484c_bfc9_62b61e56ec4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager2_Vtbl {
    pub base: IDirectManipulationManager_Vtbl,
    pub CreateBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationManager3(::windows::core::IUnknown);
impl IDirectManipulationManager3 {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Activate)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deactivate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Deactivate)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHitTestTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0, hittestwindow: Param1, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.RegisterHitTestTarget)(::core::mem::transmute_copy(self), window.into_param().abi(), hittestwindow.into_param().abi(), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.ProcessInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(message), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetUpdateManager<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.GetUpdateManager)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateViewport<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::core::Interface>(&self, frameinfo: Param0, window: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.CreateViewport)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), window.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn CreateContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, T: ::windows::core::Interface>(&self, frameinfo: Param0, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.base.CreateContent)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn CreateBehavior<T: ::windows::core::Interface>(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.CreateBehavior)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetService<T: ::windows::core::Interface>(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetService)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDirectManipulationManager3> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationManager3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationManager3> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationManager3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirectManipulationManager3> for IDirectManipulationManager {
    fn from(value: IDirectManipulationManager3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationManager3> for IDirectManipulationManager {
    fn from(value: &IDirectManipulationManager3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectManipulationManager> for IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectManipulationManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectManipulationManager> for &'a IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectManipulationManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirectManipulationManager3> for IDirectManipulationManager2 {
    fn from(value: IDirectManipulationManager3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationManager3> for IDirectManipulationManager2 {
    fn from(value: &IDirectManipulationManager3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectManipulationManager2> for IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectManipulationManager2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectManipulationManager2> for &'a IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectManipulationManager2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationManager3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationManager3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationManager3 {}
impl ::core::fmt::Debug for IDirectManipulationManager3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationManager3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationManager3 {
    type Vtable = IDirectManipulationManager3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cb6b33d_ffe8_488c_b750_fbdfe88dca8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager3_Vtbl {
    pub base: IDirectManipulationManager2_Vtbl,
    pub GetService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationPrimaryContent(::windows::core::IUnknown);
impl IDirectManipulationPrimaryContent {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetSnapInterval(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSnapInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(motion), ::core::mem::transmute(interval), ::core::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetSnapPoints(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, points: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSnapPoints)(::core::mem::transmute_copy(self), ::core::mem::transmute(motion), ::core::mem::transmute(::windows::core::as_ptr_or_null(points)), points.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetSnapType(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSnapType)(::core::mem::transmute_copy(self), ::core::mem::transmute(motion), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetSnapCoordinate(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSnapCoordinate)(::core::mem::transmute_copy(self), ::core::mem::transmute(motion), ::core::mem::transmute(coordinate), ::core::mem::transmute(origin)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetZoomBoundaries(&self, zoomminimum: f32, zoommaximum: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetZoomBoundaries)(::core::mem::transmute_copy(self), ::core::mem::transmute(zoomminimum), ::core::mem::transmute(zoommaximum)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetHorizontalAlignment(&self, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHorizontalAlignment)(::core::mem::transmute_copy(self), ::core::mem::transmute(alignment)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetVerticalAlignment(&self, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVerticalAlignment)(::core::mem::transmute_copy(self), ::core::mem::transmute(alignment)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetInertiaEndTransform(&self, matrix: &mut [f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInertiaEndTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(matrix)), matrix.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetCenterPoint(&self, centerx: *mut f32, centery: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCenterPoint)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerx), ::core::mem::transmute(centery)).ok()
    }
}
impl ::core::convert::From<IDirectManipulationPrimaryContent> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationPrimaryContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationPrimaryContent> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationPrimaryContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationPrimaryContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationPrimaryContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationPrimaryContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationPrimaryContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationPrimaryContent {}
impl ::core::fmt::Debug for IDirectManipulationPrimaryContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationPrimaryContent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationPrimaryContent {
    type Vtable = IDirectManipulationPrimaryContent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc12851e4_1698_4625_b9b1_7ca3ec18630b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationPrimaryContent_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetSnapInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::core::HRESULT,
    pub SetSnapPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub SetSnapType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::core::HRESULT,
    pub SetSnapCoordinate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::core::HRESULT,
    pub SetZoomBoundaries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, zoomminimum: f32, zoommaximum: f32) -> ::windows::core::HRESULT,
    pub SetHorizontalAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::core::HRESULT,
    pub SetVerticalAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::core::HRESULT,
    pub GetInertiaEndTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub GetCenterPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: *mut f32, centery: *mut f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationUpdateHandler(::windows::core::IUnknown);
impl IDirectManipulationUpdateHandler {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn Update(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Update)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IDirectManipulationUpdateHandler> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationUpdateHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationUpdateHandler> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationUpdateHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationUpdateHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationUpdateHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationUpdateHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationUpdateHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationUpdateHandler {}
impl ::core::fmt::Debug for IDirectManipulationUpdateHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationUpdateHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationUpdateHandler {
    type Vtable = IDirectManipulationUpdateHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x790b6337_64f8_4ff5_a269_b32bc2af27a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationUpdateHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationUpdateManager(::windows::core::IUnknown);
impl IDirectManipulationUpdateManager {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterWaitHandleCallback<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, IDirectManipulationUpdateHandler>>(&self, handle: Param0, eventhandler: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisterWaitHandleCallback)(::core::mem::transmute_copy(self), handle.into_param().abi(), eventhandler.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn UnregisterWaitHandleCallback(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterWaitHandleCallback)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn Update<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>>(&self, frameinfo: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Update)(::core::mem::transmute_copy(self), frameinfo.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDirectManipulationUpdateManager> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationUpdateManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationUpdateManager> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationUpdateManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationUpdateManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationUpdateManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationUpdateManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationUpdateManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationUpdateManager {}
impl ::core::fmt::Debug for IDirectManipulationUpdateManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationUpdateManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationUpdateManager {
    type Vtable = IDirectManipulationUpdateManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0ae62fd_be34_46e7_9caa_d361facbb9cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationUpdateManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterWaitHandleCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterWaitHandleCallback: usize,
    pub UnregisterWaitHandleCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationViewport(::windows::core::IUnknown);
impl IDirectManipulationViewport {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn Enable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Enable)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn Disable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disable)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContact)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn ReleaseContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseContact)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn ReleaseAllContacts(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseAllContacts)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<DIRECTMANIPULATION_STATUS> {
        let mut result__: DIRECTMANIPULATION_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DIRECTMANIPULATION_STATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetTag(&self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTag)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTag)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetViewportRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: super::super::Foundation::RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetViewportRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetViewportRect(&self, viewport: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetViewportRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewport)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ZoomToRect<'a, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, left: f32, top: f32, right: f32, bottom: f32, animate: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ZoomToRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(left), ::core::mem::transmute(top), ::core::mem::transmute(right), ::core::mem::transmute(bottom), animate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetViewportTransform(&self, matrix: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetViewportTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(matrix)), matrix.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SyncDisplayTransform(&self, matrix: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SyncDisplayTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(matrix)), matrix.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetPrimaryContent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetPrimaryContent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn AddContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddContent)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn RemoveContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveContent)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetViewportOptions(&self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetViewportOptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(options)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn AddConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddConfiguration)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn RemoveConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveConfiguration)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn ActivateConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ActivateConfiguration)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetManualGesture(&self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetManualGesture)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetChaining(&self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChaining)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabledtypes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddEventHandler<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, IDirectManipulationViewportEventHandler>>(&self, window: Param0, eventhandler: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AddEventHandler)(::core::mem::transmute_copy(self), window.into_param().abi(), eventhandler.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn RemoveEventHandler(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveEventHandler)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetInputMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInputMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetUpdateMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUpdateMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn Abandon(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abandon)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IDirectManipulationViewport> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationViewport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationViewport> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationViewport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationViewport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationViewport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationViewport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationViewport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationViewport {}
impl ::core::fmt::Debug for IDirectManipulationViewport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationViewport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationViewport {
    type Vtable = IDirectManipulationViewport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28b85a3d_60a0_48bd_9ba1_5ce8d9ea3a6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewport_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT,
    pub ReleaseContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT,
    pub ReleaseAllContacts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetViewportRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetViewportRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetViewportRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetViewportRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ZoomToRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ZoomToRect: usize,
    pub SetViewportTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub SyncDisplayTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub GetPrimaryContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetViewportOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::HRESULT,
    pub AddConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub RemoveConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub ActivateConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub SetManualGesture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::HRESULT,
    pub SetChaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddEventHandler: usize,
    pub RemoveEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT,
    pub SetInputMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT,
    pub SetUpdateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Abandon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationViewport2(::windows::core::IUnknown);
impl IDirectManipulationViewport2 {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn Enable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Enable)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn Disable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Disable)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetContact)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn ReleaseContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ReleaseContact)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn ReleaseAllContacts(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ReleaseAllContacts)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<DIRECTMANIPULATION_STATUS> {
        let mut result__: DIRECTMANIPULATION_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DIRECTMANIPULATION_STATUS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetTag(&self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetTag)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTag)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetViewportRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: super::super::Foundation::RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetViewportRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetViewportRect(&self, viewport: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetViewportRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewport)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ZoomToRect<'a, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, left: f32, top: f32, right: f32, bottom: f32, animate: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ZoomToRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(left), ::core::mem::transmute(top), ::core::mem::transmute(right), ::core::mem::transmute(bottom), animate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetViewportTransform(&self, matrix: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetViewportTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(matrix)), matrix.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SyncDisplayTransform(&self, matrix: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SyncDisplayTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(matrix)), matrix.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn GetPrimaryContent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetPrimaryContent)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn AddContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddContent)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn RemoveContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveContent)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetViewportOptions(&self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetViewportOptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(options)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn AddConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddConfiguration)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn RemoveConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveConfiguration)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn ActivateConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ActivateConfiguration)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetManualGesture(&self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetManualGesture)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetChaining(&self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetChaining)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabledtypes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddEventHandler<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, IDirectManipulationViewportEventHandler>>(&self, window: Param0, eventhandler: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.AddEventHandler)(::core::mem::transmute_copy(self), window.into_param().abi(), eventhandler.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn RemoveEventHandler(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveEventHandler)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetInputMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetInputMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn SetUpdateMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetUpdateMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Stop)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn Abandon(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Abandon)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn AddBehavior<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, behavior: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AddBehavior)(::core::mem::transmute_copy(self), behavior.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn RemoveBehavior(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveBehavior)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn RemoveAllBehaviors(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAllBehaviors)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IDirectManipulationViewport2> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationViewport2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationViewport2> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationViewport2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationViewport2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationViewport2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirectManipulationViewport2> for IDirectManipulationViewport {
    fn from(value: IDirectManipulationViewport2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationViewport2> for IDirectManipulationViewport {
    fn from(value: &IDirectManipulationViewport2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectManipulationViewport> for IDirectManipulationViewport2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectManipulationViewport> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectManipulationViewport> for &'a IDirectManipulationViewport2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectManipulationViewport> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationViewport2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationViewport2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationViewport2 {}
impl ::core::fmt::Debug for IDirectManipulationViewport2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationViewport2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationViewport2 {
    type Vtable = IDirectManipulationViewport2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x923ccaac_61e1_4385_b726_017af189882a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewport2_Vtbl {
    pub base: IDirectManipulationViewport_Vtbl,
    pub AddBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, behavior: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT,
    pub RemoveBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT,
    pub RemoveAllBehaviors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
#[repr(transparent)]
pub struct IDirectManipulationViewportEventHandler(::windows::core::IUnknown);
impl IDirectManipulationViewportEventHandler {
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn OnViewportStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationViewport>>(&self, viewport: Param0, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnViewportStatusChanged)(::core::mem::transmute_copy(self), viewport.into_param().abi(), ::core::mem::transmute(current), ::core::mem::transmute(previous)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn OnViewportUpdated<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationViewport>>(&self, viewport: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnViewportUpdated)(::core::mem::transmute_copy(self), viewport.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
    pub unsafe fn OnContentUpdated<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationViewport>, Param1: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, viewport: Param0, content: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnContentUpdated)(::core::mem::transmute_copy(self), viewport.into_param().abi(), content.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDirectManipulationViewportEventHandler> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationViewportEventHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectManipulationViewportEventHandler> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationViewportEventHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationViewportEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationViewportEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectManipulationViewportEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectManipulationViewportEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectManipulationViewportEventHandler {}
impl ::core::fmt::Debug for IDirectManipulationViewportEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectManipulationViewportEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationViewportEventHandler {
    type Vtable = IDirectManipulationViewportEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x952121da_d69f_45f9_b0f9_f23944321a6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewportEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnViewportStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT,
    pub OnViewportUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnContentUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
