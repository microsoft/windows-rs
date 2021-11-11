#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Input_XboxController`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn XInputEnable(enable: super::super::super::Foundation::BOOL);
    #[doc = "*Required features: `Win32_UI_Input_XboxController`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn XInputGetAudioDeviceIds(dwuserindex: u32, prenderdeviceid: super::super::super::Foundation::PWSTR, prendercount: *mut u32, pcapturedeviceid: super::super::super::Foundation::PWSTR, pcapturecount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_XboxController`*"]
    pub fn XInputGetBatteryInformation(dwuserindex: u32, devtype: u8, pbatteryinformation: *mut XINPUT_BATTERY_INFORMATION) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_XboxController`*"]
    pub fn XInputGetCapabilities(dwuserindex: u32, dwflags: u32, pcapabilities: *mut XINPUT_CAPABILITIES) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_XboxController`*"]
    pub fn XInputGetKeystroke(dwuserindex: u32, dwreserved: u32, pkeystroke: *mut XINPUT_KEYSTROKE) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_XboxController`*"]
    pub fn XInputGetState(dwuserindex: u32, pstate: *mut XINPUT_STATE) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_XboxController`*"]
    pub fn XInputSetState(dwuserindex: u32, pvibration: *const XINPUT_VIBRATION) -> u32;
}
