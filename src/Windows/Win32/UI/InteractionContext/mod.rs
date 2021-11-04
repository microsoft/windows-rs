#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn AddPointerInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, pointerid: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddPointerInteractionContext(interactioncontext: HINTERACTIONCONTEXT, pointerid: u32) -> ::windows::runtime::HRESULT;
        }
        AddPointerInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(pointerid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_Foundation`, `Win32_UI_Input_Pointer`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn BufferPointerPacketsInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, entriescount: u32, pointerinfo: *const super::Input::Pointer::POINTER_INFO) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BufferPointerPacketsInteractionContext(interactioncontext: HINTERACTIONCONTEXT, entriescount: u32, pointerinfo: *const super::Input::Pointer::POINTER_INFO) -> ::windows::runtime::HRESULT;
        }
        BufferPointerPacketsInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(entriescount), ::std::mem::transmute(pointerinfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CROSS_SLIDE_FLAGS(pub u32);
pub const CROSS_SLIDE_FLAGS_NONE: CROSS_SLIDE_FLAGS = CROSS_SLIDE_FLAGS(0u32);
pub const CROSS_SLIDE_FLAGS_SELECT: CROSS_SLIDE_FLAGS = CROSS_SLIDE_FLAGS(1u32);
pub const CROSS_SLIDE_FLAGS_SPEED_BUMP: CROSS_SLIDE_FLAGS = CROSS_SLIDE_FLAGS(2u32);
pub const CROSS_SLIDE_FLAGS_REARRANGE: CROSS_SLIDE_FLAGS = CROSS_SLIDE_FLAGS(4u32);
pub const CROSS_SLIDE_FLAGS_MAX: CROSS_SLIDE_FLAGS = CROSS_SLIDE_FLAGS(4294967295u32);
impl ::std::convert::From<u32> for CROSS_SLIDE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CROSS_SLIDE_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for CROSS_SLIDE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CROSS_SLIDE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CROSS_SLIDE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CROSS_SLIDE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CROSS_SLIDE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
pub struct CROSS_SLIDE_PARAMETER {
    pub threshold: CROSS_SLIDE_THRESHOLD,
    pub distance: f32,
}
impl CROSS_SLIDE_PARAMETER {}
impl ::std::default::Default for CROSS_SLIDE_PARAMETER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CROSS_SLIDE_PARAMETER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CROSS_SLIDE_PARAMETER").field("threshold", &self.threshold).field("distance", &self.distance).finish()
    }
}
impl ::std::cmp::PartialEq for CROSS_SLIDE_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.threshold == other.threshold && self.distance == other.distance
    }
}
impl ::std::cmp::Eq for CROSS_SLIDE_PARAMETER {}
unsafe impl ::windows::runtime::Abi for CROSS_SLIDE_PARAMETER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CROSS_SLIDE_THRESHOLD(pub i32);
pub const CROSS_SLIDE_THRESHOLD_SELECT_START: CROSS_SLIDE_THRESHOLD = CROSS_SLIDE_THRESHOLD(0i32);
pub const CROSS_SLIDE_THRESHOLD_SPEED_BUMP_START: CROSS_SLIDE_THRESHOLD = CROSS_SLIDE_THRESHOLD(1i32);
pub const CROSS_SLIDE_THRESHOLD_SPEED_BUMP_END: CROSS_SLIDE_THRESHOLD = CROSS_SLIDE_THRESHOLD(2i32);
pub const CROSS_SLIDE_THRESHOLD_REARRANGE_START: CROSS_SLIDE_THRESHOLD = CROSS_SLIDE_THRESHOLD(3i32);
pub const CROSS_SLIDE_THRESHOLD_COUNT: CROSS_SLIDE_THRESHOLD = CROSS_SLIDE_THRESHOLD(4i32);
pub const CROSS_SLIDE_THRESHOLD_MAX: CROSS_SLIDE_THRESHOLD = CROSS_SLIDE_THRESHOLD(-1i32);
impl ::std::convert::From<i32> for CROSS_SLIDE_THRESHOLD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CROSS_SLIDE_THRESHOLD {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn CreateInteractionContext() -> ::windows::runtime::Result<HINTERACTIONCONTEXT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateInteractionContext(interactioncontext: *mut HINTERACTIONCONTEXT) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HINTERACTIONCONTEXT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateInteractionContext(&mut result__).from_abi::<HINTERACTIONCONTEXT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn DestroyInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroyInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows::runtime::HRESULT;
        }
        DestroyInteractionContext(interactioncontext.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn GetCrossSlideParameterInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, threshold: CROSS_SLIDE_THRESHOLD) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCrossSlideParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, threshold: CROSS_SLIDE_THRESHOLD, distance: *mut f32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetCrossSlideParameterInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(threshold), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn GetHoldParameterInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parameter: HOLD_PARAMETER) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetHoldParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: HOLD_PARAMETER, value: *mut f32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetHoldParameterInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(parameter), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn GetInertiaParameterInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, inertiaparameter: INERTIA_PARAMETER) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetInertiaParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, inertiaparameter: INERTIA_PARAMETER, value: *mut f32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetInertiaParameterInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(inertiaparameter), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn GetInteractionConfigurationInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, configurationcount: u32, configuration: *mut INTERACTION_CONTEXT_CONFIGURATION) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetInteractionConfigurationInteractionContext(interactioncontext: HINTERACTIONCONTEXT, configurationcount: u32, configuration: *mut INTERACTION_CONTEXT_CONFIGURATION) -> ::windows::runtime::HRESULT;
        }
        GetInteractionConfigurationInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(configurationcount), ::std::mem::transmute(configuration)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn GetMouseWheelParameterInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parameter: MOUSE_WHEEL_PARAMETER) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMouseWheelParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: MOUSE_WHEEL_PARAMETER, value: *mut f32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetMouseWheelParameterInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(parameter), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn GetPropertyInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, contextproperty: INTERACTION_CONTEXT_PROPERTY) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPropertyInteractionContext(interactioncontext: HINTERACTIONCONTEXT, contextproperty: INTERACTION_CONTEXT_PROPERTY, value: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetPropertyInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(contextproperty), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_Foundation`, `Win32_UI_Input_Pointer`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn GetStateInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, pointerinfo: *const super::Input::Pointer::POINTER_INFO) -> ::windows::runtime::Result<INTERACTION_STATE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStateInteractionContext(interactioncontext: HINTERACTIONCONTEXT, pointerinfo: *const super::Input::Pointer::POINTER_INFO, state: *mut INTERACTION_STATE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <INTERACTION_STATE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetStateInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(pointerinfo), &mut result__).from_abi::<INTERACTION_STATE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn GetTapParameterInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parameter: TAP_PARAMETER) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTapParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TAP_PARAMETER, value: *mut f32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetTapParameterInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(parameter), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn GetTranslationParameterInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parameter: TRANSLATION_PARAMETER) -> ::windows::runtime::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTranslationParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TRANSLATION_PARAMETER, value: *mut f32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetTranslationParameterInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(parameter), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HINTERACTIONCONTEXT(pub isize);
impl ::std::default::Default for HINTERACTIONCONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HINTERACTIONCONTEXT {}
unsafe impl ::windows::runtime::Abi for HINTERACTIONCONTEXT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HOLD_PARAMETER(pub i32);
pub const HOLD_PARAMETER_MIN_CONTACT_COUNT: HOLD_PARAMETER = HOLD_PARAMETER(0i32);
pub const HOLD_PARAMETER_MAX_CONTACT_COUNT: HOLD_PARAMETER = HOLD_PARAMETER(1i32);
pub const HOLD_PARAMETER_THRESHOLD_RADIUS: HOLD_PARAMETER = HOLD_PARAMETER(2i32);
pub const HOLD_PARAMETER_THRESHOLD_START_DELAY: HOLD_PARAMETER = HOLD_PARAMETER(3i32);
pub const HOLD_PARAMETER_MAX: HOLD_PARAMETER = HOLD_PARAMETER(-1i32);
impl ::std::convert::From<i32> for HOLD_PARAMETER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HOLD_PARAMETER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct INERTIA_PARAMETER(pub i32);
pub const INERTIA_PARAMETER_TRANSLATION_DECELERATION: INERTIA_PARAMETER = INERTIA_PARAMETER(1i32);
pub const INERTIA_PARAMETER_TRANSLATION_DISPLACEMENT: INERTIA_PARAMETER = INERTIA_PARAMETER(2i32);
pub const INERTIA_PARAMETER_ROTATION_DECELERATION: INERTIA_PARAMETER = INERTIA_PARAMETER(3i32);
pub const INERTIA_PARAMETER_ROTATION_ANGLE: INERTIA_PARAMETER = INERTIA_PARAMETER(4i32);
pub const INERTIA_PARAMETER_EXPANSION_DECELERATION: INERTIA_PARAMETER = INERTIA_PARAMETER(5i32);
pub const INERTIA_PARAMETER_EXPANSION_EXPANSION: INERTIA_PARAMETER = INERTIA_PARAMETER(6i32);
pub const INERTIA_PARAMETER_MAX: INERTIA_PARAMETER = INERTIA_PARAMETER(-1i32);
impl ::std::convert::From<i32> for INERTIA_PARAMETER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INERTIA_PARAMETER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
pub struct INTERACTION_ARGUMENTS_CROSS_SLIDE {
    pub flags: CROSS_SLIDE_FLAGS,
}
impl INTERACTION_ARGUMENTS_CROSS_SLIDE {}
impl ::std::default::Default for INTERACTION_ARGUMENTS_CROSS_SLIDE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INTERACTION_ARGUMENTS_CROSS_SLIDE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INTERACTION_ARGUMENTS_CROSS_SLIDE").field("flags", &self.flags).finish()
    }
}
impl ::std::cmp::PartialEq for INTERACTION_ARGUMENTS_CROSS_SLIDE {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags
    }
}
impl ::std::cmp::Eq for INTERACTION_ARGUMENTS_CROSS_SLIDE {}
unsafe impl ::windows::runtime::Abi for INTERACTION_ARGUMENTS_CROSS_SLIDE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
pub struct INTERACTION_ARGUMENTS_MANIPULATION {
    pub delta: MANIPULATION_TRANSFORM,
    pub cumulative: MANIPULATION_TRANSFORM,
    pub velocity: MANIPULATION_VELOCITY,
    pub railsState: MANIPULATION_RAILS_STATE,
}
impl INTERACTION_ARGUMENTS_MANIPULATION {}
impl ::std::default::Default for INTERACTION_ARGUMENTS_MANIPULATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INTERACTION_ARGUMENTS_MANIPULATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INTERACTION_ARGUMENTS_MANIPULATION").field("delta", &self.delta).field("cumulative", &self.cumulative).field("velocity", &self.velocity).field("railsState", &self.railsState).finish()
    }
}
impl ::std::cmp::PartialEq for INTERACTION_ARGUMENTS_MANIPULATION {
    fn eq(&self, other: &Self) -> bool {
        self.delta == other.delta && self.cumulative == other.cumulative && self.velocity == other.velocity && self.railsState == other.railsState
    }
}
impl ::std::cmp::Eq for INTERACTION_ARGUMENTS_MANIPULATION {}
unsafe impl ::windows::runtime::Abi for INTERACTION_ARGUMENTS_MANIPULATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
pub struct INTERACTION_ARGUMENTS_TAP {
    pub count: u32,
}
impl INTERACTION_ARGUMENTS_TAP {}
impl ::std::default::Default for INTERACTION_ARGUMENTS_TAP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INTERACTION_ARGUMENTS_TAP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INTERACTION_ARGUMENTS_TAP").field("count", &self.count).finish()
    }
}
impl ::std::cmp::PartialEq for INTERACTION_ARGUMENTS_TAP {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}
impl ::std::cmp::Eq for INTERACTION_ARGUMENTS_TAP {}
unsafe impl ::windows::runtime::Abi for INTERACTION_ARGUMENTS_TAP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct INTERACTION_CONFIGURATION_FLAGS(pub u32);
pub const INTERACTION_CONFIGURATION_FLAG_NONE: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(0u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(1u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_X: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(2u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_Y: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(4u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_ROTATION: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(8u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_SCALING: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(16u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_INERTIA: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(32u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_ROTATION_INERTIA: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(64u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_SCALING_INERTIA: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(128u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_RAILS_X: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(256u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_RAILS_Y: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(512u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_EXACT: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(1024u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_MULTIPLE_FINGER_PANNING: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(2048u32);
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(1u32);
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_HORIZONTAL: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(2u32);
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_SELECT: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(4u32);
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_SPEED_BUMP: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(8u32);
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_REARRANGE: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(16u32);
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_EXACT: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(32u32);
pub const INTERACTION_CONFIGURATION_FLAG_TAP: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(1u32);
pub const INTERACTION_CONFIGURATION_FLAG_TAP_DOUBLE: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(2u32);
pub const INTERACTION_CONFIGURATION_FLAG_TAP_MULTIPLE_FINGER: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(4u32);
pub const INTERACTION_CONFIGURATION_FLAG_SECONDARY_TAP: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(1u32);
pub const INTERACTION_CONFIGURATION_FLAG_HOLD: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(1u32);
pub const INTERACTION_CONFIGURATION_FLAG_HOLD_MOUSE: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(2u32);
pub const INTERACTION_CONFIGURATION_FLAG_HOLD_MULTIPLE_FINGER: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(4u32);
pub const INTERACTION_CONFIGURATION_FLAG_DRAG: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(1u32);
pub const INTERACTION_CONFIGURATION_FLAG_MAX: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(4294967295u32);
impl ::std::convert::From<u32> for INTERACTION_CONFIGURATION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INTERACTION_CONFIGURATION_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for INTERACTION_CONFIGURATION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for INTERACTION_CONFIGURATION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for INTERACTION_CONFIGURATION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for INTERACTION_CONFIGURATION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for INTERACTION_CONFIGURATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
pub struct INTERACTION_CONTEXT_CONFIGURATION {
    pub interactionId: INTERACTION_ID,
    pub enable: INTERACTION_CONFIGURATION_FLAGS,
}
impl INTERACTION_CONTEXT_CONFIGURATION {}
impl ::std::default::Default for INTERACTION_CONTEXT_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INTERACTION_CONTEXT_CONFIGURATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INTERACTION_CONTEXT_CONFIGURATION").field("interactionId", &self.interactionId).field("enable", &self.enable).finish()
    }
}
impl ::std::cmp::PartialEq for INTERACTION_CONTEXT_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        self.interactionId == other.interactionId && self.enable == other.enable
    }
}
impl ::std::cmp::Eq for INTERACTION_CONTEXT_CONFIGURATION {}
unsafe impl ::windows::runtime::Abi for INTERACTION_CONTEXT_CONFIGURATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_UI_WindowsAndMessaging`*"]
pub struct INTERACTION_CONTEXT_OUTPUT {
    pub interactionId: INTERACTION_ID,
    pub interactionFlags: INTERACTION_FLAGS,
    pub inputType: super::WindowsAndMessaging::POINTER_INPUT_TYPE,
    pub x: f32,
    pub y: f32,
    pub arguments: INTERACTION_CONTEXT_OUTPUT_0,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl INTERACTION_CONTEXT_OUTPUT {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::std::default::Default for INTERACTION_CONTEXT_OUTPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::std::cmp::PartialEq for INTERACTION_CONTEXT_OUTPUT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::std::cmp::Eq for INTERACTION_CONTEXT_OUTPUT {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows::runtime::Abi for INTERACTION_CONTEXT_OUTPUT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
pub union INTERACTION_CONTEXT_OUTPUT_0 {
    pub manipulation: INTERACTION_ARGUMENTS_MANIPULATION,
    pub tap: INTERACTION_ARGUMENTS_TAP,
    pub crossSlide: INTERACTION_ARGUMENTS_CROSS_SLIDE,
}
impl INTERACTION_CONTEXT_OUTPUT_0 {}
impl ::std::default::Default for INTERACTION_CONTEXT_OUTPUT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for INTERACTION_CONTEXT_OUTPUT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for INTERACTION_CONTEXT_OUTPUT_0 {}
unsafe impl ::windows::runtime::Abi for INTERACTION_CONTEXT_OUTPUT_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_UI_WindowsAndMessaging`*"]
pub struct INTERACTION_CONTEXT_OUTPUT2 {
    pub interactionId: INTERACTION_ID,
    pub interactionFlags: INTERACTION_FLAGS,
    pub inputType: super::WindowsAndMessaging::POINTER_INPUT_TYPE,
    pub contactCount: u32,
    pub currentContactCount: u32,
    pub x: f32,
    pub y: f32,
    pub arguments: INTERACTION_CONTEXT_OUTPUT2_0,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl INTERACTION_CONTEXT_OUTPUT2 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::std::default::Default for INTERACTION_CONTEXT_OUTPUT2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::std::cmp::PartialEq for INTERACTION_CONTEXT_OUTPUT2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::std::cmp::Eq for INTERACTION_CONTEXT_OUTPUT2 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows::runtime::Abi for INTERACTION_CONTEXT_OUTPUT2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
pub union INTERACTION_CONTEXT_OUTPUT2_0 {
    pub manipulation: INTERACTION_ARGUMENTS_MANIPULATION,
    pub tap: INTERACTION_ARGUMENTS_TAP,
    pub crossSlide: INTERACTION_ARGUMENTS_CROSS_SLIDE,
}
impl INTERACTION_CONTEXT_OUTPUT2_0 {}
impl ::std::default::Default for INTERACTION_CONTEXT_OUTPUT2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for INTERACTION_CONTEXT_OUTPUT2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for INTERACTION_CONTEXT_OUTPUT2_0 {}
unsafe impl ::windows::runtime::Abi for INTERACTION_CONTEXT_OUTPUT2_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type INTERACTION_CONTEXT_OUTPUT_CALLBACK = unsafe extern "system" fn(clientdata: *const ::std::ffi::c_void, output: *const INTERACTION_CONTEXT_OUTPUT);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type INTERACTION_CONTEXT_OUTPUT_CALLBACK2 = unsafe extern "system" fn(clientdata: *const ::std::ffi::c_void, output: *const INTERACTION_CONTEXT_OUTPUT2);
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct INTERACTION_CONTEXT_PROPERTY(pub i32);
pub const INTERACTION_CONTEXT_PROPERTY_MEASUREMENT_UNITS: INTERACTION_CONTEXT_PROPERTY = INTERACTION_CONTEXT_PROPERTY(1i32);
pub const INTERACTION_CONTEXT_PROPERTY_INTERACTION_UI_FEEDBACK: INTERACTION_CONTEXT_PROPERTY = INTERACTION_CONTEXT_PROPERTY(2i32);
pub const INTERACTION_CONTEXT_PROPERTY_FILTER_POINTERS: INTERACTION_CONTEXT_PROPERTY = INTERACTION_CONTEXT_PROPERTY(3i32);
pub const INTERACTION_CONTEXT_PROPERTY_MAX: INTERACTION_CONTEXT_PROPERTY = INTERACTION_CONTEXT_PROPERTY(-1i32);
impl ::std::convert::From<i32> for INTERACTION_CONTEXT_PROPERTY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INTERACTION_CONTEXT_PROPERTY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct INTERACTION_FLAGS(pub u32);
pub const INTERACTION_FLAG_NONE: INTERACTION_FLAGS = INTERACTION_FLAGS(0u32);
pub const INTERACTION_FLAG_BEGIN: INTERACTION_FLAGS = INTERACTION_FLAGS(1u32);
pub const INTERACTION_FLAG_END: INTERACTION_FLAGS = INTERACTION_FLAGS(2u32);
pub const INTERACTION_FLAG_CANCEL: INTERACTION_FLAGS = INTERACTION_FLAGS(4u32);
pub const INTERACTION_FLAG_INERTIA: INTERACTION_FLAGS = INTERACTION_FLAGS(8u32);
pub const INTERACTION_FLAG_MAX: INTERACTION_FLAGS = INTERACTION_FLAGS(4294967295u32);
impl ::std::convert::From<u32> for INTERACTION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INTERACTION_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for INTERACTION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for INTERACTION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for INTERACTION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for INTERACTION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for INTERACTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct INTERACTION_ID(pub i32);
pub const INTERACTION_ID_NONE: INTERACTION_ID = INTERACTION_ID(0i32);
pub const INTERACTION_ID_MANIPULATION: INTERACTION_ID = INTERACTION_ID(1i32);
pub const INTERACTION_ID_TAP: INTERACTION_ID = INTERACTION_ID(2i32);
pub const INTERACTION_ID_SECONDARY_TAP: INTERACTION_ID = INTERACTION_ID(3i32);
pub const INTERACTION_ID_HOLD: INTERACTION_ID = INTERACTION_ID(4i32);
pub const INTERACTION_ID_DRAG: INTERACTION_ID = INTERACTION_ID(5i32);
pub const INTERACTION_ID_CROSS_SLIDE: INTERACTION_ID = INTERACTION_ID(6i32);
pub const INTERACTION_ID_MAX: INTERACTION_ID = INTERACTION_ID(-1i32);
impl ::std::convert::From<i32> for INTERACTION_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INTERACTION_ID {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct INTERACTION_STATE(pub i32);
pub const INTERACTION_STATE_IDLE: INTERACTION_STATE = INTERACTION_STATE(0i32);
pub const INTERACTION_STATE_IN_INTERACTION: INTERACTION_STATE = INTERACTION_STATE(1i32);
pub const INTERACTION_STATE_POSSIBLE_DOUBLE_TAP: INTERACTION_STATE = INTERACTION_STATE(2i32);
pub const INTERACTION_STATE_MAX: INTERACTION_STATE = INTERACTION_STATE(-1i32);
impl ::std::convert::From<i32> for INTERACTION_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INTERACTION_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MANIPULATION_RAILS_STATE(pub i32);
pub const MANIPULATION_RAILS_STATE_UNDECIDED: MANIPULATION_RAILS_STATE = MANIPULATION_RAILS_STATE(0i32);
pub const MANIPULATION_RAILS_STATE_FREE: MANIPULATION_RAILS_STATE = MANIPULATION_RAILS_STATE(1i32);
pub const MANIPULATION_RAILS_STATE_RAILED: MANIPULATION_RAILS_STATE = MANIPULATION_RAILS_STATE(2i32);
pub const MANIPULATION_RAILS_STATE_MAX: MANIPULATION_RAILS_STATE = MANIPULATION_RAILS_STATE(-1i32);
impl ::std::convert::From<i32> for MANIPULATION_RAILS_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MANIPULATION_RAILS_STATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
pub struct MANIPULATION_TRANSFORM {
    pub translationX: f32,
    pub translationY: f32,
    pub scale: f32,
    pub expansion: f32,
    pub rotation: f32,
}
impl MANIPULATION_TRANSFORM {}
impl ::std::default::Default for MANIPULATION_TRANSFORM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MANIPULATION_TRANSFORM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MANIPULATION_TRANSFORM").field("translationX", &self.translationX).field("translationY", &self.translationY).field("scale", &self.scale).field("expansion", &self.expansion).field("rotation", &self.rotation).finish()
    }
}
impl ::std::cmp::PartialEq for MANIPULATION_TRANSFORM {
    fn eq(&self, other: &Self) -> bool {
        self.translationX == other.translationX && self.translationY == other.translationY && self.scale == other.scale && self.expansion == other.expansion && self.rotation == other.rotation
    }
}
impl ::std::cmp::Eq for MANIPULATION_TRANSFORM {}
unsafe impl ::windows::runtime::Abi for MANIPULATION_TRANSFORM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
pub struct MANIPULATION_VELOCITY {
    pub velocityX: f32,
    pub velocityY: f32,
    pub velocityExpansion: f32,
    pub velocityAngular: f32,
}
impl MANIPULATION_VELOCITY {}
impl ::std::default::Default for MANIPULATION_VELOCITY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MANIPULATION_VELOCITY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MANIPULATION_VELOCITY").field("velocityX", &self.velocityX).field("velocityY", &self.velocityY).field("velocityExpansion", &self.velocityExpansion).field("velocityAngular", &self.velocityAngular).finish()
    }
}
impl ::std::cmp::PartialEq for MANIPULATION_VELOCITY {
    fn eq(&self, other: &Self) -> bool {
        self.velocityX == other.velocityX && self.velocityY == other.velocityY && self.velocityExpansion == other.velocityExpansion && self.velocityAngular == other.velocityAngular
    }
}
impl ::std::cmp::Eq for MANIPULATION_VELOCITY {}
unsafe impl ::windows::runtime::Abi for MANIPULATION_VELOCITY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MOUSE_WHEEL_PARAMETER(pub i32);
pub const MOUSE_WHEEL_PARAMETER_CHAR_TRANSLATION_X: MOUSE_WHEEL_PARAMETER = MOUSE_WHEEL_PARAMETER(1i32);
pub const MOUSE_WHEEL_PARAMETER_CHAR_TRANSLATION_Y: MOUSE_WHEEL_PARAMETER = MOUSE_WHEEL_PARAMETER(2i32);
pub const MOUSE_WHEEL_PARAMETER_DELTA_SCALE: MOUSE_WHEEL_PARAMETER = MOUSE_WHEEL_PARAMETER(3i32);
pub const MOUSE_WHEEL_PARAMETER_DELTA_ROTATION: MOUSE_WHEEL_PARAMETER = MOUSE_WHEEL_PARAMETER(4i32);
pub const MOUSE_WHEEL_PARAMETER_PAGE_TRANSLATION_X: MOUSE_WHEEL_PARAMETER = MOUSE_WHEEL_PARAMETER(5i32);
pub const MOUSE_WHEEL_PARAMETER_PAGE_TRANSLATION_Y: MOUSE_WHEEL_PARAMETER = MOUSE_WHEEL_PARAMETER(6i32);
pub const MOUSE_WHEEL_PARAMETER_MAX: MOUSE_WHEEL_PARAMETER = MOUSE_WHEEL_PARAMETER(-1i32);
impl ::std::convert::From<i32> for MOUSE_WHEEL_PARAMETER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MOUSE_WHEEL_PARAMETER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn ProcessBufferedPacketsInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessBufferedPacketsInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows::runtime::HRESULT;
        }
        ProcessBufferedPacketsInteractionContext(interactioncontext.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn ProcessInertiaInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessInertiaInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows::runtime::HRESULT;
        }
        ProcessInertiaInteractionContext(interactioncontext.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_Foundation`, `Win32_UI_Input_Pointer`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn ProcessPointerFramesInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, entriescount: u32, pointercount: u32, pointerinfo: *const super::Input::Pointer::POINTER_INFO) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessPointerFramesInteractionContext(interactioncontext: HINTERACTIONCONTEXT, entriescount: u32, pointercount: u32, pointerinfo: *const super::Input::Pointer::POINTER_INFO) -> ::windows::runtime::HRESULT;
        }
        ProcessPointerFramesInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(entriescount), ::std::mem::transmute(pointercount), ::std::mem::transmute(pointerinfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn RegisterOutputCallbackInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, outputcallback: ::std::option::Option<INTERACTION_CONTEXT_OUTPUT_CALLBACK>, clientdata: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterOutputCallbackInteractionContext(interactioncontext: HINTERACTIONCONTEXT, outputcallback: ::windows::runtime::RawPtr, clientdata: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        RegisterOutputCallbackInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(outputcallback), ::std::mem::transmute(clientdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn RegisterOutputCallbackInteractionContext2<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, outputcallback: ::std::option::Option<INTERACTION_CONTEXT_OUTPUT_CALLBACK2>, clientdata: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterOutputCallbackInteractionContext2(interactioncontext: HINTERACTIONCONTEXT, outputcallback: ::windows::runtime::RawPtr, clientdata: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        RegisterOutputCallbackInteractionContext2(interactioncontext.into_param().abi(), ::std::mem::transmute(outputcallback), ::std::mem::transmute(clientdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn RemovePointerInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, pointerid: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemovePointerInteractionContext(interactioncontext: HINTERACTIONCONTEXT, pointerid: u32) -> ::windows::runtime::HRESULT;
        }
        RemovePointerInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(pointerid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn ResetInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResetInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows::runtime::HRESULT;
        }
        ResetInteractionContext(interactioncontext.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn SetCrossSlideParametersInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parametercount: u32, crossslideparameters: *const CROSS_SLIDE_PARAMETER) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCrossSlideParametersInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parametercount: u32, crossslideparameters: *const CROSS_SLIDE_PARAMETER) -> ::windows::runtime::HRESULT;
        }
        SetCrossSlideParametersInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(parametercount), ::std::mem::transmute(crossslideparameters)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn SetHoldParameterInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parameter: HOLD_PARAMETER, value: f32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetHoldParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: HOLD_PARAMETER, value: f32) -> ::windows::runtime::HRESULT;
        }
        SetHoldParameterInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(parameter), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn SetInertiaParameterInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, inertiaparameter: INERTIA_PARAMETER, value: f32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetInertiaParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, inertiaparameter: INERTIA_PARAMETER, value: f32) -> ::windows::runtime::HRESULT;
        }
        SetInertiaParameterInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(inertiaparameter), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn SetInteractionConfigurationInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, configurationcount: u32, configuration: *const INTERACTION_CONTEXT_CONFIGURATION) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetInteractionConfigurationInteractionContext(interactioncontext: HINTERACTIONCONTEXT, configurationcount: u32, configuration: *const INTERACTION_CONTEXT_CONFIGURATION) -> ::windows::runtime::HRESULT;
        }
        SetInteractionConfigurationInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(configurationcount), ::std::mem::transmute(configuration)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn SetMouseWheelParameterInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parameter: MOUSE_WHEEL_PARAMETER, value: f32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMouseWheelParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: MOUSE_WHEEL_PARAMETER, value: f32) -> ::windows::runtime::HRESULT;
        }
        SetMouseWheelParameterInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(parameter), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn SetPivotInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, x: f32, y: f32, radius: f32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPivotInteractionContext(interactioncontext: HINTERACTIONCONTEXT, x: f32, y: f32, radius: f32) -> ::windows::runtime::HRESULT;
        }
        SetPivotInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(x), ::std::mem::transmute(y), ::std::mem::transmute(radius)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn SetPropertyInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, contextproperty: INTERACTION_CONTEXT_PROPERTY, value: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPropertyInteractionContext(interactioncontext: HINTERACTIONCONTEXT, contextproperty: INTERACTION_CONTEXT_PROPERTY, value: u32) -> ::windows::runtime::HRESULT;
        }
        SetPropertyInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(contextproperty), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn SetTapParameterInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parameter: TAP_PARAMETER, value: f32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetTapParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TAP_PARAMETER, value: f32) -> ::windows::runtime::HRESULT;
        }
        SetTapParameterInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(parameter), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn SetTranslationParameterInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parameter: TRANSLATION_PARAMETER, value: f32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetTranslationParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TRANSLATION_PARAMETER, value: f32) -> ::windows::runtime::HRESULT;
        }
        SetTranslationParameterInteractionContext(interactioncontext.into_param().abi(), ::std::mem::transmute(parameter), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[inline]
pub unsafe fn StopInteractionContext<'a, Param0: ::windows::runtime::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StopInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows::runtime::HRESULT;
        }
        StopInteractionContext(interactioncontext.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TAP_PARAMETER(pub i32);
pub const TAP_PARAMETER_MIN_CONTACT_COUNT: TAP_PARAMETER = TAP_PARAMETER(0i32);
pub const TAP_PARAMETER_MAX_CONTACT_COUNT: TAP_PARAMETER = TAP_PARAMETER(1i32);
pub const TAP_PARAMETER_MAX: TAP_PARAMETER = TAP_PARAMETER(-1i32);
impl ::std::convert::From<i32> for TAP_PARAMETER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TAP_PARAMETER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_InteractionContext`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TRANSLATION_PARAMETER(pub i32);
pub const TRANSLATION_PARAMETER_MIN_CONTACT_COUNT: TRANSLATION_PARAMETER = TRANSLATION_PARAMETER(0i32);
pub const TRANSLATION_PARAMETER_MAX_CONTACT_COUNT: TRANSLATION_PARAMETER = TRANSLATION_PARAMETER(1i32);
pub const TRANSLATION_PARAMETER_MAX: TRANSLATION_PARAMETER = TRANSLATION_PARAMETER(-1i32);
impl ::std::convert::From<i32> for TRANSLATION_PARAMETER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TRANSLATION_PARAMETER {
    type Abi = Self;
}
