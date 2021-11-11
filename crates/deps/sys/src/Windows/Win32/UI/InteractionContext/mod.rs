#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn AddPointerInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_Foundation`, `Win32_UI_Input_Pointer`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn BufferPointerPacketsInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn CreateInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn DestroyInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn GetCrossSlideParameterInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn GetHoldParameterInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn GetInertiaParameterInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn GetInteractionConfigurationInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn GetMouseWheelParameterInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn GetPropertyInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_Foundation`, `Win32_UI_Input_Pointer`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetStateInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn GetTapParameterInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn GetTranslationParameterInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn ProcessBufferedPacketsInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn ProcessInertiaInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_Foundation`, `Win32_UI_Input_Pointer`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ProcessPointerFramesInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn RegisterOutputCallbackInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn RegisterOutputCallbackInteractionContext2();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn RemovePointerInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn ResetInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetCrossSlideParametersInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetHoldParameterInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetInertiaParameterInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetInteractionConfigurationInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetMouseWheelParameterInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetPivotInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetPropertyInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetTapParameterInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn SetTranslationParameterInteractionContext();
    #[doc = "*Required features: `Win32_UI_InteractionContext`*"]
    pub fn StopInteractionContext();
}
