#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseGestureInfoHandle();
    #[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseTouchInputHandle();
    #[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGestureConfig();
    #[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGestureExtraArgs();
    #[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGestureInfo();
    #[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTouchInputInfo();
    #[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsTouchWindow();
    #[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterTouchWindow();
    #[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetGestureConfig();
    #[doc = "*Required features: `Win32_UI_Input_Touch`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterTouchWindow();
}
