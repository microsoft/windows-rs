#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn AddPointerInteractionContext(interactioncontext: HINTERACTIONCONTEXT, pointerid: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn BufferPointerPacketsInteractionContext(interactioncontext: HINTERACTIONCONTEXT, entriescount: u32, pointerinfo: *const super::Input::Pointer::POINTER_INFO) -> ::windows_sys::core::HRESULT;
    pub fn CreateInteractionContext(interactioncontext: *mut HINTERACTIONCONTEXT) -> ::windows_sys::core::HRESULT;
    pub fn DestroyInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows_sys::core::HRESULT;
    pub fn GetCrossSlideParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, threshold: CROSS_SLIDE_THRESHOLD, distance: *mut f32) -> ::windows_sys::core::HRESULT;
    pub fn GetHoldParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: HOLD_PARAMETER, value: *mut f32) -> ::windows_sys::core::HRESULT;
    pub fn GetInertiaParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, inertiaparameter: INERTIA_PARAMETER, value: *mut f32) -> ::windows_sys::core::HRESULT;
    pub fn GetInteractionConfigurationInteractionContext(interactioncontext: HINTERACTIONCONTEXT, configurationcount: u32, configuration: *mut INTERACTION_CONTEXT_CONFIGURATION) -> ::windows_sys::core::HRESULT;
    pub fn GetMouseWheelParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: MOUSE_WHEEL_PARAMETER, value: *mut f32) -> ::windows_sys::core::HRESULT;
    pub fn GetPropertyInteractionContext(interactioncontext: HINTERACTIONCONTEXT, contextproperty: INTERACTION_CONTEXT_PROPERTY, value: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetStateInteractionContext(interactioncontext: HINTERACTIONCONTEXT, pointerinfo: *const super::Input::Pointer::POINTER_INFO, state: *mut INTERACTION_STATE) -> ::windows_sys::core::HRESULT;
    pub fn GetTapParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TAP_PARAMETER, value: *mut f32) -> ::windows_sys::core::HRESULT;
    pub fn GetTranslationParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TRANSLATION_PARAMETER, value: *mut f32) -> ::windows_sys::core::HRESULT;
    pub fn ProcessBufferedPacketsInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows_sys::core::HRESULT;
    pub fn ProcessInertiaInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ProcessPointerFramesInteractionContext(interactioncontext: HINTERACTIONCONTEXT, entriescount: u32, pointercount: u32, pointerinfo: *const super::Input::Pointer::POINTER_INFO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn RegisterOutputCallbackInteractionContext(interactioncontext: HINTERACTIONCONTEXT, outputcallback: INTERACTION_CONTEXT_OUTPUT_CALLBACK, clientdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn RegisterOutputCallbackInteractionContext2(interactioncontext: HINTERACTIONCONTEXT, outputcallback: INTERACTION_CONTEXT_OUTPUT_CALLBACK2, clientdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn RemovePointerInteractionContext(interactioncontext: HINTERACTIONCONTEXT, pointerid: u32) -> ::windows_sys::core::HRESULT;
    pub fn ResetInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows_sys::core::HRESULT;
    pub fn SetCrossSlideParametersInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parametercount: u32, crossslideparameters: *const CROSS_SLIDE_PARAMETER) -> ::windows_sys::core::HRESULT;
    pub fn SetHoldParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: HOLD_PARAMETER, value: f32) -> ::windows_sys::core::HRESULT;
    pub fn SetInertiaParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, inertiaparameter: INERTIA_PARAMETER, value: f32) -> ::windows_sys::core::HRESULT;
    pub fn SetInteractionConfigurationInteractionContext(interactioncontext: HINTERACTIONCONTEXT, configurationcount: u32, configuration: *const INTERACTION_CONTEXT_CONFIGURATION) -> ::windows_sys::core::HRESULT;
    pub fn SetMouseWheelParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: MOUSE_WHEEL_PARAMETER, value: f32) -> ::windows_sys::core::HRESULT;
    pub fn SetPivotInteractionContext(interactioncontext: HINTERACTIONCONTEXT, x: f32, y: f32, radius: f32) -> ::windows_sys::core::HRESULT;
    pub fn SetPropertyInteractionContext(interactioncontext: HINTERACTIONCONTEXT, contextproperty: INTERACTION_CONTEXT_PROPERTY, value: u32) -> ::windows_sys::core::HRESULT;
    pub fn SetTapParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TAP_PARAMETER, value: f32) -> ::windows_sys::core::HRESULT;
    pub fn SetTranslationParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TRANSLATION_PARAMETER, value: f32) -> ::windows_sys::core::HRESULT;
    pub fn StopInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows_sys::core::HRESULT;
}
pub const CROSS_SLIDE_FLAGS_NONE: u32 = 0u32;
pub const CROSS_SLIDE_FLAGS_SELECT: u32 = 1u32;
pub const CROSS_SLIDE_FLAGS_SPEED_BUMP: u32 = 2u32;
pub const CROSS_SLIDE_FLAGS_REARRANGE: u32 = 4u32;
pub const CROSS_SLIDE_FLAGS_MAX: u32 = 4294967295u32;
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
pub const CROSS_SLIDE_THRESHOLD_SELECT_START: i32 = 0i32;
pub const CROSS_SLIDE_THRESHOLD_SPEED_BUMP_START: i32 = 1i32;
pub const CROSS_SLIDE_THRESHOLD_SPEED_BUMP_END: i32 = 2i32;
pub const CROSS_SLIDE_THRESHOLD_REARRANGE_START: i32 = 3i32;
pub const CROSS_SLIDE_THRESHOLD_COUNT: i32 = 4i32;
pub const CROSS_SLIDE_THRESHOLD_MAX: i32 = -1i32;
pub type HINTERACTIONCONTEXT = isize;
pub const HOLD_PARAMETER_MIN_CONTACT_COUNT: i32 = 0i32;
pub const HOLD_PARAMETER_MAX_CONTACT_COUNT: i32 = 1i32;
pub const HOLD_PARAMETER_THRESHOLD_RADIUS: i32 = 2i32;
pub const HOLD_PARAMETER_THRESHOLD_START_DELAY: i32 = 3i32;
pub const HOLD_PARAMETER_MAX: i32 = -1i32;
pub const INERTIA_PARAMETER_TRANSLATION_DECELERATION: i32 = 1i32;
pub const INERTIA_PARAMETER_TRANSLATION_DISPLACEMENT: i32 = 2i32;
pub const INERTIA_PARAMETER_ROTATION_DECELERATION: i32 = 3i32;
pub const INERTIA_PARAMETER_ROTATION_ANGLE: i32 = 4i32;
pub const INERTIA_PARAMETER_EXPANSION_DECELERATION: i32 = 5i32;
pub const INERTIA_PARAMETER_EXPANSION_EXPANSION: i32 = 6i32;
pub const INERTIA_PARAMETER_MAX: i32 = -1i32;
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
pub const INTERACTION_CONFIGURATION_FLAG_NONE: u32 = 0u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION: u32 = 1u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_X: u32 = 2u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_Y: u32 = 4u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_ROTATION: u32 = 8u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_SCALING: u32 = 16u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_INERTIA: u32 = 32u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_ROTATION_INERTIA: u32 = 64u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_SCALING_INERTIA: u32 = 128u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_RAILS_X: u32 = 256u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_RAILS_Y: u32 = 512u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_EXACT: u32 = 1024u32;
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_MULTIPLE_FINGER_PANNING: u32 = 2048u32;
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE: u32 = 1u32;
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_HORIZONTAL: u32 = 2u32;
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_SELECT: u32 = 4u32;
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_SPEED_BUMP: u32 = 8u32;
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_REARRANGE: u32 = 16u32;
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_EXACT: u32 = 32u32;
pub const INTERACTION_CONFIGURATION_FLAG_TAP: u32 = 1u32;
pub const INTERACTION_CONFIGURATION_FLAG_TAP_DOUBLE: u32 = 2u32;
pub const INTERACTION_CONFIGURATION_FLAG_TAP_MULTIPLE_FINGER: u32 = 4u32;
pub const INTERACTION_CONFIGURATION_FLAG_SECONDARY_TAP: u32 = 1u32;
pub const INTERACTION_CONFIGURATION_FLAG_HOLD: u32 = 1u32;
pub const INTERACTION_CONFIGURATION_FLAG_HOLD_MOUSE: u32 = 2u32;
pub const INTERACTION_CONFIGURATION_FLAG_HOLD_MULTIPLE_FINGER: u32 = 4u32;
pub const INTERACTION_CONFIGURATION_FLAG_DRAG: u32 = 1u32;
pub const INTERACTION_CONFIGURATION_FLAG_MAX: u32 = 4294967295u32;
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
pub type INTERACTION_CONTEXT_OUTPUT_CALLBACK = unsafe extern "system" fn(clientdata: *const ::core::ffi::c_void, output: *const INTERACTION_CONTEXT_OUTPUT);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type INTERACTION_CONTEXT_OUTPUT_CALLBACK2 = unsafe extern "system" fn(clientdata: *const ::core::ffi::c_void, output: *const INTERACTION_CONTEXT_OUTPUT2);
pub const INTERACTION_CONTEXT_PROPERTY_MEASUREMENT_UNITS: i32 = 1i32;
pub const INTERACTION_CONTEXT_PROPERTY_INTERACTION_UI_FEEDBACK: i32 = 2i32;
pub const INTERACTION_CONTEXT_PROPERTY_FILTER_POINTERS: i32 = 3i32;
pub const INTERACTION_CONTEXT_PROPERTY_MAX: i32 = -1i32;
pub const INTERACTION_FLAG_NONE: u32 = 0u32;
pub const INTERACTION_FLAG_BEGIN: u32 = 1u32;
pub const INTERACTION_FLAG_END: u32 = 2u32;
pub const INTERACTION_FLAG_CANCEL: u32 = 4u32;
pub const INTERACTION_FLAG_INERTIA: u32 = 8u32;
pub const INTERACTION_FLAG_MAX: u32 = 4294967295u32;
pub const INTERACTION_ID_NONE: i32 = 0i32;
pub const INTERACTION_ID_MANIPULATION: i32 = 1i32;
pub const INTERACTION_ID_TAP: i32 = 2i32;
pub const INTERACTION_ID_SECONDARY_TAP: i32 = 3i32;
pub const INTERACTION_ID_HOLD: i32 = 4i32;
pub const INTERACTION_ID_DRAG: i32 = 5i32;
pub const INTERACTION_ID_CROSS_SLIDE: i32 = 6i32;
pub const INTERACTION_ID_MAX: i32 = -1i32;
pub const INTERACTION_STATE_IDLE: i32 = 0i32;
pub const INTERACTION_STATE_IN_INTERACTION: i32 = 1i32;
pub const INTERACTION_STATE_POSSIBLE_DOUBLE_TAP: i32 = 2i32;
pub const INTERACTION_STATE_MAX: i32 = -1i32;
pub const MANIPULATION_RAILS_STATE_UNDECIDED: i32 = 0i32;
pub const MANIPULATION_RAILS_STATE_FREE: i32 = 1i32;
pub const MANIPULATION_RAILS_STATE_RAILED: i32 = 2i32;
pub const MANIPULATION_RAILS_STATE_MAX: i32 = -1i32;
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
pub const MOUSE_WHEEL_PARAMETER_CHAR_TRANSLATION_X: i32 = 1i32;
pub const MOUSE_WHEEL_PARAMETER_CHAR_TRANSLATION_Y: i32 = 2i32;
pub const MOUSE_WHEEL_PARAMETER_DELTA_SCALE: i32 = 3i32;
pub const MOUSE_WHEEL_PARAMETER_DELTA_ROTATION: i32 = 4i32;
pub const MOUSE_WHEEL_PARAMETER_PAGE_TRANSLATION_X: i32 = 5i32;
pub const MOUSE_WHEEL_PARAMETER_PAGE_TRANSLATION_Y: i32 = 6i32;
pub const MOUSE_WHEEL_PARAMETER_MAX: i32 = -1i32;
pub const TAP_PARAMETER_MIN_CONTACT_COUNT: i32 = 0i32;
pub const TAP_PARAMETER_MAX_CONTACT_COUNT: i32 = 1i32;
pub const TAP_PARAMETER_MAX: i32 = -1i32;
pub const TRANSLATION_PARAMETER_MIN_CONTACT_COUNT: i32 = 0i32;
pub const TRANSLATION_PARAMETER_MAX_CONTACT_COUNT: i32 = 1i32;
pub const TRANSLATION_PARAMETER_MAX: i32 = -1i32;
