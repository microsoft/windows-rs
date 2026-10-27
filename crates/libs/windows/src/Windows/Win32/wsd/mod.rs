#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WSDAllocateLinkedMemory(pparent: *mut core::ffi::c_void, cbsize: super::size_t) -> *mut core::ffi::c_void {
    windows_core::link!("wsdapi.dll" "system" fn WSDAllocateLinkedMemory(pparent : *mut core::ffi::c_void, cbsize : super::size_t) -> *mut core::ffi::c_void);
    unsafe { WSDAllocateLinkedMemory(pparent as _, cbsize) }
}
#[inline]
pub unsafe fn WSDAttachLinkedMemory(pparent: *mut core::ffi::c_void, pchild: *mut core::ffi::c_void) {
    windows_core::link!("wsdapi.dll" "system" fn WSDAttachLinkedMemory(pparent : *mut core::ffi::c_void, pchild : *mut core::ffi::c_void));
    unsafe { WSDAttachLinkedMemory(pparent as _, pchild as _) }
}
#[inline]
pub unsafe fn WSDCreateDeviceHost<P0, P1>(pszlocalid: P0, pcontext: P1) -> windows_core::Result<IWSDDeviceHost>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDeviceHost(pszlocalid : windows_core::PCWSTR, pcontext : *mut core::ffi::c_void, ppdevicehost : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDeviceHost(pszlocalid.param().abi(), pcontext.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSDCreateDeviceHost2<P0, P1>(pszlocalid: P0, pcontext: P1, pconfigparams: Option<&[WSD_CONFIG_PARAM]>) -> windows_core::Result<IWSDDeviceHost>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDeviceHost2(pszlocalid : windows_core::PCWSTR, pcontext : *mut core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : super::DWORD, ppdevicehost : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDeviceHost2(pszlocalid.param().abi(), pcontext.param().abi(), pconfigparams.map_or(core::ptr::null(), |slice| slice.as_ptr()), pconfigparams.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WSDCreateDeviceHostAdvanced<P0, P1>(pszlocalid: P0, pcontext: P1, pphostaddresses: Option<&[Option<IWSDAddress>]>) -> windows_core::Result<IWSDDeviceHost>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDeviceHostAdvanced(pszlocalid : windows_core::PCWSTR, pcontext : *mut core::ffi::c_void, pphostaddresses : *const *mut core::ffi::c_void, dwhostaddresscount : super::DWORD, ppdevicehost : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDeviceHostAdvanced(pszlocalid.param().abi(), pcontext.param().abi(), core::mem::transmute(pphostaddresses.map_or(core::ptr::null(), |slice| slice.as_ptr())), pphostaddresses.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WSDCreateDeviceProxy<P0, P1, P2>(pszdeviceid: P0, pszlocalid: P1, pcontext: P2) -> windows_core::Result<IWSDDeviceProxy>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxy(pszdeviceid : windows_core::PCWSTR, pszlocalid : windows_core::PCWSTR, pcontext : *mut core::ffi::c_void, ppdeviceproxy : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDeviceProxy(pszdeviceid.param().abi(), pszlocalid.param().abi(), pcontext.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSDCreateDeviceProxy2<P0, P1, P2>(pszdeviceid: P0, pszlocalid: P1, pcontext: P2, pconfigparams: Option<&[WSD_CONFIG_PARAM]>) -> windows_core::Result<IWSDDeviceProxy>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxy2(pszdeviceid : windows_core::PCWSTR, pszlocalid : windows_core::PCWSTR, pcontext : *mut core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : super::DWORD, ppdeviceproxy : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDeviceProxy2(pszdeviceid.param().abi(), pszlocalid.param().abi(), pcontext.param().abi(), pconfigparams.map_or(core::ptr::null(), |slice| slice.as_ptr()), pconfigparams.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WSDCreateDeviceProxyAdvanced<P0, P1, P2, P3>(pszdeviceid: P0, pdeviceaddress: P1, pszlocalid: P2, pcontext: P3) -> windows_core::Result<IWSDDeviceProxy>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<IWSDAddress>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxyAdvanced(pszdeviceid : windows_core::PCWSTR, pdeviceaddress : *mut core::ffi::c_void, pszlocalid : windows_core::PCWSTR, pcontext : *mut core::ffi::c_void, ppdeviceproxy : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDeviceProxyAdvanced(pszdeviceid.param().abi(), pdeviceaddress.param().abi(), pszlocalid.param().abi(), pcontext.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WSDCreateDiscoveryProvider<P0>(pcontext: P0) -> windows_core::Result<IWSDiscoveryProvider>
where
    P0: windows_core::Param<IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryProvider(pcontext : *mut core::ffi::c_void, ppprovider : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDiscoveryProvider(pcontext.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSDCreateDiscoveryProvider2<P0>(pcontext: P0, pconfigparams: Option<&[WSD_CONFIG_PARAM]>) -> windows_core::Result<IWSDiscoveryProvider>
where
    P0: windows_core::Param<IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryProvider2(pcontext : *mut core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : super::DWORD, ppprovider : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDiscoveryProvider2(pcontext.param().abi(), pconfigparams.map_or(core::ptr::null(), |slice| slice.as_ptr()), pconfigparams.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WSDCreateDiscoveryPublisher<P0>(pcontext: P0) -> windows_core::Result<IWSDiscoveryPublisher>
where
    P0: windows_core::Param<IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryPublisher(pcontext : *mut core::ffi::c_void, pppublisher : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDiscoveryPublisher(pcontext.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSDCreateDiscoveryPublisher2<P0>(pcontext: P0, pconfigparams: Option<&[WSD_CONFIG_PARAM]>) -> windows_core::Result<IWSDiscoveryPublisher>
where
    P0: windows_core::Param<IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryPublisher2(pcontext : *mut core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : super::DWORD, pppublisher : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDiscoveryPublisher2(pcontext.param().abi(), pconfigparams.map_or(core::ptr::null(), |slice| slice.as_ptr()), pconfigparams.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WSDCreateHttpAddress() -> windows_core::Result<IWSDHttpAddress> {
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateHttpAddress(ppaddress : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateHttpAddress(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WSDCreateHttpMessageParameters() -> windows_core::Result<IWSDHttpMessageParameters> {
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateHttpMessageParameters(pptxparams : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateHttpMessageParameters(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WSDCreateOutboundAttachment() -> windows_core::Result<IWSDOutboundAttachment> {
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateOutboundAttachment(ppattachment : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateOutboundAttachment(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WSDCreateUdpAddress() -> windows_core::Result<IWSDUdpAddress> {
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateUdpAddress(ppaddress : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateUdpAddress(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WSDCreateUdpMessageParameters(pptxparams: *mut Option<IWSDUdpMessageParameters>) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateUdpMessageParameters(pptxparams : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WSDCreateUdpMessageParameters(pptxparams as _) }
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSDGenerateFault<P0, P1, P2, P3, P4>(pszcode: P0, pszsubcode: P1, pszreason: P2, pszdetail: P3, pcontext: P4) -> windows_core::Result<*mut WSD_SOAP_FAULT>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDGenerateFault(pszcode : windows_core::PCWSTR, pszsubcode : windows_core::PCWSTR, pszreason : windows_core::PCWSTR, pszdetail : windows_core::PCWSTR, pcontext : *mut core::ffi::c_void, ppfault : *mut *mut WSD_SOAP_FAULT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDGenerateFault(pszcode.param().abi(), pszsubcode.param().abi(), pszreason.param().abi(), pszdetail.param().abi(), pcontext.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSDGenerateFaultEx<P3>(pcode: *const WSDXML_NAME, psubcode: Option<*const WSDXML_NAME>, preasons: *const WSD_LOCALIZED_STRING_LIST, pszdetail: P3) -> windows_core::Result<*mut WSD_SOAP_FAULT>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDGenerateFaultEx(pcode : *const WSDXML_NAME, psubcode : *const WSDXML_NAME, preasons : *const WSD_LOCALIZED_STRING_LIST, pszdetail : windows_core::PCWSTR, ppfault : *mut *mut WSD_SOAP_FAULT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDGenerateFaultEx(pcode, psubcode.unwrap_or(core::mem::zeroed()) as _, preasons, pszdetail.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WSDGetConfigurationOption(dwoption: super::DWORD, pvoid: super::LPVOID, cboutbuffer: super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDGetConfigurationOption(dwoption : super::DWORD, pvoid : super::LPVOID, cboutbuffer : super::DWORD) -> windows_core::HRESULT);
    unsafe { WSDGetConfigurationOption(dwoption, pvoid as _, cboutbuffer) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WSDSetConfigurationOption(dwoption: super::DWORD, pvoid: super::LPVOID, cbinbuffer: super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDSetConfigurationOption(dwoption : super::DWORD, pvoid : super::LPVOID, cbinbuffer : super::DWORD) -> windows_core::HRESULT);
    unsafe { WSDSetConfigurationOption(dwoption, pvoid, cbinbuffer) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WSDUriDecode(source: &[u16], destout: *mut windows_core::PWSTR, cchdestout: Option<*mut super::DWORD>) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDUriDecode(source : windows_core::PCWSTR, cchsource : super::DWORD, destout : *mut windows_core::PWSTR, cchdestout : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { WSDUriDecode(core::mem::transmute(source.as_ptr()), source.len().try_into().unwrap(), destout as _, cchdestout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WSDUriEncode(source: &[u16], destout: *mut windows_core::PWSTR, cchdestout: Option<*mut super::DWORD>) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDUriEncode(source : windows_core::PCWSTR, cchsource : super::DWORD, destout : *mut windows_core::PWSTR, cchdestout : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { WSDUriEncode(core::mem::transmute(source.as_ptr()), source.len().try_into().unwrap(), destout as _, cchdestout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSDXMLAddChild(pparent: *mut WSDXML_ELEMENT, pchild: *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDXMLAddChild(pparent : *mut WSDXML_ELEMENT, pchild : *mut WSDXML_ELEMENT) -> windows_core::HRESULT);
    unsafe { WSDXMLAddChild(pparent as _, pchild as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSDXMLAddSibling(pfirst: *mut WSDXML_ELEMENT, psecond: *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDXMLAddSibling(pfirst : *mut WSDXML_ELEMENT, psecond : *mut WSDXML_ELEMENT) -> windows_core::HRESULT);
    unsafe { WSDXMLAddSibling(pfirst as _, psecond as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSDXMLBuildAnyForSingleElement<P1>(pelementname: *mut WSDXML_NAME, psztext: P1) -> windows_core::Result<*mut WSDXML_ELEMENT>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDXMLBuildAnyForSingleElement(pelementname : *mut WSDXML_NAME, psztext : windows_core::PCWSTR, ppany : *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDXMLBuildAnyForSingleElement(pelementname as _, psztext.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSDXMLCleanupElement(pany: *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDXMLCleanupElement(pany : *mut WSDXML_ELEMENT) -> windows_core::HRESULT);
    unsafe { WSDXMLCleanupElement(pany as _) }
}
#[inline]
pub unsafe fn WSDXMLCreateContext() -> windows_core::Result<IWSDXMLContext> {
    windows_core::link!("wsdapi.dll" "system" fn WSDXMLCreateContext(ppcontext : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDXMLCreateContext(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSDXMLGetNameFromBuiltinNamespace<P0, P1>(psznamespace: P0, pszname: P1) -> windows_core::Result<*mut WSDXML_NAME>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDXMLGetNameFromBuiltinNamespace(psznamespace : windows_core::PCWSTR, pszname : windows_core::PCWSTR, ppname : *mut *mut WSDXML_NAME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDXMLGetNameFromBuiltinNamespace(psznamespace.param().abi(), pszname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSDXMLGetValueFromAny<P0, P1>(psznamespace: P0, pszname: P1, pany: *mut WSDXML_ELEMENT) -> windows_core::Result<windows_core::PCWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDXMLGetValueFromAny(psznamespace : windows_core::PCWSTR, pszname : windows_core::PCWSTR, pany : *mut WSDXML_ELEMENT, ppszvalue : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDXMLGetValueFromAny(psznamespace.param().abi(), pszname.param().abi(), pany as _, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "shared")]
pub const DPP_AppSequence: super::DWORD = 524299;
#[cfg(feature = "shared")]
pub const DPP_Bye: super::DWORD = 524289;
#[cfg(feature = "shared")]
pub const DPP_DiscoveryProxy: super::DWORD = 524305;
#[cfg(feature = "shared")]
pub const DPP_Element: super::DWORD = 524307;
#[cfg(feature = "shared")]
pub const DPP_Hello: super::DWORD = 524288;
#[cfg(feature = "shared")]
pub const DPP_Id: super::DWORD = 524309;
#[cfg(feature = "shared")]
pub const DPP_InstanceId: super::DWORD = 524300;
#[cfg(feature = "shared")]
pub const DPP_KeyId: super::DWORD = 524313;
#[cfg(feature = "shared")]
pub const DPP_MatchBy: super::DWORD = 524303;
#[cfg(feature = "shared")]
pub const DPP_MessageNumber: super::DWORD = 524302;
#[cfg(feature = "shared")]
pub const DPP_MetadataVersion: super::DWORD = 524297;
#[cfg(feature = "shared")]
pub const DPP_Probe: super::DWORD = 524290;
#[cfg(feature = "shared")]
pub const DPP_ProbeMatch: super::DWORD = 524292;
#[cfg(feature = "shared")]
pub const DPP_ProbeMatches: super::DWORD = 524291;
#[cfg(feature = "shared")]
pub const DPP_Refs: super::DWORD = 524314;
#[cfg(feature = "shared")]
pub const DPP_Resolve: super::DWORD = 524293;
#[cfg(feature = "shared")]
pub const DPP_ResolveMatch: super::DWORD = 524294;
#[cfg(feature = "shared")]
pub const DPP_ResolveMatches: super::DWORD = 524308;
#[cfg(feature = "shared")]
pub const DPP_Scheme: super::DWORD = 524312;
#[cfg(feature = "shared")]
pub const DPP_Scopes: super::DWORD = 524296;
#[cfg(feature = "shared")]
pub const DPP_Security: super::DWORD = 524310;
#[cfg(feature = "shared")]
pub const DPP_SequenceId: super::DWORD = 524301;
#[cfg(feature = "shared")]
pub const DPP_Sig: super::DWORD = 524311;
#[cfg(feature = "shared")]
pub const DPP_TargetService: super::DWORD = 524306;
#[cfg(feature = "shared")]
pub const DPP_Transport: super::DWORD = 524298;
#[cfg(feature = "shared")]
pub const DPP_Types: super::DWORD = 524295;
#[cfg(feature = "shared")]
pub const DPP_XAddrs: super::DWORD = 524304;
pub type DeviceDiscoveryMechanism = i32;
pub const DirectedDiscovery: DeviceDiscoveryMechanism = 1;
windows_core::imp::define_interface!(IWSDAddress, IWSDAddress_Vtbl, 0xb9574c6c_12a6_4f74_93a1_3318ff605759);
windows_core::imp::interface_hierarchy!(IWSDAddress, windows_core::IUnknown);
impl IWSDAddress {
    #[cfg(feature = "shared")]
    pub unsafe fn Serialize(&self, pszbuffer: windows_core::PWSTR, cchlength: super::DWORD, fsafe: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), pszbuffer, cchlength, fsafe.into()) }
    }
    pub unsafe fn Deserialize<P0>(&self, pszbuffer: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Deserialize)(windows_core::Interface::as_raw(self), pszbuffer.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAddress_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, super::DWORD, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Serialize: usize,
    pub Deserialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWSDAddress_Impl: windows_core::IUnknownImpl {
    fn Serialize(&self, pszbuffer: windows_core::PWSTR, cchlength: super::DWORD, fsafe: windows_core::BOOL) -> windows_core::Result<()>;
    fn Deserialize(&self, pszbuffer: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWSDAddress_Vtbl {
    pub const fn new<Identity: IWSDAddress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Serialize<Identity: IWSDAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbuffer: windows_core::PWSTR, cchlength: super::DWORD, fsafe: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDAddress_Impl::Serialize(this, core::mem::transmute_copy(&pszbuffer), core::mem::transmute_copy(&cchlength), core::mem::transmute_copy(&fsafe)).into()
            }
        }
        unsafe extern "system" fn Deserialize<Identity: IWSDAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbuffer: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDAddress_Impl::Deserialize(this, core::mem::transmute(&pszbuffer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Serialize: Serialize::<Identity, OFFSET>,
            Deserialize: Deserialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDAddress as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWSDAddress {}
windows_core::imp::define_interface!(IWSDAsyncCallback, IWSDAsyncCallback_Vtbl, 0xa63e109d_ce72_49e2_ba98_e845f5ee1666);
windows_core::imp::interface_hierarchy!(IWSDAsyncCallback, windows_core::IUnknown);
impl IWSDAsyncCallback {
    pub unsafe fn AsyncOperationComplete<P0, P1>(&self, pasyncresult: P0, pasyncstate: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDAsyncResult>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AsyncOperationComplete)(windows_core::Interface::as_raw(self), pasyncresult.param().abi(), pasyncstate.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAsyncCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AsyncOperationComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWSDAsyncCallback_Impl: windows_core::IUnknownImpl {
    fn AsyncOperationComplete(&self, pasyncresult: windows_core::Ref<IWSDAsyncResult>, pasyncstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IWSDAsyncCallback_Vtbl {
    pub const fn new<Identity: IWSDAsyncCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AsyncOperationComplete<Identity: IWSDAsyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasyncresult: *mut core::ffi::c_void, pasyncstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDAsyncCallback_Impl::AsyncOperationComplete(this, core::mem::transmute_copy(&pasyncresult), core::mem::transmute_copy(&pasyncstate)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AsyncOperationComplete: AsyncOperationComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDAsyncCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDAsyncCallback {}
windows_core::imp::define_interface!(IWSDAsyncResult, IWSDAsyncResult_Vtbl, 0x11a9852a_8dd8_423e_b537_9356db4fbfb8);
windows_core::imp::interface_hierarchy!(IWSDAsyncResult, windows_core::IUnknown);
impl IWSDAsyncResult {
    pub unsafe fn SetCallback<P0, P1>(&self, pcallback: P0, pasyncstate: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDAsyncCallback>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCallback)(windows_core::Interface::as_raw(self), pcallback.param().abi(), pasyncstate.param().abi()) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn SetWaitHandle(&self, hwaithandle: super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWaitHandle)(windows_core::Interface::as_raw(self), hwaithandle) }
    }
    pub unsafe fn HasCompleted(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HasCompleted)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetAsyncState(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAsyncState)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Abort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetEvent(&self, pevent: *mut WSD_EVENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEvent)(windows_core::Interface::as_raw(self), pevent) }
    }
    pub unsafe fn GetEndpointProxy(&self) -> windows_core::Result<IWSDEndpointProxy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEndpointProxy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAsyncResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub SetWaitHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetWaitHandle: usize,
    pub HasCompleted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAsyncState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSD_EVENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetEvent: usize,
    pub GetEndpointProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWSDAsyncResult_Impl: windows_core::IUnknownImpl {
    fn SetCallback(&self, pcallback: windows_core::Ref<IWSDAsyncCallback>, pasyncstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetWaitHandle(&self, hwaithandle: super::HANDLE) -> windows_core::Result<()>;
    fn HasCompleted(&self) -> windows_core::Result<()>;
    fn GetAsyncState(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn GetEvent(&self, pevent: *mut WSD_EVENT) -> windows_core::Result<()>;
    fn GetEndpointProxy(&self) -> windows_core::Result<IWSDEndpointProxy>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWSDAsyncResult_Vtbl {
    pub const fn new<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCallback<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pasyncstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDAsyncResult_Impl::SetCallback(this, core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pasyncstate)).into()
            }
        }
        unsafe extern "system" fn SetWaitHandle<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwaithandle: super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDAsyncResult_Impl::SetWaitHandle(this, core::mem::transmute_copy(&hwaithandle)).into()
            }
        }
        unsafe extern "system" fn HasCompleted<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDAsyncResult_Impl::HasCompleted(this).into()
            }
        }
        unsafe extern "system" fn GetAsyncState<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasyncstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDAsyncResult_Impl::GetAsyncState(this) {
                    Ok(ok__) => {
                        ppasyncstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Abort<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDAsyncResult_Impl::Abort(this).into()
            }
        }
        unsafe extern "system" fn GetEvent<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut WSD_EVENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDAsyncResult_Impl::GetEvent(this, core::mem::transmute_copy(&pevent)).into()
            }
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppendpoint: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDAsyncResult_Impl::GetEndpointProxy(this) {
                    Ok(ok__) => {
                        ppendpoint.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCallback: SetCallback::<Identity, OFFSET>,
            SetWaitHandle: SetWaitHandle::<Identity, OFFSET>,
            HasCompleted: HasCompleted::<Identity, OFFSET>,
            GetAsyncState: GetAsyncState::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            GetEvent: GetEvent::<Identity, OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDAsyncResult as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWSDAsyncResult {}
windows_core::imp::define_interface!(IWSDAttachment, IWSDAttachment_Vtbl, 0x5d55a616_9df8_4b09_b156_9ba351a48b76);
windows_core::imp::interface_hierarchy!(IWSDAttachment, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAttachment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IWSDAttachment_Impl: windows_core::IUnknownImpl {}
impl IWSDAttachment_Vtbl {
    pub const fn new<Identity: IWSDAttachment_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDAttachment as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDAttachment {}
windows_core::imp::define_interface!(IWSDDeviceHost, IWSDDeviceHost_Vtbl, 0x917fe891_3d13_4138_9809_934c8abeb12c);
windows_core::imp::interface_hierarchy!(IWSDDeviceHost, windows_core::IUnknown);
impl IWSDDeviceHost {
    #[cfg(feature = "shared")]
    pub unsafe fn Init<P0, P1>(&self, pszlocalid: P0, pcontext: P1, pphostaddresses: Option<&[Option<IWSDAddress>]>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWSDXMLContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), pszlocalid.param().abi(), pcontext.param().abi(), core::mem::transmute(pphostaddresses.map_or(core::ptr::null(), |slice| slice.as_ptr())), pphostaddresses.map_or(0, |slice| slice.len().try_into().unwrap())) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn Start<P2>(&self, ullinstanceid: super::ULONGLONG, pscopelist: *const WSD_URI_LIST, pnotificationsink: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWSDDeviceHostNotify>,
    {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), ullinstanceid, pscopelist, pnotificationsink.param().abi()) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Terminate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn RegisterPortType(&self, pporttype: *const WSD_PORT_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterPortType)(windows_core::Interface::as_raw(self), pporttype) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SetMetadata(&self, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: Option<*const WSD_HOST_METADATA>, pcustommetadata: Option<*const WSD_METADATA_SECTION_LIST>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMetadata)(windows_core::Interface::as_raw(self), pthismodelmetadata, pthisdevicemetadata, phostmetadata.unwrap_or(core::mem::zeroed()) as _, pcustommetadata.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn RegisterService<P0, P1>(&self, pszserviceid: P0, pservice: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterService)(windows_core::Interface::as_raw(self), pszserviceid.param().abi(), pservice.param().abi()) }
    }
    pub unsafe fn RetireService<P0>(&self, pszserviceid: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RetireService)(windows_core::Interface::as_raw(self), pszserviceid.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn AddDynamicService<P0, P1, P5>(&self, pszserviceid: P0, pszendpointaddress: P1, pporttype: Option<*const WSD_PORT_TYPE>, pportname: Option<*const WSDXML_NAME>, pany: Option<*const WSDXML_ELEMENT>, pservice: P5) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDynamicService)(windows_core::Interface::as_raw(self), pszserviceid.param().abi(), pszendpointaddress.param().abi(), pporttype.unwrap_or(core::mem::zeroed()) as _, pportname.unwrap_or(core::mem::zeroed()) as _, pany.unwrap_or(core::mem::zeroed()) as _, pservice.param().abi()) }
    }
    pub unsafe fn RemoveDynamicService<P0>(&self, pszserviceid: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveDynamicService)(windows_core::Interface::as_raw(self), pszserviceid.param().abi()) }
    }
    pub unsafe fn SetServiceDiscoverable<P0>(&self, pszserviceid: P0, fdiscoverable: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetServiceDiscoverable)(windows_core::Interface::as_raw(self), pszserviceid.param().abi(), fdiscoverable.into()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SignalEvent<P0>(&self, pszserviceid: P0, pbody: Option<*const core::ffi::c_void>, poperation: *const WSD_OPERATION) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SignalEvent)(windows_core::Interface::as_raw(self), pszserviceid.param().abi(), pbody.unwrap_or(core::mem::zeroed()) as _, poperation) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *const *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Init: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONGLONG, *const WSD_URI_LIST, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    Start: usize,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub RegisterPortType: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_PORT_TYPE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    RegisterPortType: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_THIS_MODEL_METADATA, *const WSD_THIS_DEVICE_METADATA, *const WSD_HOST_METADATA, *const WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SetMetadata: usize,
    pub RegisterService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RetireService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub AddDynamicService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const WSD_PORT_TYPE, *const WSDXML_NAME, *const WSDXML_ELEMENT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    AddDynamicService: usize,
    pub RemoveDynamicService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetServiceDiscoverable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SignalEvent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const core::ffi::c_void, *const WSD_OPERATION) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SignalEvent: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWSDDeviceHost_Impl: windows_core::IUnknownImpl {
    fn Init(&self, pszlocalid: &windows_core::PCWSTR, pcontext: windows_core::Ref<IWSDXMLContext>, pphostaddresses: *const Option<IWSDAddress>, dwhostaddresscount: super::DWORD) -> windows_core::Result<()>;
    fn Start(&self, ullinstanceid: super::ULONGLONG, pscopelist: *const WSD_URI_LIST, pnotificationsink: windows_core::Ref<IWSDDeviceHostNotify>) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Terminate(&self) -> windows_core::Result<()>;
    fn RegisterPortType(&self, pporttype: *const WSD_PORT_TYPE) -> windows_core::Result<()>;
    fn SetMetadata(&self, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> windows_core::Result<()>;
    fn RegisterService(&self, pszserviceid: &windows_core::PCWSTR, pservice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RetireService(&self, pszserviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddDynamicService(&self, pszserviceid: &windows_core::PCWSTR, pszendpointaddress: &windows_core::PCWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveDynamicService(&self, pszserviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetServiceDiscoverable(&self, pszserviceid: &windows_core::PCWSTR, fdiscoverable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SignalEvent(&self, pszserviceid: &windows_core::PCWSTR, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWSDDeviceHost_Vtbl {
    pub const fn new<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszlocalid: windows_core::PCWSTR, pcontext: *mut core::ffi::c_void, pphostaddresses: *const *mut core::ffi::c_void, dwhostaddresscount: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::Init(this, core::mem::transmute(&pszlocalid), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pphostaddresses), core::mem::transmute_copy(&dwhostaddresscount)).into()
            }
        }
        unsafe extern "system" fn Start<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullinstanceid: super::ULONGLONG, pscopelist: *const WSD_URI_LIST, pnotificationsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::Start(this, core::mem::transmute_copy(&ullinstanceid), core::mem::transmute_copy(&pscopelist), core::mem::transmute_copy(&pnotificationsink)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Terminate<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::Terminate(this).into()
            }
        }
        unsafe extern "system" fn RegisterPortType<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pporttype: *const WSD_PORT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::RegisterPortType(this, core::mem::transmute_copy(&pporttype)).into()
            }
        }
        unsafe extern "system" fn SetMetadata<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::SetMetadata(this, core::mem::transmute_copy(&pthismodelmetadata), core::mem::transmute_copy(&pthisdevicemetadata), core::mem::transmute_copy(&phostmetadata), core::mem::transmute_copy(&pcustommetadata)).into()
            }
        }
        unsafe extern "system" fn RegisterService<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, pservice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::RegisterService(this, core::mem::transmute(&pszserviceid), core::mem::transmute_copy(&pservice)).into()
            }
        }
        unsafe extern "system" fn RetireService<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::RetireService(this, core::mem::transmute(&pszserviceid)).into()
            }
        }
        unsafe extern "system" fn AddDynamicService<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, pszendpointaddress: windows_core::PCWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::AddDynamicService(this, core::mem::transmute(&pszserviceid), core::mem::transmute(&pszendpointaddress), core::mem::transmute_copy(&pporttype), core::mem::transmute_copy(&pportname), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&pservice)).into()
            }
        }
        unsafe extern "system" fn RemoveDynamicService<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::RemoveDynamicService(this, core::mem::transmute(&pszserviceid)).into()
            }
        }
        unsafe extern "system" fn SetServiceDiscoverable<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, fdiscoverable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::SetServiceDiscoverable(this, core::mem::transmute(&pszserviceid), core::mem::transmute_copy(&fdiscoverable)).into()
            }
        }
        unsafe extern "system" fn SignalEvent<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::SignalEvent(this, core::mem::transmute(&pszserviceid), core::mem::transmute_copy(&pbody), core::mem::transmute_copy(&poperation)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
            RegisterPortType: RegisterPortType::<Identity, OFFSET>,
            SetMetadata: SetMetadata::<Identity, OFFSET>,
            RegisterService: RegisterService::<Identity, OFFSET>,
            RetireService: RetireService::<Identity, OFFSET>,
            AddDynamicService: AddDynamicService::<Identity, OFFSET>,
            RemoveDynamicService: RemoveDynamicService::<Identity, OFFSET>,
            SetServiceDiscoverable: SetServiceDiscoverable::<Identity, OFFSET>,
            SignalEvent: SignalEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDDeviceHost as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWSDDeviceHost {}
windows_core::imp::define_interface!(IWSDDeviceHostNotify, IWSDDeviceHostNotify_Vtbl, 0xb5bee9f9_eeda_41fe_96f7_f45e14990fb0);
windows_core::imp::interface_hierarchy!(IWSDDeviceHostNotify, windows_core::IUnknown);
impl IWSDDeviceHostNotify {
    pub unsafe fn GetService<P0>(&self, pszserviceid: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), pszserviceid.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHostNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWSDDeviceHostNotify_Impl: windows_core::IUnknownImpl {
    fn GetService(&self, pszserviceid: &windows_core::PCWSTR) -> windows_core::Result<windows_core::IUnknown>;
}
impl IWSDDeviceHostNotify_Vtbl {
    pub const fn new<Identity: IWSDDeviceHostNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetService<Identity: IWSDDeviceHostNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceHostNotify_Impl::GetService(this, core::mem::transmute(&pszserviceid)) {
                    Ok(ok__) => {
                        ppservice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetService: GetService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDDeviceHostNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDDeviceHostNotify {}
windows_core::imp::define_interface!(IWSDDeviceProxy, IWSDDeviceProxy_Vtbl, 0xeee0c031_c578_4c0e_9a3b_973c35f409db);
windows_core::imp::interface_hierarchy!(IWSDDeviceProxy, windows_core::IUnknown);
impl IWSDDeviceProxy {
    pub unsafe fn Init<P0, P1, P2, P3, P4>(&self, pszdeviceid: P0, pdeviceaddress: P1, pszlocalid: P2, pcontext: P3, psponsor: P4) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWSDAddress>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<IWSDXMLContext>,
        P4: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), pszdeviceid.param().abi(), pdeviceaddress.param().abi(), pszlocalid.param().abi(), pcontext.param().abi(), psponsor.param().abi()) }
    }
    pub unsafe fn BeginGetMetadata(&self) -> windows_core::Result<IWSDAsyncResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginGetMetadata)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn EndGetMetadata<P0>(&self, presult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndGetMetadata)(windows_core::Interface::as_raw(self), presult.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetHostMetadata(&self) -> windows_core::Result<*mut WSD_HOST_METADATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHostMetadata)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetThisModelMetadata(&self) -> windows_core::Result<*mut WSD_THIS_MODEL_METADATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThisModelMetadata)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetThisDeviceMetadata(&self) -> windows_core::Result<*mut WSD_THIS_DEVICE_METADATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThisDeviceMetadata)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetAllMetadata(&self) -> windows_core::Result<*mut WSD_METADATA_SECTION_LIST> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllMetadata)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetServiceProxyById<P0>(&self, pszserviceid: P0) -> windows_core::Result<IWSDServiceProxy>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetServiceProxyById)(windows_core::Interface::as_raw(self), pszserviceid.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetServiceProxyByType(&self, ptype: *const WSDXML_NAME) -> windows_core::Result<IWSDServiceProxy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetServiceProxyByType)(windows_core::Interface::as_raw(self), ptype, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEndpointProxy(&self) -> windows_core::Result<IWSDEndpointProxy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEndpointProxy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceProxy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginGetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndGetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetHostMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WSD_HOST_METADATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetHostMetadata: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetThisModelMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WSD_THIS_MODEL_METADATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetThisModelMetadata: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetThisDeviceMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WSD_THIS_DEVICE_METADATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetThisDeviceMetadata: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetAllMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetAllMetadata: usize,
    pub GetServiceProxyById: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetServiceProxyByType: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSDXML_NAME, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetServiceProxyByType: usize,
    pub GetEndpointProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWSDDeviceProxy_Impl: windows_core::IUnknownImpl {
    fn Init(&self, pszdeviceid: &windows_core::PCWSTR, pdeviceaddress: windows_core::Ref<IWSDAddress>, pszlocalid: &windows_core::PCWSTR, pcontext: windows_core::Ref<IWSDXMLContext>, psponsor: windows_core::Ref<IWSDDeviceProxy>) -> windows_core::Result<()>;
    fn BeginGetMetadata(&self) -> windows_core::Result<IWSDAsyncResult>;
    fn EndGetMetadata(&self, presult: windows_core::Ref<IWSDAsyncResult>) -> windows_core::Result<()>;
    fn GetHostMetadata(&self) -> windows_core::Result<*mut WSD_HOST_METADATA>;
    fn GetThisModelMetadata(&self) -> windows_core::Result<*mut WSD_THIS_MODEL_METADATA>;
    fn GetThisDeviceMetadata(&self) -> windows_core::Result<*mut WSD_THIS_DEVICE_METADATA>;
    fn GetAllMetadata(&self) -> windows_core::Result<*mut WSD_METADATA_SECTION_LIST>;
    fn GetServiceProxyById(&self, pszserviceid: &windows_core::PCWSTR) -> windows_core::Result<IWSDServiceProxy>;
    fn GetServiceProxyByType(&self, ptype: *const WSDXML_NAME) -> windows_core::Result<IWSDServiceProxy>;
    fn GetEndpointProxy(&self) -> windows_core::Result<IWSDEndpointProxy>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWSDDeviceProxy_Vtbl {
    pub const fn new<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdeviceid: windows_core::PCWSTR, pdeviceaddress: *mut core::ffi::c_void, pszlocalid: windows_core::PCWSTR, pcontext: *mut core::ffi::c_void, psponsor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceProxy_Impl::Init(this, core::mem::transmute(&pszdeviceid), core::mem::transmute_copy(&pdeviceaddress), core::mem::transmute(&pszlocalid), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&psponsor)).into()
            }
        }
        unsafe extern "system" fn BeginGetMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceProxy_Impl::BeginGetMetadata(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndGetMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceProxy_Impl::EndGetMetadata(this, core::mem::transmute_copy(&presult)).into()
            }
        }
        unsafe extern "system" fn GetHostMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphostmetadata: *mut *mut WSD_HOST_METADATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceProxy_Impl::GetHostMetadata(this) {
                    Ok(ok__) => {
                        pphostmetadata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetThisModelMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmanufacturermetadata: *mut *mut WSD_THIS_MODEL_METADATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceProxy_Impl::GetThisModelMetadata(this) {
                    Ok(ok__) => {
                        ppmanufacturermetadata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetThisDeviceMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppthisdevicemetadata: *mut *mut WSD_THIS_DEVICE_METADATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceProxy_Impl::GetThisDeviceMetadata(this) {
                    Ok(ok__) => {
                        ppthisdevicemetadata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAllMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceProxy_Impl::GetAllMetadata(this) {
                    Ok(ok__) => {
                        ppmetadata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetServiceProxyById<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, ppserviceproxy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceProxy_Impl::GetServiceProxyById(this, core::mem::transmute(&pszserviceid)) {
                    Ok(ok__) => {
                        ppserviceproxy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetServiceProxyByType<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *const WSDXML_NAME, ppserviceproxy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceProxy_Impl::GetServiceProxyByType(this, core::mem::transmute_copy(&ptype)) {
                    Ok(ok__) => {
                        ppserviceproxy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproxy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceProxy_Impl::GetEndpointProxy(this) {
                    Ok(ok__) => {
                        ppproxy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            BeginGetMetadata: BeginGetMetadata::<Identity, OFFSET>,
            EndGetMetadata: EndGetMetadata::<Identity, OFFSET>,
            GetHostMetadata: GetHostMetadata::<Identity, OFFSET>,
            GetThisModelMetadata: GetThisModelMetadata::<Identity, OFFSET>,
            GetThisDeviceMetadata: GetThisDeviceMetadata::<Identity, OFFSET>,
            GetAllMetadata: GetAllMetadata::<Identity, OFFSET>,
            GetServiceProxyById: GetServiceProxyById::<Identity, OFFSET>,
            GetServiceProxyByType: GetServiceProxyByType::<Identity, OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDDeviceProxy as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWSDDeviceProxy {}
windows_core::imp::define_interface!(IWSDEndpointProxy, IWSDEndpointProxy_Vtbl, 0x1860d430_b24c_4975_9f90_dbb39baa24ec);
windows_core::imp::interface_hierarchy!(IWSDEndpointProxy, windows_core::IUnknown);
impl IWSDEndpointProxy {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SendOneWayRequest(&self, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendOneWayRequest)(windows_core::Interface::as_raw(self), pbody, poperation) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SendTwoWayRequest(&self, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: Option<*const WSD_SYNCHRONOUS_RESPONSE_CONTEXT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendTwoWayRequest)(windows_core::Interface::as_raw(self), pbody, poperation, presponsecontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SendTwoWayRequestAsync<P2, P3>(&self, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: P2, pcallback: P3) -> windows_core::Result<IWSDAsyncResult>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<IWSDAsyncCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SendTwoWayRequestAsync)(windows_core::Interface::as_raw(self), pbody, poperation, pasyncstate.param().abi(), pcallback.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn AbortAsyncOperation<P0>(&self, pasyncresult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).AbortAsyncOperation)(windows_core::Interface::as_raw(self), pasyncresult.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn ProcessFault(&self, pfault: *const WSD_SOAP_FAULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessFault)(windows_core::Interface::as_raw(self), pfault) }
    }
    pub unsafe fn GetErrorInfo(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetFaultInfo(&self) -> windows_core::Result<*mut WSD_SOAP_FAULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFaultInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEndpointProxy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SendOneWayRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const WSD_OPERATION) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SendOneWayRequest: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SendTwoWayRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const WSD_OPERATION, *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SendTwoWayRequest: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SendTwoWayRequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const WSD_OPERATION, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SendTwoWayRequestAsync: usize,
    pub AbortAsyncOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub ProcessFault: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_SOAP_FAULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    ProcessFault: usize,
    pub GetErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetFaultInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WSD_SOAP_FAULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetFaultInfo: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWSDEndpointProxy_Impl: windows_core::IUnknownImpl {
    fn SendOneWayRequest(&self, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION) -> windows_core::Result<()>;
    fn SendTwoWayRequest(&self, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> windows_core::Result<()>;
    fn SendTwoWayRequestAsync(&self, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: windows_core::Ref<windows_core::IUnknown>, pcallback: windows_core::Ref<IWSDAsyncCallback>) -> windows_core::Result<IWSDAsyncResult>;
    fn AbortAsyncOperation(&self, pasyncresult: windows_core::Ref<IWSDAsyncResult>) -> windows_core::Result<()>;
    fn ProcessFault(&self, pfault: *const WSD_SOAP_FAULT) -> windows_core::Result<()>;
    fn GetErrorInfo(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetFaultInfo(&self) -> windows_core::Result<*mut WSD_SOAP_FAULT>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWSDEndpointProxy_Vtbl {
    pub const fn new<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendOneWayRequest<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEndpointProxy_Impl::SendOneWayRequest(this, core::mem::transmute_copy(&pbody), core::mem::transmute_copy(&poperation)).into()
            }
        }
        unsafe extern "system" fn SendTwoWayRequest<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEndpointProxy_Impl::SendTwoWayRequest(this, core::mem::transmute_copy(&pbody), core::mem::transmute_copy(&poperation), core::mem::transmute_copy(&presponsecontext)).into()
            }
        }
        unsafe extern "system" fn SendTwoWayRequestAsync<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, presult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDEndpointProxy_Impl::SendTwoWayRequestAsync(this, core::mem::transmute_copy(&pbody), core::mem::transmute_copy(&poperation), core::mem::transmute_copy(&pasyncstate), core::mem::transmute_copy(&pcallback)) {
                    Ok(ok__) => {
                        presult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AbortAsyncOperation<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasyncresult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEndpointProxy_Impl::AbortAsyncOperation(this, core::mem::transmute_copy(&pasyncresult)).into()
            }
        }
        unsafe extern "system" fn ProcessFault<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEndpointProxy_Impl::ProcessFault(this, core::mem::transmute_copy(&pfault)).into()
            }
        }
        unsafe extern "system" fn GetErrorInfo<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszerrorinfo: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDEndpointProxy_Impl::GetErrorInfo(this) {
                    Ok(ok__) => {
                        ppszerrorinfo.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFaultInfo<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfault: *mut *mut WSD_SOAP_FAULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDEndpointProxy_Impl::GetFaultInfo(this) {
                    Ok(ok__) => {
                        ppfault.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendOneWayRequest: SendOneWayRequest::<Identity, OFFSET>,
            SendTwoWayRequest: SendTwoWayRequest::<Identity, OFFSET>,
            SendTwoWayRequestAsync: SendTwoWayRequestAsync::<Identity, OFFSET>,
            AbortAsyncOperation: AbortAsyncOperation::<Identity, OFFSET>,
            ProcessFault: ProcessFault::<Identity, OFFSET>,
            GetErrorInfo: GetErrorInfo::<Identity, OFFSET>,
            GetFaultInfo: GetFaultInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDEndpointProxy as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWSDEndpointProxy {}
windows_core::imp::define_interface!(IWSDEventingStatus, IWSDEventingStatus_Vtbl, 0x49b17f52_637a_407a_ae99_fbe82a4d38c0);
windows_core::imp::interface_hierarchy!(IWSDEventingStatus, windows_core::IUnknown);
impl IWSDEventingStatus {
    pub unsafe fn SubscriptionRenewed<P0>(&self, pszsubscriptionaction: P0)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SubscriptionRenewed)(windows_core::Interface::as_raw(self), pszsubscriptionaction.param().abi());
        }
    }
    pub unsafe fn SubscriptionRenewalFailed<P0>(&self, pszsubscriptionaction: P0, hr: windows_core::HRESULT)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SubscriptionRenewalFailed)(windows_core::Interface::as_raw(self), pszsubscriptionaction.param().abi(), hr);
        }
    }
    pub unsafe fn SubscriptionEnded<P0>(&self, pszsubscriptionaction: P0)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SubscriptionEnded)(windows_core::Interface::as_raw(self), pszsubscriptionaction.param().abi());
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEventingStatus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SubscriptionRenewed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR),
    pub SubscriptionRenewalFailed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::HRESULT),
    pub SubscriptionEnded: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR),
}
pub trait IWSDEventingStatus_Impl: windows_core::IUnknownImpl {
    fn SubscriptionRenewed(&self, pszsubscriptionaction: &windows_core::PCWSTR);
    fn SubscriptionRenewalFailed(&self, pszsubscriptionaction: &windows_core::PCWSTR, hr: windows_core::HRESULT);
    fn SubscriptionEnded(&self, pszsubscriptionaction: &windows_core::PCWSTR);
}
impl IWSDEventingStatus_Vtbl {
    pub const fn new<Identity: IWSDEventingStatus_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SubscriptionRenewed<Identity: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubscriptionaction: windows_core::PCWSTR) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEventingStatus_Impl::SubscriptionRenewed(this, core::mem::transmute(&pszsubscriptionaction));
            }
        }
        unsafe extern "system" fn SubscriptionRenewalFailed<Identity: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubscriptionaction: windows_core::PCWSTR, hr: windows_core::HRESULT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEventingStatus_Impl::SubscriptionRenewalFailed(this, core::mem::transmute(&pszsubscriptionaction), core::mem::transmute_copy(&hr));
            }
        }
        unsafe extern "system" fn SubscriptionEnded<Identity: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubscriptionaction: windows_core::PCWSTR) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEventingStatus_Impl::SubscriptionEnded(this, core::mem::transmute(&pszsubscriptionaction));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SubscriptionRenewed: SubscriptionRenewed::<Identity, OFFSET>,
            SubscriptionRenewalFailed: SubscriptionRenewalFailed::<Identity, OFFSET>,
            SubscriptionEnded: SubscriptionEnded::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDEventingStatus as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDEventingStatus {}
windows_core::imp::define_interface!(IWSDHttpAddress, IWSDHttpAddress_Vtbl, 0xd09ac7bd_2a3e_4b85_8605_2737ff3e4ea0);
impl core::ops::Deref for IWSDHttpAddress {
    type Target = IWSDTransportAddress;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDHttpAddress, windows_core::IUnknown, IWSDAddress, IWSDTransportAddress);
impl IWSDHttpAddress {
    pub unsafe fn GetSecure(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSecure)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetSecure(&self, fsecure: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSecure)(windows_core::Interface::as_raw(self), fsecure.into()) }
    }
    pub unsafe fn GetPath(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPath<P0>(&self, pszpath: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), pszpath.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpAddress_Vtbl {
    pub base__: IWSDTransportAddress_Vtbl,
    pub GetSecure: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecure: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWSDHttpAddress_Impl: IWSDTransportAddress_Impl {
    fn GetSecure(&self) -> windows_core::Result<()>;
    fn SetSecure(&self, fsecure: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetPath(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetPath(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWSDHttpAddress_Vtbl {
    pub const fn new<Identity: IWSDHttpAddress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSecure<Identity: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpAddress_Impl::GetSecure(this).into()
            }
        }
        unsafe extern "system" fn SetSecure<Identity: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsecure: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpAddress_Impl::SetSecure(this, core::mem::transmute_copy(&fsecure)).into()
            }
        }
        unsafe extern "system" fn GetPath<Identity: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpath: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDHttpAddress_Impl::GetPath(this) {
                    Ok(ok__) => {
                        ppszpath.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPath<Identity: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpAddress_Impl::SetPath(this, core::mem::transmute(&pszpath)).into()
            }
        }
        Self {
            base__: IWSDTransportAddress_Vtbl::new::<Identity, OFFSET>(),
            GetSecure: GetSecure::<Identity, OFFSET>,
            SetSecure: SetSecure::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDHttpAddress as windows_core::Interface>::IID || iid == &<IWSDAddress as windows_core::Interface>::IID || iid == &<IWSDTransportAddress as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWSDHttpAddress {}
windows_core::imp::define_interface!(IWSDHttpAuthParameters, IWSDHttpAuthParameters_Vtbl, 0x0b476df0_8dac_480d_b05c_99781a5884aa);
windows_core::imp::interface_hierarchy!(IWSDHttpAuthParameters, windows_core::IUnknown);
impl IWSDHttpAuthParameters {
    #[cfg(feature = "winnt")]
    pub unsafe fn GetClientAccessToken(&self, phtoken: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClientAccessToken)(windows_core::Interface::as_raw(self), phtoken as _) }
    }
    pub unsafe fn GetAuthType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAuthType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpAuthParameters_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub GetClientAccessToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetClientAccessToken: usize,
    pub GetAuthType: unsafe extern "system" fn(*mut core::ffi::c_void, PWSD_SECURITY_HTTP_AUTH_SCHEMES) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait IWSDHttpAuthParameters_Impl: windows_core::IUnknownImpl {
    fn GetClientAccessToken(&self, phtoken: *mut super::HANDLE) -> windows_core::Result<()>;
    fn GetAuthType(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "winnt")]
impl IWSDHttpAuthParameters_Vtbl {
    pub const fn new<Identity: IWSDHttpAuthParameters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClientAccessToken<Identity: IWSDHttpAuthParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phtoken: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpAuthParameters_Impl::GetClientAccessToken(this, core::mem::transmute_copy(&phtoken)).into()
            }
        }
        unsafe extern "system" fn GetAuthType<Identity: IWSDHttpAuthParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthtype: PWSD_SECURITY_HTTP_AUTH_SCHEMES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDHttpAuthParameters_Impl::GetAuthType(this) {
                    Ok(ok__) => {
                        pauthtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClientAccessToken: GetClientAccessToken::<Identity, OFFSET>,
            GetAuthType: GetAuthType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDHttpAuthParameters as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IWSDHttpAuthParameters {}
windows_core::imp::define_interface!(IWSDHttpMessageParameters, IWSDHttpMessageParameters_Vtbl, 0x540bd122_5c83_4dec_b396_ea62a2697fdf);
impl core::ops::Deref for IWSDHttpMessageParameters {
    type Target = IWSDMessageParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDHttpMessageParameters, windows_core::IUnknown, IWSDMessageParameters);
impl IWSDHttpMessageParameters {
    pub unsafe fn SetInboundHttpHeaders<P0>(&self, pszheaders: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInboundHttpHeaders)(windows_core::Interface::as_raw(self), pszheaders.param().abi()) }
    }
    pub unsafe fn GetInboundHttpHeaders(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInboundHttpHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOutboundHttpHeaders<P0>(&self, pszheaders: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutboundHttpHeaders)(windows_core::Interface::as_raw(self), pszheaders.param().abi()) }
    }
    pub unsafe fn GetOutboundHttpHeaders(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutboundHttpHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetID<P0>(&self, pszid: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetID)(windows_core::Interface::as_raw(self), pszid.param().abi()) }
    }
    pub unsafe fn GetID(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetContext<P0>(&self, pcontext: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), pcontext.param().abi()) }
    }
    pub unsafe fn GetContext(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpMessageParameters_Vtbl {
    pub base__: IWSDMessageParameters_Vtbl,
    pub SetInboundHttpHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetInboundHttpHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetOutboundHttpHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetOutboundHttpHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWSDHttpMessageParameters_Impl: IWSDMessageParameters_Impl {
    fn SetInboundHttpHeaders(&self, pszheaders: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetInboundHttpHeaders(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetOutboundHttpHeaders(&self, pszheaders: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetOutboundHttpHeaders(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetID(&self, pszid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetID(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetContext(&self, pcontext: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetContext(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl IWSDHttpMessageParameters_Vtbl {
    pub const fn new<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetInboundHttpHeaders<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszheaders: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpMessageParameters_Impl::SetInboundHttpHeaders(this, core::mem::transmute(&pszheaders)).into()
            }
        }
        unsafe extern "system" fn GetInboundHttpHeaders<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszheaders: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDHttpMessageParameters_Impl::GetInboundHttpHeaders(this) {
                    Ok(ok__) => {
                        ppszheaders.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutboundHttpHeaders<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszheaders: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpMessageParameters_Impl::SetOutboundHttpHeaders(this, core::mem::transmute(&pszheaders)).into()
            }
        }
        unsafe extern "system" fn GetOutboundHttpHeaders<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszheaders: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDHttpMessageParameters_Impl::GetOutboundHttpHeaders(this) {
                    Ok(ok__) => {
                        ppszheaders.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetID<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpMessageParameters_Impl::SetID(this, core::mem::transmute(&pszid)).into()
            }
        }
        unsafe extern "system" fn GetID<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszid: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDHttpMessageParameters_Impl::GetID(this) {
                    Ok(ok__) => {
                        ppszid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContext<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpMessageParameters_Impl::SetContext(this, core::mem::transmute_copy(&pcontext)).into()
            }
        }
        unsafe extern "system" fn GetContext<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDHttpMessageParameters_Impl::GetContext(this) {
                    Ok(ok__) => {
                        ppcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clear<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpMessageParameters_Impl::Clear(this).into()
            }
        }
        Self {
            base__: IWSDMessageParameters_Vtbl::new::<Identity, OFFSET>(),
            SetInboundHttpHeaders: SetInboundHttpHeaders::<Identity, OFFSET>,
            GetInboundHttpHeaders: GetInboundHttpHeaders::<Identity, OFFSET>,
            SetOutboundHttpHeaders: SetOutboundHttpHeaders::<Identity, OFFSET>,
            GetOutboundHttpHeaders: GetOutboundHttpHeaders::<Identity, OFFSET>,
            SetID: SetID::<Identity, OFFSET>,
            GetID: GetID::<Identity, OFFSET>,
            SetContext: SetContext::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDHttpMessageParameters as windows_core::Interface>::IID || iid == &<IWSDMessageParameters as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDHttpMessageParameters {}
windows_core::imp::define_interface!(IWSDInboundAttachment, IWSDInboundAttachment_Vtbl, 0x5bd6ca65_233c_4fb8_9f7a_2641619655c9);
impl core::ops::Deref for IWSDInboundAttachment {
    type Target = IWSDAttachment;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDInboundAttachment, windows_core::IUnknown, IWSDAttachment);
impl IWSDInboundAttachment {
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn Read(&self, pbuffer: *mut super::BYTE, dwbytestoread: super::DWORD, pdwnumberofbytesread: super::LPDWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pbuffer as _, dwbytestoread, pdwnumberofbytesread as _) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDInboundAttachment_Vtbl {
    pub base__: IWSDAttachment_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BYTE, super::DWORD, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    Read: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IWSDInboundAttachment_Impl: IWSDAttachment_Impl {
    fn Read(&self, pbuffer: *mut super::BYTE, dwbytestoread: super::DWORD, pdwnumberofbytesread: super::LPDWORD) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IWSDInboundAttachment_Vtbl {
    pub const fn new<Identity: IWSDInboundAttachment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: IWSDInboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut super::BYTE, dwbytestoread: super::DWORD, pdwnumberofbytesread: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDInboundAttachment_Impl::Read(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwbytestoread), core::mem::transmute_copy(&pdwnumberofbytesread)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IWSDInboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDInboundAttachment_Impl::Close(this).into()
            }
        }
        Self { base__: IWSDAttachment_Vtbl::new::<Identity, OFFSET>(), Read: Read::<Identity, OFFSET>, Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDInboundAttachment as windows_core::Interface>::IID || iid == &<IWSDAttachment as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IWSDInboundAttachment {}
windows_core::imp::define_interface!(IWSDMessageParameters, IWSDMessageParameters_Vtbl, 0x1fafe8a2_e6fc_4b80_b6cf_b7d45c416d7c);
windows_core::imp::interface_hierarchy!(IWSDMessageParameters, windows_core::IUnknown);
impl IWSDMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> windows_core::Result<IWSDAddress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetLocalAddress<P0>(&self, paddress: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDAddress>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLocalAddress)(windows_core::Interface::as_raw(self), paddress.param().abi()) }
    }
    pub unsafe fn GetRemoteAddress(&self) -> windows_core::Result<IWSDAddress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRemoteAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetRemoteAddress<P0>(&self, paddress: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDAddress>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteAddress)(windows_core::Interface::as_raw(self), paddress.param().abi()) }
    }
    pub unsafe fn GetLowerParameters(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLowerParameters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDMessageParameters_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLocalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRemoteAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLowerParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWSDMessageParameters_Impl: windows_core::IUnknownImpl {
    fn GetLocalAddress(&self) -> windows_core::Result<IWSDAddress>;
    fn SetLocalAddress(&self, paddress: windows_core::Ref<IWSDAddress>) -> windows_core::Result<()>;
    fn GetRemoteAddress(&self) -> windows_core::Result<IWSDAddress>;
    fn SetRemoteAddress(&self, paddress: windows_core::Ref<IWSDAddress>) -> windows_core::Result<()>;
    fn GetLowerParameters(&self) -> windows_core::Result<IWSDMessageParameters>;
}
impl IWSDMessageParameters_Vtbl {
    pub const fn new<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLocalAddress<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDMessageParameters_Impl::GetLocalAddress(this) {
                    Ok(ok__) => {
                        ppaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalAddress<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDMessageParameters_Impl::SetLocalAddress(this, core::mem::transmute_copy(&paddress)).into()
            }
        }
        unsafe extern "system" fn GetRemoteAddress<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDMessageParameters_Impl::GetRemoteAddress(this) {
                    Ok(ok__) => {
                        ppaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteAddress<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDMessageParameters_Impl::SetRemoteAddress(this, core::mem::transmute_copy(&paddress)).into()
            }
        }
        unsafe extern "system" fn GetLowerParameters<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptxparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDMessageParameters_Impl::GetLowerParameters(this) {
                    Ok(ok__) => {
                        pptxparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLocalAddress: GetLocalAddress::<Identity, OFFSET>,
            SetLocalAddress: SetLocalAddress::<Identity, OFFSET>,
            GetRemoteAddress: GetRemoteAddress::<Identity, OFFSET>,
            SetRemoteAddress: SetRemoteAddress::<Identity, OFFSET>,
            GetLowerParameters: GetLowerParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDMessageParameters as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDMessageParameters {}
windows_core::imp::define_interface!(IWSDMetadataExchange, IWSDMetadataExchange_Vtbl, 0x06996d57_1d67_4928_9307_3d7833fdb846);
windows_core::imp::interface_hierarchy!(IWSDMetadataExchange, windows_core::IUnknown);
impl IWSDMetadataExchange {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetMetadata(&self) -> windows_core::Result<*mut WSD_METADATA_SECTION_LIST> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadata)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDMetadataExchange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetMetadata: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWSDMetadataExchange_Impl: windows_core::IUnknownImpl {
    fn GetMetadata(&self) -> windows_core::Result<*mut WSD_METADATA_SECTION_LIST>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWSDMetadataExchange_Vtbl {
    pub const fn new<Identity: IWSDMetadataExchange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMetadata<Identity: IWSDMetadataExchange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDMetadataExchange_Impl::GetMetadata(this) {
                    Ok(ok__) => {
                        metadataout.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMetadata: GetMetadata::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDMetadataExchange as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWSDMetadataExchange {}
windows_core::imp::define_interface!(IWSDOutboundAttachment, IWSDOutboundAttachment_Vtbl, 0xaa302f8d_5a22_4ba5_b392_aa8486f4c15d);
impl core::ops::Deref for IWSDOutboundAttachment {
    type Target = IWSDAttachment;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDOutboundAttachment, windows_core::IUnknown, IWSDAttachment);
impl IWSDOutboundAttachment {
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn Write(&self, pbuffer: &[super::BYTE]) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pbuffer.as_ptr(), pbuffer.len().try_into().unwrap(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Abort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDOutboundAttachment_Vtbl {
    pub base__: IWSDAttachment_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::DWORD, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    Write: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IWSDOutboundAttachment_Impl: IWSDAttachment_Impl {
    fn Write(&self, pbuffer: *const super::BYTE, dwbytestowrite: super::DWORD) -> windows_core::Result<u32>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IWSDOutboundAttachment_Vtbl {
    pub const fn new<Identity: IWSDOutboundAttachment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Write<Identity: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *const super::BYTE, dwbytestowrite: super::DWORD, pdwnumberofbyteswritten: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDOutboundAttachment_Impl::Write(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwbytestowrite)) {
                    Ok(ok__) => {
                        pdwnumberofbyteswritten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDOutboundAttachment_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDOutboundAttachment_Impl::Abort(this).into()
            }
        }
        Self {
            base__: IWSDAttachment_Vtbl::new::<Identity, OFFSET>(),
            Write: Write::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDOutboundAttachment as windows_core::Interface>::IID || iid == &<IWSDAttachment as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IWSDOutboundAttachment {}
windows_core::imp::define_interface!(IWSDSSLClientCertificate, IWSDSSLClientCertificate_Vtbl, 0xde105e87_a0da_418e_98ad_27b9eed87bdc);
windows_core::imp::interface_hierarchy!(IWSDSSLClientCertificate, windows_core::IUnknown);
impl IWSDSSLClientCertificate {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
    pub unsafe fn GetClientCertificate(&self, ppcertcontext: *mut super::PCCERT_CONTEXT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClientCertificate)(windows_core::Interface::as_raw(self), ppcertcontext as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetMappedAccessToken(&self, phtoken: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMappedAccessToken)(windows_core::Interface::as_raw(self), phtoken as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSSLClientCertificate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
    pub GetClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::PCCERT_CONTEXT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "wincrypt")))]
    GetClientCertificate: usize,
    #[cfg(feature = "winnt")]
    pub GetMappedAccessToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetMappedAccessToken: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub trait IWSDSSLClientCertificate_Impl: windows_core::IUnknownImpl {
    fn GetClientCertificate(&self, ppcertcontext: *mut super::PCCERT_CONTEXT) -> windows_core::Result<()>;
    fn GetMappedAccessToken(&self, phtoken: *mut super::HANDLE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt", feature = "winnt"))]
impl IWSDSSLClientCertificate_Vtbl {
    pub const fn new<Identity: IWSDSSLClientCertificate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClientCertificate<Identity: IWSDSSLClientCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcertcontext: *mut super::PCCERT_CONTEXT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDSSLClientCertificate_Impl::GetClientCertificate(this, core::mem::transmute_copy(&ppcertcontext)).into()
            }
        }
        unsafe extern "system" fn GetMappedAccessToken<Identity: IWSDSSLClientCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phtoken: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDSSLClientCertificate_Impl::GetMappedAccessToken(this, core::mem::transmute_copy(&phtoken)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClientCertificate: GetClientCertificate::<Identity, OFFSET>,
            GetMappedAccessToken: GetMappedAccessToken::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDSSLClientCertificate as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt", feature = "winnt"))]
impl windows_core::RuntimeName for IWSDSSLClientCertificate {}
windows_core::imp::define_interface!(IWSDScopeMatchingRule, IWSDScopeMatchingRule_Vtbl, 0xfcafe424_fef5_481a_bd9f_33ce0574256f);
windows_core::imp::interface_hierarchy!(IWSDScopeMatchingRule, windows_core::IUnknown);
impl IWSDScopeMatchingRule {
    pub unsafe fn GetScopeRule(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScopeRule)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MatchScopes<P0, P1>(&self, pszscope1: P0, pszscope2: P1) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MatchScopes)(windows_core::Interface::as_raw(self), pszscope1.param().abi(), pszscope2.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDScopeMatchingRule_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetScopeRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub MatchScopes: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IWSDScopeMatchingRule_Impl: windows_core::IUnknownImpl {
    fn GetScopeRule(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn MatchScopes(&self, pszscope1: &windows_core::PCWSTR, pszscope2: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
}
impl IWSDScopeMatchingRule_Vtbl {
    pub const fn new<Identity: IWSDScopeMatchingRule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetScopeRule<Identity: IWSDScopeMatchingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszscopematchingrule: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDScopeMatchingRule_Impl::GetScopeRule(this) {
                    Ok(ok__) => {
                        ppszscopematchingrule.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MatchScopes<Identity: IWSDScopeMatchingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszscope1: windows_core::PCWSTR, pszscope2: windows_core::PCWSTR, pfmatch: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDScopeMatchingRule_Impl::MatchScopes(this, core::mem::transmute(&pszscope1), core::mem::transmute(&pszscope2)) {
                    Ok(ok__) => {
                        pfmatch.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetScopeRule: GetScopeRule::<Identity, OFFSET>,
            MatchScopes: MatchScopes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDScopeMatchingRule as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDScopeMatchingRule {}
windows_core::imp::define_interface!(IWSDServiceMessaging, IWSDServiceMessaging_Vtbl, 0x94974cf4_0cab_460d_a3f6_7a0ad623c0e6);
windows_core::imp::interface_hierarchy!(IWSDServiceMessaging, windows_core::IUnknown);
impl IWSDServiceMessaging {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SendResponse<P2>(&self, pbody: Option<*const core::ffi::c_void>, poperation: *const WSD_OPERATION, pmessageparameters: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWSDMessageParameters>,
    {
        unsafe { (windows_core::Interface::vtable(self).SendResponse)(windows_core::Interface::as_raw(self), pbody.unwrap_or(core::mem::zeroed()) as _, poperation, pmessageparameters.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn FaultRequest<P1>(&self, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: P1, pfault: Option<*const WSD_SOAP_FAULT>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWSDMessageParameters>,
    {
        unsafe { (windows_core::Interface::vtable(self).FaultRequest)(windows_core::Interface::as_raw(self), prequestheader, pmessageparameters.param().abi(), pfault.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceMessaging_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SendResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const WSD_OPERATION, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SendResponse: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub FaultRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_SOAP_HEADER, *mut core::ffi::c_void, *const WSD_SOAP_FAULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    FaultRequest: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWSDServiceMessaging_Impl: windows_core::IUnknownImpl {
    fn SendResponse(&self, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: windows_core::Ref<IWSDMessageParameters>) -> windows_core::Result<()>;
    fn FaultRequest(&self, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: windows_core::Ref<IWSDMessageParameters>, pfault: *const WSD_SOAP_FAULT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWSDServiceMessaging_Vtbl {
    pub const fn new<Identity: IWSDServiceMessaging_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendResponse<Identity: IWSDServiceMessaging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceMessaging_Impl::SendResponse(this, core::mem::transmute_copy(&pbody), core::mem::transmute_copy(&poperation), core::mem::transmute_copy(&pmessageparameters)).into()
            }
        }
        unsafe extern "system" fn FaultRequest<Identity: IWSDServiceMessaging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: *mut core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceMessaging_Impl::FaultRequest(this, core::mem::transmute_copy(&prequestheader), core::mem::transmute_copy(&pmessageparameters), core::mem::transmute_copy(&pfault)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendResponse: SendResponse::<Identity, OFFSET>,
            FaultRequest: FaultRequest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDServiceMessaging as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWSDServiceMessaging {}
windows_core::imp::define_interface!(IWSDServiceProxy, IWSDServiceProxy_Vtbl, 0xd4c7fb9c_03ab_4175_9d67_094fafebf487);
impl core::ops::Deref for IWSDServiceProxy {
    type Target = IWSDMetadataExchange;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDServiceProxy, windows_core::IUnknown, IWSDMetadataExchange);
impl IWSDServiceProxy {
    pub unsafe fn BeginGetMetadata(&self) -> windows_core::Result<IWSDAsyncResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginGetMetadata)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn EndGetMetadata<P0>(&self, presult: P0) -> windows_core::Result<*mut WSD_METADATA_SECTION_LIST>
    where
        P0: windows_core::Param<IWSDAsyncResult>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndGetMetadata)(windows_core::Interface::as_raw(self), presult.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetServiceMetadata(&self) -> windows_core::Result<*mut WSD_SERVICE_METADATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetServiceMetadata)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SubscribeToOperation<P1>(&self, poperation: *const WSD_OPERATION, punknown: P1, pany: *const WSDXML_ELEMENT, ppany: Option<*mut *mut WSDXML_ELEMENT>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SubscribeToOperation)(windows_core::Interface::as_raw(self), poperation, punknown.param().abi(), pany, ppany.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn UnsubscribeToOperation(&self, poperation: *const WSD_OPERATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnsubscribeToOperation)(windows_core::Interface::as_raw(self), poperation) }
    }
    pub unsafe fn SetEventingStatusCallback<P0>(&self, pstatus: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDEventingStatus>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEventingStatusCallback)(windows_core::Interface::as_raw(self), pstatus.param().abi()) }
    }
    pub unsafe fn GetEndpointProxy(&self) -> windows_core::Result<IWSDEndpointProxy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEndpointProxy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceProxy_Vtbl {
    pub base__: IWSDMetadataExchange_Vtbl,
    pub BeginGetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub EndGetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    EndGetMetadata: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetServiceMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WSD_SERVICE_METADATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetServiceMetadata: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SubscribeToOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_OPERATION, *mut core::ffi::c_void, *const WSDXML_ELEMENT, *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SubscribeToOperation: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub UnsubscribeToOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_OPERATION) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    UnsubscribeToOperation: usize,
    pub SetEventingStatusCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWSDServiceProxy_Impl: IWSDMetadataExchange_Impl {
    fn BeginGetMetadata(&self) -> windows_core::Result<IWSDAsyncResult>;
    fn EndGetMetadata(&self, presult: windows_core::Ref<IWSDAsyncResult>) -> windows_core::Result<*mut WSD_METADATA_SECTION_LIST>;
    fn GetServiceMetadata(&self) -> windows_core::Result<*mut WSD_SERVICE_METADATA>;
    fn SubscribeToOperation(&self, poperation: *const WSD_OPERATION, punknown: windows_core::Ref<windows_core::IUnknown>, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn UnsubscribeToOperation(&self, poperation: *const WSD_OPERATION) -> windows_core::Result<()>;
    fn SetEventingStatusCallback(&self, pstatus: windows_core::Ref<IWSDEventingStatus>) -> windows_core::Result<()>;
    fn GetEndpointProxy(&self) -> windows_core::Result<IWSDEndpointProxy>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWSDServiceProxy_Vtbl {
    pub const fn new<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginGetMetadata<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxy_Impl::BeginGetMetadata(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndGetMetadata<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxy_Impl::EndGetMetadata(this, core::mem::transmute_copy(&presult)) {
                    Ok(ok__) => {
                        ppmetadata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetServiceMetadata<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppservicemetadata: *mut *mut WSD_SERVICE_METADATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxy_Impl::GetServiceMetadata(this) {
                    Ok(ok__) => {
                        ppservicemetadata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SubscribeToOperation<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperation: *const WSD_OPERATION, punknown: *mut core::ffi::c_void, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxy_Impl::SubscribeToOperation(this, core::mem::transmute_copy(&poperation), core::mem::transmute_copy(&punknown), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&ppany)).into()
            }
        }
        unsafe extern "system" fn UnsubscribeToOperation<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperation: *const WSD_OPERATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxy_Impl::UnsubscribeToOperation(this, core::mem::transmute_copy(&poperation)).into()
            }
        }
        unsafe extern "system" fn SetEventingStatusCallback<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxy_Impl::SetEventingStatusCallback(this, core::mem::transmute_copy(&pstatus)).into()
            }
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproxy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxy_Impl::GetEndpointProxy(this) {
                    Ok(ok__) => {
                        ppproxy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWSDMetadataExchange_Vtbl::new::<Identity, OFFSET>(),
            BeginGetMetadata: BeginGetMetadata::<Identity, OFFSET>,
            EndGetMetadata: EndGetMetadata::<Identity, OFFSET>,
            GetServiceMetadata: GetServiceMetadata::<Identity, OFFSET>,
            SubscribeToOperation: SubscribeToOperation::<Identity, OFFSET>,
            UnsubscribeToOperation: UnsubscribeToOperation::<Identity, OFFSET>,
            SetEventingStatusCallback: SetEventingStatusCallback::<Identity, OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDServiceProxy as windows_core::Interface>::IID || iid == &<IWSDMetadataExchange as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWSDServiceProxy {}
windows_core::imp::define_interface!(IWSDServiceProxyEventing, IWSDServiceProxyEventing_Vtbl, 0xf9279d6d_1012_4a94_b8cc_fd35d2202bfe);
impl core::ops::Deref for IWSDServiceProxyEventing {
    type Target = IWSDServiceProxy;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDServiceProxyEventing, windows_core::IUnknown, IWSDMetadataExchange, IWSDServiceProxy);
impl IWSDServiceProxyEventing {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SubscribeToMultipleOperations<P2>(&self, poperations: &[WSD_OPERATION], punknown: P2, pexpires: Option<*const WSD_EVENTING_EXPIRES>, pany: Option<*const WSDXML_ELEMENT>, ppexpires: Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: Option<*mut *mut WSDXML_ELEMENT>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SubscribeToMultipleOperations)(windows_core::Interface::as_raw(self), poperations.as_ptr(), poperations.len().try_into().unwrap(), punknown.param().abi(), pexpires.unwrap_or(core::mem::zeroed()) as _, pany.unwrap_or(core::mem::zeroed()) as _, ppexpires.unwrap_or(core::mem::zeroed()) as _, ppany.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn BeginSubscribeToMultipleOperations<P2, P5, P6>(&self, poperations: &[WSD_OPERATION], punknown: P2, pexpires: Option<*const WSD_EVENTING_EXPIRES>, pany: Option<*const WSDXML_ELEMENT>, pasyncstate: P5, pasynccallback: P6) -> windows_core::Result<IWSDAsyncResult>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
        P5: windows_core::Param<windows_core::IUnknown>,
        P6: windows_core::Param<IWSDAsyncCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginSubscribeToMultipleOperations)(windows_core::Interface::as_raw(self), poperations.as_ptr(), poperations.len().try_into().unwrap(), punknown.param().abi(), pexpires.unwrap_or(core::mem::zeroed()) as _, pany.unwrap_or(core::mem::zeroed()) as _, pasyncstate.param().abi(), pasynccallback.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn EndSubscribeToMultipleOperations<P2>(&self, poperations: &[WSD_OPERATION], presult: P2, ppexpires: Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: Option<*mut *mut WSDXML_ELEMENT>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWSDAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndSubscribeToMultipleOperations)(windows_core::Interface::as_raw(self), poperations.as_ptr(), poperations.len().try_into().unwrap(), presult.param().abi(), ppexpires.unwrap_or(core::mem::zeroed()) as _, ppany.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn UnsubscribeToMultipleOperations(&self, poperations: &[WSD_OPERATION], pany: *const WSDXML_ELEMENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnsubscribeToMultipleOperations)(windows_core::Interface::as_raw(self), poperations.as_ptr(), poperations.len().try_into().unwrap(), pany) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn BeginUnsubscribeToMultipleOperations<P3, P4>(&self, poperations: &[WSD_OPERATION], pany: Option<*const WSDXML_ELEMENT>, pasyncstate: P3, pasynccallback: P4) -> windows_core::Result<IWSDAsyncResult>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
        P4: windows_core::Param<IWSDAsyncCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginUnsubscribeToMultipleOperations)(windows_core::Interface::as_raw(self), poperations.as_ptr(), poperations.len().try_into().unwrap(), pany.unwrap_or(core::mem::zeroed()) as _, pasyncstate.param().abi(), pasynccallback.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn EndUnsubscribeToMultipleOperations<P2>(&self, poperations: &[WSD_OPERATION], presult: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWSDAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndUnsubscribeToMultipleOperations)(windows_core::Interface::as_raw(self), poperations.as_ptr(), poperations.len().try_into().unwrap(), presult.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn RenewMultipleOperations(&self, poperations: &[WSD_OPERATION], pexpires: Option<*const WSD_EVENTING_EXPIRES>, pany: Option<*const WSDXML_ELEMENT>, ppexpires: Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: Option<*mut *mut WSDXML_ELEMENT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RenewMultipleOperations)(windows_core::Interface::as_raw(self), poperations.as_ptr(), poperations.len().try_into().unwrap(), pexpires.unwrap_or(core::mem::zeroed()) as _, pany.unwrap_or(core::mem::zeroed()) as _, ppexpires.unwrap_or(core::mem::zeroed()) as _, ppany.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn BeginRenewMultipleOperations<P4, P5>(&self, poperations: &[WSD_OPERATION], pexpires: Option<*const WSD_EVENTING_EXPIRES>, pany: Option<*const WSDXML_ELEMENT>, pasyncstate: P4, pasynccallback: P5) -> windows_core::Result<IWSDAsyncResult>
    where
        P4: windows_core::Param<windows_core::IUnknown>,
        P5: windows_core::Param<IWSDAsyncCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginRenewMultipleOperations)(windows_core::Interface::as_raw(self), poperations.as_ptr(), poperations.len().try_into().unwrap(), pexpires.unwrap_or(core::mem::zeroed()) as _, pany.unwrap_or(core::mem::zeroed()) as _, pasyncstate.param().abi(), pasynccallback.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn EndRenewMultipleOperations<P2>(&self, poperations: &[WSD_OPERATION], presult: P2, ppexpires: Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: Option<*mut *mut WSDXML_ELEMENT>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWSDAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndRenewMultipleOperations)(windows_core::Interface::as_raw(self), poperations.as_ptr(), poperations.len().try_into().unwrap(), presult.param().abi(), ppexpires.unwrap_or(core::mem::zeroed()) as _, ppany.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetStatusForMultipleOperations(&self, poperations: &[WSD_OPERATION], pany: Option<*const WSDXML_ELEMENT>, ppexpires: Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: Option<*mut *mut WSDXML_ELEMENT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatusForMultipleOperations)(windows_core::Interface::as_raw(self), poperations.as_ptr(), poperations.len().try_into().unwrap(), pany.unwrap_or(core::mem::zeroed()) as _, ppexpires.unwrap_or(core::mem::zeroed()) as _, ppany.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn BeginGetStatusForMultipleOperations<P3, P4>(&self, poperations: &[WSD_OPERATION], pany: Option<*const WSDXML_ELEMENT>, pasyncstate: P3, pasynccallback: P4) -> windows_core::Result<IWSDAsyncResult>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
        P4: windows_core::Param<IWSDAsyncCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginGetStatusForMultipleOperations)(windows_core::Interface::as_raw(self), poperations.as_ptr(), poperations.len().try_into().unwrap(), pany.unwrap_or(core::mem::zeroed()) as _, pasyncstate.param().abi(), pasynccallback.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn EndGetStatusForMultipleOperations<P2>(&self, poperations: &[WSD_OPERATION], presult: P2, ppexpires: Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: Option<*mut *mut WSDXML_ELEMENT>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWSDAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndGetStatusForMultipleOperations)(windows_core::Interface::as_raw(self), poperations.as_ptr(), poperations.len().try_into().unwrap(), presult.param().abi(), ppexpires.unwrap_or(core::mem::zeroed()) as _, ppany.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceProxyEventing_Vtbl {
    pub base__: IWSDServiceProxy_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SubscribeToMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_OPERATION, super::DWORD, *mut core::ffi::c_void, *const WSD_EVENTING_EXPIRES, *const WSDXML_ELEMENT, *mut *mut WSD_EVENTING_EXPIRES, *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SubscribeToMultipleOperations: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub BeginSubscribeToMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_OPERATION, super::DWORD, *mut core::ffi::c_void, *const WSD_EVENTING_EXPIRES, *const WSDXML_ELEMENT, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    BeginSubscribeToMultipleOperations: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub EndSubscribeToMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_OPERATION, super::DWORD, *mut core::ffi::c_void, *mut *mut WSD_EVENTING_EXPIRES, *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    EndSubscribeToMultipleOperations: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub UnsubscribeToMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_OPERATION, super::DWORD, *const WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    UnsubscribeToMultipleOperations: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub BeginUnsubscribeToMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_OPERATION, super::DWORD, *const WSDXML_ELEMENT, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    BeginUnsubscribeToMultipleOperations: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub EndUnsubscribeToMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_OPERATION, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    EndUnsubscribeToMultipleOperations: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub RenewMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_OPERATION, super::DWORD, *const WSD_EVENTING_EXPIRES, *const WSDXML_ELEMENT, *mut *mut WSD_EVENTING_EXPIRES, *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    RenewMultipleOperations: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub BeginRenewMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_OPERATION, super::DWORD, *const WSD_EVENTING_EXPIRES, *const WSDXML_ELEMENT, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    BeginRenewMultipleOperations: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub EndRenewMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_OPERATION, super::DWORD, *mut core::ffi::c_void, *mut *mut WSD_EVENTING_EXPIRES, *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    EndRenewMultipleOperations: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetStatusForMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_OPERATION, super::DWORD, *const WSDXML_ELEMENT, *mut *mut WSD_EVENTING_EXPIRES, *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetStatusForMultipleOperations: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub BeginGetStatusForMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_OPERATION, super::DWORD, *const WSDXML_ELEMENT, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    BeginGetStatusForMultipleOperations: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub EndGetStatusForMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_OPERATION, super::DWORD, *mut core::ffi::c_void, *mut *mut WSD_EVENTING_EXPIRES, *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    EndGetStatusForMultipleOperations: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWSDServiceProxyEventing_Impl: IWSDServiceProxy_Impl {
    fn SubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, punknown: windows_core::Ref<windows_core::IUnknown>, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn BeginSubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, punknown: windows_core::Ref<windows_core::IUnknown>, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: windows_core::Ref<windows_core::IUnknown>, pasynccallback: windows_core::Ref<IWSDAsyncCallback>) -> windows_core::Result<IWSDAsyncResult>;
    fn EndSubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, presult: windows_core::Ref<IWSDAsyncResult>, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn UnsubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, pany: *const WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn BeginUnsubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, pany: *const WSDXML_ELEMENT, pasyncstate: windows_core::Ref<windows_core::IUnknown>, pasynccallback: windows_core::Ref<IWSDAsyncCallback>) -> windows_core::Result<IWSDAsyncResult>;
    fn EndUnsubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, presult: windows_core::Ref<IWSDAsyncResult>) -> windows_core::Result<()>;
    fn RenewMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn BeginRenewMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: windows_core::Ref<windows_core::IUnknown>, pasynccallback: windows_core::Ref<IWSDAsyncCallback>) -> windows_core::Result<IWSDAsyncResult>;
    fn EndRenewMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, presult: windows_core::Ref<IWSDAsyncResult>, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn GetStatusForMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn BeginGetStatusForMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, pany: *const WSDXML_ELEMENT, pasyncstate: windows_core::Ref<windows_core::IUnknown>, pasynccallback: windows_core::Ref<IWSDAsyncCallback>) -> windows_core::Result<IWSDAsyncResult>;
    fn EndGetStatusForMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, presult: windows_core::Ref<IWSDAsyncResult>, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWSDServiceProxyEventing_Vtbl {
    pub const fn new<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, punknown: *mut core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxyEventing_Impl::SubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&punknown), core::mem::transmute_copy(&pexpires), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
            }
        }
        unsafe extern "system" fn BeginSubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, punknown: *mut core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut core::ffi::c_void, pasynccallback: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxyEventing_Impl::BeginSubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&punknown), core::mem::transmute_copy(&pexpires), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&pasyncstate), core::mem::transmute_copy(&pasynccallback)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndSubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, presult: *mut core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxyEventing_Impl::EndSubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&presult), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
            }
        }
        unsafe extern "system" fn UnsubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, pany: *const WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxyEventing_Impl::UnsubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pany)).into()
            }
        }
        unsafe extern "system" fn BeginUnsubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, pany: *const WSDXML_ELEMENT, pasyncstate: *mut core::ffi::c_void, pasynccallback: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxyEventing_Impl::BeginUnsubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&pasyncstate), core::mem::transmute_copy(&pasynccallback)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndUnsubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, presult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxyEventing_Impl::EndUnsubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&presult)).into()
            }
        }
        unsafe extern "system" fn RenewMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxyEventing_Impl::RenewMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pexpires), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
            }
        }
        unsafe extern "system" fn BeginRenewMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut core::ffi::c_void, pasynccallback: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxyEventing_Impl::BeginRenewMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pexpires), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&pasyncstate), core::mem::transmute_copy(&pasynccallback)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndRenewMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, presult: *mut core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxyEventing_Impl::EndRenewMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&presult), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
            }
        }
        unsafe extern "system" fn GetStatusForMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxyEventing_Impl::GetStatusForMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
            }
        }
        unsafe extern "system" fn BeginGetStatusForMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, pany: *const WSDXML_ELEMENT, pasyncstate: *mut core::ffi::c_void, pasynccallback: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxyEventing_Impl::BeginGetStatusForMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&pasyncstate), core::mem::transmute_copy(&pasynccallback)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndGetStatusForMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: super::DWORD, presult: *mut core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxyEventing_Impl::EndGetStatusForMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&presult), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
            }
        }
        Self {
            base__: IWSDServiceProxy_Vtbl::new::<Identity, OFFSET>(),
            SubscribeToMultipleOperations: SubscribeToMultipleOperations::<Identity, OFFSET>,
            BeginSubscribeToMultipleOperations: BeginSubscribeToMultipleOperations::<Identity, OFFSET>,
            EndSubscribeToMultipleOperations: EndSubscribeToMultipleOperations::<Identity, OFFSET>,
            UnsubscribeToMultipleOperations: UnsubscribeToMultipleOperations::<Identity, OFFSET>,
            BeginUnsubscribeToMultipleOperations: BeginUnsubscribeToMultipleOperations::<Identity, OFFSET>,
            EndUnsubscribeToMultipleOperations: EndUnsubscribeToMultipleOperations::<Identity, OFFSET>,
            RenewMultipleOperations: RenewMultipleOperations::<Identity, OFFSET>,
            BeginRenewMultipleOperations: BeginRenewMultipleOperations::<Identity, OFFSET>,
            EndRenewMultipleOperations: EndRenewMultipleOperations::<Identity, OFFSET>,
            GetStatusForMultipleOperations: GetStatusForMultipleOperations::<Identity, OFFSET>,
            BeginGetStatusForMultipleOperations: BeginGetStatusForMultipleOperations::<Identity, OFFSET>,
            EndGetStatusForMultipleOperations: EndGetStatusForMultipleOperations::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDServiceProxyEventing as windows_core::Interface>::IID || iid == &<IWSDMetadataExchange as windows_core::Interface>::IID || iid == &<IWSDServiceProxy as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWSDServiceProxyEventing {}
windows_core::imp::define_interface!(IWSDSignatureProperty, IWSDSignatureProperty_Vtbl, 0x03ce20aa_71c4_45e2_b32e_3766c61c790f);
windows_core::imp::interface_hierarchy!(IWSDSignatureProperty, windows_core::IUnknown);
impl IWSDSignatureProperty {
    pub unsafe fn IsMessageSigned(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMessageSigned)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsMessageSignatureTrusted(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMessageSignatureTrusted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetKeyInfo(&self, pbkeyinfo: Option<*mut super::BYTE>, pdwkeyinfosize: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetKeyInfo)(windows_core::Interface::as_raw(self), pbkeyinfo.unwrap_or(core::mem::zeroed()) as _, pdwkeyinfosize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSignature(&self, pbsignature: Option<*mut super::BYTE>, pdwsignaturesize: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSignature)(windows_core::Interface::as_raw(self), pbsignature.unwrap_or(core::mem::zeroed()) as _, pdwsignaturesize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSignedInfoHash(&self, pbsignedinfohash: Option<*mut super::BYTE>, pdwhashsize: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSignedInfoHash)(windows_core::Interface::as_raw(self), pbsignedinfohash.unwrap_or(core::mem::zeroed()) as _, pdwhashsize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSignatureProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsMessageSigned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsMessageSignatureTrusted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetKeyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetKeyInfo: usize,
    #[cfg(feature = "shared")]
    pub GetSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSignature: usize,
    #[cfg(feature = "shared")]
    pub GetSignedInfoHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSignedInfoHash: usize,
}
#[cfg(feature = "shared")]
pub trait IWSDSignatureProperty_Impl: windows_core::IUnknownImpl {
    fn IsMessageSigned(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsMessageSignatureTrusted(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetKeyInfo(&self, pbkeyinfo: *mut super::BYTE, pdwkeyinfosize: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetSignature(&self, pbsignature: *mut super::BYTE, pdwsignaturesize: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetSignedInfoHash(&self, pbsignedinfohash: *mut super::BYTE, pdwhashsize: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWSDSignatureProperty_Vtbl {
    pub const fn new<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsMessageSigned<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsigned: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDSignatureProperty_Impl::IsMessageSigned(this) {
                    Ok(ok__) => {
                        pbsigned.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsMessageSignatureTrusted<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsignaturetrusted: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDSignatureProperty_Impl::IsMessageSignatureTrusted(this) {
                    Ok(ok__) => {
                        pbsignaturetrusted.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetKeyInfo<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbkeyinfo: *mut super::BYTE, pdwkeyinfosize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDSignatureProperty_Impl::GetKeyInfo(this, core::mem::transmute_copy(&pbkeyinfo), core::mem::transmute_copy(&pdwkeyinfosize)).into()
            }
        }
        unsafe extern "system" fn GetSignature<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsignature: *mut super::BYTE, pdwsignaturesize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDSignatureProperty_Impl::GetSignature(this, core::mem::transmute_copy(&pbsignature), core::mem::transmute_copy(&pdwsignaturesize)).into()
            }
        }
        unsafe extern "system" fn GetSignedInfoHash<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsignedinfohash: *mut super::BYTE, pdwhashsize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDSignatureProperty_Impl::GetSignedInfoHash(this, core::mem::transmute_copy(&pbsignedinfohash), core::mem::transmute_copy(&pdwhashsize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsMessageSigned: IsMessageSigned::<Identity, OFFSET>,
            IsMessageSignatureTrusted: IsMessageSignatureTrusted::<Identity, OFFSET>,
            GetKeyInfo: GetKeyInfo::<Identity, OFFSET>,
            GetSignature: GetSignature::<Identity, OFFSET>,
            GetSignedInfoHash: GetSignedInfoHash::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDSignatureProperty as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWSDSignatureProperty {}
windows_core::imp::define_interface!(IWSDTransportAddress, IWSDTransportAddress_Vtbl, 0x70d23498_4ee6_4340_a3df_d845d2235467);
impl core::ops::Deref for IWSDTransportAddress {
    type Target = IWSDAddress;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDTransportAddress, windows_core::IUnknown, IWSDAddress);
impl IWSDTransportAddress {
    #[cfg(feature = "shared")]
    pub unsafe fn GetPort(&self) -> windows_core::Result<super::WORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetPort(&self, wport: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPort)(windows_core::Interface::as_raw(self), wport) }
    }
    pub unsafe fn GetTransportAddress(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransportAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTransportAddressEx(&self, fsafe: bool) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransportAddressEx)(windows_core::Interface::as_raw(self), fsafe.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTransportAddress<P0>(&self, pszaddress: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTransportAddress)(windows_core::Interface::as_raw(self), pszaddress.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDTransportAddress_Vtbl {
    pub base__: IWSDAddress_Vtbl,
    #[cfg(feature = "shared")]
    pub GetPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPort: usize,
    #[cfg(feature = "shared")]
    pub SetPort: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetPort: usize,
    pub GetTransportAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetTransportAddressEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetTransportAddress: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWSDTransportAddress_Impl: IWSDAddress_Impl {
    fn GetPort(&self) -> windows_core::Result<super::WORD>;
    fn SetPort(&self, wport: super::WORD) -> windows_core::Result<()>;
    fn GetTransportAddress(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetTransportAddressEx(&self, fsafe: windows_core::BOOL) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetTransportAddress(&self, pszaddress: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWSDTransportAddress_Vtbl {
    pub const fn new<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPort<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwport: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDTransportAddress_Impl::GetPort(this) {
                    Ok(ok__) => {
                        pwport.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPort<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wport: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDTransportAddress_Impl::SetPort(this, core::mem::transmute_copy(&wport)).into()
            }
        }
        unsafe extern "system" fn GetTransportAddress<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszaddress: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDTransportAddress_Impl::GetTransportAddress(this) {
                    Ok(ok__) => {
                        ppszaddress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransportAddressEx<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsafe: windows_core::BOOL, ppszaddress: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDTransportAddress_Impl::GetTransportAddressEx(this, core::mem::transmute_copy(&fsafe)) {
                    Ok(ok__) => {
                        ppszaddress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTransportAddress<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszaddress: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDTransportAddress_Impl::SetTransportAddress(this, core::mem::transmute(&pszaddress)).into()
            }
        }
        Self {
            base__: IWSDAddress_Vtbl::new::<Identity, OFFSET>(),
            GetPort: GetPort::<Identity, OFFSET>,
            SetPort: SetPort::<Identity, OFFSET>,
            GetTransportAddress: GetTransportAddress::<Identity, OFFSET>,
            GetTransportAddressEx: GetTransportAddressEx::<Identity, OFFSET>,
            SetTransportAddress: SetTransportAddress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDTransportAddress as windows_core::Interface>::IID || iid == &<IWSDAddress as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWSDTransportAddress {}
windows_core::imp::define_interface!(IWSDUdpAddress, IWSDUdpAddress_Vtbl, 0x74d6124a_a441_4f78_a1eb_97a8d1996893);
impl core::ops::Deref for IWSDUdpAddress {
    type Target = IWSDTransportAddress;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDUdpAddress, windows_core::IUnknown, IWSDAddress, IWSDTransportAddress);
impl IWSDUdpAddress {
    #[cfg(all(feature = "shared", feature = "ws2"))]
    pub unsafe fn SetSockaddr(&self, psockaddr: *const super::SOCKADDR_STORAGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSockaddr)(windows_core::Interface::as_raw(self), psockaddr) }
    }
    #[cfg(all(feature = "shared", feature = "ws2"))]
    pub unsafe fn GetSockaddr(&self, psockaddr: *mut super::SOCKADDR_STORAGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSockaddr)(windows_core::Interface::as_raw(self), psockaddr as _) }
    }
    pub unsafe fn SetExclusive(&self, fexclusive: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExclusive)(windows_core::Interface::as_raw(self), fexclusive.into()) }
    }
    pub unsafe fn GetExclusive(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExclusive)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetMessageType(&self, messagetype: WSDUdpMessageType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMessageType)(windows_core::Interface::as_raw(self), messagetype) }
    }
    pub unsafe fn GetMessageType(&self) -> windows_core::Result<WSDUdpMessageType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMessageType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetTTL(&self, dwttl: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTTL)(windows_core::Interface::as_raw(self), dwttl) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetTTL(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTTL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAlias(&self, palias: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAlias)(windows_core::Interface::as_raw(self), palias) }
    }
    pub unsafe fn GetAlias(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAlias)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpAddress_Vtbl {
    pub base__: IWSDTransportAddress_Vtbl,
    #[cfg(all(feature = "shared", feature = "ws2"))]
    pub SetSockaddr: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SOCKADDR_STORAGE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "ws2")))]
    SetSockaddr: usize,
    #[cfg(all(feature = "shared", feature = "ws2"))]
    pub GetSockaddr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SOCKADDR_STORAGE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "ws2")))]
    GetSockaddr: usize,
    pub SetExclusive: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetExclusive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMessageType: unsafe extern "system" fn(*mut core::ffi::c_void, WSDUdpMessageType) -> windows_core::HRESULT,
    pub GetMessageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSDUdpMessageType) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetTTL: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetTTL: usize,
    #[cfg(feature = "shared")]
    pub GetTTL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTTL: usize,
    pub SetAlias: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetAlias: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "ws2"))]
pub trait IWSDUdpAddress_Impl: IWSDTransportAddress_Impl {
    fn SetSockaddr(&self, psockaddr: *const super::SOCKADDR_STORAGE) -> windows_core::Result<()>;
    fn GetSockaddr(&self, psockaddr: *mut super::SOCKADDR_STORAGE) -> windows_core::Result<()>;
    fn SetExclusive(&self, fexclusive: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetExclusive(&self) -> windows_core::Result<()>;
    fn SetMessageType(&self, messagetype: WSDUdpMessageType) -> windows_core::Result<()>;
    fn GetMessageType(&self) -> windows_core::Result<WSDUdpMessageType>;
    fn SetTTL(&self, dwttl: super::DWORD) -> windows_core::Result<()>;
    fn GetTTL(&self) -> windows_core::Result<super::DWORD>;
    fn SetAlias(&self, palias: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetAlias(&self) -> windows_core::Result<windows_core::GUID>;
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl IWSDUdpAddress_Vtbl {
    pub const fn new<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSockaddr<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psockaddr: *const super::SOCKADDR_STORAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpAddress_Impl::SetSockaddr(this, core::mem::transmute_copy(&psockaddr)).into()
            }
        }
        unsafe extern "system" fn GetSockaddr<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psockaddr: *mut super::SOCKADDR_STORAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpAddress_Impl::GetSockaddr(this, core::mem::transmute_copy(&psockaddr)).into()
            }
        }
        unsafe extern "system" fn SetExclusive<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fexclusive: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpAddress_Impl::SetExclusive(this, core::mem::transmute_copy(&fexclusive)).into()
            }
        }
        unsafe extern "system" fn GetExclusive<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpAddress_Impl::GetExclusive(this).into()
            }
        }
        unsafe extern "system" fn SetMessageType<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetype: WSDUdpMessageType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpAddress_Impl::SetMessageType(this, core::mem::transmute_copy(&messagetype)).into()
            }
        }
        unsafe extern "system" fn GetMessageType<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessagetype: *mut WSDUdpMessageType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDUdpAddress_Impl::GetMessageType(this) {
                    Ok(ok__) => {
                        pmessagetype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTTL<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwttl: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpAddress_Impl::SetTTL(this, core::mem::transmute_copy(&dwttl)).into()
            }
        }
        unsafe extern "system" fn GetTTL<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwttl: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDUdpAddress_Impl::GetTTL(this) {
                    Ok(ok__) => {
                        pdwttl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAlias<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palias: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpAddress_Impl::SetAlias(this, core::mem::transmute_copy(&palias)).into()
            }
        }
        unsafe extern "system" fn GetAlias<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palias: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDUdpAddress_Impl::GetAlias(this) {
                    Ok(ok__) => {
                        palias.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWSDTransportAddress_Vtbl::new::<Identity, OFFSET>(),
            SetSockaddr: SetSockaddr::<Identity, OFFSET>,
            GetSockaddr: GetSockaddr::<Identity, OFFSET>,
            SetExclusive: SetExclusive::<Identity, OFFSET>,
            GetExclusive: GetExclusive::<Identity, OFFSET>,
            SetMessageType: SetMessageType::<Identity, OFFSET>,
            GetMessageType: GetMessageType::<Identity, OFFSET>,
            SetTTL: SetTTL::<Identity, OFFSET>,
            GetTTL: GetTTL::<Identity, OFFSET>,
            SetAlias: SetAlias::<Identity, OFFSET>,
            GetAlias: GetAlias::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDUdpAddress as windows_core::Interface>::IID || iid == &<IWSDAddress as windows_core::Interface>::IID || iid == &<IWSDTransportAddress as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl windows_core::RuntimeName for IWSDUdpAddress {}
windows_core::imp::define_interface!(IWSDUdpMessageParameters, IWSDUdpMessageParameters_Vtbl, 0x9934149f_8f0c_447b_aa0b_73124b0ca7f0);
impl core::ops::Deref for IWSDUdpMessageParameters {
    type Target = IWSDMessageParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDUdpMessageParameters, windows_core::IUnknown, IWSDMessageParameters);
impl IWSDUdpMessageParameters {
    #[cfg(feature = "shared")]
    pub unsafe fn SetRetransmitParams(&self, pparams: *const WSDUdpRetransmitParams) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRetransmitParams)(windows_core::Interface::as_raw(self), pparams) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetRetransmitParams(&self, pparams: *mut WSDUdpRetransmitParams) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRetransmitParams)(windows_core::Interface::as_raw(self), pparams as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpMessageParameters_Vtbl {
    pub base__: IWSDMessageParameters_Vtbl,
    #[cfg(feature = "shared")]
    pub SetRetransmitParams: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSDUdpRetransmitParams) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetRetransmitParams: usize,
    #[cfg(feature = "shared")]
    pub GetRetransmitParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSDUdpRetransmitParams) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetRetransmitParams: usize,
}
#[cfg(feature = "shared")]
pub trait IWSDUdpMessageParameters_Impl: IWSDMessageParameters_Impl {
    fn SetRetransmitParams(&self, pparams: *const WSDUdpRetransmitParams) -> windows_core::Result<()>;
    fn GetRetransmitParams(&self, pparams: *mut WSDUdpRetransmitParams) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWSDUdpMessageParameters_Vtbl {
    pub const fn new<Identity: IWSDUdpMessageParameters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRetransmitParams<Identity: IWSDUdpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparams: *const WSDUdpRetransmitParams) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpMessageParameters_Impl::SetRetransmitParams(this, core::mem::transmute_copy(&pparams)).into()
            }
        }
        unsafe extern "system" fn GetRetransmitParams<Identity: IWSDUdpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparams: *mut WSDUdpRetransmitParams) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpMessageParameters_Impl::GetRetransmitParams(this, core::mem::transmute_copy(&pparams)).into()
            }
        }
        Self {
            base__: IWSDMessageParameters_Vtbl::new::<Identity, OFFSET>(),
            SetRetransmitParams: SetRetransmitParams::<Identity, OFFSET>,
            GetRetransmitParams: GetRetransmitParams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDUdpMessageParameters as windows_core::Interface>::IID || iid == &<IWSDMessageParameters as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWSDUdpMessageParameters {}
windows_core::imp::define_interface!(IWSDXMLContext, IWSDXMLContext_Vtbl, 0x75d8f3ee_3e5a_43b4_a15a_bcf6887460c0);
windows_core::imp::interface_hierarchy!(IWSDXMLContext, windows_core::IUnknown);
impl IWSDXMLContext {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn AddNamespace<P0, P1>(&self, pszuri: P0, pszsuggestedprefix: P1, ppnamespace: Option<*mut *mut WSDXML_NAMESPACE>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddNamespace)(windows_core::Interface::as_raw(self), pszuri.param().abi(), pszsuggestedprefix.param().abi(), ppnamespace.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn AddNameToNamespace<P0, P1>(&self, pszuri: P0, pszname: P1, ppname: Option<*mut *mut WSDXML_NAME>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddNameToNamespace)(windows_core::Interface::as_raw(self), pszuri.param().abi(), pszname.param().abi(), ppname.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SetNamespaces(&self, pnamespaces: &[PCWSDXML_NAMESPACE], blayernumber: super::BYTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNamespaces)(windows_core::Interface::as_raw(self), pnamespaces.as_ptr(), pnamespaces.len().try_into().unwrap(), blayernumber) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SetTypes(&self, ptypes: &[PCWSDXML_TYPE], blayernumber: super::BYTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTypes)(windows_core::Interface::as_raw(self), ptypes.as_ptr(), ptypes.len().try_into().unwrap(), blayernumber) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDXMLContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub AddNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut WSDXML_NAMESPACE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    AddNamespace: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub AddNameToNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut WSDXML_NAME) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    AddNameToNamespace: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SetNamespaces: unsafe extern "system" fn(*mut core::ffi::c_void, *const PCWSDXML_NAMESPACE, super::WORD, super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SetNamespaces: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SetTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *const PCWSDXML_TYPE, super::DWORD, super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SetTypes: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWSDXMLContext_Impl: windows_core::IUnknownImpl {
    fn AddNamespace(&self, pszuri: &windows_core::PCWSTR, pszsuggestedprefix: &windows_core::PCWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> windows_core::Result<()>;
    fn AddNameToNamespace(&self, pszuri: &windows_core::PCWSTR, pszname: &windows_core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> windows_core::Result<()>;
    fn SetNamespaces(&self, pnamespaces: *const PCWSDXML_NAMESPACE, wnamespacescount: super::WORD, blayernumber: super::BYTE) -> windows_core::Result<()>;
    fn SetTypes(&self, ptypes: *const PCWSDXML_TYPE, dwtypescount: super::DWORD, blayernumber: super::BYTE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWSDXMLContext_Vtbl {
    pub const fn new<Identity: IWSDXMLContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddNamespace<Identity: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszuri: windows_core::PCWSTR, pszsuggestedprefix: windows_core::PCWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDXMLContext_Impl::AddNamespace(this, core::mem::transmute(&pszuri), core::mem::transmute(&pszsuggestedprefix), core::mem::transmute_copy(&ppnamespace)).into()
            }
        }
        unsafe extern "system" fn AddNameToNamespace<Identity: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszuri: windows_core::PCWSTR, pszname: windows_core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDXMLContext_Impl::AddNameToNamespace(this, core::mem::transmute(&pszuri), core::mem::transmute(&pszname), core::mem::transmute_copy(&ppname)).into()
            }
        }
        unsafe extern "system" fn SetNamespaces<Identity: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespaces: *const PCWSDXML_NAMESPACE, wnamespacescount: super::WORD, blayernumber: super::BYTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDXMLContext_Impl::SetNamespaces(this, core::mem::transmute_copy(&pnamespaces), core::mem::transmute_copy(&wnamespacescount), core::mem::transmute_copy(&blayernumber)).into()
            }
        }
        unsafe extern "system" fn SetTypes<Identity: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypes: *const PCWSDXML_TYPE, dwtypescount: super::DWORD, blayernumber: super::BYTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDXMLContext_Impl::SetTypes(this, core::mem::transmute_copy(&ptypes), core::mem::transmute_copy(&dwtypescount), core::mem::transmute_copy(&blayernumber)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddNamespace: AddNamespace::<Identity, OFFSET>,
            AddNameToNamespace: AddNameToNamespace::<Identity, OFFSET>,
            SetNamespaces: SetNamespaces::<Identity, OFFSET>,
            SetTypes: SetTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDXMLContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWSDXMLContext {}
windows_core::imp::define_interface!(IWSDiscoveredService, IWSDiscoveredService_Vtbl, 0x4bad8a3b_b374_4420_9632_aac945b374aa);
windows_core::imp::interface_hierarchy!(IWSDiscoveredService, windows_core::IUnknown);
impl IWSDiscoveredService {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetEndpointReference(&self) -> windows_core::Result<*mut WSD_ENDPOINT_REFERENCE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEndpointReference)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetTypes(&self) -> windows_core::Result<*mut WSD_NAME_LIST> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetScopes(&self) -> windows_core::Result<*mut WSD_URI_LIST> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScopes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetXAddrs(&self) -> windows_core::Result<*mut WSD_URI_LIST> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXAddrs)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMetadataVersion(&self) -> windows_core::Result<super::ULONGLONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadataVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetExtendedDiscoXML(&self, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExtendedDiscoXML)(windows_core::Interface::as_raw(self), ppheaderany as _, ppbodyany as _) }
    }
    pub unsafe fn GetProbeResolveTag(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProbeResolveTag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRemoteTransportAddress(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRemoteTransportAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLocalTransportAddress(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalTransportAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLocalInterfaceGUID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalInterfaceGUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetInstanceId(&self) -> windows_core::Result<super::ULONGLONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInstanceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveredService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetEndpointReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WSD_ENDPOINT_REFERENCE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetEndpointReference: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WSD_NAME_LIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetTypes: usize,
    #[cfg(feature = "winnt")]
    pub GetScopes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WSD_URI_LIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetScopes: usize,
    #[cfg(feature = "winnt")]
    pub GetXAddrs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WSD_URI_LIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetXAddrs: usize,
    #[cfg(feature = "shared")]
    pub GetMetadataVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMetadataVersion: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetExtendedDiscoXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WSDXML_ELEMENT, *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetExtendedDiscoXML: usize,
    pub GetProbeResolveTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetRemoteTransportAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetLocalTransportAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetLocalInterfaceGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetInstanceId: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWSDiscoveredService_Impl: windows_core::IUnknownImpl {
    fn GetEndpointReference(&self) -> windows_core::Result<*mut WSD_ENDPOINT_REFERENCE>;
    fn GetTypes(&self) -> windows_core::Result<*mut WSD_NAME_LIST>;
    fn GetScopes(&self) -> windows_core::Result<*mut WSD_URI_LIST>;
    fn GetXAddrs(&self) -> windows_core::Result<*mut WSD_URI_LIST>;
    fn GetMetadataVersion(&self) -> windows_core::Result<super::ULONGLONG>;
    fn GetExtendedDiscoXML(&self, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn GetProbeResolveTag(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetRemoteTransportAddress(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetLocalTransportAddress(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetLocalInterfaceGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetInstanceId(&self) -> windows_core::Result<super::ULONGLONG>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWSDiscoveredService_Vtbl {
    pub const fn new<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEndpointReference<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppendpointreference: *mut *mut WSD_ENDPOINT_REFERENCE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetEndpointReference(this) {
                    Ok(ok__) => {
                        ppendpointreference.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypes<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptypeslist: *mut *mut WSD_NAME_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetTypes(this) {
                    Ok(ok__) => {
                        pptypeslist.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScopes<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppscopeslist: *mut *mut WSD_URI_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetScopes(this) {
                    Ok(ok__) => {
                        ppscopeslist.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetXAddrs<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppxaddrslist: *mut *mut WSD_URI_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetXAddrs(this) {
                    Ok(ok__) => {
                        ppxaddrslist.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMetadataVersion<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullmetadataversion: *mut super::ULONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetMetadataVersion(this) {
                    Ok(ok__) => {
                        pullmetadataversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExtendedDiscoXML<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveredService_Impl::GetExtendedDiscoXML(this, core::mem::transmute_copy(&ppheaderany), core::mem::transmute_copy(&ppbodyany)).into()
            }
        }
        unsafe extern "system" fn GetProbeResolveTag<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsztag: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetProbeResolveTag(this) {
                    Ok(ok__) => {
                        ppsztag.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRemoteTransportAddress<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszremotetransportaddress: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetRemoteTransportAddress(this) {
                    Ok(ok__) => {
                        ppszremotetransportaddress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocalTransportAddress<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszlocaltransportaddress: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetLocalTransportAddress(this) {
                    Ok(ok__) => {
                        ppszlocaltransportaddress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocalInterfaceGUID<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetLocalInterfaceGUID(this) {
                    Ok(ok__) => {
                        pguid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInstanceId<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullinstanceid: *mut super::ULONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetInstanceId(this) {
                    Ok(ok__) => {
                        pullinstanceid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEndpointReference: GetEndpointReference::<Identity, OFFSET>,
            GetTypes: GetTypes::<Identity, OFFSET>,
            GetScopes: GetScopes::<Identity, OFFSET>,
            GetXAddrs: GetXAddrs::<Identity, OFFSET>,
            GetMetadataVersion: GetMetadataVersion::<Identity, OFFSET>,
            GetExtendedDiscoXML: GetExtendedDiscoXML::<Identity, OFFSET>,
            GetProbeResolveTag: GetProbeResolveTag::<Identity, OFFSET>,
            GetRemoteTransportAddress: GetRemoteTransportAddress::<Identity, OFFSET>,
            GetLocalTransportAddress: GetLocalTransportAddress::<Identity, OFFSET>,
            GetLocalInterfaceGUID: GetLocalInterfaceGUID::<Identity, OFFSET>,
            GetInstanceId: GetInstanceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDiscoveredService as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWSDiscoveredService {}
windows_core::imp::define_interface!(IWSDiscoveryProvider, IWSDiscoveryProvider_Vtbl, 0x8ffc8e55_f0eb_480f_88b7_b435dd281d45);
windows_core::imp::interface_hierarchy!(IWSDiscoveryProvider, windows_core::IUnknown);
impl IWSDiscoveryProvider {
    #[cfg(feature = "shared")]
    pub unsafe fn SetAddressFamily(&self, dwaddressfamily: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAddressFamily)(windows_core::Interface::as_raw(self), dwaddressfamily) }
    }
    pub unsafe fn Attach<P0>(&self, psink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDiscoveryProviderNotify>,
    {
        unsafe { (windows_core::Interface::vtable(self).Attach)(windows_core::Interface::as_raw(self), psink.param().abi()) }
    }
    pub unsafe fn Detach(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Detach)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SearchById<P0, P1>(&self, pszid: P0, psztag: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SearchById)(windows_core::Interface::as_raw(self), pszid.param().abi(), psztag.param().abi()) }
    }
    pub unsafe fn SearchByAddress<P0, P1>(&self, pszaddress: P0, psztag: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SearchByAddress)(windows_core::Interface::as_raw(self), pszaddress.param().abi(), psztag.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SearchByType<P2, P3>(&self, ptypeslist: Option<*const WSD_NAME_LIST>, pscopeslist: Option<*const WSD_URI_LIST>, pszmatchby: P2, psztag: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SearchByType)(windows_core::Interface::as_raw(self), ptypeslist.unwrap_or(core::mem::zeroed()) as _, pscopeslist.unwrap_or(core::mem::zeroed()) as _, pszmatchby.param().abi(), psztag.param().abi()) }
    }
    pub unsafe fn GetXMLContext(&self) -> windows_core::Result<IWSDXMLContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXMLContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetAddressFamily: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetAddressFamily: usize,
    pub Attach: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Detach: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SearchById: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SearchByAddress: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SearchByType: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_NAME_LIST, *const WSD_URI_LIST, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SearchByType: usize,
    pub GetXMLContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWSDiscoveryProvider_Impl: windows_core::IUnknownImpl {
    fn SetAddressFamily(&self, dwaddressfamily: super::DWORD) -> windows_core::Result<()>;
    fn Attach(&self, psink: windows_core::Ref<IWSDiscoveryProviderNotify>) -> windows_core::Result<()>;
    fn Detach(&self) -> windows_core::Result<()>;
    fn SearchById(&self, pszid: &windows_core::PCWSTR, psztag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SearchByAddress(&self, pszaddress: &windows_core::PCWSTR, psztag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SearchByType(&self, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: &windows_core::PCWSTR, psztag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetXMLContext(&self) -> windows_core::Result<IWSDXMLContext>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWSDiscoveryProvider_Vtbl {
    pub const fn new<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAddressFamily<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaddressfamily: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProvider_Impl::SetAddressFamily(this, core::mem::transmute_copy(&dwaddressfamily)).into()
            }
        }
        unsafe extern "system" fn Attach<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProvider_Impl::Attach(this, core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn Detach<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProvider_Impl::Detach(this).into()
            }
        }
        unsafe extern "system" fn SearchById<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, psztag: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProvider_Impl::SearchById(this, core::mem::transmute(&pszid), core::mem::transmute(&psztag)).into()
            }
        }
        unsafe extern "system" fn SearchByAddress<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszaddress: windows_core::PCWSTR, psztag: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProvider_Impl::SearchByAddress(this, core::mem::transmute(&pszaddress), core::mem::transmute(&psztag)).into()
            }
        }
        unsafe extern "system" fn SearchByType<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: windows_core::PCWSTR, psztag: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProvider_Impl::SearchByType(this, core::mem::transmute_copy(&ptypeslist), core::mem::transmute_copy(&pscopeslist), core::mem::transmute(&pszmatchby), core::mem::transmute(&psztag)).into()
            }
        }
        unsafe extern "system" fn GetXMLContext<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveryProvider_Impl::GetXMLContext(this) {
                    Ok(ok__) => {
                        ppcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAddressFamily: SetAddressFamily::<Identity, OFFSET>,
            Attach: Attach::<Identity, OFFSET>,
            Detach: Detach::<Identity, OFFSET>,
            SearchById: SearchById::<Identity, OFFSET>,
            SearchByAddress: SearchByAddress::<Identity, OFFSET>,
            SearchByType: SearchByType::<Identity, OFFSET>,
            GetXMLContext: GetXMLContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDiscoveryProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWSDiscoveryProvider {}
windows_core::imp::define_interface!(IWSDiscoveryProviderNotify, IWSDiscoveryProviderNotify_Vtbl, 0x73ee3ced_b6e6_4329_a546_3e8ad46563d2);
windows_core::imp::interface_hierarchy!(IWSDiscoveryProviderNotify, windows_core::IUnknown);
impl IWSDiscoveryProviderNotify {
    pub unsafe fn Add<P0>(&self, pservice: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDiscoveredService>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pservice.param().abi()) }
    }
    pub unsafe fn Remove<P0>(&self, pservice: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDiscoveredService>,
    {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), pservice.param().abi()) }
    }
    pub unsafe fn SearchFailed<P1>(&self, hr: windows_core::HRESULT, psztag: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SearchFailed)(windows_core::Interface::as_raw(self), hr, psztag.param().abi()) }
    }
    pub unsafe fn SearchComplete<P0>(&self, psztag: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SearchComplete)(windows_core::Interface::as_raw(self), psztag.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryProviderNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SearchFailed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SearchComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IWSDiscoveryProviderNotify_Impl: windows_core::IUnknownImpl {
    fn Add(&self, pservice: windows_core::Ref<IWSDiscoveredService>) -> windows_core::Result<()>;
    fn Remove(&self, pservice: windows_core::Ref<IWSDiscoveredService>) -> windows_core::Result<()>;
    fn SearchFailed(&self, hr: windows_core::HRESULT, psztag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SearchComplete(&self, psztag: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IWSDiscoveryProviderNotify_Vtbl {
    pub const fn new<Identity: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Add<Identity: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pservice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProviderNotify_Impl::Add(this, core::mem::transmute_copy(&pservice)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pservice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProviderNotify_Impl::Remove(this, core::mem::transmute_copy(&pservice)).into()
            }
        }
        unsafe extern "system" fn SearchFailed<Identity: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, psztag: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProviderNotify_Impl::SearchFailed(this, core::mem::transmute_copy(&hr), core::mem::transmute(&psztag)).into()
            }
        }
        unsafe extern "system" fn SearchComplete<Identity: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztag: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProviderNotify_Impl::SearchComplete(this, core::mem::transmute(&psztag)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            SearchFailed: SearchFailed::<Identity, OFFSET>,
            SearchComplete: SearchComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDiscoveryProviderNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDiscoveryProviderNotify {}
windows_core::imp::define_interface!(IWSDiscoveryPublisher, IWSDiscoveryPublisher_Vtbl, 0xae01e1a8_3ff9_4148_8116_057cc616fe13);
windows_core::imp::interface_hierarchy!(IWSDiscoveryPublisher, windows_core::IUnknown);
impl IWSDiscoveryPublisher {
    #[cfg(feature = "shared")]
    pub unsafe fn SetAddressFamily(&self, dwaddressfamily: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAddressFamily)(windows_core::Interface::as_raw(self), dwaddressfamily) }
    }
    pub unsafe fn RegisterNotificationSink<P0>(&self, psink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDiscoveryPublisherNotify>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterNotificationSink)(windows_core::Interface::as_raw(self), psink.param().abi()) }
    }
    pub unsafe fn UnRegisterNotificationSink<P0>(&self, psink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDiscoveryPublisherNotify>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnRegisterNotificationSink)(windows_core::Interface::as_raw(self), psink.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn Publish<P0, P4>(&self, pszid: P0, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: P4, ptypeslist: Option<*const WSD_NAME_LIST>, pscopeslist: Option<*const WSD_URI_LIST>, pxaddrslist: Option<*const WSD_URI_LIST>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Publish)(windows_core::Interface::as_raw(self), pszid.param().abi(), ullmetadataversion, ullinstanceid, ullmessagenumber, pszsessionid.param().abi(), ptypeslist.unwrap_or(core::mem::zeroed()) as _, pscopeslist.unwrap_or(core::mem::zeroed()) as _, pxaddrslist.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn UnPublish<P0, P3>(&self, pszid: P0, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: P3, pany: Option<*const WSDXML_ELEMENT>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnPublish)(windows_core::Interface::as_raw(self), pszid.param().abi(), ullinstanceid, ullmessagenumber, pszsessionid.param().abi(), pany.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn MatchProbe<P1, P2, P6>(&self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: P1, pszid: P2, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: P6, ptypeslist: Option<*const WSD_NAME_LIST>, pscopeslist: Option<*const WSD_URI_LIST>, pxaddrslist: Option<*const WSD_URI_LIST>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWSDMessageParameters>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).MatchProbe)(windows_core::Interface::as_raw(self), pprobemessage, pmessageparameters.param().abi(), pszid.param().abi(), ullmetadataversion, ullinstanceid, ullmessagenumber, pszsessionid.param().abi(), ptypeslist.unwrap_or(core::mem::zeroed()) as _, pscopeslist.unwrap_or(core::mem::zeroed()) as _, pxaddrslist.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn MatchResolve<P1, P2, P6>(&self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: P1, pszid: P2, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: P6, ptypeslist: Option<*const WSD_NAME_LIST>, pscopeslist: Option<*const WSD_URI_LIST>, pxaddrslist: Option<*const WSD_URI_LIST>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWSDMessageParameters>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).MatchResolve)(windows_core::Interface::as_raw(self), presolvemessage, pmessageparameters.param().abi(), pszid.param().abi(), ullmetadataversion, ullinstanceid, ullmessagenumber, pszsessionid.param().abi(), ptypeslist.unwrap_or(core::mem::zeroed()) as _, pscopeslist.unwrap_or(core::mem::zeroed()) as _, pxaddrslist.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn PublishEx<P0, P4>(&self, pszid: P0, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: P4, ptypeslist: Option<*const WSD_NAME_LIST>, pscopeslist: Option<*const WSD_URI_LIST>, pxaddrslist: Option<*const WSD_URI_LIST>, pheaderany: Option<*const WSDXML_ELEMENT>, preferenceparameterany: Option<*const WSDXML_ELEMENT>, ppolicyany: Option<*const WSDXML_ELEMENT>, pendpointreferenceany: Option<*const WSDXML_ELEMENT>, pany: Option<*const WSDXML_ELEMENT>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PublishEx)(
                windows_core::Interface::as_raw(self),
                pszid.param().abi(),
                ullmetadataversion,
                ullinstanceid,
                ullmessagenumber,
                pszsessionid.param().abi(),
                ptypeslist.unwrap_or(core::mem::zeroed()) as _,
                pscopeslist.unwrap_or(core::mem::zeroed()) as _,
                pxaddrslist.unwrap_or(core::mem::zeroed()) as _,
                pheaderany.unwrap_or(core::mem::zeroed()) as _,
                preferenceparameterany.unwrap_or(core::mem::zeroed()) as _,
                ppolicyany.unwrap_or(core::mem::zeroed()) as _,
                pendpointreferenceany.unwrap_or(core::mem::zeroed()) as _,
                pany.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn MatchProbeEx<P1, P2, P6>(&self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: P1, pszid: P2, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: P6, ptypeslist: Option<*const WSD_NAME_LIST>, pscopeslist: Option<*const WSD_URI_LIST>, pxaddrslist: Option<*const WSD_URI_LIST>, pheaderany: Option<*const WSDXML_ELEMENT>, preferenceparameterany: Option<*const WSDXML_ELEMENT>, ppolicyany: Option<*const WSDXML_ELEMENT>, pendpointreferenceany: Option<*const WSDXML_ELEMENT>, pany: Option<*const WSDXML_ELEMENT>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWSDMessageParameters>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).MatchProbeEx)(
                windows_core::Interface::as_raw(self),
                pprobemessage,
                pmessageparameters.param().abi(),
                pszid.param().abi(),
                ullmetadataversion,
                ullinstanceid,
                ullmessagenumber,
                pszsessionid.param().abi(),
                ptypeslist.unwrap_or(core::mem::zeroed()) as _,
                pscopeslist.unwrap_or(core::mem::zeroed()) as _,
                pxaddrslist.unwrap_or(core::mem::zeroed()) as _,
                pheaderany.unwrap_or(core::mem::zeroed()) as _,
                preferenceparameterany.unwrap_or(core::mem::zeroed()) as _,
                ppolicyany.unwrap_or(core::mem::zeroed()) as _,
                pendpointreferenceany.unwrap_or(core::mem::zeroed()) as _,
                pany.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn MatchResolveEx<P1, P2, P6>(&self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: P1, pszid: P2, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: P6, ptypeslist: Option<*const WSD_NAME_LIST>, pscopeslist: Option<*const WSD_URI_LIST>, pxaddrslist: Option<*const WSD_URI_LIST>, pheaderany: Option<*const WSDXML_ELEMENT>, preferenceparameterany: Option<*const WSDXML_ELEMENT>, ppolicyany: Option<*const WSDXML_ELEMENT>, pendpointreferenceany: Option<*const WSDXML_ELEMENT>, pany: Option<*const WSDXML_ELEMENT>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWSDMessageParameters>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).MatchResolveEx)(
                windows_core::Interface::as_raw(self),
                presolvemessage,
                pmessageparameters.param().abi(),
                pszid.param().abi(),
                ullmetadataversion,
                ullinstanceid,
                ullmessagenumber,
                pszsessionid.param().abi(),
                ptypeslist.unwrap_or(core::mem::zeroed()) as _,
                pscopeslist.unwrap_or(core::mem::zeroed()) as _,
                pxaddrslist.unwrap_or(core::mem::zeroed()) as _,
                pheaderany.unwrap_or(core::mem::zeroed()) as _,
                preferenceparameterany.unwrap_or(core::mem::zeroed()) as _,
                ppolicyany.unwrap_or(core::mem::zeroed()) as _,
                pendpointreferenceany.unwrap_or(core::mem::zeroed()) as _,
                pany.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn RegisterScopeMatchingRule<P0>(&self, pscopematchingrule: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDScopeMatchingRule>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterScopeMatchingRule)(windows_core::Interface::as_raw(self), pscopematchingrule.param().abi()) }
    }
    pub unsafe fn UnRegisterScopeMatchingRule<P0>(&self, pscopematchingrule: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDScopeMatchingRule>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnRegisterScopeMatchingRule)(windows_core::Interface::as_raw(self), pscopematchingrule.param().abi()) }
    }
    pub unsafe fn GetXMLContext(&self) -> windows_core::Result<IWSDXMLContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXMLContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryPublisher_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetAddressFamily: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetAddressFamily: usize,
    pub RegisterNotificationSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnRegisterNotificationSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub Publish: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::ULONGLONG, super::ULONGLONG, super::ULONGLONG, windows_core::PCWSTR, *const WSD_NAME_LIST, *const WSD_URI_LIST, *const WSD_URI_LIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    Publish: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub UnPublish: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::ULONGLONG, super::ULONGLONG, windows_core::PCWSTR, *const WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    UnPublish: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub MatchProbe: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_SOAP_MESSAGE, *mut core::ffi::c_void, windows_core::PCWSTR, super::ULONGLONG, super::ULONGLONG, super::ULONGLONG, windows_core::PCWSTR, *const WSD_NAME_LIST, *const WSD_URI_LIST, *const WSD_URI_LIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    MatchProbe: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub MatchResolve: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_SOAP_MESSAGE, *mut core::ffi::c_void, windows_core::PCWSTR, super::ULONGLONG, super::ULONGLONG, super::ULONGLONG, windows_core::PCWSTR, *const WSD_NAME_LIST, *const WSD_URI_LIST, *const WSD_URI_LIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    MatchResolve: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub PublishEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::ULONGLONG, super::ULONGLONG, super::ULONGLONG, windows_core::PCWSTR, *const WSD_NAME_LIST, *const WSD_URI_LIST, *const WSD_URI_LIST, *const WSDXML_ELEMENT, *const WSDXML_ELEMENT, *const WSDXML_ELEMENT, *const WSDXML_ELEMENT, *const WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    PublishEx: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub MatchProbeEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_SOAP_MESSAGE, *mut core::ffi::c_void, windows_core::PCWSTR, super::ULONGLONG, super::ULONGLONG, super::ULONGLONG, windows_core::PCWSTR, *const WSD_NAME_LIST, *const WSD_URI_LIST, *const WSD_URI_LIST, *const WSDXML_ELEMENT, *const WSDXML_ELEMENT, *const WSDXML_ELEMENT, *const WSDXML_ELEMENT, *const WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    MatchProbeEx: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub MatchResolveEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_SOAP_MESSAGE, *mut core::ffi::c_void, windows_core::PCWSTR, super::ULONGLONG, super::ULONGLONG, super::ULONGLONG, windows_core::PCWSTR, *const WSD_NAME_LIST, *const WSD_URI_LIST, *const WSD_URI_LIST, *const WSDXML_ELEMENT, *const WSDXML_ELEMENT, *const WSDXML_ELEMENT, *const WSDXML_ELEMENT, *const WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    MatchResolveEx: usize,
    pub RegisterScopeMatchingRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnRegisterScopeMatchingRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetXMLContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWSDiscoveryPublisher_Impl: windows_core::IUnknownImpl {
    fn SetAddressFamily(&self, dwaddressfamily: super::DWORD) -> windows_core::Result<()>;
    fn RegisterNotificationSink(&self, psink: windows_core::Ref<IWSDiscoveryPublisherNotify>) -> windows_core::Result<()>;
    fn UnRegisterNotificationSink(&self, psink: windows_core::Ref<IWSDiscoveryPublisherNotify>) -> windows_core::Result<()>;
    fn Publish(&self, pszid: &windows_core::PCWSTR, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> windows_core::Result<()>;
    fn UnPublish(&self, pszid: &windows_core::PCWSTR, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: &windows_core::PCWSTR, pany: *const WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn MatchProbe(&self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: windows_core::Ref<IWSDMessageParameters>, pszid: &windows_core::PCWSTR, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> windows_core::Result<()>;
    fn MatchResolve(&self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: windows_core::Ref<IWSDMessageParameters>, pszid: &windows_core::PCWSTR, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> windows_core::Result<()>;
    fn PublishEx(&self, pszid: &windows_core::PCWSTR, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn MatchProbeEx(&self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: windows_core::Ref<IWSDMessageParameters>, pszid: &windows_core::PCWSTR, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn MatchResolveEx(&self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: windows_core::Ref<IWSDMessageParameters>, pszid: &windows_core::PCWSTR, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn RegisterScopeMatchingRule(&self, pscopematchingrule: windows_core::Ref<IWSDScopeMatchingRule>) -> windows_core::Result<()>;
    fn UnRegisterScopeMatchingRule(&self, pscopematchingrule: windows_core::Ref<IWSDScopeMatchingRule>) -> windows_core::Result<()>;
    fn GetXMLContext(&self) -> windows_core::Result<IWSDXMLContext>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWSDiscoveryPublisher_Vtbl {
    pub const fn new<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAddressFamily<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaddressfamily: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::SetAddressFamily(this, core::mem::transmute_copy(&dwaddressfamily)).into()
            }
        }
        unsafe extern "system" fn RegisterNotificationSink<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::RegisterNotificationSink(this, core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn UnRegisterNotificationSink<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::UnRegisterNotificationSink(this, core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn Publish<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::Publish(this, core::mem::transmute(&pszid), core::mem::transmute_copy(&ullmetadataversion), core::mem::transmute_copy(&ullinstanceid), core::mem::transmute_copy(&ullmessagenumber), core::mem::transmute(&pszsessionid), core::mem::transmute_copy(&ptypeslist), core::mem::transmute_copy(&pscopeslist), core::mem::transmute_copy(&pxaddrslist)).into()
            }
        }
        unsafe extern "system" fn UnPublish<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: windows_core::PCWSTR, pany: *const WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::UnPublish(this, core::mem::transmute(&pszid), core::mem::transmute_copy(&ullinstanceid), core::mem::transmute_copy(&ullmessagenumber), core::mem::transmute(&pszsessionid), core::mem::transmute_copy(&pany)).into()
            }
        }
        unsafe extern "system" fn MatchProbe<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::MatchProbe(this, core::mem::transmute_copy(&pprobemessage), core::mem::transmute_copy(&pmessageparameters), core::mem::transmute(&pszid), core::mem::transmute_copy(&ullmetadataversion), core::mem::transmute_copy(&ullinstanceid), core::mem::transmute_copy(&ullmessagenumber), core::mem::transmute(&pszsessionid), core::mem::transmute_copy(&ptypeslist), core::mem::transmute_copy(&pscopeslist), core::mem::transmute_copy(&pxaddrslist)).into()
            }
        }
        unsafe extern "system" fn MatchResolve<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::MatchResolve(this, core::mem::transmute_copy(&presolvemessage), core::mem::transmute_copy(&pmessageparameters), core::mem::transmute(&pszid), core::mem::transmute_copy(&ullmetadataversion), core::mem::transmute_copy(&ullinstanceid), core::mem::transmute_copy(&ullmessagenumber), core::mem::transmute(&pszsessionid), core::mem::transmute_copy(&ptypeslist), core::mem::transmute_copy(&pscopeslist), core::mem::transmute_copy(&pxaddrslist)).into()
            }
        }
        unsafe extern "system" fn PublishEx<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::PublishEx(
                    this,
                    core::mem::transmute(&pszid),
                    core::mem::transmute_copy(&ullmetadataversion),
                    core::mem::transmute_copy(&ullinstanceid),
                    core::mem::transmute_copy(&ullmessagenumber),
                    core::mem::transmute(&pszsessionid),
                    core::mem::transmute_copy(&ptypeslist),
                    core::mem::transmute_copy(&pscopeslist),
                    core::mem::transmute_copy(&pxaddrslist),
                    core::mem::transmute_copy(&pheaderany),
                    core::mem::transmute_copy(&preferenceparameterany),
                    core::mem::transmute_copy(&ppolicyany),
                    core::mem::transmute_copy(&pendpointreferenceany),
                    core::mem::transmute_copy(&pany),
                )
                .into()
            }
        }
        unsafe extern "system" fn MatchProbeEx<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::MatchProbeEx(
                    this,
                    core::mem::transmute_copy(&pprobemessage),
                    core::mem::transmute_copy(&pmessageparameters),
                    core::mem::transmute(&pszid),
                    core::mem::transmute_copy(&ullmetadataversion),
                    core::mem::transmute_copy(&ullinstanceid),
                    core::mem::transmute_copy(&ullmessagenumber),
                    core::mem::transmute(&pszsessionid),
                    core::mem::transmute_copy(&ptypeslist),
                    core::mem::transmute_copy(&pscopeslist),
                    core::mem::transmute_copy(&pxaddrslist),
                    core::mem::transmute_copy(&pheaderany),
                    core::mem::transmute_copy(&preferenceparameterany),
                    core::mem::transmute_copy(&ppolicyany),
                    core::mem::transmute_copy(&pendpointreferenceany),
                    core::mem::transmute_copy(&pany),
                )
                .into()
            }
        }
        unsafe extern "system" fn MatchResolveEx<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullmetadataversion: super::ULONGLONG, ullinstanceid: super::ULONGLONG, ullmessagenumber: super::ULONGLONG, pszsessionid: windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::MatchResolveEx(
                    this,
                    core::mem::transmute_copy(&presolvemessage),
                    core::mem::transmute_copy(&pmessageparameters),
                    core::mem::transmute(&pszid),
                    core::mem::transmute_copy(&ullmetadataversion),
                    core::mem::transmute_copy(&ullinstanceid),
                    core::mem::transmute_copy(&ullmessagenumber),
                    core::mem::transmute(&pszsessionid),
                    core::mem::transmute_copy(&ptypeslist),
                    core::mem::transmute_copy(&pscopeslist),
                    core::mem::transmute_copy(&pxaddrslist),
                    core::mem::transmute_copy(&pheaderany),
                    core::mem::transmute_copy(&preferenceparameterany),
                    core::mem::transmute_copy(&ppolicyany),
                    core::mem::transmute_copy(&pendpointreferenceany),
                    core::mem::transmute_copy(&pany),
                )
                .into()
            }
        }
        unsafe extern "system" fn RegisterScopeMatchingRule<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscopematchingrule: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::RegisterScopeMatchingRule(this, core::mem::transmute_copy(&pscopematchingrule)).into()
            }
        }
        unsafe extern "system" fn UnRegisterScopeMatchingRule<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscopematchingrule: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::UnRegisterScopeMatchingRule(this, core::mem::transmute_copy(&pscopematchingrule)).into()
            }
        }
        unsafe extern "system" fn GetXMLContext<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveryPublisher_Impl::GetXMLContext(this) {
                    Ok(ok__) => {
                        ppcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAddressFamily: SetAddressFamily::<Identity, OFFSET>,
            RegisterNotificationSink: RegisterNotificationSink::<Identity, OFFSET>,
            UnRegisterNotificationSink: UnRegisterNotificationSink::<Identity, OFFSET>,
            Publish: Publish::<Identity, OFFSET>,
            UnPublish: UnPublish::<Identity, OFFSET>,
            MatchProbe: MatchProbe::<Identity, OFFSET>,
            MatchResolve: MatchResolve::<Identity, OFFSET>,
            PublishEx: PublishEx::<Identity, OFFSET>,
            MatchProbeEx: MatchProbeEx::<Identity, OFFSET>,
            MatchResolveEx: MatchResolveEx::<Identity, OFFSET>,
            RegisterScopeMatchingRule: RegisterScopeMatchingRule::<Identity, OFFSET>,
            UnRegisterScopeMatchingRule: UnRegisterScopeMatchingRule::<Identity, OFFSET>,
            GetXMLContext: GetXMLContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDiscoveryPublisher as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWSDiscoveryPublisher {}
windows_core::imp::define_interface!(IWSDiscoveryPublisherNotify, IWSDiscoveryPublisherNotify_Vtbl, 0xe67651b0_337a_4b3c_9758_733388568251);
windows_core::imp::interface_hierarchy!(IWSDiscoveryPublisherNotify, windows_core::IUnknown);
impl IWSDiscoveryPublisherNotify {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn ProbeHandler<P1>(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWSDMessageParameters>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProbeHandler)(windows_core::Interface::as_raw(self), psoap, pmessageparameters.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn ResolveHandler<P1>(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWSDMessageParameters>,
    {
        unsafe { (windows_core::Interface::vtable(self).ResolveHandler)(windows_core::Interface::as_raw(self), psoap, pmessageparameters.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryPublisherNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub ProbeHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_SOAP_MESSAGE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    ProbeHandler: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub ResolveHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSD_SOAP_MESSAGE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    ResolveHandler: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWSDiscoveryPublisherNotify_Impl: windows_core::IUnknownImpl {
    fn ProbeHandler(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: windows_core::Ref<IWSDMessageParameters>) -> windows_core::Result<()>;
    fn ResolveHandler(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: windows_core::Ref<IWSDMessageParameters>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWSDiscoveryPublisherNotify_Vtbl {
    pub const fn new<Identity: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProbeHandler<Identity: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisherNotify_Impl::ProbeHandler(this, core::mem::transmute_copy(&psoap), core::mem::transmute_copy(&pmessageparameters)).into()
            }
        }
        unsafe extern "system" fn ResolveHandler<Identity: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisherNotify_Impl::ResolveHandler(this, core::mem::transmute_copy(&psoap), core::mem::transmute_copy(&pmessageparameters)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProbeHandler: ProbeHandler::<Identity, OFFSET>,
            ResolveHandler: ResolveHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDiscoveryPublisherNotify as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWSDiscoveryPublisherNotify {}
pub const MulticastDiscovery: DeviceDiscoveryMechanism = 0;
pub const NAMESPACE_ENCODING_ADDRESSING: i32 = 1;
pub const NAMESPACE_ENCODING_DEVPROF: i32 = 5;
pub const NAMESPACE_ENCODING_DISCOVERY: i32 = 2;
pub const NAMESPACE_ENCODING_DISCOVERY11: i32 = 8;
pub const NAMESPACE_ENCODING_ENVELOPE: i32 = 7;
pub const NAMESPACE_ENCODING_EVENTING: i32 = 0;
pub const NAMESPACE_ENCODING_INCLUDE: i32 = 3;
pub const NAMESPACE_ENCODING_MEX: i32 = 6;
pub const NAMESPACE_ENCODING_XML: i32 = 4;
pub const ONE_WAY: WSDUdpMessageType = 0;
pub const OpAnyElement: i32 = 6;
pub const OpAnyElements: i32 = 7;
pub const OpAnyNumber: i32 = 17;
pub const OpAnyText: i32 = 8;
pub const OpAnything: i32 = 16;
pub const OpAttribute_: i32 = 9;
pub const OpBeginAll: i32 = 14;
pub const OpBeginAnyElement: i32 = 3;
pub const OpBeginChoice: i32 = 10;
pub const OpBeginElement_: i32 = 2;
pub const OpBeginSequence: i32 = 12;
pub const OpElement_: i32 = 5;
pub const OpEndAll: i32 = 15;
pub const OpEndChoice: i32 = 11;
pub const OpEndElement: i32 = 4;
pub const OpEndOfTable: i32 = 1;
pub const OpEndSequence: i32 = 13;
pub const OpFormatBool_: i32 = 20;
pub const OpFormatDateTime_: i32 = 40;
pub const OpFormatDom_: i32 = 30;
pub const OpFormatDouble_: i32 = 42;
pub const OpFormatDuration_: i32 = 39;
pub const OpFormatDynamicType_: i32 = 37;
pub const OpFormatFloat_: i32 = 41;
pub const OpFormatInt16_: i32 = 22;
pub const OpFormatInt32_: i32 = 23;
pub const OpFormatInt64_: i32 = 24;
pub const OpFormatInt8_: i32 = 21;
pub const OpFormatListInsertTail_: i32 = 35;
pub const OpFormatLookupType_: i32 = 38;
pub const OpFormatMax: i32 = 46;
pub const OpFormatName_: i32 = 34;
pub const OpFormatStruct_: i32 = 31;
pub const OpFormatType_: i32 = 36;
pub const OpFormatUInt16_: i32 = 26;
pub const OpFormatUInt32_: i32 = 27;
pub const OpFormatUInt64_: i32 = 28;
pub const OpFormatUInt8_: i32 = 25;
pub const OpFormatUnicodeString_: i32 = 29;
pub const OpFormatUri_: i32 = 32;
pub const OpFormatUuidUri_: i32 = 33;
pub const OpFormatXMLDeclaration_: i32 = 45;
pub const OpNone: i32 = 0;
pub const OpOneOrMore: i32 = 18;
pub const OpOptional: i32 = 19;
pub const OpProcess_: i32 = 43;
pub const OpQualifiedAttribute_: i32 = 44;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCWSDXML_NAMESPACE = *const _WSDXML_NAMESPACE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCWSDXML_TYPE = *const _WSDXML_TYPE;
#[cfg(feature = "shared")]
pub type PWSD_CONFIG_ADDRESSES = *mut WSD_CONFIG_ADDRESSES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWSD_CONFIG_PARAM = *mut WSD_CONFIG_PARAM;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
pub type PWSD_SECURITY_CERT_VALIDATION = *mut WSD_SECURITY_CERT_VALIDATION;
pub type PWSD_SECURITY_HTTP_AUTH_SCHEMES = *mut u32;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
pub type PWSD_SECURITY_SIGNATURE_VALIDATION = *mut WSD_SECURITY_SIGNATURE_VALIDATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWSD_SOAP_MESSAGE_HANDLER = Option<unsafe extern "C" fn(thisunknown: windows_core::Ref<windows_core::IUnknown>, event: *mut WSD_EVENT) -> windows_core::HRESULT>;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REQUESTBODY_GetStatus {
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REQUESTBODY_Renew {
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REQUESTBODY_Subscribe {
    pub EndTo: *mut WSD_ENDPOINT_REFERENCE,
    pub Delivery: *mut WSD_EVENTING_DELIVERY_MODE,
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Filter: *mut WSD_EVENTING_FILTER,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REQUESTBODY_Unsubscribe {
    pub any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RESPONSEBODY_GetMetadata {
    pub Metadata: *mut WSD_METADATA_SECTION_LIST,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RESPONSEBODY_GetStatus {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RESPONSEBODY_Renew {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RESPONSEBODY_Subscribe {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RESPONSEBODY_SubscriptionEnd {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub Status: *const super::WCHAR,
    pub Reason: *mut WSD_LOCALIZED_STRING,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "shared")]
pub const SOAP_Actor: super::DWORD = 458757;
#[cfg(feature = "shared")]
pub const SOAP_Body: super::DWORD = 458754;
#[cfg(feature = "shared")]
pub const SOAP_Code: super::DWORD = 458759;
#[cfg(feature = "shared")]
pub const SOAP_DataEncodingUnknown: super::DWORD = 458772;
#[cfg(feature = "shared")]
pub const SOAP_Detail: super::DWORD = 458766;
#[cfg(feature = "shared")]
pub const SOAP_Element: super::DWORD = 458775;
#[cfg(feature = "shared")]
pub const SOAP_Envelope: super::DWORD = 458752;
#[cfg(feature = "shared")]
pub const SOAP_Fault: super::DWORD = 458758;
#[cfg(feature = "shared")]
pub const SOAP_Header: super::DWORD = 458753;
#[cfg(feature = "shared")]
pub const SOAP_MustUnderstand: super::DWORD = 458755;
#[cfg(feature = "shared")]
pub const SOAP_MustUnderstand_1: super::DWORD = 458771;
#[cfg(feature = "shared")]
pub const SOAP_Node: super::DWORD = 458764;
#[cfg(feature = "shared")]
pub const SOAP_NotUnderstood: super::DWORD = 458769;
#[cfg(feature = "shared")]
pub const SOAP_Reason: super::DWORD = 458762;
#[cfg(feature = "shared")]
pub const SOAP_Receiver: super::DWORD = 458774;
#[cfg(feature = "shared")]
pub const SOAP_Relay: super::DWORD = 458776;
#[cfg(feature = "shared")]
pub const SOAP_Role: super::DWORD = 458756;
#[cfg(feature = "shared")]
pub const SOAP_Role_1: super::DWORD = 458765;
#[cfg(feature = "shared")]
pub const SOAP_Sender: super::DWORD = 458773;
#[cfg(feature = "shared")]
pub const SOAP_Subcode: super::DWORD = 458760;
#[cfg(feature = "shared")]
pub const SOAP_SupportedEnvelope: super::DWORD = 458768;
#[cfg(feature = "shared")]
pub const SOAP_Text: super::DWORD = 458763;
#[cfg(feature = "shared")]
pub const SOAP_Upgrade: super::DWORD = 458767;
#[cfg(feature = "shared")]
pub const SOAP_Value: super::DWORD = 458761;
#[cfg(feature = "shared")]
pub const SOAP_VersionMismatch: super::DWORD = 458770;
pub const SecureDirectedDiscovery: DeviceDiscoveryMechanism = 2;
pub const TWO_WAY: WSDUdpMessageType = 1;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_BYE: super::DWORD = 38;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_HELLO: super::DWORD = 39;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_PROBE: super::DWORD = 37;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_PROBE_MATCH: super::DWORD = 35;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_PROBE_MATCHES: super::DWORD = 36;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_RESOLVE: super::DWORD = 34;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_RESOLVE_MATCH: super::DWORD = 32;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_RESOLVE_MATCHES: super::DWORD = 33;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_APP_SEQUENCE: super::DWORD = 9;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_BYE: super::DWORD = 19;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_ENDPOINT_REFERENCE: super::DWORD = 24;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_EVENTING_DELIVERY_MODE: super::DWORD = 30;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_EVENTING_DELIVERY_MODE_PUSH: super::DWORD = 29;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_EVENTING_EXPIRES: super::DWORD = 28;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_EVENTING_FILTER: super::DWORD = 27;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_EVENTING_FILTER_ACTION: super::DWORD = 26;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_HEADER_RELATESTO: super::DWORD = 10;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_HELLO: super::DWORD = 21;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_HOST_METADATA: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_LOCALIZED_STRING: super::DWORD = 31;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_METADATA_SECTION: super::DWORD = 25;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_PROBE: super::DWORD = 18;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_PROBE_MATCH: super::DWORD = 16;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_PROBE_MATCHES: super::DWORD = 17;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_REFERENCE_PARAMETERS: super::DWORD = 22;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_REFERENCE_PROPERTIES: super::DWORD = 23;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_RELATIONSHIP_METADATA: super::DWORD = 0;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_RESOLVE: super::DWORD = 15;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_RESOLVE_MATCH: super::DWORD = 14;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_RESOLVE_MATCHES: super::DWORD = 13;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SCOPES: super::DWORD = 20;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SERVICE_METADATA: super::DWORD = 2;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_FAULT: super::DWORD = 8;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_FAULT_CODE: super::DWORD = 7;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_FAULT_REASON: super::DWORD = 5;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_FAULT_SUBCODE: super::DWORD = 6;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_HEADER: super::DWORD = 11;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_MESSAGE: super::DWORD = 12;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_THIS_DEVICE_METADATA: super::DWORD = 3;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_THIS_MODEL_METADATA: super::DWORD = 4;
#[cfg(feature = "shared")]
pub const WSA_Action: super::DWORD = 65550;
#[cfg(feature = "shared")]
pub const WSA_ActionNotSupported: super::DWORD = 65555;
#[cfg(feature = "shared")]
pub const WSA_Address: super::DWORD = 65537;
#[cfg(feature = "shared")]
pub const WSA_DestinationUnreachable: super::DWORD = 65554;
#[cfg(feature = "shared")]
pub const WSA_Element: super::DWORD = 65558;
#[cfg(feature = "shared")]
pub const WSA_EndpointReference: super::DWORD = 65536;
#[cfg(feature = "shared")]
pub const WSA_EndpointUnavailable: super::DWORD = 65556;
#[cfg(feature = "shared")]
pub const WSA_FaultTo: super::DWORD = 65548;
#[cfg(feature = "shared")]
pub const WSA_From: super::DWORD = 65547;
#[cfg(feature = "shared")]
pub const WSA_InvalidMessageInformationHeader: super::DWORD = 65552;
#[cfg(feature = "shared")]
pub const WSA_MessageID: super::DWORD = 65542;
#[cfg(feature = "shared")]
pub const WSA_MessageInformationHeaderRequired: super::DWORD = 65553;
#[cfg(feature = "shared")]
pub const WSA_PortName: super::DWORD = 65541;
#[cfg(feature = "shared")]
pub const WSA_PortType: super::DWORD = 65539;
#[cfg(feature = "shared")]
pub const WSA_Recipient: super::DWORD = 65551;
#[cfg(feature = "shared")]
pub const WSA_ReferenceParameters: super::DWORD = 65538;
#[cfg(feature = "shared")]
pub const WSA_ReferenceProperties: super::DWORD = 65557;
#[cfg(feature = "shared")]
pub const WSA_RelatesTo: super::DWORD = 65543;
#[cfg(feature = "shared")]
pub const WSA_RelationshipType: super::DWORD = 65544;
#[cfg(feature = "shared")]
pub const WSA_Reply: super::DWORD = 65545;
#[cfg(feature = "shared")]
pub const WSA_ReplyAfter: super::DWORD = 65560;
#[cfg(feature = "shared")]
pub const WSA_ReplyTo: super::DWORD = 65546;
#[cfg(feature = "shared")]
pub const WSA_RetryAfter: super::DWORD = 65559;
#[cfg(feature = "shared")]
pub const WSA_ServiceName: super::DWORD = 65540;
#[cfg(feature = "shared")]
pub const WSA_To: super::DWORD = 65549;
pub const WSDAPI_ADDRESSFAMILY_IPV4: i32 = 1;
pub const WSDAPI_ADDRESSFAMILY_IPV6: i32 = 2;
pub const WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES: i32 = 1;
pub const WSDAPI_OPTION_MAX_INBOUND_MESSAGE_SIZE: i32 = 1;
pub const WSDAPI_OPTION_TRACE_XML_TO_DEBUGGER: i32 = 2;
pub const WSDAPI_OPTION_TRACE_XML_TO_FILE: i32 = 3;
pub const WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS: i32 = 0;
pub const WSDAPI_SSL_CERT_IGNORE_EXPIRY: i32 = 2;
pub const WSDAPI_SSL_CERT_IGNORE_INVALID_CN: i32 = 16;
pub const WSDAPI_SSL_CERT_IGNORE_REVOCATION: i32 = 1;
pub const WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA: i32 = 8;
pub const WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE: i32 = 4;
pub const WSDET_INCOMING_FAULT: WSDEventType = 2;
pub const WSDET_INCOMING_MESSAGE: WSDEventType = 1;
pub const WSDET_NONE: WSDEventType = 0;
pub const WSDET_RESPONSE_TIMEOUT: WSDEventType = 4;
pub const WSDET_TRANSMISSION_FAILURE: WSDEventType = 3;
pub type WSDEventType = i32;
#[cfg(feature = "shared")]
pub const WSDISCO_AppSequence: super::DWORD = 131083;
#[cfg(feature = "shared")]
pub const WSDISCO_Bye: super::DWORD = 131073;
#[cfg(feature = "shared")]
pub const WSDISCO_DiscoveryProxy: super::DWORD = 131089;
#[cfg(feature = "shared")]
pub const WSDISCO_Element: super::DWORD = 131091;
#[cfg(feature = "shared")]
pub const WSDISCO_Hello: super::DWORD = 131072;
#[cfg(feature = "shared")]
pub const WSDISCO_Id: super::DWORD = 131093;
#[cfg(feature = "shared")]
pub const WSDISCO_InstanceId: super::DWORD = 131084;
#[cfg(feature = "shared")]
pub const WSDISCO_KeyId: super::DWORD = 131097;
#[cfg(feature = "shared")]
pub const WSDISCO_MatchBy: super::DWORD = 131087;
#[cfg(feature = "shared")]
pub const WSDISCO_MessageNumber: super::DWORD = 131086;
#[cfg(feature = "shared")]
pub const WSDISCO_MetadataVersion: super::DWORD = 131081;
#[cfg(feature = "shared")]
pub const WSDISCO_Probe: super::DWORD = 131074;
#[cfg(feature = "shared")]
pub const WSDISCO_ProbeMatch: super::DWORD = 131076;
#[cfg(feature = "shared")]
pub const WSDISCO_ProbeMatches: super::DWORD = 131075;
#[cfg(feature = "shared")]
pub const WSDISCO_Refs: super::DWORD = 131098;
#[cfg(feature = "shared")]
pub const WSDISCO_Resolve: super::DWORD = 131077;
#[cfg(feature = "shared")]
pub const WSDISCO_ResolveMatch: super::DWORD = 131078;
#[cfg(feature = "shared")]
pub const WSDISCO_ResolveMatches: super::DWORD = 131092;
#[cfg(feature = "shared")]
pub const WSDISCO_Scheme: super::DWORD = 131096;
#[cfg(feature = "shared")]
pub const WSDISCO_Scopes: super::DWORD = 131080;
#[cfg(feature = "shared")]
pub const WSDISCO_Security: super::DWORD = 131094;
#[cfg(feature = "shared")]
pub const WSDISCO_SequenceId: super::DWORD = 131085;
#[cfg(feature = "shared")]
pub const WSDISCO_Sig: super::DWORD = 131095;
#[cfg(feature = "shared")]
pub const WSDISCO_TargetService: super::DWORD = 131090;
#[cfg(feature = "shared")]
pub const WSDISCO_Transport: super::DWORD = 131082;
#[cfg(feature = "shared")]
pub const WSDISCO_Types: super::DWORD = 131079;
#[cfg(feature = "shared")]
pub const WSDISCO_XAddrs: super::DWORD = 131088;
pub type WSDUdpMessageType = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSDUdpRetransmitParams {
    pub ulSendDelay: super::ULONG,
    pub ulRepeat: super::ULONG,
    pub ulRepeatMinDelay: super::ULONG,
    pub ulRepeatMaxDelay: super::ULONG,
    pub ulRepeatUpperDelay: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_ATTRIBUTE = _WSDXML_ATTRIBUTE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_ELEMENT = _WSDXML_ELEMENT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_ELEMENT_LIST = _WSDXML_ELEMENT_LIST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_NAME = _WSDXML_NAME;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_NAMESPACE = _WSDXML_NAMESPACE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_NODE = _WSDXML_NODE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_PREFIX_MAPPING = _WSDXML_PREFIX_MAPPING;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_TEXT = _WSDXML_TEXT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_TYPE = _WSDXML_TYPE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_APP_SEQUENCE = _WSD_APP_SEQUENCE;
#[cfg(feature = "shared")]
pub const WSD_Actions: super::DWORD = 327682;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_BYE = _WSD_BYE;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSD_CONFIG_ADDRESSES {
    pub addresses: *mut Option<IWSDAddress>,
    pub dwAddressCount: super::DWORD,
}
pub const WSD_CONFIG_DEVICE_ADDRESSES: WSD_CONFIG_PARAM_TYPE = 10;
pub const WSD_CONFIG_HOSTING_ADDRESSES: WSD_CONFIG_PARAM_TYPE = 9;
pub const WSD_CONFIG_MAX_INBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = 1;
pub const WSD_CONFIG_MAX_OUTBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSD_CONFIG_PARAM {
    pub configParamType: WSD_CONFIG_PARAM_TYPE,
    pub pConfigData: super::PVOID,
    pub dwConfigDataSize: super::DWORD,
}
pub type WSD_CONFIG_PARAM_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSD_DATETIME {
    pub isPositive: windows_core::BOOL,
    pub year: super::ULONG,
    pub month: super::UCHAR,
    pub day: super::UCHAR,
    pub hour: super::UCHAR,
    pub minute: super::UCHAR,
    pub second: super::UCHAR,
    pub millisecond: super::UINT,
    pub TZIsLocal: windows_core::BOOL,
    pub TZIsPositive: windows_core::BOOL,
    pub TZHour: super::UCHAR,
    pub TZMinute: super::UCHAR,
}
pub const WSD_DEFAULT_EVENTING_ADDRESS: windows_core::PCWSTR = windows_core::w!("http://*:5357/");
pub const WSD_DEFAULT_HOSTING_ADDRESS: windows_core::PCWSTR = windows_core::w!("http://*:5357/");
pub const WSD_DEFAULT_SECURE_HOSTING_ADDRESS: windows_core::PCWSTR = windows_core::w!("https://*:5358/");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSD_DURATION {
    pub isPositive: windows_core::BOOL,
    pub year: super::ULONG,
    pub month: super::ULONG,
    pub day: super::ULONG,
    pub hour: super::ULONG,
    pub minute: super::ULONG,
    pub second: super::ULONG,
    pub millisecond: super::ULONG,
}
#[cfg(feature = "shared")]
pub const WSD_Data: super::DWORD = 327701;
#[cfg(feature = "shared")]
pub const WSD_Device: super::DWORD = 327680;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_ENDPOINT_REFERENCE = _WSD_ENDPOINT_REFERENCE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_ENDPOINT_REFERENCE_LIST = _WSD_ENDPOINT_REFERENCE_LIST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_EVENT = _WSD_EVENT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_EVENTING_DELIVERY_MODE = _WSD_EVENTING_DELIVERY_MODE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_EVENTING_DELIVERY_MODE_PUSH = _WSD_EVENTING_DELIVERY_MODE_PUSH;
#[cfg(feature = "shared")]
pub type WSD_EVENTING_EXPIRES = _WSD_EVENTING_EXPIRES;
#[cfg(feature = "winnt")]
pub type WSD_EVENTING_FILTER = _WSD_EVENTING_FILTER;
#[cfg(feature = "winnt")]
pub type WSD_EVENTING_FILTER_ACTION = _WSD_EVENTING_FILTER_ACTION;
#[cfg(feature = "shared")]
pub const WSD_Element: super::DWORD = 327683;
#[cfg(feature = "shared")]
pub const WSD_FilterActionNotSupported: super::DWORD = 327681;
#[cfg(feature = "shared")]
pub const WSD_FirmwareVersion: super::DWORD = 327693;
#[cfg(feature = "shared")]
pub const WSD_FriendlyName: super::DWORD = 327692;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_HANDLER_CONTEXT = _WSD_HANDLER_CONTEXT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_HEADER_RELATESTO = _WSD_HEADER_RELATESTO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_HELLO = _WSD_HELLO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_HOST_METADATA = _WSD_HOST_METADATA;
#[cfg(feature = "shared")]
pub const WSD_Host: super::DWORD = 327697;
#[cfg(feature = "shared")]
pub const WSD_Hosted: super::DWORD = 327698;
#[cfg(feature = "winnt")]
pub type WSD_LOCALIZED_STRING = _WSD_LOCALIZED_STRING;
#[cfg(feature = "winnt")]
pub type WSD_LOCALIZED_STRING_LIST = _WSD_LOCALIZED_STRING_LIST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_METADATA_SECTION = _WSD_METADATA_SECTION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_METADATA_SECTION_LIST = _WSD_METADATA_SECTION_LIST;
#[cfg(feature = "shared")]
pub const WSD_Manufacturer: super::DWORD = 327685;
#[cfg(feature = "shared")]
pub const WSD_ManufacturerUrl: super::DWORD = 327686;
#[cfg(feature = "shared")]
pub const WSD_ModelName: super::DWORD = 327687;
#[cfg(feature = "shared")]
pub const WSD_ModelNumber: super::DWORD = 327688;
#[cfg(feature = "shared")]
pub const WSD_ModelUrl: super::DWORD = 327689;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_NAME_LIST = _WSD_NAME_LIST;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct WSD_OPERATION {
    pub RequestType: *mut WSDXML_TYPE,
    pub ResponseType: *mut WSDXML_TYPE,
    pub RequestStubFunction: WSD_STUB_FUNCTION,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSD_PORT_TYPE {
    pub EncodedName: super::DWORD,
    pub OperationCount: super::DWORD,
    pub Operations: *mut WSD_OPERATION,
    pub ProtocolType: WSD_PROTOCOL_TYPE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_PROBE = _WSD_PROBE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_PROBE_MATCH = _WSD_PROBE_MATCH;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_PROBE_MATCHES = _WSD_PROBE_MATCHES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_PROBE_MATCH_LIST = _WSD_PROBE_MATCH_LIST;
pub type WSD_PROTOCOL_TYPE = i32;
pub const WSD_PT_ALL: WSD_PROTOCOL_TYPE = 255;
pub const WSD_PT_HTTP: WSD_PROTOCOL_TYPE = 2;
pub const WSD_PT_HTTPS: WSD_PROTOCOL_TYPE = 4;
pub const WSD_PT_NONE: WSD_PROTOCOL_TYPE = 0;
pub const WSD_PT_UDP: WSD_PROTOCOL_TYPE = 1;
#[cfg(feature = "shared")]
pub const WSD_PresentationUrl: super::DWORD = 327690;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_REFERENCE_PARAMETERS = _WSD_REFERENCE_PARAMETERS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_REFERENCE_PROPERTIES = _WSD_REFERENCE_PROPERTIES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_RELATIONSHIP_METADATA = _WSD_RELATIONSHIP_METADATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_RESOLVE = _WSD_RESOLVE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_RESOLVE_MATCH = _WSD_RESOLVE_MATCH;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_RESOLVE_MATCHES = _WSD_RESOLVE_MATCHES;
#[cfg(feature = "shared")]
pub const WSD_Relationship: super::DWORD = 327699;
#[cfg(feature = "winnt")]
pub type WSD_SCOPES = _WSD_SCOPES;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSD_SECURITY_CERT_VALIDATION {
    pub certMatchArray: *mut super::PCCERT_CONTEXT,
    pub dwCertMatchArrayCount: super::DWORD,
    pub hCertMatchStore: super::HCERTSTORE,
    pub hCertIssuerStore: super::HCERTSTORE,
    pub dwCertCheckOptions: super::DWORD,
    pub pszCNGHashAlgId: windows_core::PCWSTR,
    pub pbCertHash: *mut super::BYTE,
    pub dwCertHashSize: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSD_SECURITY_CERT_VALIDATION_V1 {
    pub certMatchArray: *mut super::PCCERT_CONTEXT,
    pub dwCertMatchArrayCount: super::DWORD,
    pub hCertMatchStore: super::HCERTSTORE,
    pub hCertIssuerStore: super::HCERTSTORE,
    pub dwCertCheckOptions: super::DWORD,
}
pub const WSD_SECURITY_COMPACTSIG_SIGNING_CERT: WSD_CONFIG_PARAM_TYPE = 7;
pub const WSD_SECURITY_COMPACTSIG_VALIDATION: WSD_CONFIG_PARAM_TYPE = 8;
pub type WSD_SECURITY_HTTP_AUTH_SCHEMES = u32;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE: i32 = 1;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM: i32 = 2;
pub const WSD_SECURITY_REQUIRE_CLIENT_CERT_OR_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 12;
pub const WSD_SECURITY_REQUIRE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 11;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSD_SECURITY_SIGNATURE_VALIDATION {
    pub signingCertArray: *mut super::PCCERT_CONTEXT,
    pub dwSigningCertArrayCount: super::DWORD,
    pub hSigningCertStore: super::HCERTSTORE,
    pub dwFlags: super::DWORD,
}
pub const WSD_SECURITY_SSL_CERT_FOR_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 3;
pub const WSD_SECURITY_SSL_CLIENT_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = 5;
pub const WSD_SECURITY_SSL_NEGOTIATE_CLIENT_CERT: WSD_CONFIG_PARAM_TYPE = 6;
pub const WSD_SECURITY_SSL_SERVER_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = 4;
pub const WSD_SECURITY_USE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 13;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_SERVICE_METADATA = _WSD_SERVICE_METADATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_SERVICE_METADATA_LIST = _WSD_SERVICE_METADATA_LIST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_SOAP_FAULT = _WSD_SOAP_FAULT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_SOAP_FAULT_CODE = _WSD_SOAP_FAULT_CODE;
#[cfg(feature = "winnt")]
pub type WSD_SOAP_FAULT_REASON = _WSD_SOAP_FAULT_REASON;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_SOAP_FAULT_SUBCODE = _WSD_SOAP_FAULT_SUBCODE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_SOAP_HEADER = _WSD_SOAP_HEADER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_SOAP_MESSAGE = _WSD_SOAP_MESSAGE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_STUB_FUNCTION = Option<unsafe extern "C" fn(server: windows_core::Ref<windows_core::IUnknown>, session: windows_core::Ref<IWSDServiceMessaging>, event: *mut WSD_EVENT) -> windows_core::HRESULT>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    pub hr: windows_core::HRESULT,
    pub eventHandle: super::HANDLE,
    pub messageParameters: core::mem::ManuallyDrop<Option<IWSDMessageParameters>>,
    pub results: *mut core::ffi::c_void,
}
#[cfg(feature = "shared")]
pub const WSD_SerialNumber: super::DWORD = 327694;
#[cfg(feature = "shared")]
pub const WSD_ServiceId: super::DWORD = 327696;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_THIS_DEVICE_METADATA = _WSD_THIS_DEVICE_METADATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_THIS_MODEL_METADATA = _WSD_THIS_MODEL_METADATA;
#[cfg(feature = "shared")]
pub const WSD_ThisDevice: super::DWORD = 327691;
#[cfg(feature = "shared")]
pub const WSD_ThisModel: super::DWORD = 327684;
#[cfg(feature = "shared")]
pub const WSD_Type: super::DWORD = 327700;
#[cfg(feature = "shared")]
pub const WSD_Types: super::DWORD = 327695;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSD_UNKNOWN_LOOKUP {
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "winnt")]
pub type WSD_URI_LIST = _WSD_URI_LIST;
#[cfg(feature = "shared")]
pub const WSE_Any: super::DWORD = 27;
#[cfg(feature = "shared")]
pub const WSE_Any_1: super::DWORD = 29;
#[cfg(feature = "shared")]
pub const WSE_Data: super::DWORD = 19;
#[cfg(feature = "shared")]
pub const WSE_DateTime: super::DWORD = 22;
#[cfg(feature = "shared")]
pub const WSE_Delivery: super::DWORD = 20;
#[cfg(feature = "shared")]
pub const WSE_DeliveryModeRequestedUnavailable: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const WSE_Dialect: super::DWORD = 24;
#[cfg(feature = "shared")]
pub const WSE_Duration: super::DWORD = 21;
#[cfg(feature = "shared")]
pub const WSE_EndTo: super::DWORD = 15;
#[cfg(feature = "shared")]
pub const WSE_EventSink: super::DWORD = 36;
#[cfg(feature = "shared")]
pub const WSE_EventSourceUnableToProcess: super::DWORD = 6;
#[cfg(feature = "shared")]
pub const WSE_Eventing: super::DWORD = 11;
#[cfg(feature = "shared")]
pub const WSE_Expires: super::DWORD = 23;
#[cfg(feature = "shared")]
pub const WSE_Filter: super::DWORD = 26;
#[cfg(feature = "shared")]
pub const WSE_FilterAction: super::DWORD = 25;
#[cfg(feature = "shared")]
pub const WSE_FilteringNotSupported: super::DWORD = 4;
#[cfg(feature = "shared")]
pub const WSE_FilteringRequestedUnavailable: super::DWORD = 5;
#[cfg(feature = "shared")]
pub const WSE_GetStatus: super::DWORD = 32;
#[cfg(feature = "shared")]
pub const WSE_GetStatusResponse: super::DWORD = 33;
#[cfg(feature = "shared")]
pub const WSE_Identifier: super::DWORD = 0;
#[cfg(feature = "shared")]
pub const WSE_InvalidExpirationTime: super::DWORD = 2;
#[cfg(feature = "shared")]
pub const WSE_InvalidMessage: super::DWORD = 8;
#[cfg(feature = "shared")]
pub const WSE_Mode: super::DWORD = 16;
#[cfg(feature = "shared")]
pub const WSE_NotifyTo: super::DWORD = 18;
#[cfg(feature = "shared")]
pub const WSE_Push: super::DWORD = 17;
#[cfg(feature = "shared")]
pub const WSE_Reason: super::DWORD = 40;
#[cfg(feature = "shared")]
pub const WSE_Renew: super::DWORD = 30;
#[cfg(feature = "shared")]
pub const WSE_RenewResponse: super::DWORD = 31;
#[cfg(feature = "shared")]
pub const WSE_ReturnValue: super::DWORD = 14;
#[cfg(feature = "shared")]
pub const WSE_Status: super::DWORD = 38;
#[cfg(feature = "shared")]
pub const WSE_String: super::DWORD = 39;
#[cfg(feature = "shared")]
pub const WSE_Subscribe: super::DWORD = 12;
#[cfg(feature = "shared")]
pub const WSE_SubscribeResponse: super::DWORD = 13;
#[cfg(feature = "shared")]
pub const WSE_SubscriptionEnd: super::DWORD = 37;
#[cfg(feature = "shared")]
pub const WSE_SubscriptionManager: super::DWORD = 28;
#[cfg(feature = "shared")]
pub const WSE_SupportedDeliveryMode: super::DWORD = 10;
#[cfg(feature = "shared")]
pub const WSE_SupportedDialect: super::DWORD = 9;
#[cfg(feature = "shared")]
pub const WSE_UnableToRenew: super::DWORD = 7;
#[cfg(feature = "shared")]
pub const WSE_Unsubscribe: super::DWORD = 34;
#[cfg(feature = "shared")]
pub const WSE_UnsubscribeResponse: super::DWORD = 35;
#[cfg(feature = "shared")]
pub const WSE_UnsupportedExpirationTime: super::DWORD = 3;
#[cfg(feature = "shared")]
pub const WSX_Data: super::DWORD = 393223;
#[cfg(feature = "shared")]
pub const WSX_Dialect: super::DWORD = 393221;
#[cfg(feature = "shared")]
pub const WSX_Element: super::DWORD = 393226;
#[cfg(feature = "shared")]
pub const WSX_GetMetadata: super::DWORD = 393217;
#[cfg(feature = "shared")]
pub const WSX_GetMetadataResponse: super::DWORD = 393218;
#[cfg(feature = "shared")]
pub const WSX_Identifier: super::DWORD = 393222;
#[cfg(feature = "shared")]
pub const WSX_Location: super::DWORD = 393225;
#[cfg(feature = "shared")]
pub const WSX_Metadata: super::DWORD = 393227;
#[cfg(feature = "shared")]
pub const WSX_MetadataReference: super::DWORD = 393224;
#[cfg(feature = "shared")]
pub const WSX_MetadataSection: super::DWORD = 393220;
#[cfg(feature = "shared")]
pub const WSX_Mex: super::DWORD = 393216;
#[cfg(feature = "shared")]
pub const WSX_ReturnValue: super::DWORD = 393219;
#[cfg(feature = "shared")]
pub const XML_Lang: super::DWORD = 262144;
#[cfg(feature = "shared")]
pub const XML_Space: super::DWORD = 262145;
#[cfg(feature = "shared")]
pub const XOP_Href: super::DWORD = 196609;
#[cfg(feature = "shared")]
pub const XOP_Include: super::DWORD = 196608;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSDXML_ATTRIBUTE {
    pub Element: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_ATTRIBUTE,
    pub Name: *mut WSDXML_NAME,
    pub Value: *mut super::WCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSDXML_ELEMENT {
    pub Node: WSDXML_NODE,
    pub Name: *mut WSDXML_NAME,
    pub FirstAttribute: *mut WSDXML_ATTRIBUTE,
    pub FirstChild: *mut WSDXML_NODE,
    pub PrefixMappings: *mut WSDXML_PREFIX_MAPPING,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSDXML_ELEMENT_LIST {
    pub Next: *mut WSDXML_ELEMENT_LIST,
    pub Element: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSDXML_NAME {
    pub Space: *mut WSDXML_NAMESPACE,
    pub LocalName: *mut super::WCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSDXML_NAMESPACE {
    pub Uri: *const super::WCHAR,
    pub PreferredPrefix: *const super::WCHAR,
    pub Names: *mut WSDXML_NAME,
    pub NamesCount: super::WORD,
    pub Encoding: super::WORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSDXML_NODE {
    pub Type: i32,
    pub Parent: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_NODE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl _WSDXML_NODE {
    pub const ElementType: i32 = 0;
    pub const TextType: i32 = 1;
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSDXML_PREFIX_MAPPING {
    pub Refs: super::DWORD,
    pub Next: *mut WSDXML_PREFIX_MAPPING,
    pub Space: *mut WSDXML_NAMESPACE,
    pub Prefix: *mut super::WCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSDXML_TEXT {
    pub Node: WSDXML_NODE,
    pub Text: *mut super::WCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSDXML_TYPE {
    pub Uri: *const super::WCHAR,
    pub Table: *const super::BYTE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_APP_SEQUENCE {
    pub InstanceId: super::ULONGLONG,
    pub SequenceId: *const super::WCHAR,
    pub MessageNumber: super::ULONGLONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_BYE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_ENDPOINT_REFERENCE {
    pub Address: *const super::WCHAR,
    pub ReferenceProperties: WSD_REFERENCE_PROPERTIES,
    pub ReferenceParameters: WSD_REFERENCE_PARAMETERS,
    pub PortType: *mut WSDXML_NAME,
    pub ServiceName: *mut WSDXML_NAME,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_ENDPOINT_REFERENCE_LIST {
    pub Next: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Element: *mut WSD_ENDPOINT_REFERENCE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Debug, Default)]
pub struct _WSD_EVENT {
    pub Hr: windows_core::HRESULT,
    pub EventType: super::DWORD,
    pub DispatchTag: *mut super::WCHAR,
    pub HandlerContext: WSD_HANDLER_CONTEXT,
    pub Soap: *mut WSD_SOAP_MESSAGE,
    pub Operation: *mut WSD_OPERATION,
    pub MessageParameters: core::mem::ManuallyDrop<Option<IWSDMessageParameters>>,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_EVENTING_DELIVERY_MODE {
    pub Mode: *const super::WCHAR,
    pub Push: *mut WSD_EVENTING_DELIVERY_MODE_PUSH,
    pub Data: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_EVENTING_DELIVERY_MODE_PUSH {
    pub NotifyTo: *mut WSD_ENDPOINT_REFERENCE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_EVENTING_EXPIRES {
    pub Duration: *mut WSD_DURATION,
    pub DateTime: *mut WSD_DATETIME,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_EVENTING_FILTER {
    pub Dialect: *const super::WCHAR,
    pub FilterAction: *mut WSD_EVENTING_FILTER_ACTION,
    pub Data: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_EVENTING_FILTER_ACTION {
    pub Actions: *mut WSD_URI_LIST,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Debug, Default)]
pub struct _WSD_HANDLER_CONTEXT {
    pub Handler: PWSD_SOAP_MESSAGE_HANDLER,
    pub PVoid: *mut core::ffi::c_void,
    pub Unknown: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_HEADER_RELATESTO {
    pub RelationshipType: *mut WSDXML_NAME,
    pub MessageID: *const super::WCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_HELLO {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: super::ULONGLONG,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_HOST_METADATA {
    pub Host: *mut WSD_SERVICE_METADATA,
    pub Hosted: *mut WSD_SERVICE_METADATA_LIST,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_LOCALIZED_STRING {
    pub lang: *const super::WCHAR,
    pub String: *const super::WCHAR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_LOCALIZED_STRING_LIST {
    pub Next: *mut WSD_LOCALIZED_STRING_LIST,
    pub Element: *mut WSD_LOCALIZED_STRING,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_METADATA_SECTION {
    pub Dialect: *const super::WCHAR,
    pub Identifier: *const super::WCHAR,
    pub Data: *mut core::ffi::c_void,
    pub MetadataReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Location: *const super::WCHAR,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_METADATA_SECTION_LIST {
    pub Next: *mut WSD_METADATA_SECTION_LIST,
    pub Element: *mut WSD_METADATA_SECTION,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_NAME_LIST {
    pub Next: *mut WSD_NAME_LIST,
    pub Element: *mut WSDXML_NAME,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_PROBE {
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_PROBE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: super::ULONGLONG,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_PROBE_MATCHES {
    pub ProbeMatch: *mut WSD_PROBE_MATCH_LIST,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_PROBE_MATCH_LIST {
    pub Next: *mut WSD_PROBE_MATCH_LIST,
    pub Element: *mut WSD_PROBE_MATCH,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_REFERENCE_PARAMETERS {
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_REFERENCE_PROPERTIES {
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_RELATIONSHIP_METADATA {
    pub Type: *const super::WCHAR,
    pub Data: *mut WSD_HOST_METADATA,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_RESOLVE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_RESOLVE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: super::ULONGLONG,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_RESOLVE_MATCHES {
    pub ResolveMatch: *mut WSD_RESOLVE_MATCH,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_SCOPES {
    pub MatchBy: *const super::WCHAR,
    pub Scopes: *mut WSD_URI_LIST,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_SERVICE_METADATA {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Types: *mut WSD_NAME_LIST,
    pub ServiceId: *const super::WCHAR,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_SERVICE_METADATA_LIST {
    pub Next: *mut WSD_SERVICE_METADATA_LIST,
    pub Element: *mut WSD_SERVICE_METADATA,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_SOAP_FAULT {
    pub Code: *mut WSD_SOAP_FAULT_CODE,
    pub Reason: *mut WSD_SOAP_FAULT_REASON,
    pub Node: *const super::WCHAR,
    pub Role: *const super::WCHAR,
    pub Detail: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_SOAP_FAULT_CODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_SOAP_FAULT_REASON {
    pub Text: *mut WSD_LOCALIZED_STRING_LIST,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_SOAP_FAULT_SUBCODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_SOAP_HEADER {
    pub To: *const super::WCHAR,
    pub Action: *const super::WCHAR,
    pub MessageID: *const super::WCHAR,
    pub RelatesTo: WSD_HEADER_RELATESTO,
    pub ReplyTo: *mut WSD_ENDPOINT_REFERENCE,
    pub From: *mut WSD_ENDPOINT_REFERENCE,
    pub FaultTo: *mut WSD_ENDPOINT_REFERENCE,
    pub AppSequence: *mut WSD_APP_SEQUENCE,
    pub AnyHeaders: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_SOAP_MESSAGE {
    pub Header: WSD_SOAP_HEADER,
    pub Body: *mut core::ffi::c_void,
    pub BodyType: *mut WSDXML_TYPE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_THIS_DEVICE_METADATA {
    pub FriendlyName: *mut WSD_LOCALIZED_STRING_LIST,
    pub FirmwareVersion: *const super::WCHAR,
    pub SerialNumber: *const super::WCHAR,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_THIS_MODEL_METADATA {
    pub Manufacturer: *mut WSD_LOCALIZED_STRING_LIST,
    pub ManufacturerUrl: *const super::WCHAR,
    pub ModelName: *mut WSD_LOCALIZED_STRING_LIST,
    pub ModelNumber: *const super::WCHAR,
    pub ModelUrl: *const super::WCHAR,
    pub PresentationUrl: *const super::WCHAR,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_URI_LIST {
    pub Next: *mut WSD_URI_LIST,
    pub Element: *const super::WCHAR,
}
