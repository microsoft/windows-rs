#[cfg(feature = "Win32_wsdxml")]
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryProvider(pcontext : *mut core::ffi::c_void, ppprovider : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml"))]
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryProvider2(pcontext : *mut core::ffi::c_void, pconfigparams : *const super::wsdbase::WSD_CONFIG_PARAM, dwconfigparamcount : u32, ppprovider : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wsdxml")]
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryPublisher(pcontext : *mut core::ffi::c_void, pppublisher : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml"))]
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryPublisher2(pcontext : *mut core::ffi::c_void, pconfigparams : *const super::wsdbase::WSD_CONFIG_PARAM, dwconfigparamcount : u32, pppublisher : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
