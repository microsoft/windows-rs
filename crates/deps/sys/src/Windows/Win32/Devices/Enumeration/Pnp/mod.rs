#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub fn SwDeviceClose();
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Devices_Properties`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn SwDeviceCreate();
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub fn SwDeviceGetLifetime();
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SwDeviceInterfacePropertySet();
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SwDeviceInterfaceRegister();
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SwDeviceInterfaceSetState();
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SwDevicePropertySet();
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub fn SwDeviceSetLifetime();
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub fn SwMemFree();
}
