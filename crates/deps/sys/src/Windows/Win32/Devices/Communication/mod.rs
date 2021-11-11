#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildCommDCBA();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildCommDCBAndTimeoutsA();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildCommDCBAndTimeoutsW();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildCommDCBW();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClearCommBreak();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClearCommError();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommConfigDialogA();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommConfigDialogW();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EscapeCommFunction();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommConfig();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommMask();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommModemStatus();
    #[doc = "*Required features: `Win32_Devices_Communication`*"]
    pub fn GetCommPorts();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommProperties();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommState();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommTimeouts();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultCommConfigA();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultCommConfigW();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenCommPort();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PurgeComm();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCommBreak();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCommConfig();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCommMask();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCommState();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCommTimeouts();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDefaultCommConfigA();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDefaultCommConfigW();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupComm();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TransmitCommChar();
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WaitCommEvent();
}
