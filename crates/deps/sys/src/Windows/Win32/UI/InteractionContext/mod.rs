#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct CROSS_SLIDE_FLAGS(i32);
#[repr(C)]
pub struct CROSS_SLIDE_PARAMETER(i32);
#[repr(C)]
pub struct CROSS_SLIDE_THRESHOLD(i32);
#[repr(C)]
pub struct HINTERACTIONCONTEXT(i32);
#[repr(C)]
pub struct HOLD_PARAMETER(i32);
#[repr(C)]
pub struct INERTIA_PARAMETER(i32);
#[repr(C)]
pub struct INTERACTION_ARGUMENTS_CROSS_SLIDE(i32);
#[repr(C)]
pub struct INTERACTION_ARGUMENTS_MANIPULATION(i32);
#[repr(C)]
pub struct INTERACTION_ARGUMENTS_TAP(i32);
#[repr(C)]
pub struct INTERACTION_CONFIGURATION_FLAGS(i32);
#[repr(C)]
pub struct INTERACTION_CONTEXT_CONFIGURATION(i32);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[repr(C)]
pub struct INTERACTION_CONTEXT_OUTPUT(i32);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[repr(C)]
pub struct INTERACTION_CONTEXT_OUTPUT2(i32);
#[repr(C)]
pub struct INTERACTION_CONTEXT_OUTPUT_CALLBACK(i32);
#[repr(C)]
pub struct INTERACTION_CONTEXT_OUTPUT_CALLBACK2(i32);
#[repr(C)]
pub struct INTERACTION_CONTEXT_PROPERTY(i32);
#[repr(C)]
pub struct INTERACTION_FLAGS(i32);
#[repr(C)]
pub struct INTERACTION_ID(i32);
#[repr(C)]
pub struct INTERACTION_STATE(i32);
#[repr(C)]
pub struct MANIPULATION_RAILS_STATE(i32);
#[repr(C)]
pub struct MANIPULATION_TRANSFORM(i32);
#[repr(C)]
pub struct MANIPULATION_VELOCITY(i32);
#[repr(C)]
pub struct MOUSE_WHEEL_PARAMETER(i32);
#[repr(C)]
pub struct TAP_PARAMETER(i32);
#[repr(C)]
pub struct TRANSLATION_PARAMETER(i32);
