#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const CLSID_AutoScrollBehavior: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26126a51_3c70_4c9a_aec2_948849eeb093);
pub const CLSID_DeferContactService: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7b67cf4_84bb_434e_86ae_6592bbc9abd9);
pub const CLSID_DragDropConfigurationBehavior: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09b01b3e_ba6c_454d_82e8_95e352329f23);
pub const CLSID_HorizontalIndicatorContent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7d18cf5_3ec7_44d5_a76b_3770f3cf903d);
pub const CLSID_VerticalIndicatorContent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa10b5f17_afe0_4aa2_91e9_3e7001d2e6b4);
pub const CLSID_VirtualViewportContent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3206a19a_86f0_4cb4_a7f3_16e3b7e2d852);
pub const DCompManipulationCompositor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79dea627_a08a_43ac_8ef5_6900b9299126);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub type DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_STOP: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = 0i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_FORWARD: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = 1i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_REVERSE: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = 2i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub type DIRECTMANIPULATION_CONFIGURATION = i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_CONFIGURATION_NONE: DIRECTMANIPULATION_CONFIGURATION = 0i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_CONFIGURATION_INTERACTION: DIRECTMANIPULATION_CONFIGURATION = 1i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_X: DIRECTMANIPULATION_CONFIGURATION = 2i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_Y: DIRECTMANIPULATION_CONFIGURATION = 4i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING: DIRECTMANIPULATION_CONFIGURATION = 16i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_INERTIA: DIRECTMANIPULATION_CONFIGURATION = 32i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING_INERTIA: DIRECTMANIPULATION_CONFIGURATION = 128i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_X: DIRECTMANIPULATION_CONFIGURATION = 256i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_Y: DIRECTMANIPULATION_CONFIGURATION = 512i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub type DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_VERTICAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 1i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HORIZONTAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 2i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_ONLY: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 16i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 32i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HOLD_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 64i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub type DIRECTMANIPULATION_DRAG_DROP_STATUS = i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_DRAG_DROP_READY: DIRECTMANIPULATION_DRAG_DROP_STATUS = 0i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_DRAG_DROP_PRESELECT: DIRECTMANIPULATION_DRAG_DROP_STATUS = 1i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_DRAG_DROP_SELECTING: DIRECTMANIPULATION_DRAG_DROP_STATUS = 2i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_DRAG_DROP_DRAGGING: DIRECTMANIPULATION_DRAG_DROP_STATUS = 3i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CANCELLED: DIRECTMANIPULATION_DRAG_DROP_STATUS = 4i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_DRAG_DROP_COMMITTED: DIRECTMANIPULATION_DRAG_DROP_STATUS = 5i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub type DIRECTMANIPULATION_GESTURE_CONFIGURATION = i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_GESTURE_NONE: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 0i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_GESTURE_DEFAULT: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 0i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_VERTICAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 8i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_HORIZONTAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 16i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 32i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub type DIRECTMANIPULATION_HITTEST_TYPE = i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_ASYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_AUTO_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub type DIRECTMANIPULATION_HORIZONTALALIGNMENT = i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_NONE: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 0i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_LEFT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 1i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_CENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 2i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_RIGHT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 4i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 8i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub type DIRECTMANIPULATION_INPUT_MODE = i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_INPUT_MODE_AUTOMATIC: DIRECTMANIPULATION_INPUT_MODE = 0i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_INPUT_MODE_MANUAL: DIRECTMANIPULATION_INPUT_MODE = 1i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub type DIRECTMANIPULATION_INTERACTION_TYPE = i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_INTERACTION_BEGIN: DIRECTMANIPULATION_INTERACTION_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_MANIPULATION: DIRECTMANIPULATION_INTERACTION_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_TAP: DIRECTMANIPULATION_INTERACTION_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_HOLD: DIRECTMANIPULATION_INTERACTION_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_CROSS_SLIDE: DIRECTMANIPULATION_INTERACTION_TYPE = 4i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_INTERACTION_TYPE = 5i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_INTERACTION_END: DIRECTMANIPULATION_INTERACTION_TYPE = 100i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_KEYBOARDFOCUS: u32 = 4294967294u32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub type DIRECTMANIPULATION_MOTION_TYPES = i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_MOTION_NONE: DIRECTMANIPULATION_MOTION_TYPES = 0i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_MOTION_TRANSLATEX: DIRECTMANIPULATION_MOTION_TYPES = 1i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_MOTION_TRANSLATEY: DIRECTMANIPULATION_MOTION_TYPES = 2i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_MOTION_ZOOM: DIRECTMANIPULATION_MOTION_TYPES = 4i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_MOTION_CENTERX: DIRECTMANIPULATION_MOTION_TYPES = 16i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_MOTION_CENTERY: DIRECTMANIPULATION_MOTION_TYPES = 32i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_MOTION_ALL: DIRECTMANIPULATION_MOTION_TYPES = 55i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_MOUSEFOCUS: u32 = 4294967293u32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub type DIRECTMANIPULATION_SNAPPOINT_COORDINATE = i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_COORDINATE_BOUNDARY: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = 0i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_COORDINATE_ORIGIN: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = 1i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_COORDINATE_MIRRORED: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = 16i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub type DIRECTMANIPULATION_SNAPPOINT_TYPE = i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY: DIRECTMANIPULATION_SNAPPOINT_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL: DIRECTMANIPULATION_SNAPPOINT_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub type DIRECTMANIPULATION_STATUS = i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_BUILDING: DIRECTMANIPULATION_STATUS = 0i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_ENABLED: DIRECTMANIPULATION_STATUS = 1i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_DISABLED: DIRECTMANIPULATION_STATUS = 2i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_RUNNING: DIRECTMANIPULATION_STATUS = 3i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_INERTIA: DIRECTMANIPULATION_STATUS = 4i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_READY: DIRECTMANIPULATION_STATUS = 5i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_SUSPENDED: DIRECTMANIPULATION_STATUS = 6i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub type DIRECTMANIPULATION_VERTICALALIGNMENT = i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_NONE: DIRECTMANIPULATION_VERTICALALIGNMENT = 0i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_TOP: DIRECTMANIPULATION_VERTICALALIGNMENT = 1i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_CENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = 2i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_BOTTOM: DIRECTMANIPULATION_VERTICALALIGNMENT = 4i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = 8i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub type DIRECTMANIPULATION_VIEWPORT_OPTIONS = i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DEFAULT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 0i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_AUTODISABLE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 1i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_MANUALUPDATE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 2i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_INPUT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 4i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_EXPLICITHITTEST: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 8i32;
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DISABLEPIXELSNAPPING: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 16i32;
pub const DirectManipulationManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54e211b6_3650_4f75_8334_fa359598e1c5);
pub const DirectManipulationPrimaryContent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaa02661_d59e_41c7_8393_3ba3bacb6b57);
pub const DirectManipulationSharedManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99793286_77cc_4b57_96db_3b354f6f9fb5);
pub const DirectManipulationUpdateManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fc1bfd5_1835_441a_b3b1_b6cc74b727d0);
pub const DirectManipulationViewport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34e211b6_3650_4f75_8334_fa359598e1c5);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationAutoScrollBehavior(::windows::core::IUnknown);
impl IDirectManipulationAutoScrollBehavior {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetConfiguration(&self, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(motiontypes), ::core::mem::transmute(scrollmotion)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationAutoScrollBehavior {
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
unsafe impl ::windows::core::Interface for IDirectManipulationAutoScrollBehavior {
    type Vtable = IDirectManipulationAutoScrollBehaviorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d5954d4_2003_4356_9b31_d051c9ff0af7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationAutoScrollBehaviorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationCompositor(::windows::core::IUnknown);
impl IDirectManipulationCompositor {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn AddContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0, device: Param1, parentvisual: Param2, childvisual: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), content.into_param().abi(), device.into_param().abi(), parentvisual.into_param().abi(), childvisual.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn RemoveContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetUpdateManager<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationUpdateManager>>(&self, updatemanager: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), updatemanager.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationCompositor {
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
unsafe impl ::windows::core::Interface for IDirectManipulationCompositor {
    type Vtable = IDirectManipulationCompositorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x537a0825_0387_4efa_b62f_71eb1f085a7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationCompositorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatemanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationCompositor2(::windows::core::IUnknown);
impl IDirectManipulationCompositor2 {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn AddContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0, device: Param1, parentvisual: Param2, childvisual: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), content.into_param().abi(), device.into_param().abi(), parentvisual.into_param().abi(), childvisual.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn RemoveContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetUpdateManager<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationUpdateManager>>(&self, updatemanager: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), updatemanager.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn AddContentWithCrossProcessChaining<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationPrimaryContent>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0, device: Param1, parentvisual: Param2, childvisual: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), content.into_param().abi(), device.into_param().abi(), parentvisual.into_param().abi(), childvisual.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationCompositor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IDirectManipulationCompositor2 {
    type Vtable = IDirectManipulationCompositor2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd38c7822_f1cb_43cb_b4b9_ac0c767a412e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationCompositor2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatemanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationContent(::windows::core::IUnknown);
impl IDirectManipulationContent {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetContentRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: super::super::Foundation::RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContentRect(&self, contentsize: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(contentsize)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetViewport<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetTag(&self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetOutputTransform(&self, matrix: *mut f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(pointcount)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetContentTransform(&self, matrix: *mut f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(pointcount)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SyncContentTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(pointcount)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationContent {
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
unsafe impl ::windows::core::Interface for IDirectManipulationContent {
    type Vtable = IDirectManipulationContentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb89962cb_3d89_442b_bb58_5098fa0f9f16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationContentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentsize: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentsize: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationDeferContactService(::windows::core::IUnknown);
impl IDirectManipulationDeferContactService {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn DeferContact(&self, pointerid: u32, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid), ::core::mem::transmute(timeout)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn CancelContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn CancelDeferral(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationDeferContactService {
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
unsafe impl ::windows::core::Interface for IDirectManipulationDeferContactService {
    type Vtable = IDirectManipulationDeferContactServiceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x652d5c71_fe60_4a98_be70_e5f21291e7f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDeferContactServiceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, timeout: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationDragDropBehavior(::windows::core::IUnknown);
impl IDirectManipulationDragDropBehavior {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetConfiguration(&self, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<DIRECTMANIPULATION_DRAG_DROP_STATUS> {
        let mut result__: DIRECTMANIPULATION_DRAG_DROP_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DIRECTMANIPULATION_DRAG_DROP_STATUS>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationDragDropBehavior {
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
unsafe impl ::windows::core::Interface for IDirectManipulationDragDropBehavior {
    type Vtable = IDirectManipulationDragDropBehaviorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x814b5af5_c2c8_4270_a9b7_a198ce8d02fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDragDropBehaviorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationDragDropEventHandler(::windows::core::IUnknown);
impl IDirectManipulationDragDropEventHandler {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn OnDragDropStatusChange<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationViewport2>>(&self, viewport: Param0, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), viewport.into_param().abi(), ::core::mem::transmute(current), ::core::mem::transmute(previous)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationDragDropEventHandler {
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
unsafe impl ::windows::core::Interface for IDirectManipulationDragDropEventHandler {
    type Vtable = IDirectManipulationDragDropEventHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fa11b10_701b_41ae_b5f2_49e36bd595aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDragDropEventHandlerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationFrameInfoProvider(::windows::core::IUnknown);
impl IDirectManipulationFrameInfoProvider {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetNextFrameInfo(&self, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(time), ::core::mem::transmute(processtime), ::core::mem::transmute(compositiontime)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationFrameInfoProvider {
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
unsafe impl ::windows::core::Interface for IDirectManipulationFrameInfoProvider {
    type Vtable = IDirectManipulationFrameInfoProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb759dba_6f4c_4c01_874e_19c8a05907f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationFrameInfoProviderVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationInteractionEventHandler(::windows::core::IUnknown);
impl IDirectManipulationInteractionEventHandler {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn OnInteraction<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationViewport2>>(&self, viewport: Param0, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), viewport.into_param().abi(), ::core::mem::transmute(interaction)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationInteractionEventHandler {
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
unsafe impl ::windows::core::Interface for IDirectManipulationInteractionEventHandler {
    type Vtable = IDirectManipulationInteractionEventHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe43f45b8_42b4_403e_b1f2_273b8f510830);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationInteractionEventHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationManager(::windows::core::IUnknown);
impl IDirectManipulationManager {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deactivate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHitTestTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0, hittestwindow: Param1, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), window.into_param().abi(), hittestwindow.into_param().abi(), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(message), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetUpdateManager<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateViewport<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::core::Interface>(&self, frameinfo: Param0, window: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), window.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn CreateContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, T: ::windows::core::Interface>(&self, frameinfo: Param0, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationManager {
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
unsafe impl ::windows::core::Interface for IDirectManipulationManager {
    type Vtable = IDirectManipulationManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbf5d3b4_70c7_4163_9322_5a6f660d6fbc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationManager2(::windows::core::IUnknown);
impl IDirectManipulationManager2 {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deactivate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHitTestTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0, hittestwindow: Param1, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), window.into_param().abi(), hittestwindow.into_param().abi(), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(message), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetUpdateManager<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateViewport<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::core::Interface>(&self, frameinfo: Param0, window: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), window.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn CreateContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, T: ::windows::core::Interface>(&self, frameinfo: Param0, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn CreateBehavior<T: ::windows::core::Interface>(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IDirectManipulationManager2 {
    type Vtable = IDirectManipulationManager2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa1005e9_3d16_484c_bfc9_62b61e56ec4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationManager3(::windows::core::IUnknown);
impl IDirectManipulationManager3 {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deactivate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), window.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHitTestTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, window: Param0, hittestwindow: Param1, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), window.into_param().abi(), hittestwindow.into_param().abi(), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(message), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetUpdateManager<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateViewport<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::core::Interface>(&self, frameinfo: Param0, window: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), window.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn CreateContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>, T: ::windows::core::Interface>(&self, frameinfo: Param0, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), frameinfo.into_param().abi(), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn CreateBehavior<T: ::windows::core::Interface>(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetService<T: ::windows::core::Interface>(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IDirectManipulationManager3 {
    type Vtable = IDirectManipulationManager3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cb6b33d_ffe8_488c_b750_fbdfe88dca8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationPrimaryContent(::windows::core::IUnknown);
impl IDirectManipulationPrimaryContent {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetSnapInterval(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(motion), ::core::mem::transmute(interval), ::core::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetSnapPoints(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(motion), ::core::mem::transmute(points), ::core::mem::transmute(pointcount)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetSnapType(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(motion), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetSnapCoordinate(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(motion), ::core::mem::transmute(coordinate), ::core::mem::transmute(origin)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetZoomBoundaries(&self, zoomminimum: f32, zoommaximum: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(zoomminimum), ::core::mem::transmute(zoommaximum)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetHorizontalAlignment(&self, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(alignment)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetVerticalAlignment(&self, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(alignment)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetInertiaEndTransform(&self, matrix: *mut f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(pointcount)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetCenterPoint(&self, centerx: *mut f32, centery: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerx), ::core::mem::transmute(centery)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationPrimaryContent {
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
unsafe impl ::windows::core::Interface for IDirectManipulationPrimaryContent {
    type Vtable = IDirectManipulationPrimaryContentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc12851e4_1698_4625_b9b1_7ca3ec18630b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationPrimaryContentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, zoomminimum: f32, zoommaximum: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: *mut f32, centery: *mut f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationUpdateHandler(::windows::core::IUnknown);
impl IDirectManipulationUpdateHandler {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn Update(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationUpdateHandler {
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
unsafe impl ::windows::core::Interface for IDirectManipulationUpdateHandler {
    type Vtable = IDirectManipulationUpdateHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x790b6337_64f8_4ff5_a269_b32bc2af27a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationUpdateHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationUpdateManager(::windows::core::IUnknown);
impl IDirectManipulationUpdateManager {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterWaitHandleCallback<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, IDirectManipulationUpdateHandler>>(&self, handle: Param0, eventhandler: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), handle.into_param().abi(), eventhandler.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn UnregisterWaitHandleCallback(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn Update<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationFrameInfoProvider>>(&self, frameinfo: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), frameinfo.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationUpdateManager {
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
unsafe impl ::windows::core::Interface for IDirectManipulationUpdateManager {
    type Vtable = IDirectManipulationUpdateManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0ae62fd_be34_46e7_9caa_d361facbb9cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationUpdateManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationViewport(::windows::core::IUnknown);
impl IDirectManipulationViewport {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn Enable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn Disable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn ReleaseContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn ReleaseAllContacts(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<DIRECTMANIPULATION_STATUS> {
        let mut result__: DIRECTMANIPULATION_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DIRECTMANIPULATION_STATUS>(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetTag(&self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetViewportRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: super::super::Foundation::RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetViewportRect(&self, viewport: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewport)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ZoomToRect<'a, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, left: f32, top: f32, right: f32, bottom: f32, animate: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(left), ::core::mem::transmute(top), ::core::mem::transmute(right), ::core::mem::transmute(bottom), animate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetViewportTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(pointcount)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SyncDisplayTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(pointcount)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetPrimaryContent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn AddContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn RemoveContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetViewportOptions(&self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(options)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn AddConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn RemoveConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn ActivateConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetManualGesture(&self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetChaining(&self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabledtypes)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddEventHandler<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, IDirectManipulationViewportEventHandler>>(&self, window: Param0, eventhandler: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), window.into_param().abi(), eventhandler.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn RemoveEventHandler(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetInputMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetUpdateMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn Abandon(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationViewport {
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
unsafe impl ::windows::core::Interface for IDirectManipulationViewport {
    type Vtable = IDirectManipulationViewportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28b85a3d_60a0_48bd_9ba1_5ce8d9ea3a6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewportVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationViewport2(::windows::core::IUnknown);
impl IDirectManipulationViewport2 {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn Enable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn Disable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn ReleaseContact(&self, pointerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerid)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn ReleaseAllContacts(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<DIRECTMANIPULATION_STATUS> {
        let mut result__: DIRECTMANIPULATION_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DIRECTMANIPULATION_STATUS>(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetTag(&self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(object), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, object: Param0, id: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetViewportRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: super::super::Foundation::RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetViewportRect(&self, viewport: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(viewport)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ZoomToRect<'a, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, left: f32, top: f32, right: f32, bottom: f32, animate: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(left), ::core::mem::transmute(top), ::core::mem::transmute(right), ::core::mem::transmute(bottom), animate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetViewportTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(pointcount)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SyncDisplayTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix), ::core::mem::transmute(pointcount)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn GetPrimaryContent<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn AddContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn RemoveContent<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetViewportOptions(&self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(options)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn AddConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn RemoveConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn ActivateConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetManualGesture(&self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(configuration)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetChaining(&self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabledtypes)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddEventHandler<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, IDirectManipulationViewportEventHandler>>(&self, window: Param0, eventhandler: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), window.into_param().abi(), eventhandler.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn RemoveEventHandler(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetInputMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn SetUpdateMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn Abandon(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn AddBehavior<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, behavior: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), behavior.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn RemoveBehavior(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn RemoveAllBehaviors(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationViewport2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IDirectManipulationViewport2 {
    type Vtable = IDirectManipulationViewport2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x923ccaac_61e1_4385_b726_017af189882a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewport2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, behavior: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
#[repr(transparent)]
pub struct IDirectManipulationViewportEventHandler(::windows::core::IUnknown);
impl IDirectManipulationViewportEventHandler {
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn OnViewportStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationViewport>>(&self, viewport: Param0, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), viewport.into_param().abi(), ::core::mem::transmute(current), ::core::mem::transmute(previous)).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn OnViewportUpdated<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationViewport>>(&self, viewport: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), viewport.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Graphics_DirectManipulation'*"]
    pub unsafe fn OnContentUpdated<'a, Param0: ::windows::core::IntoParam<'a, IDirectManipulationViewport>, Param1: ::windows::core::IntoParam<'a, IDirectManipulationContent>>(&self, viewport: Param0, content: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), viewport.into_param().abi(), content.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectManipulationViewportEventHandler {
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
unsafe impl ::windows::core::Interface for IDirectManipulationViewportEventHandler {
    type Vtable = IDirectManipulationViewportEventHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x952121da_d69f_45f9_b0f9_f23944321a6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewportEventHandlerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
