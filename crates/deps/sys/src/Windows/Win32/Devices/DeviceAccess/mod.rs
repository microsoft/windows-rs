#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_DeviceAccess`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeviceAccessInstance(deviceinterfacepath: super::super::Foundation::PWSTR, desiredaccess: u32, createasync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
}
