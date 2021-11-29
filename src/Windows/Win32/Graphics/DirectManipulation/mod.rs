#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_AutoScrollBehavior: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26126a51_3c70_4c9a_aec2_948849eeb093);
pub const CLSID_DeferContactService: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7b67cf4_84bb_434e_86ae_6592bbc9abd9);
pub const CLSID_DragDropConfigurationBehavior: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09b01b3e_ba6c_454d_82e8_95e352329f23);
pub const CLSID_HorizontalIndicatorContent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7d18cf5_3ec7_44d5_a76b_3770f3cf903d);
pub const CLSID_VerticalIndicatorContent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa10b5f17_afe0_4aa2_91e9_3e7001d2e6b4);
pub const CLSID_VirtualViewportContent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3206a19a_86f0_4cb4_a7f3_16e3b7e2d852);
pub const DCompManipulationCompositor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79dea627_a08a_43ac_8ef5_6900b9299126);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(pub i32);
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_STOP: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(0i32);
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_FORWARD: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(1i32);
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_REVERSE: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(2i32);
impl ::core::convert::From<i32> for DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for DIRECTMANIPULATION_CONFIGURATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_CONFIGURATION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(pub i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_VERTICAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(1i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HORIZONTAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(2i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_ONLY: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(16i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(32i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HOLD_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(64i32);
impl ::core::convert::From<i32> for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_DRAG_DROP_STATUS(pub i32);
pub const DIRECTMANIPULATION_DRAG_DROP_READY: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(0i32);
pub const DIRECTMANIPULATION_DRAG_DROP_PRESELECT: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(1i32);
pub const DIRECTMANIPULATION_DRAG_DROP_SELECTING: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(2i32);
pub const DIRECTMANIPULATION_DRAG_DROP_DRAGGING: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(3i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CANCELLED: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(4i32);
pub const DIRECTMANIPULATION_DRAG_DROP_COMMITTED: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(5i32);
impl ::core::convert::From<i32> for DIRECTMANIPULATION_DRAG_DROP_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_DRAG_DROP_STATUS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_GESTURE_CONFIGURATION(pub i32);
pub const DIRECTMANIPULATION_GESTURE_NONE: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(0i32);
pub const DIRECTMANIPULATION_GESTURE_DEFAULT: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(0i32);
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_VERTICAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(8i32);
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_HORIZONTAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(16i32);
pub const DIRECTMANIPULATION_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(32i32);
impl ::core::convert::From<i32> for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_HITTEST_TYPE(pub i32);
pub const DIRECTMANIPULATION_HITTEST_TYPE_ASYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = DIRECTMANIPULATION_HITTEST_TYPE(0i32);
pub const DIRECTMANIPULATION_HITTEST_TYPE_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = DIRECTMANIPULATION_HITTEST_TYPE(1i32);
pub const DIRECTMANIPULATION_HITTEST_TYPE_AUTO_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = DIRECTMANIPULATION_HITTEST_TYPE(2i32);
impl ::core::convert::From<i32> for DIRECTMANIPULATION_HITTEST_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_HITTEST_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_HORIZONTALALIGNMENT(pub i32);
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_NONE: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(0i32);
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_LEFT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(1i32);
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_CENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(2i32);
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_RIGHT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(4i32);
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(8i32);
impl ::core::convert::From<i32> for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_INPUT_MODE(pub i32);
pub const DIRECTMANIPULATION_INPUT_MODE_AUTOMATIC: DIRECTMANIPULATION_INPUT_MODE = DIRECTMANIPULATION_INPUT_MODE(0i32);
pub const DIRECTMANIPULATION_INPUT_MODE_MANUAL: DIRECTMANIPULATION_INPUT_MODE = DIRECTMANIPULATION_INPUT_MODE(1i32);
impl ::core::convert::From<i32> for DIRECTMANIPULATION_INPUT_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_INPUT_MODE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_INTERACTION_TYPE(pub i32);
pub const DIRECTMANIPULATION_INTERACTION_BEGIN: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(0i32);
pub const DIRECTMANIPULATION_INTERACTION_TYPE_MANIPULATION: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(1i32);
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_TAP: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(2i32);
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_HOLD: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(3i32);
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_CROSS_SLIDE: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(4i32);
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(5i32);
pub const DIRECTMANIPULATION_INTERACTION_END: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(100i32);
impl ::core::convert::From<i32> for DIRECTMANIPULATION_INTERACTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_INTERACTION_TYPE {
    type Abi = Self;
}
pub const DIRECTMANIPULATION_KEYBOARDFOCUS: u32 = 4294967294u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_MOTION_TYPES(pub i32);
pub const DIRECTMANIPULATION_MOTION_NONE: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(0i32);
pub const DIRECTMANIPULATION_MOTION_TRANSLATEX: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(1i32);
pub const DIRECTMANIPULATION_MOTION_TRANSLATEY: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(2i32);
pub const DIRECTMANIPULATION_MOTION_ZOOM: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(4i32);
pub const DIRECTMANIPULATION_MOTION_CENTERX: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(16i32);
pub const DIRECTMANIPULATION_MOTION_CENTERY: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(32i32);
pub const DIRECTMANIPULATION_MOTION_ALL: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(55i32);
impl ::core::convert::From<i32> for DIRECTMANIPULATION_MOTION_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_MOTION_TYPES {
    type Abi = Self;
}
pub const DIRECTMANIPULATION_MOUSEFOCUS: u32 = 4294967293u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_SNAPPOINT_COORDINATE(pub i32);
pub const DIRECTMANIPULATION_COORDINATE_BOUNDARY: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = DIRECTMANIPULATION_SNAPPOINT_COORDINATE(0i32);
pub const DIRECTMANIPULATION_COORDINATE_ORIGIN: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = DIRECTMANIPULATION_SNAPPOINT_COORDINATE(1i32);
pub const DIRECTMANIPULATION_COORDINATE_MIRRORED: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = DIRECTMANIPULATION_SNAPPOINT_COORDINATE(16i32);
impl ::core::convert::From<i32> for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_SNAPPOINT_TYPE(pub i32);
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(0i32);
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(1i32);
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(2i32);
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(3i32);
impl ::core::convert::From<i32> for DIRECTMANIPULATION_SNAPPOINT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_SNAPPOINT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_STATUS(pub i32);
pub const DIRECTMANIPULATION_BUILDING: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(0i32);
pub const DIRECTMANIPULATION_ENABLED: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(1i32);
pub const DIRECTMANIPULATION_DISABLED: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(2i32);
pub const DIRECTMANIPULATION_RUNNING: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(3i32);
pub const DIRECTMANIPULATION_INERTIA: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(4i32);
pub const DIRECTMANIPULATION_READY: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(5i32);
pub const DIRECTMANIPULATION_SUSPENDED: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(6i32);
impl ::core::convert::From<i32> for DIRECTMANIPULATION_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_STATUS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_VERTICALALIGNMENT(pub i32);
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_NONE: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(0i32);
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_TOP: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(1i32);
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_CENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(2i32);
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_BOTTOM: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(4i32);
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(8i32);
impl ::core::convert::From<i32> for DIRECTMANIPULATION_VERTICALALIGNMENT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_VERTICALALIGNMENT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTMANIPULATION_VIEWPORT_OPTIONS(pub i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DEFAULT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(0i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_AUTODISABLE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(1i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_MANUALUPDATE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(2i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_INPUT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(4i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_EXPLICITHITTEST: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(8i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DISABLEPIXELSNAPPING: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(16i32);
impl ::core::convert::From<i32> for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    type Abi = Self;
}
pub const DirectManipulationManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54e211b6_3650_4f75_8334_fa359598e1c5);
pub const DirectManipulationPrimaryContent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaa02661_d59e_41c7_8393_3ba3bacb6b57);
pub const DirectManipulationSharedManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99793286_77cc_4b57_96db_3b354f6f9fb5);
pub const DirectManipulationUpdateManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fc1bfd5_1835_441a_b3b1_b6cc74b727d0);
pub const DirectManipulationViewport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34e211b6_3650_4f75_8334_fa359598e1c5);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationAutoScrollBehavior(pub ::windows::core::IUnknown);
impl IDirectManipulationAutoScrollBehavior {
    pub unsafe fn SetConfiguration(&self, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(motiontypes), ::core::mem::transmute(scrollmotion)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationAutoScrollBehavior {
    type Vtable = IDirectManipulationAutoScrollBehavior_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d5954d4_2003_4356_9b31_d051c9ff0af7);
}
impl ::core::convert::From<IDirectManipulationAutoScrollBehavior> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationAutoScrollBehavior) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationAutoScrollBehavior> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationAutoScrollBehavior) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationAutoScrollBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationAutoScrollBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationAutoScrollBehavior_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationCompositor(pub ::windows::core::IUnknown);
impl IDirectManipulationCompositor {
    pub unsafe fn AddContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0, device: Param1, parentvisual: Param2, childvisual: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), content.into_param().abi(), device.into_param().abi(), parentvisual.into_param().abi(), childvisual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    pub unsafe fn SetUpdateManager<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationUpdateManager>>(&self, updatemanager: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), updatemanager.into_param().abi()).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationCompositor {
    type Vtable = IDirectManipulationCompositor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x537a0825_0387_4efa_b62f_71eb1f085a7e);
}
impl ::core::convert::From<IDirectManipulationCompositor> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationCompositor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationCompositor> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationCompositor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationCompositor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationCompositor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationCompositor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr, device: ::windows::core::RawPtr, parentvisual: ::windows::core::RawPtr, childvisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, updatemanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationCompositor2(pub ::windows::core::IUnknown);
impl IDirectManipulationCompositor2 {
    pub unsafe fn AddContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0, device: Param1, parentvisual: Param2, childvisual: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), content.into_param().abi(), device.into_param().abi(), parentvisual.into_param().abi(), childvisual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    pub unsafe fn SetUpdateManager<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationUpdateManager>>(&self, updatemanager: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), updatemanager.into_param().abi()).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn AddContentWithCrossProcessChaining<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationPrimaryContent>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0, device: Param1, parentvisual: Param2, childvisual: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), content.into_param().abi(), device.into_param().abi(), parentvisual.into_param().abi(), childvisual.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationCompositor2 {
    type Vtable = IDirectManipulationCompositor2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd38c7822_f1cb_43cb_b4b9_ac0c767a412e);
}
impl ::core::convert::From<IDirectManipulationCompositor2> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationCompositor2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationCompositor2> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationCompositor2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationCompositor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationCompositor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDirectManipulationCompositor> for &IDirectManipulationCompositor2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectManipulationCompositor> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationCompositor2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr, device: ::windows::core::RawPtr, parentvisual: ::windows::core::RawPtr, childvisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, updatemanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr, device: ::windows::core::RawPtr, parentvisual: ::windows::core::RawPtr, childvisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationContent(pub ::windows::core::IUnknown);
impl IDirectManipulationContent {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetContentRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContentRect(&self, contentsize: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(contentsize)).ok()
    }
    pub unsafe fn GetViewport<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetTag(&self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    pub unsafe fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    pub unsafe fn GetOutputTransform(&self, matrix: *mut f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(pointcount)).ok()
    }
    pub unsafe fn GetContentTransform(&self, matrix: *mut f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(pointcount)).ok()
    }
    pub unsafe fn SyncContentTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(pointcount)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationContent {
    type Vtable = IDirectManipulationContent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb89962cb_3d89_442b_bb58_5098fa0f9f16);
}
impl ::core::convert::From<IDirectManipulationContent> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationContent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationContent> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationContent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationContent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contentsize: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contentsize: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, object: ::windows::core::RawPtr, id: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationDeferContactService(pub ::windows::core::IUnknown);
impl IDirectManipulationDeferContactService {
    pub unsafe fn DeferContact(&self, pointerid: u32, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid), ::core::mem::transmute(timeout)).ok()
    }
    pub unsafe fn CancelContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
    pub unsafe fn CancelDeferral(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationDeferContactService {
    type Vtable = IDirectManipulationDeferContactService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x652d5c71_fe60_4a98_be70_e5f21291e7f1);
}
impl ::core::convert::From<IDirectManipulationDeferContactService> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationDeferContactService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationDeferContactService> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationDeferContactService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationDeferContactService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationDeferContactService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDeferContactService_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pointerid: u32, timeout: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pointerid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pointerid: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationDragDropBehavior(pub ::windows::core::IUnknown);
impl IDirectManipulationDragDropBehavior {
    pub unsafe fn SetConfiguration(&self, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<DIRECTMANIPULATION_DRAG_DROP_STATUS> {
        let mut result__: <DIRECTMANIPULATION_DRAG_DROP_STATUS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DIRECTMANIPULATION_DRAG_DROP_STATUS>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationDragDropBehavior {
    type Vtable = IDirectManipulationDragDropBehavior_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x814b5af5_c2c8_4270_a9b7_a198ce8d02fa);
}
impl ::core::convert::From<IDirectManipulationDragDropBehavior> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationDragDropBehavior) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationDragDropBehavior> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationDragDropBehavior) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationDragDropBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationDragDropBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDragDropBehavior_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, status: *mut DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationDragDropEventHandler(pub ::windows::core::IUnknown);
impl IDirectManipulationDragDropEventHandler {
    pub unsafe fn OnDragDropStatusChange<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationViewport2>>(&self, viewport: Param0, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), viewport.into_param().abi(), ::core::mem::transmute(current), ::core::mem::transmute(previous)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationDragDropEventHandler {
    type Vtable = IDirectManipulationDragDropEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fa11b10_701b_41ae_b5f2_49e36bd595aa);
}
impl ::core::convert::From<IDirectManipulationDragDropEventHandler> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationDragDropEventHandler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationDragDropEventHandler> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationDragDropEventHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationDragDropEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationDragDropEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDragDropEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, viewport: ::windows::core::RawPtr, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationFrameInfoProvider(pub ::windows::core::IUnknown);
impl IDirectManipulationFrameInfoProvider {
    pub unsafe fn GetNextFrameInfo(&self, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(time), ::core::mem::transmute(processtime), ::core::mem::transmute(compositiontime)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationFrameInfoProvider {
    type Vtable = IDirectManipulationFrameInfoProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb759dba_6f4c_4c01_874e_19c8a05907f9);
}
impl ::core::convert::From<IDirectManipulationFrameInfoProvider> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationFrameInfoProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationFrameInfoProvider> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationFrameInfoProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationFrameInfoProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationFrameInfoProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationFrameInfoProvider_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::core::HRESULT);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationInteractionEventHandler(pub ::windows::core::IUnknown);
impl IDirectManipulationInteractionEventHandler {
    pub unsafe fn OnInteraction<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationViewport2>>(&self, viewport: Param0, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), viewport.into_param().abi(), ::core::mem::transmute(interaction)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationInteractionEventHandler {
    type Vtable = IDirectManipulationInteractionEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe43f45b8_42b4_403e_b1f2_273b8f510830);
}
impl ::core::convert::From<IDirectManipulationInteractionEventHandler> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationInteractionEventHandler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationInteractionEventHandler> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationInteractionEventHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationInteractionEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationInteractionEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationInteractionEventHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, viewport: ::windows::core::RawPtr, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::core::HRESULT);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationManager(pub ::windows::core::IUnknown);
impl IDirectManipulationManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deactivate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHitTestTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0, hittestwindow: Param1, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), window.into_param().abi(), hittestwindow.into_param().abi(), ::core::mem::transmute(r#type)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(message), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetUpdateManager<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateViewport<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::core::Interface>(&self, frameinfo: Param0, window: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), window.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn CreateContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, T: ::windows::core::Interface>(&self, frameinfo: Param0, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationManager {
    type Vtable = IDirectManipulationManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbf5d3b4_70c7_4163_9322_5a6f660d6fbc);
}
impl ::core::convert::From<IDirectManipulationManager> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationManager> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, frameinfo: ::windows::core::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, frameinfo: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationManager2(pub ::windows::core::IUnknown);
impl IDirectManipulationManager2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deactivate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHitTestTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0, hittestwindow: Param1, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), window.into_param().abi(), hittestwindow.into_param().abi(), ::core::mem::transmute(r#type)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(message), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetUpdateManager<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateViewport<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::core::Interface>(&self, frameinfo: Param0, window: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), window.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn CreateContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, T: ::windows::core::Interface>(&self, frameinfo: Param0, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn CreateBehavior<T: ::windows::core::Interface>(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationManager2 {
    type Vtable = IDirectManipulationManager2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa1005e9_3d16_484c_bfc9_62b61e56ec4e);
}
impl ::core::convert::From<IDirectManipulationManager2> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationManager2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationManager2> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationManager2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDirectManipulationManager> for &IDirectManipulationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectManipulationManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, frameinfo: ::windows::core::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, frameinfo: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationManager3(pub ::windows::core::IUnknown);
impl IDirectManipulationManager3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deactivate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHitTestTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0, hittestwindow: Param1, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), window.into_param().abi(), hittestwindow.into_param().abi(), ::core::mem::transmute(r#type)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(message), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetUpdateManager<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateViewport<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::core::Interface>(&self, frameinfo: Param0, window: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), window.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn CreateContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, T: ::windows::core::Interface>(&self, frameinfo: Param0, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn CreateBehavior<T: ::windows::core::Interface>(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetService<T: ::windows::core::Interface>(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationManager3 {
    type Vtable = IDirectManipulationManager3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cb6b33d_ffe8_488c_b750_fbdfe88dca8c);
}
impl ::core::convert::From<IDirectManipulationManager3> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationManager3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationManager3> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationManager3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDirectManipulationManager2> for &IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectManipulationManager2> {
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
impl<'a> ::windows::core::IntoParam<'a, IDirectManipulationManager> for &IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectManipulationManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, frameinfo: ::windows::core::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, frameinfo: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationPrimaryContent(pub ::windows::core::IUnknown);
impl IDirectManipulationPrimaryContent {
    pub unsafe fn SetSnapInterval(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(motion), ::core::mem::transmute(interval), ::core::mem::transmute(offset)).ok()
    }
    pub unsafe fn SetSnapPoints(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(motion), ::core::mem::transmute(points), ::core::mem::transmute(pointcount)).ok()
    }
    pub unsafe fn SetSnapType(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(motion), ::core::mem::transmute(r#type)).ok()
    }
    pub unsafe fn SetSnapCoordinate(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(motion), ::core::mem::transmute(coordinate), ::core::mem::transmute(origin)).ok()
    }
    pub unsafe fn SetZoomBoundaries(&self, zoomminimum: f32, zoommaximum: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(zoomminimum), ::core::mem::transmute(zoommaximum)).ok()
    }
    pub unsafe fn SetHorizontalAlignment(&self, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(alignment)).ok()
    }
    pub unsafe fn SetVerticalAlignment(&self, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(alignment)).ok()
    }
    pub unsafe fn GetInertiaEndTransform(&self, matrix: *mut f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(pointcount)).ok()
    }
    pub unsafe fn GetCenterPoint(&self, centerx: *mut f32, centery: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerx), ::core::mem::transmute(centery)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationPrimaryContent {
    type Vtable = IDirectManipulationPrimaryContent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc12851e4_1698_4625_b9b1_7ca3ec18630b);
}
impl ::core::convert::From<IDirectManipulationPrimaryContent> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationPrimaryContent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationPrimaryContent> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationPrimaryContent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationPrimaryContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationPrimaryContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationPrimaryContent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, zoomminimum: f32, zoommaximum: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, centerx: *mut f32, centery: *mut f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationUpdateHandler(pub ::windows::core::IUnknown);
impl IDirectManipulationUpdateHandler {
    pub unsafe fn Update(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationUpdateHandler {
    type Vtable = IDirectManipulationUpdateHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x790b6337_64f8_4ff5_a269_b32bc2af27a7);
}
impl ::core::convert::From<IDirectManipulationUpdateHandler> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationUpdateHandler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationUpdateHandler> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationUpdateHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationUpdateHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationUpdateHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationUpdateHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationUpdateManager(pub ::windows::core::IUnknown);
impl IDirectManipulationUpdateManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterWaitHandleCallback<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, IDirectManipulationUpdateHandler>>(&self, handle: Param0, eventhandler: Param1) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), handle.into_param().abi(), eventhandler.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn UnregisterWaitHandleCallback(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
    pub unsafe fn Update<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>>(&self, frameinfo: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), frameinfo.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationUpdateManager {
    type Vtable = IDirectManipulationUpdateManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0ae62fd_be34_46e7_9caa_d361facbb9cc);
}
impl ::core::convert::From<IDirectManipulationUpdateManager> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationUpdateManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationUpdateManager> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationUpdateManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationUpdateManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationUpdateManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationUpdateManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handle: super::super::Foundation::HANDLE, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, frameinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationViewport(pub ::windows::core::IUnknown);
impl IDirectManipulationViewport {
    pub unsafe fn Enable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Disable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
    pub unsafe fn ReleaseContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
    pub unsafe fn ReleaseAllContacts(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<DIRECTMANIPULATION_STATUS> {
        let mut result__: <DIRECTMANIPULATION_STATUS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DIRECTMANIPULATION_STATUS>(result__)
    }
    pub unsafe fn GetTag(&self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    pub unsafe fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetViewportRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetViewportRect(&self, viewport: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ZoomToRect<'a, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, left: f32, top: f32, right: f32, bottom: f32, animate: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(left), ::core::mem::transmute(top), ::core::mem::transmute(right), ::core::mem::transmute(bottom), animate.into_param().abi()).ok()
    }
    pub unsafe fn SetViewportTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(pointcount)).ok()
    }
    pub unsafe fn SyncDisplayTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(pointcount)).ok()
    }
    pub unsafe fn GetPrimaryContent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn AddContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    pub unsafe fn RemoveContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    pub unsafe fn SetViewportOptions(&self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(options)).ok()
    }
    pub unsafe fn AddConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    pub unsafe fn RemoveConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    pub unsafe fn ActivateConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    pub unsafe fn SetManualGesture(&self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    pub unsafe fn SetChaining(&self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabledtypes)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddEventHandler<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, IDirectManipulationViewportEventHandler>>(&self, window: Param0, eventhandler: Param1) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), window.into_param().abi(), eventhandler.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn RemoveEventHandler(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
    pub unsafe fn SetInputMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn SetUpdateMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Abandon(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationViewport {
    type Vtable = IDirectManipulationViewport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28b85a3d_60a0_48bd_9ba1_5ce8d9ea3a6d);
}
impl ::core::convert::From<IDirectManipulationViewport> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationViewport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationViewport> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationViewport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationViewport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationViewport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pointerid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pointerid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, status: *mut DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, object: ::windows::core::RawPtr, id: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, viewport: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, viewport: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: super::super::Foundation::HWND, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationViewport2(pub ::windows::core::IUnknown);
impl IDirectManipulationViewport2 {
    pub unsafe fn Enable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Disable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
    pub unsafe fn ReleaseContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
    pub unsafe fn ReleaseAllContacts(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<DIRECTMANIPULATION_STATUS> {
        let mut result__: <DIRECTMANIPULATION_STATUS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DIRECTMANIPULATION_STATUS>(result__)
    }
    pub unsafe fn GetTag(&self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    pub unsafe fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetViewportRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetViewportRect(&self, viewport: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ZoomToRect<'a, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, left: f32, top: f32, right: f32, bottom: f32, animate: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(left), ::core::mem::transmute(top), ::core::mem::transmute(right), ::core::mem::transmute(bottom), animate.into_param().abi()).ok()
    }
    pub unsafe fn SetViewportTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(pointcount)).ok()
    }
    pub unsafe fn SyncDisplayTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(pointcount)).ok()
    }
    pub unsafe fn GetPrimaryContent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn AddContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    pub unsafe fn RemoveContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    pub unsafe fn SetViewportOptions(&self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(options)).ok()
    }
    pub unsafe fn AddConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    pub unsafe fn RemoveConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    pub unsafe fn ActivateConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    pub unsafe fn SetManualGesture(&self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    pub unsafe fn SetChaining(&self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabledtypes)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddEventHandler<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, IDirectManipulationViewportEventHandler>>(&self, window: Param0, eventhandler: Param1) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), window.into_param().abi(), eventhandler.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn RemoveEventHandler(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
    pub unsafe fn SetInputMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn SetUpdateMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Abandon(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn AddBehavior<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, behavior: Param0) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), behavior.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn RemoveBehavior(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
    pub unsafe fn RemoveAllBehaviors(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationViewport2 {
    type Vtable = IDirectManipulationViewport2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x923ccaac_61e1_4385_b726_017af189882a);
}
impl ::core::convert::From<IDirectManipulationViewport2> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationViewport2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationViewport2> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationViewport2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationViewport2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationViewport2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDirectManipulationViewport> for &IDirectManipulationViewport2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectManipulationViewport> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewport2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pointerid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pointerid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, status: *mut DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, object: ::windows::core::RawPtr, id: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, viewport: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, viewport: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: super::super::Foundation::HWND, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, behavior: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectManipulationViewportEventHandler(pub ::windows::core::IUnknown);
impl IDirectManipulationViewportEventHandler {
    pub unsafe fn OnViewportStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationViewport>>(&self, viewport: Param0, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), viewport.into_param().abi(), ::core::mem::transmute(current), ::core::mem::transmute(previous)).ok()
    }
    pub unsafe fn OnViewportUpdated<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationViewport>>(&self, viewport: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), viewport.into_param().abi()).ok()
    }
    pub unsafe fn OnContentUpdated<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationViewport>, Param1: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, viewport: Param0, content: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), viewport.into_param().abi(), content.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectManipulationViewportEventHandler {
    type Vtable = IDirectManipulationViewportEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x952121da_d69f_45f9_b0f9_f23944321a6d);
}
impl ::core::convert::From<IDirectManipulationViewportEventHandler> for ::windows::core::IUnknown {
    fn from(value: IDirectManipulationViewportEventHandler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectManipulationViewportEventHandler> for ::windows::core::IUnknown {
    fn from(value: &IDirectManipulationViewportEventHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectManipulationViewportEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectManipulationViewportEventHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewportEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, viewport: ::windows::core::RawPtr, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, viewport: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, viewport: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
