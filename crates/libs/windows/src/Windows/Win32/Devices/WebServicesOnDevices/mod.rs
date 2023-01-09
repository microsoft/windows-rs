#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDAllocateLinkedMemory(pparent: *mut ::core::ffi::c_void, cbsize: usize) -> *mut ::core::ffi::c_void {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDAllocateLinkedMemory ( pparent : *mut ::core::ffi::c_void , cbsize : usize ) -> *mut ::core::ffi::c_void );
    WSDAllocateLinkedMemory(pparent, cbsize)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDAttachLinkedMemory(pparent: *mut ::core::ffi::c_void, pchild: *mut ::core::ffi::c_void) {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDAttachLinkedMemory ( pparent : *mut ::core::ffi::c_void , pchild : *mut ::core::ffi::c_void ) -> ( ) );
    WSDAttachLinkedMemory(pparent, pchild)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDeviceHost<P0, P1>(pszlocalid: P0, pcontext: P1) -> ::windows::core::Result<IWSDDeviceHost>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<IWSDXMLContext>>,
{
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDCreateDeviceHost ( pszlocalid : :: windows::core::PCWSTR , pcontext : * mut::core::ffi::c_void , ppdevicehost : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDeviceHost(pszlocalid.into().abi(), pcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDeviceHost2<P0, P1>(pszlocalid: P0, pcontext: P1, pconfigparams: ::core::option::Option<&[WSD_CONFIG_PARAM]>) -> ::windows::core::Result<IWSDDeviceHost>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<IWSDXMLContext>>,
{
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDCreateDeviceHost2 ( pszlocalid : :: windows::core::PCWSTR , pcontext : * mut::core::ffi::c_void , pconfigparams : *const WSD_CONFIG_PARAM , dwconfigparamcount : u32 , ppdevicehost : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDeviceHost2(pszlocalid.into().abi(), pcontext.into().abi(), ::core::mem::transmute(pconfigparams.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pconfigparams.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDeviceHostAdvanced<P0, P1>(pszlocalid: P0, pcontext: P1, pphostaddresses: ::core::option::Option<&[IWSDAddress]>) -> ::windows::core::Result<IWSDDeviceHost>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<IWSDXMLContext>>,
{
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDCreateDeviceHostAdvanced ( pszlocalid : :: windows::core::PCWSTR , pcontext : * mut::core::ffi::c_void , pphostaddresses : *const * mut::core::ffi::c_void , dwhostaddresscount : u32 , ppdevicehost : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDeviceHostAdvanced(pszlocalid.into().abi(), pcontext.into().abi(), ::core::mem::transmute(pphostaddresses.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pphostaddresses.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDeviceProxy<P0, P1, P2>(pszdeviceid: P0, pszlocalid: P1, pcontext: P2) -> ::windows::core::Result<IWSDDeviceProxy>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<IWSDXMLContext>>,
{
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDCreateDeviceProxy ( pszdeviceid : :: windows::core::PCWSTR , pszlocalid : :: windows::core::PCWSTR , pcontext : * mut::core::ffi::c_void , ppdeviceproxy : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDeviceProxy(pszdeviceid.into().abi(), pszlocalid.into().abi(), pcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDeviceProxy2<P0, P1, P2>(pszdeviceid: P0, pszlocalid: P1, pcontext: P2, pconfigparams: ::core::option::Option<&[WSD_CONFIG_PARAM]>) -> ::windows::core::Result<IWSDDeviceProxy>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<IWSDXMLContext>>,
{
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDCreateDeviceProxy2 ( pszdeviceid : :: windows::core::PCWSTR , pszlocalid : :: windows::core::PCWSTR , pcontext : * mut::core::ffi::c_void , pconfigparams : *const WSD_CONFIG_PARAM , dwconfigparamcount : u32 , ppdeviceproxy : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDeviceProxy2(pszdeviceid.into().abi(), pszlocalid.into().abi(), pcontext.into().abi(), ::core::mem::transmute(pconfigparams.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pconfigparams.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDeviceProxyAdvanced<P0, P1, P2, P3>(pszdeviceid: P0, pdeviceaddress: P1, pszlocalid: P2, pcontext: P3) -> ::windows::core::Result<IWSDDeviceProxy>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<IWSDAddress>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<IWSDXMLContext>>,
{
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDCreateDeviceProxyAdvanced ( pszdeviceid : :: windows::core::PCWSTR , pdeviceaddress : * mut::core::ffi::c_void , pszlocalid : :: windows::core::PCWSTR , pcontext : * mut::core::ffi::c_void , ppdeviceproxy : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDeviceProxyAdvanced(pszdeviceid.into().abi(), pdeviceaddress.into().abi(), pszlocalid.into().abi(), pcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDiscoveryProvider<P0>(pcontext: P0) -> ::windows::core::Result<IWSDiscoveryProvider>
where
    P0: ::std::convert::Into<::windows::core::InParam<IWSDXMLContext>>,
{
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDCreateDiscoveryProvider ( pcontext : * mut::core::ffi::c_void , ppprovider : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDiscoveryProvider(pcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDiscoveryProvider2<P0>(pcontext: P0, pconfigparams: ::core::option::Option<&[WSD_CONFIG_PARAM]>) -> ::windows::core::Result<IWSDiscoveryProvider>
where
    P0: ::std::convert::Into<::windows::core::InParam<IWSDXMLContext>>,
{
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDCreateDiscoveryProvider2 ( pcontext : * mut::core::ffi::c_void , pconfigparams : *const WSD_CONFIG_PARAM , dwconfigparamcount : u32 , ppprovider : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDiscoveryProvider2(pcontext.into().abi(), ::core::mem::transmute(pconfigparams.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pconfigparams.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDiscoveryPublisher<P0>(pcontext: P0) -> ::windows::core::Result<IWSDiscoveryPublisher>
where
    P0: ::std::convert::Into<::windows::core::InParam<IWSDXMLContext>>,
{
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDCreateDiscoveryPublisher ( pcontext : * mut::core::ffi::c_void , pppublisher : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDiscoveryPublisher(pcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateDiscoveryPublisher2<P0>(pcontext: P0, pconfigparams: ::core::option::Option<&[WSD_CONFIG_PARAM]>) -> ::windows::core::Result<IWSDiscoveryPublisher>
where
    P0: ::std::convert::Into<::windows::core::InParam<IWSDXMLContext>>,
{
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDCreateDiscoveryPublisher2 ( pcontext : * mut::core::ffi::c_void , pconfigparams : *const WSD_CONFIG_PARAM , dwconfigparamcount : u32 , pppublisher : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateDiscoveryPublisher2(pcontext.into().abi(), ::core::mem::transmute(pconfigparams.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pconfigparams.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateHttpAddress() -> ::windows::core::Result<IWSDHttpAddress> {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDCreateHttpAddress ( ppaddress : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateHttpAddress(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateHttpMessageParameters() -> ::windows::core::Result<IWSDHttpMessageParameters> {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDCreateHttpMessageParameters ( pptxparams : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateHttpMessageParameters(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateOutboundAttachment() -> ::windows::core::Result<IWSDOutboundAttachment> {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDCreateOutboundAttachment ( ppattachment : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateOutboundAttachment(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateUdpAddress() -> ::windows::core::Result<IWSDUdpAddress> {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDCreateUdpAddress ( ppaddress : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateUdpAddress(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDCreateUdpMessageParameters() -> ::windows::core::Result<IWSDUdpMessageParameters> {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDCreateUdpMessageParameters ( pptxparams : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDCreateUdpMessageParameters(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDDetachLinkedMemory(pvoid: *mut ::core::ffi::c_void) {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDDetachLinkedMemory ( pvoid : *mut ::core::ffi::c_void ) -> ( ) );
    WSDDetachLinkedMemory(pvoid)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDFreeLinkedMemory(pvoid: *mut ::core::ffi::c_void) {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDFreeLinkedMemory ( pvoid : *mut ::core::ffi::c_void ) -> ( ) );
    WSDFreeLinkedMemory(pvoid)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDGenerateFault<P0, P1, P2, P3, P4>(pszcode: P0, pszsubcode: P1, pszreason: P2, pszdetail: P3, pcontext: P4) -> ::windows::core::Result<*mut WSD_SOAP_FAULT>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P4: ::std::convert::Into<::windows::core::InParam<IWSDXMLContext>>,
{
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDGenerateFault ( pszcode : :: windows::core::PCWSTR , pszsubcode : :: windows::core::PCWSTR , pszreason : :: windows::core::PCWSTR , pszdetail : :: windows::core::PCWSTR , pcontext : * mut::core::ffi::c_void , ppfault : *mut *mut WSD_SOAP_FAULT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDGenerateFault(pszcode.into().abi(), pszsubcode.into().abi(), pszreason.into().abi(), pszdetail.into().abi(), pcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDGenerateFaultEx<P0>(pcode: *const WSDXML_NAME, psubcode: ::core::option::Option<*const WSDXML_NAME>, preasons: *const WSD_LOCALIZED_STRING_LIST, pszdetail: P0) -> ::windows::core::Result<*mut WSD_SOAP_FAULT>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDGenerateFaultEx ( pcode : *const WSDXML_NAME , psubcode : *const WSDXML_NAME , preasons : *const WSD_LOCALIZED_STRING_LIST , pszdetail : :: windows::core::PCWSTR , ppfault : *mut *mut WSD_SOAP_FAULT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDGenerateFaultEx(pcode, ::core::mem::transmute(psubcode.unwrap_or(::std::ptr::null())), preasons, pszdetail.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDGetConfigurationOption(dwoption: u32, pvoid: *mut ::core::ffi::c_void, cboutbuffer: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDGetConfigurationOption ( dwoption : u32 , pvoid : *mut ::core::ffi::c_void , cboutbuffer : u32 ) -> :: windows::core::HRESULT );
    WSDGetConfigurationOption(dwoption, pvoid, cboutbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDSetConfigurationOption(dwoption: u32, pvoid: *const ::core::ffi::c_void, cbinbuffer: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDSetConfigurationOption ( dwoption : u32 , pvoid : *const ::core::ffi::c_void , cbinbuffer : u32 ) -> :: windows::core::HRESULT );
    WSDSetConfigurationOption(dwoption, pvoid, cbinbuffer).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDUriDecode(source: &[u16], destout: *mut ::windows::core::PWSTR, cchdestout: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDUriDecode ( source : :: windows::core::PCWSTR , cchsource : u32 , destout : *mut :: windows::core::PWSTR , cchdestout : *mut u32 ) -> :: windows::core::HRESULT );
    WSDUriDecode(::core::mem::transmute(source.as_ptr()), source.len() as _, destout, ::core::mem::transmute(cchdestout.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDUriEncode(source: &[u16], destout: *mut ::windows::core::PWSTR, cchdestout: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDUriEncode ( source : :: windows::core::PCWSTR , cchsource : u32 , destout : *mut :: windows::core::PWSTR , cchdestout : *mut u32 ) -> :: windows::core::HRESULT );
    WSDUriEncode(::core::mem::transmute(source.as_ptr()), source.len() as _, destout, ::core::mem::transmute(cchdestout.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDXMLAddChild(pparent: *mut WSDXML_ELEMENT, pchild: *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDXMLAddChild ( pparent : *mut WSDXML_ELEMENT , pchild : *mut WSDXML_ELEMENT ) -> :: windows::core::HRESULT );
    WSDXMLAddChild(pparent, pchild).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDXMLAddSibling(pfirst: *mut WSDXML_ELEMENT, psecond: *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDXMLAddSibling ( pfirst : *mut WSDXML_ELEMENT , psecond : *mut WSDXML_ELEMENT ) -> :: windows::core::HRESULT );
    WSDXMLAddSibling(pfirst, psecond).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDXMLBuildAnyForSingleElement<P0>(pelementname: *mut WSDXML_NAME, psztext: P0, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDXMLBuildAnyForSingleElement ( pelementname : *mut WSDXML_NAME , psztext : :: windows::core::PCWSTR , ppany : *mut *mut WSDXML_ELEMENT ) -> :: windows::core::HRESULT );
    WSDXMLBuildAnyForSingleElement(pelementname, psztext.into().abi(), ppany).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDXMLCleanupElement(pany: *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDXMLCleanupElement ( pany : *mut WSDXML_ELEMENT ) -> :: windows::core::HRESULT );
    WSDXMLCleanupElement(pany).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDXMLCreateContext() -> ::windows::core::Result<IWSDXMLContext> {
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDXMLCreateContext ( ppcontext : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDXMLCreateContext(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDXMLGetNameFromBuiltinNamespace<P0, P1>(psznamespace: P0, pszname: P1) -> ::windows::core::Result<*mut WSDXML_NAME>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDXMLGetNameFromBuiltinNamespace ( psznamespace : :: windows::core::PCWSTR , pszname : :: windows::core::PCWSTR , ppname : *mut *mut WSDXML_NAME ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WSDXMLGetNameFromBuiltinNamespace(psznamespace.into().abi(), pszname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[inline]
pub unsafe fn WSDXMLGetValueFromAny<P0, P1>(psznamespace: P0, pszname: P1, pany: *mut WSDXML_ELEMENT, ppszvalue: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "wsdapi.dll""system" fn WSDXMLGetValueFromAny ( psznamespace : :: windows::core::PCWSTR , pszname : :: windows::core::PCWSTR , pany : *mut WSDXML_ELEMENT , ppszvalue : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    WSDXMLGetValueFromAny(psznamespace.into().abi(), pszname.into().abi(), pany, ppszvalue).ok()
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDAddress(::windows::core::IUnknown);
impl IWSDAddress {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Serialize<P0>(&self, pszbuffer: &mut [u16], fsafe: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Serialize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len() as _, fsafe.into()).ok()
    }
    pub unsafe fn Deserialize<P0>(&self, pszbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Deserialize)(::windows::core::Vtable::as_raw(self), pszbuffer.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IWSDAddress, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDAddress {
    type Vtable = IWSDAddress_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDAddress {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9574c6c_12a6_4f74_93a1_3318ff605759);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAddress_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszbuffer: ::windows::core::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Serialize: usize,
    pub Deserialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszbuffer: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDAsyncCallback(::windows::core::IUnknown);
impl IWSDAsyncCallback {
    pub unsafe fn AsyncOperationComplete<P0, P1>(&self, pasyncresult: P0, pasyncstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDAsyncResult>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).AsyncOperationComplete)(::windows::core::Vtable::as_raw(self), pasyncresult.into().abi(), pasyncstate.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IWSDAsyncCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDAsyncCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDAsyncCallback {
    type Vtable = IWSDAsyncCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDAsyncCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa63e109d_ce72_49e2_ba98_e845f5ee1666);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAsyncCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AsyncOperationComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pasyncstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDAsyncResult(::windows::core::IUnknown);
impl IWSDAsyncResult {
    pub unsafe fn SetCallback<P0, P1>(&self, pcallback: P0, pasyncstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDAsyncCallback>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetCallback)(::windows::core::Vtable::as_raw(self), pcallback.into().abi(), pasyncstate.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWaitHandle<P0>(&self, hwaithandle: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).SetWaitHandle)(::windows::core::Vtable::as_raw(self), hwaithandle.into()).ok()
    }
    pub unsafe fn HasCompleted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).HasCompleted)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetAsyncState(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAsyncState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Abort)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetEvent(&self, pevent: *mut WSD_EVENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetEvent)(::windows::core::Vtable::as_raw(self), pevent).ok()
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows::core::Result<IWSDEndpointProxy> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEndpointProxy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWSDAsyncResult, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDAsyncResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDAsyncResult {
    type Vtable = IWSDAsyncResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDAsyncResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11a9852a_8dd8_423e_b537_9356db4fbfb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAsyncResult_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pasyncstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWaitHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwaithandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWaitHandle: usize,
    pub HasCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAsyncState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppasyncstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevent: *mut WSD_EVENT) -> ::windows::core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppendpoint: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDAttachment(::windows::core::IUnknown);
impl IWSDAttachment {}
::windows::core::interface_hierarchy!(IWSDAttachment, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDAttachment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDAttachment {
    type Vtable = IWSDAttachment_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDAttachment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d55a616_9df8_4b09_b156_9ba351a48b76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAttachment_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDDeviceHost(::windows::core::IUnknown);
impl IWSDDeviceHost {
    pub unsafe fn Init<P0, P1>(&self, pszlocalid: P0, pcontext: P1, pphostaddresses: ::core::option::Option<&[IWSDAddress]>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWSDXMLContext>>,
    {
        (::windows::core::Vtable::vtable(self).Init)(::windows::core::Vtable::as_raw(self), pszlocalid.into().abi(), pcontext.into().abi(), ::core::mem::transmute(pphostaddresses.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pphostaddresses.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn Start<P0>(&self, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDDeviceHostNotify>>,
    {
        (::windows::core::Vtable::vtable(self).Start)(::windows::core::Vtable::as_raw(self), ullinstanceid, pscopelist, pnotificationsink.into().abi()).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Terminate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RegisterPortType(&self, pporttype: *const WSD_PORT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RegisterPortType)(::windows::core::Vtable::as_raw(self), pporttype).ok()
    }
    pub unsafe fn SetMetadata(&self, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: ::core::option::Option<*const WSD_HOST_METADATA>, pcustommetadata: ::core::option::Option<*const WSD_METADATA_SECTION_LIST>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMetadata)(::windows::core::Vtable::as_raw(self), pthismodelmetadata, pthisdevicemetadata, ::core::mem::transmute(phostmetadata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pcustommetadata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn RegisterService<P0, P1>(&self, pszserviceid: P0, pservice: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).RegisterService)(::windows::core::Vtable::as_raw(self), pszserviceid.into().abi(), pservice.into().abi()).ok()
    }
    pub unsafe fn RetireService<P0>(&self, pszserviceid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).RetireService)(::windows::core::Vtable::as_raw(self), pszserviceid.into().abi()).ok()
    }
    pub unsafe fn AddDynamicService<P0, P1, P2>(&self, pszserviceid: P0, pszendpointaddress: P1, pporttype: ::core::option::Option<*const WSD_PORT_TYPE>, pportname: ::core::option::Option<*const WSDXML_NAME>, pany: ::core::option::Option<*const WSDXML_ELEMENT>, pservice: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).AddDynamicService)(::windows::core::Vtable::as_raw(self), pszserviceid.into().abi(), pszendpointaddress.into().abi(), ::core::mem::transmute(pporttype.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pportname.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())), pservice.into().abi()).ok()
    }
    pub unsafe fn RemoveDynamicService<P0>(&self, pszserviceid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveDynamicService)(::windows::core::Vtable::as_raw(self), pszserviceid.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceDiscoverable<P0, P1>(&self, pszserviceid: P0, fdiscoverable: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetServiceDiscoverable)(::windows::core::Vtable::as_raw(self), pszserviceid.into().abi(), fdiscoverable.into()).ok()
    }
    pub unsafe fn SignalEvent<P0>(&self, pszserviceid: P0, pbody: ::core::option::Option<*const ::core::ffi::c_void>, poperation: *const WSD_OPERATION) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SignalEvent)(::windows::core::Vtable::as_raw(self), pszserviceid.into().abi(), ::core::mem::transmute(pbody.unwrap_or(::std::ptr::null())), poperation).ok()
    }
}
::windows::core::interface_hierarchy!(IWSDDeviceHost, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDDeviceHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDDeviceHost {
    type Vtable = IWSDDeviceHost_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDDeviceHost {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x917fe891_3d13_4138_9809_934c8abeb12c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHost_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszlocalid: ::windows::core::PCWSTR, pcontext: *mut ::core::ffi::c_void, pphostaddresses: *const *mut ::core::ffi::c_void, dwhostaddresscount: u32) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RegisterPortType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pporttype: *const WSD_PORT_TYPE) -> ::windows::core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT,
    pub RegisterService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows::core::PCWSTR, pservice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RetireService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub AddDynamicService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows::core::PCWSTR, pszendpointaddress: ::windows::core::PCWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveDynamicService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServiceDiscoverable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows::core::PCWSTR, fdiscoverable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServiceDiscoverable: usize,
    pub SignalEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows::core::PCWSTR, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDDeviceHostNotify(::windows::core::IUnknown);
impl IWSDDeviceHostNotify {
    pub unsafe fn GetService<P0>(&self, pszserviceid: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetService)(::windows::core::Vtable::as_raw(self), pszserviceid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWSDDeviceHostNotify, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDDeviceHostNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDDeviceHostNotify {
    type Vtable = IWSDDeviceHostNotify_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDDeviceHostNotify {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5bee9f9_eeda_41fe_96f7_f45e14990fb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHostNotify_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows::core::PCWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDDeviceProxy(::windows::core::IUnknown);
impl IWSDDeviceProxy {
    pub unsafe fn Init<P0, P1, P2, P3, P4>(&self, pszdeviceid: P0, pdeviceaddress: P1, pszlocalid: P2, pcontext: P3, psponsor: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWSDAddress>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<IWSDXMLContext>>,
        P4: ::std::convert::Into<::windows::core::InParam<IWSDDeviceProxy>>,
    {
        (::windows::core::Vtable::vtable(self).Init)(::windows::core::Vtable::as_raw(self), pszdeviceid.into().abi(), pdeviceaddress.into().abi(), pszlocalid.into().abi(), pcontext.into().abi(), psponsor.into().abi()).ok()
    }
    pub unsafe fn BeginGetMetadata(&self) -> ::windows::core::Result<IWSDAsyncResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginGetMetadata)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EndGetMetadata<P0>(&self, presult: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDAsyncResult>>,
    {
        (::windows::core::Vtable::vtable(self).EndGetMetadata)(::windows::core::Vtable::as_raw(self), presult.into().abi()).ok()
    }
    pub unsafe fn GetHostMetadata(&self) -> ::windows::core::Result<*mut WSD_HOST_METADATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHostMetadata)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetThisModelMetadata(&self) -> ::windows::core::Result<*mut WSD_THIS_MODEL_METADATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetThisModelMetadata)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetThisDeviceMetadata(&self) -> ::windows::core::Result<*mut WSD_THIS_DEVICE_METADATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetThisDeviceMetadata)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAllMetadata(&self) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAllMetadata)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetServiceProxyById<P0>(&self, pszserviceid: P0) -> ::windows::core::Result<IWSDServiceProxy>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetServiceProxyById)(::windows::core::Vtable::as_raw(self), pszserviceid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetServiceProxyByType(&self, ptype: *const WSDXML_NAME) -> ::windows::core::Result<IWSDServiceProxy> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetServiceProxyByType)(::windows::core::Vtable::as_raw(self), ptype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows::core::Result<IWSDEndpointProxy> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEndpointProxy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWSDDeviceProxy, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDDeviceProxy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDDeviceProxy {
    type Vtable = IWSDDeviceProxy_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDDeviceProxy {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeee0c031_c578_4c0e_9a3b_973c35f409db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceProxy_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdeviceid: ::windows::core::PCWSTR, pdeviceaddress: *mut ::core::ffi::c_void, pszlocalid: ::windows::core::PCWSTR, pcontext: *mut ::core::ffi::c_void, psponsor: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BeginGetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndGetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetHostMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphostmetadata: *mut *mut WSD_HOST_METADATA) -> ::windows::core::HRESULT,
    pub GetThisModelMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmanufacturermetadata: *mut *mut WSD_THIS_MODEL_METADATA) -> ::windows::core::HRESULT,
    pub GetThisDeviceMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppthisdevicemetadata: *mut *mut WSD_THIS_DEVICE_METADATA) -> ::windows::core::HRESULT,
    pub GetAllMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT,
    pub GetServiceProxyById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows::core::PCWSTR, ppserviceproxy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetServiceProxyByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *const WSDXML_NAME, ppserviceproxy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproxy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDEndpointProxy(::windows::core::IUnknown);
impl IWSDEndpointProxy {
    pub unsafe fn SendOneWayRequest(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SendOneWayRequest)(::windows::core::Vtable::as_raw(self), pbody, poperation).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendTwoWayRequest(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: ::core::option::Option<*const WSD_SYNCHRONOUS_RESPONSE_CONTEXT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SendTwoWayRequest)(::windows::core::Vtable::as_raw(self), pbody, poperation, ::core::mem::transmute(presponsecontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SendTwoWayRequestAsync<P0, P1>(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: P0, pcallback: P1) -> ::windows::core::Result<IWSDAsyncResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWSDAsyncCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SendTwoWayRequestAsync)(::windows::core::Vtable::as_raw(self), pbody, poperation, pasyncstate.into().abi(), pcallback.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AbortAsyncOperation<P0>(&self, pasyncresult: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDAsyncResult>>,
    {
        (::windows::core::Vtable::vtable(self).AbortAsyncOperation)(::windows::core::Vtable::as_raw(self), pasyncresult.into().abi()).ok()
    }
    pub unsafe fn ProcessFault(&self, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ProcessFault)(::windows::core::Vtable::as_raw(self), pfault).ok()
    }
    pub unsafe fn GetErrorInfo(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetErrorInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFaultInfo(&self) -> ::windows::core::Result<*mut WSD_SOAP_FAULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFaultInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWSDEndpointProxy, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDEndpointProxy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDEndpointProxy {
    type Vtable = IWSDEndpointProxy_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDEndpointProxy {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1860d430_b24c_4975_9f90_dbb39baa24ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEndpointProxy_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SendOneWayRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SendTwoWayRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendTwoWayRequest: usize,
    pub SendTwoWayRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, presult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AbortAsyncOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessFault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::HRESULT,
    pub GetErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszerrorinfo: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetFaultInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDEventingStatus(::windows::core::IUnknown);
impl IWSDEventingStatus {
    pub unsafe fn SubscriptionRenewed<P0>(&self, pszsubscriptionaction: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SubscriptionRenewed)(::windows::core::Vtable::as_raw(self), pszsubscriptionaction.into().abi())
    }
    pub unsafe fn SubscriptionRenewalFailed<P0>(&self, pszsubscriptionaction: P0, hr: ::windows::core::HRESULT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SubscriptionRenewalFailed)(::windows::core::Vtable::as_raw(self), pszsubscriptionaction.into().abi(), hr)
    }
    pub unsafe fn SubscriptionEnded<P0>(&self, pszsubscriptionaction: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SubscriptionEnded)(::windows::core::Vtable::as_raw(self), pszsubscriptionaction.into().abi())
    }
}
::windows::core::interface_hierarchy!(IWSDEventingStatus, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDEventingStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDEventingStatus {
    type Vtable = IWSDEventingStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDEventingStatus {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49b17f52_637a_407a_ae99_fbe82a4d38c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEventingStatus_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SubscriptionRenewed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows::core::PCWSTR),
    pub SubscriptionRenewalFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows::core::PCWSTR, hr: ::windows::core::HRESULT),
    pub SubscriptionEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows::core::PCWSTR),
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDHttpAddress(::windows::core::IUnknown);
impl IWSDHttpAddress {
    pub unsafe fn GetSecure(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSecure)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSecure<P0>(&self, fsecure: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetSecure)(::windows::core::Vtable::as_raw(self), fsecure.into()).ok()
    }
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPath<P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetPath)(::windows::core::Vtable::as_raw(self), pszpath.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IWSDHttpAddress, ::windows::core::IUnknown, IWSDAddress, IWSDTransportAddress);
impl ::core::clone::Clone for IWSDHttpAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDHttpAddress {
    type Vtable = IWSDHttpAddress_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDHttpAddress {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd09ac7bd_2a3e_4b85_8605_2737ff3e4ea0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpAddress_Vtbl {
    pub base__: IWSDTransportAddress_Vtbl,
    pub GetSecure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSecure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsecure: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSecure: usize,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDHttpAuthParameters(::windows::core::IUnknown);
impl IWSDHttpAuthParameters {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClientAccessToken(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetClientAccessToken)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAuthType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAuthType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWSDHttpAuthParameters, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDHttpAuthParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDHttpAuthParameters {
    type Vtable = IWSDHttpAuthParameters_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDHttpAuthParameters {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b476df0_8dac_480d_b05c_99781a5884aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpAuthParameters_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetClientAccessToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetClientAccessToken: usize,
    pub GetAuthType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauthtype: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDHttpMessageParameters(::windows::core::IUnknown);
impl IWSDHttpMessageParameters {
    pub unsafe fn SetInboundHttpHeaders<P0>(&self, pszheaders: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetInboundHttpHeaders)(::windows::core::Vtable::as_raw(self), pszheaders.into().abi()).ok()
    }
    pub unsafe fn GetInboundHttpHeaders(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetInboundHttpHeaders)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOutboundHttpHeaders<P0>(&self, pszheaders: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetOutboundHttpHeaders)(::windows::core::Vtable::as_raw(self), pszheaders.into().abi()).ok()
    }
    pub unsafe fn GetOutboundHttpHeaders(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOutboundHttpHeaders)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetID<P0>(&self, pszid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetID)(::windows::core::Vtable::as_raw(self), pszid.into().abi()).ok()
    }
    pub unsafe fn GetID(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetContext<P0>(&self, pcontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetContext)(::windows::core::Vtable::as_raw(self), pcontext.into().abi()).ok()
    }
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Clear)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IWSDHttpMessageParameters, ::windows::core::IUnknown, IWSDMessageParameters);
impl ::core::clone::Clone for IWSDHttpMessageParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDHttpMessageParameters {
    type Vtable = IWSDHttpMessageParameters_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDHttpMessageParameters {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x540bd122_5c83_4dec_b396_ea62a2697fdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpMessageParameters_Vtbl {
    pub base__: IWSDMessageParameters_Vtbl,
    pub SetInboundHttpHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszheaders: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetInboundHttpHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszheaders: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetOutboundHttpHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszheaders: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetOutboundHttpHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszheaders: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDInboundAttachment(::windows::core::IUnknown);
impl IWSDInboundAttachment {
    pub unsafe fn Read(&self, pbuffer: &mut [u8], pdwnumberofbytesread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Read)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len() as _, pdwnumberofbytesread).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IWSDInboundAttachment, ::windows::core::IUnknown, IWSDAttachment);
impl ::core::clone::Clone for IWSDInboundAttachment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDInboundAttachment {
    type Vtable = IWSDInboundAttachment_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDInboundAttachment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bd6ca65_233c_4fb8_9f7a_2641619655c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDInboundAttachment_Vtbl {
    pub base__: IWSDAttachment_Vtbl,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDMessageParameters(::windows::core::IUnknown);
impl IWSDMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLocalAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLocalAddress<P0>(&self, paddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDAddress>>,
    {
        (::windows::core::Vtable::vtable(self).SetLocalAddress)(::windows::core::Vtable::as_raw(self), paddress.into().abi()).ok()
    }
    pub unsafe fn GetRemoteAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRemoteAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRemoteAddress<P0>(&self, paddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDAddress>>,
    {
        (::windows::core::Vtable::vtable(self).SetRemoteAddress)(::windows::core::Vtable::as_raw(self), paddress.into().abi()).ok()
    }
    pub unsafe fn GetLowerParameters(&self) -> ::windows::core::Result<IWSDMessageParameters> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLowerParameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWSDMessageParameters, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDMessageParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDMessageParameters {
    type Vtable = IWSDMessageParameters_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDMessageParameters {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fafe8a2_e6fc_4b80_b6cf_b7d45c416d7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDMessageParameters_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetLocalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLocalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRemoteAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRemoteAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLowerParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptxparams: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDMetadataExchange(::windows::core::IUnknown);
impl IWSDMetadataExchange {
    pub unsafe fn GetMetadata(&self) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMetadata)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWSDMetadataExchange, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDMetadataExchange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDMetadataExchange {
    type Vtable = IWSDMetadataExchange_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDMetadataExchange {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06996d57_1d67_4928_9307_3d7833fdb846);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDMetadataExchange_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDOutboundAttachment(::windows::core::IUnknown);
impl IWSDOutboundAttachment {
    pub unsafe fn Write(&self, pbuffer: &[u8]) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Write)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Abort)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IWSDOutboundAttachment, ::windows::core::IUnknown, IWSDAttachment);
impl ::core::clone::Clone for IWSDOutboundAttachment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDOutboundAttachment {
    type Vtable = IWSDOutboundAttachment_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDOutboundAttachment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa302f8d_5a22_4ba5_b392_aa8486f4c15d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDOutboundAttachment_Vtbl {
    pub base__: IWSDAttachment_Vtbl,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwbytestowrite: u32, pdwnumberofbyteswritten: *mut u32) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDSSLClientCertificate(::windows::core::IUnknown);
impl IWSDSSLClientCertificate {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn GetClientCertificate(&self) -> ::windows::core::Result<*mut super::super::Security::Cryptography::CERT_CONTEXT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetClientCertificate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMappedAccessToken(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMappedAccessToken)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWSDSSLClientCertificate, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDSSLClientCertificate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDSSLClientCertificate {
    type Vtable = IWSDSSLClientCertificate_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDSSLClientCertificate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde105e87_a0da_418e_98ad_27b9eed87bdc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSSLClientCertificate_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub GetClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    GetClientCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMappedAccessToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMappedAccessToken: usize,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDScopeMatchingRule(::windows::core::IUnknown);
impl IWSDScopeMatchingRule {
    pub unsafe fn GetScopeRule(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetScopeRule)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchScopes<P0, P1>(&self, pszscope1: P0, pszscope2: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MatchScopes)(::windows::core::Vtable::as_raw(self), pszscope1.into().abi(), pszscope2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWSDScopeMatchingRule, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDScopeMatchingRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDScopeMatchingRule {
    type Vtable = IWSDScopeMatchingRule_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDScopeMatchingRule {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcafe424_fef5_481a_bd9f_33ce0574256f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDScopeMatchingRule_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetScopeRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszscopematchingrule: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MatchScopes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszscope1: ::windows::core::PCWSTR, pszscope2: ::windows::core::PCWSTR, pfmatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MatchScopes: usize,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDServiceMessaging(::windows::core::IUnknown);
impl IWSDServiceMessaging {
    pub unsafe fn SendResponse<P0>(&self, pbody: ::core::option::Option<*const ::core::ffi::c_void>, poperation: *const WSD_OPERATION, pmessageparameters: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDMessageParameters>>,
    {
        (::windows::core::Vtable::vtable(self).SendResponse)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbody.unwrap_or(::std::ptr::null())), poperation, pmessageparameters.into().abi()).ok()
    }
    pub unsafe fn FaultRequest<P0>(&self, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: P0, pfault: ::core::option::Option<*const WSD_SOAP_FAULT>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDMessageParameters>>,
    {
        (::windows::core::Vtable::vtable(self).FaultRequest)(::windows::core::Vtable::as_raw(self), prequestheader, pmessageparameters.into().abi(), ::core::mem::transmute(pfault.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::core::interface_hierarchy!(IWSDServiceMessaging, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDServiceMessaging {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDServiceMessaging {
    type Vtable = IWSDServiceMessaging_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDServiceMessaging {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94974cf4_0cab_460d_a3f6_7a0ad623c0e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceMessaging_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SendResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FaultRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: *mut ::core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDServiceProxy(::windows::core::IUnknown);
impl IWSDServiceProxy {
    pub unsafe fn BeginGetMetadata(&self) -> ::windows::core::Result<IWSDAsyncResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginGetMetadata)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EndGetMetadata<P0>(&self, presult: P0) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDAsyncResult>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EndGetMetadata)(::windows::core::Vtable::as_raw(self), presult.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetServiceMetadata(&self) -> ::windows::core::Result<*mut WSD_SERVICE_METADATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetServiceMetadata)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SubscribeToOperation<P0>(&self, poperation: *const WSD_OPERATION, punknown: P0, pany: *const WSDXML_ELEMENT, ppany: ::core::option::Option<*mut *mut WSDXML_ELEMENT>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SubscribeToOperation)(::windows::core::Vtable::as_raw(self), poperation, punknown.into().abi(), pany, ::core::mem::transmute(ppany.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn UnsubscribeToOperation(&self, poperation: *const WSD_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnsubscribeToOperation)(::windows::core::Vtable::as_raw(self), poperation).ok()
    }
    pub unsafe fn SetEventingStatusCallback<P0>(&self, pstatus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDEventingStatus>>,
    {
        (::windows::core::Vtable::vtable(self).SetEventingStatusCallback)(::windows::core::Vtable::as_raw(self), pstatus.into().abi()).ok()
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows::core::Result<IWSDEndpointProxy> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEndpointProxy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWSDServiceProxy, ::windows::core::IUnknown, IWSDMetadataExchange);
impl ::core::clone::Clone for IWSDServiceProxy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDServiceProxy {
    type Vtable = IWSDServiceProxy_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDServiceProxy {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4c7fb9c_03ab_4175_9d67_094fafebf487);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceProxy_Vtbl {
    pub base__: IWSDMetadataExchange_Vtbl,
    pub BeginGetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndGetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut ::core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT,
    pub GetServiceMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppservicemetadata: *mut *mut WSD_SERVICE_METADATA) -> ::windows::core::HRESULT,
    pub SubscribeToOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION, punknown: *mut ::core::ffi::c_void, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    pub UnsubscribeToOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::HRESULT,
    pub SetEventingStatusCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproxy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDServiceProxyEventing(::windows::core::IUnknown);
impl IWSDServiceProxyEventing {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubscribeToMultipleOperations<P0>(&self, poperations: &[WSD_OPERATION], punknown: P0, pexpires: ::core::option::Option<*const WSD_EVENTING_EXPIRES>, pany: ::core::option::Option<*const WSDXML_ELEMENT>, ppexpires: ::core::option::Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: ::core::option::Option<*mut *mut WSDXML_ELEMENT>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SubscribeToMultipleOperations)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len() as _, punknown.into().abi(), ::core::mem::transmute(pexpires.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppexpires.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppany.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginSubscribeToMultipleOperations<P0, P1, P2>(&self, poperations: &[WSD_OPERATION], punknown: P0, pexpires: ::core::option::Option<*const WSD_EVENTING_EXPIRES>, pany: ::core::option::Option<*const WSDXML_ELEMENT>, pasyncstate: P1, pasynccallback: P2) -> ::windows::core::Result<IWSDAsyncResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<IWSDAsyncCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginSubscribeToMultipleOperations)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len() as _, punknown.into().abi(), ::core::mem::transmute(pexpires.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())), pasyncstate.into().abi(), pasynccallback.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndSubscribeToMultipleOperations<P0>(&self, poperations: &[WSD_OPERATION], presult: P0, ppexpires: ::core::option::Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: ::core::option::Option<*mut *mut WSDXML_ELEMENT>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDAsyncResult>>,
    {
        (::windows::core::Vtable::vtable(self).EndSubscribeToMultipleOperations)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len() as _, presult.into().abi(), ::core::mem::transmute(ppexpires.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppany.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn UnsubscribeToMultipleOperations(&self, poperations: &[WSD_OPERATION], pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnsubscribeToMultipleOperations)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len() as _, pany).ok()
    }
    pub unsafe fn BeginUnsubscribeToMultipleOperations<P0, P1>(&self, poperations: &[WSD_OPERATION], pany: ::core::option::Option<*const WSDXML_ELEMENT>, pasyncstate: P0, pasynccallback: P1) -> ::windows::core::Result<IWSDAsyncResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWSDAsyncCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginUnsubscribeToMultipleOperations)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len() as _, ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())), pasyncstate.into().abi(), pasynccallback.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EndUnsubscribeToMultipleOperations<P0>(&self, poperations: &[WSD_OPERATION], presult: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDAsyncResult>>,
    {
        (::windows::core::Vtable::vtable(self).EndUnsubscribeToMultipleOperations)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len() as _, presult.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenewMultipleOperations(&self, poperations: &[WSD_OPERATION], pexpires: ::core::option::Option<*const WSD_EVENTING_EXPIRES>, pany: ::core::option::Option<*const WSDXML_ELEMENT>, ppexpires: ::core::option::Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: ::core::option::Option<*mut *mut WSDXML_ELEMENT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RenewMultipleOperations)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len() as _, ::core::mem::transmute(pexpires.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppexpires.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppany.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginRenewMultipleOperations<P0, P1>(&self, poperations: &[WSD_OPERATION], pexpires: ::core::option::Option<*const WSD_EVENTING_EXPIRES>, pany: ::core::option::Option<*const WSDXML_ELEMENT>, pasyncstate: P0, pasynccallback: P1) -> ::windows::core::Result<IWSDAsyncResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWSDAsyncCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginRenewMultipleOperations)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len() as _, ::core::mem::transmute(pexpires.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())), pasyncstate.into().abi(), pasynccallback.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndRenewMultipleOperations<P0>(&self, poperations: &[WSD_OPERATION], presult: P0, ppexpires: ::core::option::Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: ::core::option::Option<*mut *mut WSDXML_ELEMENT>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDAsyncResult>>,
    {
        (::windows::core::Vtable::vtable(self).EndRenewMultipleOperations)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len() as _, presult.into().abi(), ::core::mem::transmute(ppexpires.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppany.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStatusForMultipleOperations(&self, poperations: &[WSD_OPERATION], pany: ::core::option::Option<*const WSDXML_ELEMENT>, ppexpires: ::core::option::Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: ::core::option::Option<*mut *mut WSDXML_ELEMENT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetStatusForMultipleOperations)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len() as _, ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppexpires.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppany.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn BeginGetStatusForMultipleOperations<P0, P1>(&self, poperations: &[WSD_OPERATION], pany: ::core::option::Option<*const WSDXML_ELEMENT>, pasyncstate: P0, pasynccallback: P1) -> ::windows::core::Result<IWSDAsyncResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWSDAsyncCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BeginGetStatusForMultipleOperations)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len() as _, ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())), pasyncstate.into().abi(), pasynccallback.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndGetStatusForMultipleOperations<P0>(&self, poperations: &[WSD_OPERATION], presult: P0, ppexpires: ::core::option::Option<*mut *mut WSD_EVENTING_EXPIRES>, ppany: ::core::option::Option<*mut *mut WSDXML_ELEMENT>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDAsyncResult>>,
    {
        (::windows::core::Vtable::vtable(self).EndGetStatusForMultipleOperations)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(poperations.as_ptr()), poperations.len() as _, presult.into().abi(), ::core::mem::transmute(ppexpires.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppany.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::core::interface_hierarchy!(IWSDServiceProxyEventing, ::windows::core::IUnknown, IWSDMetadataExchange, IWSDServiceProxy);
impl ::core::clone::Clone for IWSDServiceProxyEventing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDServiceProxyEventing {
    type Vtable = IWSDServiceProxyEventing_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDServiceProxyEventing {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9279d6d_1012_4a94_b8cc_fd35d2202bfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceProxyEventing_Vtbl {
    pub base__: IWSDServiceProxy_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubscribeToMultipleOperations: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginSubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginSubscribeToMultipleOperations: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EndSubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndSubscribeToMultipleOperations: usize,
    pub UnsubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    pub BeginUnsubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndUnsubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RenewMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RenewMultipleOperations: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginRenewMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginRenewMultipleOperations: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EndRenewMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndRenewMultipleOperations: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetStatusForMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetStatusForMultipleOperations: usize,
    pub BeginGetStatusForMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EndGetStatusForMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndGetStatusForMultipleOperations: usize,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDSignatureProperty(::windows::core::IUnknown);
impl IWSDSignatureProperty {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMessageSigned(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsMessageSigned)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMessageSignatureTrusted(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsMessageSignatureTrusted)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetKeyInfo(&self, pbkeyinfo: ::core::option::Option<*mut u8>, pdwkeyinfosize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetKeyInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbkeyinfo.unwrap_or(::std::ptr::null_mut())), pdwkeyinfosize).ok()
    }
    pub unsafe fn GetSignature(&self, pbsignature: ::core::option::Option<*mut u8>, pdwsignaturesize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSignature)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbsignature.unwrap_or(::std::ptr::null_mut())), pdwsignaturesize).ok()
    }
    pub unsafe fn GetSignedInfoHash(&self, pbsignedinfohash: ::core::option::Option<*mut u8>, pdwhashsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSignedInfoHash)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbsignedinfohash.unwrap_or(::std::ptr::null_mut())), pdwhashsize).ok()
    }
}
::windows::core::interface_hierarchy!(IWSDSignatureProperty, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDSignatureProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDSignatureProperty {
    type Vtable = IWSDSignatureProperty_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDSignatureProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03ce20aa_71c4_45e2_b32e_3766c61c790f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSignatureProperty_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMessageSigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsigned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMessageSigned: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMessageSignatureTrusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsignaturetrusted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMessageSignatureTrusted: usize,
    pub GetKeyInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows::core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows::core::HRESULT,
    pub GetSignedInfoHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDTransportAddress(::windows::core::IUnknown);
impl IWSDTransportAddress {
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPort)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPort)(::windows::core::Vtable::as_raw(self), wport).ok()
    }
    pub unsafe fn GetTransportAddress(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTransportAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransportAddressEx<P0>(&self, fsafe: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTransportAddressEx)(::windows::core::Vtable::as_raw(self), fsafe.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransportAddress<P0>(&self, pszaddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetTransportAddress)(::windows::core::Vtable::as_raw(self), pszaddress.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IWSDTransportAddress, ::windows::core::IUnknown, IWSDAddress);
impl ::core::clone::Clone for IWSDTransportAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDTransportAddress {
    type Vtable = IWSDTransportAddress_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDTransportAddress {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70d23498_4ee6_4340_a3df_d845d2235467);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDTransportAddress_Vtbl {
    pub base__: IWSDAddress_Vtbl,
    pub GetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwport: *mut u16) -> ::windows::core::HRESULT,
    pub SetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wport: u16) -> ::windows::core::HRESULT,
    pub GetTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszaddress: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTransportAddressEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsafe: super::super::Foundation::BOOL, ppszaddress: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTransportAddressEx: usize,
    pub SetTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszaddress: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDUdpAddress(::windows::core::IUnknown);
impl IWSDUdpAddress {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub unsafe fn SetSockaddr(&self, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSockaddr)(::windows::core::Vtable::as_raw(self), psockaddr).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub unsafe fn GetSockaddr(&self, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSockaddr)(::windows::core::Vtable::as_raw(self), psockaddr).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExclusive<P0>(&self, fexclusive: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetExclusive)(::windows::core::Vtable::as_raw(self), fexclusive.into()).ok()
    }
    pub unsafe fn GetExclusive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetExclusive)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetMessageType(&self, messagetype: WSDUdpMessageType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMessageType)(::windows::core::Vtable::as_raw(self), messagetype).ok()
    }
    pub unsafe fn GetMessageType(&self) -> ::windows::core::Result<WSDUdpMessageType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMessageType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTTL(&self, dwttl: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTTL)(::windows::core::Vtable::as_raw(self), dwttl).ok()
    }
    pub unsafe fn GetTTL(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTTL)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAlias(&self, palias: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAlias)(::windows::core::Vtable::as_raw(self), palias).ok()
    }
    pub unsafe fn GetAlias(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAlias)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWSDUdpAddress, ::windows::core::IUnknown, IWSDAddress, IWSDTransportAddress);
impl ::core::clone::Clone for IWSDUdpAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDUdpAddress {
    type Vtable = IWSDUdpAddress_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDUdpAddress {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74d6124a_a441_4f78_a1eb_97a8d1996893);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpAddress_Vtbl {
    pub base__: IWSDTransportAddress_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub SetSockaddr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock")))]
    SetSockaddr: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub GetSockaddr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock")))]
    GetSockaddr: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fexclusive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExclusive: usize,
    pub GetExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: WSDUdpMessageType) -> ::windows::core::HRESULT,
    pub GetMessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessagetype: *mut WSDUdpMessageType) -> ::windows::core::HRESULT,
    pub SetTTL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwttl: u32) -> ::windows::core::HRESULT,
    pub GetTTL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwttl: *mut u32) -> ::windows::core::HRESULT,
    pub SetAlias: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, palias: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetAlias: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, palias: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDUdpMessageParameters(::windows::core::IUnknown);
impl IWSDUdpMessageParameters {
    pub unsafe fn SetRetransmitParams(&self, pparams: *const WSDUdpRetransmitParams) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRetransmitParams)(::windows::core::Vtable::as_raw(self), pparams).ok()
    }
    pub unsafe fn GetRetransmitParams(&self, pparams: *mut WSDUdpRetransmitParams) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetRetransmitParams)(::windows::core::Vtable::as_raw(self), pparams).ok()
    }
}
::windows::core::interface_hierarchy!(IWSDUdpMessageParameters, ::windows::core::IUnknown, IWSDMessageParameters);
impl ::core::clone::Clone for IWSDUdpMessageParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDUdpMessageParameters {
    type Vtable = IWSDUdpMessageParameters_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDUdpMessageParameters {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9934149f_8f0c_447b_aa0b_73124b0ca7f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpMessageParameters_Vtbl {
    pub base__: IWSDMessageParameters_Vtbl,
    pub SetRetransmitParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparams: *const WSDUdpRetransmitParams) -> ::windows::core::HRESULT,
    pub GetRetransmitParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparams: *mut WSDUdpRetransmitParams) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDXMLContext(::windows::core::IUnknown);
impl IWSDXMLContext {
    pub unsafe fn AddNamespace<P0, P1>(&self, pszuri: P0, pszsuggestedprefix: P1, ppnamespace: ::core::option::Option<*mut *mut WSDXML_NAMESPACE>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).AddNamespace)(::windows::core::Vtable::as_raw(self), pszuri.into().abi(), pszsuggestedprefix.into().abi(), ::core::mem::transmute(ppnamespace.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn AddNameToNamespace<P0, P1>(&self, pszuri: P0, pszname: P1, ppname: ::core::option::Option<*mut *mut WSDXML_NAME>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).AddNameToNamespace)(::windows::core::Vtable::as_raw(self), pszuri.into().abi(), pszname.into().abi(), ::core::mem::transmute(ppname.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetNamespaces(&self, pnamespaces: &[*const WSDXML_NAMESPACE], blayernumber: u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetNamespaces)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pnamespaces.as_ptr()), pnamespaces.len() as _, blayernumber).ok()
    }
    pub unsafe fn SetTypes(&self, ptypes: &[*const WSDXML_TYPE], blayernumber: u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTypes)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ptypes.as_ptr()), ptypes.len() as _, blayernumber).ok()
    }
}
::windows::core::interface_hierarchy!(IWSDXMLContext, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDXMLContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDXMLContext {
    type Vtable = IWSDXMLContext_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDXMLContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75d8f3ee_3e5a_43b4_a15a_bcf6887460c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDXMLContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszuri: ::windows::core::PCWSTR, pszsuggestedprefix: ::windows::core::PCWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> ::windows::core::HRESULT,
    pub AddNameToNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszuri: ::windows::core::PCWSTR, pszname: ::windows::core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows::core::HRESULT,
    pub SetNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows::core::HRESULT,
    pub SetTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDiscoveredService(::windows::core::IUnknown);
impl IWSDiscoveredService {
    pub unsafe fn GetEndpointReference(&self) -> ::windows::core::Result<*mut WSD_ENDPOINT_REFERENCE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEndpointReference)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTypes(&self) -> ::windows::core::Result<*mut WSD_NAME_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetScopes(&self) -> ::windows::core::Result<*mut WSD_URI_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetScopes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetXAddrs(&self) -> ::windows::core::Result<*mut WSD_URI_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetXAddrs)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMetadataVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMetadataVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetExtendedDiscoXML(&self, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetExtendedDiscoXML)(::windows::core::Vtable::as_raw(self), ppheaderany, ppbodyany).ok()
    }
    pub unsafe fn GetProbeResolveTag(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProbeResolveTag)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRemoteTransportAddress(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRemoteTransportAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLocalTransportAddress(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLocalTransportAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLocalInterfaceGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLocalInterfaceGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInstanceId(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetInstanceId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWSDiscoveredService, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDiscoveredService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDiscoveredService {
    type Vtable = IWSDiscoveredService_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDiscoveredService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bad8a3b_b374_4420_9632_aac945b374aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveredService_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetEndpointReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppendpointreference: *mut *mut WSD_ENDPOINT_REFERENCE) -> ::windows::core::HRESULT,
    pub GetTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptypeslist: *mut *mut WSD_NAME_LIST) -> ::windows::core::HRESULT,
    pub GetScopes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppscopeslist: *mut *mut WSD_URI_LIST) -> ::windows::core::HRESULT,
    pub GetXAddrs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppxaddrslist: *mut *mut WSD_URI_LIST) -> ::windows::core::HRESULT,
    pub GetMetadataVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullmetadataversion: *mut u64) -> ::windows::core::HRESULT,
    pub GetExtendedDiscoXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    pub GetProbeResolveTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsztag: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetRemoteTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszremotetransportaddress: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetLocalTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszlocaltransportaddress: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetLocalInterfaceGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullinstanceid: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDiscoveryProvider(::windows::core::IUnknown);
impl IWSDiscoveryProvider {
    pub unsafe fn SetAddressFamily(&self, dwaddressfamily: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAddressFamily)(::windows::core::Vtable::as_raw(self), dwaddressfamily).ok()
    }
    pub unsafe fn Attach<P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDiscoveryProviderNotify>>,
    {
        (::windows::core::Vtable::vtable(self).Attach)(::windows::core::Vtable::as_raw(self), psink.into().abi()).ok()
    }
    pub unsafe fn Detach(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Detach)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SearchById<P0, P1>(&self, pszid: P0, psztag: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SearchById)(::windows::core::Vtable::as_raw(self), pszid.into().abi(), psztag.into().abi()).ok()
    }
    pub unsafe fn SearchByAddress<P0, P1>(&self, pszaddress: P0, psztag: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SearchByAddress)(::windows::core::Vtable::as_raw(self), pszaddress.into().abi(), psztag.into().abi()).ok()
    }
    pub unsafe fn SearchByType<P0, P1>(&self, ptypeslist: ::core::option::Option<*const WSD_NAME_LIST>, pscopeslist: ::core::option::Option<*const WSD_URI_LIST>, pszmatchby: P0, psztag: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SearchByType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ptypeslist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pscopeslist.unwrap_or(::std::ptr::null())), pszmatchby.into().abi(), psztag.into().abi()).ok()
    }
    pub unsafe fn GetXMLContext(&self) -> ::windows::core::Result<IWSDXMLContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetXMLContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWSDiscoveryProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDiscoveryProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDiscoveryProvider {
    type Vtable = IWSDiscoveryProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDiscoveryProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ffc8e55_f0eb_480f_88b7_b435dd281d45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetAddressFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows::core::HRESULT,
    pub Attach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Detach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SearchById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR, psztag: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SearchByAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszaddress: ::windows::core::PCWSTR, psztag: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SearchByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: ::windows::core::PCWSTR, psztag: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetXMLContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDiscoveryProviderNotify(::windows::core::IUnknown);
impl IWSDiscoveryProviderNotify {
    pub unsafe fn Add<P0>(&self, pservice: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDiscoveredService>>,
    {
        (::windows::core::Vtable::vtable(self).Add)(::windows::core::Vtable::as_raw(self), pservice.into().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, pservice: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDiscoveredService>>,
    {
        (::windows::core::Vtable::vtable(self).Remove)(::windows::core::Vtable::as_raw(self), pservice.into().abi()).ok()
    }
    pub unsafe fn SearchFailed<P0>(&self, hr: ::windows::core::HRESULT, psztag: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SearchFailed)(::windows::core::Vtable::as_raw(self), hr, psztag.into().abi()).ok()
    }
    pub unsafe fn SearchComplete<P0>(&self, psztag: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SearchComplete)(::windows::core::Vtable::as_raw(self), psztag.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IWSDiscoveryProviderNotify, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDiscoveryProviderNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDiscoveryProviderNotify {
    type Vtable = IWSDiscoveryProviderNotify_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDiscoveryProviderNotify {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73ee3ced_b6e6_4329_a546_3e8ad46563d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryProviderNotify_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pservice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pservice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SearchFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, psztag: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SearchComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztag: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDiscoveryPublisher(::windows::core::IUnknown);
impl IWSDiscoveryPublisher {
    pub unsafe fn SetAddressFamily(&self, dwaddressfamily: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAddressFamily)(::windows::core::Vtable::as_raw(self), dwaddressfamily).ok()
    }
    pub unsafe fn RegisterNotificationSink<P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDiscoveryPublisherNotify>>,
    {
        (::windows::core::Vtable::vtable(self).RegisterNotificationSink)(::windows::core::Vtable::as_raw(self), psink.into().abi()).ok()
    }
    pub unsafe fn UnRegisterNotificationSink<P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDiscoveryPublisherNotify>>,
    {
        (::windows::core::Vtable::vtable(self).UnRegisterNotificationSink)(::windows::core::Vtable::as_raw(self), psink.into().abi()).ok()
    }
    pub unsafe fn Publish<P0, P1>(&self, pszid: P0, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P1, ptypeslist: ::core::option::Option<*const WSD_NAME_LIST>, pscopeslist: ::core::option::Option<*const WSD_URI_LIST>, pxaddrslist: ::core::option::Option<*const WSD_URI_LIST>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Publish)(::windows::core::Vtable::as_raw(self), pszid.into().abi(), ullmetadataversion, ullinstanceid, ullmessagenumber, pszsessionid.into().abi(), ::core::mem::transmute(ptypeslist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pscopeslist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pxaddrslist.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn UnPublish<P0, P1>(&self, pszid: P0, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P1, pany: ::core::option::Option<*const WSDXML_ELEMENT>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).UnPublish)(::windows::core::Vtable::as_raw(self), pszid.into().abi(), ullinstanceid, ullmessagenumber, pszsessionid.into().abi(), ::core::mem::transmute(pany.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn MatchProbe<P0, P1, P2>(&self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: P0, pszid: P1, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P2, ptypeslist: ::core::option::Option<*const WSD_NAME_LIST>, pscopeslist: ::core::option::Option<*const WSD_URI_LIST>, pxaddrslist: ::core::option::Option<*const WSD_URI_LIST>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDMessageParameters>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).MatchProbe)(::windows::core::Vtable::as_raw(self), pprobemessage, pmessageparameters.into().abi(), pszid.into().abi(), ullmetadataversion, ullinstanceid, ullmessagenumber, pszsessionid.into().abi(), ::core::mem::transmute(ptypeslist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pscopeslist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pxaddrslist.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn MatchResolve<P0, P1, P2>(&self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: P0, pszid: P1, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P2, ptypeslist: ::core::option::Option<*const WSD_NAME_LIST>, pscopeslist: ::core::option::Option<*const WSD_URI_LIST>, pxaddrslist: ::core::option::Option<*const WSD_URI_LIST>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDMessageParameters>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).MatchResolve)(::windows::core::Vtable::as_raw(self), presolvemessage, pmessageparameters.into().abi(), pszid.into().abi(), ullmetadataversion, ullinstanceid, ullmessagenumber, pszsessionid.into().abi(), ::core::mem::transmute(ptypeslist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pscopeslist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pxaddrslist.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn PublishEx<P0, P1>(&self, pszid: P0, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P1, ptypeslist: ::core::option::Option<*const WSD_NAME_LIST>, pscopeslist: ::core::option::Option<*const WSD_URI_LIST>, pxaddrslist: ::core::option::Option<*const WSD_URI_LIST>, pheaderany: ::core::option::Option<*const WSDXML_ELEMENT>, preferenceparameterany: ::core::option::Option<*const WSDXML_ELEMENT>, ppolicyany: ::core::option::Option<*const WSDXML_ELEMENT>, pendpointreferenceany: ::core::option::Option<*const WSDXML_ELEMENT>, pany: ::core::option::Option<*const WSDXML_ELEMENT>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).PublishEx)(
            ::windows::core::Vtable::as_raw(self),
            pszid.into().abi(),
            ullmetadataversion,
            ullinstanceid,
            ullmessagenumber,
            pszsessionid.into().abi(),
            ::core::mem::transmute(ptypeslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pscopeslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pxaddrslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pheaderany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(preferenceparameterany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(ppolicyany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pendpointreferenceany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())),
        )
        .ok()
    }
    pub unsafe fn MatchProbeEx<P0, P1, P2>(&self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: P0, pszid: P1, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P2, ptypeslist: ::core::option::Option<*const WSD_NAME_LIST>, pscopeslist: ::core::option::Option<*const WSD_URI_LIST>, pxaddrslist: ::core::option::Option<*const WSD_URI_LIST>, pheaderany: ::core::option::Option<*const WSDXML_ELEMENT>, preferenceparameterany: ::core::option::Option<*const WSDXML_ELEMENT>, ppolicyany: ::core::option::Option<*const WSDXML_ELEMENT>, pendpointreferenceany: ::core::option::Option<*const WSDXML_ELEMENT>, pany: ::core::option::Option<*const WSDXML_ELEMENT>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDMessageParameters>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).MatchProbeEx)(
            ::windows::core::Vtable::as_raw(self),
            pprobemessage,
            pmessageparameters.into().abi(),
            pszid.into().abi(),
            ullmetadataversion,
            ullinstanceid,
            ullmessagenumber,
            pszsessionid.into().abi(),
            ::core::mem::transmute(ptypeslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pscopeslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pxaddrslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pheaderany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(preferenceparameterany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(ppolicyany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pendpointreferenceany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())),
        )
        .ok()
    }
    pub unsafe fn MatchResolveEx<P0, P1, P2>(&self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: P0, pszid: P1, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P2, ptypeslist: ::core::option::Option<*const WSD_NAME_LIST>, pscopeslist: ::core::option::Option<*const WSD_URI_LIST>, pxaddrslist: ::core::option::Option<*const WSD_URI_LIST>, pheaderany: ::core::option::Option<*const WSDXML_ELEMENT>, preferenceparameterany: ::core::option::Option<*const WSDXML_ELEMENT>, ppolicyany: ::core::option::Option<*const WSDXML_ELEMENT>, pendpointreferenceany: ::core::option::Option<*const WSDXML_ELEMENT>, pany: ::core::option::Option<*const WSDXML_ELEMENT>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDMessageParameters>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).MatchResolveEx)(
            ::windows::core::Vtable::as_raw(self),
            presolvemessage,
            pmessageparameters.into().abi(),
            pszid.into().abi(),
            ullmetadataversion,
            ullinstanceid,
            ullmessagenumber,
            pszsessionid.into().abi(),
            ::core::mem::transmute(ptypeslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pscopeslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pxaddrslist.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pheaderany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(preferenceparameterany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(ppolicyany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pendpointreferenceany.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pany.unwrap_or(::std::ptr::null())),
        )
        .ok()
    }
    pub unsafe fn RegisterScopeMatchingRule<P0>(&self, pscopematchingrule: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDScopeMatchingRule>>,
    {
        (::windows::core::Vtable::vtable(self).RegisterScopeMatchingRule)(::windows::core::Vtable::as_raw(self), pscopematchingrule.into().abi()).ok()
    }
    pub unsafe fn UnRegisterScopeMatchingRule<P0>(&self, pscopematchingrule: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDScopeMatchingRule>>,
    {
        (::windows::core::Vtable::vtable(self).UnRegisterScopeMatchingRule)(::windows::core::Vtable::as_raw(self), pscopematchingrule.into().abi()).ok()
    }
    pub unsafe fn GetXMLContext(&self) -> ::windows::core::Result<IWSDXMLContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetXMLContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWSDiscoveryPublisher, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDiscoveryPublisher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDiscoveryPublisher {
    type Vtable = IWSDiscoveryPublisher_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDiscoveryPublisher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae01e1a8_3ff9_4148_8116_057cc616fe13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryPublisher_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetAddressFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows::core::HRESULT,
    pub RegisterNotificationSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnRegisterNotificationSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Publish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT,
    pub UnPublish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows::core::PCWSTR, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    pub MatchProbe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT,
    pub MatchResolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT,
    pub PublishEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    pub MatchProbeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    pub MatchResolveEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT,
    pub RegisterScopeMatchingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscopematchingrule: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnRegisterScopeMatchingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscopematchingrule: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetXMLContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
pub struct IWSDiscoveryPublisherNotify(::windows::core::IUnknown);
impl IWSDiscoveryPublisherNotify {
    pub unsafe fn ProbeHandler<P0>(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDMessageParameters>>,
    {
        (::windows::core::Vtable::vtable(self).ProbeHandler)(::windows::core::Vtable::as_raw(self), psoap, pmessageparameters.into().abi()).ok()
    }
    pub unsafe fn ResolveHandler<P0>(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDMessageParameters>>,
    {
        (::windows::core::Vtable::vtable(self).ResolveHandler)(::windows::core::Vtable::as_raw(self), psoap, pmessageparameters.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IWSDiscoveryPublisherNotify, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWSDiscoveryPublisherNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWSDiscoveryPublisherNotify {
    type Vtable = IWSDiscoveryPublisherNotify_Vtbl;
}
unsafe impl ::windows::core::Interface for IWSDiscoveryPublisherNotify {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe67651b0_337a_4b3c_9758_733388568251);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryPublisherNotify_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ProbeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResolveHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_ADDRESSFAMILY_IPV4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_ADDRESSFAMILY_IPV6: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_OPTION_MAX_INBOUND_MESSAGE_SIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_OPTION_TRACE_XML_TO_DEBUGGER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_OPTION_TRACE_XML_TO_FILE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_IGNORE_EXPIRY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_IGNORE_INVALID_CN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_IGNORE_REVOCATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_DEFAULT_EVENTING_ADDRESS: ::windows::core::PCWSTR = ::windows::w!("http://*:5357/");
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_DEFAULT_HOSTING_ADDRESS: ::windows::core::PCWSTR = ::windows::w!("http://*:5357/");
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_DEFAULT_SECURE_HOSTING_ADDRESS: ::windows::core::PCWSTR = ::windows::w!("https://*:5358/");
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeviceDiscoveryMechanism(pub i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const MulticastDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(0i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const DirectedDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(1i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const SecureDirectedDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(2i32);
impl ::core::marker::Copy for DeviceDiscoveryMechanism {}
impl ::core::clone::Clone for DeviceDiscoveryMechanism {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DeviceDiscoveryMechanism {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSDEventType(pub i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDET_NONE: WSDEventType = WSDEventType(0i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDET_INCOMING_MESSAGE: WSDEventType = WSDEventType(1i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDET_INCOMING_FAULT: WSDEventType = WSDEventType(2i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDET_TRANSMISSION_FAILURE: WSDEventType = WSDEventType(3i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDET_RESPONSE_TIMEOUT: WSDEventType = WSDEventType(4i32);
impl ::core::marker::Copy for WSDEventType {}
impl ::core::clone::Clone for WSDEventType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSDEventType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSDUdpMessageType(pub i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const ONE_WAY: WSDUdpMessageType = WSDUdpMessageType(0i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const TWO_WAY: WSDUdpMessageType = WSDUdpMessageType(1i32);
impl ::core::marker::Copy for WSDUdpMessageType {}
impl ::core::clone::Clone for WSDUdpMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSDUdpMessageType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSDXML_OP(pub i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpNone: WSDXML_OP = WSDXML_OP(0i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpEndOfTable: WSDXML_OP = WSDXML_OP(1i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpBeginElement_: WSDXML_OP = WSDXML_OP(2i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpBeginAnyElement: WSDXML_OP = WSDXML_OP(3i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpEndElement: WSDXML_OP = WSDXML_OP(4i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpElement_: WSDXML_OP = WSDXML_OP(5i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAnyElement: WSDXML_OP = WSDXML_OP(6i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAnyElements: WSDXML_OP = WSDXML_OP(7i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAnyText: WSDXML_OP = WSDXML_OP(8i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAttribute_: WSDXML_OP = WSDXML_OP(9i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpBeginChoice: WSDXML_OP = WSDXML_OP(10i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpEndChoice: WSDXML_OP = WSDXML_OP(11i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpBeginSequence: WSDXML_OP = WSDXML_OP(12i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpEndSequence: WSDXML_OP = WSDXML_OP(13i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpBeginAll: WSDXML_OP = WSDXML_OP(14i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpEndAll: WSDXML_OP = WSDXML_OP(15i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAnything: WSDXML_OP = WSDXML_OP(16i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAnyNumber: WSDXML_OP = WSDXML_OP(17i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpOneOrMore: WSDXML_OP = WSDXML_OP(18i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpOptional: WSDXML_OP = WSDXML_OP(19i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatBool_: WSDXML_OP = WSDXML_OP(20i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatInt8_: WSDXML_OP = WSDXML_OP(21i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatInt16_: WSDXML_OP = WSDXML_OP(22i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatInt32_: WSDXML_OP = WSDXML_OP(23i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatInt64_: WSDXML_OP = WSDXML_OP(24i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUInt8_: WSDXML_OP = WSDXML_OP(25i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUInt16_: WSDXML_OP = WSDXML_OP(26i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUInt32_: WSDXML_OP = WSDXML_OP(27i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUInt64_: WSDXML_OP = WSDXML_OP(28i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUnicodeString_: WSDXML_OP = WSDXML_OP(29i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatDom_: WSDXML_OP = WSDXML_OP(30i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatStruct_: WSDXML_OP = WSDXML_OP(31i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUri_: WSDXML_OP = WSDXML_OP(32i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUuidUri_: WSDXML_OP = WSDXML_OP(33i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatName_: WSDXML_OP = WSDXML_OP(34i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatListInsertTail_: WSDXML_OP = WSDXML_OP(35i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatType_: WSDXML_OP = WSDXML_OP(36i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatDynamicType_: WSDXML_OP = WSDXML_OP(37i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatLookupType_: WSDXML_OP = WSDXML_OP(38i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatDuration_: WSDXML_OP = WSDXML_OP(39i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatDateTime_: WSDXML_OP = WSDXML_OP(40i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatFloat_: WSDXML_OP = WSDXML_OP(41i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatDouble_: WSDXML_OP = WSDXML_OP(42i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpProcess_: WSDXML_OP = WSDXML_OP(43i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpQualifiedAttribute_: WSDXML_OP = WSDXML_OP(44i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatXMLDeclaration_: WSDXML_OP = WSDXML_OP(45i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatMax: WSDXML_OP = WSDXML_OP(46i32);
impl ::core::marker::Copy for WSDXML_OP {}
impl ::core::clone::Clone for WSDXML_OP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSDXML_OP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSD_CONFIG_PARAM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_CONFIG_MAX_INBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_CONFIG_MAX_OUTBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_SSL_CERT_FOR_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_SSL_SERVER_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_SSL_CLIENT_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_SSL_NEGOTIATE_CLIENT_CERT: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_COMPACTSIG_SIGNING_CERT: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_COMPACTSIG_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_CONFIG_HOSTING_ADDRESSES: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_CONFIG_DEVICE_ADDRESSES: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_REQUIRE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_REQUIRE_CLIENT_CERT_OR_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_USE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(13i32);
impl ::core::marker::Copy for WSD_CONFIG_PARAM_TYPE {}
impl ::core::clone::Clone for WSD_CONFIG_PARAM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_CONFIG_PARAM_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSD_PROTOCOL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_PT_NONE: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_PT_UDP: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_PT_HTTP: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_PT_HTTPS: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_PT_ALL: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(255i32);
impl ::core::marker::Copy for WSD_PROTOCOL_TYPE {}
impl ::core::clone::Clone for WSD_PROTOCOL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_PROTOCOL_TYPE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct REQUESTBODY_GetStatus {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for REQUESTBODY_GetStatus {}
impl ::core::clone::Clone for REQUESTBODY_GetStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for REQUESTBODY_GetStatus {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REQUESTBODY_Renew {
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REQUESTBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REQUESTBODY_Renew {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REQUESTBODY_Renew {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REQUESTBODY_Subscribe {
    pub EndTo: *mut WSD_ENDPOINT_REFERENCE,
    pub Delivery: *mut WSD_EVENTING_DELIVERY_MODE,
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Filter: *mut WSD_EVENTING_FILTER,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REQUESTBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REQUESTBODY_Subscribe {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REQUESTBODY_Subscribe {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct REQUESTBODY_Unsubscribe {
    pub any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for REQUESTBODY_Unsubscribe {}
impl ::core::clone::Clone for REQUESTBODY_Unsubscribe {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for REQUESTBODY_Unsubscribe {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct RESPONSEBODY_GetMetadata {
    pub Metadata: *mut WSD_METADATA_SECTION_LIST,
}
impl ::core::marker::Copy for RESPONSEBODY_GetMetadata {}
impl ::core::clone::Clone for RESPONSEBODY_GetMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RESPONSEBODY_GetMetadata {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESPONSEBODY_GetStatus {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESPONSEBODY_GetStatus {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESPONSEBODY_GetStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESPONSEBODY_GetStatus {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESPONSEBODY_Renew {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESPONSEBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESPONSEBODY_Renew {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESPONSEBODY_Renew {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESPONSEBODY_Subscribe {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESPONSEBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESPONSEBODY_Subscribe {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESPONSEBODY_Subscribe {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct RESPONSEBODY_SubscriptionEnd {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub Status: ::windows::core::PCWSTR,
    pub Reason: *mut WSD_LOCALIZED_STRING,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for RESPONSEBODY_SubscriptionEnd {}
impl ::core::clone::Clone for RESPONSEBODY_SubscriptionEnd {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RESPONSEBODY_SubscriptionEnd {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDUdpRetransmitParams {
    pub ulSendDelay: u32,
    pub ulRepeat: u32,
    pub ulRepeatMinDelay: u32,
    pub ulRepeatMaxDelay: u32,
    pub ulRepeatUpperDelay: u32,
}
impl ::core::marker::Copy for WSDUdpRetransmitParams {}
impl ::core::clone::Clone for WSDUdpRetransmitParams {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSDUdpRetransmitParams {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_ATTRIBUTE {
    pub Element: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_ATTRIBUTE,
    pub Name: *mut WSDXML_NAME,
    pub Value: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_ATTRIBUTE {}
impl ::core::clone::Clone for WSDXML_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSDXML_ATTRIBUTE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_ELEMENT {
    pub Node: WSDXML_NODE,
    pub Name: *mut WSDXML_NAME,
    pub FirstAttribute: *mut WSDXML_ATTRIBUTE,
    pub FirstChild: *mut WSDXML_NODE,
    pub PrefixMappings: *mut WSDXML_PREFIX_MAPPING,
}
impl ::core::marker::Copy for WSDXML_ELEMENT {}
impl ::core::clone::Clone for WSDXML_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSDXML_ELEMENT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_ELEMENT_LIST {
    pub Next: *mut WSDXML_ELEMENT_LIST,
    pub Element: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSDXML_ELEMENT_LIST {}
impl ::core::clone::Clone for WSDXML_ELEMENT_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSDXML_ELEMENT_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_NAME {
    pub Space: *mut WSDXML_NAMESPACE,
    pub LocalName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_NAME {}
impl ::core::clone::Clone for WSDXML_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSDXML_NAME {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_NAMESPACE {
    pub Uri: ::windows::core::PCWSTR,
    pub PreferredPrefix: ::windows::core::PCWSTR,
    pub Names: *mut WSDXML_NAME,
    pub NamesCount: u16,
    pub Encoding: u16,
}
impl ::core::marker::Copy for WSDXML_NAMESPACE {}
impl ::core::clone::Clone for WSDXML_NAMESPACE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSDXML_NAMESPACE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_NODE {
    pub Type: i32,
    pub Parent: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_NODE,
}
impl WSDXML_NODE {
    pub const ElementType: i32 = 0i32;
    pub const TextType: i32 = 1i32;
}
impl ::core::marker::Copy for WSDXML_NODE {}
impl ::core::clone::Clone for WSDXML_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSDXML_NODE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_PREFIX_MAPPING {
    pub Refs: u32,
    pub Next: *mut WSDXML_PREFIX_MAPPING,
    pub Space: *mut WSDXML_NAMESPACE,
    pub Prefix: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_PREFIX_MAPPING {}
impl ::core::clone::Clone for WSDXML_PREFIX_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSDXML_PREFIX_MAPPING {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_TEXT {
    pub Node: WSDXML_NODE,
    pub Text: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_TEXT {}
impl ::core::clone::Clone for WSDXML_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSDXML_TEXT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_TYPE {
    pub Uri: ::windows::core::PCWSTR,
    pub Table: *const u8,
}
impl ::core::marker::Copy for WSDXML_TYPE {}
impl ::core::clone::Clone for WSDXML_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSDXML_TYPE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_APP_SEQUENCE {
    pub InstanceId: u64,
    pub SequenceId: ::windows::core::PCWSTR,
    pub MessageNumber: u64,
}
impl ::core::marker::Copy for WSD_APP_SEQUENCE {}
impl ::core::clone::Clone for WSD_APP_SEQUENCE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_APP_SEQUENCE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_BYE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_BYE {}
impl ::core::clone::Clone for WSD_BYE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_BYE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_CONFIG_ADDRESSES {
    pub addresses: *mut ::core::option::Option<IWSDAddress>,
    pub dwAddressCount: u32,
}
impl ::core::marker::Copy for WSD_CONFIG_ADDRESSES {}
impl ::core::clone::Clone for WSD_CONFIG_ADDRESSES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_CONFIG_ADDRESSES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_CONFIG_PARAM {
    pub configParamType: WSD_CONFIG_PARAM_TYPE,
    pub pConfigData: *mut ::core::ffi::c_void,
    pub dwConfigDataSize: u32,
}
impl ::core::marker::Copy for WSD_CONFIG_PARAM {}
impl ::core::clone::Clone for WSD_CONFIG_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_CONFIG_PARAM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_DATETIME {
    pub isPositive: super::super::Foundation::BOOL,
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u32,
    pub TZIsLocal: super::super::Foundation::BOOL,
    pub TZIsPositive: super::super::Foundation::BOOL,
    pub TZHour: u8,
    pub TZMinute: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSD_DATETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSD_DATETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_DATETIME {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_DURATION {
    pub isPositive: super::super::Foundation::BOOL,
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millisecond: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSD_DURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSD_DURATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_DURATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_ENDPOINT_REFERENCE {
    pub Address: ::windows::core::PCWSTR,
    pub ReferenceProperties: WSD_REFERENCE_PROPERTIES,
    pub ReferenceParameters: WSD_REFERENCE_PARAMETERS,
    pub PortType: *mut WSDXML_NAME,
    pub ServiceName: *mut WSDXML_NAME,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_ENDPOINT_REFERENCE {}
impl ::core::clone::Clone for WSD_ENDPOINT_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_ENDPOINT_REFERENCE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_ENDPOINT_REFERENCE_LIST {
    pub Next: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Element: *mut WSD_ENDPOINT_REFERENCE,
}
impl ::core::marker::Copy for WSD_ENDPOINT_REFERENCE_LIST {}
impl ::core::clone::Clone for WSD_ENDPOINT_REFERENCE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_ENDPOINT_REFERENCE_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_EVENT {
    pub Hr: ::windows::core::HRESULT,
    pub EventType: u32,
    pub DispatchTag: ::windows::core::PWSTR,
    pub HandlerContext: WSD_HANDLER_CONTEXT,
    pub Soap: *mut WSD_SOAP_MESSAGE,
    pub Operation: *mut WSD_OPERATION,
    pub MessageParameters: ::windows::core::ManuallyDrop<IWSDMessageParameters>,
}
impl ::core::clone::Clone for WSD_EVENT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for WSD_EVENT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_EVENTING_DELIVERY_MODE {
    pub Mode: ::windows::core::PCWSTR,
    pub Push: *mut WSD_EVENTING_DELIVERY_MODE_PUSH,
    pub Data: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WSD_EVENTING_DELIVERY_MODE {}
impl ::core::clone::Clone for WSD_EVENTING_DELIVERY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_EVENTING_DELIVERY_MODE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_EVENTING_DELIVERY_MODE_PUSH {
    pub NotifyTo: *mut WSD_ENDPOINT_REFERENCE,
}
impl ::core::marker::Copy for WSD_EVENTING_DELIVERY_MODE_PUSH {}
impl ::core::clone::Clone for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_EVENTING_DELIVERY_MODE_PUSH {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_EVENTING_EXPIRES {
    pub Duration: *mut WSD_DURATION,
    pub DateTime: *mut WSD_DATETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSD_EVENTING_EXPIRES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSD_EVENTING_EXPIRES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_EVENTING_EXPIRES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_EVENTING_FILTER {
    pub Dialect: ::windows::core::PCWSTR,
    pub FilterAction: *mut WSD_EVENTING_FILTER_ACTION,
    pub Data: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WSD_EVENTING_FILTER {}
impl ::core::clone::Clone for WSD_EVENTING_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_EVENTING_FILTER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_EVENTING_FILTER_ACTION {
    pub Actions: *mut WSD_URI_LIST,
}
impl ::core::marker::Copy for WSD_EVENTING_FILTER_ACTION {}
impl ::core::clone::Clone for WSD_EVENTING_FILTER_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_EVENTING_FILTER_ACTION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_HANDLER_CONTEXT {
    pub Handler: PWSD_SOAP_MESSAGE_HANDLER,
    pub PVoid: *mut ::core::ffi::c_void,
    pub Unknown: ::windows::core::ManuallyDrop<::windows::core::IUnknown>,
}
impl ::core::clone::Clone for WSD_HANDLER_CONTEXT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for WSD_HANDLER_CONTEXT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_HEADER_RELATESTO {
    pub RelationshipType: *mut WSDXML_NAME,
    pub MessageID: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for WSD_HEADER_RELATESTO {}
impl ::core::clone::Clone for WSD_HEADER_RELATESTO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_HEADER_RELATESTO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_HELLO {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_HELLO {}
impl ::core::clone::Clone for WSD_HELLO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_HELLO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_HOST_METADATA {
    pub Host: *mut WSD_SERVICE_METADATA,
    pub Hosted: *mut WSD_SERVICE_METADATA_LIST,
}
impl ::core::marker::Copy for WSD_HOST_METADATA {}
impl ::core::clone::Clone for WSD_HOST_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_HOST_METADATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_LOCALIZED_STRING {
    pub lang: ::windows::core::PCWSTR,
    pub String: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for WSD_LOCALIZED_STRING {}
impl ::core::clone::Clone for WSD_LOCALIZED_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_LOCALIZED_STRING {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_LOCALIZED_STRING_LIST {
    pub Next: *mut WSD_LOCALIZED_STRING_LIST,
    pub Element: *mut WSD_LOCALIZED_STRING,
}
impl ::core::marker::Copy for WSD_LOCALIZED_STRING_LIST {}
impl ::core::clone::Clone for WSD_LOCALIZED_STRING_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_LOCALIZED_STRING_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_METADATA_SECTION {
    pub Dialect: ::windows::core::PCWSTR,
    pub Identifier: ::windows::core::PCWSTR,
    pub Data: *mut ::core::ffi::c_void,
    pub MetadataReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Location: ::windows::core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_METADATA_SECTION {}
impl ::core::clone::Clone for WSD_METADATA_SECTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_METADATA_SECTION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_METADATA_SECTION_LIST {
    pub Next: *mut WSD_METADATA_SECTION_LIST,
    pub Element: *mut WSD_METADATA_SECTION,
}
impl ::core::marker::Copy for WSD_METADATA_SECTION_LIST {}
impl ::core::clone::Clone for WSD_METADATA_SECTION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_METADATA_SECTION_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_NAME_LIST {
    pub Next: *mut WSD_NAME_LIST,
    pub Element: *mut WSDXML_NAME,
}
impl ::core::marker::Copy for WSD_NAME_LIST {}
impl ::core::clone::Clone for WSD_NAME_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_NAME_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_OPERATION {
    pub RequestType: *mut WSDXML_TYPE,
    pub ResponseType: *mut WSDXML_TYPE,
    pub RequestStubFunction: WSD_STUB_FUNCTION,
}
impl ::core::marker::Copy for WSD_OPERATION {}
impl ::core::clone::Clone for WSD_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_OPERATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_PORT_TYPE {
    pub EncodedName: u32,
    pub OperationCount: u32,
    pub Operations: *mut WSD_OPERATION,
    pub ProtocolType: WSD_PROTOCOL_TYPE,
}
impl ::core::marker::Copy for WSD_PORT_TYPE {}
impl ::core::clone::Clone for WSD_PORT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_PORT_TYPE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_PROBE {
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_PROBE {}
impl ::core::clone::Clone for WSD_PROBE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_PROBE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_PROBE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_PROBE_MATCH {}
impl ::core::clone::Clone for WSD_PROBE_MATCH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_PROBE_MATCH {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_PROBE_MATCHES {
    pub ProbeMatch: *mut WSD_PROBE_MATCH_LIST,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_PROBE_MATCHES {}
impl ::core::clone::Clone for WSD_PROBE_MATCHES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_PROBE_MATCHES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_PROBE_MATCH_LIST {
    pub Next: *mut WSD_PROBE_MATCH_LIST,
    pub Element: *mut WSD_PROBE_MATCH,
}
impl ::core::marker::Copy for WSD_PROBE_MATCH_LIST {}
impl ::core::clone::Clone for WSD_PROBE_MATCH_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_PROBE_MATCH_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_REFERENCE_PARAMETERS {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_REFERENCE_PARAMETERS {}
impl ::core::clone::Clone for WSD_REFERENCE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_REFERENCE_PARAMETERS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_REFERENCE_PROPERTIES {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_REFERENCE_PROPERTIES {}
impl ::core::clone::Clone for WSD_REFERENCE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_REFERENCE_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_RELATIONSHIP_METADATA {
    pub Type: ::windows::core::PCWSTR,
    pub Data: *mut WSD_HOST_METADATA,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RELATIONSHIP_METADATA {}
impl ::core::clone::Clone for WSD_RELATIONSHIP_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_RELATIONSHIP_METADATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_RESOLVE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RESOLVE {}
impl ::core::clone::Clone for WSD_RESOLVE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_RESOLVE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_RESOLVE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RESOLVE_MATCH {}
impl ::core::clone::Clone for WSD_RESOLVE_MATCH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_RESOLVE_MATCH {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_RESOLVE_MATCHES {
    pub ResolveMatch: *mut WSD_RESOLVE_MATCH,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RESOLVE_MATCHES {}
impl ::core::clone::Clone for WSD_RESOLVE_MATCHES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_RESOLVE_MATCHES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SCOPES {
    pub MatchBy: ::windows::core::PCWSTR,
    pub Scopes: *mut WSD_URI_LIST,
}
impl ::core::marker::Copy for WSD_SCOPES {}
impl ::core::clone::Clone for WSD_SCOPES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_SCOPES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WSD_SECURITY_CERT_VALIDATION {
    pub certMatchArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: super::super::Security::Cryptography::HCERTSTORE,
    pub hCertIssuerStore: super::super::Security::Cryptography::HCERTSTORE,
    pub dwCertCheckOptions: u32,
    pub pszCNGHashAlgId: ::windows::core::PCWSTR,
    pub pbCertHash: *mut u8,
    pub dwCertHashSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WSD_SECURITY_CERT_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WSD_SECURITY_CERT_VALIDATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WSD_SECURITY_CERT_VALIDATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WSD_SECURITY_CERT_VALIDATION_V1 {
    pub certMatchArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: super::super::Security::Cryptography::HCERTSTORE,
    pub hCertIssuerStore: super::super::Security::Cryptography::HCERTSTORE,
    pub dwCertCheckOptions: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WSD_SECURITY_CERT_VALIDATION_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WSD_SECURITY_CERT_VALIDATION_V1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WSD_SECURITY_SIGNATURE_VALIDATION {
    pub signingCertArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwSigningCertArrayCount: u32,
    pub hSigningCertStore: super::super::Security::Cryptography::HCERTSTORE,
    pub dwFlags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WSD_SECURITY_SIGNATURE_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WSD_SECURITY_SIGNATURE_VALIDATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SERVICE_METADATA {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Types: *mut WSD_NAME_LIST,
    pub ServiceId: ::windows::core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_SERVICE_METADATA {}
impl ::core::clone::Clone for WSD_SERVICE_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_SERVICE_METADATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SERVICE_METADATA_LIST {
    pub Next: *mut WSD_SERVICE_METADATA_LIST,
    pub Element: *mut WSD_SERVICE_METADATA,
}
impl ::core::marker::Copy for WSD_SERVICE_METADATA_LIST {}
impl ::core::clone::Clone for WSD_SERVICE_METADATA_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_SERVICE_METADATA_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_FAULT {
    pub Code: *mut WSD_SOAP_FAULT_CODE,
    pub Reason: *mut WSD_SOAP_FAULT_REASON,
    pub Node: ::windows::core::PCWSTR,
    pub Role: ::windows::core::PCWSTR,
    pub Detail: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT {}
impl ::core::clone::Clone for WSD_SOAP_FAULT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_SOAP_FAULT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_FAULT_CODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT_CODE {}
impl ::core::clone::Clone for WSD_SOAP_FAULT_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_SOAP_FAULT_CODE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_FAULT_REASON {
    pub Text: *mut WSD_LOCALIZED_STRING_LIST,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT_REASON {}
impl ::core::clone::Clone for WSD_SOAP_FAULT_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_SOAP_FAULT_REASON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_FAULT_SUBCODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT_SUBCODE {}
impl ::core::clone::Clone for WSD_SOAP_FAULT_SUBCODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_SOAP_FAULT_SUBCODE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_HEADER {
    pub To: ::windows::core::PCWSTR,
    pub Action: ::windows::core::PCWSTR,
    pub MessageID: ::windows::core::PCWSTR,
    pub RelatesTo: WSD_HEADER_RELATESTO,
    pub ReplyTo: *mut WSD_ENDPOINT_REFERENCE,
    pub From: *mut WSD_ENDPOINT_REFERENCE,
    pub FaultTo: *mut WSD_ENDPOINT_REFERENCE,
    pub AppSequence: *mut WSD_APP_SEQUENCE,
    pub AnyHeaders: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_SOAP_HEADER {}
impl ::core::clone::Clone for WSD_SOAP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_SOAP_HEADER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_MESSAGE {
    pub Header: WSD_SOAP_HEADER,
    pub Body: *mut ::core::ffi::c_void,
    pub BodyType: *mut WSDXML_TYPE,
}
impl ::core::marker::Copy for WSD_SOAP_MESSAGE {}
impl ::core::clone::Clone for WSD_SOAP_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_SOAP_MESSAGE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    pub hr: ::windows::core::HRESULT,
    pub eventHandle: super::super::Foundation::HANDLE,
    pub messageParameters: ::windows::core::ManuallyDrop<IWSDMessageParameters>,
    pub results: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_THIS_DEVICE_METADATA {
    pub FriendlyName: *mut WSD_LOCALIZED_STRING_LIST,
    pub FirmwareVersion: ::windows::core::PCWSTR,
    pub SerialNumber: ::windows::core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_THIS_DEVICE_METADATA {}
impl ::core::clone::Clone for WSD_THIS_DEVICE_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_THIS_DEVICE_METADATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_THIS_MODEL_METADATA {
    pub Manufacturer: *mut WSD_LOCALIZED_STRING_LIST,
    pub ManufacturerUrl: ::windows::core::PCWSTR,
    pub ModelName: *mut WSD_LOCALIZED_STRING_LIST,
    pub ModelNumber: ::windows::core::PCWSTR,
    pub ModelUrl: ::windows::core::PCWSTR,
    pub PresentationUrl: ::windows::core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_THIS_MODEL_METADATA {}
impl ::core::clone::Clone for WSD_THIS_MODEL_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_THIS_MODEL_METADATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_UNKNOWN_LOOKUP {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_UNKNOWN_LOOKUP {}
impl ::core::clone::Clone for WSD_UNKNOWN_LOOKUP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_UNKNOWN_LOOKUP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_URI_LIST {
    pub Next: *mut WSD_URI_LIST,
    pub Element: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for WSD_URI_LIST {}
impl ::core::clone::Clone for WSD_URI_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSD_URI_LIST {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub type PWSD_SOAP_MESSAGE_HANDLER = ::core::option::Option<unsafe extern "system" fn(thisunknown: ::core::option::Option<::windows::core::IUnknown>, event: *mut WSD_EVENT) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub type WSD_STUB_FUNCTION = ::core::option::Option<unsafe extern "system" fn(server: ::core::option::Option<::windows::core::IUnknown>, session: ::core::option::Option<IWSDServiceMessaging>, event: *mut WSD_EVENT) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
