#[cfg(feature = "Win32_wsdxml")]
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxy(pszdeviceid : windows_sys::core::PCWSTR, pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, ppdeviceproxy : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml"))]
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxy2(pszdeviceid : windows_sys::core::PCWSTR, pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, pconfigparams : *const super::wsdbase::WSD_CONFIG_PARAM, dwconfigparamcount : u32, ppdeviceproxy : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml"))]
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxyAdvanced(pszdeviceid : windows_sys::core::PCWSTR, pdeviceaddress : *mut core::ffi::c_void, pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, ppdeviceproxy : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
