#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_AbortPipe();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_ControlTransfer();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_FlushPipe();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_Free();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetAdjustedFrameNumber();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetAssociatedInterface();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetCurrentAlternateSetting();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetCurrentFrameNumber();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetCurrentFrameNumberAndQpc();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetDescriptor();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_GetOverlappedResult();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetPipePolicy();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetPowerPolicy();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_Initialize();
    #[doc = "*Required features: `Win32_Devices_Usb`*"]
    pub fn WinUsb_ParseConfigurationDescriptor();
    #[doc = "*Required features: `Win32_Devices_Usb`*"]
    pub fn WinUsb_ParseDescriptors();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_QueryDeviceInformation();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_QueryInterfaceSettings();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_QueryPipe();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_QueryPipeEx();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_ReadIsochPipe();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_ReadIsochPipeAsap();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_ReadPipe();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_RegisterIsochBuffer();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_ResetPipe();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_SetCurrentAlternateSetting();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_SetPipePolicy();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_SetPowerPolicy();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_StartTrackingForTimeSync();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_StopTrackingForTimeSync();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_UnregisterIsochBuffer();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_WriteIsochPipe();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_WriteIsochPipeAsap();
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_WritePipe();
}
