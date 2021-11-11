#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Input_XboxController`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn XInputEnable();
    #[doc = "*Required features: `Win32_UI_Input_XboxController`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn XInputGetAudioDeviceIds();
    #[doc = "*Required features: `Win32_UI_Input_XboxController`*"]
    pub fn XInputGetBatteryInformation();
    #[doc = "*Required features: `Win32_UI_Input_XboxController`*"]
    pub fn XInputGetCapabilities();
    #[doc = "*Required features: `Win32_UI_Input_XboxController`*"]
    pub fn XInputGetKeystroke();
    #[doc = "*Required features: `Win32_UI_Input_XboxController`*"]
    pub fn XInputGetState();
    #[doc = "*Required features: `Win32_UI_Input_XboxController`*"]
    pub fn XInputSetState();
}
