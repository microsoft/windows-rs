#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`*"]
    pub fn DevCloseObjectQuery();
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevCreateObjectQuery();
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevCreateObjectQueryEx();
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevCreateObjectQueryFromId();
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevCreateObjectQueryFromIdEx();
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevCreateObjectQueryFromIds();
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevCreateObjectQueryFromIdsEx();
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevFindProperty();
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevFreeObjectProperties();
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevFreeObjects();
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevGetObjectProperties();
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevGetObjectPropertiesEx();
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevGetObjects();
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevGetObjectsEx();
}
