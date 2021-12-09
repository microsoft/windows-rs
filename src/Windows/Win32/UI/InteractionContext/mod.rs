#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[inline]
pub unsafe fn AddPointerInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, pointerid: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddPointerInteractionContext(interactioncontext: HINTERACTIONCONTEXT, pointerid: u32) -> ::windows::core::HRESULT;
        }
        AddPointerInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(pointerid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn BufferPointerPacketsInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, entriescount: u32, pointerinfo: *const super::Input::Pointer::POINTER_INFO) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BufferPointerPacketsInteractionContext(interactioncontext: HINTERACTIONCONTEXT, entriescount: u32, pointerinfo: *const super::Input::Pointer::POINTER_INFO) -> ::windows::core::HRESULT;
        }
        BufferPointerPacketsInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(entriescount), ::core::mem::transmute(pointerinfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type CROSS_SLIDE_FLAGS = u32;
pub const CROSS_SLIDE_FLAGS_NONE: CROSS_SLIDE_FLAGS = 0u32;
pub const CROSS_SLIDE_FLAGS_SELECT: CROSS_SLIDE_FLAGS = 1u32;
pub const CROSS_SLIDE_FLAGS_SPEED_BUMP: CROSS_SLIDE_FLAGS = 2u32;
pub const CROSS_SLIDE_FLAGS_REARRANGE: CROSS_SLIDE_FLAGS = 4u32;
pub const CROSS_SLIDE_FLAGS_MAX: CROSS_SLIDE_FLAGS = 4294967295u32;
#[repr(C)]
pub struct CROSS_SLIDE_PARAMETER {
    pub threshold: CROSS_SLIDE_THRESHOLD,
    pub distance: f32,
}
impl ::core::marker::Copy for CROSS_SLIDE_PARAMETER {}
impl ::core::clone::Clone for CROSS_SLIDE_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CROSS_SLIDE_PARAMETER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CROSS_SLIDE_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CROSS_SLIDE_PARAMETER>()) == 0 }
    }
}
impl ::core::cmp::Eq for CROSS_SLIDE_PARAMETER {}
impl ::core::default::Default for CROSS_SLIDE_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type CROSS_SLIDE_THRESHOLD = i32;
pub const CROSS_SLIDE_THRESHOLD_SELECT_START: CROSS_SLIDE_THRESHOLD = 0i32;
pub const CROSS_SLIDE_THRESHOLD_SPEED_BUMP_START: CROSS_SLIDE_THRESHOLD = 1i32;
pub const CROSS_SLIDE_THRESHOLD_SPEED_BUMP_END: CROSS_SLIDE_THRESHOLD = 2i32;
pub const CROSS_SLIDE_THRESHOLD_REARRANGE_START: CROSS_SLIDE_THRESHOLD = 3i32;
pub const CROSS_SLIDE_THRESHOLD_COUNT: CROSS_SLIDE_THRESHOLD = 4i32;
pub const CROSS_SLIDE_THRESHOLD_MAX: CROSS_SLIDE_THRESHOLD = -1i32;
#[inline]
pub unsafe fn CreateInteractionContext() -> ::windows::core::Result<HINTERACTIONCONTEXT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateInteractionContext(interactioncontext: *mut HINTERACTIONCONTEXT) -> ::windows::core::HRESULT;
        }
        let mut result__: HINTERACTIONCONTEXT = ::core::mem::zeroed();
        CreateInteractionContext(::core::mem::transmute(&mut result__)).from_abi::<HINTERACTIONCONTEXT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DestroyInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroyInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows::core::HRESULT;
        }
        DestroyInteractionContext(interactioncontext.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetCrossSlideParameterInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, threshold: CROSS_SLIDE_THRESHOLD) -> ::windows::core::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCrossSlideParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, threshold: CROSS_SLIDE_THRESHOLD, distance: *mut f32) -> ::windows::core::HRESULT;
        }
        let mut result__: f32 = ::core::mem::zeroed();
        GetCrossSlideParameterInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(threshold), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetHoldParameterInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parameter: HOLD_PARAMETER) -> ::windows::core::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetHoldParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: HOLD_PARAMETER, value: *mut f32) -> ::windows::core::HRESULT;
        }
        let mut result__: f32 = ::core::mem::zeroed();
        GetHoldParameterInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(parameter), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetInertiaParameterInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, inertiaparameter: INERTIA_PARAMETER) -> ::windows::core::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetInertiaParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, inertiaparameter: INERTIA_PARAMETER, value: *mut f32) -> ::windows::core::HRESULT;
        }
        let mut result__: f32 = ::core::mem::zeroed();
        GetInertiaParameterInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(inertiaparameter), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetInteractionConfigurationInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, configurationcount: u32, configuration: *mut INTERACTION_CONTEXT_CONFIGURATION) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetInteractionConfigurationInteractionContext(interactioncontext: HINTERACTIONCONTEXT, configurationcount: u32, configuration: *mut INTERACTION_CONTEXT_CONFIGURATION) -> ::windows::core::HRESULT;
        }
        GetInteractionConfigurationInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(configurationcount), ::core::mem::transmute(configuration)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetMouseWheelParameterInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parameter: MOUSE_WHEEL_PARAMETER) -> ::windows::core::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMouseWheelParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: MOUSE_WHEEL_PARAMETER, value: *mut f32) -> ::windows::core::HRESULT;
        }
        let mut result__: f32 = ::core::mem::zeroed();
        GetMouseWheelParameterInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(parameter), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetPropertyInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, contextproperty: INTERACTION_CONTEXT_PROPERTY) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPropertyInteractionContext(interactioncontext: HINTERACTIONCONTEXT, contextproperty: INTERACTION_CONTEXT_PROPERTY, value: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        GetPropertyInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(contextproperty), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn GetStateInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, pointerinfo: *const super::Input::Pointer::POINTER_INFO) -> ::windows::core::Result<INTERACTION_STATE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStateInteractionContext(interactioncontext: HINTERACTIONCONTEXT, pointerinfo: *const super::Input::Pointer::POINTER_INFO, state: *mut INTERACTION_STATE) -> ::windows::core::HRESULT;
        }
        let mut result__: INTERACTION_STATE = ::core::mem::zeroed();
        GetStateInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(pointerinfo), ::core::mem::transmute(&mut result__)).from_abi::<INTERACTION_STATE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetTapParameterInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parameter: TAP_PARAMETER) -> ::windows::core::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTapParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TAP_PARAMETER, value: *mut f32) -> ::windows::core::HRESULT;
        }
        let mut result__: f32 = ::core::mem::zeroed();
        GetTapParameterInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(parameter), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetTranslationParameterInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parameter: TRANSLATION_PARAMETER) -> ::windows::core::Result<f32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTranslationParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TRANSLATION_PARAMETER, value: *mut f32) -> ::windows::core::HRESULT;
        }
        let mut result__: f32 = ::core::mem::zeroed();
        GetTranslationParameterInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(parameter), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type HINTERACTIONCONTEXT = isize;
pub type HOLD_PARAMETER = i32;
pub const HOLD_PARAMETER_MIN_CONTACT_COUNT: HOLD_PARAMETER = 0i32;
pub const HOLD_PARAMETER_MAX_CONTACT_COUNT: HOLD_PARAMETER = 1i32;
pub const HOLD_PARAMETER_THRESHOLD_RADIUS: HOLD_PARAMETER = 2i32;
pub const HOLD_PARAMETER_THRESHOLD_START_DELAY: HOLD_PARAMETER = 3i32;
pub const HOLD_PARAMETER_MAX: HOLD_PARAMETER = -1i32;
pub type INERTIA_PARAMETER = i32;
pub const INERTIA_PARAMETER_TRANSLATION_DECELERATION: INERTIA_PARAMETER = 1i32;
pub const INERTIA_PARAMETER_TRANSLATION_DISPLACEMENT: INERTIA_PARAMETER = 2i32;
pub const INERTIA_PARAMETER_ROTATION_DECELERATION: INERTIA_PARAMETER = 3i32;
pub const INERTIA_PARAMETER_ROTATION_ANGLE: INERTIA_PARAMETER = 4i32;
pub const INERTIA_PARAMETER_EXPANSION_DECELERATION: INERTIA_PARAMETER = 5i32;
pub const INERTIA_PARAMETER_EXPANSION_EXPANSION: INERTIA_PARAMETER = 6i32;
pub const INERTIA_PARAMETER_MAX: INERTIA_PARAMETER = -1i32;
#[repr(C)]
pub struct INTERACTION_ARGUMENTS_CROSS_SLIDE {
    pub flags: CROSS_SLIDE_FLAGS,
}
impl ::core::marker::Copy for INTERACTION_ARGUMENTS_CROSS_SLIDE {}
impl ::core::clone::Clone for INTERACTION_ARGUMENTS_CROSS_SLIDE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for INTERACTION_ARGUMENTS_CROSS_SLIDE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INTERACTION_ARGUMENTS_CROSS_SLIDE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INTERACTION_ARGUMENTS_CROSS_SLIDE>()) == 0 }
    }
}
impl ::core::cmp::Eq for INTERACTION_ARGUMENTS_CROSS_SLIDE {}
impl ::core::default::Default for INTERACTION_ARGUMENTS_CROSS_SLIDE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct INTERACTION_ARGUMENTS_MANIPULATION {
    pub delta: MANIPULATION_TRANSFORM,
    pub cumulative: MANIPULATION_TRANSFORM,
    pub velocity: MANIPULATION_VELOCITY,
    pub railsState: MANIPULATION_RAILS_STATE,
}
impl ::core::marker::Copy for INTERACTION_ARGUMENTS_MANIPULATION {}
impl ::core::clone::Clone for INTERACTION_ARGUMENTS_MANIPULATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for INTERACTION_ARGUMENTS_MANIPULATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INTERACTION_ARGUMENTS_MANIPULATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INTERACTION_ARGUMENTS_MANIPULATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for INTERACTION_ARGUMENTS_MANIPULATION {}
impl ::core::default::Default for INTERACTION_ARGUMENTS_MANIPULATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct INTERACTION_ARGUMENTS_TAP {
    pub count: u32,
}
impl ::core::marker::Copy for INTERACTION_ARGUMENTS_TAP {}
impl ::core::clone::Clone for INTERACTION_ARGUMENTS_TAP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for INTERACTION_ARGUMENTS_TAP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INTERACTION_ARGUMENTS_TAP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INTERACTION_ARGUMENTS_TAP>()) == 0 }
    }
}
impl ::core::cmp::Eq for INTERACTION_ARGUMENTS_TAP {}
impl ::core::default::Default for INTERACTION_ARGUMENTS_TAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type INTERACTION_CONFIGURATION_FLAGS = u32;
pub const INTERACTION_CONFIGURATION_FLAG_NONE: INTERACTION_CONFIGURATION_FLAGS = 0u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION: INTERACTION_CONFIGURATION_FLAGS = 1u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_X: INTERACTION_CONFIGURATION_FLAGS = 2u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_Y: INTERACTION_CONFIGURATION_FLAGS = 4u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_ROTATION: INTERACTION_CONFIGURATION_FLAGS = 8u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_SCALING: INTERACTION_CONFIGURATION_FLAGS = 16u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_INERTIA: INTERACTION_CONFIGURATION_FLAGS = 32u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_ROTATION_INERTIA: INTERACTION_CONFIGURATION_FLAGS = 64u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_SCALING_INERTIA: INTERACTION_CONFIGURATION_FLAGS = 128u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_RAILS_X: INTERACTION_CONFIGURATION_FLAGS = 256u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_RAILS_Y: INTERACTION_CONFIGURATION_FLAGS = 512u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_EXACT: INTERACTION_CONFIGURATION_FLAGS = 1024u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_MULTIPLE_FINGER_PANNING: INTERACTION_CONFIGURATION_FLAGS = 2048u32;
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE: INTERACTION_CONFIGURATION_FLAGS = 1u32;
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_HORIZONTAL: INTERACTION_CONFIGURATION_FLAGS = 2u32;
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_SELECT: INTERACTION_CONFIGURATION_FLAGS = 4u32;
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_SPEED_BUMP: INTERACTION_CONFIGURATION_FLAGS = 8u32;
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_REARRANGE: INTERACTION_CONFIGURATION_FLAGS = 16u32;
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_EXACT: INTERACTION_CONFIGURATION_FLAGS = 32u32;
pub const INTERACTION_CONFIGURATION_FLAG_TAP: INTERACTION_CONFIGURATION_FLAGS = 1u32;
pub const INTERACTION_CONFIGURATION_FLAG_TAP_DOUBLE: INTERACTION_CONFIGURATION_FLAGS = 2u32;
pub const INTERACTION_CONFIGURATION_FLAG_TAP_MULTIPLE_FINGER: INTERACTION_CONFIGURATION_FLAGS = 4u32;
pub const INTERACTION_CONFIGURATION_FLAG_SECONDARY_TAP: INTERACTION_CONFIGURATION_FLAGS = 1u32;
pub const INTERACTION_CONFIGURATION_FLAG_HOLD: INTERACTION_CONFIGURATION_FLAGS = 1u32;
pub const INTERACTION_CONFIGURATION_FLAG_HOLD_MOUSE: INTERACTION_CONFIGURATION_FLAGS = 2u32;
pub const INTERACTION_CONFIGURATION_FLAG_HOLD_MULTIPLE_FINGER: INTERACTION_CONFIGURATION_FLAGS = 4u32;
pub const INTERACTION_CONFIGURATION_FLAG_DRAG: INTERACTION_CONFIGURATION_FLAGS = 1u32;
pub const INTERACTION_CONFIGURATION_FLAG_MAX: INTERACTION_CONFIGURATION_FLAGS = 4294967295u32;
#[repr(C)]
pub struct INTERACTION_CONTEXT_CONFIGURATION {
    pub interactionId: INTERACTION_ID,
    pub enable: INTERACTION_CONFIGURATION_FLAGS,
}
impl ::core::marker::Copy for INTERACTION_CONTEXT_CONFIGURATION {}
impl ::core::clone::Clone for INTERACTION_CONTEXT_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for INTERACTION_CONTEXT_CONFIGURATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INTERACTION_CONTEXT_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INTERACTION_CONTEXT_CONFIGURATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for INTERACTION_CONTEXT_CONFIGURATION {}
impl ::core::default::Default for INTERACTION_CONTEXT_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct INTERACTION_CONTEXT_OUTPUT {
    pub interactionId: INTERACTION_ID,
    pub interactionFlags: INTERACTION_FLAGS,
    pub inputType: super::WindowsAndMessaging::POINTER_INPUT_TYPE,
    pub x: f32,
    pub y: f32,
    pub arguments: INTERACTION_CONTEXT_OUTPUT_0,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for INTERACTION_CONTEXT_OUTPUT {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for INTERACTION_CONTEXT_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows::core::Abi for INTERACTION_CONTEXT_OUTPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for INTERACTION_CONTEXT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INTERACTION_CONTEXT_OUTPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for INTERACTION_CONTEXT_OUTPUT {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for INTERACTION_CONTEXT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub union INTERACTION_CONTEXT_OUTPUT_0 {
    pub manipulation: INTERACTION_ARGUMENTS_MANIPULATION,
    pub tap: INTERACTION_ARGUMENTS_TAP,
    pub crossSlide: INTERACTION_ARGUMENTS_CROSS_SLIDE,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for INTERACTION_CONTEXT_OUTPUT_0 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for INTERACTION_CONTEXT_OUTPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows::core::Abi for INTERACTION_CONTEXT_OUTPUT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for INTERACTION_CONTEXT_OUTPUT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INTERACTION_CONTEXT_OUTPUT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for INTERACTION_CONTEXT_OUTPUT_0 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for INTERACTION_CONTEXT_OUTPUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
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
impl ::core::marker::Copy for INTERACTION_CONTEXT_OUTPUT2 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for INTERACTION_CONTEXT_OUTPUT2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows::core::Abi for INTERACTION_CONTEXT_OUTPUT2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for INTERACTION_CONTEXT_OUTPUT2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INTERACTION_CONTEXT_OUTPUT2>()) == 0 }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for INTERACTION_CONTEXT_OUTPUT2 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for INTERACTION_CONTEXT_OUTPUT2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub union INTERACTION_CONTEXT_OUTPUT2_0 {
    pub manipulation: INTERACTION_ARGUMENTS_MANIPULATION,
    pub tap: INTERACTION_ARGUMENTS_TAP,
    pub crossSlide: INTERACTION_ARGUMENTS_CROSS_SLIDE,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for INTERACTION_CONTEXT_OUTPUT2_0 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for INTERACTION_CONTEXT_OUTPUT2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows::core::Abi for INTERACTION_CONTEXT_OUTPUT2_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for INTERACTION_CONTEXT_OUTPUT2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INTERACTION_CONTEXT_OUTPUT2_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for INTERACTION_CONTEXT_OUTPUT2_0 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for INTERACTION_CONTEXT_OUTPUT2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type INTERACTION_CONTEXT_OUTPUT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(clientdata: *const ::core::ffi::c_void, output: *const INTERACTION_CONTEXT_OUTPUT)>;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type INTERACTION_CONTEXT_OUTPUT_CALLBACK2 = ::core::option::Option<unsafe extern "system" fn(clientdata: *const ::core::ffi::c_void, output: *const INTERACTION_CONTEXT_OUTPUT2)>;
pub type INTERACTION_CONTEXT_PROPERTY = i32;
pub const INTERACTION_CONTEXT_PROPERTY_MEASUREMENT_UNITS: INTERACTION_CONTEXT_PROPERTY = 1i32;
pub const INTERACTION_CONTEXT_PROPERTY_INTERACTION_UI_FEEDBACK: INTERACTION_CONTEXT_PROPERTY = 2i32;
pub const INTERACTION_CONTEXT_PROPERTY_FILTER_POINTERS: INTERACTION_CONTEXT_PROPERTY = 3i32;
pub const INTERACTION_CONTEXT_PROPERTY_MAX: INTERACTION_CONTEXT_PROPERTY = -1i32;
pub type INTERACTION_FLAGS = u32;
pub const INTERACTION_FLAG_NONE: INTERACTION_FLAGS = 0u32;
pub const INTERACTION_FLAG_BEGIN: INTERACTION_FLAGS = 1u32;
pub const INTERACTION_FLAG_END: INTERACTION_FLAGS = 2u32;
pub const INTERACTION_FLAG_CANCEL: INTERACTION_FLAGS = 4u32;
pub const INTERACTION_FLAG_INERTIA: INTERACTION_FLAGS = 8u32;
pub const INTERACTION_FLAG_MAX: INTERACTION_FLAGS = 4294967295u32;
pub type INTERACTION_ID = i32;
pub const INTERACTION_ID_NONE: INTERACTION_ID = 0i32;
pub const INTERACTION_ID_MANIPULATION: INTERACTION_ID = 1i32;
pub const INTERACTION_ID_TAP: INTERACTION_ID = 2i32;
pub const INTERACTION_ID_SECONDARY_TAP: INTERACTION_ID = 3i32;
pub const INTERACTION_ID_HOLD: INTERACTION_ID = 4i32;
pub const INTERACTION_ID_DRAG: INTERACTION_ID = 5i32;
pub const INTERACTION_ID_CROSS_SLIDE: INTERACTION_ID = 6i32;
pub const INTERACTION_ID_MAX: INTERACTION_ID = -1i32;
pub type INTERACTION_STATE = i32;
pub const INTERACTION_STATE_IDLE: INTERACTION_STATE = 0i32;
pub const INTERACTION_STATE_IN_INTERACTION: INTERACTION_STATE = 1i32;
pub const INTERACTION_STATE_POSSIBLE_DOUBLE_TAP: INTERACTION_STATE = 2i32;
pub const INTERACTION_STATE_MAX: INTERACTION_STATE = -1i32;
pub type MANIPULATION_RAILS_STATE = i32;
pub const MANIPULATION_RAILS_STATE_UNDECIDED: MANIPULATION_RAILS_STATE = 0i32;
pub const MANIPULATION_RAILS_STATE_FREE: MANIPULATION_RAILS_STATE = 1i32;
pub const MANIPULATION_RAILS_STATE_RAILED: MANIPULATION_RAILS_STATE = 2i32;
pub const MANIPULATION_RAILS_STATE_MAX: MANIPULATION_RAILS_STATE = -1i32;
#[repr(C)]
pub struct MANIPULATION_TRANSFORM {
    pub translationX: f32,
    pub translationY: f32,
    pub scale: f32,
    pub expansion: f32,
    pub rotation: f32,
}
impl ::core::marker::Copy for MANIPULATION_TRANSFORM {}
impl ::core::clone::Clone for MANIPULATION_TRANSFORM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MANIPULATION_TRANSFORM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MANIPULATION_TRANSFORM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MANIPULATION_TRANSFORM>()) == 0 }
    }
}
impl ::core::cmp::Eq for MANIPULATION_TRANSFORM {}
impl ::core::default::Default for MANIPULATION_TRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MANIPULATION_VELOCITY {
    pub velocityX: f32,
    pub velocityY: f32,
    pub velocityExpansion: f32,
    pub velocityAngular: f32,
}
impl ::core::marker::Copy for MANIPULATION_VELOCITY {}
impl ::core::clone::Clone for MANIPULATION_VELOCITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MANIPULATION_VELOCITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MANIPULATION_VELOCITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MANIPULATION_VELOCITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for MANIPULATION_VELOCITY {}
impl ::core::default::Default for MANIPULATION_VELOCITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type MOUSE_WHEEL_PARAMETER = i32;
pub const MOUSE_WHEEL_PARAMETER_CHAR_TRANSLATION_X: MOUSE_WHEEL_PARAMETER = 1i32;
pub const MOUSE_WHEEL_PARAMETER_CHAR_TRANSLATION_Y: MOUSE_WHEEL_PARAMETER = 2i32;
pub const MOUSE_WHEEL_PARAMETER_DELTA_SCALE: MOUSE_WHEEL_PARAMETER = 3i32;
pub const MOUSE_WHEEL_PARAMETER_DELTA_ROTATION: MOUSE_WHEEL_PARAMETER = 4i32;
pub const MOUSE_WHEEL_PARAMETER_PAGE_TRANSLATION_X: MOUSE_WHEEL_PARAMETER = 5i32;
pub const MOUSE_WHEEL_PARAMETER_PAGE_TRANSLATION_Y: MOUSE_WHEEL_PARAMETER = 6i32;
pub const MOUSE_WHEEL_PARAMETER_MAX: MOUSE_WHEEL_PARAMETER = -1i32;
#[inline]
pub unsafe fn ProcessBufferedPacketsInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessBufferedPacketsInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows::core::HRESULT;
        }
        ProcessBufferedPacketsInteractionContext(interactioncontext.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ProcessInertiaInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessInertiaInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows::core::HRESULT;
        }
        ProcessInertiaInteractionContext(interactioncontext.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn ProcessPointerFramesInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, entriescount: u32, pointercount: u32, pointerinfo: *const super::Input::Pointer::POINTER_INFO) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessPointerFramesInteractionContext(interactioncontext: HINTERACTIONCONTEXT, entriescount: u32, pointercount: u32, pointerinfo: *const super::Input::Pointer::POINTER_INFO) -> ::windows::core::HRESULT;
        }
        ProcessPointerFramesInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(entriescount), ::core::mem::transmute(pointercount), ::core::mem::transmute(pointerinfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn RegisterOutputCallbackInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, outputcallback: INTERACTION_CONTEXT_OUTPUT_CALLBACK, clientdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterOutputCallbackInteractionContext(interactioncontext: HINTERACTIONCONTEXT, outputcallback: ::windows::core::RawPtr, clientdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        RegisterOutputCallbackInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(outputcallback), ::core::mem::transmute(clientdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn RegisterOutputCallbackInteractionContext2<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, outputcallback: INTERACTION_CONTEXT_OUTPUT_CALLBACK2, clientdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterOutputCallbackInteractionContext2(interactioncontext: HINTERACTIONCONTEXT, outputcallback: ::windows::core::RawPtr, clientdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        RegisterOutputCallbackInteractionContext2(interactioncontext.into_param().abi(), ::core::mem::transmute(outputcallback), ::core::mem::transmute(clientdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RemovePointerInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, pointerid: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemovePointerInteractionContext(interactioncontext: HINTERACTIONCONTEXT, pointerid: u32) -> ::windows::core::HRESULT;
        }
        RemovePointerInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(pointerid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ResetInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResetInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows::core::HRESULT;
        }
        ResetInteractionContext(interactioncontext.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetCrossSlideParametersInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parametercount: u32, crossslideparameters: *const CROSS_SLIDE_PARAMETER) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCrossSlideParametersInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parametercount: u32, crossslideparameters: *const CROSS_SLIDE_PARAMETER) -> ::windows::core::HRESULT;
        }
        SetCrossSlideParametersInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(parametercount), ::core::mem::transmute(crossslideparameters)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetHoldParameterInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parameter: HOLD_PARAMETER, value: f32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetHoldParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: HOLD_PARAMETER, value: f32) -> ::windows::core::HRESULT;
        }
        SetHoldParameterInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(parameter), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetInertiaParameterInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, inertiaparameter: INERTIA_PARAMETER, value: f32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetInertiaParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, inertiaparameter: INERTIA_PARAMETER, value: f32) -> ::windows::core::HRESULT;
        }
        SetInertiaParameterInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(inertiaparameter), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetInteractionConfigurationInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, configurationcount: u32, configuration: *const INTERACTION_CONTEXT_CONFIGURATION) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetInteractionConfigurationInteractionContext(interactioncontext: HINTERACTIONCONTEXT, configurationcount: u32, configuration: *const INTERACTION_CONTEXT_CONFIGURATION) -> ::windows::core::HRESULT;
        }
        SetInteractionConfigurationInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(configurationcount), ::core::mem::transmute(configuration)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetMouseWheelParameterInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parameter: MOUSE_WHEEL_PARAMETER, value: f32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMouseWheelParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: MOUSE_WHEEL_PARAMETER, value: f32) -> ::windows::core::HRESULT;
        }
        SetMouseWheelParameterInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(parameter), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetPivotInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, x: f32, y: f32, radius: f32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPivotInteractionContext(interactioncontext: HINTERACTIONCONTEXT, x: f32, y: f32, radius: f32) -> ::windows::core::HRESULT;
        }
        SetPivotInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(radius)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetPropertyInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, contextproperty: INTERACTION_CONTEXT_PROPERTY, value: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPropertyInteractionContext(interactioncontext: HINTERACTIONCONTEXT, contextproperty: INTERACTION_CONTEXT_PROPERTY, value: u32) -> ::windows::core::HRESULT;
        }
        SetPropertyInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(contextproperty), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetTapParameterInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parameter: TAP_PARAMETER, value: f32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetTapParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TAP_PARAMETER, value: f32) -> ::windows::core::HRESULT;
        }
        SetTapParameterInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(parameter), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetTranslationParameterInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0, parameter: TRANSLATION_PARAMETER, value: f32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetTranslationParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TRANSLATION_PARAMETER, value: f32) -> ::windows::core::HRESULT;
        }
        SetTranslationParameterInteractionContext(interactioncontext.into_param().abi(), ::core::mem::transmute(parameter), ::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn StopInteractionContext<'a, Param0: ::windows::core::IntoParam<'a, HINTERACTIONCONTEXT>>(interactioncontext: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StopInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows::core::HRESULT;
        }
        StopInteractionContext(interactioncontext.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type TAP_PARAMETER = i32;
pub const TAP_PARAMETER_MIN_CONTACT_COUNT: TAP_PARAMETER = 0i32;
pub const TAP_PARAMETER_MAX_CONTACT_COUNT: TAP_PARAMETER = 1i32;
pub const TAP_PARAMETER_MAX: TAP_PARAMETER = -1i32;
pub type TRANSLATION_PARAMETER = i32;
pub const TRANSLATION_PARAMETER_MIN_CONTACT_COUNT: TRANSLATION_PARAMETER = 0i32;
pub const TRANSLATION_PARAMETER_MAX_CONTACT_COUNT: TRANSLATION_PARAMETER = 1i32;
pub const TRANSLATION_PARAMETER_MAX: TRANSLATION_PARAMETER = -1i32;
