#[cfg(feature = "Win32_wsdxml")]
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceHost(pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, ppdevicehost : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml"))]
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceHost2(pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, pconfigparams : *const super::wsdbase::WSD_CONFIG_PARAM, dwconfigparamcount : u32, ppdevicehost : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml"))]
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceHostAdvanced(pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, pphostaddresses : *const *mut core::ffi::c_void, dwhostaddresscount : u32, ppdevicehost : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
