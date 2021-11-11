#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_SerialCommunication`*"]
    pub fn ComDBClaimNextFreePort();
    #[doc = "*Required features: `Win32_Devices_SerialCommunication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ComDBClaimPort();
    #[doc = "*Required features: `Win32_Devices_SerialCommunication`*"]
    pub fn ComDBClose();
    #[doc = "*Required features: `Win32_Devices_SerialCommunication`*"]
    pub fn ComDBGetCurrentPortUsage();
    #[doc = "*Required features: `Win32_Devices_SerialCommunication`*"]
    pub fn ComDBOpen();
    #[doc = "*Required features: `Win32_Devices_SerialCommunication`*"]
    pub fn ComDBReleasePort();
    #[doc = "*Required features: `Win32_Devices_SerialCommunication`*"]
    pub fn ComDBResizeDatabase();
}
