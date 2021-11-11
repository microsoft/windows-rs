#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn AddPointerInteractionContext(interactioncontext: HINTERACTIONCONTEXT, pointerid: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_Foundation`, `Win32_UI_Input_Pointer`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn BufferPointerPacketsInteractionContext(interactioncontext: HINTERACTIONCONTEXT, entriescount: u32, pointerinfo: *const super::Input::Pointer::POINTER_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn CreateInteractionContext(interactioncontext: *mut HINTERACTIONCONTEXT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn DestroyInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn GetCrossSlideParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, threshold: CROSS_SLIDE_THRESHOLD, distance: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn GetHoldParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: HOLD_PARAMETER, value: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn GetInertiaParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, inertiaparameter: INERTIA_PARAMETER, value: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn GetInteractionConfigurationInteractionContext(interactioncontext: HINTERACTIONCONTEXT, configurationcount: u32, configuration: *mut INTERACTION_CONTEXT_CONFIGURATION) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn GetMouseWheelParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: MOUSE_WHEEL_PARAMETER, value: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn GetPropertyInteractionContext(interactioncontext: HINTERACTIONCONTEXT, contextproperty: INTERACTION_CONTEXT_PROPERTY, value: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_Foundation`, `Win32_UI_Input_Pointer`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetStateInteractionContext(interactioncontext: HINTERACTIONCONTEXT, pointerinfo: *const super::Input::Pointer::POINTER_INFO, state: *mut INTERACTION_STATE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn GetTapParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TAP_PARAMETER, value: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn GetTranslationParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TRANSLATION_PARAMETER, value: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn ProcessBufferedPacketsInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn ProcessInertiaInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_Foundation`, `Win32_UI_Input_Pointer`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ProcessPointerFramesInteractionContext(interactioncontext: HINTERACTIONCONTEXT, entriescount: u32, pointercount: u32, pointerinfo: *const super::Input::Pointer::POINTER_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn RegisterOutputCallbackInteractionContext(interactioncontext: HINTERACTIONCONTEXT, outputcallback: ::windows::runtime::RawPtr, clientdata: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn RegisterOutputCallbackInteractionContext2(interactioncontext: HINTERACTIONCONTEXT, outputcallback: ::windows::runtime::RawPtr, clientdata: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn RemovePointerInteractionContext(interactioncontext: HINTERACTIONCONTEXT, pointerid: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn ResetInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetCrossSlideParametersInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parametercount: u32, crossslideparameters: *const CROSS_SLIDE_PARAMETER) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetHoldParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: HOLD_PARAMETER, value: f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetInertiaParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, inertiaparameter: INERTIA_PARAMETER, value: f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetInteractionConfigurationInteractionContext(interactioncontext: HINTERACTIONCONTEXT, configurationcount: u32, configuration: *const INTERACTION_CONTEXT_CONFIGURATION) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetMouseWheelParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: MOUSE_WHEEL_PARAMETER, value: f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetPivotInteractionContext(interactioncontext: HINTERACTIONCONTEXT, x: f32, y: f32, radius: f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetPropertyInteractionContext(interactioncontext: HINTERACTIONCONTEXT, contextproperty: INTERACTION_CONTEXT_PROPERTY, value: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetTapParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TAP_PARAMETER, value: f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetTranslationParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TRANSLATION_PARAMETER, value: f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn StopInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> ::windows::runtime::HRESULT;
}
