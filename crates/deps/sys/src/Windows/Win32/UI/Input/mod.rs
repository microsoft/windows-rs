#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_UI_Input_Ime")]
pub mod Ime;
#[cfg(feature = "Win32_UI_Input_Ink")]
pub mod Ink;
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
pub mod KeyboardAndMouse;
#[cfg(feature = "Win32_UI_Input_Pointer")]
pub mod Pointer;
#[cfg(feature = "Win32_UI_Input_Radial")]
pub mod Radial;
#[cfg(feature = "Win32_UI_Input_Touch")]
pub mod Touch;
#[cfg(feature = "Win32_UI_Input_XboxController")]
pub mod XboxController;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefRawInputProc();
    #[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCIMSSM();
    #[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentInputMessageSource();
    #[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRawInputBuffer();
    #[doc = "*Required features: `Win32_UI_Input`*"]
    pub fn GetRawInputData();
    #[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRawInputDeviceInfoA();
    #[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRawInputDeviceInfoW();
    #[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRawInputDeviceList();
    #[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRegisteredRawInputDevices();
    #[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterRawInputDevices();
}
