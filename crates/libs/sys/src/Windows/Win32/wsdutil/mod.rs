windows_link::link!("wsdapi.dll" "system" fn WSDAllocateLinkedMemory(pparent : *mut core::ffi::c_void, cbsize : usize) -> *mut core::ffi::c_void);
windows_link::link!("wsdapi.dll" "system" fn WSDAttachLinkedMemory(pparent : *mut core::ffi::c_void, pchild : *mut core::ffi::c_void));
windows_link::link!("wsdapi.dll" "system" fn WSDDetachLinkedMemory(pvoid : *mut core::ffi::c_void));
windows_link::link!("wsdapi.dll" "system" fn WSDFreeLinkedMemory(pvoid : *mut core::ffi::c_void));
#[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
windows_link::link!("wsdapi.dll" "system" fn WSDGenerateFault(pszcode : windows_sys::core::PCWSTR, pszsubcode : windows_sys::core::PCWSTR, pszreason : windows_sys::core::PCWSTR, pszdetail : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, ppfault : *mut *mut super::wsdtypes::WSD_SOAP_FAULT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
windows_link::link!("wsdapi.dll" "system" fn WSDGenerateFaultEx(pcode : *const super::wsdxmldom::WSDXML_NAME, psubcode : *const super::wsdxmldom::WSDXML_NAME, preasons : *const super::wsdtypes::WSD_LOCALIZED_STRING_LIST, pszdetail : windows_sys::core::PCWSTR, ppfault : *mut *mut super::wsdtypes::WSD_SOAP_FAULT) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDGetConfigurationOption(dwoption : u32, pvoid : *mut core::ffi::c_void, cboutbuffer : u32) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDSetConfigurationOption(dwoption : u32, pvoid : *const core::ffi::c_void, cbinbuffer : u32) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDUriDecode(source : windows_sys::core::PCWSTR, cchsource : u32, destout : *mut windows_sys::core::PWSTR, cchdestout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDUriEncode(source : windows_sys::core::PCWSTR, cchsource : u32, destout : *mut windows_sys::core::PWSTR, cchdestout : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wsdxmldom")]
windows_link::link!("wsdapi.dll" "system" fn WSDXMLAddChild(pparent : *mut super::wsdxmldom::WSDXML_ELEMENT, pchild : *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wsdxmldom")]
windows_link::link!("wsdapi.dll" "system" fn WSDXMLAddSibling(pfirst : *mut super::wsdxmldom::WSDXML_ELEMENT, psecond : *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wsdxmldom")]
windows_link::link!("wsdapi.dll" "system" fn WSDXMLBuildAnyForSingleElement(pelementname : *mut super::wsdxmldom::WSDXML_NAME, psztext : windows_sys::core::PCWSTR, ppany : *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wsdxmldom")]
windows_link::link!("wsdapi.dll" "system" fn WSDXMLCleanupElement(pany : *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wsdxmldom")]
windows_link::link!("wsdapi.dll" "system" fn WSDXMLGetValueFromAny(psznamespace : windows_sys::core::PCWSTR, pszname : windows_sys::core::PCWSTR, pany : *mut super::wsdxmldom::WSDXML_ELEMENT, ppszvalue : *mut windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
pub const WSDAPI_OPTION_MAX_INBOUND_MESSAGE_SIZE: u32 = 1;
pub const WSDAPI_OPTION_TRACE_XML_TO_DEBUGGER: u32 = 2;
pub const WSDAPI_OPTION_TRACE_XML_TO_FILE: u32 = 3;
pub const WSD_DEFAULT_EVENTING_ADDRESS: windows_sys::core::PCWSTR = windows_sys::core::w!("http://*:5357/");
pub const WSD_DEFAULT_HOSTING_ADDRESS: windows_sys::core::PCWSTR = windows_sys::core::w!("http://*:5357/");
pub const WSD_DEFAULT_SECURE_HOSTING_ADDRESS: windows_sys::core::PCWSTR = windows_sys::core::w!("https://*:5358/");
