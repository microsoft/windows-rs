#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub fn SwDeviceClose(hswdevice: HSWDEVICE);
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Devices_Properties`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn SwDeviceCreate(pszenumeratorname: super::super::super::Foundation::PWSTR, pszparentdeviceinstance: super::super::super::Foundation::PWSTR, pcreateinfo: *const SW_DEVICE_CREATE_INFO, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY, pcallback: ::windows::runtime::RawPtr, pcontext: *const ::core::ffi::c_void, phswdevice: *mut isize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub fn SwDeviceGetLifetime(hswdevice: HSWDEVICE, plifetime: *mut SW_DEVICE_LIFETIME) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SwDeviceInterfacePropertySet(hswdevice: HSWDEVICE, pszdeviceinterfaceid: super::super::super::Foundation::PWSTR, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SwDeviceInterfaceRegister(hswdevice: HSWDEVICE, pinterfaceclassguid: *const ::windows::runtime::GUID, pszreferencestring: super::super::super::Foundation::PWSTR, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY, fenabled: super::super::super::Foundation::BOOL, ppszdeviceinterfaceid: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SwDeviceInterfaceSetState(hswdevice: HSWDEVICE, pszdeviceinterfaceid: super::super::super::Foundation::PWSTR, fenabled: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SwDevicePropertySet(hswdevice: HSWDEVICE, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub fn SwDeviceSetLifetime(hswdevice: HSWDEVICE, lifetime: SW_DEVICE_LIFETIME) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_Enumeration_Pnp`*"]
    pub fn SwMemFree(pmem: *const ::core::ffi::c_void);
}
