#[cfg(all(feature = "Win32_rpcdce", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn DsAddSidHistoryA<P2, P3, P4, P6, P7>(hds: super::winnt::HANDLE, flags: Option<u32>, srcdomain: P2, srcprincipal: P3, srcdomaincontroller: P4, srcdomaincreds: Option<super::rpcdce::RPC_AUTH_IDENTITY_HANDLE>, dstdomain: P6, dstprincipal: P7) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
    P7: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsAddSidHistoryA(hds : super::winnt::HANDLE, flags : u32, srcdomain : windows_core::PCSTR, srcprincipal : windows_core::PCSTR, srcdomaincontroller : windows_core::PCSTR, srcdomaincreds : super::rpcdce::RPC_AUTH_IDENTITY_HANDLE, dstdomain : windows_core::PCSTR, dstprincipal : windows_core::PCSTR) -> u32);
    unsafe { DsAddSidHistoryA(hds, flags.unwrap_or(core::mem::zeroed()) as _, srcdomain.param().abi(), srcprincipal.param().abi(), srcdomaincontroller.param().abi(), srcdomaincreds.unwrap_or(core::mem::zeroed()) as _, dstdomain.param().abi(), dstprincipal.param().abi()) }
}
#[cfg(all(feature = "Win32_rpcdce", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn DsAddSidHistoryW<P2, P3, P4, P6, P7>(hds: super::winnt::HANDLE, flags: Option<u32>, srcdomain: P2, srcprincipal: P3, srcdomaincontroller: P4, srcdomaincreds: Option<super::rpcdce::RPC_AUTH_IDENTITY_HANDLE>, dstdomain: P6, dstprincipal: P7) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsAddSidHistoryW(hds : super::winnt::HANDLE, flags : u32, srcdomain : windows_core::PCWSTR, srcprincipal : windows_core::PCWSTR, srcdomaincontroller : windows_core::PCWSTR, srcdomaincreds : super::rpcdce::RPC_AUTH_IDENTITY_HANDLE, dstdomain : windows_core::PCWSTR, dstprincipal : windows_core::PCWSTR) -> u32);
    unsafe { DsAddSidHistoryW(hds, flags.unwrap_or(core::mem::zeroed()) as _, srcdomain.param().abi(), srcprincipal.param().abi(), srcdomaincontroller.param().abi(), srcdomaincreds.unwrap_or(core::mem::zeroed()) as _, dstdomain.param().abi(), dstprincipal.param().abi()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsBindA<P0, P1>(domaincontrollername: P0, dnsdomainname: P1, phds: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsBindA(domaincontrollername : windows_core::PCSTR, dnsdomainname : windows_core::PCSTR, phds : *mut super::winnt::HANDLE) -> u32);
    unsafe { DsBindA(domaincontrollername.param().abi(), dnsdomainname.param().abi(), phds as _) }
}
#[cfg(all(feature = "Win32_rpcdce", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn DsBindByInstanceA<P0, P1, P3, P5>(servername: P0, annotation: P1, instanceguid: Option<*const windows_core::GUID>, dnsdomainname: P3, authidentity: Option<super::rpcdce::RPC_AUTH_IDENTITY_HANDLE>, serviceprincipalname: P5, bindflags: Option<u32>, phds: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsBindByInstanceA(servername : windows_core::PCSTR, annotation : windows_core::PCSTR, instanceguid : *const windows_core::GUID, dnsdomainname : windows_core::PCSTR, authidentity : super::rpcdce::RPC_AUTH_IDENTITY_HANDLE, serviceprincipalname : windows_core::PCSTR, bindflags : u32, phds : *mut super::winnt::HANDLE) -> u32);
    unsafe { DsBindByInstanceA(servername.param().abi(), annotation.param().abi(), instanceguid.unwrap_or(core::mem::zeroed()) as _, dnsdomainname.param().abi(), authidentity.unwrap_or(core::mem::zeroed()) as _, serviceprincipalname.param().abi(), bindflags.unwrap_or(core::mem::zeroed()) as _, phds as _) }
}
#[cfg(all(feature = "Win32_rpcdce", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn DsBindByInstanceW<P0, P1, P3, P5>(servername: P0, annotation: P1, instanceguid: Option<*const windows_core::GUID>, dnsdomainname: P3, authidentity: Option<super::rpcdce::RPC_AUTH_IDENTITY_HANDLE>, serviceprincipalname: P5, bindflags: Option<u32>, phds: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsBindByInstanceW(servername : windows_core::PCWSTR, annotation : windows_core::PCWSTR, instanceguid : *const windows_core::GUID, dnsdomainname : windows_core::PCWSTR, authidentity : super::rpcdce::RPC_AUTH_IDENTITY_HANDLE, serviceprincipalname : windows_core::PCWSTR, bindflags : u32, phds : *mut super::winnt::HANDLE) -> u32);
    unsafe { DsBindByInstanceW(servername.param().abi(), annotation.param().abi(), instanceguid.unwrap_or(core::mem::zeroed()) as _, dnsdomainname.param().abi(), authidentity.unwrap_or(core::mem::zeroed()) as _, serviceprincipalname.param().abi(), bindflags.unwrap_or(core::mem::zeroed()) as _, phds as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsBindToISTGA<P0>(sitename: P0, phds: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsBindToISTGA(sitename : windows_core::PCSTR, phds : *mut super::winnt::HANDLE) -> u32);
    unsafe { DsBindToISTGA(sitename.param().abi(), phds as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsBindToISTGW<P0>(sitename: P0, phds: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsBindToISTGW(sitename : windows_core::PCWSTR, phds : *mut super::winnt::HANDLE) -> u32);
    unsafe { DsBindToISTGW(sitename.param().abi(), phds as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsBindW<P0, P1>(domaincontrollername: P0, dnsdomainname: P1, phds: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsBindW(domaincontrollername : windows_core::PCWSTR, dnsdomainname : windows_core::PCWSTR, phds : *mut super::winnt::HANDLE) -> u32);
    unsafe { DsBindW(domaincontrollername.param().abi(), dnsdomainname.param().abi(), phds as _) }
}
#[cfg(all(feature = "Win32_rpcdce", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn DsBindWithCredA<P0, P1>(domaincontrollername: P0, dnsdomainname: P1, authidentity: Option<super::rpcdce::RPC_AUTH_IDENTITY_HANDLE>, phds: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsBindWithCredA(domaincontrollername : windows_core::PCSTR, dnsdomainname : windows_core::PCSTR, authidentity : super::rpcdce::RPC_AUTH_IDENTITY_HANDLE, phds : *mut super::winnt::HANDLE) -> u32);
    unsafe { DsBindWithCredA(domaincontrollername.param().abi(), dnsdomainname.param().abi(), authidentity.unwrap_or(core::mem::zeroed()) as _, phds as _) }
}
#[cfg(all(feature = "Win32_rpcdce", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn DsBindWithCredW<P0, P1>(domaincontrollername: P0, dnsdomainname: P1, authidentity: Option<super::rpcdce::RPC_AUTH_IDENTITY_HANDLE>, phds: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsBindWithCredW(domaincontrollername : windows_core::PCWSTR, dnsdomainname : windows_core::PCWSTR, authidentity : super::rpcdce::RPC_AUTH_IDENTITY_HANDLE, phds : *mut super::winnt::HANDLE) -> u32);
    unsafe { DsBindWithCredW(domaincontrollername.param().abi(), dnsdomainname.param().abi(), authidentity.unwrap_or(core::mem::zeroed()) as _, phds as _) }
}
#[cfg(all(feature = "Win32_rpcdce", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn DsBindWithSpnA<P0, P1, P3>(domaincontrollername: P0, dnsdomainname: P1, authidentity: Option<super::rpcdce::RPC_AUTH_IDENTITY_HANDLE>, serviceprincipalname: P3, phds: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsBindWithSpnA(domaincontrollername : windows_core::PCSTR, dnsdomainname : windows_core::PCSTR, authidentity : super::rpcdce::RPC_AUTH_IDENTITY_HANDLE, serviceprincipalname : windows_core::PCSTR, phds : *mut super::winnt::HANDLE) -> u32);
    unsafe { DsBindWithSpnA(domaincontrollername.param().abi(), dnsdomainname.param().abi(), authidentity.unwrap_or(core::mem::zeroed()) as _, serviceprincipalname.param().abi(), phds as _) }
}
#[cfg(all(feature = "Win32_rpcdce", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn DsBindWithSpnExA<P0, P1, P3>(domaincontrollername: P0, dnsdomainname: P1, authidentity: Option<super::rpcdce::RPC_AUTH_IDENTITY_HANDLE>, serviceprincipalname: P3, bindflags: Option<u32>, phds: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsBindWithSpnExA(domaincontrollername : windows_core::PCSTR, dnsdomainname : windows_core::PCSTR, authidentity : super::rpcdce::RPC_AUTH_IDENTITY_HANDLE, serviceprincipalname : windows_core::PCSTR, bindflags : u32, phds : *mut super::winnt::HANDLE) -> u32);
    unsafe { DsBindWithSpnExA(domaincontrollername.param().abi(), dnsdomainname.param().abi(), authidentity.unwrap_or(core::mem::zeroed()) as _, serviceprincipalname.param().abi(), bindflags.unwrap_or(core::mem::zeroed()) as _, phds as _) }
}
#[cfg(all(feature = "Win32_rpcdce", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn DsBindWithSpnExW<P0, P1, P3>(domaincontrollername: P0, dnsdomainname: P1, authidentity: Option<super::rpcdce::RPC_AUTH_IDENTITY_HANDLE>, serviceprincipalname: P3, bindflags: Option<u32>, phds: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsBindWithSpnExW(domaincontrollername : windows_core::PCWSTR, dnsdomainname : windows_core::PCWSTR, authidentity : super::rpcdce::RPC_AUTH_IDENTITY_HANDLE, serviceprincipalname : windows_core::PCWSTR, bindflags : u32, phds : *mut super::winnt::HANDLE) -> u32);
    unsafe { DsBindWithSpnExW(domaincontrollername.param().abi(), dnsdomainname.param().abi(), authidentity.unwrap_or(core::mem::zeroed()) as _, serviceprincipalname.param().abi(), bindflags.unwrap_or(core::mem::zeroed()) as _, phds as _) }
}
#[cfg(all(feature = "Win32_rpcdce", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn DsBindWithSpnW<P0, P1, P3>(domaincontrollername: P0, dnsdomainname: P1, authidentity: Option<super::rpcdce::RPC_AUTH_IDENTITY_HANDLE>, serviceprincipalname: P3, phds: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsBindWithSpnW(domaincontrollername : windows_core::PCWSTR, dnsdomainname : windows_core::PCWSTR, authidentity : super::rpcdce::RPC_AUTH_IDENTITY_HANDLE, serviceprincipalname : windows_core::PCWSTR, phds : *mut super::winnt::HANDLE) -> u32);
    unsafe { DsBindWithSpnW(domaincontrollername.param().abi(), dnsdomainname.param().abi(), authidentity.unwrap_or(core::mem::zeroed()) as _, serviceprincipalname.param().abi(), phds as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsBindingSetTimeout(hds: super::winnt::HANDLE, ctimeoutsecs: u32) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsBindingSetTimeout(hds : super::winnt::HANDLE, ctimeoutsecs : u32) -> u32);
    unsafe { DsBindingSetTimeout(hds, ctimeoutsecs) }
}
#[inline]
pub unsafe fn DsClientMakeSpnForTargetServerA<P0, P1>(serviceclass: P0, servicename: P1, pcspnlength: *mut u32, pszspn: windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsClientMakeSpnForTargetServerA(serviceclass : windows_core::PCSTR, servicename : windows_core::PCSTR, pcspnlength : *mut u32, pszspn : windows_core::PSTR) -> u32);
    unsafe { DsClientMakeSpnForTargetServerA(serviceclass.param().abi(), servicename.param().abi(), pcspnlength as _, core::mem::transmute(pszspn)) }
}
#[inline]
pub unsafe fn DsClientMakeSpnForTargetServerW<P0, P1>(serviceclass: P0, servicename: P1, pcspnlength: *mut u32, pszspn: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsClientMakeSpnForTargetServerW(serviceclass : windows_core::PCWSTR, servicename : windows_core::PCWSTR, pcspnlength : *mut u32, pszspn : windows_core::PWSTR) -> u32);
    unsafe { DsClientMakeSpnForTargetServerW(serviceclass.param().abi(), servicename.param().abi(), pcspnlength as _, core::mem::transmute(pszspn)) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsCrackNamesA(hds: Option<super::winnt::HANDLE>, flags: DS_NAME_FLAGS, formatoffered: DS_NAME_FORMAT, formatdesired: DS_NAME_FORMAT, rpnames: &[windows_core::PCSTR], ppresult: *mut PDS_NAME_RESULTA) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsCrackNamesA(hds : super::winnt::HANDLE, flags : DS_NAME_FLAGS, formatoffered : DS_NAME_FORMAT, formatdesired : DS_NAME_FORMAT, cnames : u32, rpnames : *const windows_core::PCSTR, ppresult : *mut PDS_NAME_RESULTA) -> u32);
    unsafe { DsCrackNamesA(hds.unwrap_or(core::mem::zeroed()) as _, flags, formatoffered, formatdesired, rpnames.len().try_into().unwrap(), core::mem::transmute(rpnames.as_ptr()), ppresult as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsCrackNamesW(hds: Option<super::winnt::HANDLE>, flags: DS_NAME_FLAGS, formatoffered: DS_NAME_FORMAT, formatdesired: DS_NAME_FORMAT, rpnames: &[windows_core::PCWSTR], ppresult: *mut PDS_NAME_RESULTW) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsCrackNamesW(hds : super::winnt::HANDLE, flags : DS_NAME_FLAGS, formatoffered : DS_NAME_FORMAT, formatdesired : DS_NAME_FORMAT, cnames : u32, rpnames : *const windows_core::PCWSTR, ppresult : *mut PDS_NAME_RESULTW) -> u32);
    unsafe { DsCrackNamesW(hds.unwrap_or(core::mem::zeroed()) as _, flags, formatoffered, formatdesired, rpnames.len().try_into().unwrap(), core::mem::transmute(rpnames.as_ptr()), ppresult as _) }
}
#[inline]
pub unsafe fn DsFreeDomainControllerInfoA(infolevel: u32, pinfo: &[u8]) {
    windows_core::link!("ntdsapi.dll" "system" fn DsFreeDomainControllerInfoA(infolevel : u32, cinfo : u32, pinfo : *const core::ffi::c_void));
    unsafe { DsFreeDomainControllerInfoA(infolevel, pinfo.len().try_into().unwrap(), core::mem::transmute(pinfo.as_ptr())) }
}
#[inline]
pub unsafe fn DsFreeDomainControllerInfoW(infolevel: u32, pinfo: &[u8]) {
    windows_core::link!("ntdsapi.dll" "system" fn DsFreeDomainControllerInfoW(infolevel : u32, cinfo : u32, pinfo : *const core::ffi::c_void));
    unsafe { DsFreeDomainControllerInfoW(infolevel, pinfo.len().try_into().unwrap(), core::mem::transmute(pinfo.as_ptr())) }
}
#[inline]
pub unsafe fn DsFreeNameResultA(presult: *const DS_NAME_RESULTA) {
    windows_core::link!("ntdsapi.dll" "system" fn DsFreeNameResultA(presult : *const DS_NAME_RESULTA));
    unsafe { DsFreeNameResultA(presult) }
}
#[inline]
pub unsafe fn DsFreeNameResultW(presult: *const DS_NAME_RESULTW) {
    windows_core::link!("ntdsapi.dll" "system" fn DsFreeNameResultW(presult : *const DS_NAME_RESULTW));
    unsafe { DsFreeNameResultW(presult) }
}
#[cfg(feature = "Win32_rpcdce")]
#[inline]
pub unsafe fn DsFreePasswordCredentials(authidentity: super::rpcdce::RPC_AUTH_IDENTITY_HANDLE) {
    windows_core::link!("ntdsapi.dll" "system" fn DsFreePasswordCredentials(authidentity : super::rpcdce::RPC_AUTH_IDENTITY_HANDLE));
    unsafe { DsFreePasswordCredentials(authidentity) }
}
#[inline]
pub unsafe fn DsFreeSchemaGuidMapA(pguidmap: *const DS_SCHEMA_GUID_MAPA) {
    windows_core::link!("ntdsapi.dll" "system" fn DsFreeSchemaGuidMapA(pguidmap : *const DS_SCHEMA_GUID_MAPA));
    unsafe { DsFreeSchemaGuidMapA(pguidmap) }
}
#[inline]
pub unsafe fn DsFreeSchemaGuidMapW(pguidmap: *const DS_SCHEMA_GUID_MAPW) {
    windows_core::link!("ntdsapi.dll" "system" fn DsFreeSchemaGuidMapW(pguidmap : *const DS_SCHEMA_GUID_MAPW));
    unsafe { DsFreeSchemaGuidMapW(pguidmap) }
}
#[inline]
pub unsafe fn DsFreeSpnArrayA(rpszspn: &mut [windows_core::PSTR]) {
    windows_core::link!("ntdsapi.dll" "system" fn DsFreeSpnArrayA(cspn : u32, rpszspn : *mut windows_core::PSTR));
    unsafe { DsFreeSpnArrayA(rpszspn.len().try_into().unwrap(), core::mem::transmute(rpszspn.as_ptr())) }
}
#[inline]
pub unsafe fn DsFreeSpnArrayW(rpszspn: &mut [windows_core::PWSTR]) {
    windows_core::link!("ntdsapi.dll" "system" fn DsFreeSpnArrayW(cspn : u32, rpszspn : *mut windows_core::PWSTR));
    unsafe { DsFreeSpnArrayW(rpszspn.len().try_into().unwrap(), core::mem::transmute(rpszspn.as_ptr())) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsGetDomainControllerInfoA<P1>(hds: super::winnt::HANDLE, domainname: P1, infolevel: u32, pcout: *mut u32, ppinfo: *mut *mut core::ffi::c_void) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsGetDomainControllerInfoA(hds : super::winnt::HANDLE, domainname : windows_core::PCSTR, infolevel : u32, pcout : *mut u32, ppinfo : *mut *mut core::ffi::c_void) -> u32);
    unsafe { DsGetDomainControllerInfoA(hds, domainname.param().abi(), infolevel, pcout as _, ppinfo as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsGetDomainControllerInfoW<P1>(hds: super::winnt::HANDLE, domainname: P1, infolevel: u32, pcout: *mut u32, ppinfo: *mut *mut core::ffi::c_void) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsGetDomainControllerInfoW(hds : super::winnt::HANDLE, domainname : windows_core::PCWSTR, infolevel : u32, pcout : *mut u32, ppinfo : *mut *mut core::ffi::c_void) -> u32);
    unsafe { DsGetDomainControllerInfoW(hds, domainname.param().abi(), infolevel, pcout as _, ppinfo as _) }
}
#[inline]
pub unsafe fn DsGetSpnA<P1, P2>(servicetype: DS_SPN_NAME_TYPE, serviceclass: P1, servicename: P2, instanceport: u16, cinstancenames: u16, pinstancenames: Option<*const windows_core::PCSTR>, pinstanceports: Option<*const u16>, pcspn: *mut u32, prpszspn: *mut *mut windows_core::PSTR) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsGetSpnA(servicetype : DS_SPN_NAME_TYPE, serviceclass : windows_core::PCSTR, servicename : windows_core::PCSTR, instanceport : u16, cinstancenames : u16, pinstancenames : *const windows_core::PCSTR, pinstanceports : *const u16, pcspn : *mut u32, prpszspn : *mut *mut windows_core::PSTR) -> u32);
    unsafe { DsGetSpnA(servicetype, serviceclass.param().abi(), servicename.param().abi(), instanceport, cinstancenames, pinstancenames.unwrap_or(core::mem::zeroed()) as _, pinstanceports.unwrap_or(core::mem::zeroed()) as _, pcspn as _, prpszspn as _) }
}
#[inline]
pub unsafe fn DsGetSpnW<P1, P2>(servicetype: DS_SPN_NAME_TYPE, serviceclass: P1, servicename: P2, instanceport: u16, cinstancenames: u16, pinstancenames: Option<*const windows_core::PCWSTR>, pinstanceports: Option<*const u16>, pcspn: *mut u32, prpszspn: *mut *mut windows_core::PWSTR) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsGetSpnW(servicetype : DS_SPN_NAME_TYPE, serviceclass : windows_core::PCWSTR, servicename : windows_core::PCWSTR, instanceport : u16, cinstancenames : u16, pinstancenames : *const windows_core::PCWSTR, pinstanceports : *const u16, pcspn : *mut u32, prpszspn : *mut *mut windows_core::PWSTR) -> u32);
    unsafe { DsGetSpnW(servicetype, serviceclass.param().abi(), servicename.param().abi(), instanceport, cinstancenames, pinstancenames.unwrap_or(core::mem::zeroed()) as _, pinstanceports.unwrap_or(core::mem::zeroed()) as _, pcspn as _, prpszspn as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsInheritSecurityIdentityA<P2, P3>(hds: super::winnt::HANDLE, flags: Option<u32>, srcprincipal: P2, dstprincipal: P3) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsInheritSecurityIdentityA(hds : super::winnt::HANDLE, flags : u32, srcprincipal : windows_core::PCSTR, dstprincipal : windows_core::PCSTR) -> u32);
    unsafe { DsInheritSecurityIdentityA(hds, flags.unwrap_or(core::mem::zeroed()) as _, srcprincipal.param().abi(), dstprincipal.param().abi()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsInheritSecurityIdentityW<P2, P3>(hds: super::winnt::HANDLE, flags: Option<u32>, srcprincipal: P2, dstprincipal: P3) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsInheritSecurityIdentityW(hds : super::winnt::HANDLE, flags : u32, srcprincipal : windows_core::PCWSTR, dstprincipal : windows_core::PCWSTR) -> u32);
    unsafe { DsInheritSecurityIdentityW(hds, flags.unwrap_or(core::mem::zeroed()) as _, srcprincipal.param().abi(), dstprincipal.param().abi()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsListDomainsInSiteA<P1>(hds: super::winnt::HANDLE, site: P1, ppdomains: *mut PDS_NAME_RESULTA) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsListDomainsInSiteA(hds : super::winnt::HANDLE, site : windows_core::PCSTR, ppdomains : *mut PDS_NAME_RESULTA) -> u32);
    unsafe { DsListDomainsInSiteA(hds, site.param().abi(), ppdomains as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsListDomainsInSiteW<P1>(hds: super::winnt::HANDLE, site: P1, ppdomains: *mut PDS_NAME_RESULTW) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsListDomainsInSiteW(hds : super::winnt::HANDLE, site : windows_core::PCWSTR, ppdomains : *mut PDS_NAME_RESULTW) -> u32);
    unsafe { DsListDomainsInSiteW(hds, site.param().abi(), ppdomains as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsListInfoForServerA<P1>(hds: super::winnt::HANDLE, server: P1, ppinfo: *mut PDS_NAME_RESULTA) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsListInfoForServerA(hds : super::winnt::HANDLE, server : windows_core::PCSTR, ppinfo : *mut PDS_NAME_RESULTA) -> u32);
    unsafe { DsListInfoForServerA(hds, server.param().abi(), ppinfo as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsListInfoForServerW<P1>(hds: super::winnt::HANDLE, server: P1, ppinfo: *mut PDS_NAME_RESULTW) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsListInfoForServerW(hds : super::winnt::HANDLE, server : windows_core::PCWSTR, ppinfo : *mut PDS_NAME_RESULTW) -> u32);
    unsafe { DsListInfoForServerW(hds, server.param().abi(), ppinfo as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsListRolesA(hds: super::winnt::HANDLE, pproles: *mut PDS_NAME_RESULTA) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsListRolesA(hds : super::winnt::HANDLE, pproles : *mut PDS_NAME_RESULTA) -> u32);
    unsafe { DsListRolesA(hds, pproles as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsListRolesW(hds: super::winnt::HANDLE, pproles: *mut PDS_NAME_RESULTW) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsListRolesW(hds : super::winnt::HANDLE, pproles : *mut PDS_NAME_RESULTW) -> u32);
    unsafe { DsListRolesW(hds, pproles as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsListServersForDomainInSiteA<P1, P2>(hds: super::winnt::HANDLE, domain: P1, site: P2, ppservers: *mut PDS_NAME_RESULTA) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsListServersForDomainInSiteA(hds : super::winnt::HANDLE, domain : windows_core::PCSTR, site : windows_core::PCSTR, ppservers : *mut PDS_NAME_RESULTA) -> u32);
    unsafe { DsListServersForDomainInSiteA(hds, domain.param().abi(), site.param().abi(), ppservers as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsListServersForDomainInSiteW<P1, P2>(hds: super::winnt::HANDLE, domain: P1, site: P2, ppservers: *mut PDS_NAME_RESULTW) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsListServersForDomainInSiteW(hds : super::winnt::HANDLE, domain : windows_core::PCWSTR, site : windows_core::PCWSTR, ppservers : *mut PDS_NAME_RESULTW) -> u32);
    unsafe { DsListServersForDomainInSiteW(hds, domain.param().abi(), site.param().abi(), ppservers as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsListServersInSiteA<P1>(hds: super::winnt::HANDLE, site: P1, ppservers: *mut PDS_NAME_RESULTA) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsListServersInSiteA(hds : super::winnt::HANDLE, site : windows_core::PCSTR, ppservers : *mut PDS_NAME_RESULTA) -> u32);
    unsafe { DsListServersInSiteA(hds, site.param().abi(), ppservers as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsListServersInSiteW<P1>(hds: super::winnt::HANDLE, site: P1, ppservers: *mut PDS_NAME_RESULTW) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsListServersInSiteW(hds : super::winnt::HANDLE, site : windows_core::PCWSTR, ppservers : *mut PDS_NAME_RESULTW) -> u32);
    unsafe { DsListServersInSiteW(hds, site.param().abi(), ppservers as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsListSitesA(hds: super::winnt::HANDLE, ppsites: *mut PDS_NAME_RESULTA) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsListSitesA(hds : super::winnt::HANDLE, ppsites : *mut PDS_NAME_RESULTA) -> u32);
    unsafe { DsListSitesA(hds, ppsites as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsListSitesW(hds: super::winnt::HANDLE, ppsites: *mut PDS_NAME_RESULTW) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsListSitesW(hds : super::winnt::HANDLE, ppsites : *mut PDS_NAME_RESULTW) -> u32);
    unsafe { DsListSitesW(hds, ppsites as _) }
}
#[cfg(feature = "Win32_rpcdce")]
#[inline]
pub unsafe fn DsMakePasswordCredentialsA<P0, P1, P2>(user: P0, domain: P1, password: P2, pauthidentity: *mut super::rpcdce::RPC_AUTH_IDENTITY_HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsMakePasswordCredentialsA(user : windows_core::PCSTR, domain : windows_core::PCSTR, password : windows_core::PCSTR, pauthidentity : *mut super::rpcdce::RPC_AUTH_IDENTITY_HANDLE) -> u32);
    unsafe { DsMakePasswordCredentialsA(user.param().abi(), domain.param().abi(), password.param().abi(), pauthidentity as _) }
}
#[cfg(feature = "Win32_rpcdce")]
#[inline]
pub unsafe fn DsMakePasswordCredentialsW<P0, P1, P2>(user: P0, domain: P1, password: P2, pauthidentity: *mut super::rpcdce::RPC_AUTH_IDENTITY_HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsMakePasswordCredentialsW(user : windows_core::PCWSTR, domain : windows_core::PCWSTR, password : windows_core::PCWSTR, pauthidentity : *mut super::rpcdce::RPC_AUTH_IDENTITY_HANDLE) -> u32);
    unsafe { DsMakePasswordCredentialsW(user.param().abi(), domain.param().abi(), password.param().abi(), pauthidentity as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsMapSchemaGuidsA(hds: super::winnt::HANDLE, rguids: &[windows_core::GUID], ppguidmap: *mut *mut DS_SCHEMA_GUID_MAPA) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsMapSchemaGuidsA(hds : super::winnt::HANDLE, cguids : u32, rguids : *const windows_core::GUID, ppguidmap : *mut *mut DS_SCHEMA_GUID_MAPA) -> u32);
    unsafe { DsMapSchemaGuidsA(hds, rguids.len().try_into().unwrap(), core::mem::transmute(rguids.as_ptr()), ppguidmap as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsMapSchemaGuidsW(hds: super::winnt::HANDLE, rguids: &[windows_core::GUID], ppguidmap: *mut *mut DS_SCHEMA_GUID_MAPW) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsMapSchemaGuidsW(hds : super::winnt::HANDLE, cguids : u32, rguids : *const windows_core::GUID, ppguidmap : *mut *mut DS_SCHEMA_GUID_MAPW) -> u32);
    unsafe { DsMapSchemaGuidsW(hds, rguids.len().try_into().unwrap(), core::mem::transmute(rguids.as_ptr()), ppguidmap as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsQuerySitesByCostA<P1>(hds: super::winnt::HANDLE, pszfromsite: P1, rgsztosites: &[windows_core::PCSTR], dwflags: Option<u32>, prgsiteinfo: *mut PDS_SITE_COST_INFO) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsQuerySitesByCostA(hds : super::winnt::HANDLE, pszfromsite : windows_core::PCSTR, rgsztosites : *const windows_core::PCSTR, ctosites : u32, dwflags : u32, prgsiteinfo : *mut PDS_SITE_COST_INFO) -> u32);
    unsafe { DsQuerySitesByCostA(hds, pszfromsite.param().abi(), core::mem::transmute(rgsztosites.as_ptr()), rgsztosites.len().try_into().unwrap(), dwflags.unwrap_or(core::mem::zeroed()) as _, prgsiteinfo as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsQuerySitesByCostW<P1>(hds: super::winnt::HANDLE, pwszfromsite: P1, rgwsztosites: &[windows_core::PCWSTR], dwflags: Option<u32>, prgsiteinfo: *mut PDS_SITE_COST_INFO) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsQuerySitesByCostW(hds : super::winnt::HANDLE, pwszfromsite : windows_core::PCWSTR, rgwsztosites : *const windows_core::PCWSTR, ctosites : u32, dwflags : u32, prgsiteinfo : *mut PDS_SITE_COST_INFO) -> u32);
    unsafe { DsQuerySitesByCostW(hds, pwszfromsite.param().abi(), core::mem::transmute(rgwsztosites.as_ptr()), rgwsztosites.len().try_into().unwrap(), dwflags.unwrap_or(core::mem::zeroed()) as _, prgsiteinfo as _) }
}
#[inline]
pub unsafe fn DsQuerySitesFree(rgsiteinfo: *const DS_SITE_COST_INFO) {
    windows_core::link!("ntdsapi.dll" "system" fn DsQuerySitesFree(rgsiteinfo : *const DS_SITE_COST_INFO));
    unsafe { DsQuerySitesFree(rgsiteinfo) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsRemoveDsDomainA<P1>(hds: super::winnt::HANDLE, domaindn: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsRemoveDsDomainA(hds : super::winnt::HANDLE, domaindn : windows_core::PCSTR) -> u32);
    unsafe { DsRemoveDsDomainA(hds, domaindn.param().abi()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsRemoveDsDomainW<P1>(hds: super::winnt::HANDLE, domaindn: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsRemoveDsDomainW(hds : super::winnt::HANDLE, domaindn : windows_core::PCWSTR) -> u32);
    unsafe { DsRemoveDsDomainW(hds, domaindn.param().abi()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsRemoveDsServerA<P1, P2>(hds: super::winnt::HANDLE, serverdn: P1, domaindn: P2, flastdcindomain: Option<*mut windows_core::BOOL>, fcommit: bool) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsRemoveDsServerA(hds : super::winnt::HANDLE, serverdn : windows_core::PCSTR, domaindn : windows_core::PCSTR, flastdcindomain : *mut windows_core::BOOL, fcommit : windows_core::BOOL) -> u32);
    unsafe { DsRemoveDsServerA(hds, serverdn.param().abi(), domaindn.param().abi(), flastdcindomain.unwrap_or(core::mem::zeroed()) as _, fcommit.into()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsRemoveDsServerW<P1, P2>(hds: super::winnt::HANDLE, serverdn: P1, domaindn: P2, flastdcindomain: Option<*mut windows_core::BOOL>, fcommit: bool) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsRemoveDsServerW(hds : super::winnt::HANDLE, serverdn : windows_core::PCWSTR, domaindn : windows_core::PCWSTR, flastdcindomain : *mut windows_core::BOOL, fcommit : windows_core::BOOL) -> u32);
    unsafe { DsRemoveDsServerW(hds, serverdn.param().abi(), domaindn.param().abi(), flastdcindomain.unwrap_or(core::mem::zeroed()) as _, fcommit.into()) }
}
#[cfg(all(feature = "Win32_schedule", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn DsReplicaAddA<P1, P2, P3, P4>(hds: super::winnt::HANDLE, namecontext: P1, sourcedsadn: P2, transportdn: P3, sourcedsaaddress: P4, pschedule: Option<*const super::schedule::SCHEDULE>, options: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaAddA(hds : super::winnt::HANDLE, namecontext : windows_core::PCSTR, sourcedsadn : windows_core::PCSTR, transportdn : windows_core::PCSTR, sourcedsaaddress : windows_core::PCSTR, pschedule : *const super::schedule::SCHEDULE, options : u32) -> u32);
    unsafe { DsReplicaAddA(hds, namecontext.param().abi(), sourcedsadn.param().abi(), transportdn.param().abi(), sourcedsaaddress.param().abi(), pschedule.unwrap_or(core::mem::zeroed()) as _, options) }
}
#[cfg(all(feature = "Win32_schedule", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn DsReplicaAddW<P1, P2, P3, P4>(hds: super::winnt::HANDLE, namecontext: P1, sourcedsadn: P2, transportdn: P3, sourcedsaaddress: P4, pschedule: Option<*const super::schedule::SCHEDULE>, options: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaAddW(hds : super::winnt::HANDLE, namecontext : windows_core::PCWSTR, sourcedsadn : windows_core::PCWSTR, transportdn : windows_core::PCWSTR, sourcedsaaddress : windows_core::PCWSTR, pschedule : *const super::schedule::SCHEDULE, options : u32) -> u32);
    unsafe { DsReplicaAddW(hds, namecontext.param().abi(), sourcedsadn.param().abi(), transportdn.param().abi(), sourcedsaaddress.param().abi(), pschedule.unwrap_or(core::mem::zeroed()) as _, options) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsReplicaConsistencyCheck(hds: super::winnt::HANDLE, taskid: DS_KCC_TASKID, dwflags: u32) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaConsistencyCheck(hds : super::winnt::HANDLE, taskid : DS_KCC_TASKID, dwflags : u32) -> u32);
    unsafe { DsReplicaConsistencyCheck(hds, taskid, dwflags) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsReplicaDelA<P1, P2>(hds: super::winnt::HANDLE, namecontext: P1, dsasrc: P2, options: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaDelA(hds : super::winnt::HANDLE, namecontext : windows_core::PCSTR, dsasrc : windows_core::PCSTR, options : u32) -> u32);
    unsafe { DsReplicaDelA(hds, namecontext.param().abi(), dsasrc.param().abi(), options) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsReplicaDelW<P1, P2>(hds: super::winnt::HANDLE, namecontext: P1, dsasrc: P2, options: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaDelW(hds : super::winnt::HANDLE, namecontext : windows_core::PCWSTR, dsasrc : windows_core::PCWSTR, options : u32) -> u32);
    unsafe { DsReplicaDelW(hds, namecontext.param().abi(), dsasrc.param().abi(), options) }
}
#[inline]
pub unsafe fn DsReplicaFreeInfo(infotype: DS_REPL_INFO_TYPE, pinfo: *const core::ffi::c_void) {
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaFreeInfo(infotype : DS_REPL_INFO_TYPE, pinfo : *const core::ffi::c_void));
    unsafe { DsReplicaFreeInfo(infotype, pinfo) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsReplicaGetInfo2W<P2, P4, P5>(hds: super::winnt::HANDLE, infotype: DS_REPL_INFO_TYPE, pszobject: P2, puuidforsourcedsaobjguid: Option<*const windows_core::GUID>, pszattributename: P4, pszvalue: P5, dwflags: u32, dwenumerationcontext: u32, ppinfo: *mut *mut core::ffi::c_void) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaGetInfo2W(hds : super::winnt::HANDLE, infotype : DS_REPL_INFO_TYPE, pszobject : windows_core::PCWSTR, puuidforsourcedsaobjguid : *const windows_core::GUID, pszattributename : windows_core::PCWSTR, pszvalue : windows_core::PCWSTR, dwflags : u32, dwenumerationcontext : u32, ppinfo : *mut *mut core::ffi::c_void) -> u32);
    unsafe { DsReplicaGetInfo2W(hds, infotype, pszobject.param().abi(), puuidforsourcedsaobjguid.unwrap_or(core::mem::zeroed()) as _, pszattributename.param().abi(), pszvalue.param().abi(), dwflags, dwenumerationcontext, ppinfo as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsReplicaGetInfoW<P2>(hds: super::winnt::HANDLE, infotype: DS_REPL_INFO_TYPE, pszobject: P2, puuidforsourcedsaobjguid: Option<*const windows_core::GUID>, ppinfo: *mut *mut core::ffi::c_void) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaGetInfoW(hds : super::winnt::HANDLE, infotype : DS_REPL_INFO_TYPE, pszobject : windows_core::PCWSTR, puuidforsourcedsaobjguid : *const windows_core::GUID, ppinfo : *mut *mut core::ffi::c_void) -> u32);
    unsafe { DsReplicaGetInfoW(hds, infotype, pszobject.param().abi(), puuidforsourcedsaobjguid.unwrap_or(core::mem::zeroed()) as _, ppinfo as _) }
}
#[cfg(all(feature = "Win32_schedule", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn DsReplicaModifyA<P1, P3, P4>(hds: super::winnt::HANDLE, namecontext: P1, puuidsourcedsa: Option<*const windows_core::GUID>, transportdn: P3, sourcedsaaddress: P4, pschedule: Option<*const super::schedule::SCHEDULE>, replicaflags: Option<u32>, modifyfields: u32, options: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaModifyA(hds : super::winnt::HANDLE, namecontext : windows_core::PCSTR, puuidsourcedsa : *const windows_core::GUID, transportdn : windows_core::PCSTR, sourcedsaaddress : windows_core::PCSTR, pschedule : *const super::schedule::SCHEDULE, replicaflags : u32, modifyfields : u32, options : u32) -> u32);
    unsafe { DsReplicaModifyA(hds, namecontext.param().abi(), puuidsourcedsa.unwrap_or(core::mem::zeroed()) as _, transportdn.param().abi(), sourcedsaaddress.param().abi(), pschedule.unwrap_or(core::mem::zeroed()) as _, replicaflags.unwrap_or(core::mem::zeroed()) as _, modifyfields, options) }
}
#[cfg(all(feature = "Win32_schedule", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn DsReplicaModifyW<P1, P3, P4>(hds: super::winnt::HANDLE, namecontext: P1, puuidsourcedsa: Option<*const windows_core::GUID>, transportdn: P3, sourcedsaaddress: P4, pschedule: Option<*const super::schedule::SCHEDULE>, replicaflags: u32, modifyfields: u32, options: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaModifyW(hds : super::winnt::HANDLE, namecontext : windows_core::PCWSTR, puuidsourcedsa : *const windows_core::GUID, transportdn : windows_core::PCWSTR, sourcedsaaddress : windows_core::PCWSTR, pschedule : *const super::schedule::SCHEDULE, replicaflags : u32, modifyfields : u32, options : u32) -> u32);
    unsafe { DsReplicaModifyW(hds, namecontext.param().abi(), puuidsourcedsa.unwrap_or(core::mem::zeroed()) as _, transportdn.param().abi(), sourcedsaaddress.param().abi(), pschedule.unwrap_or(core::mem::zeroed()) as _, replicaflags, modifyfields, options) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsReplicaSyncA<P1>(hds: super::winnt::HANDLE, namecontext: P1, puuiddsasrc: *const windows_core::GUID, options: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaSyncA(hds : super::winnt::HANDLE, namecontext : windows_core::PCSTR, puuiddsasrc : *const windows_core::GUID, options : u32) -> u32);
    unsafe { DsReplicaSyncA(hds, namecontext.param().abi(), puuiddsasrc, options) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsReplicaSyncAllA<P1>(hds: super::winnt::HANDLE, psznamecontext: P1, ulflags: u32, pfncallback: *mut u8, pcallbackdata: Option<*const core::ffi::c_void>, perrors: *mut *mut PDS_REPSYNCALL_ERRINFOA) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaSyncAllA(hds : super::winnt::HANDLE, psznamecontext : windows_core::PCSTR, ulflags : u32, pfncallback : *mut u8, pcallbackdata : *const core::ffi::c_void, perrors : *mut *mut PDS_REPSYNCALL_ERRINFOA) -> u32);
    unsafe { DsReplicaSyncAllA(hds, psznamecontext.param().abi(), ulflags, pfncallback as _, pcallbackdata.unwrap_or(core::mem::zeroed()) as _, perrors as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsReplicaSyncAllW<P1>(hds: super::winnt::HANDLE, psznamecontext: P1, ulflags: u32, pfncallback: *mut u8, pcallbackdata: Option<*const core::ffi::c_void>, perrors: *mut *mut PDS_REPSYNCALL_ERRINFOW) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaSyncAllW(hds : super::winnt::HANDLE, psznamecontext : windows_core::PCWSTR, ulflags : u32, pfncallback : *mut u8, pcallbackdata : *const core::ffi::c_void, perrors : *mut *mut PDS_REPSYNCALL_ERRINFOW) -> u32);
    unsafe { DsReplicaSyncAllW(hds, psznamecontext.param().abi(), ulflags, pfncallback as _, pcallbackdata.unwrap_or(core::mem::zeroed()) as _, perrors as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsReplicaSyncW<P1>(hds: super::winnt::HANDLE, namecontext: P1, puuiddsasrc: *const windows_core::GUID, options: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaSyncW(hds : super::winnt::HANDLE, namecontext : windows_core::PCWSTR, puuiddsasrc : *const windows_core::GUID, options : u32) -> u32);
    unsafe { DsReplicaSyncW(hds, namecontext.param().abi(), puuiddsasrc, options) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsReplicaUpdateRefsA<P1, P2>(hds: super::winnt::HANDLE, namecontext: P1, dsadest: P2, puuiddsadest: *const windows_core::GUID, options: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaUpdateRefsA(hds : super::winnt::HANDLE, namecontext : windows_core::PCSTR, dsadest : windows_core::PCSTR, puuiddsadest : *const windows_core::GUID, options : u32) -> u32);
    unsafe { DsReplicaUpdateRefsA(hds, namecontext.param().abi(), dsadest.param().abi(), puuiddsadest, options) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsReplicaUpdateRefsW<P1, P2>(hds: super::winnt::HANDLE, namecontext: P1, dsadest: P2, puuiddsadest: *const windows_core::GUID, options: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaUpdateRefsW(hds : super::winnt::HANDLE, namecontext : windows_core::PCWSTR, dsadest : windows_core::PCWSTR, puuiddsadest : *const windows_core::GUID, options : u32) -> u32);
    unsafe { DsReplicaUpdateRefsW(hds, namecontext.param().abi(), dsadest.param().abi(), puuiddsadest, options) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsReplicaVerifyObjectsA<P1>(hds: super::winnt::HANDLE, namecontext: P1, puuiddsasrc: *const windows_core::GUID, uloptions: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaVerifyObjectsA(hds : super::winnt::HANDLE, namecontext : windows_core::PCSTR, puuiddsasrc : *const windows_core::GUID, uloptions : u32) -> u32);
    unsafe { DsReplicaVerifyObjectsA(hds, namecontext.param().abi(), puuiddsasrc, uloptions) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsReplicaVerifyObjectsW<P1>(hds: super::winnt::HANDLE, namecontext: P1, puuiddsasrc: *const windows_core::GUID, uloptions: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsReplicaVerifyObjectsW(hds : super::winnt::HANDLE, namecontext : windows_core::PCWSTR, puuiddsasrc : *const windows_core::GUID, uloptions : u32) -> u32);
    unsafe { DsReplicaVerifyObjectsW(hds, namecontext.param().abi(), puuiddsasrc, uloptions) }
}
#[inline]
pub unsafe fn DsServerRegisterSpnA<P1, P2>(operation: DS_SPN_WRITE_OP, serviceclass: P1, userobjectdn: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsServerRegisterSpnA(operation : DS_SPN_WRITE_OP, serviceclass : windows_core::PCSTR, userobjectdn : windows_core::PCSTR) -> u32);
    unsafe { DsServerRegisterSpnA(operation, serviceclass.param().abi(), userobjectdn.param().abi()) }
}
#[inline]
pub unsafe fn DsServerRegisterSpnW<P1, P2>(operation: DS_SPN_WRITE_OP, serviceclass: P1, userobjectdn: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsServerRegisterSpnW(operation : DS_SPN_WRITE_OP, serviceclass : windows_core::PCWSTR, userobjectdn : windows_core::PCWSTR) -> u32);
    unsafe { DsServerRegisterSpnW(operation, serviceclass.param().abi(), userobjectdn.param().abi()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsUnBindA(phds: *const super::winnt::HANDLE) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsUnBindA(phds : *const super::winnt::HANDLE) -> u32);
    unsafe { DsUnBindA(phds) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsUnBindW(phds: *const super::winnt::HANDLE) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsUnBindW(phds : *const super::winnt::HANDLE) -> u32);
    unsafe { DsUnBindW(phds) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsWriteAccountSpnA<P2>(hds: super::winnt::HANDLE, operation: DS_SPN_WRITE_OP, pszaccount: P2, rpszspn: &[windows_core::PCSTR]) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsWriteAccountSpnA(hds : super::winnt::HANDLE, operation : DS_SPN_WRITE_OP, pszaccount : windows_core::PCSTR, cspn : u32, rpszspn : *const windows_core::PCSTR) -> u32);
    unsafe { DsWriteAccountSpnA(hds, operation, pszaccount.param().abi(), rpszspn.len().try_into().unwrap(), core::mem::transmute(rpszspn.as_ptr())) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsWriteAccountSpnW<P2>(hds: super::winnt::HANDLE, operation: DS_SPN_WRITE_OP, pszaccount: P2, rpszspn: &[windows_core::PCWSTR]) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsWriteAccountSpnW(hds : super::winnt::HANDLE, operation : DS_SPN_WRITE_OP, pszaccount : windows_core::PCWSTR, cspn : u32, rpszspn : *const windows_core::PCWSTR) -> u32);
    unsafe { DsWriteAccountSpnW(hds, operation, pszaccount.param().abi(), rpszspn.len().try_into().unwrap(), core::mem::transmute(rpszspn.as_ptr())) }
}
pub const ADAM_REPL_AUTHENTICATION_MODE_MUTUAL_AUTH_REQUIRED: u32 = 2;
pub const ADAM_REPL_AUTHENTICATION_MODE_NEGOTIATE: u32 = 1;
pub const ADAM_REPL_AUTHENTICATION_MODE_NEGOTIATE_PASS_THROUGH: u32 = 0;
pub const ADAM_SCP_FSMO_NAMING_STRING: windows_core::PCSTR = windows_core::s!("naming");
pub const ADAM_SCP_FSMO_NAMING_STRING_W: windows_core::PCWSTR = windows_core::w!("naming");
pub const ADAM_SCP_FSMO_SCHEMA_STRING: windows_core::PCSTR = windows_core::s!("schema");
pub const ADAM_SCP_FSMO_SCHEMA_STRING_W: windows_core::PCWSTR = windows_core::w!("schema");
pub const ADAM_SCP_FSMO_STRING: windows_core::PCSTR = windows_core::s!("fsmo:");
pub const ADAM_SCP_FSMO_STRING_W: windows_core::PCWSTR = windows_core::w!("fsmo:");
pub const ADAM_SCP_INSTANCE_NAME_STRING: windows_core::PCSTR = windows_core::s!("instance:");
pub const ADAM_SCP_INSTANCE_NAME_STRING_W: windows_core::PCWSTR = windows_core::w!("instance:");
pub const ADAM_SCP_PARTITION_STRING: windows_core::PCSTR = windows_core::s!("partition:");
pub const ADAM_SCP_PARTITION_STRING_W: windows_core::PCWSTR = windows_core::w!("partition:");
pub const ADAM_SCP_SITE_NAME_STRING: windows_core::PCSTR = windows_core::s!("site:");
pub const ADAM_SCP_SITE_NAME_STRING_W: windows_core::PCWSTR = windows_core::w!("site:");
pub const DS_BEHAVIOR_LONGHORN: u32 = 3;
pub const DS_BEHAVIOR_WIN2000: u32 = 0;
pub const DS_BEHAVIOR_WIN2003: u32 = 2;
pub const DS_BEHAVIOR_WIN2003_WITH_MIXED_DOMAINS: u32 = 1;
pub const DS_BEHAVIOR_WIN2008: u32 = 3;
pub const DS_BEHAVIOR_WIN2008R2: u32 = 4;
pub const DS_BEHAVIOR_WIN2012: u32 = 5;
pub const DS_BEHAVIOR_WIN2012R2: u32 = 6;
pub const DS_BEHAVIOR_WIN2016: u32 = 7;
pub const DS_BEHAVIOR_WIN2025: u32 = 10;
pub const DS_BEHAVIOR_WIN7: u32 = 4;
pub const DS_BEHAVIOR_WIN8: u32 = 5;
pub const DS_BEHAVIOR_WINBLUE: u32 = 6;
pub const DS_BEHAVIOR_WINTHRESHOLD: u32 = 7;
pub const DS_CANONICAL_NAME: DS_NAME_FORMAT = 7;
pub const DS_CANONICAL_NAME_EX: DS_NAME_FORMAT = 9;
pub const DS_DEFAULT_LOCALE: u32 = 1033;
pub const DS_DEFAULT_LOCALE_COMPARE_FLAGS: u32 = 200707;
pub const DS_DISPLAY_NAME: DS_NAME_FORMAT = 3;
pub const DS_DNS_DOMAIN_NAME: DS_NAME_FORMAT = 12;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_DOMAIN_CONTROLLER_INFO_1A {
    pub NetbiosName: windows_core::PSTR,
    pub DnsHostName: windows_core::PSTR,
    pub SiteName: windows_core::PSTR,
    pub ComputerObjectName: windows_core::PSTR,
    pub ServerObjectName: windows_core::PSTR,
    pub fIsPdc: windows_core::BOOL,
    pub fDsEnabled: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_DOMAIN_CONTROLLER_INFO_1W {
    pub NetbiosName: windows_core::PWSTR,
    pub DnsHostName: windows_core::PWSTR,
    pub SiteName: windows_core::PWSTR,
    pub ComputerObjectName: windows_core::PWSTR,
    pub ServerObjectName: windows_core::PWSTR,
    pub fIsPdc: windows_core::BOOL,
    pub fDsEnabled: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_DOMAIN_CONTROLLER_INFO_2A {
    pub NetbiosName: windows_core::PSTR,
    pub DnsHostName: windows_core::PSTR,
    pub SiteName: windows_core::PSTR,
    pub SiteObjectName: windows_core::PSTR,
    pub ComputerObjectName: windows_core::PSTR,
    pub ServerObjectName: windows_core::PSTR,
    pub NtdsDsaObjectName: windows_core::PSTR,
    pub fIsPdc: windows_core::BOOL,
    pub fDsEnabled: windows_core::BOOL,
    pub fIsGc: windows_core::BOOL,
    pub SiteObjectGuid: windows_core::GUID,
    pub ComputerObjectGuid: windows_core::GUID,
    pub ServerObjectGuid: windows_core::GUID,
    pub NtdsDsaObjectGuid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_DOMAIN_CONTROLLER_INFO_2W {
    pub NetbiosName: windows_core::PWSTR,
    pub DnsHostName: windows_core::PWSTR,
    pub SiteName: windows_core::PWSTR,
    pub SiteObjectName: windows_core::PWSTR,
    pub ComputerObjectName: windows_core::PWSTR,
    pub ServerObjectName: windows_core::PWSTR,
    pub NtdsDsaObjectName: windows_core::PWSTR,
    pub fIsPdc: windows_core::BOOL,
    pub fDsEnabled: windows_core::BOOL,
    pub fIsGc: windows_core::BOOL,
    pub SiteObjectGuid: windows_core::GUID,
    pub ComputerObjectGuid: windows_core::GUID,
    pub ServerObjectGuid: windows_core::GUID,
    pub NtdsDsaObjectGuid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_DOMAIN_CONTROLLER_INFO_3A {
    pub NetbiosName: windows_core::PSTR,
    pub DnsHostName: windows_core::PSTR,
    pub SiteName: windows_core::PSTR,
    pub SiteObjectName: windows_core::PSTR,
    pub ComputerObjectName: windows_core::PSTR,
    pub ServerObjectName: windows_core::PSTR,
    pub NtdsDsaObjectName: windows_core::PSTR,
    pub fIsPdc: windows_core::BOOL,
    pub fDsEnabled: windows_core::BOOL,
    pub fIsGc: windows_core::BOOL,
    pub fIsRodc: windows_core::BOOL,
    pub SiteObjectGuid: windows_core::GUID,
    pub ComputerObjectGuid: windows_core::GUID,
    pub ServerObjectGuid: windows_core::GUID,
    pub NtdsDsaObjectGuid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_DOMAIN_CONTROLLER_INFO_3W {
    pub NetbiosName: windows_core::PWSTR,
    pub DnsHostName: windows_core::PWSTR,
    pub SiteName: windows_core::PWSTR,
    pub SiteObjectName: windows_core::PWSTR,
    pub ComputerObjectName: windows_core::PWSTR,
    pub ServerObjectName: windows_core::PWSTR,
    pub NtdsDsaObjectName: windows_core::PWSTR,
    pub fIsPdc: windows_core::BOOL,
    pub fDsEnabled: windows_core::BOOL,
    pub fIsGc: windows_core::BOOL,
    pub fIsRodc: windows_core::BOOL,
    pub SiteObjectGuid: windows_core::GUID,
    pub ComputerObjectGuid: windows_core::GUID,
    pub ServerObjectGuid: windows_core::GUID,
    pub NtdsDsaObjectGuid: windows_core::GUID,
}
pub const DS_DOMAIN_SIMPLE_NAME: u32 = 8;
pub const DS_ENTERPRISE_SIMPLE_NAME: u32 = 8;
pub const DS_EXIST_ADVISORY_MODE: u32 = 1;
pub const DS_FQDN_1779_NAME: DS_NAME_FORMAT = 1;
pub const DS_INSTANCETYPE_IS_NC_HEAD: u32 = 1;
pub const DS_INSTANCETYPE_NC_COMING: u32 = 16;
pub const DS_INSTANCETYPE_NC_GOING: u32 = 32;
pub const DS_INSTANCETYPE_NC_IS_WRITEABLE: u32 = 4;
pub const DS_KCC_FLAG_ASYNC_OP: u32 = 1;
pub const DS_KCC_FLAG_DAMPED: u32 = 2;
pub type DS_KCC_TASKID = i32;
pub const DS_KCC_TASKID_UPDATE_TOPOLOGY: DS_KCC_TASKID = 0;
pub const DS_LIST_ACCOUNT_OBJECT_FOR_SERVER: u32 = 2;
pub const DS_LIST_DNS_HOST_NAME_FOR_SERVER: u32 = 1;
pub const DS_LIST_DSA_OBJECT_FOR_SERVER: u32 = 0;
pub type DS_NAME_ERROR = i32;
pub const DS_NAME_ERROR_DOMAIN_ONLY: DS_NAME_ERROR = 5;
pub const DS_NAME_ERROR_NOT_FOUND: DS_NAME_ERROR = 2;
pub const DS_NAME_ERROR_NOT_UNIQUE: DS_NAME_ERROR = 3;
pub const DS_NAME_ERROR_NO_MAPPING: DS_NAME_ERROR = 4;
pub const DS_NAME_ERROR_NO_SYNTACTICAL_MAPPING: DS_NAME_ERROR = 6;
pub const DS_NAME_ERROR_RESOLVING: DS_NAME_ERROR = 1;
pub const DS_NAME_ERROR_TRUST_REFERRAL: DS_NAME_ERROR = 7;
pub type DS_NAME_FLAGS = i32;
pub const DS_NAME_FLAG_EVAL_AT_DC: DS_NAME_FLAGS = 2;
pub const DS_NAME_FLAG_GCVERIFY: DS_NAME_FLAGS = 4;
pub const DS_NAME_FLAG_SYNTACTICAL_ONLY: DS_NAME_FLAGS = 1;
pub const DS_NAME_FLAG_TRUST_REFERRAL: DS_NAME_FLAGS = 8;
pub type DS_NAME_FORMAT = i32;
pub const DS_NAME_LEGAL_FLAGS: u32 = 1;
pub const DS_NAME_NO_ERROR: DS_NAME_ERROR = 0;
pub const DS_NAME_NO_FLAGS: DS_NAME_FLAGS = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_NAME_RESULTA {
    pub cItems: u32,
    pub rItems: PDS_NAME_RESULT_ITEMA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_NAME_RESULTW {
    pub cItems: u32,
    pub rItems: PDS_NAME_RESULT_ITEMW,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_NAME_RESULT_ITEMA {
    pub status: u32,
    pub pDomain: windows_core::PSTR,
    pub pName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_NAME_RESULT_ITEMW {
    pub status: u32,
    pub pDomain: windows_core::PWSTR,
    pub pName: windows_core::PWSTR,
}
pub const DS_NT4_ACCOUNT_NAME: DS_NAME_FORMAT = 2;
pub const DS_REPADD_ASYNCHRONOUS_OPERATION: u32 = 1;
pub const DS_REPADD_ASYNCHRONOUS_REPLICA: u32 = 32;
pub const DS_REPADD_CRITICAL: u32 = 2048;
pub const DS_REPADD_DISABLE_NOTIFICATION: u32 = 64;
pub const DS_REPADD_DISABLE_PERIODIC: u32 = 128;
pub const DS_REPADD_INITIAL: u32 = 4;
pub const DS_REPADD_INTERSITE_MESSAGING: u32 = 16;
pub const DS_REPADD_NEVER_NOTIFY: u32 = 512;
pub const DS_REPADD_NONGC_RO_REPLICA: u32 = 16777216;
pub const DS_REPADD_PERIODIC: u32 = 8;
pub const DS_REPADD_SELECT_SECRETS: u32 = 4096;
pub const DS_REPADD_TWO_WAY: u32 = 1024;
pub const DS_REPADD_USE_COMPRESSION: u32 = 256;
pub const DS_REPADD_WRITEABLE: u32 = 2;
pub const DS_REPDEL_ASYNCHRONOUS_OPERATION: u32 = 1;
pub const DS_REPDEL_IGNORE_ERRORS: u32 = 8;
pub const DS_REPDEL_INTERSITE_MESSAGING: u32 = 4;
pub const DS_REPDEL_LOCAL_ONLY: u32 = 16;
pub const DS_REPDEL_NO_SOURCE: u32 = 32;
pub const DS_REPDEL_REF_OK: u32 = 64;
pub const DS_REPDEL_WRITEABLE: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPL_ATTR_META_DATA {
    pub pszAttributeName: windows_core::PWSTR,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::minwindef::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_core::GUID,
    pub usnOriginatingChange: super::winnt::USN,
    pub usnLocalChange: super::winnt::USN,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPL_ATTR_META_DATA_2 {
    pub pszAttributeName: windows_core::PWSTR,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::minwindef::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_core::GUID,
    pub usnOriginatingChange: super::winnt::USN,
    pub usnLocalChange: super::winnt::USN,
    pub pszLastOriginatingDsaDN: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPL_ATTR_META_DATA_BLOB {
    pub oszAttributeName: u32,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::minwindef::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_core::GUID,
    pub usnOriginatingChange: super::winnt::USN,
    pub usnLocalChange: super::winnt::USN,
    pub oszLastOriginatingDsaDN: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPL_ATTR_VALUE_META_DATA {
    pub cNumEntries: u32,
    pub dwEnumerationContext: u32,
    pub rgMetaData: [DS_REPL_VALUE_META_DATA; 1],
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for DS_REPL_ATTR_VALUE_META_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPL_ATTR_VALUE_META_DATA_2 {
    pub cNumEntries: u32,
    pub dwEnumerationContext: u32,
    pub rgMetaData: [DS_REPL_VALUE_META_DATA_2; 1],
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for DS_REPL_ATTR_VALUE_META_DATA_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPL_ATTR_VALUE_META_DATA_EXT {
    pub cNumEntries: u32,
    pub dwEnumerationContext: u32,
    pub rgMetaData: [DS_REPL_VALUE_META_DATA_EXT; 1],
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for DS_REPL_ATTR_VALUE_META_DATA_EXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPL_CURSOR {
    pub uuidSourceDsaInvocationID: windows_core::GUID,
    pub usnAttributeFilter: super::winnt::USN,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPL_CURSORS {
    pub cNumCursors: u32,
    pub dwReserved: u32,
    pub rgCursor: [DS_REPL_CURSOR; 1],
}
#[cfg(feature = "Win32_winnt")]
impl Default for DS_REPL_CURSORS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPL_CURSORS_2 {
    pub cNumCursors: u32,
    pub dwEnumerationContext: u32,
    pub rgCursor: [DS_REPL_CURSOR_2; 1],
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for DS_REPL_CURSORS_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPL_CURSORS_3W {
    pub cNumCursors: u32,
    pub dwEnumerationContext: u32,
    pub rgCursor: [DS_REPL_CURSOR_3W; 1],
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for DS_REPL_CURSORS_3W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPL_CURSOR_2 {
    pub uuidSourceDsaInvocationID: windows_core::GUID,
    pub usnAttributeFilter: super::winnt::USN,
    pub ftimeLastSyncSuccess: super::minwindef::FILETIME,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPL_CURSOR_3W {
    pub uuidSourceDsaInvocationID: windows_core::GUID,
    pub usnAttributeFilter: super::winnt::USN,
    pub ftimeLastSyncSuccess: super::minwindef::FILETIME,
    pub pszSourceDsaDN: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPL_CURSOR_BLOB {
    pub uuidSourceDsaInvocationID: windows_core::GUID,
    pub usnAttributeFilter: super::winnt::USN,
    pub ftimeLastSyncSuccess: super::minwindef::FILETIME,
    pub oszSourceDsaDN: u32,
}
pub const DS_REPL_INFO_CURSORS_2_FOR_NC: DS_REPL_INFO_TYPE = 7;
pub const DS_REPL_INFO_CURSORS_3_FOR_NC: DS_REPL_INFO_TYPE = 8;
pub const DS_REPL_INFO_CURSORS_FOR_NC: DS_REPL_INFO_TYPE = 1;
pub const DS_REPL_INFO_FLAG_IMPROVE_LINKED_ATTRS: u32 = 1;
pub const DS_REPL_INFO_KCC_DSA_CONNECT_FAILURES: DS_REPL_INFO_TYPE = 3;
pub const DS_REPL_INFO_KCC_DSA_LINK_FAILURES: DS_REPL_INFO_TYPE = 4;
pub const DS_REPL_INFO_METADATA_2_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = 10;
pub const DS_REPL_INFO_METADATA_2_FOR_OBJ: DS_REPL_INFO_TYPE = 9;
pub const DS_REPL_INFO_METADATA_EXT_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = 11;
pub const DS_REPL_INFO_METADATA_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = 6;
pub const DS_REPL_INFO_METADATA_FOR_OBJ: DS_REPL_INFO_TYPE = 2;
pub const DS_REPL_INFO_NEIGHBORS: DS_REPL_INFO_TYPE = 0;
pub const DS_REPL_INFO_PENDING_OPS: DS_REPL_INFO_TYPE = 5;
pub type DS_REPL_INFO_TYPE = i32;
pub const DS_REPL_INFO_TYPE_MAX: DS_REPL_INFO_TYPE = 12;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPL_KCC_DSA_FAILURESW {
    pub cNumEntries: u32,
    pub dwReserved: u32,
    pub rgDsaFailure: [DS_REPL_KCC_DSA_FAILUREW; 1],
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DS_REPL_KCC_DSA_FAILURESW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPL_KCC_DSA_FAILUREW {
    pub pszDsaDN: windows_core::PWSTR,
    pub uuidDsaObjGuid: windows_core::GUID,
    pub ftimeFirstFailure: super::minwindef::FILETIME,
    pub cNumFailures: u32,
    pub dwLastResult: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPL_KCC_DSA_FAILUREW_BLOB {
    pub oszDsaDN: u32,
    pub uuidDsaObjGuid: windows_core::GUID,
    pub ftimeFirstFailure: super::minwindef::FILETIME,
    pub cNumFailures: u32,
    pub dwLastResult: u32,
}
pub const DS_REPL_NBR_COMPRESS_CHANGES: u32 = 268435456;
pub const DS_REPL_NBR_DISABLE_SCHEDULED_SYNC: u32 = 134217728;
pub const DS_REPL_NBR_DO_SCHEDULED_SYNCS: u32 = 64;
pub const DS_REPL_NBR_FULL_SYNC_IN_PROGRESS: u32 = 65536;
pub const DS_REPL_NBR_FULL_SYNC_NEXT_PACKET: u32 = 131072;
pub const DS_REPL_NBR_GCSPN: u32 = 1048576;
pub const DS_REPL_NBR_IGNORE_CHANGE_NOTIFICATIONS: u32 = 67108864;
pub const DS_REPL_NBR_MODIFIABLE_MASK: u32 = 1006633568;
pub const DS_REPL_NBR_NEVER_SYNCED: u32 = 2097152;
pub const DS_REPL_NBR_NONGC_RO_REPLICA: u32 = 1024;
pub const DS_REPL_NBR_NO_CHANGE_NOTIFICATIONS: u32 = 536870912;
pub const DS_REPL_NBR_PARTIAL_ATTRIBUTE_SET: u32 = 1073741824;
pub const DS_REPL_NBR_PREEMPTED: u32 = 16777216;
pub const DS_REPL_NBR_RETURN_OBJECT_PARENTS: u32 = 2048;
pub const DS_REPL_NBR_SELECT_SECRETS: u32 = 4096;
pub const DS_REPL_NBR_SYNC_ON_STARTUP: u32 = 32;
pub const DS_REPL_NBR_TWO_WAY_SYNC: u32 = 512;
pub const DS_REPL_NBR_USE_ASYNC_INTERSITE_TRANSPORT: u32 = 128;
pub const DS_REPL_NBR_WRITEABLE: u32 = 16;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPL_NEIGHBORSW {
    pub cNumNeighbors: u32,
    pub dwReserved: u32,
    pub rgNeighbor: [DS_REPL_NEIGHBORW; 1],
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for DS_REPL_NEIGHBORSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPL_NEIGHBORW {
    pub pszNamingContext: windows_core::PWSTR,
    pub pszSourceDsaDN: windows_core::PWSTR,
    pub pszSourceDsaAddress: windows_core::PWSTR,
    pub pszAsyncIntersiteTransportDN: windows_core::PWSTR,
    pub dwReplicaFlags: u32,
    pub dwReserved: u32,
    pub uuidNamingContextObjGuid: windows_core::GUID,
    pub uuidSourceDsaObjGuid: windows_core::GUID,
    pub uuidSourceDsaInvocationID: windows_core::GUID,
    pub uuidAsyncIntersiteTransportObjGuid: windows_core::GUID,
    pub usnLastObjChangeSynced: super::winnt::USN,
    pub usnAttributeFilter: super::winnt::USN,
    pub ftimeLastSyncSuccess: super::minwindef::FILETIME,
    pub ftimeLastSyncAttempt: super::minwindef::FILETIME,
    pub dwLastSyncResult: u32,
    pub cNumConsecutiveSyncFailures: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPL_NEIGHBORW_BLOB {
    pub oszNamingContext: u32,
    pub oszSourceDsaDN: u32,
    pub oszSourceDsaAddress: u32,
    pub oszAsyncIntersiteTransportDN: u32,
    pub dwReplicaFlags: u32,
    pub dwReserved: u32,
    pub uuidNamingContextObjGuid: windows_core::GUID,
    pub uuidSourceDsaObjGuid: windows_core::GUID,
    pub uuidSourceDsaInvocationID: windows_core::GUID,
    pub uuidAsyncIntersiteTransportObjGuid: windows_core::GUID,
    pub usnLastObjChangeSynced: super::winnt::USN,
    pub usnAttributeFilter: super::winnt::USN,
    pub ftimeLastSyncSuccess: super::minwindef::FILETIME,
    pub ftimeLastSyncAttempt: super::minwindef::FILETIME,
    pub dwLastSyncResult: u32,
    pub cNumConsecutiveSyncFailures: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPL_OBJ_META_DATA {
    pub cNumEntries: u32,
    pub dwReserved: u32,
    pub rgMetaData: [DS_REPL_ATTR_META_DATA; 1],
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for DS_REPL_OBJ_META_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPL_OBJ_META_DATA_2 {
    pub cNumEntries: u32,
    pub dwReserved: u32,
    pub rgMetaData: [DS_REPL_ATTR_META_DATA_2; 1],
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for DS_REPL_OBJ_META_DATA_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPL_OPW {
    pub ftimeEnqueued: super::minwindef::FILETIME,
    pub ulSerialNumber: u32,
    pub ulPriority: u32,
    pub OpType: DS_REPL_OP_TYPE,
    pub ulOptions: u32,
    pub pszNamingContext: windows_core::PWSTR,
    pub pszDsaDN: windows_core::PWSTR,
    pub pszDsaAddress: windows_core::PWSTR,
    pub uuidNamingContextObjGuid: windows_core::GUID,
    pub uuidDsaObjGuid: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPL_OPW_BLOB {
    pub ftimeEnqueued: super::minwindef::FILETIME,
    pub ulSerialNumber: u32,
    pub ulPriority: u32,
    pub OpType: DS_REPL_OP_TYPE,
    pub ulOptions: u32,
    pub oszNamingContext: u32,
    pub oszDsaDN: u32,
    pub oszDsaAddress: u32,
    pub uuidNamingContextObjGuid: windows_core::GUID,
    pub uuidDsaObjGuid: windows_core::GUID,
}
pub type DS_REPL_OP_TYPE = i32;
pub const DS_REPL_OP_TYPE_ADD: DS_REPL_OP_TYPE = 1;
pub const DS_REPL_OP_TYPE_DELETE: DS_REPL_OP_TYPE = 2;
pub const DS_REPL_OP_TYPE_MODIFY: DS_REPL_OP_TYPE = 3;
pub const DS_REPL_OP_TYPE_SYNC: DS_REPL_OP_TYPE = 0;
pub const DS_REPL_OP_TYPE_UPDATE_REFS: DS_REPL_OP_TYPE = 4;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPL_PENDING_OPSW {
    pub ftimeCurrentOpStarted: super::minwindef::FILETIME,
    pub cNumPendingOps: u32,
    pub rgPendingOp: [DS_REPL_OPW; 1],
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DS_REPL_PENDING_OPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPL_QUEUE_STATISTICSW {
    pub ftimeCurrentOpStarted: super::minwindef::FILETIME,
    pub cNumPendingOps: u32,
    pub ftimeOldestSync: super::minwindef::FILETIME,
    pub ftimeOldestAdd: super::minwindef::FILETIME,
    pub ftimeOldestMod: super::minwindef::FILETIME,
    pub ftimeOldestDel: super::minwindef::FILETIME,
    pub ftimeOldestUpdRefs: super::minwindef::FILETIME,
}
#[cfg(feature = "Win32_minwindef")]
pub type DS_REPL_QUEUE_STATISTICSW_BLOB = DS_REPL_QUEUE_STATISTICSW;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPL_VALUE_META_DATA {
    pub pszAttributeName: windows_core::PWSTR,
    pub pszObjectDn: windows_core::PWSTR,
    pub cbData: u32,
    pub pbData: *mut u8,
    pub ftimeDeleted: super::minwindef::FILETIME,
    pub ftimeCreated: super::minwindef::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::minwindef::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_core::GUID,
    pub usnOriginatingChange: super::winnt::USN,
    pub usnLocalChange: super::winnt::USN,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for DS_REPL_VALUE_META_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPL_VALUE_META_DATA_2 {
    pub pszAttributeName: windows_core::PWSTR,
    pub pszObjectDn: windows_core::PWSTR,
    pub cbData: u32,
    pub pbData: *mut u8,
    pub ftimeDeleted: super::minwindef::FILETIME,
    pub ftimeCreated: super::minwindef::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::minwindef::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_core::GUID,
    pub usnOriginatingChange: super::winnt::USN,
    pub usnLocalChange: super::winnt::USN,
    pub pszLastOriginatingDsaDN: windows_core::PWSTR,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for DS_REPL_VALUE_META_DATA_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPL_VALUE_META_DATA_BLOB {
    pub oszAttributeName: u32,
    pub oszObjectDn: u32,
    pub cbData: u32,
    pub obData: u32,
    pub ftimeDeleted: super::minwindef::FILETIME,
    pub ftimeCreated: super::minwindef::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::minwindef::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_core::GUID,
    pub usnOriginatingChange: super::winnt::USN,
    pub usnLocalChange: super::winnt::USN,
    pub oszLastOriginatingDsaDN: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPL_VALUE_META_DATA_BLOB_EXT {
    pub oszAttributeName: u32,
    pub oszObjectDn: u32,
    pub cbData: u32,
    pub obData: u32,
    pub ftimeDeleted: super::minwindef::FILETIME,
    pub ftimeCreated: super::minwindef::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::minwindef::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_core::GUID,
    pub usnOriginatingChange: super::winnt::USN,
    pub usnLocalChange: super::winnt::USN,
    pub oszLastOriginatingDsaDN: u32,
    pub dwUserIdentifier: u32,
    pub dwPriorLinkState: u32,
    pub dwCurrentLinkState: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPL_VALUE_META_DATA_EXT {
    pub pszAttributeName: windows_core::PWSTR,
    pub pszObjectDn: windows_core::PWSTR,
    pub cbData: u32,
    pub pbData: *mut u8,
    pub ftimeDeleted: super::minwindef::FILETIME,
    pub ftimeCreated: super::minwindef::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::minwindef::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_core::GUID,
    pub usnOriginatingChange: super::winnt::USN,
    pub usnLocalChange: super::winnt::USN,
    pub pszLastOriginatingDsaDN: windows_core::PWSTR,
    pub dwUserIdentifier: u32,
    pub dwPriorLinkState: u32,
    pub dwCurrentLinkState: u32,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for DS_REPL_VALUE_META_DATA_EXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DS_REPMOD_ASYNCHRONOUS_OPERATION: u32 = 1;
pub const DS_REPMOD_UPDATE_ADDRESS: u32 = 2;
pub const DS_REPMOD_UPDATE_FLAGS: u32 = 1;
pub const DS_REPMOD_UPDATE_INSTANCE: u32 = 2;
pub const DS_REPMOD_UPDATE_RESULT: u32 = 8;
pub const DS_REPMOD_UPDATE_SCHEDULE: u32 = 4;
pub const DS_REPMOD_UPDATE_TRANSPORT: u32 = 16;
pub const DS_REPMOD_WRITEABLE: u32 = 2;
pub const DS_REPSYNCALL_ABORT_IF_SERVER_UNAVAILABLE: u32 = 1;
pub const DS_REPSYNCALL_CROSS_SITE_BOUNDARIES: u32 = 64;
pub const DS_REPSYNCALL_DO_NOT_SYNC: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPSYNCALL_ERRINFOA {
    pub pszSvrId: windows_core::PSTR,
    pub error: DS_REPSYNCALL_ERROR,
    pub dwWin32Err: u32,
    pub pszSrcId: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_REPSYNCALL_ERRINFOW {
    pub pszSvrId: windows_core::PWSTR,
    pub error: DS_REPSYNCALL_ERROR,
    pub dwWin32Err: u32,
    pub pszSrcId: windows_core::PWSTR,
}
pub type DS_REPSYNCALL_ERROR = i32;
pub type DS_REPSYNCALL_EVENT = i32;
pub const DS_REPSYNCALL_EVENT_ERROR: DS_REPSYNCALL_EVENT = 0;
pub const DS_REPSYNCALL_EVENT_FINISHED: DS_REPSYNCALL_EVENT = 3;
pub const DS_REPSYNCALL_EVENT_SYNC_COMPLETED: DS_REPSYNCALL_EVENT = 2;
pub const DS_REPSYNCALL_EVENT_SYNC_STARTED: DS_REPSYNCALL_EVENT = 1;
pub const DS_REPSYNCALL_ID_SERVERS_BY_DN: u32 = 4;
pub const DS_REPSYNCALL_NO_OPTIONS: u32 = 0;
pub const DS_REPSYNCALL_PUSH_CHANGES_OUTWARD: u32 = 32;
pub const DS_REPSYNCALL_SERVER_UNREACHABLE: DS_REPSYNCALL_ERROR = 2;
pub const DS_REPSYNCALL_SKIP_INITIAL_CHECK: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPSYNCALL_SYNCA {
    pub pszSrcId: windows_core::PSTR,
    pub pszDstId: windows_core::PSTR,
    pub pszNC: windows_core::PSTR,
    pub pguidSrc: *mut windows_core::GUID,
    pub pguidDst: *mut windows_core::GUID,
}
impl Default for DS_REPSYNCALL_SYNCA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPSYNCALL_SYNCW {
    pub pszSrcId: windows_core::PWSTR,
    pub pszDstId: windows_core::PWSTR,
    pub pszNC: windows_core::PWSTR,
    pub pguidSrc: *mut windows_core::GUID,
    pub pguidDst: *mut windows_core::GUID,
}
impl Default for DS_REPSYNCALL_SYNCW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DS_REPSYNCALL_SYNC_ADJACENT_SERVERS_ONLY: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPSYNCALL_UPDATEA {
    pub event: DS_REPSYNCALL_EVENT,
    pub pErrInfo: *mut DS_REPSYNCALL_ERRINFOA,
    pub pSync: *mut DS_REPSYNCALL_SYNCA,
}
impl Default for DS_REPSYNCALL_UPDATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS_REPSYNCALL_UPDATEW {
    pub event: DS_REPSYNCALL_EVENT,
    pub pErrInfo: *mut DS_REPSYNCALL_ERRINFOW,
    pub pSync: *mut DS_REPSYNCALL_SYNCW,
}
impl Default for DS_REPSYNCALL_UPDATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DS_REPSYNCALL_WIN32_ERROR_CONTACTING_SERVER: DS_REPSYNCALL_ERROR = 0;
pub const DS_REPSYNCALL_WIN32_ERROR_REPLICATING: DS_REPSYNCALL_ERROR = 1;
pub const DS_REPSYNC_ABANDONED: u32 = 32768;
pub const DS_REPSYNC_ADD_REFERENCE: u32 = 512;
pub const DS_REPSYNC_ASYNCHRONOUS_OPERATION: u32 = 1;
pub const DS_REPSYNC_ASYNCHRONOUS_REPLICA: u32 = 1048576;
pub const DS_REPSYNC_CRITICAL: u32 = 2097152;
pub const DS_REPSYNC_FORCE: u32 = 256;
pub const DS_REPSYNC_FULL: u32 = 32;
pub const DS_REPSYNC_FULL_IN_PROGRESS: u32 = 4194304;
pub const DS_REPSYNC_INITIAL: u32 = 8192;
pub const DS_REPSYNC_INITIAL_IN_PROGRESS: u32 = 65536;
pub const DS_REPSYNC_INTERSITE_MESSAGING: u32 = 8;
pub const DS_REPSYNC_NEVER_COMPLETED: u32 = 1024;
pub const DS_REPSYNC_NEVER_NOTIFY: u32 = 4096;
pub const DS_REPSYNC_NONGC_RO_REPLICA: u32 = 16777216;
pub const DS_REPSYNC_NOTIFICATION: u32 = 524288;
pub const DS_REPSYNC_NO_DISCARD: u32 = 128;
pub const DS_REPSYNC_PARTIAL_ATTRIBUTE_SET: u32 = 131072;
pub const DS_REPSYNC_PERIODIC: u32 = 4;
pub const DS_REPSYNC_PREEMPTED: u32 = 8388608;
pub const DS_REPSYNC_REQUEUE: u32 = 262144;
pub const DS_REPSYNC_SELECT_SECRETS: u32 = 32768;
pub const DS_REPSYNC_TWO_WAY: u32 = 2048;
pub const DS_REPSYNC_URGENT: u32 = 64;
pub const DS_REPSYNC_USE_COMPRESSION: u32 = 16384;
pub const DS_REPSYNC_WRITEABLE: u32 = 2;
pub const DS_REPUPD_ADD_REFERENCE: u32 = 4;
pub const DS_REPUPD_ASYNCHRONOUS_OPERATION: u32 = 1;
pub const DS_REPUPD_DELETE_REFERENCE: u32 = 8;
pub const DS_REPUPD_REFERENCE_GCSPN: u32 = 16;
pub const DS_REPUPD_WRITEABLE: u32 = 2;
pub const DS_ROLE_DOMAIN_OWNER: u32 = 1;
pub const DS_ROLE_INFRASTRUCTURE_OWNER: u32 = 4;
pub const DS_ROLE_PDC_OWNER: u32 = 2;
pub const DS_ROLE_RID_OWNER: u32 = 3;
pub const DS_ROLE_SCHEMA_OWNER: u32 = 0;
pub const DS_SCHEMA_GUID_ATTR: u32 = 1;
pub const DS_SCHEMA_GUID_ATTR_SET: u32 = 2;
pub const DS_SCHEMA_GUID_CLASS: u32 = 3;
pub const DS_SCHEMA_GUID_CONTROL_RIGHT: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_SCHEMA_GUID_MAPA {
    pub guid: windows_core::GUID,
    pub guidType: u32,
    pub pName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_SCHEMA_GUID_MAPW {
    pub guid: windows_core::GUID,
    pub guidType: u32,
    pub pName: windows_core::PWSTR,
}
pub const DS_SCHEMA_GUID_NOT_FOUND: u32 = 0;
pub const DS_SERVICE_PRINCIPAL_NAME: DS_NAME_FORMAT = 10;
pub const DS_SID_OR_SID_HISTORY_NAME: DS_NAME_FORMAT = 11;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_SITE_COST_INFO {
    pub errorCode: u32,
    pub cost: u32,
}
pub const DS_SPN_ADD_SPN_OP: DS_SPN_WRITE_OP = 0;
pub const DS_SPN_DELETE_SPN_OP: DS_SPN_WRITE_OP = 2;
pub const DS_SPN_DNS_HOST: DS_SPN_NAME_TYPE = 0;
pub const DS_SPN_DN_HOST: DS_SPN_NAME_TYPE = 1;
pub const DS_SPN_DOMAIN: DS_SPN_NAME_TYPE = 3;
pub type DS_SPN_NAME_TYPE = i32;
pub const DS_SPN_NB_DOMAIN: DS_SPN_NAME_TYPE = 4;
pub const DS_SPN_NB_HOST: DS_SPN_NAME_TYPE = 2;
pub const DS_SPN_REPLACE_SPN_OP: DS_SPN_WRITE_OP = 1;
pub const DS_SPN_SERVICE: DS_SPN_NAME_TYPE = 5;
pub type DS_SPN_WRITE_OP = i32;
pub const DS_SYNCED_EVENT_NAME: windows_core::PCSTR = windows_core::s!("NTDSInitialSyncsCompleted");
pub const DS_SYNCED_EVENT_NAME_W: windows_core::PCWSTR = windows_core::w!("NTDSInitialSyncsCompleted");
pub const DS_UNIQUE_ID_NAME: DS_NAME_FORMAT = 6;
pub const DS_UNKNOWN_NAME: DS_NAME_FORMAT = 0;
pub const DS_USER_PRINCIPAL_NAME: DS_NAME_FORMAT = 8;
pub const FLAG_DISABLABLE_OPTIONAL_FEATURE: u32 = 4;
pub const FLAG_DOMAIN_OPTIONAL_FEATURE: u32 = 2;
pub const FLAG_FOREST_OPTIONAL_FEATURE: u32 = 1;
pub const FLAG_SERVER_OPTIONAL_FEATURE: u32 = 8;
pub const FRSCONN_MAX_PRIORITY: u32 = 8;
pub const FRSCONN_PRIORITY_MASK: u32 = 1879048192;
pub const GUID_COMPUTRS_CONTAINER_A: windows_core::PCSTR = windows_core::s!("aa312825768811d1aded00c04fd8d5cd");
pub const GUID_COMPUTRS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("aa312825768811d1aded00c04fd8d5cd");
pub const GUID_DATABASE_32K_PAGES_OPTIONAL_FEATURE_A: windows_core::PCSTR = windows_core::s!("c62a9852731e4f75ae2473ae2775aab8");
pub const GUID_DATABASE_32K_PAGES_OPTIONAL_FEATURE_W: windows_core::PCWSTR = windows_core::w!("c62a9852731e4f75ae2473ae2775aab8");
pub const GUID_DELETED_OBJECTS_CONTAINER_A: windows_core::PCSTR = windows_core::s!("18e2ea80684f11d2b9aa00c04f79f805");
pub const GUID_DELETED_OBJECTS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("18e2ea80684f11d2b9aa00c04f79f805");
pub const GUID_DOMAIN_CONTROLLERS_CONTAINER_A: windows_core::PCSTR = windows_core::s!("a361b2ffffd211d1aa4b00c04fd7d83a");
pub const GUID_DOMAIN_CONTROLLERS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("a361b2ffffd211d1aa4b00c04fd7d83a");
pub const GUID_FOREIGNSECURITYPRINCIPALS_CONTAINER_A: windows_core::PCSTR = windows_core::s!("22b70c67d56e4efb91e9300fca3dc1aa");
pub const GUID_FOREIGNSECURITYPRINCIPALS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("22b70c67d56e4efb91e9300fca3dc1aa");
pub const GUID_INFRASTRUCTURE_CONTAINER_A: windows_core::PCSTR = windows_core::s!("2fbac1870ade11d297c400c04fd8d5cd");
pub const GUID_INFRASTRUCTURE_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("2fbac1870ade11d297c400c04fd8d5cd");
pub const GUID_KEYS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("683A24E2E8164BD3AF86AC3C2CF3F981");
pub const GUID_LOSTANDFOUND_CONTAINER_A: windows_core::PCSTR = windows_core::s!("ab8153b7768811d1aded00c04fd8d5cd");
pub const GUID_LOSTANDFOUND_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("ab8153b7768811d1aded00c04fd8d5cd");
pub const GUID_MANAGED_SERVICE_ACCOUNTS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("1EB93889E40C45DF9F0C64D23BBB6237");
pub const GUID_MICROSOFT_PROGRAM_DATA_CONTAINER_A: windows_core::PCSTR = windows_core::s!("f4be92a4c777485e878e9421d53087db");
pub const GUID_MICROSOFT_PROGRAM_DATA_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("f4be92a4c777485e878e9421d53087db");
pub const GUID_NTDS_QUOTAS_CONTAINER_A: windows_core::PCSTR = windows_core::s!("6227f0af1fc2410d8e3bb10615bb5b0f");
pub const GUID_NTDS_QUOTAS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("6227f0af1fc2410d8e3bb10615bb5b0f");
pub const GUID_PRIVILEGED_ACCESS_MANAGEMENT_OPTIONAL_FEATURE_A: windows_core::PCSTR = windows_core::s!("73e843ece8cc4046b4ab07ffe4ab5bcd");
pub const GUID_PRIVILEGED_ACCESS_MANAGEMENT_OPTIONAL_FEATURE_W: windows_core::PCWSTR = windows_core::w!("73e843ece8cc4046b4ab07ffe4ab5bcd");
pub const GUID_PROGRAM_DATA_CONTAINER_A: windows_core::PCSTR = windows_core::s!("09460c08ae1e4a4ea0f64aee7daa1e5a");
pub const GUID_PROGRAM_DATA_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("09460c08ae1e4a4ea0f64aee7daa1e5a");
pub const GUID_RECYCLE_BIN_OPTIONAL_FEATURE_A: windows_core::PCSTR = windows_core::s!("d8dc6d76d0ac5e44f3b9a7f9b6744f2a");
pub const GUID_RECYCLE_BIN_OPTIONAL_FEATURE_W: windows_core::PCWSTR = windows_core::w!("d8dc6d76d0ac5e44f3b9a7f9b6744f2a");
pub const GUID_SYSTEMS_CONTAINER_A: windows_core::PCSTR = windows_core::s!("ab1d30f3768811d1aded00c04fd8d5cd");
pub const GUID_SYSTEMS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("ab1d30f3768811d1aded00c04fd8d5cd");
pub const GUID_USERS_CONTAINER_A: windows_core::PCSTR = windows_core::s!("a9d1ca15768811d1aded00c04fd8d5cd");
pub const GUID_USERS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("a9d1ca15768811d1aded00c04fd8d5cd");
pub const NTDSAPI_BIND_ALLOW_DELEGATION: u32 = 1;
pub const NTDSAPI_BIND_FIND_BINDING: u32 = 2;
pub const NTDSAPI_BIND_FORCE_KERBEROS: u32 = 4;
pub const NTDSCONN_KCC_GC_TOPOLOGY: u32 = 1;
pub const NTDSCONN_KCC_INTERSITE_GC_TOPOLOGY: u32 = 32;
pub const NTDSCONN_KCC_INTERSITE_TOPOLOGY: u32 = 64;
pub const NTDSCONN_KCC_MINIMIZE_HOPS_TOPOLOGY: u32 = 4;
pub const NTDSCONN_KCC_NO_REASON: u32 = 0;
pub const NTDSCONN_KCC_OSCILLATING_CONNECTION_TOPOLOGY: u32 = 16;
pub const NTDSCONN_KCC_REDUNDANT_SERVER_TOPOLOGY: u32 = 512;
pub const NTDSCONN_KCC_RING_TOPOLOGY: u32 = 2;
pub const NTDSCONN_KCC_SERVER_FAILOVER_TOPOLOGY: u32 = 128;
pub const NTDSCONN_KCC_SITE_FAILOVER_TOPOLOGY: u32 = 256;
pub const NTDSCONN_KCC_STALE_SERVERS_TOPOLOGY: u32 = 8;
pub const NTDSCONN_OPT_DISABLE_INTERSITE_COMPRESSION: u32 = 16;
pub const NTDSCONN_OPT_IGNORE_SCHEDULE_MASK: u32 = 2147483648;
pub const NTDSCONN_OPT_IS_GENERATED: u32 = 1;
pub const NTDSCONN_OPT_OVERRIDE_NOTIFY_DEFAULT: u32 = 4;
pub const NTDSCONN_OPT_RODC_TOPOLOGY: u32 = 64;
pub const NTDSCONN_OPT_TWOWAY_SYNC: u32 = 2;
pub const NTDSCONN_OPT_USER_OWNED_SCHEDULE: u32 = 32;
pub const NTDSCONN_OPT_USE_NOTIFY: u32 = 8;
pub const NTDSDSA_OPT_BLOCK_RPC: u32 = 64;
pub const NTDSDSA_OPT_DISABLE_INBOUND_REPL: u32 = 2;
pub const NTDSDSA_OPT_DISABLE_NTDSCONN_XLATE: u32 = 8;
pub const NTDSDSA_OPT_DISABLE_OUTBOUND_REPL: u32 = 4;
pub const NTDSDSA_OPT_DISABLE_SPN_REGISTRATION: u32 = 16;
pub const NTDSDSA_OPT_GENERATE_OWN_TOPO: u32 = 32;
pub const NTDSDSA_OPT_IS_GC: u32 = 1;
pub const NTDSSETTINGS_DEFAULT_SERVER_REDUNDANCY: u32 = 2;
pub const NTDSSETTINGS_OPT_FORCE_KCC_W2K_ELECTION: u32 = 128;
pub const NTDSSETTINGS_OPT_FORCE_KCC_WHISTLER_BEHAVIOR: u32 = 64;
pub const NTDSSETTINGS_OPT_IS_AUTO_TOPOLOGY_DISABLED: u32 = 1;
pub const NTDSSETTINGS_OPT_IS_GROUP_CACHING_ENABLED: u32 = 32;
pub const NTDSSETTINGS_OPT_IS_INTER_SITE_AUTO_TOPOLOGY_DISABLED: u32 = 16;
pub const NTDSSETTINGS_OPT_IS_RAND_BH_SELECTION_DISABLED: u32 = 256;
pub const NTDSSETTINGS_OPT_IS_REDUNDANT_SERVER_TOPOLOGY_ENABLED: u32 = 1024;
pub const NTDSSETTINGS_OPT_IS_SCHEDULE_HASHING_ENABLED: u32 = 512;
pub const NTDSSETTINGS_OPT_IS_TOPL_CLEANUP_DISABLED: u32 = 2;
pub const NTDSSETTINGS_OPT_IS_TOPL_DETECT_STALE_DISABLED: u32 = 8;
pub const NTDSSETTINGS_OPT_IS_TOPL_MIN_HOPS_DISABLED: u32 = 4;
pub const NTDSSETTINGS_OPT_W2K3_BRIDGES_REQUIRED: u32 = 4096;
pub const NTDSSETTINGS_OPT_W2K3_IGNORE_SCHEDULES: u32 = 2048;
pub const NTDSSITECONN_OPT_DISABLE_COMPRESSION: u32 = 4;
pub const NTDSSITECONN_OPT_TWOWAY_SYNC: u32 = 2;
pub const NTDSSITECONN_OPT_USE_NOTIFY: u32 = 1;
pub const NTDSSITELINK_OPT_DISABLE_COMPRESSION: u32 = 4;
pub const NTDSSITELINK_OPT_TWOWAY_SYNC: u32 = 2;
pub const NTDSSITELINK_OPT_USE_NOTIFY: u32 = 1;
pub const NTDSTRANSPORT_OPT_BRIDGES_REQUIRED: u32 = 2;
pub const NTDSTRANSPORT_OPT_IGNORE_SCHEDULES: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_DOMAIN_CONTROLLER_INFO_1A(pub *mut DS_DOMAIN_CONTROLLER_INFO_1A);
impl PDS_DOMAIN_CONTROLLER_INFO_1A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_DOMAIN_CONTROLLER_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_DOMAIN_CONTROLLER_INFO_1W(pub *mut DS_DOMAIN_CONTROLLER_INFO_1W);
impl PDS_DOMAIN_CONTROLLER_INFO_1W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_DOMAIN_CONTROLLER_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_DOMAIN_CONTROLLER_INFO_2A(pub *mut DS_DOMAIN_CONTROLLER_INFO_2A);
impl PDS_DOMAIN_CONTROLLER_INFO_2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_DOMAIN_CONTROLLER_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_DOMAIN_CONTROLLER_INFO_2W(pub *mut DS_DOMAIN_CONTROLLER_INFO_2W);
impl PDS_DOMAIN_CONTROLLER_INFO_2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_DOMAIN_CONTROLLER_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_DOMAIN_CONTROLLER_INFO_3A(pub *mut DS_DOMAIN_CONTROLLER_INFO_3A);
impl PDS_DOMAIN_CONTROLLER_INFO_3A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_DOMAIN_CONTROLLER_INFO_3A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_DOMAIN_CONTROLLER_INFO_3W(pub *mut DS_DOMAIN_CONTROLLER_INFO_3W);
impl PDS_DOMAIN_CONTROLLER_INFO_3W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_DOMAIN_CONTROLLER_INFO_3W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_NAME_RESULTA(pub *mut DS_NAME_RESULTA);
impl PDS_NAME_RESULTA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_NAME_RESULTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_NAME_RESULTW(pub *mut DS_NAME_RESULTW);
impl PDS_NAME_RESULTW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_NAME_RESULTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_NAME_RESULT_ITEMA(pub *mut DS_NAME_RESULT_ITEMA);
impl PDS_NAME_RESULT_ITEMA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_NAME_RESULT_ITEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_NAME_RESULT_ITEMW(pub *mut DS_NAME_RESULT_ITEMW);
impl PDS_NAME_RESULT_ITEMW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_NAME_RESULT_ITEMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_REPSYNCALL_ERRINFOA(pub *mut DS_REPSYNCALL_ERRINFOA);
impl PDS_REPSYNCALL_ERRINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_REPSYNCALL_ERRINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_REPSYNCALL_ERRINFOW(pub *mut DS_REPSYNCALL_ERRINFOW);
impl PDS_REPSYNCALL_ERRINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_REPSYNCALL_ERRINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_REPSYNCALL_SYNCA(pub *mut DS_REPSYNCALL_SYNCA);
impl PDS_REPSYNCALL_SYNCA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_REPSYNCALL_SYNCA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_REPSYNCALL_SYNCW(pub *mut DS_REPSYNCALL_SYNCW);
impl PDS_REPSYNCALL_SYNCW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_REPSYNCALL_SYNCW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_REPSYNCALL_UPDATEA(pub *mut DS_REPSYNCALL_UPDATEA);
impl PDS_REPSYNCALL_UPDATEA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_REPSYNCALL_UPDATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_REPSYNCALL_UPDATEW(pub *mut DS_REPSYNCALL_UPDATEW);
impl PDS_REPSYNCALL_UPDATEW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_REPSYNCALL_UPDATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_SCHEMA_GUID_MAPA(pub *mut DS_SCHEMA_GUID_MAPA);
impl PDS_SCHEMA_GUID_MAPA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_SCHEMA_GUID_MAPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_SCHEMA_GUID_MAPW(pub *mut DS_SCHEMA_GUID_MAPW);
impl PDS_SCHEMA_GUID_MAPW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_SCHEMA_GUID_MAPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_SITE_COST_INFO(pub *mut DS_SITE_COST_INFO);
impl PDS_SITE_COST_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDS_SITE_COST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
