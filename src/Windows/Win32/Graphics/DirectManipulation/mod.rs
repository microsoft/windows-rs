#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_AutoScrollBehavior: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(638741073, 15472, 19610, [174, 194, 148, 136, 73, 238, 176, 147]);
pub const CLSID_DeferContactService: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3619060980, 33979, 17230, [134, 174, 101, 146, 187, 201, 171, 217]);
pub const CLSID_DragDropConfigurationBehavior: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(162536254, 47724, 17741, [130, 232, 149, 227, 82, 50, 159, 35]);
pub const CLSID_HorizontalIndicatorContent: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3889270005, 16071, 17621, [167, 107, 55, 112, 243, 207, 144, 61]);
pub const CLSID_VerticalIndicatorContent: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2701877015, 45024, 19106, [145, 233, 62, 112, 1, 210, 230, 180]);
pub const CLSID_VirtualViewportContent: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(839295386, 34544, 19636, [167, 243, 22, 227, 183, 226, 216, 82]);
pub const DCompManipulationCompositor: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2044634663, 41098, 17324, [142, 245, 105, 0, 185, 41, 145, 38]);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(pub i32);
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_STOP: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(0i32);
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_FORWARD: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(1i32);
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_REVERSE: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(2i32);
impl ::std::convert::From<i32> for DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_CONFIGURATION(pub i32);
pub const DIRECTMANIPULATION_CONFIGURATION_NONE: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(0i32);
pub const DIRECTMANIPULATION_CONFIGURATION_INTERACTION: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(1i32);
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_X: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(2i32);
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_Y: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(4i32);
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(16i32);
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_INERTIA: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(32i32);
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING_INERTIA: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(128i32);
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_X: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(256i32);
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_Y: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(512i32);
impl ::std::convert::From<i32> for DIRECTMANIPULATION_CONFIGURATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTMANIPULATION_CONFIGURATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(pub i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_VERTICAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(1i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HORIZONTAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(2i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_ONLY: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(16i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(32i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HOLD_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(64i32);
impl ::std::convert::From<i32> for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_DRAG_DROP_STATUS(pub i32);
pub const DIRECTMANIPULATION_DRAG_DROP_READY: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(0i32);
pub const DIRECTMANIPULATION_DRAG_DROP_PRESELECT: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(1i32);
pub const DIRECTMANIPULATION_DRAG_DROP_SELECTING: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(2i32);
pub const DIRECTMANIPULATION_DRAG_DROP_DRAGGING: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(3i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CANCELLED: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(4i32);
pub const DIRECTMANIPULATION_DRAG_DROP_COMMITTED: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(5i32);
impl ::std::convert::From<i32> for DIRECTMANIPULATION_DRAG_DROP_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTMANIPULATION_DRAG_DROP_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_GESTURE_CONFIGURATION(pub i32);
pub const DIRECTMANIPULATION_GESTURE_NONE: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(0i32);
pub const DIRECTMANIPULATION_GESTURE_DEFAULT: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(0i32);
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_VERTICAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(8i32);
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_HORIZONTAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(16i32);
pub const DIRECTMANIPULATION_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(32i32);
impl ::std::convert::From<i32> for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_HITTEST_TYPE(pub i32);
pub const DIRECTMANIPULATION_HITTEST_TYPE_ASYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = DIRECTMANIPULATION_HITTEST_TYPE(0i32);
pub const DIRECTMANIPULATION_HITTEST_TYPE_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = DIRECTMANIPULATION_HITTEST_TYPE(1i32);
pub const DIRECTMANIPULATION_HITTEST_TYPE_AUTO_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = DIRECTMANIPULATION_HITTEST_TYPE(2i32);
impl ::std::convert::From<i32> for DIRECTMANIPULATION_HITTEST_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTMANIPULATION_HITTEST_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_HORIZONTALALIGNMENT(pub i32);
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_NONE: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(0i32);
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_LEFT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(1i32);
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_CENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(2i32);
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_RIGHT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(4i32);
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(8i32);
impl ::std::convert::From<i32> for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_INPUT_MODE(pub i32);
pub const DIRECTMANIPULATION_INPUT_MODE_AUTOMATIC: DIRECTMANIPULATION_INPUT_MODE = DIRECTMANIPULATION_INPUT_MODE(0i32);
pub const DIRECTMANIPULATION_INPUT_MODE_MANUAL: DIRECTMANIPULATION_INPUT_MODE = DIRECTMANIPULATION_INPUT_MODE(1i32);
impl ::std::convert::From<i32> for DIRECTMANIPULATION_INPUT_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTMANIPULATION_INPUT_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_INTERACTION_TYPE(pub i32);
pub const DIRECTMANIPULATION_INTERACTION_BEGIN: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(0i32);
pub const DIRECTMANIPULATION_INTERACTION_TYPE_MANIPULATION: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(1i32);
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_TAP: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(2i32);
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_HOLD: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(3i32);
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_CROSS_SLIDE: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(4i32);
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(5i32);
pub const DIRECTMANIPULATION_INTERACTION_END: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(100i32);
impl ::std::convert::From<i32> for DIRECTMANIPULATION_INTERACTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTMANIPULATION_INTERACTION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
pub const DIRECTMANIPULATION_KEYBOARDFOCUS: u32 = 4294967294u32;
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_MOTION_TYPES(pub i32);
pub const DIRECTMANIPULATION_MOTION_NONE: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(0i32);
pub const DIRECTMANIPULATION_MOTION_TRANSLATEX: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(1i32);
pub const DIRECTMANIPULATION_MOTION_TRANSLATEY: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(2i32);
pub const DIRECTMANIPULATION_MOTION_ZOOM: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(4i32);
pub const DIRECTMANIPULATION_MOTION_CENTERX: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(16i32);
pub const DIRECTMANIPULATION_MOTION_CENTERY: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(32i32);
pub const DIRECTMANIPULATION_MOTION_ALL: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(55i32);
impl ::std::convert::From<i32> for DIRECTMANIPULATION_MOTION_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTMANIPULATION_MOTION_TYPES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
pub const DIRECTMANIPULATION_MOUSEFOCUS: u32 = 4294967293u32;
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_SNAPPOINT_COORDINATE(pub i32);
pub const DIRECTMANIPULATION_COORDINATE_BOUNDARY: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = DIRECTMANIPULATION_SNAPPOINT_COORDINATE(0i32);
pub const DIRECTMANIPULATION_COORDINATE_ORIGIN: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = DIRECTMANIPULATION_SNAPPOINT_COORDINATE(1i32);
pub const DIRECTMANIPULATION_COORDINATE_MIRRORED: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = DIRECTMANIPULATION_SNAPPOINT_COORDINATE(16i32);
impl ::std::convert::From<i32> for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_SNAPPOINT_TYPE(pub i32);
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(0i32);
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(1i32);
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(2i32);
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(3i32);
impl ::std::convert::From<i32> for DIRECTMANIPULATION_SNAPPOINT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTMANIPULATION_SNAPPOINT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_STATUS(pub i32);
pub const DIRECTMANIPULATION_BUILDING: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(0i32);
pub const DIRECTMANIPULATION_ENABLED: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(1i32);
pub const DIRECTMANIPULATION_DISABLED: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(2i32);
pub const DIRECTMANIPULATION_RUNNING: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(3i32);
pub const DIRECTMANIPULATION_INERTIA: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(4i32);
pub const DIRECTMANIPULATION_READY: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(5i32);
pub const DIRECTMANIPULATION_SUSPENDED: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(6i32);
impl ::std::convert::From<i32> for DIRECTMANIPULATION_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTMANIPULATION_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_VERTICALALIGNMENT(pub i32);
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_NONE: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(0i32);
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_TOP: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(1i32);
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_CENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(2i32);
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_BOTTOM: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(4i32);
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(8i32);
impl ::std::convert::From<i32> for DIRECTMANIPULATION_VERTICALALIGNMENT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTMANIPULATION_VERTICALALIGNMENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_VIEWPORT_OPTIONS(pub i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DEFAULT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(0i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_AUTODISABLE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(1i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_MANUALUPDATE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(2i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_INPUT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(4i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_EXPLICITHITTEST: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(8i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DISABLEPIXELSNAPPING: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(16i32);
impl ::std::convert::From<i32> for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    type Abi = Self;
}
pub const DirectManipulationManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1424101814, 13904, 20341, [131, 52, 250, 53, 149, 152, 225, 197]);
pub const DirectManipulationPrimaryContent: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3399493217, 54686, 16839, [131, 147, 59, 163, 186, 203, 107, 87]);
pub const DirectManipulationSharedManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2574856838, 30668, 19287, [150, 219, 59, 53, 79, 111, 159, 181]);
pub const DirectManipulationUpdateManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2680274901, 6197, 17434, [179, 177, 182, 204, 116, 183, 39, 208]);
pub const DirectManipulationViewport: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(887230902, 13904, 20341, [131, 52, 250, 53, 149, 152, 225, 197]);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationAutoScrollBehavior(::windows::runtime::IUnknown);
impl IDirectManipulationAutoScrollBehavior {
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetConfiguration(&self, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(motiontypes), ::std::mem::transmute(scrollmotion)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationAutoScrollBehavior {
    type Vtable = IDirectManipulationAutoScrollBehavior_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1834570964, 8195, 17238, [155, 49, 208, 81, 201, 255, 10, 247]);
}
impl ::std::convert::From<IDirectManipulationAutoScrollBehavior> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationAutoScrollBehavior) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationAutoScrollBehavior> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationAutoScrollBehavior) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationAutoScrollBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationAutoScrollBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationAutoScrollBehavior_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationCompositor(::windows::runtime::IUnknown);
impl IDirectManipulationCompositor {
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn AddContent<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationContent>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, content: Param0, device: Param1, parentvisual: Param2, childvisual: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), content.into_param().abi(), device.into_param().abi(), parentvisual.into_param().abi(), childvisual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn RemoveContent<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetUpdateManager<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationUpdateManager>>(&self, updatemanager: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), updatemanager.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn Flush(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationCompositor {
    type Vtable = IDirectManipulationCompositor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1400506405, 903, 20218, [182, 47, 113, 235, 31, 8, 90, 126]);
}
impl ::std::convert::From<IDirectManipulationCompositor> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationCompositor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationCompositor> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationCompositor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationCompositor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationCompositor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationCompositor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, content: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr, parentvisual: ::windows::runtime::RawPtr, childvisual: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, content: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updatemanager: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationCompositor2(::windows::runtime::IUnknown);
impl IDirectManipulationCompositor2 {
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn AddContent<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationContent>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, content: Param0, device: Param1, parentvisual: Param2, childvisual: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), content.into_param().abi(), device.into_param().abi(), parentvisual.into_param().abi(), childvisual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn RemoveContent<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetUpdateManager<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationUpdateManager>>(&self, updatemanager: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), updatemanager.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn Flush(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn AddContentWithCrossProcessChaining<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationPrimaryContent>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, content: Param0, device: Param1, parentvisual: Param2, childvisual: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), content.into_param().abi(), device.into_param().abi(), parentvisual.into_param().abi(), childvisual.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationCompositor2 {
    type Vtable = IDirectManipulationCompositor2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3549198370, 61899, 17355, [180, 185, 172, 12, 118, 122, 65, 46]);
}
impl ::std::convert::From<IDirectManipulationCompositor2> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationCompositor2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationCompositor2> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationCompositor2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationCompositor2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationCompositor2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IDirectManipulationCompositor2> for IDirectManipulationCompositor {
    fn from(value: IDirectManipulationCompositor2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationCompositor2> for IDirectManipulationCompositor {
    fn from(value: &IDirectManipulationCompositor2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectManipulationCompositor> for IDirectManipulationCompositor2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectManipulationCompositor> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDirectManipulationCompositor>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectManipulationCompositor> for &IDirectManipulationCompositor2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectManipulationCompositor> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDirectManipulationCompositor>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationCompositor2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, content: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr, parentvisual: ::windows::runtime::RawPtr, childvisual: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, content: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updatemanager: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, content: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr, parentvisual: ::windows::runtime::RawPtr, childvisual: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationContent(::windows::runtime::IUnknown);
impl IDirectManipulationContent {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn GetContentRect(&self) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn SetContentRect(&self, contentsize: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(contentsize)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetViewport<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetTag(&self, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void, id: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(object), ::std::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), object.into_param().abi(), ::std::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetOutputTransform(&self, matrix: *mut f32, pointcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(matrix), ::std::mem::transmute(pointcount)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetContentTransform(&self, matrix: *mut f32, pointcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(matrix), ::std::mem::transmute(pointcount)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SyncContentTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(matrix), ::std::mem::transmute(pointcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationContent {
    type Vtable = IDirectManipulationContent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3097060043, 15753, 17451, [187, 88, 80, 152, 250, 15, 159, 22]);
}
impl ::std::convert::From<IDirectManipulationContent> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationContent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationContent> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationContent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationContent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentsize: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentsize: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void, id: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, id: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrix: *mut f32, pointcount: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrix: *mut f32, pointcount: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrix: *const f32, pointcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationDeferContactService(::windows::runtime::IUnknown);
impl IDirectManipulationDeferContactService {
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn DeferContact(&self, pointerid: u32, timeout: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pointerid), ::std::mem::transmute(timeout)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn CancelContact(&self, pointerid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pointerid)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn CancelDeferral(&self, pointerid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pointerid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationDeferContactService {
    type Vtable = IDirectManipulationDeferContactService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1697471601, 65120, 19096, [190, 112, 229, 242, 18, 145, 231, 241]);
}
impl ::std::convert::From<IDirectManipulationDeferContactService> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationDeferContactService) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationDeferContactService> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationDeferContactService) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationDeferContactService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationDeferContactService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDeferContactService_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerid: u32, timeout: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerid: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationDragDropBehavior(::windows::runtime::IUnknown);
impl IDirectManipulationDragDropBehavior {
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetConfiguration(&self, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<DIRECTMANIPULATION_DRAG_DROP_STATUS> {
        let mut result__: <DIRECTMANIPULATION_DRAG_DROP_STATUS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DIRECTMANIPULATION_DRAG_DROP_STATUS>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationDragDropBehavior {
    type Vtable = IDirectManipulationDragDropBehavior_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2169199349, 49864, 17008, [169, 183, 161, 152, 206, 141, 2, 250]);
}
impl ::std::convert::From<IDirectManipulationDragDropBehavior> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationDragDropBehavior) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationDragDropBehavior> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationDragDropBehavior) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationDragDropBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationDragDropBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDragDropBehavior_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: *mut DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationDragDropEventHandler(::windows::runtime::IUnknown);
impl IDirectManipulationDragDropEventHandler {
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn OnDragDropStatusChange<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationViewport2>>(&self, viewport: Param0, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), viewport.into_param().abi(), ::std::mem::transmute(current), ::std::mem::transmute(previous)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationDragDropEventHandler {
    type Vtable = IDirectManipulationDragDropEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(530651920, 28699, 16814, [181, 242, 73, 227, 107, 213, 149, 170]);
}
impl ::std::convert::From<IDirectManipulationDragDropEventHandler> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationDragDropEventHandler) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationDragDropEventHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationDragDropEventHandler) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationDragDropEventHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationDragDropEventHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDragDropEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewport: ::windows::runtime::RawPtr, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationFrameInfoProvider(::windows::runtime::IUnknown);
impl IDirectManipulationFrameInfoProvider {
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetNextFrameInfo(&self, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(time), ::std::mem::transmute(processtime), ::std::mem::transmute(compositiontime)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationFrameInfoProvider {
    type Vtable = IDirectManipulationFrameInfoProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218789306, 28492, 19457, [135, 78, 25, 200, 160, 89, 7, 249]);
}
impl ::std::convert::From<IDirectManipulationFrameInfoProvider> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationFrameInfoProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationFrameInfoProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationFrameInfoProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationFrameInfoProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationFrameInfoProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationFrameInfoProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationInteractionEventHandler(::windows::runtime::IUnknown);
impl IDirectManipulationInteractionEventHandler {
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn OnInteraction<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationViewport2>>(&self, viewport: Param0, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), viewport.into_param().abi(), ::std::mem::transmute(interaction)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationInteractionEventHandler {
    type Vtable = IDirectManipulationInteractionEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3829351864, 17076, 16446, [177, 242, 39, 59, 143, 81, 8, 48]);
}
impl ::std::convert::From<IDirectManipulationInteractionEventHandler> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationInteractionEventHandler) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationInteractionEventHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationInteractionEventHandler) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationInteractionEventHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationInteractionEventHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationInteractionEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewport: ::windows::runtime::RawPtr, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationManager(::windows::runtime::IUnknown);
impl IDirectManipulationManager {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn Deactivate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn RegisterHitTestTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0, hittestwindow: Param1, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), window.into_param().abi(), hittestwindow.into_param().abi(), ::std::mem::transmute(r#type)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(message), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetUpdateManager<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn CreateViewport<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationFrameInfoProvider>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, frameinfo: Param0, window: Param1) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), frameinfo.into_param().abi(), window.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn CreateContent<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationFrameInfoProvider>, T: ::windows::runtime::Interface>(&self, frameinfo: Param0, clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), frameinfo.into_param().abi(), ::std::mem::transmute(clsid), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationManager {
    type Vtable = IDirectManipulationManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4227191732, 28871, 16739, [147, 34, 90, 111, 102, 13, 111, 188]);
}
impl ::std::convert::From<IDirectManipulationManager> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationManager> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frameinfo: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frameinfo: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationManager2(::windows::runtime::IUnknown);
impl IDirectManipulationManager2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn Deactivate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn RegisterHitTestTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0, hittestwindow: Param1, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), window.into_param().abi(), hittestwindow.into_param().abi(), ::std::mem::transmute(r#type)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(message), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetUpdateManager<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn CreateViewport<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationFrameInfoProvider>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, frameinfo: Param0, window: Param1) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), frameinfo.into_param().abi(), window.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn CreateContent<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationFrameInfoProvider>, T: ::windows::runtime::Interface>(&self, frameinfo: Param0, clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), frameinfo.into_param().abi(), ::std::mem::transmute(clsid), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn CreateBehavior<T: ::windows::runtime::Interface>(&self, clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(clsid), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationManager2 {
    type Vtable = IDirectManipulationManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4195354089, 15638, 18508, [191, 201, 98, 182, 30, 86, 236, 78]);
}
impl ::std::convert::From<IDirectManipulationManager2> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationManager2> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IDirectManipulationManager2> for IDirectManipulationManager {
    fn from(value: IDirectManipulationManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationManager2> for IDirectManipulationManager {
    fn from(value: &IDirectManipulationManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectManipulationManager> for IDirectManipulationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectManipulationManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDirectManipulationManager>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectManipulationManager> for &IDirectManipulationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectManipulationManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDirectManipulationManager>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frameinfo: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frameinfo: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationManager3(::windows::runtime::IUnknown);
impl IDirectManipulationManager3 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn Deactivate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn RegisterHitTestTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0, hittestwindow: Param1, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), window.into_param().abi(), hittestwindow.into_param().abi(), ::std::mem::transmute(r#type)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(message), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetUpdateManager<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn CreateViewport<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationFrameInfoProvider>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, frameinfo: Param0, window: Param1) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), frameinfo.into_param().abi(), window.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn CreateContent<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationFrameInfoProvider>, T: ::windows::runtime::Interface>(&self, frameinfo: Param0, clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), frameinfo.into_param().abi(), ::std::mem::transmute(clsid), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn CreateBehavior<T: ::windows::runtime::Interface>(&self, clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(clsid), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetService<T: ::windows::runtime::Interface>(&self, clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(clsid), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationManager3 {
    type Vtable = IDirectManipulationManager3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(750170941, 65512, 18572, [183, 80, 251, 223, 232, 141, 202, 140]);
}
impl ::std::convert::From<IDirectManipulationManager3> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationManager3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationManager3> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationManager3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IDirectManipulationManager3> for IDirectManipulationManager2 {
    fn from(value: IDirectManipulationManager3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationManager3> for IDirectManipulationManager2 {
    fn from(value: &IDirectManipulationManager3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectManipulationManager2> for IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectManipulationManager2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDirectManipulationManager2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectManipulationManager2> for &IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectManipulationManager2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDirectManipulationManager2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IDirectManipulationManager3> for IDirectManipulationManager {
    fn from(value: IDirectManipulationManager3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationManager3> for IDirectManipulationManager {
    fn from(value: &IDirectManipulationManager3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectManipulationManager> for IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectManipulationManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDirectManipulationManager>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectManipulationManager> for &IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectManipulationManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDirectManipulationManager>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frameinfo: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frameinfo: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationPrimaryContent(::windows::runtime::IUnknown);
impl IDirectManipulationPrimaryContent {
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetSnapInterval(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(motion), ::std::mem::transmute(interval), ::std::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetSnapPoints(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(motion), ::std::mem::transmute(points), ::std::mem::transmute(pointcount)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetSnapType(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(motion), ::std::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetSnapCoordinate(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(motion), ::std::mem::transmute(coordinate), ::std::mem::transmute(origin)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetZoomBoundaries(&self, zoomminimum: f32, zoommaximum: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(zoomminimum), ::std::mem::transmute(zoommaximum)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetHorizontalAlignment(&self, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(alignment)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetVerticalAlignment(&self, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(alignment)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetInertiaEndTransform(&self, matrix: *mut f32, pointcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(matrix), ::std::mem::transmute(pointcount)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetCenterPoint(&self, centerx: *mut f32, centery: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(centerx), ::std::mem::transmute(centery)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationPrimaryContent {
    type Vtable = IDirectManipulationPrimaryContent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3240645092, 5784, 17957, [185, 177, 124, 163, 236, 24, 99, 11]);
}
impl ::std::convert::From<IDirectManipulationPrimaryContent> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationPrimaryContent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationPrimaryContent> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationPrimaryContent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationPrimaryContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationPrimaryContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationPrimaryContent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, zoomminimum: f32, zoommaximum: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrix: *mut f32, pointcount: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, centerx: *mut f32, centery: *mut f32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationUpdateHandler(::windows::runtime::IUnknown);
impl IDirectManipulationUpdateHandler {
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn Update(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationUpdateHandler {
    type Vtable = IDirectManipulationUpdateHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2030789431, 25848, 20469, [162, 105, 179, 43, 194, 175, 39, 167]);
}
impl ::std::convert::From<IDirectManipulationUpdateHandler> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationUpdateHandler) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationUpdateHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationUpdateHandler) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationUpdateHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationUpdateHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationUpdateHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationUpdateManager(::windows::runtime::IUnknown);
impl IDirectManipulationUpdateManager {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn RegisterWaitHandleCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, IDirectManipulationUpdateHandler>>(&self, handle: Param0, eventhandler: Param1) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), handle.into_param().abi(), eventhandler.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn UnregisterWaitHandleCallback(&self, cookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cookie)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn Update<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationFrameInfoProvider>>(&self, frameinfo: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), frameinfo.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationUpdateManager {
    type Vtable = IDirectManipulationUpdateManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2964218621, 48692, 18151, [156, 170, 211, 97, 250, 203, 185, 204]);
}
impl ::std::convert::From<IDirectManipulationUpdateManager> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationUpdateManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationUpdateManager> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationUpdateManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationUpdateManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationUpdateManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationUpdateManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handle: super::super::Foundation::HANDLE, eventhandler: ::windows::runtime::RawPtr, cookie: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frameinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationViewport(::windows::runtime::IUnknown);
impl IDirectManipulationViewport {
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn Enable(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn Disable(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetContact(&self, pointerid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pointerid)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn ReleaseContact(&self, pointerid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pointerid)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn ReleaseAllContacts(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<DIRECTMANIPULATION_STATUS> {
        let mut result__: <DIRECTMANIPULATION_STATUS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DIRECTMANIPULATION_STATUS>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetTag(&self, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void, id: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(object), ::std::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), object.into_param().abi(), ::std::mem::transmute(id)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn GetViewportRect(&self) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn SetViewportRect(&self, viewport: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(viewport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn ZoomToRect<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, left: f32, top: f32, right: f32, bottom: f32, animate: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(left), ::std::mem::transmute(top), ::std::mem::transmute(right), ::std::mem::transmute(bottom), animate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetViewportTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(matrix), ::std::mem::transmute(pointcount)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SyncDisplayTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(matrix), ::std::mem::transmute(pointcount)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetPrimaryContent<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn AddContent<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn RemoveContent<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetViewportOptions(&self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(options)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn AddConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn RemoveConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn ActivateConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetManualGesture(&self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetChaining(&self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabledtypes)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn AddEventHandler<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, IDirectManipulationViewportEventHandler>>(&self, window: Param0, eventhandler: Param1) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), window.into_param().abi(), eventhandler.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn RemoveEventHandler(&self, cookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(cookie)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetInputMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetUpdateMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn Abandon(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationViewport {
    type Vtable = IDirectManipulationViewport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(683170365, 24736, 18621, [155, 161, 92, 232, 217, 234, 58, 109]);
}
impl ::std::convert::From<IDirectManipulationViewport> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationViewport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationViewport> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationViewport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationViewport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationViewport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: *mut DIRECTMANIPULATION_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void, id: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, id: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewport: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewport: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrix: *const f32, pointcount: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrix: *const f32, pointcount: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, content: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, content: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND, eventhandler: ::windows::runtime::RawPtr, cookie: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationViewport2(::windows::runtime::IUnknown);
impl IDirectManipulationViewport2 {
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn Enable(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn Disable(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetContact(&self, pointerid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pointerid)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn ReleaseContact(&self, pointerid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pointerid)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn ReleaseAllContacts(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<DIRECTMANIPULATION_STATUS> {
        let mut result__: <DIRECTMANIPULATION_STATUS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DIRECTMANIPULATION_STATUS>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetTag(&self, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void, id: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(object), ::std::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), object.into_param().abi(), ::std::mem::transmute(id)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn GetViewportRect(&self) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn SetViewportRect(&self, viewport: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(viewport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn ZoomToRect<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, left: f32, top: f32, right: f32, bottom: f32, animate: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(left), ::std::mem::transmute(top), ::std::mem::transmute(right), ::std::mem::transmute(bottom), animate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetViewportTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(matrix), ::std::mem::transmute(pointcount)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SyncDisplayTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(matrix), ::std::mem::transmute(pointcount)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn GetPrimaryContent<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn AddContent<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn RemoveContent<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetViewportOptions(&self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(options)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn AddConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn RemoveConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn ActivateConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetManualGesture(&self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetChaining(&self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabledtypes)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`, `Win32_Foundation`*"]
    pub unsafe fn AddEventHandler<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, IDirectManipulationViewportEventHandler>>(&self, window: Param0, eventhandler: Param1) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), window.into_param().abi(), eventhandler.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn RemoveEventHandler(&self, cookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(cookie)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetInputMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn SetUpdateMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn Abandon(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn AddBehavior<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, behavior: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), behavior.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn RemoveBehavior(&self, cookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(cookie)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn RemoveAllBehaviors(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationViewport2 {
    type Vtable = IDirectManipulationViewport2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2453457580, 25057, 17285, [183, 38, 1, 122, 241, 137, 136, 42]);
}
impl ::std::convert::From<IDirectManipulationViewport2> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationViewport2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationViewport2> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationViewport2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationViewport2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationViewport2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IDirectManipulationViewport2> for IDirectManipulationViewport {
    fn from(value: IDirectManipulationViewport2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationViewport2> for IDirectManipulationViewport {
    fn from(value: &IDirectManipulationViewport2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectManipulationViewport> for IDirectManipulationViewport2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectManipulationViewport> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDirectManipulationViewport>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectManipulationViewport> for &IDirectManipulationViewport2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectManipulationViewport> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDirectManipulationViewport>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewport2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: *mut DIRECTMANIPULATION_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void, id: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, id: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewport: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewport: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrix: *const f32, pointcount: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matrix: *const f32, pointcount: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, object: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, content: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, content: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND, eventhandler: ::windows::runtime::RawPtr, cookie: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, behavior: ::windows::runtime::RawPtr, cookie: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectManipulationViewportEventHandler(::windows::runtime::IUnknown);
impl IDirectManipulationViewportEventHandler {
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn OnViewportStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationViewport>>(&self, viewport: Param0, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), viewport.into_param().abi(), ::std::mem::transmute(current), ::std::mem::transmute(previous)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn OnViewportUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationViewport>>(&self, viewport: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), viewport.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectManipulation`*"]
    pub unsafe fn OnContentUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectManipulationViewport>, Param1: ::windows::runtime::IntoParam<'a, IDirectManipulationContent>>(&self, viewport: Param0, content: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), viewport.into_param().abi(), content.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectManipulationViewportEventHandler {
    type Vtable = IDirectManipulationViewportEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2501976538, 54943, 17913, [176, 249, 242, 57, 68, 50, 26, 109]);
}
impl ::std::convert::From<IDirectManipulationViewportEventHandler> for ::windows::runtime::IUnknown {
    fn from(value: IDirectManipulationViewportEventHandler) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectManipulationViewportEventHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectManipulationViewportEventHandler) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectManipulationViewportEventHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectManipulationViewportEventHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewportEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewport: ::windows::runtime::RawPtr, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewport: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewport: ::windows::runtime::RawPtr, content: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
