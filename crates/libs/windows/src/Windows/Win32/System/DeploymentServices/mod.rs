#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeAsyncRecvDone<P0>(hclientrequest: P0, action: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeAsyncRecvDone(hclientrequest : super::super::Foundation:: HANDLE, action : u32) -> u32);
    PxeAsyncRecvDone(hclientrequest.into_param().abi(), action)
}
#[inline]
pub unsafe fn PxeDhcpAppendOption(preplypacket: *mut ::core::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32, boption: u8, boptionlen: u8, pvalue: ::core::option::Option<*const ::core::ffi::c_void>) -> u32 {
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeDhcpAppendOption(preplypacket : *mut ::core::ffi::c_void, umaxreplypacketlen : u32, pureplypacketlen : *mut u32, boption : u8, boptionlen : u8, pvalue : *const ::core::ffi::c_void) -> u32);
    PxeDhcpAppendOption(preplypacket, umaxreplypacketlen, pureplypacketlen, boption, boptionlen, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null())))
}
#[inline]
pub unsafe fn PxeDhcpAppendOptionRaw(preplypacket: *mut ::core::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32, ubufferlen: u16, pbuffer: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeDhcpAppendOptionRaw(preplypacket : *mut ::core::ffi::c_void, umaxreplypacketlen : u32, pureplypacketlen : *mut u32, ubufferlen : u16, pbuffer : *const ::core::ffi::c_void) -> u32);
    PxeDhcpAppendOptionRaw(preplypacket, umaxreplypacketlen, pureplypacketlen, ubufferlen, pbuffer)
}
#[inline]
pub unsafe fn PxeDhcpGetOptionValue(ppacket: *const ::core::ffi::c_void, upacketlen: u32, uinstance: u32, boption: u8, pboptionlen: ::core::option::Option<*mut u8>, ppoptionvalue: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> u32 {
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeDhcpGetOptionValue(ppacket : *const ::core::ffi::c_void, upacketlen : u32, uinstance : u32, boption : u8, pboptionlen : *mut u8, ppoptionvalue : *mut *mut ::core::ffi::c_void) -> u32);
    PxeDhcpGetOptionValue(ppacket, upacketlen, uinstance, boption, ::core::mem::transmute(pboptionlen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppoptionvalue.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PxeDhcpGetVendorOptionValue(ppacket: *const ::core::ffi::c_void, upacketlen: u32, boption: u8, uinstance: u32, pboptionlen: ::core::option::Option<*mut u8>, ppoptionvalue: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> u32 {
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeDhcpGetVendorOptionValue(ppacket : *const ::core::ffi::c_void, upacketlen : u32, boption : u8, uinstance : u32, pboptionlen : *mut u8, ppoptionvalue : *mut *mut ::core::ffi::c_void) -> u32);
    PxeDhcpGetVendorOptionValue(ppacket, upacketlen, boption, uinstance, ::core::mem::transmute(pboptionlen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppoptionvalue.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PxeDhcpInitialize(precvpacket: *const ::core::ffi::c_void, urecvpacketlen: u32, preplypacket: *mut ::core::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32) -> u32 {
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeDhcpInitialize(precvpacket : *const ::core::ffi::c_void, urecvpacketlen : u32, preplypacket : *mut ::core::ffi::c_void, umaxreplypacketlen : u32, pureplypacketlen : *mut u32) -> u32);
    PxeDhcpInitialize(precvpacket, urecvpacketlen, preplypacket, umaxreplypacketlen, pureplypacketlen)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeDhcpIsValid<P0>(ppacket: *const ::core::ffi::c_void, upacketlen: u32, brequestpacket: P0, pbpxeoptionpresent: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeDhcpIsValid(ppacket : *const ::core::ffi::c_void, upacketlen : u32, brequestpacket : super::super::Foundation:: BOOL, pbpxeoptionpresent : *mut super::super::Foundation:: BOOL) -> u32);
    PxeDhcpIsValid(ppacket, upacketlen, brequestpacket.into_param().abi(), ::core::mem::transmute(pbpxeoptionpresent.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PxeDhcpv6AppendOption(preply: *mut ::core::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32, woptiontype: u16, cboption: u16, poption: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeDhcpv6AppendOption(preply : *mut ::core::ffi::c_void, cbreply : u32, pcbreplyused : *mut u32, woptiontype : u16, cboption : u16, poption : *const ::core::ffi::c_void) -> u32);
    PxeDhcpv6AppendOption(preply, cbreply, pcbreplyused, woptiontype, cboption, poption)
}
#[inline]
pub unsafe fn PxeDhcpv6AppendOptionRaw(preply: *mut ::core::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32, cbbuffer: u16, pbuffer: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeDhcpv6AppendOptionRaw(preply : *mut ::core::ffi::c_void, cbreply : u32, pcbreplyused : *mut u32, cbbuffer : u16, pbuffer : *const ::core::ffi::c_void) -> u32);
    PxeDhcpv6AppendOptionRaw(preply, cbreply, pcbreplyused, cbbuffer, pbuffer)
}
#[inline]
pub unsafe fn PxeDhcpv6CreateRelayRepl(prelaymessages: &[PXE_DHCPV6_NESTED_RELAY_MESSAGE], pinnerpacket: &[u8], preplybuffer: *mut ::core::ffi::c_void, cbreplybuffer: u32, pcbreplybuffer: *mut u32) -> u32 {
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeDhcpv6CreateRelayRepl(prelaymessages : *const PXE_DHCPV6_NESTED_RELAY_MESSAGE, nrelaymessages : u32, pinnerpacket : *const u8, cbinnerpacket : u32, preplybuffer : *mut ::core::ffi::c_void, cbreplybuffer : u32, pcbreplybuffer : *mut u32) -> u32);
    PxeDhcpv6CreateRelayRepl(::core::mem::transmute(prelaymessages.as_ptr()), prelaymessages.len().try_into().unwrap(), ::core::mem::transmute(pinnerpacket.as_ptr()), pinnerpacket.len().try_into().unwrap(), preplybuffer, cbreplybuffer, pcbreplybuffer)
}
#[inline]
pub unsafe fn PxeDhcpv6GetOptionValue(ppacket: *const ::core::ffi::c_void, upacketlen: u32, uinstance: u32, woption: u16, pwoptionlen: ::core::option::Option<*mut u16>, ppoptionvalue: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> u32 {
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeDhcpv6GetOptionValue(ppacket : *const ::core::ffi::c_void, upacketlen : u32, uinstance : u32, woption : u16, pwoptionlen : *mut u16, ppoptionvalue : *mut *mut ::core::ffi::c_void) -> u32);
    PxeDhcpv6GetOptionValue(ppacket, upacketlen, uinstance, woption, ::core::mem::transmute(pwoptionlen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppoptionvalue.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PxeDhcpv6GetVendorOptionValue(ppacket: *const ::core::ffi::c_void, upacketlen: u32, dwenterprisenumber: u32, woption: u16, uinstance: u32, pwoptionlen: ::core::option::Option<*mut u16>, ppoptionvalue: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> u32 {
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeDhcpv6GetVendorOptionValue(ppacket : *const ::core::ffi::c_void, upacketlen : u32, dwenterprisenumber : u32, woption : u16, uinstance : u32, pwoptionlen : *mut u16, ppoptionvalue : *mut *mut ::core::ffi::c_void) -> u32);
    PxeDhcpv6GetVendorOptionValue(ppacket, upacketlen, dwenterprisenumber, woption, uinstance, ::core::mem::transmute(pwoptionlen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppoptionvalue.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PxeDhcpv6Initialize(prequest: *const ::core::ffi::c_void, cbrequest: u32, preply: *mut ::core::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32) -> u32 {
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeDhcpv6Initialize(prequest : *const ::core::ffi::c_void, cbrequest : u32, preply : *mut ::core::ffi::c_void, cbreply : u32, pcbreplyused : *mut u32) -> u32);
    PxeDhcpv6Initialize(prequest, cbrequest, preply, cbreply, pcbreplyused)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeDhcpv6IsValid<P0>(ppacket: *const ::core::ffi::c_void, upacketlen: u32, brequestpacket: P0, pbpxeoptionpresent: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeDhcpv6IsValid(ppacket : *const ::core::ffi::c_void, upacketlen : u32, brequestpacket : super::super::Foundation:: BOOL, pbpxeoptionpresent : *mut super::super::Foundation:: BOOL) -> u32);
    PxeDhcpv6IsValid(ppacket, upacketlen, brequestpacket.into_param().abi(), ::core::mem::transmute(pbpxeoptionpresent.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PxeDhcpv6ParseRelayForw(prelayforwpacket: *const ::core::ffi::c_void, urelayforwpacketlen: u32, prelaymessages: &mut [PXE_DHCPV6_NESTED_RELAY_MESSAGE], pnrelaymessages: *mut u32, ppinnerpacket: *mut *mut u8, pcbinnerpacket: *mut u32) -> u32 {
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeDhcpv6ParseRelayForw(prelayforwpacket : *const ::core::ffi::c_void, urelayforwpacketlen : u32, prelaymessages : *mut PXE_DHCPV6_NESTED_RELAY_MESSAGE, nrelaymessages : u32, pnrelaymessages : *mut u32, ppinnerpacket : *mut *mut u8, pcbinnerpacket : *mut u32) -> u32);
    PxeDhcpv6ParseRelayForw(prelayforwpacket, urelayforwpacketlen, ::core::mem::transmute(prelaymessages.as_ptr()), prelaymessages.len().try_into().unwrap(), pnrelaymessages, ppinnerpacket, pcbinnerpacket)
}
#[inline]
pub unsafe fn PxeGetServerInfo(uinfotype: u32, pbuffer: *mut ::core::ffi::c_void, ubufferlen: u32) -> u32 {
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeGetServerInfo(uinfotype : u32, pbuffer : *mut ::core::ffi::c_void, ubufferlen : u32) -> u32);
    PxeGetServerInfo(uinfotype, pbuffer, ubufferlen)
}
#[inline]
pub unsafe fn PxeGetServerInfoEx(uinfotype: u32, pbuffer: *mut ::core::ffi::c_void, ubufferlen: u32, pubufferused: *mut u32) -> u32 {
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeGetServerInfoEx(uinfotype : u32, pbuffer : *mut ::core::ffi::c_void, ubufferlen : u32, pubufferused : *mut u32) -> u32);
    PxeGetServerInfoEx(uinfotype, pbuffer, ubufferlen, pubufferused)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxePacketAllocate<P0, P1>(hprovider: P0, hclientrequest: P1, usize: u32) -> *mut ::core::ffi::c_void
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdspxe.dll" "system" fn PxePacketAllocate(hprovider : super::super::Foundation:: HANDLE, hclientrequest : super::super::Foundation:: HANDLE, usize : u32) -> *mut ::core::ffi::c_void);
    PxePacketAllocate(hprovider.into_param().abi(), hclientrequest.into_param().abi(), usize)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxePacketFree<P0, P1>(hprovider: P0, hclientrequest: P1, ppacket: *const ::core::ffi::c_void) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdspxe.dll" "system" fn PxePacketFree(hprovider : super::super::Foundation:: HANDLE, hclientrequest : super::super::Foundation:: HANDLE, ppacket : *const ::core::ffi::c_void) -> u32);
    PxePacketFree(hprovider.into_param().abi(), hclientrequest.into_param().abi(), ppacket)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeProviderEnumClose<P0>(henum: P0) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeProviderEnumClose(henum : super::super::Foundation:: HANDLE) -> u32);
    PxeProviderEnumClose(henum.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeProviderEnumFirst(phenum: *mut super::super::Foundation::HANDLE) -> u32 {
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeProviderEnumFirst(phenum : *mut super::super::Foundation:: HANDLE) -> u32);
    PxeProviderEnumFirst(phenum)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeProviderEnumNext<P0>(henum: P0, ppprovider: *mut *mut PXE_PROVIDER) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeProviderEnumNext(henum : super::super::Foundation:: HANDLE, ppprovider : *mut *mut PXE_PROVIDER) -> u32);
    PxeProviderEnumNext(henum.into_param().abi(), ppprovider)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeProviderFreeInfo(pprovider: *const PXE_PROVIDER) -> u32 {
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeProviderFreeInfo(pprovider : *const PXE_PROVIDER) -> u32);
    PxeProviderFreeInfo(pprovider)
}
#[inline]
pub unsafe fn PxeProviderQueryIndex<P0>(pszprovidername: P0, puindex: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeProviderQueryIndex(pszprovidername : ::windows_core::PCWSTR, puindex : *mut u32) -> u32);
    PxeProviderQueryIndex(pszprovidername.into_param().abi(), puindex)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn PxeProviderRegister<P0, P1, P2>(pszprovidername: P0, pszmodulepath: P1, index: u32, biscritical: P2, phproviderkey: ::core::option::Option<*mut super::Registry::HKEY>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeProviderRegister(pszprovidername : ::windows_core::PCWSTR, pszmodulepath : ::windows_core::PCWSTR, index : u32, biscritical : super::super::Foundation:: BOOL, phproviderkey : *mut super::Registry:: HKEY) -> u32);
    PxeProviderRegister(pszprovidername.into_param().abi(), pszmodulepath.into_param().abi(), index, biscritical.into_param().abi(), ::core::mem::transmute(phproviderkey.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeProviderSetAttribute<P0>(hprovider: P0, attribute: u32, pparameterbuffer: *const ::core::ffi::c_void, uparamlen: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeProviderSetAttribute(hprovider : super::super::Foundation:: HANDLE, attribute : u32, pparameterbuffer : *const ::core::ffi::c_void, uparamlen : u32) -> u32);
    PxeProviderSetAttribute(hprovider.into_param().abi(), attribute, pparameterbuffer, uparamlen)
}
#[inline]
pub unsafe fn PxeProviderUnRegister<P0>(pszprovidername: P0) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeProviderUnRegister(pszprovidername : ::windows_core::PCWSTR) -> u32);
    PxeProviderUnRegister(pszprovidername.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeRegisterCallback<P0>(hprovider: P0, callbacktype: u32, pcallbackfunction: *const ::core::ffi::c_void, pcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeRegisterCallback(hprovider : super::super::Foundation:: HANDLE, callbacktype : u32, pcallbackfunction : *const ::core::ffi::c_void, pcontext : *const ::core::ffi::c_void) -> u32);
    PxeRegisterCallback(hprovider.into_param().abi(), callbacktype, pcallbackfunction, ::core::mem::transmute(pcontext.unwrap_or(::std::ptr::null())))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeSendReply<P0>(hclientrequest: P0, ppacket: *const ::core::ffi::c_void, upacketlen: u32, paddress: *const PXE_ADDRESS) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeSendReply(hclientrequest : super::super::Foundation:: HANDLE, ppacket : *const ::core::ffi::c_void, upacketlen : u32, paddress : *const PXE_ADDRESS) -> u32);
    PxeSendReply(hclientrequest.into_param().abi(), ppacket, upacketlen, paddress)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeTrace<P0, P1>(hprovider: P0, severity: u32, pszformat: P1) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wdspxe.dll" "cdecl" fn PxeTrace(hprovider : super::super::Foundation:: HANDLE, severity : u32, pszformat : ::windows_core::PCWSTR) -> u32);
    PxeTrace(hprovider.into_param().abi(), severity, pszformat.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PxeTraceV<P0, P1>(hprovider: P0, severity: u32, pszformat: P1, params: *const i8) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wdspxe.dll" "system" fn PxeTraceV(hprovider : super::super::Foundation:: HANDLE, severity : u32, pszformat : ::windows_core::PCWSTR, params : *const i8) -> u32);
    PxeTraceV(hprovider.into_param().abi(), severity, pszformat.into_param().abi(), params)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsBpAddOption<P0>(hhandle: P0, uoption: u32, uvaluelen: u32, pvalue: *const ::core::ffi::c_void) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsbp.dll" "system" fn WdsBpAddOption(hhandle : super::super::Foundation:: HANDLE, uoption : u32, uvaluelen : u32, pvalue : *const ::core::ffi::c_void) -> u32);
    WdsBpAddOption(hhandle.into_param().abi(), uoption, uvaluelen, pvalue)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsBpCloseHandle<P0>(hhandle: P0) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsbp.dll" "system" fn WdsBpCloseHandle(hhandle : super::super::Foundation:: HANDLE) -> u32);
    WdsBpCloseHandle(hhandle.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsBpGetOptionBuffer<P0>(hhandle: P0, ubufferlen: u32, pbuffer: *mut ::core::ffi::c_void, pubytes: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsbp.dll" "system" fn WdsBpGetOptionBuffer(hhandle : super::super::Foundation:: HANDLE, ubufferlen : u32, pbuffer : *mut ::core::ffi::c_void, pubytes : *mut u32) -> u32);
    WdsBpGetOptionBuffer(hhandle.into_param().abi(), ubufferlen, pbuffer, pubytes)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsBpInitialize(bpackettype: u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    ::windows_targets::link!("wdsbp.dll" "system" fn WdsBpInitialize(bpackettype : u8, phhandle : *mut super::super::Foundation:: HANDLE) -> u32);
    WdsBpInitialize(bpackettype, phhandle)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsBpParseInitialize(ppacket: *const ::core::ffi::c_void, upacketlen: u32, pbpackettype: ::core::option::Option<*mut u8>, phhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    ::windows_targets::link!("wdsbp.dll" "system" fn WdsBpParseInitialize(ppacket : *const ::core::ffi::c_void, upacketlen : u32, pbpackettype : *mut u8, phhandle : *mut super::super::Foundation:: HANDLE) -> u32);
    WdsBpParseInitialize(ppacket, upacketlen, ::core::mem::transmute(pbpackettype.unwrap_or(::std::ptr::null_mut())), phhandle)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsBpParseInitializev6(ppacket: *const ::core::ffi::c_void, upacketlen: u32, pbpackettype: ::core::option::Option<*mut u8>, phhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    ::windows_targets::link!("wdsbp.dll" "system" fn WdsBpParseInitializev6(ppacket : *const ::core::ffi::c_void, upacketlen : u32, pbpackettype : *mut u8, phhandle : *mut super::super::Foundation:: HANDLE) -> u32);
    WdsBpParseInitializev6(ppacket, upacketlen, ::core::mem::transmute(pbpackettype.unwrap_or(::std::ptr::null_mut())), phhandle)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsBpQueryOption<P0>(hhandle: P0, uoption: u32, uvaluelen: u32, pvalue: *mut ::core::ffi::c_void, pubytes: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsbp.dll" "system" fn WdsBpQueryOption(hhandle : super::super::Foundation:: HANDLE, uoption : u32, uvaluelen : u32, pvalue : *mut ::core::ffi::c_void, pubytes : *mut u32) -> u32);
    WdsBpQueryOption(hhandle.into_param().abi(), uoption, uvaluelen, pvalue, ::core::mem::transmute(pubytes.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliAuthorizeSession<P0>(hsession: P0, pcred: ::core::option::Option<*const WDS_CLI_CRED>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliAuthorizeSession(hsession : super::super::Foundation:: HANDLE, pcred : *const WDS_CLI_CRED) -> ::windows_core::HRESULT);
    WdsCliAuthorizeSession(hsession.into_param().abi(), ::core::mem::transmute(pcred.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliCancelTransfer<P0>(htransfer: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliCancelTransfer(htransfer : super::super::Foundation:: HANDLE) -> ::windows_core::HRESULT);
    WdsCliCancelTransfer(htransfer.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliClose<P0>(handle: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliClose(handle : super::super::Foundation:: HANDLE) -> ::windows_core::HRESULT);
    WdsCliClose(handle.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliCreateSession<P0>(pwszserver: P0, pcred: ::core::option::Option<*const WDS_CLI_CRED>) -> ::windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliCreateSession(pwszserver : ::windows_core::PCWSTR, pcred : *const WDS_CLI_CRED, phsession : *mut super::super::Foundation:: HANDLE) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliCreateSession(pwszserver.into_param().abi(), ::core::mem::transmute(pcred.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliFindFirstImage<P0>(hsession: P0) -> ::windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliFindFirstImage(hsession : super::super::Foundation:: HANDLE, phfindhandle : *mut super::super::Foundation:: HANDLE) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliFindFirstImage(hsession.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliFindNextImage<P0>(handle: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliFindNextImage(handle : super::super::Foundation:: HANDLE) -> ::windows_core::HRESULT);
    WdsCliFindNextImage(handle.into_param().abi()).ok()
}
#[inline]
pub unsafe fn WdsCliFreeStringArray(ppwszarray: ::core::option::Option<&mut [::windows_core::PWSTR]>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliFreeStringArray(ppwszarray : *mut ::windows_core::PWSTR, ulcount : u32) -> ::windows_core::HRESULT);
    WdsCliFreeStringArray(::core::mem::transmute(ppwszarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppwszarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
}
#[inline]
pub unsafe fn WdsCliGetDriverQueryXml<P0>(pwszwindirpath: P0) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetDriverQueryXml(pwszwindirpath : ::windows_core::PCWSTR, ppwszdriverquery : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetDriverQueryXml(pwszwindirpath.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetEnumerationFlags<P0>(handle: P0) -> ::windows_core::Result<u32>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetEnumerationFlags(handle : super::super::Foundation:: HANDLE, pdwflags : *mut u32) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetEnumerationFlags(handle.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageArchitecture<P0>(hifh: P0) -> ::windows_core::Result<CPU_ARCHITECTURE>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageArchitecture(hifh : super::super::Foundation:: HANDLE, pdwvalue : *mut CPU_ARCHITECTURE) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetImageArchitecture(hifh.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageDescription<P0>(hifh: P0) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageDescription(hifh : super::super::Foundation:: HANDLE, ppwszvalue : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetImageDescription(hifh.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageFiles<P0>(hifh: P0, pppwszfiles: *mut *mut ::windows_core::PWSTR, pdwcount: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageFiles(hifh : super::super::Foundation:: HANDLE, pppwszfiles : *mut *mut ::windows_core::PWSTR, pdwcount : *mut u32) -> ::windows_core::HRESULT);
    WdsCliGetImageFiles(hifh.into_param().abi(), pppwszfiles, pdwcount).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageGroup<P0>(hifh: P0) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageGroup(hifh : super::super::Foundation:: HANDLE, ppwszvalue : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetImageGroup(hifh.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageHalName<P0>(hifh: P0) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageHalName(hifh : super::super::Foundation:: HANDLE, ppwszvalue : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetImageHalName(hifh.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageHandleFromFindHandle<P0>(findhandle: P0) -> ::windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageHandleFromFindHandle(findhandle : super::super::Foundation:: HANDLE, phimagehandle : *mut super::super::Foundation:: HANDLE) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetImageHandleFromFindHandle(findhandle.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageHandleFromTransferHandle<P0>(htransfer: P0) -> ::windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageHandleFromTransferHandle(htransfer : super::super::Foundation:: HANDLE, phimagehandle : *mut super::super::Foundation:: HANDLE) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetImageHandleFromTransferHandle(htransfer.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageIndex<P0>(hifh: P0) -> ::windows_core::Result<u32>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageIndex(hifh : super::super::Foundation:: HANDLE, pdwvalue : *mut u32) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetImageIndex(hifh.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageLanguage<P0>(hifh: P0) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageLanguage(hifh : super::super::Foundation:: HANDLE, ppwszvalue : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetImageLanguage(hifh.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageLanguages<P0>(hifh: P0, pppszvalues: *mut *mut *mut i8, pdwnumvalues: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageLanguages(hifh : super::super::Foundation:: HANDLE, pppszvalues : *mut *mut *mut i8, pdwnumvalues : *mut u32) -> ::windows_core::HRESULT);
    WdsCliGetImageLanguages(hifh.into_param().abi(), pppszvalues, pdwnumvalues).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageLastModifiedTime<P0>(hifh: P0) -> ::windows_core::Result<*mut super::super::Foundation::SYSTEMTIME>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageLastModifiedTime(hifh : super::super::Foundation:: HANDLE, ppsystimevalue : *mut *mut super::super::Foundation:: SYSTEMTIME) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetImageLastModifiedTime(hifh.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageName<P0>(hifh: P0) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageName(hifh : super::super::Foundation:: HANDLE, ppwszvalue : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetImageName(hifh.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageNamespace<P0>(hifh: P0) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageNamespace(hifh : super::super::Foundation:: HANDLE, ppwszvalue : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetImageNamespace(hifh.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageParameter<P0>(hifh: P0, paramtype: WDS_CLI_IMAGE_PARAM_TYPE, presponse: *mut ::core::ffi::c_void, uresponselen: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageParameter(hifh : super::super::Foundation:: HANDLE, paramtype : WDS_CLI_IMAGE_PARAM_TYPE, presponse : *mut ::core::ffi::c_void, uresponselen : u32) -> ::windows_core::HRESULT);
    WdsCliGetImageParameter(hifh.into_param().abi(), paramtype, presponse, uresponselen).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImagePath<P0>(hifh: P0) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImagePath(hifh : super::super::Foundation:: HANDLE, ppwszvalue : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetImagePath(hifh.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageSize<P0>(hifh: P0) -> ::windows_core::Result<u64>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageSize(hifh : super::super::Foundation:: HANDLE, pullvalue : *mut u64) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetImageSize(hifh.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageType<P0>(hifh: P0) -> ::windows_core::Result<WDS_CLI_IMAGE_TYPE>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageType(hifh : super::super::Foundation:: HANDLE, pimagetype : *mut WDS_CLI_IMAGE_TYPE) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetImageType(hifh.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetImageVersion<P0>(hifh: P0) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetImageVersion(hifh : super::super::Foundation:: HANDLE, ppwszvalue : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetImageVersion(hifh.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliGetTransferSize<P0>(hifh: P0) -> ::windows_core::Result<u64>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliGetTransferSize(hifh : super::super::Foundation:: HANDLE, pullvalue : *mut u64) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliGetTransferSize(hifh.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliInitializeLog<P0, P1, P2>(hsession: P0, ulclientarchitecture: CPU_ARCHITECTURE, pwszclientid: P1, pwszclientaddress: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliInitializeLog(hsession : super::super::Foundation:: HANDLE, ulclientarchitecture : CPU_ARCHITECTURE, pwszclientid : ::windows_core::PCWSTR, pwszclientaddress : ::windows_core::PCWSTR) -> ::windows_core::HRESULT);
    WdsCliInitializeLog(hsession.into_param().abi(), ulclientarchitecture, pwszclientid.into_param().abi(), pwszclientaddress.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliLog<P0>(hsession: P0, ulloglevel: u32, ulmessagecode: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "cdecl" fn WdsCliLog(hsession : super::super::Foundation:: HANDLE, ulloglevel : u32, ulmessagecode : u32) -> ::windows_core::HRESULT);
    WdsCliLog(hsession.into_param().abi(), ulloglevel, ulmessagecode).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliObtainDriverPackages<P0>(himage: P0, ppwszservername: *mut ::windows_core::PWSTR, pppwszdriverpackages: *mut *mut ::windows_core::PWSTR, pulcount: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliObtainDriverPackages(himage : super::super::Foundation:: HANDLE, ppwszservername : *mut ::windows_core::PWSTR, pppwszdriverpackages : *mut *mut ::windows_core::PWSTR, pulcount : *mut u32) -> ::windows_core::HRESULT);
    WdsCliObtainDriverPackages(himage.into_param().abi(), ppwszservername, pppwszdriverpackages, pulcount).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliObtainDriverPackagesEx<P0, P1>(hsession: P0, pwszmachineinfo: P1, ppwszservername: *mut ::windows_core::PWSTR, pppwszdriverpackages: *mut *mut ::windows_core::PWSTR, pulcount: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliObtainDriverPackagesEx(hsession : super::super::Foundation:: HANDLE, pwszmachineinfo : ::windows_core::PCWSTR, ppwszservername : *mut ::windows_core::PWSTR, pppwszdriverpackages : *mut *mut ::windows_core::PWSTR, pulcount : *mut u32) -> ::windows_core::HRESULT);
    WdsCliObtainDriverPackagesEx(hsession.into_param().abi(), pwszmachineinfo.into_param().abi(), ppwszservername, pppwszdriverpackages, pulcount).ok()
}
#[inline]
pub unsafe fn WdsCliRegisterTrace(pfn: PFN_WdsCliTraceFunction) -> ::windows_core::Result<()> {
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliRegisterTrace(pfn : PFN_WdsCliTraceFunction) -> ::windows_core::HRESULT);
    WdsCliRegisterTrace(pfn).ok()
}
#[inline]
pub unsafe fn WdsCliSetTransferBufferSize(ulsizeinbytes: u32) {
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliSetTransferBufferSize(ulsizeinbytes : u32) -> ());
    WdsCliSetTransferBufferSize(ulsizeinbytes)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliTransferFile<P0, P1, P2, P3>(pwszserver: P0, pwsznamespace: P1, pwszremotefilepath: P2, pwszlocalfilepath: P3, dwflags: u32, dwreserved: u32, pfnwdsclicallback: PFN_WdsCliCallback, pvuserdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliTransferFile(pwszserver : ::windows_core::PCWSTR, pwsznamespace : ::windows_core::PCWSTR, pwszremotefilepath : ::windows_core::PCWSTR, pwszlocalfilepath : ::windows_core::PCWSTR, dwflags : u32, dwreserved : u32, pfnwdsclicallback : PFN_WdsCliCallback, pvuserdata : *const ::core::ffi::c_void, phtransfer : *mut super::super::Foundation:: HANDLE) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliTransferFile(pwszserver.into_param().abi(), pwsznamespace.into_param().abi(), pwszremotefilepath.into_param().abi(), pwszlocalfilepath.into_param().abi(), dwflags, dwreserved, pfnwdsclicallback, ::core::mem::transmute(pvuserdata.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliTransferImage<P0, P1>(himage: P0, pwszlocalpath: P1, dwflags: u32, dwreserved: u32, pfnwdsclicallback: PFN_WdsCliCallback, pvuserdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliTransferImage(himage : super::super::Foundation:: HANDLE, pwszlocalpath : ::windows_core::PCWSTR, dwflags : u32, dwreserved : u32, pfnwdsclicallback : PFN_WdsCliCallback, pvuserdata : *const ::core::ffi::c_void, phtransfer : *mut super::super::Foundation:: HANDLE) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WdsCliTransferImage(himage.into_param().abi(), pwszlocalpath.into_param().abi(), dwflags, dwreserved, pfnwdsclicallback, ::core::mem::transmute(pvuserdata.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsCliWaitForTransfer<P0>(htransfer: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsclientapi.dll" "system" fn WdsCliWaitForTransfer(htransfer : super::super::Foundation:: HANDLE) -> ::windows_core::HRESULT);
    WdsCliWaitForTransfer(htransfer.into_param().abi()).ok()
}
#[inline]
pub unsafe fn WdsTransportClientAddRefBuffer(pvbuffer: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link!("wdstptc.dll" "system" fn WdsTransportClientAddRefBuffer(pvbuffer : *const ::core::ffi::c_void) -> u32);
    WdsTransportClientAddRefBuffer(pvbuffer)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientCancelSession<P0>(hsessionkey: P0) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdstptc.dll" "system" fn WdsTransportClientCancelSession(hsessionkey : super::super::Foundation:: HANDLE) -> u32);
    WdsTransportClientCancelSession(hsessionkey.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientCancelSessionEx<P0>(hsessionkey: P0, dwerrorcode: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdstptc.dll" "system" fn WdsTransportClientCancelSessionEx(hsessionkey : super::super::Foundation:: HANDLE, dwerrorcode : u32) -> u32);
    WdsTransportClientCancelSessionEx(hsessionkey.into_param().abi(), dwerrorcode)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientCloseSession<P0>(hsessionkey: P0) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdstptc.dll" "system" fn WdsTransportClientCloseSession(hsessionkey : super::super::Foundation:: HANDLE) -> u32);
    WdsTransportClientCloseSession(hsessionkey.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientCompleteReceive<P0>(hsessionkey: P0, ulsize: u32, pulloffset: *const u64) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdstptc.dll" "system" fn WdsTransportClientCompleteReceive(hsessionkey : super::super::Foundation:: HANDLE, ulsize : u32, pulloffset : *const u64) -> u32);
    WdsTransportClientCompleteReceive(hsessionkey.into_param().abi(), ulsize, pulloffset)
}
#[inline]
pub unsafe fn WdsTransportClientInitialize() -> u32 {
    ::windows_targets::link!("wdstptc.dll" "system" fn WdsTransportClientInitialize() -> u32);
    WdsTransportClientInitialize()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientInitializeSession(psessionrequest: *const WDS_TRANSPORTCLIENT_REQUEST, pcallerdata: *const ::core::ffi::c_void, hsessionkey: *mut super::super::Foundation::HANDLE) -> u32 {
    ::windows_targets::link!("wdstptc.dll" "system" fn WdsTransportClientInitializeSession(psessionrequest : *const WDS_TRANSPORTCLIENT_REQUEST, pcallerdata : *const ::core::ffi::c_void, hsessionkey : *mut super::super::Foundation:: HANDLE) -> u32);
    WdsTransportClientInitializeSession(psessionrequest, pcallerdata, hsessionkey)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientQueryStatus<P0>(hsessionkey: P0, pustatus: *mut u32, puerrorcode: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdstptc.dll" "system" fn WdsTransportClientQueryStatus(hsessionkey : super::super::Foundation:: HANDLE, pustatus : *mut u32, puerrorcode : *mut u32) -> u32);
    WdsTransportClientQueryStatus(hsessionkey.into_param().abi(), pustatus, puerrorcode)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientRegisterCallback<P0>(hsessionkey: P0, callbackid: TRANSPORTCLIENT_CALLBACK_ID, pfncallback: *const ::core::ffi::c_void) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdstptc.dll" "system" fn WdsTransportClientRegisterCallback(hsessionkey : super::super::Foundation:: HANDLE, callbackid : TRANSPORTCLIENT_CALLBACK_ID, pfncallback : *const ::core::ffi::c_void) -> u32);
    WdsTransportClientRegisterCallback(hsessionkey.into_param().abi(), callbackid, pfncallback)
}
#[inline]
pub unsafe fn WdsTransportClientReleaseBuffer(pvbuffer: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link!("wdstptc.dll" "system" fn WdsTransportClientReleaseBuffer(pvbuffer : *const ::core::ffi::c_void) -> u32);
    WdsTransportClientReleaseBuffer(pvbuffer)
}
#[inline]
pub unsafe fn WdsTransportClientShutdown() -> u32 {
    ::windows_targets::link!("wdstptc.dll" "system" fn WdsTransportClientShutdown() -> u32);
    WdsTransportClientShutdown()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientStartSession<P0>(hsessionkey: P0) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdstptc.dll" "system" fn WdsTransportClientStartSession(hsessionkey : super::super::Foundation:: HANDLE) -> u32);
    WdsTransportClientStartSession(hsessionkey.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportClientWaitForCompletion<P0>(hsessionkey: P0, utimeout: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdstptc.dll" "system" fn WdsTransportClientWaitForCompletion(hsessionkey : super::super::Foundation:: HANDLE, utimeout : u32) -> u32);
    WdsTransportClientWaitForCompletion(hsessionkey.into_param().abi(), utimeout)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportServerAllocateBuffer<P0>(hprovider: P0, ulbuffersize: u32) -> *mut ::core::ffi::c_void
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsmc.dll" "system" fn WdsTransportServerAllocateBuffer(hprovider : super::super::Foundation:: HANDLE, ulbuffersize : u32) -> *mut ::core::ffi::c_void);
    WdsTransportServerAllocateBuffer(hprovider.into_param().abi(), ulbuffersize)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportServerCompleteRead<P0>(hprovider: P0, ulbytesread: u32, pvuserdata: *const ::core::ffi::c_void, hreadresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsmc.dll" "system" fn WdsTransportServerCompleteRead(hprovider : super::super::Foundation:: HANDLE, ulbytesread : u32, pvuserdata : *const ::core::ffi::c_void, hreadresult : ::windows_core::HRESULT) -> ::windows_core::HRESULT);
    WdsTransportServerCompleteRead(hprovider.into_param().abi(), ulbytesread, pvuserdata, hreadresult).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportServerFreeBuffer<P0>(hprovider: P0, pvbuffer: *const ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsmc.dll" "system" fn WdsTransportServerFreeBuffer(hprovider : super::super::Foundation:: HANDLE, pvbuffer : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    WdsTransportServerFreeBuffer(hprovider.into_param().abi(), pvbuffer).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportServerRegisterCallback<P0>(hprovider: P0, callbackid: TRANSPORTPROVIDER_CALLBACK_ID, pfncallback: *const ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wdsmc.dll" "system" fn WdsTransportServerRegisterCallback(hprovider : super::super::Foundation:: HANDLE, callbackid : TRANSPORTPROVIDER_CALLBACK_ID, pfncallback : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    WdsTransportServerRegisterCallback(hprovider.into_param().abi(), callbackid, pfncallback).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportServerTrace<P0, P1>(hprovider: P0, severity: u32, pwszformat: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wdsmc.dll" "cdecl" fn WdsTransportServerTrace(hprovider : super::super::Foundation:: HANDLE, severity : u32, pwszformat : ::windows_core::PCWSTR) -> ::windows_core::HRESULT);
    WdsTransportServerTrace(hprovider.into_param().abi(), severity, pwszformat.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WdsTransportServerTraceV<P0, P1>(hprovider: P0, severity: u32, pwszformat: P1, params: *const i8) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wdsmc.dll" "system" fn WdsTransportServerTraceV(hprovider : super::super::Foundation:: HANDLE, severity : u32, pwszformat : ::windows_core::PCWSTR, params : *const i8) -> ::windows_core::HRESULT);
    WdsTransportServerTraceV(hprovider.into_param().abi(), severity, pwszformat.into_param().abi(), params).ok()
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportCacheable(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportCacheable {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Dirty(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Dirty)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Discard(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Discard)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportCacheable, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportCacheable {
    type Vtable = IWdsTransportCacheable_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportCacheable {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46ad894b_0bab_47dc_84b2_7b553f1d8f80);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportCacheable_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Dirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdirty: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Dirty: usize,
    pub Discard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportClient(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportClient {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Session(&self) -> ::windows_core::Result<IWdsTransportSession> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Session)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MacAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MacAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IpAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IpAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PercentCompletion(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PercentCompletion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn JoinDuration(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).JoinDuration)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CpuUtilization(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CpuUtilization)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MemoryUtilization(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MemoryUtilization)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NetworkUtilization(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NetworkUtilization)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserIdentity(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UserIdentity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Disconnect(&self, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Disconnect)(::windows_core::Interface::as_raw(self), disconnectiontype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportClient, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportClient {
    type Vtable = IWdsTransportClient_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportClient {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5dbc93a_cabe_46ca_837f_3e44e93c6545);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportClient_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwdstransportsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Session: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub MacAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszmacaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub IpAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszipaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PercentCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulpercentcompletion: *mut u32) -> ::windows_core::HRESULT,
    pub JoinDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puljoinduration: *mut u32) -> ::windows_core::HRESULT,
    pub CpuUtilization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulcpuutilization: *mut u32) -> ::windows_core::HRESULT,
    pub MemoryUtilization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulmemoryutilization: *mut u32) -> ::windows_core::HRESULT,
    pub NetworkUtilization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulnetworkutilization: *mut u32) -> ::windows_core::HRESULT,
    pub UserIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszuseridentity: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, ulindex: u32) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ulindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportCollection, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportCollection {
    type Vtable = IWdsTransportCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8ba4b1a_2ff4_43ab_996c_b2b10a91a6eb);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportConfigurationManager(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportConfigurationManager {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ServicePolicy(&self) -> ::windows_core::Result<IWdsTransportServicePolicy> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ServicePolicy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DiagnosticsPolicy(&self) -> ::windows_core::Result<IWdsTransportDiagnosticsPolicy> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DiagnosticsPolicy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_WdsTransportServicesRunning<P0>(&self, brealtimestatus: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_WdsTransportServicesRunning)(::windows_core::Interface::as_raw(self), brealtimestatus.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnableWdsTransportServices(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableWdsTransportServices)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableWdsTransportServices(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableWdsTransportServices)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartWdsTransportServices(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartWdsTransportServices)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StopWdsTransportServices(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopWdsTransportServices)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RestartWdsTransportServices(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RestartWdsTransportServices)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn NotifyWdsTransportServices(&self, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyWdsTransportServices)(::windows_core::Interface::as_raw(self), servicenotification).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportConfigurationManager, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportConfigurationManager {
    type Vtable = IWdsTransportConfigurationManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportConfigurationManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84cc4779_42dd_4792_891e_1321d6d74b44);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportConfigurationManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ServicePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwdstransportservicepolicy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ServicePolicy: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DiagnosticsPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwdstransportdiagnosticspolicy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DiagnosticsPolicy: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_WdsTransportServicesRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brealtimestatus: super::super::Foundation::VARIANT_BOOL, pbservicesrunning: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_WdsTransportServicesRunning: usize,
    pub EnableWdsTransportServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DisableWdsTransportServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartWdsTransportServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StopWdsTransportServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RestartWdsTransportServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyWdsTransportServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportConfigurationManager2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportConfigurationManager2 {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ServicePolicy(&self) -> ::windows_core::Result<IWdsTransportServicePolicy> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ServicePolicy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DiagnosticsPolicy(&self) -> ::windows_core::Result<IWdsTransportDiagnosticsPolicy> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DiagnosticsPolicy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_WdsTransportServicesRunning<P0>(&self, brealtimestatus: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_WdsTransportServicesRunning)(::windows_core::Interface::as_raw(self), brealtimestatus.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnableWdsTransportServices(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EnableWdsTransportServices)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableWdsTransportServices(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DisableWdsTransportServices)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartWdsTransportServices(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartWdsTransportServices)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StopWdsTransportServices(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StopWdsTransportServices)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RestartWdsTransportServices(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RestartWdsTransportServices)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn NotifyWdsTransportServices(&self, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.NotifyWdsTransportServices)(::windows_core::Interface::as_raw(self), servicenotification).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MulticastSessionPolicy(&self) -> ::windows_core::Result<IWdsTransportMulticastSessionPolicy> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MulticastSessionPolicy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportConfigurationManager2, ::windows_core::IUnknown, super::Com::IDispatch, IWdsTransportConfigurationManager);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportConfigurationManager2 {
    type Vtable = IWdsTransportConfigurationManager2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportConfigurationManager2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0d85caf_a153_4f1d_a9dd_96f431c50717);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportConfigurationManager2_Vtbl {
    pub base__: IWdsTransportConfigurationManager_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub MulticastSessionPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwdstransportmulticastsessionpolicy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MulticastSessionPolicy: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportContent(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportContent {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Namespace(&self) -> ::windows_core::Result<IWdsTransportNamespace> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Namespace)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RetrieveSessions(&self) -> ::windows_core::Result<IWdsTransportCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RetrieveSessions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Terminate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Terminate)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportContent, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportContent {
    type Vtable = IWdsTransportContent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportContent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd405d711_0296_4ab4_a860_ac7d32e65798);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportContent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Namespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwdstransportnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Namespace: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RetrieveSessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwdstransportsessions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RetrieveSessions: usize,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportContentProvider(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportContentProvider {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FilePath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FilePath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InitializationRoutine(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InitializationRoutine)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportContentProvider, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportContentProvider {
    type Vtable = IWdsTransportContentProvider_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportContentProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9489f24_f219_4acf_aad7_265c7c08a6ae);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportContentProvider_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszfilepath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub InitializationRoutine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszinitializationroutine: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportDiagnosticsPolicy(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportDiagnosticsPolicy {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Dirty(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Dirty)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Discard(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Discard)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, benabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
    pub unsafe fn Components(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Components)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetComponents(&self, ulcomponents: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetComponents)(::windows_core::Interface::as_raw(self), ulcomponents).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportDiagnosticsPolicy, ::windows_core::IUnknown, super::Com::IDispatch, IWdsTransportCacheable);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportDiagnosticsPolicy {
    type Vtable = IWdsTransportDiagnosticsPolicy_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportDiagnosticsPolicy {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13b33efc_7856_4f61_9a59_8de67b6b87b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportDiagnosticsPolicy_Vtbl {
    pub base__: IWdsTransportCacheable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
    pub Components: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulcomponents: *mut u32) -> ::windows_core::HRESULT,
    pub SetComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcomponents: u32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportManager(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportManager {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWdsTransportServer<P0>(&self, bszservername: P0) -> ::windows_core::Result<IWdsTransportServer>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetWdsTransportServer)(::windows_core::Interface::as_raw(self), bszservername.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportManager, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportManager {
    type Vtable = IWdsTransportManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b0d35f5_1b13_4afd_b878_6526dc340b5d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWdsTransportServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bszservername: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppwdstransportserver: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWdsTransportServer: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportMulticastSessionPolicy(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportMulticastSessionPolicy {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Dirty(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Dirty)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Discard(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Discard)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SlowClientHandling(&self) -> ::windows_core::Result<WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SlowClientHandling)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSlowClientHandling(&self, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSlowClientHandling)(::windows_core::Interface::as_raw(self), slowclienthandling).ok()
    }
    pub unsafe fn AutoDisconnectThreshold(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AutoDisconnectThreshold)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAutoDisconnectThreshold(&self, ulthreshold: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAutoDisconnectThreshold)(::windows_core::Interface::as_raw(self), ulthreshold).ok()
    }
    pub unsafe fn MultistreamStreamCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MultistreamStreamCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMultistreamStreamCount(&self, ulstreamcount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMultistreamStreamCount)(::windows_core::Interface::as_raw(self), ulstreamcount).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SlowClientFallback(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SlowClientFallback)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSlowClientFallback<P0>(&self, bclientfallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetSlowClientFallback)(::windows_core::Interface::as_raw(self), bclientfallback.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportMulticastSessionPolicy, ::windows_core::IUnknown, super::Com::IDispatch, IWdsTransportCacheable);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportMulticastSessionPolicy {
    type Vtable = IWdsTransportMulticastSessionPolicy_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportMulticastSessionPolicy {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e5753cf_68ec_4504_a951_4a003266606b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportMulticastSessionPolicy_Vtbl {
    pub base__: IWdsTransportCacheable_Vtbl,
    pub SlowClientHandling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pslowclienthandling: *mut WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows_core::HRESULT,
    pub SetSlowClientHandling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows_core::HRESULT,
    pub AutoDisconnectThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulthreshold: *mut u32) -> ::windows_core::HRESULT,
    pub SetAutoDisconnectThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulthreshold: u32) -> ::windows_core::HRESULT,
    pub MultistreamStreamCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulstreamcount: *mut u32) -> ::windows_core::HRESULT,
    pub SetMultistreamStreamCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulstreamcount: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SlowClientFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbclientfallback: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SlowClientFallback: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSlowClientFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bclientfallback: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSlowClientFallback: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportNamespace(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespace {
    pub unsafe fn Type(&self) -> ::windows_core::Result<WDSTRANSPORT_NAMESPACE_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bszname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), bszname.into_param().abi()).ok()
    }
    pub unsafe fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FriendlyName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFriendlyName<P0>(&self, bszfriendlyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFriendlyName)(::windows_core::Interface::as_raw(self), bszfriendlyname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bszdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bszdescription.into_param().abi()).ok()
    }
    pub unsafe fn ContentProvider(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ContentProvider)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetContentProvider<P0>(&self, bszcontentprovider: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetContentProvider)(::windows_core::Interface::as_raw(self), bszcontentprovider.into_param().abi()).ok()
    }
    pub unsafe fn Configuration(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Configuration)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetConfiguration<P0>(&self, bszconfiguration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetConfiguration)(::windows_core::Interface::as_raw(self), bszconfiguration.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Registered(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Registered)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Tombstoned(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Tombstoned)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TombstoneTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TombstoneTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransmissionStarted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TransmissionStarted)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Register(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Register)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deregister<P0>(&self, bterminatesessions: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).Deregister)(::windows_core::Interface::as_raw(self), bterminatesessions.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IWdsTransportNamespace> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RetrieveContents(&self) -> ::windows_core::Result<IWdsTransportCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RetrieveContents)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportNamespace, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportNamespace {
    type Vtable = IWdsTransportNamespace_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportNamespace {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa561f57_fbef_4ed3_b056_127cb1b33b84);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespace_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WDSTRANSPORT_NAMESPACE_TYPE) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bszname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszfriendlyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bszfriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bszdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ContentProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszcontentprovider: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetContentProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bszcontentprovider: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszconfiguration: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bszconfiguration: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Registered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbregistered: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Registered: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Tombstoned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbtombstoned: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Tombstoned: usize,
    pub TombstoneTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptombstonetime: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TransmissionStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbtransmissionstarted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransmissionStarted: usize,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Deregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bterminatesessions: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Deregister: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwdstransportnamespaceclone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RetrieveContents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwdstransportcontents: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RetrieveContents: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportNamespaceAutoCast(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceAutoCast {
    pub unsafe fn Type(&self) -> ::windows_core::Result<WDSTRANSPORT_NAMESPACE_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bszname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), bszname.into_param().abi()).ok()
    }
    pub unsafe fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FriendlyName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFriendlyName<P0>(&self, bszfriendlyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetFriendlyName)(::windows_core::Interface::as_raw(self), bszfriendlyname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bszdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), bszdescription.into_param().abi()).ok()
    }
    pub unsafe fn ContentProvider(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ContentProvider)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetContentProvider<P0>(&self, bszcontentprovider: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetContentProvider)(::windows_core::Interface::as_raw(self), bszcontentprovider.into_param().abi()).ok()
    }
    pub unsafe fn Configuration(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Configuration)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetConfiguration<P0>(&self, bszconfiguration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetConfiguration)(::windows_core::Interface::as_raw(self), bszconfiguration.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Registered(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Registered)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Tombstoned(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Tombstoned)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TombstoneTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TombstoneTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransmissionStarted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TransmissionStarted)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Register(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Register)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deregister<P0>(&self, bterminatesessions: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.Deregister)(::windows_core::Interface::as_raw(self), bterminatesessions.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IWdsTransportNamespace> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RetrieveContents(&self) -> ::windows_core::Result<IWdsTransportCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RetrieveContents)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportNamespaceAutoCast, ::windows_core::IUnknown, super::Com::IDispatch, IWdsTransportNamespace);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportNamespaceAutoCast {
    type Vtable = IWdsTransportNamespaceAutoCast_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportNamespaceAutoCast {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad931a72_c4bd_4c41_8fbc_59c9c748df9e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespaceAutoCast_Vtbl {
    pub base__: IWdsTransportNamespace_Vtbl,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportNamespaceManager(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceManager {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateNamespace<P0, P1, P2>(&self, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: P0, bszcontentprovider: P1, bszconfiguration: P2) -> ::windows_core::Result<IWdsTransportNamespace>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateNamespace)(::windows_core::Interface::as_raw(self), namespacetype, bsznamespacename.into_param().abi(), bszcontentprovider.into_param().abi(), bszconfiguration.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RetrieveNamespace<P0>(&self, bsznamespacename: P0) -> ::windows_core::Result<IWdsTransportNamespace>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RetrieveNamespace)(::windows_core::Interface::as_raw(self), bsznamespacename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn RetrieveNamespaces<P0, P1, P2>(&self, bszcontentprovider: P0, bsznamespacename: P1, bincludetombstones: P2) -> ::windows_core::Result<IWdsTransportCollection>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RetrieveNamespaces)(::windows_core::Interface::as_raw(self), bszcontentprovider.into_param().abi(), bsznamespacename.into_param().abi(), bincludetombstones.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportNamespaceManager, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportNamespaceManager {
    type Vtable = IWdsTransportNamespaceManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportNamespaceManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e22d9f6_3777_4d98_83e1_f98696717ba3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespaceManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bszcontentprovider: ::std::mem::MaybeUninit<::windows_core::BSTR>, bszconfiguration: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppwdstransportnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateNamespace: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RetrieveNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsznamespacename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppwdstransportnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RetrieveNamespace: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RetrieveNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bszcontentprovider: ::std::mem::MaybeUninit<::windows_core::BSTR>, bsznamespacename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bincludetombstones: super::super::Foundation::VARIANT_BOOL, ppwdstransportnamespaces: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RetrieveNamespaces: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportNamespaceScheduledCast(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceScheduledCast {
    pub unsafe fn Type(&self) -> ::windows_core::Result<WDSTRANSPORT_NAMESPACE_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bszname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), bszname.into_param().abi()).ok()
    }
    pub unsafe fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FriendlyName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFriendlyName<P0>(&self, bszfriendlyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetFriendlyName)(::windows_core::Interface::as_raw(self), bszfriendlyname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bszdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), bszdescription.into_param().abi()).ok()
    }
    pub unsafe fn ContentProvider(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ContentProvider)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetContentProvider<P0>(&self, bszcontentprovider: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetContentProvider)(::windows_core::Interface::as_raw(self), bszcontentprovider.into_param().abi()).ok()
    }
    pub unsafe fn Configuration(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Configuration)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetConfiguration<P0>(&self, bszconfiguration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetConfiguration)(::windows_core::Interface::as_raw(self), bszconfiguration.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Registered(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Registered)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Tombstoned(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Tombstoned)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TombstoneTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TombstoneTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransmissionStarted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TransmissionStarted)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Register(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Register)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deregister<P0>(&self, bterminatesessions: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.Deregister)(::windows_core::Interface::as_raw(self), bterminatesessions.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IWdsTransportNamespace> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RetrieveContents(&self) -> ::windows_core::Result<IWdsTransportCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RetrieveContents)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StartTransmission(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartTransmission)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportNamespaceScheduledCast, ::windows_core::IUnknown, super::Com::IDispatch, IWdsTransportNamespace);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportNamespaceScheduledCast {
    type Vtable = IWdsTransportNamespaceScheduledCast_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportNamespaceScheduledCast {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3840cecf_d76c_416e_a4cc_31c741d2874b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespaceScheduledCast_Vtbl {
    pub base__: IWdsTransportNamespace_Vtbl,
    pub StartTransmission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportNamespaceScheduledCastAutoStart(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceScheduledCastAutoStart {
    pub unsafe fn Type(&self) -> ::windows_core::Result<WDSTRANSPORT_NAMESPACE_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Type)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bszname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetName)(::windows_core::Interface::as_raw(self), bszname.into_param().abi()).ok()
    }
    pub unsafe fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.FriendlyName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFriendlyName<P0>(&self, bszfriendlyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetFriendlyName)(::windows_core::Interface::as_raw(self), bszfriendlyname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bszdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), bszdescription.into_param().abi()).ok()
    }
    pub unsafe fn ContentProvider(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ContentProvider)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetContentProvider<P0>(&self, bszcontentprovider: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetContentProvider)(::windows_core::Interface::as_raw(self), bszcontentprovider.into_param().abi()).ok()
    }
    pub unsafe fn Configuration(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Configuration)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetConfiguration<P0>(&self, bszconfiguration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetConfiguration)(::windows_core::Interface::as_raw(self), bszconfiguration.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Registered(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Registered)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Tombstoned(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Tombstoned)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TombstoneTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.TombstoneTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransmissionStarted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.TransmissionStarted)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Register(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Register)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deregister<P0>(&self, bterminatesessions: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Deregister)(::windows_core::Interface::as_raw(self), bterminatesessions.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IWdsTransportNamespace> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RetrieveContents(&self) -> ::windows_core::Result<IWdsTransportCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RetrieveContents)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StartTransmission(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartTransmission)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MinimumClients(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MinimumClients)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMinimumClients(&self, ulminimumclients: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMinimumClients)(::windows_core::Interface::as_raw(self), ulminimumclients).ok()
    }
    pub unsafe fn StartTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStartTime(&self, starttime: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStartTime)(::windows_core::Interface::as_raw(self), starttime).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportNamespaceScheduledCastAutoStart, ::windows_core::IUnknown, super::Com::IDispatch, IWdsTransportNamespace, IWdsTransportNamespaceScheduledCast);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportNamespaceScheduledCastAutoStart {
    type Vtable = IWdsTransportNamespaceScheduledCastAutoStart_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportNamespaceScheduledCastAutoStart {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd606af3d_ea9c_4219_961e_7491d618d9b9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespaceScheduledCastAutoStart_Vtbl {
    pub base__: IWdsTransportNamespaceScheduledCast_Vtbl,
    pub MinimumClients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulminimumclients: *mut u32) -> ::windows_core::HRESULT,
    pub SetMinimumClients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulminimumclients: u32) -> ::windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstarttime: *mut f64) -> ::windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: f64) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportNamespaceScheduledCastManualStart(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceScheduledCastManualStart {
    pub unsafe fn Type(&self) -> ::windows_core::Result<WDSTRANSPORT_NAMESPACE_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Type)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bszname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetName)(::windows_core::Interface::as_raw(self), bszname.into_param().abi()).ok()
    }
    pub unsafe fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.FriendlyName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFriendlyName<P0>(&self, bszfriendlyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetFriendlyName)(::windows_core::Interface::as_raw(self), bszfriendlyname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bszdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), bszdescription.into_param().abi()).ok()
    }
    pub unsafe fn ContentProvider(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ContentProvider)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetContentProvider<P0>(&self, bszcontentprovider: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetContentProvider)(::windows_core::Interface::as_raw(self), bszcontentprovider.into_param().abi()).ok()
    }
    pub unsafe fn Configuration(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Configuration)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetConfiguration<P0>(&self, bszconfiguration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetConfiguration)(::windows_core::Interface::as_raw(self), bszconfiguration.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Registered(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Registered)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Tombstoned(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Tombstoned)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TombstoneTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.TombstoneTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransmissionStarted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.TransmissionStarted)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Register(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Register)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deregister<P0>(&self, bterminatesessions: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Deregister)(::windows_core::Interface::as_raw(self), bterminatesessions.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IWdsTransportNamespace> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RetrieveContents(&self) -> ::windows_core::Result<IWdsTransportCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RetrieveContents)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StartTransmission(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartTransmission)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportNamespaceScheduledCastManualStart, ::windows_core::IUnknown, super::Com::IDispatch, IWdsTransportNamespace, IWdsTransportNamespaceScheduledCast);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportNamespaceScheduledCastManualStart {
    type Vtable = IWdsTransportNamespaceScheduledCastManualStart_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportNamespaceScheduledCastManualStart {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x013e6e4c_e6a7_4fb5_b7ff_d9f5da805c31);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespaceScheduledCastManualStart_Vtbl {
    pub base__: IWdsTransportNamespaceScheduledCast_Vtbl,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportServer(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportServer {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetupManager(&self) -> ::windows_core::Result<IWdsTransportSetupManager> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SetupManager)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ConfigurationManager(&self) -> ::windows_core::Result<IWdsTransportConfigurationManager> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ConfigurationManager)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceManager(&self) -> ::windows_core::Result<IWdsTransportNamespaceManager> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NamespaceManager)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DisconnectClient(&self, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisconnectClient)(::windows_core::Interface::as_raw(self), ulclientid, disconnectiontype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportServer, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportServer {
    type Vtable = IWdsTransportServer_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportServer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09ccd093_830d_4344_a30a_73ae8e8fca90);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportServer_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetupManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwdstransportsetupmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetupManager: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ConfigurationManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwdstransportconfigurationmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ConfigurationManager: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NamespaceManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwdstransportnamespacemanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NamespaceManager: usize,
    pub DisconnectClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportServer2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportServer2 {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetupManager(&self) -> ::windows_core::Result<IWdsTransportSetupManager> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SetupManager)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ConfigurationManager(&self) -> ::windows_core::Result<IWdsTransportConfigurationManager> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ConfigurationManager)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceManager(&self) -> ::windows_core::Result<IWdsTransportNamespaceManager> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.NamespaceManager)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DisconnectClient(&self, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DisconnectClient)(::windows_core::Interface::as_raw(self), ulclientid, disconnectiontype).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TftpManager(&self) -> ::windows_core::Result<IWdsTransportTftpManager> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TftpManager)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportServer2, ::windows_core::IUnknown, super::Com::IDispatch, IWdsTransportServer);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportServer2 {
    type Vtable = IWdsTransportServer2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportServer2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x256e999f_6df4_4538_81b9_857b9ab8fb47);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportServer2_Vtbl {
    pub base__: IWdsTransportServer_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub TftpManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwdstransporttftpmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TftpManager: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportServicePolicy(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportServicePolicy {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Dirty(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Dirty)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Discard(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Discard)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn get_IpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows_core::Result<WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_IpAddressSource)(::windows_core::Interface::as_raw(self), addresstype, &mut result__).from_abi(result__)
    }
    pub unsafe fn put_IpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_IpAddressSource)(::windows_core::Interface::as_raw(self), addresstype, sourcetype).ok()
    }
    pub unsafe fn get_StartIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_StartIpAddress)(::windows_core::Interface::as_raw(self), addresstype, &mut result__).from_abi(result__)
    }
    pub unsafe fn put_StartIpAddress<P0>(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).put_StartIpAddress)(::windows_core::Interface::as_raw(self), addresstype, bszstartipaddress.into_param().abi()).ok()
    }
    pub unsafe fn get_EndIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_EndIpAddress)(::windows_core::Interface::as_raw(self), addresstype, &mut result__).from_abi(result__)
    }
    pub unsafe fn put_EndIpAddress<P0>(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).put_EndIpAddress)(::windows_core::Interface::as_raw(self), addresstype, bszendipaddress.into_param().abi()).ok()
    }
    pub unsafe fn StartPort(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStartPort(&self, ulstartport: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStartPort)(::windows_core::Interface::as_raw(self), ulstartport).ok()
    }
    pub unsafe fn EndPort(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEndPort(&self, ulendport: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEndPort)(::windows_core::Interface::as_raw(self), ulendport).ok()
    }
    pub unsafe fn NetworkProfile(&self) -> ::windows_core::Result<WDSTRANSPORT_NETWORK_PROFILE_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NetworkProfile)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNetworkProfile(&self, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNetworkProfile)(::windows_core::Interface::as_raw(self), profiletype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportServicePolicy, ::windows_core::IUnknown, super::Com::IDispatch, IWdsTransportCacheable);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportServicePolicy {
    type Vtable = IWdsTransportServicePolicy_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportServicePolicy {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9468578_9f2b_48cc_b27a_a60799c2750c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportServicePolicy_Vtbl {
    pub base__: IWdsTransportCacheable_Vtbl,
    pub get_IpAddressSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, psourcetype: *mut WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows_core::HRESULT,
    pub put_IpAddressSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows_core::HRESULT,
    pub get_StartIpAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszstartipaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub put_StartIpAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub get_EndIpAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszendipaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub put_EndIpAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub StartPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulstartport: *mut u32) -> ::windows_core::HRESULT,
    pub SetStartPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulstartport: u32) -> ::windows_core::HRESULT,
    pub EndPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulendport: *mut u32) -> ::windows_core::HRESULT,
    pub SetEndPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulendport: u32) -> ::windows_core::HRESULT,
    pub NetworkProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprofiletype: *mut WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows_core::HRESULT,
    pub SetNetworkProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportServicePolicy2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportServicePolicy2 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Dirty(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Dirty)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Discard(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Discard)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn get_IpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows_core::Result<WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_IpAddressSource)(::windows_core::Interface::as_raw(self), addresstype, &mut result__).from_abi(result__)
    }
    pub unsafe fn put_IpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.put_IpAddressSource)(::windows_core::Interface::as_raw(self), addresstype, sourcetype).ok()
    }
    pub unsafe fn get_StartIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_StartIpAddress)(::windows_core::Interface::as_raw(self), addresstype, &mut result__).from_abi(result__)
    }
    pub unsafe fn put_StartIpAddress<P0>(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.put_StartIpAddress)(::windows_core::Interface::as_raw(self), addresstype, bszstartipaddress.into_param().abi()).ok()
    }
    pub unsafe fn get_EndIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_EndIpAddress)(::windows_core::Interface::as_raw(self), addresstype, &mut result__).from_abi(result__)
    }
    pub unsafe fn put_EndIpAddress<P0>(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.put_EndIpAddress)(::windows_core::Interface::as_raw(self), addresstype, bszendipaddress.into_param().abi()).ok()
    }
    pub unsafe fn StartPort(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.StartPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStartPort(&self, ulstartport: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStartPort)(::windows_core::Interface::as_raw(self), ulstartport).ok()
    }
    pub unsafe fn EndPort(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EndPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEndPort(&self, ulendport: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEndPort)(::windows_core::Interface::as_raw(self), ulendport).ok()
    }
    pub unsafe fn NetworkProfile(&self) -> ::windows_core::Result<WDSTRANSPORT_NETWORK_PROFILE_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.NetworkProfile)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNetworkProfile(&self, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetNetworkProfile)(::windows_core::Interface::as_raw(self), profiletype).ok()
    }
    pub unsafe fn UdpPortPolicy(&self) -> ::windows_core::Result<WDSTRANSPORT_UDP_PORT_POLICY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UdpPortPolicy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUdpPortPolicy(&self, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUdpPortPolicy)(::windows_core::Interface::as_raw(self), udpportpolicy).ok()
    }
    pub unsafe fn TftpMaximumBlockSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TftpMaximumBlockSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTftpMaximumBlockSize(&self, ultftpmaximumblocksize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTftpMaximumBlockSize)(::windows_core::Interface::as_raw(self), ultftpmaximumblocksize).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableTftpVariableWindowExtension(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnableTftpVariableWindowExtension)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableTftpVariableWindowExtension<P0>(&self, benabletftpvariablewindowextension: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetEnableTftpVariableWindowExtension)(::windows_core::Interface::as_raw(self), benabletftpvariablewindowextension.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportServicePolicy2, ::windows_core::IUnknown, super::Com::IDispatch, IWdsTransportCacheable, IWdsTransportServicePolicy);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportServicePolicy2 {
    type Vtable = IWdsTransportServicePolicy2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportServicePolicy2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65c19e5c_aa7e_4b91_8944_91e0e5572797);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportServicePolicy2_Vtbl {
    pub base__: IWdsTransportServicePolicy_Vtbl,
    pub UdpPortPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pudpportpolicy: *mut WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows_core::HRESULT,
    pub SetUdpPortPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows_core::HRESULT,
    pub TftpMaximumBlockSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pultftpmaximumblocksize: *mut u32) -> ::windows_core::HRESULT,
    pub SetTftpMaximumBlockSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ultftpmaximumblocksize: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableTftpVariableWindowExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabletftpvariablewindowextension: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableTftpVariableWindowExtension: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableTftpVariableWindowExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabletftpvariablewindowextension: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableTftpVariableWindowExtension: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportSession(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportSession {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Content(&self) -> ::windows_core::Result<IWdsTransportContent> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Content)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NetworkInterfaceName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NetworkInterfaceName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NetworkInterfaceAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NetworkInterfaceAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TransferRate(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TransferRate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MasterClientId(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MasterClientId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RetrieveClients(&self) -> ::windows_core::Result<IWdsTransportCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RetrieveClients)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Terminate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Terminate)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportSession, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportSession {
    type Vtable = IWdsTransportSession_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4efea88_65b1_4f30_a4b9_2793987796fb);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportSession_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwdstransportcontent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Content: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows_core::HRESULT,
    pub NetworkInterfaceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsznetworkinterfacename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub NetworkInterfaceAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsznetworkinterfaceaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TransferRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pultransferrate: *mut u32) -> ::windows_core::HRESULT,
    pub MasterClientId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulmasterclientid: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RetrieveClients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwdstransportclients: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RetrieveClients: usize,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportSetupManager(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportSetupManager {
    pub unsafe fn Version(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Version)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InstalledFeatures(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InstalledFeatures)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Protocols(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Protocols)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterContentProvider<P0, P1, P2, P3>(&self, bszname: P0, bszdescription: P1, bszfilepath: P2, bszinitializationroutine: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RegisterContentProvider)(::windows_core::Interface::as_raw(self), bszname.into_param().abi(), bszdescription.into_param().abi(), bszfilepath.into_param().abi(), bszinitializationroutine.into_param().abi()).ok()
    }
    pub unsafe fn DeregisterContentProvider<P0>(&self, bszname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).DeregisterContentProvider)(::windows_core::Interface::as_raw(self), bszname.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportSetupManager, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportSetupManager {
    type Vtable = IWdsTransportSetupManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportSetupManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7238425_efa8_40a4_aef9_c98d969c0b75);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportSetupManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullversion: *mut u64) -> ::windows_core::HRESULT,
    pub InstalledFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulinstalledfeatures: *mut u32) -> ::windows_core::HRESULT,
    pub Protocols: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulprotocols: *mut u32) -> ::windows_core::HRESULT,
    pub RegisterContentProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bszname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bszdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>, bszfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, bszinitializationroutine: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DeregisterContentProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bszname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportSetupManager2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportSetupManager2 {
    pub unsafe fn Version(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Version)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InstalledFeatures(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.InstalledFeatures)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Protocols(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Protocols)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterContentProvider<P0, P1, P2, P3>(&self, bszname: P0, bszdescription: P1, bszfilepath: P2, bszinitializationroutine: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.RegisterContentProvider)(::windows_core::Interface::as_raw(self), bszname.into_param().abi(), bszdescription.into_param().abi(), bszfilepath.into_param().abi(), bszinitializationroutine.into_param().abi()).ok()
    }
    pub unsafe fn DeregisterContentProvider<P0>(&self, bszname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.DeregisterContentProvider)(::windows_core::Interface::as_raw(self), bszname.into_param().abi()).ok()
    }
    pub unsafe fn TftpCapabilities(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TftpCapabilities)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ContentProviders(&self) -> ::windows_core::Result<IWdsTransportCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ContentProviders)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportSetupManager2, ::windows_core::IUnknown, super::Com::IDispatch, IWdsTransportSetupManager);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportSetupManager2 {
    type Vtable = IWdsTransportSetupManager2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportSetupManager2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02be79da_7e9e_4366_8b6e_2aa9a91be47f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportSetupManager2_Vtbl {
    pub base__: IWdsTransportSetupManager_Vtbl,
    pub TftpCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pultftpcapabilities: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ContentProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprovidercollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ContentProviders: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportTftpClient(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportTftpClient {
    pub unsafe fn FileName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FileName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IpAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IpAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Timeout(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Timeout)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentFileOffset(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentFileOffset)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FileSize(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FileSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BlockSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BlockSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn WindowSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).WindowSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportTftpClient, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportTftpClient {
    type Vtable = IWdsTransportTftpClient_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportTftpClient {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb022d3ae_884d_4d85_b146_53320e76ef62);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportTftpClient_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub FileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszfilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub IpAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbszipaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Timeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pultimeout: *mut u32) -> ::windows_core::HRESULT,
    pub CurrentFileOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pul64currentoffset: *mut u64) -> ::windows_core::HRESULT,
    pub FileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pul64filesize: *mut u64) -> ::windows_core::HRESULT,
    pub BlockSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulblocksize: *mut u32) -> ::windows_core::HRESULT,
    pub WindowSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulwindowsize: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWdsTransportTftpManager(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportTftpManager {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RetrieveTftpClients(&self) -> ::windows_core::Result<IWdsTransportCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RetrieveTftpClients)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWdsTransportTftpManager, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWdsTransportTftpManager {
    type Vtable = IWdsTransportTftpManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWdsTransportTftpManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1327a7c8_ae8a_4fb3_8150_136227c37e9a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportTftpManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RetrieveTftpClients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwdstransporttftpclients: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RetrieveTftpClients: usize,
}
pub const CPU_ARCHITECTURE_AMD64: CPU_ARCHITECTURE = CPU_ARCHITECTURE(9u32);
pub const CPU_ARCHITECTURE_IA64: CPU_ARCHITECTURE = CPU_ARCHITECTURE(6u32);
pub const CPU_ARCHITECTURE_INTEL: CPU_ARCHITECTURE = CPU_ARCHITECTURE(0u32);
pub const EVT_WDSMCS_E_CP_CALLBACKS_NOT_REG: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801324i32);
pub const EVT_WDSMCS_E_CP_CLOSE_INSTANCE_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801320i32);
pub const EVT_WDSMCS_E_CP_DLL_LOAD_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801328i32);
pub const EVT_WDSMCS_E_CP_DLL_LOAD_FAILED_CRITICAL: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801317i32);
pub const EVT_WDSMCS_E_CP_INCOMPATIBLE_SERVER_VERSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801325i32);
pub const EVT_WDSMCS_E_CP_INIT_FUNC_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801326i32);
pub const EVT_WDSMCS_E_CP_INIT_FUNC_MISSING: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801327i32);
pub const EVT_WDSMCS_E_CP_MEMORY_LEAK: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801322i32);
pub const EVT_WDSMCS_E_CP_OPEN_CONTENT_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801319i32);
pub const EVT_WDSMCS_E_CP_OPEN_INSTANCE_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801321i32);
pub const EVT_WDSMCS_E_CP_SHUTDOWN_FUNC_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801323i32);
pub const EVT_WDSMCS_E_DUPLICATE_MULTICAST_ADDR: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801406i32);
pub const EVT_WDSMCS_E_NON_WDS_DUPLICATE_MULTICAST_ADDR: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801405i32);
pub const EVT_WDSMCS_E_NSREG_CONTENT_PROVIDER_NOT_REG: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801151i32);
pub const EVT_WDSMCS_E_NSREG_FAILURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801149i32);
pub const EVT_WDSMCS_E_NSREG_NAMESPACE_EXISTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801150i32);
pub const EVT_WDSMCS_E_NSREG_START_TIME_IN_PAST: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801152i32);
pub const EVT_WDSMCS_E_PARAMETERS_READ_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801407i32);
pub const EVT_WDSMCS_S_PARAMETERS_READ: ::windows_core::HRESULT = ::windows_core::HRESULT(1092682240i32);
pub const EVT_WDSMCS_W_CP_DLL_LOAD_FAILED_NOT_CRITICAL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2128543142i32);
pub const FACILITY_WDSMCCLIENT: u32 = 290u32;
pub const FACILITY_WDSMCSERVER: u32 = 289u32;
pub const FACILITY_WDSTPTMGMT: u32 = 272u32;
pub const MC_SERVER_CURRENT_VERSION: u32 = 1u32;
pub const PXE_ADDR_BROADCAST: u32 = 1u32;
pub const PXE_ADDR_USE_ADDR: u32 = 4u32;
pub const PXE_ADDR_USE_DHCP_RULES: u32 = 8u32;
pub const PXE_ADDR_USE_PORT: u32 = 2u32;
pub const PXE_BA_CUSTOM: u32 = 2u32;
pub const PXE_BA_IGNORE: u32 = 3u32;
pub const PXE_BA_NBP: u32 = 1u32;
pub const PXE_BA_REJECTED: u32 = 4u32;
pub const PXE_CALLBACK_MAX: u32 = 3u32;
pub const PXE_CALLBACK_RECV_REQUEST: u32 = 0u32;
pub const PXE_CALLBACK_SERVICE_CONTROL: u32 = 2u32;
pub const PXE_CALLBACK_SHUTDOWN: u32 = 1u32;
pub const PXE_DHCPV6_CLIENT_PORT: u32 = 546u32;
pub const PXE_DHCPV6_RELAY_HOP_COUNT_LIMIT: u32 = 32u32;
pub const PXE_DHCPV6_SERVER_PORT: u32 = 547u32;
pub const PXE_DHCP_CLIENT_PORT: u32 = 68u32;
pub const PXE_DHCP_FILE_SIZE: u32 = 128u32;
pub const PXE_DHCP_HWAADR_SIZE: u32 = 16u32;
pub const PXE_DHCP_MAGIC_COOKIE_SIZE: u32 = 4u32;
pub const PXE_DHCP_SERVER_PORT: u32 = 67u32;
pub const PXE_DHCP_SERVER_SIZE: u32 = 64u32;
pub const PXE_GSI_SERVER_DUID: u32 = 2u32;
pub const PXE_GSI_TRACE_ENABLED: u32 = 1u32;
pub const PXE_MAX_ADDRESS: u32 = 16u32;
pub const PXE_PROV_ATTR_FILTER: u32 = 0u32;
pub const PXE_PROV_ATTR_FILTER_IPV6: u32 = 1u32;
pub const PXE_PROV_ATTR_IPV6_CAPABLE: u32 = 2u32;
pub const PXE_PROV_FILTER_ALL: u32 = 0u32;
pub const PXE_PROV_FILTER_DHCP_ONLY: u32 = 1u32;
pub const PXE_PROV_FILTER_PXE_ONLY: u32 = 2u32;
pub const PXE_REG_INDEX_BOTTOM: u32 = 4294967295u32;
pub const PXE_REG_INDEX_TOP: u32 = 0u32;
pub const PXE_SERVER_PORT: u32 = 4011u32;
pub const PXE_TRACE_ERROR: u32 = 524288u32;
pub const PXE_TRACE_FATAL: u32 = 1048576u32;
pub const PXE_TRACE_INFO: u32 = 131072u32;
pub const PXE_TRACE_VERBOSE: u32 = 65536u32;
pub const PXE_TRACE_WARNING: u32 = 262144u32;
pub const TRANSPORTPROVIDER_CURRENT_VERSION: u32 = 1u32;
pub const WDSBP_OPTVAL_ACTION_ABORT: u32 = 5u32;
pub const WDSBP_OPTVAL_ACTION_APPROVAL: u32 = 1u32;
pub const WDSBP_OPTVAL_ACTION_REFERRAL: u32 = 3u32;
pub const WDSBP_OPTVAL_NBP_VER_7: u32 = 1792u32;
pub const WDSBP_OPTVAL_NBP_VER_8: u32 = 2048u32;
pub const WDSBP_OPTVAL_PXE_PROMPT_NOPROMPT: u32 = 2u32;
pub const WDSBP_OPTVAL_PXE_PROMPT_OPTIN: u32 = 1u32;
pub const WDSBP_OPTVAL_PXE_PROMPT_OPTOUT: u32 = 3u32;
pub const WDSBP_OPT_TYPE_BYTE: u32 = 1u32;
pub const WDSBP_OPT_TYPE_IP4: u32 = 6u32;
pub const WDSBP_OPT_TYPE_IP6: u32 = 7u32;
pub const WDSBP_OPT_TYPE_NONE: u32 = 0u32;
pub const WDSBP_OPT_TYPE_STR: u32 = 5u32;
pub const WDSBP_OPT_TYPE_ULONG: u32 = 3u32;
pub const WDSBP_OPT_TYPE_USHORT: u32 = 2u32;
pub const WDSBP_OPT_TYPE_WSTR: u32 = 4u32;
pub const WDSBP_PK_TYPE_BCD: u32 = 4u32;
pub const WDSBP_PK_TYPE_DHCP: u32 = 1u32;
pub const WDSBP_PK_TYPE_DHCPV6: u32 = 8u32;
pub const WDSBP_PK_TYPE_WDSNBP: u32 = 2u32;
pub const WDSMCCLIENT_CATEGORY: ::windows_core::HRESULT = ::windows_core::HRESULT(2i32);
pub const WDSMCSERVER_CATEGORY: ::windows_core::HRESULT = ::windows_core::HRESULT(1i32);
pub const WDSMCS_E_CLIENT_DOESNOT_SUPPORT_SECURITY_MODE: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801648i32);
pub const WDSMCS_E_CLIENT_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801660i32);
pub const WDSMCS_E_CONTENT_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801661i32);
pub const WDSMCS_E_CONTENT_PROVIDER_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801658i32);
pub const WDSMCS_E_INCOMPATIBLE_VERSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801662i32);
pub const WDSMCS_E_NAMESPACE_ALREADY_EXISTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801657i32);
pub const WDSMCS_E_NAMESPACE_ALREADY_STARTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801655i32);
pub const WDSMCS_E_NAMESPACE_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801659i32);
pub const WDSMCS_E_NAMESPACE_SHUTDOWN_IN_PROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801656i32);
pub const WDSMCS_E_NS_START_FAILED_NO_CLIENTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801654i32);
pub const WDSMCS_E_PACKET_HAS_SECURITY: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801650i32);
pub const WDSMCS_E_PACKET_NOT_CHECKSUMED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801649i32);
pub const WDSMCS_E_PACKET_NOT_HASHED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801652i32);
pub const WDSMCS_E_PACKET_NOT_SIGNED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801651i32);
pub const WDSMCS_E_REQCALLBACKS_NOT_REG: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801663i32);
pub const WDSMCS_E_SESSION_SHUTDOWN_IN_PROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801664i32);
pub const WDSMCS_E_START_TIME_IN_PAST: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054801653i32);
pub const WDSTPC_E_ALREADY_COMPLETED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054735615i32);
pub const WDSTPC_E_ALREADY_IN_LOWEST_SESSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054735606i32);
pub const WDSTPC_E_ALREADY_IN_PROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054735614i32);
pub const WDSTPC_E_CALLBACKS_NOT_REG: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054735616i32);
pub const WDSTPC_E_CLIENT_DEMOTE_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054735605i32);
pub const WDSTPC_E_KICKED_FAIL: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054735609i32);
pub const WDSTPC_E_KICKED_FALLBACK: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054735610i32);
pub const WDSTPC_E_KICKED_POLICY_NOT_MET: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054735611i32);
pub const WDSTPC_E_KICKED_UNKNOWN: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054735608i32);
pub const WDSTPC_E_MULTISTREAM_NOT_ENABLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054735607i32);
pub const WDSTPC_E_NOT_INITIALIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054735612i32);
pub const WDSTPC_E_NO_IP4_INTERFACE: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054735604i32);
pub const WDSTPC_E_UNKNOWN_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054735613i32);
pub const WDSTPTC_E_WIM_APPLY_REQUIRES_REFERENCE_IMAGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-1054735603i32);
pub const WDSTPTMGMT_CATEGORY: ::windows_core::HRESULT = ::windows_core::HRESULT(1i32);
pub const WDSTPTMGMT_E_CANNOT_REFRESH_DIRTY_OBJECT: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915761i32);
pub const WDSTPTMGMT_E_CANNOT_REINITIALIZE_OBJECT: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915767i32);
pub const WDSTPTMGMT_E_CONTENT_PROVIDER_ALREADY_REGISTERED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915773i32);
pub const WDSTPTMGMT_E_CONTENT_PROVIDER_NOT_REGISTERED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915772i32);
pub const WDSTPTMGMT_E_INVALID_AUTO_DISCONNECT_THRESHOLD: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915748i32);
pub const WDSTPTMGMT_E_INVALID_CLASS: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915774i32);
pub const WDSTPTMGMT_E_INVALID_CONTENT_PROVIDER_NAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915771i32);
pub const WDSTPTMGMT_E_INVALID_DIAGNOSTICS_COMPONENTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915762i32);
pub const WDSTPTMGMT_E_INVALID_IPV4_MULTICAST_ADDRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915753i32);
pub const WDSTPTMGMT_E_INVALID_IPV6_MULTICAST_ADDRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915752i32);
pub const WDSTPTMGMT_E_INVALID_IPV6_MULTICAST_ADDRESS_SOURCE: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915750i32);
pub const WDSTPTMGMT_E_INVALID_IP_ADDRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915754i32);
pub const WDSTPTMGMT_E_INVALID_MULTISTREAM_STREAM_COUNT: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915749i32);
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_DATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915765i32);
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_NAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915766i32);
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_START_PARAMETERS: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915758i32);
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_START_TIME: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915763i32);
pub const WDSTPTMGMT_E_INVALID_OPERATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915775i32);
pub const WDSTPTMGMT_E_INVALID_PROPERTY: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915776i32);
pub const WDSTPTMGMT_E_INVALID_SERVICE_IP_ADDRESS_RANGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915760i32);
pub const WDSTPTMGMT_E_INVALID_SERVICE_PORT_RANGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915759i32);
pub const WDSTPTMGMT_E_INVALID_SLOW_CLIENT_HANDLING_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915746i32);
pub const WDSTPTMGMT_E_INVALID_TFTP_MAX_BLOCKSIZE: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915741i32);
pub const WDSTPTMGMT_E_IPV6_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915751i32);
pub const WDSTPTMGMT_E_MULTICAST_SESSION_POLICY_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915747i32);
pub const WDSTPTMGMT_E_NAMESPACE_ALREADY_REGISTERED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915769i32);
pub const WDSTPTMGMT_E_NAMESPACE_NOT_ON_SERVER: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915756i32);
pub const WDSTPTMGMT_E_NAMESPACE_NOT_REGISTERED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915768i32);
pub const WDSTPTMGMT_E_NAMESPACE_READ_ONLY: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915764i32);
pub const WDSTPTMGMT_E_NAMESPACE_REMOVED_FROM_SERVER: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915755i32);
pub const WDSTPTMGMT_E_NETWORK_PROFILES_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915745i32);
pub const WDSTPTMGMT_E_TFTP_MAX_BLOCKSIZE_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915743i32);
pub const WDSTPTMGMT_E_TFTP_VAR_WINDOW_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915742i32);
pub const WDSTPTMGMT_E_TRANSPORT_SERVER_ROLE_NOT_CONFIGURED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915770i32);
pub const WDSTPTMGMT_E_TRANSPORT_SERVER_UNAVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915757i32);
pub const WDSTPTMGMT_E_UDP_PORT_POLICY_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1055915744i32);
pub const WDSTRANSPORT_RESOURCE_UTILIZATION_UNKNOWN: u32 = 255u32;
pub const WDS_CLI_FIRMWARE_BIOS: WDS_CLI_FIRMWARE_TYPE = WDS_CLI_FIRMWARE_TYPE(1i32);
pub const WDS_CLI_FIRMWARE_EFI: WDS_CLI_FIRMWARE_TYPE = WDS_CLI_FIRMWARE_TYPE(2i32);
pub const WDS_CLI_FIRMWARE_UNKNOWN: WDS_CLI_FIRMWARE_TYPE = WDS_CLI_FIRMWARE_TYPE(0i32);
pub const WDS_CLI_IMAGE_PARAM_SPARSE_FILE: WDS_CLI_IMAGE_PARAM_TYPE = WDS_CLI_IMAGE_PARAM_TYPE(1i32);
pub const WDS_CLI_IMAGE_PARAM_SUPPORTED_FIRMWARES: WDS_CLI_IMAGE_PARAM_TYPE = WDS_CLI_IMAGE_PARAM_TYPE(2i32);
pub const WDS_CLI_IMAGE_PARAM_UNKNOWN: WDS_CLI_IMAGE_PARAM_TYPE = WDS_CLI_IMAGE_PARAM_TYPE(0i32);
pub const WDS_CLI_IMAGE_TYPE_UNKNOWN: WDS_CLI_IMAGE_TYPE = WDS_CLI_IMAGE_TYPE(0i32);
pub const WDS_CLI_IMAGE_TYPE_VHD: WDS_CLI_IMAGE_TYPE = WDS_CLI_IMAGE_TYPE(2i32);
pub const WDS_CLI_IMAGE_TYPE_VHDX: WDS_CLI_IMAGE_TYPE = WDS_CLI_IMAGE_TYPE(3i32);
pub const WDS_CLI_IMAGE_TYPE_WIM: WDS_CLI_IMAGE_TYPE = WDS_CLI_IMAGE_TYPE(1i32);
pub const WDS_CLI_MSG_COMPLETE: PFN_WDS_CLI_CALLBACK_MESSAGE_ID = PFN_WDS_CLI_CALLBACK_MESSAGE_ID(1u32);
pub const WDS_CLI_MSG_PROGRESS: PFN_WDS_CLI_CALLBACK_MESSAGE_ID = PFN_WDS_CLI_CALLBACK_MESSAGE_ID(2u32);
pub const WDS_CLI_MSG_START: PFN_WDS_CLI_CALLBACK_MESSAGE_ID = PFN_WDS_CLI_CALLBACK_MESSAGE_ID(0u32);
pub const WDS_CLI_MSG_TEXT: PFN_WDS_CLI_CALLBACK_MESSAGE_ID = PFN_WDS_CLI_CALLBACK_MESSAGE_ID(3u32);
pub const WDS_CLI_NO_SPARSE_FILE: u32 = 2u32;
pub const WDS_CLI_TRANSFER_ASYNCHRONOUS: u32 = 1u32;
pub const WDS_LOG_LEVEL_DISABLED: i32 = 0i32;
pub const WDS_LOG_LEVEL_ERROR: i32 = 1i32;
pub const WDS_LOG_LEVEL_INFO: i32 = 3i32;
pub const WDS_LOG_LEVEL_WARNING: i32 = 2i32;
pub const WDS_LOG_TYPE_CLIENT_APPLY_FINISHED: i32 = 6i32;
pub const WDS_LOG_TYPE_CLIENT_APPLY_FINISHED_2: i32 = 16i32;
pub const WDS_LOG_TYPE_CLIENT_APPLY_STARTED: i32 = 5i32;
pub const WDS_LOG_TYPE_CLIENT_APPLY_STARTED_2: i32 = 15i32;
pub const WDS_LOG_TYPE_CLIENT_DOMAINJOINERROR: i32 = 12i32;
pub const WDS_LOG_TYPE_CLIENT_DOMAINJOINERROR_2: i32 = 17i32;
pub const WDS_LOG_TYPE_CLIENT_DRIVER_PACKAGE_NOT_ACCESSIBLE: i32 = 18i32;
pub const WDS_LOG_TYPE_CLIENT_ERROR: i32 = 1i32;
pub const WDS_LOG_TYPE_CLIENT_FINISHED: i32 = 3i32;
pub const WDS_LOG_TYPE_CLIENT_GENERIC_MESSAGE: i32 = 7i32;
pub const WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED: i32 = 4i32;
pub const WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED2: i32 = 22i32;
pub const WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED3: i32 = 23i32;
pub const WDS_LOG_TYPE_CLIENT_MAX_CODE: i32 = 24i32;
pub const WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_END: i32 = 20i32;
pub const WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_FAILURE: i32 = 21i32;
pub const WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_START: i32 = 19i32;
pub const WDS_LOG_TYPE_CLIENT_POST_ACTIONS_END: i32 = 14i32;
pub const WDS_LOG_TYPE_CLIENT_POST_ACTIONS_START: i32 = 13i32;
pub const WDS_LOG_TYPE_CLIENT_STARTED: i32 = 2i32;
pub const WDS_LOG_TYPE_CLIENT_TRANSFER_DOWNGRADE: i32 = 11i32;
pub const WDS_LOG_TYPE_CLIENT_TRANSFER_END: i32 = 10i32;
pub const WDS_LOG_TYPE_CLIENT_TRANSFER_START: i32 = 9i32;
pub const WDS_LOG_TYPE_CLIENT_UNATTEND_MODE: i32 = 8i32;
pub const WDS_MC_TRACE_ERROR: u32 = 524288u32;
pub const WDS_MC_TRACE_FATAL: u32 = 1048576u32;
pub const WDS_MC_TRACE_INFO: u32 = 131072u32;
pub const WDS_MC_TRACE_VERBOSE: u32 = 65536u32;
pub const WDS_MC_TRACE_WARNING: u32 = 262144u32;
pub const WDS_TRANSPORTCLIENT_AUTH: WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL = WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL(1u32);
pub const WDS_TRANSPORTCLIENT_CURRENT_API_VERSION: u32 = 1u32;
pub const WDS_TRANSPORTCLIENT_MAX_CALLBACKS: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(6i32);
pub const WDS_TRANSPORTCLIENT_NO_AUTH: WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL = WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL(2u32);
pub const WDS_TRANSPORTCLIENT_NO_CACHE: u32 = 0u32;
pub const WDS_TRANSPORTCLIENT_PROTOCOL_MULTICAST: u32 = 1u32;
pub const WDS_TRANSPORTCLIENT_RECEIVE_CONTENTS: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(1i32);
pub const WDS_TRANSPORTCLIENT_RECEIVE_METADATA: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(3i32);
pub const WDS_TRANSPORTCLIENT_SESSION_COMPLETE: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(2i32);
pub const WDS_TRANSPORTCLIENT_SESSION_NEGOTIATE: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(5i32);
pub const WDS_TRANSPORTCLIENT_SESSION_START: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(0i32);
pub const WDS_TRANSPORTCLIENT_SESSION_STARTEX: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(4i32);
pub const WDS_TRANSPORTCLIENT_STATUS_FAILURE: u32 = 3u32;
pub const WDS_TRANSPORTCLIENT_STATUS_IN_PROGRESS: u32 = 1u32;
pub const WDS_TRANSPORTCLIENT_STATUS_SUCCESS: u32 = 2u32;
pub const WDS_TRANSPORTPROVIDER_CLOSE_CONTENT: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(6i32);
pub const WDS_TRANSPORTPROVIDER_CLOSE_INSTANCE: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(7i32);
pub const WDS_TRANSPORTPROVIDER_COMPARE_CONTENT: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(1i32);
pub const WDS_TRANSPORTPROVIDER_CREATE_INSTANCE: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(0i32);
pub const WDS_TRANSPORTPROVIDER_DUMP_STATE: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(9i32);
pub const WDS_TRANSPORTPROVIDER_GET_CONTENT_METADATA: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(11i32);
pub const WDS_TRANSPORTPROVIDER_GET_CONTENT_SIZE: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(4i32);
pub const WDS_TRANSPORTPROVIDER_MAX_CALLBACKS: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(12i32);
pub const WDS_TRANSPORTPROVIDER_OPEN_CONTENT: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(2i32);
pub const WDS_TRANSPORTPROVIDER_READ_CONTENT: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(5i32);
pub const WDS_TRANSPORTPROVIDER_REFRESH_SETTINGS: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(10i32);
pub const WDS_TRANSPORTPROVIDER_SHUTDOWN: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(8i32);
pub const WDS_TRANSPORTPROVIDER_USER_ACCESS_CHECK: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(3i32);
pub const WdsCliFlagEnumFilterFirmware: i32 = 2i32;
pub const WdsCliFlagEnumFilterVersion: i32 = 1i32;
pub const WdsTptDiagnosticsComponentImageServer: WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS(4i32);
pub const WdsTptDiagnosticsComponentMulticast: WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS(8i32);
pub const WdsTptDiagnosticsComponentPxe: WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS(1i32);
pub const WdsTptDiagnosticsComponentTftp: WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS(2i32);
pub const WdsTptDisconnectAbort: WDSTRANSPORT_DISCONNECT_TYPE = WDSTRANSPORT_DISCONNECT_TYPE(2i32);
pub const WdsTptDisconnectFallback: WDSTRANSPORT_DISCONNECT_TYPE = WDSTRANSPORT_DISCONNECT_TYPE(1i32);
pub const WdsTptDisconnectUnknown: WDSTRANSPORT_DISCONNECT_TYPE = WDSTRANSPORT_DISCONNECT_TYPE(0i32);
pub const WdsTptFeatureAdminPack: WDSTRANSPORT_FEATURE_FLAGS = WDSTRANSPORT_FEATURE_FLAGS(1i32);
pub const WdsTptFeatureDeploymentServer: WDSTRANSPORT_FEATURE_FLAGS = WDSTRANSPORT_FEATURE_FLAGS(4i32);
pub const WdsTptFeatureTransportServer: WDSTRANSPORT_FEATURE_FLAGS = WDSTRANSPORT_FEATURE_FLAGS(2i32);
pub const WdsTptIpAddressIpv4: WDSTRANSPORT_IP_ADDRESS_TYPE = WDSTRANSPORT_IP_ADDRESS_TYPE(1i32);
pub const WdsTptIpAddressIpv6: WDSTRANSPORT_IP_ADDRESS_TYPE = WDSTRANSPORT_IP_ADDRESS_TYPE(2i32);
pub const WdsTptIpAddressSourceDhcp: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE = WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE(1i32);
pub const WdsTptIpAddressSourceRange: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE = WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE(2i32);
pub const WdsTptIpAddressSourceUnknown: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE = WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE(0i32);
pub const WdsTptIpAddressUnknown: WDSTRANSPORT_IP_ADDRESS_TYPE = WDSTRANSPORT_IP_ADDRESS_TYPE(0i32);
pub const WdsTptNamespaceTypeAutoCast: WDSTRANSPORT_NAMESPACE_TYPE = WDSTRANSPORT_NAMESPACE_TYPE(1i32);
pub const WdsTptNamespaceTypeScheduledCastAutoStart: WDSTRANSPORT_NAMESPACE_TYPE = WDSTRANSPORT_NAMESPACE_TYPE(3i32);
pub const WdsTptNamespaceTypeScheduledCastManualStart: WDSTRANSPORT_NAMESPACE_TYPE = WDSTRANSPORT_NAMESPACE_TYPE(2i32);
pub const WdsTptNamespaceTypeUnknown: WDSTRANSPORT_NAMESPACE_TYPE = WDSTRANSPORT_NAMESPACE_TYPE(0i32);
pub const WdsTptNetworkProfile100Mbps: WDSTRANSPORT_NETWORK_PROFILE_TYPE = WDSTRANSPORT_NETWORK_PROFILE_TYPE(3i32);
pub const WdsTptNetworkProfile10Mbps: WDSTRANSPORT_NETWORK_PROFILE_TYPE = WDSTRANSPORT_NETWORK_PROFILE_TYPE(2i32);
pub const WdsTptNetworkProfile1Gbps: WDSTRANSPORT_NETWORK_PROFILE_TYPE = WDSTRANSPORT_NETWORK_PROFILE_TYPE(4i32);
pub const WdsTptNetworkProfileCustom: WDSTRANSPORT_NETWORK_PROFILE_TYPE = WDSTRANSPORT_NETWORK_PROFILE_TYPE(1i32);
pub const WdsTptNetworkProfileUnknown: WDSTRANSPORT_NETWORK_PROFILE_TYPE = WDSTRANSPORT_NETWORK_PROFILE_TYPE(0i32);
pub const WdsTptProtocolMulticast: WDSTRANSPORT_PROTOCOL_FLAGS = WDSTRANSPORT_PROTOCOL_FLAGS(2i32);
pub const WdsTptProtocolUnicast: WDSTRANSPORT_PROTOCOL_FLAGS = WDSTRANSPORT_PROTOCOL_FLAGS(1i32);
pub const WdsTptServiceNotifyReadSettings: WDSTRANSPORT_SERVICE_NOTIFICATION = WDSTRANSPORT_SERVICE_NOTIFICATION(1i32);
pub const WdsTptServiceNotifyUnknown: WDSTRANSPORT_SERVICE_NOTIFICATION = WDSTRANSPORT_SERVICE_NOTIFICATION(0i32);
pub const WdsTptSlowClientHandlingAutoDisconnect: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE(2i32);
pub const WdsTptSlowClientHandlingMultistream: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE(3i32);
pub const WdsTptSlowClientHandlingNone: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE(1i32);
pub const WdsTptSlowClientHandlingUnknown: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE(0i32);
pub const WdsTptTftpCapMaximumBlockSize: WDSTRANSPORT_TFTP_CAPABILITY = WDSTRANSPORT_TFTP_CAPABILITY(1i32);
pub const WdsTptTftpCapVariableWindow: WDSTRANSPORT_TFTP_CAPABILITY = WDSTRANSPORT_TFTP_CAPABILITY(2i32);
pub const WdsTptUdpPortPolicyDynamic: WDSTRANSPORT_UDP_PORT_POLICY = WDSTRANSPORT_UDP_PORT_POLICY(0i32);
pub const WdsTptUdpPortPolicyFixed: WDSTRANSPORT_UDP_PORT_POLICY = WDSTRANSPORT_UDP_PORT_POLICY(1i32);
pub const WdsTransportCacheable: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70590b16_f146_46bd_bd9d_4aaa90084bf5);
pub const WdsTransportClient: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x66d2c5e9_0ff6_49ec_9733_dafb1e01df1c);
pub const WdsTransportCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7f18b09_391e_436e_b10b_c3ef46f2c34f);
pub const WdsTransportConfigurationManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8743f674_904c_47ca_8512_35fe98f6b0ac);
pub const WdsTransportContent: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a891fe7_4a3f_4c65_b6f2_1467619679ea);
pub const WdsTransportContentProvider: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0be741f_5a75_4eb9_8a2d_5e189b45f327);
pub const WdsTransportDiagnosticsPolicy: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb3333e1_a7ad_46f5_80d6_6b740204e509);
pub const WdsTransportManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf21523f6_837c_4a58_af99_8a7e27f8ff59);
pub const WdsTransportMulticastSessionPolicy: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c6bc3f4_6418_472a_b6f1_52d457195437);
pub const WdsTransportNamespace: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8385768_0732_4ec1_95ea_16da581908a1);
pub const WdsTransportNamespaceAutoCast: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb091f5a8_6a99_478d_b23b_09e8fee04574);
pub const WdsTransportNamespaceManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf08cdb63_85de_4a28_a1a9_5ca3e7efda73);
pub const WdsTransportNamespaceScheduledCast: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbadc1897_7025_44eb_9108_fb61c4055792);
pub const WdsTransportNamespaceScheduledCastAutoStart: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1107052_122c_4b81_9b7c_386e6855383f);
pub const WdsTransportNamespaceScheduledCastManualStart: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd3e1a2aa_caac_460e_b98a_47f9f318a1fa);
pub const WdsTransportServer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea19b643_4adf_4413_942c_14f379118760);
pub const WdsTransportServicePolicy: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65aceadc_2f0b_4f43_9f4d_811865d8cead);
pub const WdsTransportSession: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x749ac4e0_67bc_4743_bfe5_cacb1f26f57f);
pub const WdsTransportSetupManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7beeaad_9f04_4923_9f0c_fbf52bc7590f);
pub const WdsTransportTftpClient: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50343925_7c5c_4c8c_96c4_ad9fa5005fba);
pub const WdsTransportTftpManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8e9dca2_3241_4e4d_b806_bc74019dfeda);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CPU_ARCHITECTURE(pub u32);
impl ::core::marker::Copy for CPU_ARCHITECTURE {}
impl ::core::clone::Clone for CPU_ARCHITECTURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CPU_ARCHITECTURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CPU_ARCHITECTURE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CPU_ARCHITECTURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CPU_ARCHITECTURE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PFN_WDS_CLI_CALLBACK_MESSAGE_ID(pub u32);
impl ::core::marker::Copy for PFN_WDS_CLI_CALLBACK_MESSAGE_ID {}
impl ::core::clone::Clone for PFN_WDS_CLI_CALLBACK_MESSAGE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PFN_WDS_CLI_CALLBACK_MESSAGE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PFN_WDS_CLI_CALLBACK_MESSAGE_ID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PFN_WDS_CLI_CALLBACK_MESSAGE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PFN_WDS_CLI_CALLBACK_MESSAGE_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRANSPORTCLIENT_CALLBACK_ID(pub i32);
impl ::core::marker::Copy for TRANSPORTCLIENT_CALLBACK_ID {}
impl ::core::clone::Clone for TRANSPORTCLIENT_CALLBACK_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRANSPORTCLIENT_CALLBACK_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TRANSPORTCLIENT_CALLBACK_ID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TRANSPORTCLIENT_CALLBACK_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSPORTCLIENT_CALLBACK_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRANSPORTPROVIDER_CALLBACK_ID(pub i32);
impl ::core::marker::Copy for TRANSPORTPROVIDER_CALLBACK_ID {}
impl ::core::clone::Clone for TRANSPORTPROVIDER_CALLBACK_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRANSPORTPROVIDER_CALLBACK_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TRANSPORTPROVIDER_CALLBACK_ID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TRANSPORTPROVIDER_CALLBACK_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSPORTPROVIDER_CALLBACK_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS(pub i32);
impl ::core::marker::Copy for WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS {}
impl ::core::clone::Clone for WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WDSTRANSPORT_DISCONNECT_TYPE(pub i32);
impl ::core::marker::Copy for WDSTRANSPORT_DISCONNECT_TYPE {}
impl ::core::clone::Clone for WDSTRANSPORT_DISCONNECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WDSTRANSPORT_DISCONNECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WDSTRANSPORT_DISCONNECT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WDSTRANSPORT_DISCONNECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDSTRANSPORT_DISCONNECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WDSTRANSPORT_FEATURE_FLAGS(pub i32);
impl ::core::marker::Copy for WDSTRANSPORT_FEATURE_FLAGS {}
impl ::core::clone::Clone for WDSTRANSPORT_FEATURE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WDSTRANSPORT_FEATURE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WDSTRANSPORT_FEATURE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WDSTRANSPORT_FEATURE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDSTRANSPORT_FEATURE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE(pub i32);
impl ::core::marker::Copy for WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE {}
impl ::core::clone::Clone for WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WDSTRANSPORT_IP_ADDRESS_TYPE(pub i32);
impl ::core::marker::Copy for WDSTRANSPORT_IP_ADDRESS_TYPE {}
impl ::core::clone::Clone for WDSTRANSPORT_IP_ADDRESS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WDSTRANSPORT_IP_ADDRESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WDSTRANSPORT_IP_ADDRESS_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WDSTRANSPORT_IP_ADDRESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDSTRANSPORT_IP_ADDRESS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WDSTRANSPORT_NAMESPACE_TYPE(pub i32);
impl ::core::marker::Copy for WDSTRANSPORT_NAMESPACE_TYPE {}
impl ::core::clone::Clone for WDSTRANSPORT_NAMESPACE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WDSTRANSPORT_NAMESPACE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WDSTRANSPORT_NAMESPACE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WDSTRANSPORT_NAMESPACE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDSTRANSPORT_NAMESPACE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WDSTRANSPORT_NETWORK_PROFILE_TYPE(pub i32);
impl ::core::marker::Copy for WDSTRANSPORT_NETWORK_PROFILE_TYPE {}
impl ::core::clone::Clone for WDSTRANSPORT_NETWORK_PROFILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WDSTRANSPORT_NETWORK_PROFILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WDSTRANSPORT_NETWORK_PROFILE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WDSTRANSPORT_NETWORK_PROFILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDSTRANSPORT_NETWORK_PROFILE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WDSTRANSPORT_PROTOCOL_FLAGS(pub i32);
impl ::core::marker::Copy for WDSTRANSPORT_PROTOCOL_FLAGS {}
impl ::core::clone::Clone for WDSTRANSPORT_PROTOCOL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WDSTRANSPORT_PROTOCOL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WDSTRANSPORT_PROTOCOL_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WDSTRANSPORT_PROTOCOL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDSTRANSPORT_PROTOCOL_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WDSTRANSPORT_SERVICE_NOTIFICATION(pub i32);
impl ::core::marker::Copy for WDSTRANSPORT_SERVICE_NOTIFICATION {}
impl ::core::clone::Clone for WDSTRANSPORT_SERVICE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WDSTRANSPORT_SERVICE_NOTIFICATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WDSTRANSPORT_SERVICE_NOTIFICATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WDSTRANSPORT_SERVICE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDSTRANSPORT_SERVICE_NOTIFICATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE(pub i32);
impl ::core::marker::Copy for WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE {}
impl ::core::clone::Clone for WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WDSTRANSPORT_TFTP_CAPABILITY(pub i32);
impl ::core::marker::Copy for WDSTRANSPORT_TFTP_CAPABILITY {}
impl ::core::clone::Clone for WDSTRANSPORT_TFTP_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WDSTRANSPORT_TFTP_CAPABILITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WDSTRANSPORT_TFTP_CAPABILITY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WDSTRANSPORT_TFTP_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDSTRANSPORT_TFTP_CAPABILITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WDSTRANSPORT_UDP_PORT_POLICY(pub i32);
impl ::core::marker::Copy for WDSTRANSPORT_UDP_PORT_POLICY {}
impl ::core::clone::Clone for WDSTRANSPORT_UDP_PORT_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WDSTRANSPORT_UDP_PORT_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WDSTRANSPORT_UDP_PORT_POLICY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WDSTRANSPORT_UDP_PORT_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDSTRANSPORT_UDP_PORT_POLICY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WDS_CLI_FIRMWARE_TYPE(pub i32);
impl ::core::marker::Copy for WDS_CLI_FIRMWARE_TYPE {}
impl ::core::clone::Clone for WDS_CLI_FIRMWARE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WDS_CLI_FIRMWARE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WDS_CLI_FIRMWARE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WDS_CLI_FIRMWARE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDS_CLI_FIRMWARE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WDS_CLI_IMAGE_PARAM_TYPE(pub i32);
impl ::core::marker::Copy for WDS_CLI_IMAGE_PARAM_TYPE {}
impl ::core::clone::Clone for WDS_CLI_IMAGE_PARAM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WDS_CLI_IMAGE_PARAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WDS_CLI_IMAGE_PARAM_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WDS_CLI_IMAGE_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDS_CLI_IMAGE_PARAM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WDS_CLI_IMAGE_TYPE(pub i32);
impl ::core::marker::Copy for WDS_CLI_IMAGE_TYPE {}
impl ::core::clone::Clone for WDS_CLI_IMAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WDS_CLI_IMAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WDS_CLI_IMAGE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WDS_CLI_IMAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDS_CLI_IMAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL(pub u32);
impl ::core::marker::Copy for WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL {}
impl ::core::clone::Clone for WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct PXE_ADDRESS {
    pub uFlags: u32,
    pub Anonymous: PXE_ADDRESS_0,
    pub uAddrLen: u32,
    pub uPort: u16,
}
impl ::core::marker::Copy for PXE_ADDRESS {}
impl ::core::clone::Clone for PXE_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for PXE_ADDRESS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PXE_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union PXE_ADDRESS_0 {
    pub bAddress: [u8; 16],
    pub uIpAddress: u32,
}
impl ::core::marker::Copy for PXE_ADDRESS_0 {}
impl ::core::clone::Clone for PXE_ADDRESS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for PXE_ADDRESS_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PXE_ADDRESS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct PXE_DHCPV6_MESSAGE {
    pub MessageType: u8,
    pub TransactionIDByte1: u8,
    pub TransactionIDByte2: u8,
    pub TransactionIDByte3: u8,
    pub Options: [PXE_DHCPV6_OPTION; 1],
}
impl ::core::marker::Copy for PXE_DHCPV6_MESSAGE {}
impl ::core::clone::Clone for PXE_DHCPV6_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for PXE_DHCPV6_MESSAGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PXE_DHCPV6_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct PXE_DHCPV6_MESSAGE_HEADER {
    pub MessageType: u8,
    pub Message: [u8; 1],
}
impl ::core::marker::Copy for PXE_DHCPV6_MESSAGE_HEADER {}
impl ::core::clone::Clone for PXE_DHCPV6_MESSAGE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for PXE_DHCPV6_MESSAGE_HEADER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PXE_DHCPV6_MESSAGE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PXE_DHCPV6_NESTED_RELAY_MESSAGE {
    pub pRelayMessage: *mut PXE_DHCPV6_RELAY_MESSAGE,
    pub cbRelayMessage: u32,
    pub pInterfaceIdOption: *mut ::core::ffi::c_void,
    pub cbInterfaceIdOption: u16,
}
impl ::core::marker::Copy for PXE_DHCPV6_NESTED_RELAY_MESSAGE {}
impl ::core::clone::Clone for PXE_DHCPV6_NESTED_RELAY_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PXE_DHCPV6_NESTED_RELAY_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PXE_DHCPV6_NESTED_RELAY_MESSAGE").field("pRelayMessage", &self.pRelayMessage).field("cbRelayMessage", &self.cbRelayMessage).field("pInterfaceIdOption", &self.pInterfaceIdOption).field("cbInterfaceIdOption", &self.cbInterfaceIdOption).finish()
    }
}
impl ::windows_core::TypeKind for PXE_DHCPV6_NESTED_RELAY_MESSAGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PXE_DHCPV6_NESTED_RELAY_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.pRelayMessage == other.pRelayMessage && self.cbRelayMessage == other.cbRelayMessage && self.pInterfaceIdOption == other.pInterfaceIdOption && self.cbInterfaceIdOption == other.cbInterfaceIdOption
    }
}
impl ::core::cmp::Eq for PXE_DHCPV6_NESTED_RELAY_MESSAGE {}
impl ::core::default::Default for PXE_DHCPV6_NESTED_RELAY_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct PXE_DHCPV6_OPTION {
    pub OptionCode: u16,
    pub DataLength: u16,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for PXE_DHCPV6_OPTION {}
impl ::core::clone::Clone for PXE_DHCPV6_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for PXE_DHCPV6_OPTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PXE_DHCPV6_OPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct PXE_DHCPV6_RELAY_MESSAGE {
    pub MessageType: u8,
    pub HopCount: u8,
    pub LinkAddress: [u8; 16],
    pub PeerAddress: [u8; 16],
    pub Options: [PXE_DHCPV6_OPTION; 1],
}
impl ::core::marker::Copy for PXE_DHCPV6_RELAY_MESSAGE {}
impl ::core::clone::Clone for PXE_DHCPV6_RELAY_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for PXE_DHCPV6_RELAY_MESSAGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PXE_DHCPV6_RELAY_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct PXE_DHCP_MESSAGE {
    pub Operation: u8,
    pub HardwareAddressType: u8,
    pub HardwareAddressLength: u8,
    pub HopCount: u8,
    pub TransactionID: u32,
    pub SecondsSinceBoot: u16,
    pub Reserved: u16,
    pub ClientIpAddress: u32,
    pub YourIpAddress: u32,
    pub BootstrapServerAddress: u32,
    pub RelayAgentIpAddress: u32,
    pub HardwareAddress: [u8; 16],
    pub HostName: [u8; 64],
    pub BootFileName: [u8; 128],
    pub Anonymous: PXE_DHCP_MESSAGE_0,
    pub Option: PXE_DHCP_OPTION,
}
impl ::core::marker::Copy for PXE_DHCP_MESSAGE {}
impl ::core::clone::Clone for PXE_DHCP_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for PXE_DHCP_MESSAGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PXE_DHCP_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union PXE_DHCP_MESSAGE_0 {
    pub bMagicCookie: [u8; 4],
    pub uMagicCookie: u32,
}
impl ::core::marker::Copy for PXE_DHCP_MESSAGE_0 {}
impl ::core::clone::Clone for PXE_DHCP_MESSAGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for PXE_DHCP_MESSAGE_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PXE_DHCP_MESSAGE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct PXE_DHCP_OPTION {
    pub OptionType: u8,
    pub OptionLength: u8,
    pub OptionValue: [u8; 1],
}
impl ::core::marker::Copy for PXE_DHCP_OPTION {}
impl ::core::clone::Clone for PXE_DHCP_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for PXE_DHCP_OPTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PXE_DHCP_OPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct PXE_PROVIDER {
    pub uSizeOfStruct: u32,
    pub pwszName: ::windows_core::PWSTR,
    pub pwszFilePath: ::windows_core::PWSTR,
    pub bIsCritical: super::super::Foundation::BOOL,
    pub uIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PXE_PROVIDER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PXE_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PXE_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PXE_PROVIDER").field("uSizeOfStruct", &self.uSizeOfStruct).field("pwszName", &self.pwszName).field("pwszFilePath", &self.pwszFilePath).field("bIsCritical", &self.bIsCritical).field("uIndex", &self.uIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for PXE_PROVIDER {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PXE_PROVIDER {
    fn eq(&self, other: &Self) -> bool {
        self.uSizeOfStruct == other.uSizeOfStruct && self.pwszName == other.pwszName && self.pwszFilePath == other.pwszFilePath && self.bIsCritical == other.bIsCritical && self.uIndex == other.uIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PXE_PROVIDER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PXE_PROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TRANSPORTCLIENT_SESSION_INFO {
    pub ulStructureLength: u32,
    pub ullFileSize: u64,
    pub ulBlockSize: u32,
}
impl ::core::marker::Copy for TRANSPORTCLIENT_SESSION_INFO {}
impl ::core::clone::Clone for TRANSPORTCLIENT_SESSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSPORTCLIENT_SESSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORTCLIENT_SESSION_INFO").field("ulStructureLength", &self.ulStructureLength).field("ullFileSize", &self.ullFileSize).field("ulBlockSize", &self.ulBlockSize).finish()
    }
}
impl ::windows_core::TypeKind for TRANSPORTCLIENT_SESSION_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for TRANSPORTCLIENT_SESSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulStructureLength == other.ulStructureLength && self.ullFileSize == other.ullFileSize && self.ulBlockSize == other.ulBlockSize
    }
}
impl ::core::cmp::Eq for TRANSPORTCLIENT_SESSION_INFO {}
impl ::core::default::Default for TRANSPORTCLIENT_SESSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WDS_CLI_CRED {
    pub pwszUserName: ::windows_core::PCWSTR,
    pub pwszDomain: ::windows_core::PCWSTR,
    pub pwszPassword: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WDS_CLI_CRED {}
impl ::core::clone::Clone for WDS_CLI_CRED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WDS_CLI_CRED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WDS_CLI_CRED").field("pwszUserName", &self.pwszUserName).field("pwszDomain", &self.pwszDomain).field("pwszPassword", &self.pwszPassword).finish()
    }
}
impl ::windows_core::TypeKind for WDS_CLI_CRED {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WDS_CLI_CRED {
    fn eq(&self, other: &Self) -> bool {
        self.pwszUserName == other.pwszUserName && self.pwszDomain == other.pwszDomain && self.pwszPassword == other.pwszPassword
    }
}
impl ::core::cmp::Eq for WDS_CLI_CRED {}
impl ::core::default::Default for WDS_CLI_CRED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WDS_TRANSPORTCLIENT_CALLBACKS {
    pub SessionStart: PFN_WdsTransportClientSessionStart,
    pub SessionStartEx: PFN_WdsTransportClientSessionStartEx,
    pub ReceiveContents: PFN_WdsTransportClientReceiveContents,
    pub ReceiveMetadata: PFN_WdsTransportClientReceiveMetadata,
    pub SessionComplete: PFN_WdsTransportClientSessionComplete,
    pub SessionNegotiate: PFN_WdsTransportClientSessionNegotiate,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WDS_TRANSPORTCLIENT_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WDS_TRANSPORTCLIENT_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WDS_TRANSPORTCLIENT_CALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WDS_TRANSPORTCLIENT_CALLBACKS").finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WDS_TRANSPORTCLIENT_CALLBACKS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WDS_TRANSPORTCLIENT_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WDS_TRANSPORTCLIENT_REQUEST {
    pub ulLength: u32,
    pub ulApiVersion: u32,
    pub ulAuthLevel: WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL,
    pub pwszServer: ::windows_core::PCWSTR,
    pub pwszNamespace: ::windows_core::PCWSTR,
    pub pwszObjectName: ::windows_core::PCWSTR,
    pub ulCacheSize: u32,
    pub ulProtocol: u32,
    pub pvProtocolData: *mut ::core::ffi::c_void,
    pub ulProtocolDataLength: u32,
}
impl ::core::marker::Copy for WDS_TRANSPORTCLIENT_REQUEST {}
impl ::core::clone::Clone for WDS_TRANSPORTCLIENT_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WDS_TRANSPORTCLIENT_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WDS_TRANSPORTCLIENT_REQUEST").field("ulLength", &self.ulLength).field("ulApiVersion", &self.ulApiVersion).field("ulAuthLevel", &self.ulAuthLevel).field("pwszServer", &self.pwszServer).field("pwszNamespace", &self.pwszNamespace).field("pwszObjectName", &self.pwszObjectName).field("ulCacheSize", &self.ulCacheSize).field("ulProtocol", &self.ulProtocol).field("pvProtocolData", &self.pvProtocolData).field("ulProtocolDataLength", &self.ulProtocolDataLength).finish()
    }
}
impl ::windows_core::TypeKind for WDS_TRANSPORTCLIENT_REQUEST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WDS_TRANSPORTCLIENT_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.ulLength == other.ulLength && self.ulApiVersion == other.ulApiVersion && self.ulAuthLevel == other.ulAuthLevel && self.pwszServer == other.pwszServer && self.pwszNamespace == other.pwszNamespace && self.pwszObjectName == other.pwszObjectName && self.ulCacheSize == other.ulCacheSize && self.ulProtocol == other.ulProtocol && self.pvProtocolData == other.pvProtocolData && self.ulProtocolDataLength == other.ulProtocolDataLength
    }
}
impl ::core::cmp::Eq for WDS_TRANSPORTCLIENT_REQUEST {}
impl ::core::default::Default for WDS_TRANSPORTCLIENT_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct WDS_TRANSPORTPROVIDER_INIT_PARAMS {
    pub ulLength: u32,
    pub ulMcServerVersion: u32,
    pub hRegistryKey: super::Registry::HKEY,
    pub hProvider: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for WDS_TRANSPORTPROVIDER_INIT_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for WDS_TRANSPORTPROVIDER_INIT_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::fmt::Debug for WDS_TRANSPORTPROVIDER_INIT_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WDS_TRANSPORTPROVIDER_INIT_PARAMS").field("ulLength", &self.ulLength).field("ulMcServerVersion", &self.ulMcServerVersion).field("hRegistryKey", &self.hRegistryKey).field("hProvider", &self.hProvider).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows_core::TypeKind for WDS_TRANSPORTPROVIDER_INIT_PARAMS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::cmp::PartialEq for WDS_TRANSPORTPROVIDER_INIT_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.ulLength == other.ulLength && self.ulMcServerVersion == other.ulMcServerVersion && self.hRegistryKey == other.hRegistryKey && self.hProvider == other.hProvider
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::cmp::Eq for WDS_TRANSPORTPROVIDER_INIT_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::default::Default for WDS_TRANSPORTPROVIDER_INIT_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WDS_TRANSPORTPROVIDER_SETTINGS {
    pub ulLength: u32,
    pub ulProviderVersion: u32,
}
impl ::core::marker::Copy for WDS_TRANSPORTPROVIDER_SETTINGS {}
impl ::core::clone::Clone for WDS_TRANSPORTPROVIDER_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WDS_TRANSPORTPROVIDER_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WDS_TRANSPORTPROVIDER_SETTINGS").field("ulLength", &self.ulLength).field("ulProviderVersion", &self.ulProviderVersion).finish()
    }
}
impl ::windows_core::TypeKind for WDS_TRANSPORTPROVIDER_SETTINGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WDS_TRANSPORTPROVIDER_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.ulLength == other.ulLength && self.ulProviderVersion == other.ulProviderVersion
    }
}
impl ::core::cmp::Eq for WDS_TRANSPORTPROVIDER_SETTINGS {}
impl ::core::default::Default for WDS_TRANSPORTPROVIDER_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsCliCallback = ::core::option::Option<unsafe extern "system" fn(dwmessageid: PFN_WDS_CLI_CALLBACK_MESSAGE_ID, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pvuserdata: *const ::core::ffi::c_void) -> ()>;
pub type PFN_WdsCliTraceFunction = ::core::option::Option<unsafe extern "system" fn(pwszformat: ::windows_core::PCWSTR, params: *const i8) -> ()>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientReceiveContents = ::core::option::Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::core::ffi::c_void, pcontents: *const ::core::ffi::c_void, ulsize: u32, pullcontentoffset: *const u64) -> ()>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientReceiveMetadata = ::core::option::Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::core::ffi::c_void, pmetadata: *const ::core::ffi::c_void, ulsize: u32) -> ()>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientSessionComplete = ::core::option::Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::core::ffi::c_void, dwerror: u32) -> ()>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientSessionNegotiate = ::core::option::Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::core::ffi::c_void, pinfo: *const TRANSPORTCLIENT_SESSION_INFO, hnegotiatekey: super::super::Foundation::HANDLE) -> ()>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientSessionStart = ::core::option::Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::core::ffi::c_void, ullfilesize: *const u64) -> ()>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_WdsTransportClientSessionStartEx = ::core::option::Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const ::core::ffi::c_void, info: *const TRANSPORTCLIENT_SESSION_INFO) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
