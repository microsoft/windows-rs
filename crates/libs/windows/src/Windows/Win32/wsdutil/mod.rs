#[inline]
pub unsafe fn WSDAllocateLinkedMemory(pparent: *mut core::ffi::c_void, cbsize: usize) -> *mut core::ffi::c_void {
    windows_core::link!("wsdapi.dll" "system" fn WSDAllocateLinkedMemory(pparent : *mut core::ffi::c_void, cbsize : usize) -> *mut core::ffi::c_void);
    unsafe { WSDAllocateLinkedMemory(pparent as _, cbsize) }
}
#[inline]
pub unsafe fn WSDAttachLinkedMemory(pparent: *mut core::ffi::c_void, pchild: *mut core::ffi::c_void) {
    windows_core::link!("wsdapi.dll" "system" fn WSDAttachLinkedMemory(pparent : *mut core::ffi::c_void, pchild : *mut core::ffi::c_void));
    unsafe { WSDAttachLinkedMemory(pparent as _, pchild as _) }
}
#[inline]
pub unsafe fn WSDDetachLinkedMemory(pvoid: *mut core::ffi::c_void) {
    windows_core::link!("wsdapi.dll" "system" fn WSDDetachLinkedMemory(pvoid : *mut core::ffi::c_void));
    unsafe { WSDDetachLinkedMemory(pvoid as _) }
}
#[inline]
pub unsafe fn WSDFreeLinkedMemory(pvoid: *mut core::ffi::c_void) {
    windows_core::link!("wsdapi.dll" "system" fn WSDFreeLinkedMemory(pvoid : *mut core::ffi::c_void));
    unsafe { WSDFreeLinkedMemory(pvoid as _) }
}
#[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
#[inline]
pub unsafe fn WSDGenerateFault<P0, P1, P2, P3, P4>(pszcode: P0, pszsubcode: P1, pszreason: P2, pszdetail: P3, pcontext: P4) -> windows_core::Result<*mut super::wsdtypes::WSD_SOAP_FAULT>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<super::wsdxml::IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDGenerateFault(pszcode : windows_core::PCWSTR, pszsubcode : windows_core::PCWSTR, pszreason : windows_core::PCWSTR, pszdetail : windows_core::PCWSTR, pcontext : *mut core::ffi::c_void, ppfault : *mut *mut super::wsdtypes::WSD_SOAP_FAULT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDGenerateFault(pszcode.param().abi(), pszsubcode.param().abi(), pszreason.param().abi(), pszdetail.param().abi(), pcontext.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
#[inline]
pub unsafe fn WSDGenerateFaultEx<P3>(pcode: *const super::wsdxmldom::WSDXML_NAME, psubcode: Option<*const super::wsdxmldom::WSDXML_NAME>, preasons: *const super::wsdtypes::WSD_LOCALIZED_STRING_LIST, pszdetail: P3) -> windows_core::Result<*mut super::wsdtypes::WSD_SOAP_FAULT>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDGenerateFaultEx(pcode : *const super::wsdxmldom::WSDXML_NAME, psubcode : *const super::wsdxmldom::WSDXML_NAME, preasons : *const super::wsdtypes::WSD_LOCALIZED_STRING_LIST, pszdetail : windows_core::PCWSTR, ppfault : *mut *mut super::wsdtypes::WSD_SOAP_FAULT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDGenerateFaultEx(pcode, psubcode.unwrap_or(core::mem::zeroed()) as _, preasons, pszdetail.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WSDGetConfigurationOption(dwoption: u32, pvoid: *mut core::ffi::c_void, cboutbuffer: u32) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDGetConfigurationOption(dwoption : u32, pvoid : *mut core::ffi::c_void, cboutbuffer : u32) -> windows_core::HRESULT);
    unsafe { WSDGetConfigurationOption(dwoption, pvoid as _, cboutbuffer) }
}
#[inline]
pub unsafe fn WSDSetConfigurationOption(dwoption: u32, pvoid: *const core::ffi::c_void, cbinbuffer: u32) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDSetConfigurationOption(dwoption : u32, pvoid : *const core::ffi::c_void, cbinbuffer : u32) -> windows_core::HRESULT);
    unsafe { WSDSetConfigurationOption(dwoption, pvoid, cbinbuffer) }
}
#[inline]
pub unsafe fn WSDUriDecode(source: &[u16], destout: *mut windows_core::PWSTR, cchdestout: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDUriDecode(source : windows_core::PCWSTR, cchsource : u32, destout : *mut windows_core::PWSTR, cchdestout : *mut u32) -> windows_core::HRESULT);
    unsafe { WSDUriDecode(core::mem::transmute(source.as_ptr()), source.len().try_into().unwrap(), destout as _, cchdestout.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WSDUriEncode(source: &[u16], destout: *mut windows_core::PWSTR, cchdestout: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDUriEncode(source : windows_core::PCWSTR, cchsource : u32, destout : *mut windows_core::PWSTR, cchdestout : *mut u32) -> windows_core::HRESULT);
    unsafe { WSDUriEncode(core::mem::transmute(source.as_ptr()), source.len().try_into().unwrap(), destout as _, cchdestout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_wsdxmldom")]
#[inline]
pub unsafe fn WSDXMLAddChild(pparent: *mut super::wsdxmldom::WSDXML_ELEMENT, pchild: *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDXMLAddChild(pparent : *mut super::wsdxmldom::WSDXML_ELEMENT, pchild : *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT);
    unsafe { WSDXMLAddChild(pparent as _, pchild as _) }
}
#[cfg(feature = "Win32_wsdxmldom")]
#[inline]
pub unsafe fn WSDXMLAddSibling(pfirst: *mut super::wsdxmldom::WSDXML_ELEMENT, psecond: *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDXMLAddSibling(pfirst : *mut super::wsdxmldom::WSDXML_ELEMENT, psecond : *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT);
    unsafe { WSDXMLAddSibling(pfirst as _, psecond as _) }
}
#[cfg(feature = "Win32_wsdxmldom")]
#[inline]
pub unsafe fn WSDXMLBuildAnyForSingleElement<P1>(pelementname: *mut super::wsdxmldom::WSDXML_NAME, psztext: P1, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDXMLBuildAnyForSingleElement(pelementname : *mut super::wsdxmldom::WSDXML_NAME, psztext : windows_core::PCWSTR, ppany : *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT);
    unsafe { WSDXMLBuildAnyForSingleElement(pelementname as _, psztext.param().abi(), ppany as _) }
}
#[cfg(feature = "Win32_wsdxmldom")]
#[inline]
pub unsafe fn WSDXMLCleanupElement(pany: *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDXMLCleanupElement(pany : *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT);
    unsafe { WSDXMLCleanupElement(pany as _) }
}
#[cfg(feature = "Win32_wsdxmldom")]
#[inline]
pub unsafe fn WSDXMLGetValueFromAny<P0, P1>(psznamespace: P0, pszname: P1, pany: *mut super::wsdxmldom::WSDXML_ELEMENT, ppszvalue: *mut windows_core::PCWSTR) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDXMLGetValueFromAny(psznamespace : windows_core::PCWSTR, pszname : windows_core::PCWSTR, pany : *mut super::wsdxmldom::WSDXML_ELEMENT, ppszvalue : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { WSDXMLGetValueFromAny(psznamespace.param().abi(), pszname.param().abi(), pany as _, ppszvalue as _) }
}
pub const WSDAPI_OPTION_MAX_INBOUND_MESSAGE_SIZE: u32 = 1;
pub const WSDAPI_OPTION_TRACE_XML_TO_DEBUGGER: u32 = 2;
pub const WSDAPI_OPTION_TRACE_XML_TO_FILE: u32 = 3;
pub const WSD_DEFAULT_EVENTING_ADDRESS: windows_core::PCWSTR = windows_core::w!("http://*:5357/");
pub const WSD_DEFAULT_HOSTING_ADDRESS: windows_core::PCWSTR = windows_core::w!("http://*:5357/");
pub const WSD_DEFAULT_SECURE_HOSTING_ADDRESS: windows_core::PCWSTR = windows_core::w!("https://*:5358/");
