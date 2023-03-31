#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpAddFilterV4<P0, P1>(serveripaddress: P0, addfilterinfo: *const DHCP_FILTER_ADD_INFO, forceflag: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpAddFilterV4 ( serveripaddress : ::windows::core::PCWSTR , addfilterinfo : *const DHCP_FILTER_ADD_INFO , forceflag : super::super::Foundation:: BOOL ) -> u32 );
    DhcpAddFilterV4(serveripaddress.into_param().abi(), addfilterinfo, forceflag.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpAddSecurityGroup<P0>(pserver: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpAddSecurityGroup ( pserver : ::windows::core::PCWSTR ) -> u32 );
    DhcpAddSecurityGroup(pserver.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpAddServer(flags: u32, idinfo: *mut ::core::ffi::c_void, newserver: *mut DHCPDS_SERVER, callbackfn: *mut ::core::ffi::c_void, callbackdata: *mut ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpAddServer ( flags : u32 , idinfo : *mut ::core::ffi::c_void , newserver : *mut DHCPDS_SERVER , callbackfn : *mut ::core::ffi::c_void , callbackdata : *mut ::core::ffi::c_void ) -> u32 );
    DhcpAddServer(flags, idinfo, newserver, callbackfn, callbackdata)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpAddSubnetElement<P0>(serveripaddress: P0, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpAddSubnetElement ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , addelementinfo : *const DHCP_SUBNET_ELEMENT_DATA ) -> u32 );
    DhcpAddSubnetElement(serveripaddress.into_param().abi(), subnetaddress, addelementinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpAddSubnetElementV4<P0>(serveripaddress: P0, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V4) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpAddSubnetElementV4 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , addelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V4 ) -> u32 );
    DhcpAddSubnetElementV4(serveripaddress.into_param().abi(), subnetaddress, addelementinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpAddSubnetElementV5<P0>(serveripaddress: P0, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V5) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpAddSubnetElementV5 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , addelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V5 ) -> u32 );
    DhcpAddSubnetElementV5(serveripaddress.into_param().abi(), subnetaddress, addelementinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpAddSubnetElementV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, addelementinfo: *mut DHCP_SUBNET_ELEMENT_DATA_V6) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpAddSubnetElementV6 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : DHCP_IPV6_ADDRESS , addelementinfo : *mut DHCP_SUBNET_ELEMENT_DATA_V6 ) -> u32 );
    DhcpAddSubnetElementV6(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), addelementinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpAuditLogGetParams<P0>(serveripaddress: P0, flags: u32, auditlogdir: *mut ::windows::core::PWSTR, diskcheckinterval: *mut u32, maxlogfilessize: *mut u32, minspaceondisk: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpAuditLogGetParams ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , auditlogdir : *mut ::windows::core::PWSTR , diskcheckinterval : *mut u32 , maxlogfilessize : *mut u32 , minspaceondisk : *mut u32 ) -> u32 );
    DhcpAuditLogGetParams(serveripaddress.into_param().abi(), flags, auditlogdir, diskcheckinterval, maxlogfilessize, minspaceondisk)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpAuditLogSetParams<P0, P1>(serveripaddress: P0, flags: u32, auditlogdir: P1, diskcheckinterval: u32, maxlogfilessize: u32, minspaceondisk: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpAuditLogSetParams ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , auditlogdir : ::windows::core::PCWSTR , diskcheckinterval : u32 , maxlogfilessize : u32 , minspaceondisk : u32 ) -> u32 );
    DhcpAuditLogSetParams(serveripaddress.into_param().abi(), flags, auditlogdir.into_param().abi(), diskcheckinterval, maxlogfilessize, minspaceondisk)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpCApiCleanup() {
    ::windows_targets::link ! ( "dhcpcsvc.dll""system" fn DhcpCApiCleanup ( ) -> ( ) );
    DhcpCApiCleanup()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpCApiInitialize(version: *mut u32) -> u32 {
    ::windows_targets::link ! ( "dhcpcsvc.dll""system" fn DhcpCApiInitialize ( version : *mut u32 ) -> u32 );
    DhcpCApiInitialize(version)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpCreateClass<P0>(serveripaddress: P0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpCreateClass ( serveripaddress : ::windows::core::PCWSTR , reservedmustbezero : u32 , classinfo : *mut DHCP_CLASS_INFO ) -> u32 );
    DhcpCreateClass(serveripaddress.into_param().abi(), reservedmustbezero, classinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpCreateClassV6<P0>(serveripaddress: P0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO_V6) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpCreateClassV6 ( serveripaddress : ::windows::core::PCWSTR , reservedmustbezero : u32 , classinfo : *mut DHCP_CLASS_INFO_V6 ) -> u32 );
    DhcpCreateClassV6(serveripaddress.into_param().abi(), reservedmustbezero, classinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpCreateClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpCreateClientInfo ( serveripaddress : ::windows::core::PCWSTR , clientinfo : *const DHCP_CLIENT_INFO ) -> u32 );
    DhcpCreateClientInfo(serveripaddress.into_param().abi(), clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpCreateClientInfoV4<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_V4) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpCreateClientInfoV4 ( serveripaddress : ::windows::core::PCWSTR , clientinfo : *const DHCP_CLIENT_INFO_V4 ) -> u32 );
    DhcpCreateClientInfoV4(serveripaddress.into_param().abi(), clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpCreateClientInfoVQ<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_VQ) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpCreateClientInfoVQ ( serveripaddress : ::windows::core::PCWSTR , clientinfo : *const DHCP_CLIENT_INFO_VQ ) -> u32 );
    DhcpCreateClientInfoVQ(serveripaddress.into_param().abi(), clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpCreateOption<P0>(serveripaddress: P0, optionid: u32, optioninfo: *const DHCP_OPTION) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpCreateOption ( serveripaddress : ::windows::core::PCWSTR , optionid : u32 , optioninfo : *const DHCP_OPTION ) -> u32 );
    DhcpCreateOption(serveripaddress.into_param().abi(), optionid, optioninfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpCreateOptionV5<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, optioninfo: *mut DHCP_OPTION) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpCreateOptionV5 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , optioninfo : *mut DHCP_OPTION ) -> u32 );
    DhcpCreateOptionV5(serveripaddress.into_param().abi(), flags, optionid, classname.into_param().abi(), vendorname.into_param().abi(), optioninfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpCreateOptionV6<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, optioninfo: *mut DHCP_OPTION) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpCreateOptionV6 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , optioninfo : *mut DHCP_OPTION ) -> u32 );
    DhcpCreateOptionV6(serveripaddress.into_param().abi(), flags, optionid, classname.into_param().abi(), vendorname.into_param().abi(), optioninfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpCreateSubnet<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpCreateSubnet ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , subnetinfo : *const DHCP_SUBNET_INFO ) -> u32 );
    DhcpCreateSubnet(serveripaddress.into_param().abi(), subnetaddress, subnetinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpCreateSubnetV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut DHCP_SUBNET_INFO_V6) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpCreateSubnetV6 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : DHCP_IPV6_ADDRESS , subnetinfo : *mut DHCP_SUBNET_INFO_V6 ) -> u32 );
    DhcpCreateSubnetV6(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), subnetinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpCreateSubnetVQ<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO_VQ) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpCreateSubnetVQ ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , subnetinfo : *const DHCP_SUBNET_INFO_VQ ) -> u32 );
    DhcpCreateSubnetVQ(serveripaddress.into_param().abi(), subnetaddress, subnetinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpDeRegisterParamChange(flags: u32, reserved: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "dhcpcsvc.dll""system" fn DhcpDeRegisterParamChange ( flags : u32 , reserved : *mut ::core::ffi::c_void , event : *mut ::core::ffi::c_void ) -> u32 );
    DhcpDeRegisterParamChange(flags, reserved, event)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpDeleteClass<P0, P1>(serveripaddress: P0, reservedmustbezero: u32, classname: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpDeleteClass ( serveripaddress : ::windows::core::PCWSTR , reservedmustbezero : u32 , classname : ::windows::core::PCWSTR ) -> u32 );
    DhcpDeleteClass(serveripaddress.into_param().abi(), reservedmustbezero, classname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpDeleteClassV6<P0, P1>(serveripaddress: P0, reservedmustbezero: u32, classname: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpDeleteClassV6 ( serveripaddress : ::windows::core::PCWSTR , reservedmustbezero : u32 , classname : ::windows::core::PCWSTR ) -> u32 );
    DhcpDeleteClassV6(serveripaddress.into_param().abi(), reservedmustbezero, classname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpDeleteClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_SEARCH_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpDeleteClientInfo ( serveripaddress : ::windows::core::PCWSTR , clientinfo : *const DHCP_SEARCH_INFO ) -> u32 );
    DhcpDeleteClientInfo(serveripaddress.into_param().abi(), clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpDeleteClientInfoV6<P0>(serveripaddress: P0, clientinfo: *const DHCP_SEARCH_INFO_V6) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpDeleteClientInfoV6 ( serveripaddress : ::windows::core::PCWSTR , clientinfo : *const DHCP_SEARCH_INFO_V6 ) -> u32 );
    DhcpDeleteClientInfoV6(serveripaddress.into_param().abi(), clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpDeleteFilterV4<P0>(serveripaddress: P0, deletefilterinfo: *const DHCP_ADDR_PATTERN) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpDeleteFilterV4 ( serveripaddress : ::windows::core::PCWSTR , deletefilterinfo : *const DHCP_ADDR_PATTERN ) -> u32 );
    DhcpDeleteFilterV4(serveripaddress.into_param().abi(), deletefilterinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpDeleteServer(flags: u32, idinfo: *mut ::core::ffi::c_void, newserver: *mut DHCPDS_SERVER, callbackfn: *mut ::core::ffi::c_void, callbackdata: *mut ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpDeleteServer ( flags : u32 , idinfo : *mut ::core::ffi::c_void , newserver : *mut DHCPDS_SERVER , callbackfn : *mut ::core::ffi::c_void , callbackdata : *mut ::core::ffi::c_void ) -> u32 );
    DhcpDeleteServer(flags, idinfo, newserver, callbackfn, callbackdata)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpDeleteSubnet<P0>(serveripaddress: P0, subnetaddress: u32, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpDeleteSubnet ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , forceflag : DHCP_FORCE_FLAG ) -> u32 );
    DhcpDeleteSubnet(serveripaddress.into_param().abi(), subnetaddress, forceflag)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpDeleteSubnetV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpDeleteSubnetV6 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : DHCP_IPV6_ADDRESS , forceflag : DHCP_FORCE_FLAG ) -> u32 );
    DhcpDeleteSubnetV6(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), forceflag)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpDeleteSuperScopeV4<P0, P1>(serveripaddress: P0, superscopename: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpDeleteSuperScopeV4 ( serveripaddress : ::windows::core::PCWSTR , superscopename : ::windows::core::PCWSTR ) -> u32 );
    DhcpDeleteSuperScopeV4(serveripaddress.into_param().abi(), superscopename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpDsCleanup() {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpDsCleanup ( ) -> ( ) );
    DhcpDsCleanup()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpDsInit() -> u32 {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpDsInit ( ) -> u32 );
    DhcpDsInit()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumClasses<P0>(serveripaddress: P0, reservedmustbezero: u32, resumehandle: *mut u32, preferredmaximum: u32, classinfoarray: *mut *mut DHCP_CLASS_INFO_ARRAY, nread: *mut u32, ntotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumClasses ( serveripaddress : ::windows::core::PCWSTR , reservedmustbezero : u32 , resumehandle : *mut u32 , preferredmaximum : u32 , classinfoarray : *mut *mut DHCP_CLASS_INFO_ARRAY , nread : *mut u32 , ntotal : *mut u32 ) -> u32 );
    DhcpEnumClasses(serveripaddress.into_param().abi(), reservedmustbezero, resumehandle, preferredmaximum, classinfoarray, nread, ntotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumClassesV6<P0>(serveripaddress: P0, reservedmustbezero: u32, resumehandle: *mut u32, preferredmaximum: u32, classinfoarray: *mut *mut DHCP_CLASS_INFO_ARRAY_V6, nread: *mut u32, ntotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumClassesV6 ( serveripaddress : ::windows::core::PCWSTR , reservedmustbezero : u32 , resumehandle : *mut u32 , preferredmaximum : u32 , classinfoarray : *mut *mut DHCP_CLASS_INFO_ARRAY_V6 , nread : *mut u32 , ntotal : *mut u32 ) -> u32 );
    DhcpEnumClassesV6(serveripaddress.into_param().abi(), reservedmustbezero, resumehandle, preferredmaximum, classinfoarray, nread, ntotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumFilterV4<P0>(serveripaddress: P0, resumehandle: *mut DHCP_ADDR_PATTERN, preferredmaximum: u32, listtype: DHCP_FILTER_LIST_TYPE, enumfilterinfo: *mut *mut DHCP_FILTER_ENUM_INFO, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumFilterV4 ( serveripaddress : ::windows::core::PCWSTR , resumehandle : *mut DHCP_ADDR_PATTERN , preferredmaximum : u32 , listtype : DHCP_FILTER_LIST_TYPE , enumfilterinfo : *mut *mut DHCP_FILTER_ENUM_INFO , elementsread : *mut u32 , elementstotal : *mut u32 ) -> u32 );
    DhcpEnumFilterV4(serveripaddress.into_param().abi(), resumehandle, preferredmaximum, listtype, enumfilterinfo, elementsread, elementstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumOptionValues<P0>(serveripaddress: P0, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumOptionValues ( serveripaddress : ::windows::core::PCWSTR , scopeinfo : *const DHCP_OPTION_SCOPE_INFO , resumehandle : *mut u32 , preferredmaximum : u32 , optionvalues : *mut *mut DHCP_OPTION_VALUE_ARRAY , optionsread : *mut u32 , optionstotal : *mut u32 ) -> u32 );
    DhcpEnumOptionValues(serveripaddress.into_param().abi(), scopeinfo, resumehandle, preferredmaximum, optionvalues, optionsread, optionstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumOptionValuesV5<P0, P1, P2>(serveripaddress: P0, flags: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumOptionValuesV5 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO , resumehandle : *mut u32 , preferredmaximum : u32 , optionvalues : *mut *mut DHCP_OPTION_VALUE_ARRAY , optionsread : *mut u32 , optionstotal : *mut u32 ) -> u32 );
    DhcpEnumOptionValuesV5(serveripaddress.into_param().abi(), flags, classname.into_param().abi(), vendorname.into_param().abi(), scopeinfo, resumehandle, preferredmaximum, optionvalues, optionsread, optionstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumOptionValuesV6<P0, P1, P2>(serveripaddress: P0, flags: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumOptionValuesV6 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6 , resumehandle : *mut u32 , preferredmaximum : u32 , optionvalues : *mut *mut DHCP_OPTION_VALUE_ARRAY , optionsread : *mut u32 , optionstotal : *mut u32 ) -> u32 );
    DhcpEnumOptionValuesV6(serveripaddress.into_param().abi(), flags, classname.into_param().abi(), vendorname.into_param().abi(), scopeinfo, resumehandle, preferredmaximum, optionvalues, optionsread, optionstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumOptions<P0>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumOptions ( serveripaddress : ::windows::core::PCWSTR , resumehandle : *mut u32 , preferredmaximum : u32 , options : *mut *mut DHCP_OPTION_ARRAY , optionsread : *mut u32 , optionstotal : *mut u32 ) -> u32 );
    DhcpEnumOptions(serveripaddress.into_param().abi(), resumehandle, preferredmaximum, options, optionsread, optionstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumOptionsV5<P0, P1, P2>(serveripaddress: P0, flags: u32, classname: P1, vendorname: P2, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumOptionsV5 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , resumehandle : *mut u32 , preferredmaximum : u32 , options : *mut *mut DHCP_OPTION_ARRAY , optionsread : *mut u32 , optionstotal : *mut u32 ) -> u32 );
    DhcpEnumOptionsV5(serveripaddress.into_param().abi(), flags, classname.into_param().abi(), vendorname.into_param().abi(), resumehandle, preferredmaximum, options, optionsread, optionstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumOptionsV6<P0, P1, P2>(serveripaddress: P0, flags: u32, classname: P1, vendorname: P2, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumOptionsV6 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , resumehandle : *mut u32 , preferredmaximum : u32 , options : *mut *mut DHCP_OPTION_ARRAY , optionsread : *mut u32 , optionstotal : *mut u32 ) -> u32 );
    DhcpEnumOptionsV6(serveripaddress.into_param().abi(), flags, classname.into_param().abi(), vendorname.into_param().abi(), resumehandle, preferredmaximum, options, optionsread, optionstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumServers(flags: u32, idinfo: *mut ::core::ffi::c_void, servers: *mut *mut DHCPDS_SERVERS, callbackfn: *mut ::core::ffi::c_void, callbackdata: *mut ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumServers ( flags : u32 , idinfo : *mut ::core::ffi::c_void , servers : *mut *mut DHCPDS_SERVERS , callbackfn : *mut ::core::ffi::c_void , callbackdata : *mut ::core::ffi::c_void ) -> u32 );
    DhcpEnumServers(flags, idinfo, servers, callbackfn, callbackdata)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumSubnetClients<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumSubnetClients ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , resumehandle : *mut u32 , preferredmaximum : u32 , clientinfo : *mut *mut DHCP_CLIENT_INFO_ARRAY , clientsread : *mut u32 , clientstotal : *mut u32 ) -> u32 );
    DhcpEnumSubnetClients(serveripaddress.into_param().abi(), subnetaddress, resumehandle, preferredmaximum, clientinfo, clientsread, clientstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumSubnetClientsFilterStatusInfo<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumSubnetClientsFilterStatusInfo ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , resumehandle : *mut u32 , preferredmaximum : u32 , clientinfo : *mut *mut DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY , clientsread : *mut u32 , clientstotal : *mut u32 ) -> u32 );
    DhcpEnumSubnetClientsFilterStatusInfo(serveripaddress.into_param().abi(), subnetaddress, resumehandle, preferredmaximum, clientinfo, clientsread, clientstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumSubnetClientsV4<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V4, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumSubnetClientsV4 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , resumehandle : *mut u32 , preferredmaximum : u32 , clientinfo : *mut *mut DHCP_CLIENT_INFO_ARRAY_V4 , clientsread : *mut u32 , clientstotal : *mut u32 ) -> u32 );
    DhcpEnumSubnetClientsV4(serveripaddress.into_param().abi(), subnetaddress, resumehandle, preferredmaximum, clientinfo, clientsread, clientstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumSubnetClientsV5<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V5, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumSubnetClientsV5 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , resumehandle : *mut u32 , preferredmaximum : u32 , clientinfo : *mut *mut DHCP_CLIENT_INFO_ARRAY_V5 , clientsread : *mut u32 , clientstotal : *mut u32 ) -> u32 );
    DhcpEnumSubnetClientsV5(serveripaddress.into_param().abi(), subnetaddress, resumehandle, preferredmaximum, clientinfo, clientsread, clientstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumSubnetClientsV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, resumehandle: *mut DHCP_IPV6_ADDRESS, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V6, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumSubnetClientsV6 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : DHCP_IPV6_ADDRESS , resumehandle : *mut DHCP_IPV6_ADDRESS , preferredmaximum : u32 , clientinfo : *mut *mut DHCP_CLIENT_INFO_ARRAY_V6 , clientsread : *mut u32 , clientstotal : *mut u32 ) -> u32 );
    DhcpEnumSubnetClientsV6(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), resumehandle, preferredmaximum, clientinfo, clientsread, clientstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpEnumSubnetClientsVQ<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_VQ, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumSubnetClientsVQ ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , resumehandle : *mut u32 , preferredmaximum : u32 , clientinfo : *mut *mut DHCP_CLIENT_INFO_ARRAY_VQ , clientsread : *mut u32 , clientstotal : *mut u32 ) -> u32 );
    DhcpEnumSubnetClientsVQ(serveripaddress.into_param().abi(), subnetaddress, resumehandle, preferredmaximum, clientinfo, clientsread, clientstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumSubnetElements<P0>(serveripaddress: P0, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumSubnetElements ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , enumelementtype : DHCP_SUBNET_ELEMENT_TYPE , resumehandle : *mut u32 , preferredmaximum : u32 , enumelementinfo : *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY , elementsread : *mut u32 , elementstotal : *mut u32 ) -> u32 );
    DhcpEnumSubnetElements(serveripaddress.into_param().abi(), subnetaddress, enumelementtype, resumehandle, preferredmaximum, enumelementinfo, elementsread, elementstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumSubnetElementsV4<P0>(serveripaddress: P0, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumSubnetElementsV4 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , enumelementtype : DHCP_SUBNET_ELEMENT_TYPE , resumehandle : *mut u32 , preferredmaximum : u32 , enumelementinfo : *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 , elementsread : *mut u32 , elementstotal : *mut u32 ) -> u32 );
    DhcpEnumSubnetElementsV4(serveripaddress.into_param().abi(), subnetaddress, enumelementtype, resumehandle, preferredmaximum, enumelementinfo, elementsread, elementstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumSubnetElementsV5<P0>(serveripaddress: P0, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumSubnetElementsV5 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , enumelementtype : DHCP_SUBNET_ELEMENT_TYPE , resumehandle : *mut u32 , preferredmaximum : u32 , enumelementinfo : *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 , elementsread : *mut u32 , elementstotal : *mut u32 ) -> u32 );
    DhcpEnumSubnetElementsV5(serveripaddress.into_param().abi(), subnetaddress, enumelementtype, resumehandle, preferredmaximum, enumelementinfo, elementsread, elementstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumSubnetElementsV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE_V6, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumSubnetElementsV6 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : DHCP_IPV6_ADDRESS , enumelementtype : DHCP_SUBNET_ELEMENT_TYPE_V6 , resumehandle : *mut u32 , preferredmaximum : u32 , enumelementinfo : *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 , elementsread : *mut u32 , elementstotal : *mut u32 ) -> u32 );
    DhcpEnumSubnetElementsV6(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), enumelementtype, resumehandle, preferredmaximum, enumelementinfo, elementsread, elementstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumSubnets<P0>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, enuminfo: *mut *mut DHCP_IP_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumSubnets ( serveripaddress : ::windows::core::PCWSTR , resumehandle : *mut u32 , preferredmaximum : u32 , enuminfo : *mut *mut DHCP_IP_ARRAY , elementsread : *mut u32 , elementstotal : *mut u32 ) -> u32 );
    DhcpEnumSubnets(serveripaddress.into_param().abi(), resumehandle, preferredmaximum, enuminfo, elementsread, elementstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpEnumSubnetsV6<P0>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, enuminfo: *mut *mut DHCPV6_IP_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpEnumSubnetsV6 ( serveripaddress : ::windows::core::PCWSTR , resumehandle : *mut u32 , preferredmaximum : u32 , enuminfo : *mut *mut DHCPV6_IP_ARRAY , elementsread : *mut u32 , elementstotal : *mut u32 ) -> u32 );
    DhcpEnumSubnetsV6(serveripaddress.into_param().abi(), resumehandle, preferredmaximum, enuminfo, elementsread, elementstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetAllOptionValues<P0>(serveripaddress: P0, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, values: *mut *mut DHCP_ALL_OPTION_VALUES) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetAllOptionValues ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO , values : *mut *mut DHCP_ALL_OPTION_VALUES ) -> u32 );
    DhcpGetAllOptionValues(serveripaddress.into_param().abi(), flags, scopeinfo, values)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetAllOptionValuesV6<P0>(serveripaddress: P0, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, values: *mut *mut DHCP_ALL_OPTION_VALUES) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetAllOptionValuesV6 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6 , values : *mut *mut DHCP_ALL_OPTION_VALUES ) -> u32 );
    DhcpGetAllOptionValuesV6(serveripaddress.into_param().abi(), flags, scopeinfo, values)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetAllOptions<P0>(serveripaddress: P0, flags: u32, optionstruct: *mut *mut DHCP_ALL_OPTIONS) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetAllOptions ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionstruct : *mut *mut DHCP_ALL_OPTIONS ) -> u32 );
    DhcpGetAllOptions(serveripaddress.into_param().abi(), flags, optionstruct)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetAllOptionsV6<P0>(serveripaddress: P0, flags: u32, optionstruct: *mut *mut DHCP_ALL_OPTIONS) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetAllOptionsV6 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionstruct : *mut *mut DHCP_ALL_OPTIONS ) -> u32 );
    DhcpGetAllOptionsV6(serveripaddress.into_param().abi(), flags, optionstruct)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetClassInfo<P0>(serveripaddress: P0, reservedmustbezero: u32, partialclassinfo: *mut DHCP_CLASS_INFO, filledclassinfo: *mut *mut DHCP_CLASS_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetClassInfo ( serveripaddress : ::windows::core::PCWSTR , reservedmustbezero : u32 , partialclassinfo : *mut DHCP_CLASS_INFO , filledclassinfo : *mut *mut DHCP_CLASS_INFO ) -> u32 );
    DhcpGetClassInfo(serveripaddress.into_param().abi(), reservedmustbezero, partialclassinfo, filledclassinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetClientInfo<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetClientInfo ( serveripaddress : ::windows::core::PCWSTR , searchinfo : *const DHCP_SEARCH_INFO , clientinfo : *mut *mut DHCP_CLIENT_INFO ) -> u32 );
    DhcpGetClientInfo(serveripaddress.into_param().abi(), searchinfo, clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetClientInfoV4<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_V4) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetClientInfoV4 ( serveripaddress : ::windows::core::PCWSTR , searchinfo : *const DHCP_SEARCH_INFO , clientinfo : *mut *mut DHCP_CLIENT_INFO_V4 ) -> u32 );
    DhcpGetClientInfoV4(serveripaddress.into_param().abi(), searchinfo, clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetClientInfoV6<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO_V6, clientinfo: *mut *mut DHCP_CLIENT_INFO_V6) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetClientInfoV6 ( serveripaddress : ::windows::core::PCWSTR , searchinfo : *const DHCP_SEARCH_INFO_V6 , clientinfo : *mut *mut DHCP_CLIENT_INFO_V6 ) -> u32 );
    DhcpGetClientInfoV6(serveripaddress.into_param().abi(), searchinfo, clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetClientInfoVQ<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_VQ) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetClientInfoVQ ( serveripaddress : ::windows::core::PCWSTR , searchinfo : *const DHCP_SEARCH_INFO , clientinfo : *mut *mut DHCP_CLIENT_INFO_VQ ) -> u32 );
    DhcpGetClientInfoVQ(serveripaddress.into_param().abi(), searchinfo, clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetClientOptions<P0>(serveripaddress: P0, clientipaddress: u32, clientsubnetmask: u32, clientoptions: *mut *mut DHCP_OPTION_LIST) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetClientOptions ( serveripaddress : ::windows::core::PCWSTR , clientipaddress : u32 , clientsubnetmask : u32 , clientoptions : *mut *mut DHCP_OPTION_LIST ) -> u32 );
    DhcpGetClientOptions(serveripaddress.into_param().abi(), clientipaddress, clientsubnetmask, clientoptions)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetFilterV4<P0>(serveripaddress: P0, globalfilterinfo: *mut DHCP_FILTER_GLOBAL_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetFilterV4 ( serveripaddress : ::windows::core::PCWSTR , globalfilterinfo : *mut DHCP_FILTER_GLOBAL_INFO ) -> u32 );
    DhcpGetFilterV4(serveripaddress.into_param().abi(), globalfilterinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetMibInfo<P0>(serveripaddress: P0, mibinfo: *mut *mut DHCP_MIB_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetMibInfo ( serveripaddress : ::windows::core::PCWSTR , mibinfo : *mut *mut DHCP_MIB_INFO ) -> u32 );
    DhcpGetMibInfo(serveripaddress.into_param().abi(), mibinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetMibInfoV5<P0>(serveripaddress: P0, mibinfo: *mut *mut DHCP_MIB_INFO_V5) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetMibInfoV5 ( serveripaddress : ::windows::core::PCWSTR , mibinfo : *mut *mut DHCP_MIB_INFO_V5 ) -> u32 );
    DhcpGetMibInfoV5(serveripaddress.into_param().abi(), mibinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetMibInfoV6<P0>(serveripaddress: P0, mibinfo: *mut *mut DHCP_MIB_INFO_V6) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetMibInfoV6 ( serveripaddress : ::windows::core::PCWSTR , mibinfo : *mut *mut DHCP_MIB_INFO_V6 ) -> u32 );
    DhcpGetMibInfoV6(serveripaddress.into_param().abi(), mibinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetOptionInfo<P0>(serveripaddress: P0, optionid: u32, optioninfo: *mut *mut DHCP_OPTION) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetOptionInfo ( serveripaddress : ::windows::core::PCWSTR , optionid : u32 , optioninfo : *mut *mut DHCP_OPTION ) -> u32 );
    DhcpGetOptionInfo(serveripaddress.into_param().abi(), optionid, optioninfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetOptionInfoV5<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, optioninfo: *mut *mut DHCP_OPTION) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetOptionInfoV5 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , optioninfo : *mut *mut DHCP_OPTION ) -> u32 );
    DhcpGetOptionInfoV5(serveripaddress.into_param().abi(), flags, optionid, classname.into_param().abi(), vendorname.into_param().abi(), optioninfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetOptionInfoV6<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, optioninfo: *mut *mut DHCP_OPTION) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetOptionInfoV6 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , optioninfo : *mut *mut DHCP_OPTION ) -> u32 );
    DhcpGetOptionInfoV6(serveripaddress.into_param().abi(), flags, optionid, classname.into_param().abi(), vendorname.into_param().abi(), optioninfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetOptionValue<P0>(serveripaddress: P0, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetOptionValue ( serveripaddress : ::windows::core::PCWSTR , optionid : u32 , scopeinfo : *const DHCP_OPTION_SCOPE_INFO , optionvalue : *mut *mut DHCP_OPTION_VALUE ) -> u32 );
    DhcpGetOptionValue(serveripaddress.into_param().abi(), optionid, scopeinfo, optionvalue)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetOptionValueV5<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetOptionValueV5 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO , optionvalue : *mut *mut DHCP_OPTION_VALUE ) -> u32 );
    DhcpGetOptionValueV5(serveripaddress.into_param().abi(), flags, optionid, classname.into_param().abi(), vendorname.into_param().abi(), scopeinfo, optionvalue)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetOptionValueV6<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetOptionValueV6 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6 , optionvalue : *mut *mut DHCP_OPTION_VALUE ) -> u32 );
    DhcpGetOptionValueV6(serveripaddress.into_param().abi(), flags, optionid, classname.into_param().abi(), vendorname.into_param().abi(), scopeinfo, optionvalue)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetOriginalSubnetMask<P0>(sadaptername: P0, dwsubnetmask: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpcsvc.dll""system" fn DhcpGetOriginalSubnetMask ( sadaptername : ::windows::core::PCWSTR , dwsubnetmask : *mut u32 ) -> u32 );
    DhcpGetOriginalSubnetMask(sadaptername.into_param().abi(), dwsubnetmask)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetServerBindingInfo<P0>(serveripaddress: P0, flags: u32, bindelementsinfo: *mut *mut DHCP_BIND_ELEMENT_ARRAY) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetServerBindingInfo ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , bindelementsinfo : *mut *mut DHCP_BIND_ELEMENT_ARRAY ) -> u32 );
    DhcpGetServerBindingInfo(serveripaddress.into_param().abi(), flags, bindelementsinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpGetServerBindingInfoV6<P0>(serveripaddress: P0, flags: u32, bindelementsinfo: *mut *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetServerBindingInfoV6 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , bindelementsinfo : *mut *mut DHCPV6_BIND_ELEMENT_ARRAY ) -> u32 );
    DhcpGetServerBindingInfoV6(serveripaddress.into_param().abi(), flags, bindelementsinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetServerSpecificStrings<P0>(serveripaddress: P0, serverspecificstrings: *mut *mut DHCP_SERVER_SPECIFIC_STRINGS) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetServerSpecificStrings ( serveripaddress : ::windows::core::PCWSTR , serverspecificstrings : *mut *mut DHCP_SERVER_SPECIFIC_STRINGS ) -> u32 );
    DhcpGetServerSpecificStrings(serveripaddress.into_param().abi(), serverspecificstrings)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetSubnetDelayOffer<P0>(serveripaddress: P0, subnetaddress: u32, timedelayinmilliseconds: *mut u16) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetSubnetDelayOffer ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , timedelayinmilliseconds : *mut u16 ) -> u32 );
    DhcpGetSubnetDelayOffer(serveripaddress.into_param().abi(), subnetaddress, timedelayinmilliseconds)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetSubnetInfo<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *mut *mut DHCP_SUBNET_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetSubnetInfo ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , subnetinfo : *mut *mut DHCP_SUBNET_INFO ) -> u32 );
    DhcpGetSubnetInfo(serveripaddress.into_param().abi(), subnetaddress, subnetinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetSubnetInfoV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut *mut DHCP_SUBNET_INFO_V6) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetSubnetInfoV6 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : DHCP_IPV6_ADDRESS , subnetinfo : *mut *mut DHCP_SUBNET_INFO_V6 ) -> u32 );
    DhcpGetSubnetInfoV6(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), subnetinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetSubnetInfoVQ<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *mut *mut DHCP_SUBNET_INFO_VQ) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetSubnetInfoVQ ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , subnetinfo : *mut *mut DHCP_SUBNET_INFO_VQ ) -> u32 );
    DhcpGetSubnetInfoVQ(serveripaddress.into_param().abi(), subnetaddress, subnetinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetSuperScopeInfoV4<P0>(serveripaddress: P0, superscopetable: *mut *mut DHCP_SUPER_SCOPE_TABLE) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetSuperScopeInfoV4 ( serveripaddress : ::windows::core::PCWSTR , superscopetable : *mut *mut DHCP_SUPER_SCOPE_TABLE ) -> u32 );
    DhcpGetSuperScopeInfoV4(serveripaddress.into_param().abi(), superscopetable)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetThreadOptions(pflags: *mut u32, reserved: *mut ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetThreadOptions ( pflags : *mut u32 , reserved : *mut ::core::ffi::c_void ) -> u32 );
    DhcpGetThreadOptions(pflags, reserved)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpGetVersion<P0>(serveripaddress: P0, majorversion: *mut u32, minorversion: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpGetVersion ( serveripaddress : ::windows::core::PCWSTR , majorversion : *mut u32 , minorversion : *mut u32 ) -> u32 );
    DhcpGetVersion(serveripaddress.into_param().abi(), majorversion, minorversion)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprAddV4PolicyCondition<P0>(policy: *mut DHCP_POLICY, parentexpr: u32, r#type: DHCP_POL_ATTR_TYPE, optionid: u32, suboptionid: u32, vendorname: P0, operator: DHCP_POL_COMPARATOR, value: &[u8], conditionindex: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprAddV4PolicyCondition ( policy : *mut DHCP_POLICY , parentexpr : u32 , r#type : DHCP_POL_ATTR_TYPE , optionid : u32 , suboptionid : u32 , vendorname : ::windows::core::PCWSTR , operator : DHCP_POL_COMPARATOR , value : *const u8 , valuelength : u32 , conditionindex : *mut u32 ) -> u32 );
    DhcpHlprAddV4PolicyCondition(policy, parentexpr, r#type, optionid, suboptionid, vendorname.into_param().abi(), operator, ::core::mem::transmute(value.as_ptr()), value.len() as _, conditionindex)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprAddV4PolicyExpr(policy: *mut DHCP_POLICY, parentexpr: u32, operator: DHCP_POL_LOGIC_OPER, exprindex: *mut u32) -> u32 {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprAddV4PolicyExpr ( policy : *mut DHCP_POLICY , parentexpr : u32 , operator : DHCP_POL_LOGIC_OPER , exprindex : *mut u32 ) -> u32 );
    DhcpHlprAddV4PolicyExpr(policy, parentexpr, operator, exprindex)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprAddV4PolicyRange(policy: *mut DHCP_POLICY, range: *const DHCP_IP_RANGE) -> u32 {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprAddV4PolicyRange ( policy : *mut DHCP_POLICY , range : *const DHCP_IP_RANGE ) -> u32 );
    DhcpHlprAddV4PolicyRange(policy, range)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprCreateV4Policy<P0, P1, P2, P3>(policyname: P0, fglobalpolicy: P1, subnet: u32, processingorder: u32, rootoperator: DHCP_POL_LOGIC_OPER, description: P2, enabled: P3, policy: *mut *mut DHCP_POLICY) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprCreateV4Policy ( policyname : ::windows::core::PCWSTR , fglobalpolicy : super::super::Foundation:: BOOL , subnet : u32 , processingorder : u32 , rootoperator : DHCP_POL_LOGIC_OPER , description : ::windows::core::PCWSTR , enabled : super::super::Foundation:: BOOL , policy : *mut *mut DHCP_POLICY ) -> u32 );
    DhcpHlprCreateV4Policy(policyname.into_param().abi(), fglobalpolicy.into_param().abi(), subnet, processingorder, rootoperator, description.into_param().abi(), enabled.into_param().abi(), policy)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprCreateV4PolicyEx<P0, P1, P2, P3>(policyname: P0, fglobalpolicy: P1, subnet: u32, processingorder: u32, rootoperator: DHCP_POL_LOGIC_OPER, description: P2, enabled: P3, policy: *mut *mut DHCP_POLICY_EX) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprCreateV4PolicyEx ( policyname : ::windows::core::PCWSTR , fglobalpolicy : super::super::Foundation:: BOOL , subnet : u32 , processingorder : u32 , rootoperator : DHCP_POL_LOGIC_OPER , description : ::windows::core::PCWSTR , enabled : super::super::Foundation:: BOOL , policy : *mut *mut DHCP_POLICY_EX ) -> u32 );
    DhcpHlprCreateV4PolicyEx(policyname.into_param().abi(), fglobalpolicy.into_param().abi(), subnet, processingorder, rootoperator, description.into_param().abi(), enabled.into_param().abi(), policy)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpHlprFindV4DhcpProperty(propertyarray: *const DHCP_PROPERTY_ARRAY, id: DHCP_PROPERTY_ID, r#type: DHCP_PROPERTY_TYPE) -> *mut DHCP_PROPERTY {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprFindV4DhcpProperty ( propertyarray : *const DHCP_PROPERTY_ARRAY , id : DHCP_PROPERTY_ID , r#type : DHCP_PROPERTY_TYPE ) -> *mut DHCP_PROPERTY );
    DhcpHlprFindV4DhcpProperty(propertyarray, id, r#type)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpHlprFreeV4DhcpProperty(property: *mut DHCP_PROPERTY) {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprFreeV4DhcpProperty ( property : *mut DHCP_PROPERTY ) -> ( ) );
    DhcpHlprFreeV4DhcpProperty(property)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpHlprFreeV4DhcpPropertyArray(propertyarray: *mut DHCP_PROPERTY_ARRAY) {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprFreeV4DhcpPropertyArray ( propertyarray : *mut DHCP_PROPERTY_ARRAY ) -> ( ) );
    DhcpHlprFreeV4DhcpPropertyArray(propertyarray)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprFreeV4Policy(policy: *mut DHCP_POLICY) {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprFreeV4Policy ( policy : *mut DHCP_POLICY ) -> ( ) );
    DhcpHlprFreeV4Policy(policy)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprFreeV4PolicyArray(policyarray: *mut DHCP_POLICY_ARRAY) {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprFreeV4PolicyArray ( policyarray : *mut DHCP_POLICY_ARRAY ) -> ( ) );
    DhcpHlprFreeV4PolicyArray(policyarray)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprFreeV4PolicyEx(policyex: *mut DHCP_POLICY_EX) {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprFreeV4PolicyEx ( policyex : *mut DHCP_POLICY_EX ) -> ( ) );
    DhcpHlprFreeV4PolicyEx(policyex)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprFreeV4PolicyExArray(policyexarray: *mut DHCP_POLICY_EX_ARRAY) {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprFreeV4PolicyExArray ( policyexarray : *mut DHCP_POLICY_EX_ARRAY ) -> ( ) );
    DhcpHlprFreeV4PolicyExArray(policyexarray)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprIsV4PolicySingleUC(policy: *const DHCP_POLICY) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprIsV4PolicySingleUC ( policy : *const DHCP_POLICY ) -> super::super::Foundation:: BOOL );
    DhcpHlprIsV4PolicySingleUC(policy)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprIsV4PolicyValid(ppolicy: *const DHCP_POLICY) -> u32 {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprIsV4PolicyValid ( ppolicy : *const DHCP_POLICY ) -> u32 );
    DhcpHlprIsV4PolicyValid(ppolicy)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprIsV4PolicyWellFormed(ppolicy: *const DHCP_POLICY) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprIsV4PolicyWellFormed ( ppolicy : *const DHCP_POLICY ) -> super::super::Foundation:: BOOL );
    DhcpHlprIsV4PolicyWellFormed(ppolicy)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprModifyV4PolicyExpr(policy: *mut DHCP_POLICY, operator: DHCP_POL_LOGIC_OPER) -> u32 {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprModifyV4PolicyExpr ( policy : *mut DHCP_POLICY , operator : DHCP_POL_LOGIC_OPER ) -> u32 );
    DhcpHlprModifyV4PolicyExpr(policy, operator)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpHlprResetV4PolicyExpr(policy: *mut DHCP_POLICY) -> u32 {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpHlprResetV4PolicyExpr ( policy : *mut DHCP_POLICY ) -> u32 );
    DhcpHlprResetV4PolicyExpr(policy)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpModifyClass<P0>(serveripaddress: P0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpModifyClass ( serveripaddress : ::windows::core::PCWSTR , reservedmustbezero : u32 , classinfo : *mut DHCP_CLASS_INFO ) -> u32 );
    DhcpModifyClass(serveripaddress.into_param().abi(), reservedmustbezero, classinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpModifyClassV6<P0>(serveripaddress: P0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO_V6) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpModifyClassV6 ( serveripaddress : ::windows::core::PCWSTR , reservedmustbezero : u32 , classinfo : *mut DHCP_CLASS_INFO_V6 ) -> u32 );
    DhcpModifyClassV6(serveripaddress.into_param().abi(), reservedmustbezero, classinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpRegisterParamChange<P0>(flags: u32, reserved: ::core::option::Option<*const ::core::ffi::c_void>, adaptername: P0, classid: *mut DHCPCAPI_CLASSID, params: DHCPCAPI_PARAMS_ARRAY, handle: *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpcsvc.dll""system" fn DhcpRegisterParamChange ( flags : u32 , reserved : *const ::core::ffi::c_void , adaptername : ::windows::core::PCWSTR , classid : *mut DHCPCAPI_CLASSID , params : DHCPCAPI_PARAMS_ARRAY , handle : *mut ::core::ffi::c_void ) -> u32 );
    DhcpRegisterParamChange(flags, ::core::mem::transmute(reserved.unwrap_or(::std::ptr::null())), adaptername.into_param().abi(), classid, ::core::mem::transmute(params), handle)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpRemoveDNSRegistrations() -> u32 {
    ::windows_targets::link ! ( "dhcpcsvc.dll""system" fn DhcpRemoveDNSRegistrations ( ) -> u32 );
    DhcpRemoveDNSRegistrations()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpRemoveOption<P0>(serveripaddress: P0, optionid: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpRemoveOption ( serveripaddress : ::windows::core::PCWSTR , optionid : u32 ) -> u32 );
    DhcpRemoveOption(serveripaddress.into_param().abi(), optionid)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpRemoveOptionV5<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpRemoveOptionV5 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR ) -> u32 );
    DhcpRemoveOptionV5(serveripaddress.into_param().abi(), flags, optionid, classname.into_param().abi(), vendorname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpRemoveOptionV6<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpRemoveOptionV6 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR ) -> u32 );
    DhcpRemoveOptionV6(serveripaddress.into_param().abi(), flags, optionid, classname.into_param().abi(), vendorname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpRemoveOptionValue<P0>(serveripaddress: P0, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpRemoveOptionValue ( serveripaddress : ::windows::core::PCWSTR , optionid : u32 , scopeinfo : *const DHCP_OPTION_SCOPE_INFO ) -> u32 );
    DhcpRemoveOptionValue(serveripaddress.into_param().abi(), optionid, scopeinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpRemoveOptionValueV5<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpRemoveOptionValueV5 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO ) -> u32 );
    DhcpRemoveOptionValueV5(serveripaddress.into_param().abi(), flags, optionid, classname.into_param().abi(), vendorname.into_param().abi(), scopeinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpRemoveOptionValueV6<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpRemoveOptionValueV6 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6 ) -> u32 );
    DhcpRemoveOptionValueV6(serveripaddress.into_param().abi(), flags, optionid, classname.into_param().abi(), vendorname.into_param().abi(), scopeinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpRemoveSubnetElement<P0>(serveripaddress: P0, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpRemoveSubnetElement ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , removeelementinfo : *const DHCP_SUBNET_ELEMENT_DATA , forceflag : DHCP_FORCE_FLAG ) -> u32 );
    DhcpRemoveSubnetElement(serveripaddress.into_param().abi(), subnetaddress, removeelementinfo, forceflag)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpRemoveSubnetElementV4<P0>(serveripaddress: P0, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V4, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpRemoveSubnetElementV4 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , removeelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V4 , forceflag : DHCP_FORCE_FLAG ) -> u32 );
    DhcpRemoveSubnetElementV4(serveripaddress.into_param().abi(), subnetaddress, removeelementinfo, forceflag)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpRemoveSubnetElementV5<P0>(serveripaddress: P0, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V5, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpRemoveSubnetElementV5 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , removeelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V5 , forceflag : DHCP_FORCE_FLAG ) -> u32 );
    DhcpRemoveSubnetElementV5(serveripaddress.into_param().abi(), subnetaddress, removeelementinfo, forceflag)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpRemoveSubnetElementV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, removeelementinfo: *mut DHCP_SUBNET_ELEMENT_DATA_V6, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpRemoveSubnetElementV6 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : DHCP_IPV6_ADDRESS , removeelementinfo : *mut DHCP_SUBNET_ELEMENT_DATA_V6 , forceflag : DHCP_FORCE_FLAG ) -> u32 );
    DhcpRemoveSubnetElementV6(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), removeelementinfo, forceflag)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpRequestParams<P0, P1>(flags: u32, reserved: *mut ::core::ffi::c_void, adaptername: P0, classid: *mut DHCPCAPI_CLASSID, sendparams: DHCPCAPI_PARAMS_ARRAY, recdparams: DHCPCAPI_PARAMS_ARRAY, buffer: *mut u8, psize: *mut u32, requestidstr: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpcsvc.dll""system" fn DhcpRequestParams ( flags : u32 , reserved : *mut ::core::ffi::c_void , adaptername : ::windows::core::PCWSTR , classid : *mut DHCPCAPI_CLASSID , sendparams : DHCPCAPI_PARAMS_ARRAY , recdparams : DHCPCAPI_PARAMS_ARRAY , buffer : *mut u8 , psize : *mut u32 , requestidstr : ::windows::core::PCWSTR ) -> u32 );
    DhcpRequestParams(flags, reserved, adaptername.into_param().abi(), classid, ::core::mem::transmute(sendparams), ::core::mem::transmute(recdparams), buffer, psize, requestidstr.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpRpcFreeMemory(bufferpointer: *mut ::core::ffi::c_void) {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpRpcFreeMemory ( bufferpointer : *mut ::core::ffi::c_void ) -> ( ) );
    DhcpRpcFreeMemory(bufferpointer)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpScanDatabase<P0>(serveripaddress: P0, subnetaddress: u32, fixflag: u32, scanlist: *mut *mut DHCP_SCAN_LIST) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpScanDatabase ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , fixflag : u32 , scanlist : *mut *mut DHCP_SCAN_LIST ) -> u32 );
    DhcpScanDatabase(serveripaddress.into_param().abi(), subnetaddress, fixflag, scanlist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerAuditlogParamsFree(configinfo: *mut DHCP_SERVER_CONFIG_INFO_VQ) {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerAuditlogParamsFree ( configinfo : *mut DHCP_SERVER_CONFIG_INFO_VQ ) -> ( ) );
    DhcpServerAuditlogParamsFree(configinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpServerBackupDatabase<P0, P1>(serveripaddress: P0, path: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerBackupDatabase ( serveripaddress : ::windows::core::PCWSTR , path : ::windows::core::PCWSTR ) -> u32 );
    DhcpServerBackupDatabase(serveripaddress.into_param().abi(), path.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpServerGetConfig<P0>(serveripaddress: P0, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerGetConfig ( serveripaddress : ::windows::core::PCWSTR , configinfo : *mut *mut DHCP_SERVER_CONFIG_INFO ) -> u32 );
    DhcpServerGetConfig(serveripaddress.into_param().abi(), configinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerGetConfigV4<P0>(serveripaddress: P0, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerGetConfigV4 ( serveripaddress : ::windows::core::PCWSTR , configinfo : *mut *mut DHCP_SERVER_CONFIG_INFO_V4 ) -> u32 );
    DhcpServerGetConfigV4(serveripaddress.into_param().abi(), configinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerGetConfigV6<P0>(serveripaddress: P0, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerGetConfigV6 ( serveripaddress : ::windows::core::PCWSTR , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6 , configinfo : *mut *mut DHCP_SERVER_CONFIG_INFO_V6 ) -> u32 );
    DhcpServerGetConfigV6(serveripaddress.into_param().abi(), scopeinfo, configinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerGetConfigVQ<P0>(serveripaddress: P0, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerGetConfigVQ ( serveripaddress : ::windows::core::PCWSTR , configinfo : *mut *mut DHCP_SERVER_CONFIG_INFO_VQ ) -> u32 );
    DhcpServerGetConfigVQ(serveripaddress.into_param().abi(), configinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerQueryAttribute<P0>(serveripaddr: P0, dwreserved: u32, dhcpattribid: u32, pdhcpattrib: *mut *mut DHCP_ATTRIB) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerQueryAttribute ( serveripaddr : ::windows::core::PCWSTR , dwreserved : u32 , dhcpattribid : u32 , pdhcpattrib : *mut *mut DHCP_ATTRIB ) -> u32 );
    DhcpServerQueryAttribute(serveripaddr.into_param().abi(), dwreserved, dhcpattribid, pdhcpattrib)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerQueryAttributes<P0>(serveripaddr: P0, dwreserved: u32, dwattribcount: u32, pdhcpattribs: *mut u32, pdhcpattribarr: *mut *mut DHCP_ATTRIB_ARRAY) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerQueryAttributes ( serveripaddr : ::windows::core::PCWSTR , dwreserved : u32 , dwattribcount : u32 , pdhcpattribs : *mut u32 , pdhcpattribarr : *mut *mut DHCP_ATTRIB_ARRAY ) -> u32 );
    DhcpServerQueryAttributes(serveripaddr.into_param().abi(), dwreserved, dwattribcount, pdhcpattribs, pdhcpattribarr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpServerQueryDnsRegCredentials<P0>(serveripaddress: P0, uname: &mut [u16], domain: &mut [u16]) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerQueryDnsRegCredentials ( serveripaddress : ::windows::core::PCWSTR , unamesize : u32 , uname : ::windows::core::PWSTR , domainsize : u32 , domain : ::windows::core::PWSTR ) -> u32 );
    DhcpServerQueryDnsRegCredentials(serveripaddress.into_param().abi(), uname.len() as _, ::core::mem::transmute(uname.as_ptr()), domain.len() as _, ::core::mem::transmute(domain.as_ptr()))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpServerRedoAuthorization<P0>(serveripaddr: P0, dwreserved: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerRedoAuthorization ( serveripaddr : ::windows::core::PCWSTR , dwreserved : u32 ) -> u32 );
    DhcpServerRedoAuthorization(serveripaddr.into_param().abi(), dwreserved)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpServerRestoreDatabase<P0, P1>(serveripaddress: P0, path: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerRestoreDatabase ( serveripaddress : ::windows::core::PCWSTR , path : ::windows::core::PCWSTR ) -> u32 );
    DhcpServerRestoreDatabase(serveripaddress.into_param().abi(), path.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpServerSetConfig<P0>(serveripaddress: P0, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerSetConfig ( serveripaddress : ::windows::core::PCWSTR , fieldstoset : u32 , configinfo : *mut DHCP_SERVER_CONFIG_INFO ) -> u32 );
    DhcpServerSetConfig(serveripaddress.into_param().abi(), fieldstoset, configinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerSetConfigV4<P0>(serveripaddress: P0, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerSetConfigV4 ( serveripaddress : ::windows::core::PCWSTR , fieldstoset : u32 , configinfo : *mut DHCP_SERVER_CONFIG_INFO_V4 ) -> u32 );
    DhcpServerSetConfigV4(serveripaddress.into_param().abi(), fieldstoset, configinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerSetConfigV6<P0>(serveripaddress: P0, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerSetConfigV6 ( serveripaddress : ::windows::core::PCWSTR , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6 , fieldstoset : u32 , configinfo : *mut DHCP_SERVER_CONFIG_INFO_V6 ) -> u32 );
    DhcpServerSetConfigV6(serveripaddress.into_param().abi(), scopeinfo, fieldstoset, configinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpServerSetConfigVQ<P0>(serveripaddress: P0, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerSetConfigVQ ( serveripaddress : ::windows::core::PCWSTR , fieldstoset : u32 , configinfo : *mut DHCP_SERVER_CONFIG_INFO_VQ ) -> u32 );
    DhcpServerSetConfigVQ(serveripaddress.into_param().abi(), fieldstoset, configinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpServerSetDnsRegCredentials<P0, P1, P2, P3>(serveripaddress: P0, uname: P1, domain: P2, passwd: P3) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerSetDnsRegCredentials ( serveripaddress : ::windows::core::PCWSTR , uname : ::windows::core::PCWSTR , domain : ::windows::core::PCWSTR , passwd : ::windows::core::PCWSTR ) -> u32 );
    DhcpServerSetDnsRegCredentials(serveripaddress.into_param().abi(), uname.into_param().abi(), domain.into_param().abi(), passwd.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpServerSetDnsRegCredentialsV5<P0, P1, P2, P3>(serveripaddress: P0, uname: P1, domain: P2, passwd: P3) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpServerSetDnsRegCredentialsV5 ( serveripaddress : ::windows::core::PCWSTR , uname : ::windows::core::PCWSTR , domain : ::windows::core::PCWSTR , passwd : ::windows::core::PCWSTR ) -> u32 );
    DhcpServerSetDnsRegCredentialsV5(serveripaddress.into_param().abi(), uname.into_param().abi(), domain.into_param().abi(), passwd.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpSetClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetClientInfo ( serveripaddress : ::windows::core::PCWSTR , clientinfo : *const DHCP_CLIENT_INFO ) -> u32 );
    DhcpSetClientInfo(serveripaddress.into_param().abi(), clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpSetClientInfoV4<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_V4) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetClientInfoV4 ( serveripaddress : ::windows::core::PCWSTR , clientinfo : *const DHCP_CLIENT_INFO_V4 ) -> u32 );
    DhcpSetClientInfoV4(serveripaddress.into_param().abi(), clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpSetClientInfoV6<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_V6) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetClientInfoV6 ( serveripaddress : ::windows::core::PCWSTR , clientinfo : *const DHCP_CLIENT_INFO_V6 ) -> u32 );
    DhcpSetClientInfoV6(serveripaddress.into_param().abi(), clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetClientInfoVQ<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_VQ) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetClientInfoVQ ( serveripaddress : ::windows::core::PCWSTR , clientinfo : *const DHCP_CLIENT_INFO_VQ ) -> u32 );
    DhcpSetClientInfoVQ(serveripaddress.into_param().abi(), clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetFilterV4<P0>(serveripaddress: P0, globalfilterinfo: *const DHCP_FILTER_GLOBAL_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetFilterV4 ( serveripaddress : ::windows::core::PCWSTR , globalfilterinfo : *const DHCP_FILTER_GLOBAL_INFO ) -> u32 );
    DhcpSetFilterV4(serveripaddress.into_param().abi(), globalfilterinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpSetOptionInfo<P0>(serveripaddress: P0, optionid: u32, optioninfo: *const DHCP_OPTION) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetOptionInfo ( serveripaddress : ::windows::core::PCWSTR , optionid : u32 , optioninfo : *const DHCP_OPTION ) -> u32 );
    DhcpSetOptionInfo(serveripaddress.into_param().abi(), optionid, optioninfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpSetOptionInfoV5<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, optioninfo: *mut DHCP_OPTION) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetOptionInfoV5 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , optioninfo : *mut DHCP_OPTION ) -> u32 );
    DhcpSetOptionInfoV5(serveripaddress.into_param().abi(), flags, optionid, classname.into_param().abi(), vendorname.into_param().abi(), optioninfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpSetOptionInfoV6<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, optioninfo: *mut DHCP_OPTION) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetOptionInfoV6 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , optioninfo : *mut DHCP_OPTION ) -> u32 );
    DhcpSetOptionInfoV6(serveripaddress.into_param().abi(), flags, optionid, classname.into_param().abi(), vendorname.into_param().abi(), optioninfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpSetOptionValue<P0>(serveripaddress: P0, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalue: *const DHCP_OPTION_DATA) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetOptionValue ( serveripaddress : ::windows::core::PCWSTR , optionid : u32 , scopeinfo : *const DHCP_OPTION_SCOPE_INFO , optionvalue : *const DHCP_OPTION_DATA ) -> u32 );
    DhcpSetOptionValue(serveripaddress.into_param().abi(), optionid, scopeinfo, optionvalue)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpSetOptionValueV5<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut DHCP_OPTION_DATA) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetOptionValueV5 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO , optionvalue : *mut DHCP_OPTION_DATA ) -> u32 );
    DhcpSetOptionValueV5(serveripaddress.into_param().abi(), flags, optionid, classname.into_param().abi(), vendorname.into_param().abi(), scopeinfo, optionvalue)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpSetOptionValueV6<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, optionvalue: *mut DHCP_OPTION_DATA) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetOptionValueV6 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6 , optionvalue : *mut DHCP_OPTION_DATA ) -> u32 );
    DhcpSetOptionValueV6(serveripaddress.into_param().abi(), flags, optionid, classname.into_param().abi(), vendorname.into_param().abi(), scopeinfo, optionvalue)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpSetOptionValues<P0>(serveripaddress: P0, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalues: *const DHCP_OPTION_VALUE_ARRAY) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetOptionValues ( serveripaddress : ::windows::core::PCWSTR , scopeinfo : *const DHCP_OPTION_SCOPE_INFO , optionvalues : *const DHCP_OPTION_VALUE_ARRAY ) -> u32 );
    DhcpSetOptionValues(serveripaddress.into_param().abi(), scopeinfo, optionvalues)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpSetOptionValuesV5<P0, P1, P2>(serveripaddress: P0, flags: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalues: *mut DHCP_OPTION_VALUE_ARRAY) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetOptionValuesV5 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , classname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO , optionvalues : *mut DHCP_OPTION_VALUE_ARRAY ) -> u32 );
    DhcpSetOptionValuesV5(serveripaddress.into_param().abi(), flags, classname.into_param().abi(), vendorname.into_param().abi(), scopeinfo, optionvalues)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetServerBindingInfo<P0>(serveripaddress: P0, flags: u32, bindelementinfo: *mut DHCP_BIND_ELEMENT_ARRAY) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetServerBindingInfo ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , bindelementinfo : *mut DHCP_BIND_ELEMENT_ARRAY ) -> u32 );
    DhcpSetServerBindingInfo(serveripaddress.into_param().abi(), flags, bindelementinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetServerBindingInfoV6<P0>(serveripaddress: P0, flags: u32, bindelementinfo: *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetServerBindingInfoV6 ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , bindelementinfo : *mut DHCPV6_BIND_ELEMENT_ARRAY ) -> u32 );
    DhcpSetServerBindingInfoV6(serveripaddress.into_param().abi(), flags, bindelementinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpSetSubnetDelayOffer<P0>(serveripaddress: P0, subnetaddress: u32, timedelayinmilliseconds: u16) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetSubnetDelayOffer ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , timedelayinmilliseconds : u16 ) -> u32 );
    DhcpSetSubnetDelayOffer(serveripaddress.into_param().abi(), subnetaddress, timedelayinmilliseconds)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpSetSubnetInfo<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetSubnetInfo ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , subnetinfo : *const DHCP_SUBNET_INFO ) -> u32 );
    DhcpSetSubnetInfo(serveripaddress.into_param().abi(), subnetaddress, subnetinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpSetSubnetInfoV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut DHCP_SUBNET_INFO_V6) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetSubnetInfoV6 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : DHCP_IPV6_ADDRESS , subnetinfo : *mut DHCP_SUBNET_INFO_V6 ) -> u32 );
    DhcpSetSubnetInfoV6(serveripaddress.into_param().abi(), ::core::mem::transmute(subnetaddress), subnetinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpSetSubnetInfoVQ<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO_VQ) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetSubnetInfoVQ ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , subnetinfo : *const DHCP_SUBNET_INFO_VQ ) -> u32 );
    DhcpSetSubnetInfoVQ(serveripaddress.into_param().abi(), subnetaddress, subnetinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpSetSuperScopeV4<P0, P1, P2>(serveripaddress: P0, subnetaddress: u32, superscopename: P1, changeexisting: P2) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetSuperScopeV4 ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , superscopename : ::windows::core::PCWSTR , changeexisting : super::super::Foundation:: BOOL ) -> u32 );
    DhcpSetSuperScopeV4(serveripaddress.into_param().abi(), subnetaddress, superscopename.into_param().abi(), changeexisting.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpSetThreadOptions(flags: u32, reserved: *mut ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpSetThreadOptions ( flags : u32 , reserved : *mut ::core::ffi::c_void ) -> u32 );
    DhcpSetThreadOptions(flags, reserved)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpUndoRequestParams<P0, P1>(flags: u32, reserved: ::core::option::Option<*const ::core::ffi::c_void>, adaptername: P0, requestidstr: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpcsvc.dll""system" fn DhcpUndoRequestParams ( flags : u32 , reserved : *const ::core::ffi::c_void , adaptername : ::windows::core::PCWSTR , requestidstr : ::windows::core::PCWSTR ) -> u32 );
    DhcpUndoRequestParams(flags, ::core::mem::transmute(reserved.unwrap_or(::std::ptr::null())), adaptername.into_param().abi(), requestidstr.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4AddPolicyRange<P0, P1>(serveripaddress: P0, subnetaddress: u32, policyname: P1, range: *const DHCP_IP_RANGE) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4AddPolicyRange ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , policyname : ::windows::core::PCWSTR , range : *const DHCP_IP_RANGE ) -> u32 );
    DhcpV4AddPolicyRange(serveripaddress.into_param().abi(), subnetaddress, policyname.into_param().abi(), range)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4CreateClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_PB) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4CreateClientInfo ( serveripaddress : ::windows::core::PCWSTR , clientinfo : *const DHCP_CLIENT_INFO_PB ) -> u32 );
    DhcpV4CreateClientInfo(serveripaddress.into_param().abi(), clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4CreateClientInfoEx<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_EX) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4CreateClientInfoEx ( serveripaddress : ::windows::core::PCWSTR , clientinfo : *const DHCP_CLIENT_INFO_EX ) -> u32 );
    DhcpV4CreateClientInfoEx(serveripaddress.into_param().abi(), clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4CreatePolicy<P0>(serveripaddress: P0, ppolicy: *const DHCP_POLICY) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4CreatePolicy ( serveripaddress : ::windows::core::PCWSTR , ppolicy : *const DHCP_POLICY ) -> u32 );
    DhcpV4CreatePolicy(serveripaddress.into_param().abi(), ppolicy)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4CreatePolicyEx<P0>(serveripaddress: P0, policyex: *const DHCP_POLICY_EX) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4CreatePolicyEx ( serveripaddress : ::windows::core::PCWSTR , policyex : *const DHCP_POLICY_EX ) -> u32 );
    DhcpV4CreatePolicyEx(serveripaddress.into_param().abi(), policyex)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4DeletePolicy<P0, P1, P2>(serveripaddress: P0, fglobalpolicy: P1, subnetaddress: u32, policyname: P2) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4DeletePolicy ( serveripaddress : ::windows::core::PCWSTR , fglobalpolicy : super::super::Foundation:: BOOL , subnetaddress : u32 , policyname : ::windows::core::PCWSTR ) -> u32 );
    DhcpV4DeletePolicy(serveripaddress.into_param().abi(), fglobalpolicy.into_param().abi(), subnetaddress, policyname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4EnumPolicies<P0, P1>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, fglobalpolicy: P1, subnetaddress: u32, enuminfo: *mut *mut DHCP_POLICY_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4EnumPolicies ( serveripaddress : ::windows::core::PCWSTR , resumehandle : *mut u32 , preferredmaximum : u32 , fglobalpolicy : super::super::Foundation:: BOOL , subnetaddress : u32 , enuminfo : *mut *mut DHCP_POLICY_ARRAY , elementsread : *mut u32 , elementstotal : *mut u32 ) -> u32 );
    DhcpV4EnumPolicies(serveripaddress.into_param().abi(), resumehandle, preferredmaximum, fglobalpolicy.into_param().abi(), subnetaddress, enuminfo, elementsread, elementstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4EnumPoliciesEx<P0, P1>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, globalpolicy: P1, subnetaddress: u32, enuminfo: *mut *mut DHCP_POLICY_EX_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4EnumPoliciesEx ( serveripaddress : ::windows::core::PCWSTR , resumehandle : *mut u32 , preferredmaximum : u32 , globalpolicy : super::super::Foundation:: BOOL , subnetaddress : u32 , enuminfo : *mut *mut DHCP_POLICY_EX_ARRAY , elementsread : *mut u32 , elementstotal : *mut u32 ) -> u32 );
    DhcpV4EnumPoliciesEx(serveripaddress.into_param().abi(), resumehandle, preferredmaximum, globalpolicy.into_param().abi(), subnetaddress, enuminfo, elementsread, elementstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4EnumSubnetClients<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_PB_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4EnumSubnetClients ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , resumehandle : *mut u32 , preferredmaximum : u32 , clientinfo : *mut *mut DHCP_CLIENT_INFO_PB_ARRAY , clientsread : *mut u32 , clientstotal : *mut u32 ) -> u32 );
    DhcpV4EnumSubnetClients(serveripaddress.into_param().abi(), subnetaddress, resumehandle, preferredmaximum, clientinfo, clientsread, clientstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4EnumSubnetClientsEx<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_EX_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4EnumSubnetClientsEx ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , resumehandle : *mut u32 , preferredmaximum : u32 , clientinfo : *mut *mut DHCP_CLIENT_INFO_EX_ARRAY , clientsread : *mut u32 , clientstotal : *mut u32 ) -> u32 );
    DhcpV4EnumSubnetClientsEx(serveripaddress.into_param().abi(), subnetaddress, resumehandle, preferredmaximum, clientinfo, clientsread, clientstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4EnumSubnetReservations<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_RESERVATION_INFO_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4EnumSubnetReservations ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , resumehandle : *mut u32 , preferredmaximum : u32 , enumelementinfo : *mut *mut DHCP_RESERVATION_INFO_ARRAY , elementsread : *mut u32 , elementstotal : *mut u32 ) -> u32 );
    DhcpV4EnumSubnetReservations(serveripaddress.into_param().abi(), subnetaddress, resumehandle, preferredmaximum, enumelementinfo, elementsread, elementstotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4FailoverAddScopeToRelationship<P0>(serveripaddress: P0, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4FailoverAddScopeToRelationship ( serveripaddress : ::windows::core::PCWSTR , prelationship : *const DHCP_FAILOVER_RELATIONSHIP ) -> u32 );
    DhcpV4FailoverAddScopeToRelationship(serveripaddress.into_param().abi(), prelationship)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4FailoverCreateRelationship<P0>(serveripaddress: P0, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4FailoverCreateRelationship ( serveripaddress : ::windows::core::PCWSTR , prelationship : *const DHCP_FAILOVER_RELATIONSHIP ) -> u32 );
    DhcpV4FailoverCreateRelationship(serveripaddress.into_param().abi(), prelationship)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4FailoverDeleteRelationship<P0, P1>(serveripaddress: P0, prelationshipname: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4FailoverDeleteRelationship ( serveripaddress : ::windows::core::PCWSTR , prelationshipname : ::windows::core::PCWSTR ) -> u32 );
    DhcpV4FailoverDeleteRelationship(serveripaddress.into_param().abi(), prelationshipname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4FailoverDeleteScopeFromRelationship<P0>(serveripaddress: P0, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4FailoverDeleteScopeFromRelationship ( serveripaddress : ::windows::core::PCWSTR , prelationship : *const DHCP_FAILOVER_RELATIONSHIP ) -> u32 );
    DhcpV4FailoverDeleteScopeFromRelationship(serveripaddress.into_param().abi(), prelationship)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4FailoverEnumRelationship<P0>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP_ARRAY, relationshipread: *mut u32, relationshiptotal: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4FailoverEnumRelationship ( serveripaddress : ::windows::core::PCWSTR , resumehandle : *mut u32 , preferredmaximum : u32 , prelationship : *mut *mut DHCP_FAILOVER_RELATIONSHIP_ARRAY , relationshipread : *mut u32 , relationshiptotal : *mut u32 ) -> u32 );
    DhcpV4FailoverEnumRelationship(serveripaddress.into_param().abi(), resumehandle, preferredmaximum, prelationship, relationshipread, relationshiptotal)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4FailoverGetAddressStatus<P0>(serveripaddress: P0, subnetaddress: u32, pstatus: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4FailoverGetAddressStatus ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , pstatus : *mut u32 ) -> u32 );
    DhcpV4FailoverGetAddressStatus(serveripaddress.into_param().abi(), subnetaddress, pstatus)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4FailoverGetClientInfo<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCPV4_FAILOVER_CLIENT_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4FailoverGetClientInfo ( serveripaddress : ::windows::core::PCWSTR , searchinfo : *const DHCP_SEARCH_INFO , clientinfo : *mut *mut DHCPV4_FAILOVER_CLIENT_INFO ) -> u32 );
    DhcpV4FailoverGetClientInfo(serveripaddress.into_param().abi(), searchinfo, clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4FailoverGetRelationship<P0, P1>(serveripaddress: P0, prelationshipname: P1, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4FailoverGetRelationship ( serveripaddress : ::windows::core::PCWSTR , prelationshipname : ::windows::core::PCWSTR , prelationship : *mut *mut DHCP_FAILOVER_RELATIONSHIP ) -> u32 );
    DhcpV4FailoverGetRelationship(serveripaddress.into_param().abi(), prelationshipname.into_param().abi(), prelationship)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4FailoverGetScopeRelationship<P0>(serveripaddress: P0, scopeid: u32, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4FailoverGetScopeRelationship ( serveripaddress : ::windows::core::PCWSTR , scopeid : u32 , prelationship : *mut *mut DHCP_FAILOVER_RELATIONSHIP ) -> u32 );
    DhcpV4FailoverGetScopeRelationship(serveripaddress.into_param().abi(), scopeid, prelationship)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4FailoverGetScopeStatistics<P0>(serveripaddress: P0, scopeid: u32, pstats: *mut *mut DHCP_FAILOVER_STATISTICS) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4FailoverGetScopeStatistics ( serveripaddress : ::windows::core::PCWSTR , scopeid : u32 , pstats : *mut *mut DHCP_FAILOVER_STATISTICS ) -> u32 );
    DhcpV4FailoverGetScopeStatistics(serveripaddress.into_param().abi(), scopeid, pstats)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4FailoverGetSystemTime<P0>(serveripaddress: P0, ptime: *mut u32, pmaxalloweddeltatime: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4FailoverGetSystemTime ( serveripaddress : ::windows::core::PCWSTR , ptime : *mut u32 , pmaxalloweddeltatime : *mut u32 ) -> u32 );
    DhcpV4FailoverGetSystemTime(serveripaddress.into_param().abi(), ptime, pmaxalloweddeltatime)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4FailoverSetRelationship<P0>(serveripaddress: P0, flags: u32, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4FailoverSetRelationship ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , prelationship : *const DHCP_FAILOVER_RELATIONSHIP ) -> u32 );
    DhcpV4FailoverSetRelationship(serveripaddress.into_param().abi(), flags, prelationship)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4FailoverTriggerAddrAllocation<P0, P1>(serveripaddress: P0, pfailrelname: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4FailoverTriggerAddrAllocation ( serveripaddress : ::windows::core::PCWSTR , pfailrelname : ::windows::core::PCWSTR ) -> u32 );
    DhcpV4FailoverTriggerAddrAllocation(serveripaddress.into_param().abi(), pfailrelname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4GetAllOptionValues<P0>(serveripaddress: P0, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, values: *mut *mut DHCP_ALL_OPTION_VALUES_PB) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4GetAllOptionValues ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO , values : *mut *mut DHCP_ALL_OPTION_VALUES_PB ) -> u32 );
    DhcpV4GetAllOptionValues(serveripaddress.into_param().abi(), flags, scopeinfo, values)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4GetClientInfo<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_PB) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4GetClientInfo ( serveripaddress : ::windows::core::PCWSTR , searchinfo : *const DHCP_SEARCH_INFO , clientinfo : *mut *mut DHCP_CLIENT_INFO_PB ) -> u32 );
    DhcpV4GetClientInfo(serveripaddress.into_param().abi(), searchinfo, clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4GetClientInfoEx<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_EX) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4GetClientInfoEx ( serveripaddress : ::windows::core::PCWSTR , searchinfo : *const DHCP_SEARCH_INFO , clientinfo : *mut *mut DHCP_CLIENT_INFO_EX ) -> u32 );
    DhcpV4GetClientInfoEx(serveripaddress.into_param().abi(), searchinfo, clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4GetFreeIPAddress<P0>(serveripaddress: P0, scopeid: u32, startip: u32, endip: u32, numfreeaddrreq: u32, ipaddrlist: *mut *mut DHCP_IP_ARRAY) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4GetFreeIPAddress ( serveripaddress : ::windows::core::PCWSTR , scopeid : u32 , startip : u32 , endip : u32 , numfreeaddrreq : u32 , ipaddrlist : *mut *mut DHCP_IP_ARRAY ) -> u32 );
    DhcpV4GetFreeIPAddress(serveripaddress.into_param().abi(), scopeid, startip, endip, numfreeaddrreq, ipaddrlist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4GetOptionValue<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, policyname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4GetOptionValue ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , policyname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO , optionvalue : *mut *mut DHCP_OPTION_VALUE ) -> u32 );
    DhcpV4GetOptionValue(serveripaddress.into_param().abi(), flags, optionid, policyname.into_param().abi(), vendorname.into_param().abi(), scopeinfo, optionvalue)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4GetPolicy<P0, P1, P2>(serveripaddress: P0, fglobalpolicy: P1, subnetaddress: u32, policyname: P2, policy: *mut *mut DHCP_POLICY) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4GetPolicy ( serveripaddress : ::windows::core::PCWSTR , fglobalpolicy : super::super::Foundation:: BOOL , subnetaddress : u32 , policyname : ::windows::core::PCWSTR , policy : *mut *mut DHCP_POLICY ) -> u32 );
    DhcpV4GetPolicy(serveripaddress.into_param().abi(), fglobalpolicy.into_param().abi(), subnetaddress, policyname.into_param().abi(), policy)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4GetPolicyEx<P0, P1, P2>(serveripaddress: P0, globalpolicy: P1, subnetaddress: u32, policyname: P2, policy: *mut *mut DHCP_POLICY_EX) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4GetPolicyEx ( serveripaddress : ::windows::core::PCWSTR , globalpolicy : super::super::Foundation:: BOOL , subnetaddress : u32 , policyname : ::windows::core::PCWSTR , policy : *mut *mut DHCP_POLICY_EX ) -> u32 );
    DhcpV4GetPolicyEx(serveripaddress.into_param().abi(), globalpolicy.into_param().abi(), subnetaddress, policyname.into_param().abi(), policy)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4QueryPolicyEnforcement<P0, P1>(serveripaddress: P0, fglobalpolicy: P1, subnetaddress: u32, enabled: *mut super::super::Foundation::BOOL) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4QueryPolicyEnforcement ( serveripaddress : ::windows::core::PCWSTR , fglobalpolicy : super::super::Foundation:: BOOL , subnetaddress : u32 , enabled : *mut super::super::Foundation:: BOOL ) -> u32 );
    DhcpV4QueryPolicyEnforcement(serveripaddress.into_param().abi(), fglobalpolicy.into_param().abi(), subnetaddress, enabled)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4RemoveOptionValue<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, policyname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4RemoveOptionValue ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , policyname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO ) -> u32 );
    DhcpV4RemoveOptionValue(serveripaddress.into_param().abi(), flags, optionid, policyname.into_param().abi(), vendorname.into_param().abi(), scopeinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4RemovePolicyRange<P0, P1>(serveripaddress: P0, subnetaddress: u32, policyname: P1, range: *const DHCP_IP_RANGE) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4RemovePolicyRange ( serveripaddress : ::windows::core::PCWSTR , subnetaddress : u32 , policyname : ::windows::core::PCWSTR , range : *const DHCP_IP_RANGE ) -> u32 );
    DhcpV4RemovePolicyRange(serveripaddress.into_param().abi(), subnetaddress, policyname.into_param().abi(), range)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4SetOptionValue<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, policyname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut DHCP_OPTION_DATA) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4SetOptionValue ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , optionid : u32 , policyname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO , optionvalue : *mut DHCP_OPTION_DATA ) -> u32 );
    DhcpV4SetOptionValue(serveripaddress.into_param().abi(), flags, optionid, policyname.into_param().abi(), vendorname.into_param().abi(), scopeinfo, optionvalue)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV4SetOptionValues<P0, P1, P2>(serveripaddress: P0, flags: u32, policyname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalues: *mut DHCP_OPTION_VALUE_ARRAY) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4SetOptionValues ( serveripaddress : ::windows::core::PCWSTR , flags : u32 , policyname : ::windows::core::PCWSTR , vendorname : ::windows::core::PCWSTR , scopeinfo : *mut DHCP_OPTION_SCOPE_INFO , optionvalues : *mut DHCP_OPTION_VALUE_ARRAY ) -> u32 );
    DhcpV4SetOptionValues(serveripaddress.into_param().abi(), flags, policyname.into_param().abi(), vendorname.into_param().abi(), scopeinfo, optionvalues)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4SetPolicy<P0, P1, P2>(serveripaddress: P0, fieldsmodified: u32, fglobalpolicy: P1, subnetaddress: u32, policyname: P2, policy: *const DHCP_POLICY) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4SetPolicy ( serveripaddress : ::windows::core::PCWSTR , fieldsmodified : u32 , fglobalpolicy : super::super::Foundation:: BOOL , subnetaddress : u32 , policyname : ::windows::core::PCWSTR , policy : *const DHCP_POLICY ) -> u32 );
    DhcpV4SetPolicy(serveripaddress.into_param().abi(), fieldsmodified, fglobalpolicy.into_param().abi(), subnetaddress, policyname.into_param().abi(), policy)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4SetPolicyEnforcement<P0, P1, P2>(serveripaddress: P0, fglobalpolicy: P1, subnetaddress: u32, enable: P2) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4SetPolicyEnforcement ( serveripaddress : ::windows::core::PCWSTR , fglobalpolicy : super::super::Foundation:: BOOL , subnetaddress : u32 , enable : super::super::Foundation:: BOOL ) -> u32 );
    DhcpV4SetPolicyEnforcement(serveripaddress.into_param().abi(), fglobalpolicy.into_param().abi(), subnetaddress, enable.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV4SetPolicyEx<P0, P1, P2>(serveripaddress: P0, fieldsmodified: u32, globalpolicy: P1, subnetaddress: u32, policyname: P2, policy: *const DHCP_POLICY_EX) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV4SetPolicyEx ( serveripaddress : ::windows::core::PCWSTR , fieldsmodified : u32 , globalpolicy : super::super::Foundation:: BOOL , subnetaddress : u32 , policyname : ::windows::core::PCWSTR , policy : *const DHCP_POLICY_EX ) -> u32 );
    DhcpV4SetPolicyEx(serveripaddress.into_param().abi(), fieldsmodified, globalpolicy.into_param().abi(), subnetaddress, policyname.into_param().abi(), policy)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV6CreateClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_V6) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV6CreateClientInfo ( serveripaddress : ::windows::core::PCWSTR , clientinfo : *const DHCP_CLIENT_INFO_V6 ) -> u32 );
    DhcpV6CreateClientInfo(serveripaddress.into_param().abi(), clientinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV6GetFreeIPAddress<P0>(serveripaddress: P0, scopeid: DHCP_IPV6_ADDRESS, startip: DHCP_IPV6_ADDRESS, endip: DHCP_IPV6_ADDRESS, numfreeaddrreq: u32, ipaddrlist: *mut *mut DHCPV6_IP_ARRAY) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV6GetFreeIPAddress ( serveripaddress : ::windows::core::PCWSTR , scopeid : DHCP_IPV6_ADDRESS , startip : DHCP_IPV6_ADDRESS , endip : DHCP_IPV6_ADDRESS , numfreeaddrreq : u32 , ipaddrlist : *mut *mut DHCPV6_IP_ARRAY ) -> u32 );
    DhcpV6GetFreeIPAddress(serveripaddress.into_param().abi(), ::core::mem::transmute(scopeid), ::core::mem::transmute(startip), ::core::mem::transmute(endip), numfreeaddrreq, ipaddrlist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn DhcpV6GetStatelessStatistics<P0>(serveripaddress: P0, statelessstats: *mut *mut DHCPV6_STATELESS_STATS) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV6GetStatelessStatistics ( serveripaddress : ::windows::core::PCWSTR , statelessstats : *mut *mut DHCPV6_STATELESS_STATS ) -> u32 );
    DhcpV6GetStatelessStatistics(serveripaddress.into_param().abi(), statelessstats)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV6GetStatelessStoreParams<P0, P1>(serveripaddress: P0, fserverlevel: P1, subnetaddress: DHCP_IPV6_ADDRESS, params: *mut *mut DHCPV6_STATELESS_PARAMS) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV6GetStatelessStoreParams ( serveripaddress : ::windows::core::PCWSTR , fserverlevel : super::super::Foundation:: BOOL , subnetaddress : DHCP_IPV6_ADDRESS , params : *mut *mut DHCPV6_STATELESS_PARAMS ) -> u32 );
    DhcpV6GetStatelessStoreParams(serveripaddress.into_param().abi(), fserverlevel.into_param().abi(), ::core::mem::transmute(subnetaddress), params)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DhcpV6SetStatelessStoreParams<P0, P1>(serveripaddress: P0, fserverlevel: P1, subnetaddress: DHCP_IPV6_ADDRESS, fieldmodified: u32, params: *const DHCPV6_STATELESS_PARAMS) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "dhcpsapi.dll""system" fn DhcpV6SetStatelessStoreParams ( serveripaddress : ::windows::core::PCWSTR , fserverlevel : super::super::Foundation:: BOOL , subnetaddress : DHCP_IPV6_ADDRESS , fieldmodified : u32 , params : *const DHCPV6_STATELESS_PARAMS ) -> u32 );
    DhcpV6SetStatelessStoreParams(serveripaddress.into_param().abi(), fserverlevel.into_param().abi(), ::core::mem::transmute(subnetaddress), fieldmodified, params)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn Dhcpv6CApiCleanup() {
    ::windows_targets::link ! ( "dhcpcsvc6.dll""system" fn Dhcpv6CApiCleanup ( ) -> ( ) );
    Dhcpv6CApiCleanup()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn Dhcpv6CApiInitialize(version: *mut u32) {
    ::windows_targets::link ! ( "dhcpcsvc6.dll""system" fn Dhcpv6CApiInitialize ( version : *mut u32 ) -> ( ) );
    Dhcpv6CApiInitialize(version)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn Dhcpv6ReleasePrefix<P0>(adaptername: P0, classid: *mut DHCPV6CAPI_CLASSID, leaseinfo: *mut DHCPV6PrefixLeaseInformation) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpcsvc6.dll""system" fn Dhcpv6ReleasePrefix ( adaptername : ::windows::core::PCWSTR , classid : *mut DHCPV6CAPI_CLASSID , leaseinfo : *mut DHCPV6PrefixLeaseInformation ) -> u32 );
    Dhcpv6ReleasePrefix(adaptername.into_param().abi(), classid, leaseinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn Dhcpv6RenewPrefix<P0>(adaptername: P0, pclassid: *mut DHCPV6CAPI_CLASSID, prefixleaseinfo: *mut DHCPV6PrefixLeaseInformation, pdwtimetowait: *mut u32, bvalidateprefix: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpcsvc6.dll""system" fn Dhcpv6RenewPrefix ( adaptername : ::windows::core::PCWSTR , pclassid : *mut DHCPV6CAPI_CLASSID , prefixleaseinfo : *mut DHCPV6PrefixLeaseInformation , pdwtimetowait : *mut u32 , bvalidateprefix : u32 ) -> u32 );
    Dhcpv6RenewPrefix(adaptername.into_param().abi(), pclassid, prefixleaseinfo, pdwtimetowait, bvalidateprefix)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Dhcpv6RequestParams<P0, P1>(forcenewinform: P0, reserved: *mut ::core::ffi::c_void, adaptername: P1, classid: *mut DHCPV6CAPI_CLASSID, recdparams: DHCPV6CAPI_PARAMS_ARRAY, buffer: *mut u8, psize: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpcsvc6.dll""system" fn Dhcpv6RequestParams ( forcenewinform : super::super::Foundation:: BOOL , reserved : *mut ::core::ffi::c_void , adaptername : ::windows::core::PCWSTR , classid : *mut DHCPV6CAPI_CLASSID , recdparams : DHCPV6CAPI_PARAMS_ARRAY , buffer : *mut u8 , psize : *mut u32 ) -> u32 );
    Dhcpv6RequestParams(forcenewinform.into_param().abi(), reserved, adaptername.into_param().abi(), classid, ::core::mem::transmute(recdparams), buffer, psize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[inline]
pub unsafe fn Dhcpv6RequestPrefix<P0>(adaptername: P0, pclassid: *mut DHCPV6CAPI_CLASSID, prefixleaseinfo: *mut DHCPV6PrefixLeaseInformation, pdwtimetowait: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dhcpcsvc6.dll""system" fn Dhcpv6RequestPrefix ( adaptername : ::windows::core::PCWSTR , pclassid : *mut DHCPV6CAPI_CLASSID , prefixleaseinfo : *mut DHCPV6PrefixLeaseInformation , pdwtimetowait : *mut u32 ) -> u32 );
    Dhcpv6RequestPrefix(adaptername.into_param().abi(), pclassid, prefixleaseinfo, pdwtimetowait)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ADDRESS_TYPE_IANA: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ADDRESS_TYPE_IATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const CHANGESTATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const CLIENT_TYPE_BOOTP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const CLIENT_TYPE_DHCP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const CLIENT_TYPE_NONE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const CLIENT_TYPE_RESERVATION_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const CLIENT_TYPE_UNSPECIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPCAPI_DEREGISTER_HANDLE_EVENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPCAPI_REGISTER_HANDLE_EVENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPCAPI_REQUEST_ASYNCHRONOUS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPCAPI_REQUEST_CANCEL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPCAPI_REQUEST_MASK: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPCAPI_REQUEST_PERSISTENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPCAPI_REQUEST_SYNCHRONOUS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_CLIENTID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_DNS_SERVERS: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_DOMAIN_LIST: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_IA_NA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_IA_PD: u32 = 25u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_IA_TA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_NISP_DOMAIN_NAME: u32 = 30u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_NISP_SERVERS: u32 = 28u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_NIS_DOMAIN_NAME: u32 = 29u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_NIS_SERVERS: u32 = 27u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_ORO: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_PREFERENCE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_RAPID_COMMIT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_RECONF_MSG: u32 = 19u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_SERVERID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_SIP_SERVERS_ADDRS: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_SIP_SERVERS_NAMES: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_UNICAST: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_USER_CLASS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_VENDOR_CLASS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCPV6_OPTION_VENDOR_OPTS: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_ATTRIB_BOOL_IS_ADMIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_ATTRIB_BOOL_IS_BINDING_AWARE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_ATTRIB_BOOL_IS_DYNBOOTP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_ATTRIB_BOOL_IS_PART_OF_DSDC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_ATTRIB_BOOL_IS_ROGUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_ATTRIB_TYPE_BOOL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_ATTRIB_TYPE_ULONG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_ATTRIB_ULONG_RESTORE_STATUS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_CALLOUT_ENTRY_POINT: ::windows::core::PCSTR = ::windows::core::s!("DhcpServerCalloutEntry");
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_CALLOUT_LIST_KEY: ::windows::core::PCWSTR = ::windows::core::w!("System\\CurrentControlSet\\Services\\DHCPServer\\Parameters");
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_CALLOUT_LIST_VALUE: ::windows::core::PCWSTR = ::windows::core::w!("CalloutDlls");
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_CLIENT_BOOTP: u32 = 805306371u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_CLIENT_DHCP: u32 = 805306372u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_CONTROL_CONTINUE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_CONTROL_PAUSE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_CONTROL_START: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_CONTROL_STOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_DROP_DUPLICATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_DROP_GEN_FAILURE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_DROP_INTERNAL_ERROR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_DROP_INVALID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_DROP_NOADDRESS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_DROP_NOMEM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_DROP_NO_SUBNETS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_DROP_PAUSED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_DROP_PROCESSED: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_DROP_TIMEOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_DROP_UNAUTH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_DROP_WRONG_SERVER: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_ENDPOINT_FLAG_CANT_MODIFY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_FAILOVER_DELETE_SCOPES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_FAILOVER_MAX_NUM_ADD_SCOPES: u32 = 400u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_FAILOVER_MAX_NUM_REL: u32 = 31u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_FLAGS_DONT_ACCESS_DS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_FLAGS_DONT_DO_RPC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_FLAGS_OPTION_IS_VENDOR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_GIVE_ADDRESS_NEW: u32 = 805306369u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_GIVE_ADDRESS_OLD: u32 = 805306370u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_MAX_DELAY: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_MIN_DELAY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_OPT_ENUM_IGNORE_VENDOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_OPT_ENUM_USE_CLASSNAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_PROB_CONFLICT: u32 = 536870913u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_PROB_DECLINE: u32 = 536870914u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_PROB_NACKED: u32 = 536870916u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_PROB_RELEASE: u32 = 536870915u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_SEND_PACKET: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DHCP_SUBNET_INFO_VQ_FLAG_QUARANTINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DNS_FLAG_CLEANUP_EXPIRED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DNS_FLAG_DISABLE_PTR_UPDATE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DNS_FLAG_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DNS_FLAG_HAS_DNS_SUFFIX: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DNS_FLAG_UPDATE_BOTH_ALWAYS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DNS_FLAG_UPDATE_DHCID: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DNS_FLAG_UPDATE_DOWNLEVEL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_CLASS_DOES_NOT_EXIST: u32 = 20078u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_CLASS_EXISTS: u32 = 20077u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_DHCP_SERVER_NOT_FOUND: u32 = 20074u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_NO_DHCP_ROOT: u32 = 20071u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_NO_DS_AVAILABLE: u32 = 20070u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_OPTION_ALREADY_EXISTS: u32 = 20075u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_OPTION_DOES_NOT_EXIST: u32 = 20076u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_POSSIBLE_RANGE_CONFLICT: u32 = 20087u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_RANGE_DOES_NOT_EXIST: u32 = 20088u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_RESERVATION_CONFLICT: u32 = 20086u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_RESERVATION_NOT_PRESENT: u32 = 20085u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_SERVER_ADDRESS_MISMATCH: u32 = 20081u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_SERVER_ALREADY_EXISTS: u32 = 20079u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_SERVER_DOES_NOT_EXIST: u32 = 20080u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_SUBNET_EXISTS: u32 = 20082u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_SUBNET_HAS_DIFF_SSCOPE: u32 = 20083u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_SUBNET_NOT_PRESENT: u32 = 20084u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_TOO_MANY_ERRORS: u32 = 20073u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DDS_UNEXPECTED_ERROR: u32 = 20072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_ADDRESS_NOT_AVAILABLE: u32 = 20011u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_CANNOT_MODIFY_BINDINGS: u32 = 20051u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_CANT_CHANGE_ATTRIBUTE: u32 = 20048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_CLASS_ALREADY_EXISTS: u32 = 20045u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_CLASS_NOT_FOUND: u32 = 20044u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_CLIENT_EXISTS: u32 = 20014u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_DATABASE_INIT_FAILED: u32 = 20001u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_DEFAULT_SCOPE_EXITS: u32 = 20047u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_DELETE_BUILTIN_CLASS: u32 = 20089u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_ELEMENT_CANT_REMOVE: u32 = 20007u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_EXEMPTION_EXISTS: u32 = 20055u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_EXEMPTION_NOT_PRESENT: u32 = 20056u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_ADDSCOPE_LEASES_NOT_SYNCED: u32 = 20127u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_BOOT_NOT_SUPPORTED: u32 = 20131u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_FEATURE_NOT_SUPPORTED: u32 = 20134u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_IPRANGE_TYPE_CONV_ILLEGAL: u32 = 20129u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_MAX_ADD_SCOPES: u32 = 20130u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_MAX_RELATIONSHIPS: u32 = 20128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_NOT_SUPPORTED: u32 = 20118u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_RANGE_PART_OF_REL: u32 = 20132u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_RELATIONSHIP_DOES_NOT_EXIST: u32 = 20115u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_RELATIONSHIP_EXISTS: u32 = 20114u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_RELATIONSHIP_NAME_TOO_LONG: u32 = 20125u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_RELATION_IS_SECONDARY: u32 = 20117u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_SCOPE_ALREADY_IN_RELATIONSHIP: u32 = 20113u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_SCOPE_NOT_IN_RELATIONSHIP: u32 = 20116u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_SCOPE_SYNC_IN_PROGRESS: u32 = 20133u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_STATE_NOT_NORMAL: u32 = 20120u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_FO_TIME_OUT_OF_SYNC: u32 = 20119u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_HARDWARE_ADDRESS_TYPE_ALREADY_EXEMPT: u32 = 20101u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_INVALID_DELAY: u32 = 20092u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_INVALID_DHCP_CLIENT: u32 = 20016u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_INVALID_DHCP_MESSAGE: u32 = 20015u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_INVALID_PARAMETER_OPTION32: u32 = 20057u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_INVALID_POLICY_EXPRESSION: u32 = 20109u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_INVALID_PROCESSING_ORDER: u32 = 20110u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_INVALID_RANGE: u32 = 20023u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_INVALID_SUBNET_PREFIX: u32 = 20091u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_IPRANGE_CONV_ILLEGAL: u32 = 20049u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_IPRANGE_EXITS: u32 = 20021u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_IP_ADDRESS_IN_USE: u32 = 20032u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_JET97_CONV_REQUIRED: u32 = 20036u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_JET_CONV_REQUIRED: u32 = 20027u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_JET_ERROR: u32 = 20013u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_LINKLAYER_ADDRESS_DOES_NOT_EXIST: u32 = 20095u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_LINKLAYER_ADDRESS_EXISTS: u32 = 20093u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_LINKLAYER_ADDRESS_RESERVATION_EXISTS: u32 = 20094u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_LOG_FILE_PATH_TOO_LONG: u32 = 20033u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_MSCOPE_EXISTS: u32 = 20053u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_NAP_NOT_SUPPORTED: u32 = 20138u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_NETWORK_CHANGED: u32 = 20050u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_NETWORK_INIT_FAILED: u32 = 20003u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_NOT_RESERVED_CLIENT: u32 = 20018u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_NO_ADMIN_PERMISSION: u32 = 20121u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_OPTION_EXITS: u32 = 20009u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_OPTION_NOT_PRESENT: u32 = 20010u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_OPTION_TYPE_MISMATCH: u32 = 20103u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_POLICY_BAD_PARENT_EXPR: u32 = 20104u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_POLICY_EDIT_FQDN_UNSUPPORTED: u32 = 20137u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_POLICY_EXISTS: u32 = 20105u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_POLICY_FQDN_OPTION_UNSUPPORTED: u32 = 20136u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_POLICY_FQDN_RANGE_UNSUPPORTED: u32 = 20135u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_POLICY_NOT_FOUND: u32 = 20111u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_POLICY_RANGE_BAD: u32 = 20107u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_POLICY_RANGE_EXISTS: u32 = 20106u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_PRIMARY_NOT_FOUND: u32 = 20006u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_RANGE_EXTENDED: u32 = 20024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_RANGE_FULL: u32 = 20012u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_RANGE_INVALID_IN_SERVER_POLICY: u32 = 20108u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_RANGE_TOO_SMALL: u32 = 20020u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_REACHED_END_OF_SELECTION: u32 = 20126u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_REGISTRY_INIT_FAILED: u32 = 20000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_RESERVEDIP_EXITS: u32 = 20022u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_RESERVED_CLIENT: u32 = 20019u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_ROGUE_DS_CONFLICT: u32 = 20041u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_ROGUE_DS_UNREACHABLE: u32 = 20040u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_ROGUE_INIT_FAILED: u32 = 20037u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_ROGUE_NOT_AUTHORIZED: u32 = 20039u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_ROGUE_NOT_OUR_ENTERPRISE: u32 = 20042u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_ROGUE_SAMSHUTDOWN: u32 = 20038u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_ROGUE_STANDALONE_IN_DS: u32 = 20043u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_RPC_INIT_FAILED: u32 = 20002u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_SCOPE_NAME_TOO_LONG: u32 = 20046u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_SERVER_NAME_NOT_RESOLVED: u32 = 20124u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_SERVER_NOT_REACHABLE: u32 = 20122u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_SERVER_NOT_RUNNING: u32 = 20123u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_SERVICE_PAUSED: u32 = 20017u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_SUBNET_EXISTS: u32 = 20052u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_SUBNET_EXITS: u32 = 20004u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_SUBNET_NOT_PRESENT: u32 = 20005u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_SUPER_SCOPE_NAME_TOO_LONG: u32 = 20030u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_UNDEFINED_HARDWARE_ADDRESS_TYPE: u32 = 20102u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_DHCP_UNSUPPORTED_CLIENT: u32 = 20034u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_EXTEND_TOO_SMALL: u32 = 20025u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_LAST_DHCP_SERVER_ERROR: u32 = 20139u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_MSCOPE_RANGE_TOO_SMALL: u32 = 20054u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_SCOPE_RANGE_POLICY_RANGE_CONFLICT: u32 = 20112u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_SERVER_INVALID_BOOT_FILE_TABLE: u32 = 20028u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const ERROR_SERVER_UNKNOWN_BOOT_FILE_NAME: u32 = 20029u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const FILTER_STATUS_FULL_MATCH_IN_ALLOW_LIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const FILTER_STATUS_FULL_MATCH_IN_DENY_LIST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const FILTER_STATUS_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const FILTER_STATUS_WILDCARD_MATCH_IN_ALLOW_LIST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const FILTER_STATUS_WILDCARD_MATCH_IN_DENY_LIST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const HWTYPE_ETHERNET_10MB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const MAC_ADDRESS_LENGTH: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const MAX_PATTERN_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const MCLT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const MODE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_ALL_SUBNETS_MTU: u32 = 27u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_ARP_CACHE_TIMEOUT: u32 = 35u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_BE_A_MASK_SUPPLIER: u32 = 30u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_BE_A_ROUTER: u32 = 19u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_BOOTFILE_NAME: u32 = 67u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_BOOT_FILE_SIZE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_BROADCAST_ADDRESS: u32 = 28u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_CLIENT_CLASS_INFO: u32 = 60u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_CLIENT_ID: u32 = 61u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_COOKIE_SERVERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_DEFAULT_TTL: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_DOMAIN_NAME: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_DOMAIN_NAME_SERVERS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_END: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_ETHERNET_ENCAPSULATION: u32 = 36u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_EXTENSIONS_PATH: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_HOST_NAME: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_IEN116_NAME_SERVERS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_IMPRESS_SERVERS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_KEEP_ALIVE_DATA_SIZE: u32 = 39u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_KEEP_ALIVE_INTERVAL: u32 = 38u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_LEASE_TIME: u32 = 51u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_LOG_SERVERS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_LPR_SERVERS: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_MAX_REASSEMBLY_SIZE: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_MERIT_DUMP_FILE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_MESSAGE: u32 = 56u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_MESSAGE_LENGTH: u32 = 57u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_MESSAGE_TYPE: u32 = 53u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_MSFT_IE_PROXY: u32 = 252u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_MTU: u32 = 26u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_NETBIOS_DATAGRAM_SERVER: u32 = 45u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_NETBIOS_NAME_SERVER: u32 = 44u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_NETBIOS_NODE_TYPE: u32 = 46u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_NETBIOS_SCOPE_OPTION: u32 = 47u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_NETWORK_INFO_SERVERS: u32 = 41u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_NETWORK_INFO_SERVICE_DOM: u32 = 40u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_NETWORK_TIME_SERVERS: u32 = 42u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_NON_LOCAL_SOURCE_ROUTING: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_OK_TO_OVERLAY: u32 = 52u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_PAD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_PARAMETER_REQUEST_LIST: u32 = 55u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_PERFORM_MASK_DISCOVERY: u32 = 29u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_PERFORM_ROUTER_DISCOVERY: u32 = 31u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_PMTU_AGING_TIMEOUT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_PMTU_PLATEAU_TABLE: u32 = 25u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_POLICY_FILTER_FOR_NLSR: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_REBIND_TIME: u32 = 59u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_RENEWAL_TIME: u32 = 58u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_REQUESTED_ADDRESS: u32 = 50u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_RLP_SERVERS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_ROOT_DISK: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_ROUTER_ADDRESS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_ROUTER_SOLICITATION_ADDR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_SERVER_IDENTIFIER: u32 = 54u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_STATIC_ROUTES: u32 = 33u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_SUBNET_MASK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_SWAP_SERVER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_TFTP_SERVER_NAME: u32 = 66u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_TIME_OFFSET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_TIME_SERVERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_TRAILERS: u32 = 34u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_TTL: u32 = 37u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_VENDOR_SPEC_INFO: u32 = 43u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_XWINDOW_DISPLAY_MANAGER: u32 = 49u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const OPTION_XWINDOW_FONT_SERVER: u32 = 48u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const PERCENTAGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const PREVSTATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const QUARANTINE_CONFIG_OPTION: u32 = 43222u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const QUARANTINE_SCOPE_QUARPROFILE_OPTION: u32 = 43221u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const QUARANTIN_OPTION_BASE: u32 = 43220u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const SAFEPERIOD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const SHAREDSECRET: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_APIProtocolSupport: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_AuditLogState: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_BackupInterval: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_BackupPath: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_BootFileTable: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_DatabaseCleanupInterval: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_DatabaseLoggingFlag: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_DatabaseName: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_DatabasePath: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_DebugFlag: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_PingRetries: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_PreferredLifetime: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_PreferredLifetimeIATA: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_QuarantineDefFail: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_QuarantineON: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_RapidCommitFlag: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_RestoreFlag: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_T1: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_T2: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_UnicastFlag: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_ValidLifetime: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Set_ValidLifetimeIATA: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const V5_ADDRESS_BIT_BOTH_REC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const V5_ADDRESS_BIT_DELETED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const V5_ADDRESS_BIT_UNREGISTERED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const V5_ADDRESS_EX_BIT_DISABLE_PTR_RR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const V5_ADDRESS_STATE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const V5_ADDRESS_STATE_DECLINED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const V5_ADDRESS_STATE_DOOM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const V5_ADDRESS_STATE_OFFERED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const WARNING_EXTENDED_LESS: i32 = 20026i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCPV6_STATELESS_PARAM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpStatelessPurgeInterval: DHCPV6_STATELESS_PARAM_TYPE = DHCPV6_STATELESS_PARAM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpStatelessStatus: DHCPV6_STATELESS_PARAM_TYPE = DHCPV6_STATELESS_PARAM_TYPE(2i32);
impl ::core::marker::Copy for DHCPV6_STATELESS_PARAM_TYPE {}
impl ::core::clone::Clone for DHCPV6_STATELESS_PARAM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCPV6_STATELESS_PARAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCPV6_STATELESS_PARAM_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCPV6_STATELESS_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCPV6_STATELESS_PARAM_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_FAILOVER_MODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const LoadBalance: DHCP_FAILOVER_MODE = DHCP_FAILOVER_MODE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const HotStandby: DHCP_FAILOVER_MODE = DHCP_FAILOVER_MODE(1i32);
impl ::core::marker::Copy for DHCP_FAILOVER_MODE {}
impl ::core::clone::Clone for DHCP_FAILOVER_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_FAILOVER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_FAILOVER_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_FAILOVER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_FAILOVER_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_FAILOVER_SERVER(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const PrimaryServer: DHCP_FAILOVER_SERVER = DHCP_FAILOVER_SERVER(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const SecondaryServer: DHCP_FAILOVER_SERVER = DHCP_FAILOVER_SERVER(1i32);
impl ::core::marker::Copy for DHCP_FAILOVER_SERVER {}
impl ::core::clone::Clone for DHCP_FAILOVER_SERVER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_FAILOVER_SERVER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_FAILOVER_SERVER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_FAILOVER_SERVER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_FAILOVER_SERVER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_FILTER_LIST_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Deny: DHCP_FILTER_LIST_TYPE = DHCP_FILTER_LIST_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Allow: DHCP_FILTER_LIST_TYPE = DHCP_FILTER_LIST_TYPE(1i32);
impl ::core::marker::Copy for DHCP_FILTER_LIST_TYPE {}
impl ::core::clone::Clone for DHCP_FILTER_LIST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_FILTER_LIST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_FILTER_LIST_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_FILTER_LIST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_FILTER_LIST_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_FORCE_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpFullForce: DHCP_FORCE_FLAG = DHCP_FORCE_FLAG(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpNoForce: DHCP_FORCE_FLAG = DHCP_FORCE_FLAG(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpFailoverForce: DHCP_FORCE_FLAG = DHCP_FORCE_FLAG(2i32);
impl ::core::marker::Copy for DHCP_FORCE_FLAG {}
impl ::core::clone::Clone for DHCP_FORCE_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_FORCE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_FORCE_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_FORCE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_FORCE_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_OPTION_DATA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpByteOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpWordOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpDWordOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpDWordDWordOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpIpAddressOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpStringDataOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpBinaryDataOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpEncapsulatedDataOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpIpv6AddressOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(8i32);
impl ::core::marker::Copy for DHCP_OPTION_DATA_TYPE {}
impl ::core::clone::Clone for DHCP_OPTION_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_OPTION_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_OPTION_DATA_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_OPTION_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_OPTION_DATA_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_OPTION_SCOPE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpDefaultOptions: DHCP_OPTION_SCOPE_TYPE = DHCP_OPTION_SCOPE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpGlobalOptions: DHCP_OPTION_SCOPE_TYPE = DHCP_OPTION_SCOPE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpSubnetOptions: DHCP_OPTION_SCOPE_TYPE = DHCP_OPTION_SCOPE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpReservedOptions: DHCP_OPTION_SCOPE_TYPE = DHCP_OPTION_SCOPE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpMScopeOptions: DHCP_OPTION_SCOPE_TYPE = DHCP_OPTION_SCOPE_TYPE(4i32);
impl ::core::marker::Copy for DHCP_OPTION_SCOPE_TYPE {}
impl ::core::clone::Clone for DHCP_OPTION_SCOPE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_OPTION_SCOPE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_OPTION_SCOPE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_OPTION_SCOPE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_OPTION_SCOPE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_OPTION_SCOPE_TYPE6(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpDefaultOptions6: DHCP_OPTION_SCOPE_TYPE6 = DHCP_OPTION_SCOPE_TYPE6(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpScopeOptions6: DHCP_OPTION_SCOPE_TYPE6 = DHCP_OPTION_SCOPE_TYPE6(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpReservedOptions6: DHCP_OPTION_SCOPE_TYPE6 = DHCP_OPTION_SCOPE_TYPE6(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpGlobalOptions6: DHCP_OPTION_SCOPE_TYPE6 = DHCP_OPTION_SCOPE_TYPE6(3i32);
impl ::core::marker::Copy for DHCP_OPTION_SCOPE_TYPE6 {}
impl ::core::clone::Clone for DHCP_OPTION_SCOPE_TYPE6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_OPTION_SCOPE_TYPE6 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_OPTION_SCOPE_TYPE6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_OPTION_SCOPE_TYPE6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_OPTION_SCOPE_TYPE6").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_OPTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpUnaryElementTypeOption: DHCP_OPTION_TYPE = DHCP_OPTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpArrayTypeOption: DHCP_OPTION_TYPE = DHCP_OPTION_TYPE(1i32);
impl ::core::marker::Copy for DHCP_OPTION_TYPE {}
impl ::core::clone::Clone for DHCP_OPTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_OPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_OPTION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_OPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_OPTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_POLICY_FIELDS_TO_UPDATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpUpdatePolicyName: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpUpdatePolicyOrder: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpUpdatePolicyExpr: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpUpdatePolicyRanges: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpUpdatePolicyDescr: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpUpdatePolicyStatus: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(32i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpUpdatePolicyDnsSuffix: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(64i32);
impl ::core::marker::Copy for DHCP_POLICY_FIELDS_TO_UPDATE {}
impl ::core::clone::Clone for DHCP_POLICY_FIELDS_TO_UPDATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_POLICY_FIELDS_TO_UPDATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_POLICY_FIELDS_TO_UPDATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_POLICY_FIELDS_TO_UPDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_POLICY_FIELDS_TO_UPDATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_POL_ATTR_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpAttrHWAddr: DHCP_POL_ATTR_TYPE = DHCP_POL_ATTR_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpAttrOption: DHCP_POL_ATTR_TYPE = DHCP_POL_ATTR_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpAttrSubOption: DHCP_POL_ATTR_TYPE = DHCP_POL_ATTR_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpAttrFqdn: DHCP_POL_ATTR_TYPE = DHCP_POL_ATTR_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpAttrFqdnSingleLabel: DHCP_POL_ATTR_TYPE = DHCP_POL_ATTR_TYPE(4i32);
impl ::core::marker::Copy for DHCP_POL_ATTR_TYPE {}
impl ::core::clone::Clone for DHCP_POL_ATTR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_POL_ATTR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_POL_ATTR_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_POL_ATTR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_POL_ATTR_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_POL_COMPARATOR(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpCompEqual: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpCompNotEqual: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpCompBeginsWith: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpCompNotBeginWith: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpCompEndsWith: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpCompNotEndWith: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(5i32);
impl ::core::marker::Copy for DHCP_POL_COMPARATOR {}
impl ::core::clone::Clone for DHCP_POL_COMPARATOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_POL_COMPARATOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_POL_COMPARATOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_POL_COMPARATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_POL_COMPARATOR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_POL_LOGIC_OPER(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpLogicalOr: DHCP_POL_LOGIC_OPER = DHCP_POL_LOGIC_OPER(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpLogicalAnd: DHCP_POL_LOGIC_OPER = DHCP_POL_LOGIC_OPER(1i32);
impl ::core::marker::Copy for DHCP_POL_LOGIC_OPER {}
impl ::core::clone::Clone for DHCP_POL_LOGIC_OPER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_POL_LOGIC_OPER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_POL_LOGIC_OPER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_POL_LOGIC_OPER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_POL_LOGIC_OPER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpPropIdPolicyDnsSuffix: DHCP_PROPERTY_ID = DHCP_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpPropIdClientAddressStateEx: DHCP_PROPERTY_ID = DHCP_PROPERTY_ID(1i32);
impl ::core::marker::Copy for DHCP_PROPERTY_ID {}
impl ::core::clone::Clone for DHCP_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_PROPERTY_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_PROPERTY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpPropTypeByte: DHCP_PROPERTY_TYPE = DHCP_PROPERTY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpPropTypeWord: DHCP_PROPERTY_TYPE = DHCP_PROPERTY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpPropTypeDword: DHCP_PROPERTY_TYPE = DHCP_PROPERTY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpPropTypeString: DHCP_PROPERTY_TYPE = DHCP_PROPERTY_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpPropTypeBinary: DHCP_PROPERTY_TYPE = DHCP_PROPERTY_TYPE(4i32);
impl ::core::marker::Copy for DHCP_PROPERTY_TYPE {}
impl ::core::clone::Clone for DHCP_PROPERTY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_PROPERTY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_PROPERTY_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_PROPERTY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_PROPERTY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_SCAN_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpRegistryFix: DHCP_SCAN_FLAG = DHCP_SCAN_FLAG(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpDatabaseFix: DHCP_SCAN_FLAG = DHCP_SCAN_FLAG(1i32);
impl ::core::marker::Copy for DHCP_SCAN_FLAG {}
impl ::core::clone::Clone for DHCP_SCAN_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_SCAN_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_SCAN_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_SCAN_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_SCAN_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_SEARCH_INFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpClientIpAddress: DHCP_SEARCH_INFO_TYPE = DHCP_SEARCH_INFO_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpClientHardwareAddress: DHCP_SEARCH_INFO_TYPE = DHCP_SEARCH_INFO_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpClientName: DHCP_SEARCH_INFO_TYPE = DHCP_SEARCH_INFO_TYPE(2i32);
impl ::core::marker::Copy for DHCP_SEARCH_INFO_TYPE {}
impl ::core::clone::Clone for DHCP_SEARCH_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_SEARCH_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_SEARCH_INFO_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_SEARCH_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_SEARCH_INFO_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_SEARCH_INFO_TYPE_V6(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Dhcpv6ClientIpAddress: DHCP_SEARCH_INFO_TYPE_V6 = DHCP_SEARCH_INFO_TYPE_V6(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Dhcpv6ClientDUID: DHCP_SEARCH_INFO_TYPE_V6 = DHCP_SEARCH_INFO_TYPE_V6(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Dhcpv6ClientName: DHCP_SEARCH_INFO_TYPE_V6 = DHCP_SEARCH_INFO_TYPE_V6(2i32);
impl ::core::marker::Copy for DHCP_SEARCH_INFO_TYPE_V6 {}
impl ::core::clone::Clone for DHCP_SEARCH_INFO_TYPE_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_SEARCH_INFO_TYPE_V6 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_SEARCH_INFO_TYPE_V6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_SEARCH_INFO_TYPE_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_SEARCH_INFO_TYPE_V6").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_SUBNET_ELEMENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpIpRanges: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpSecondaryHosts: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpReservedIps: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpExcludedIpRanges: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpIpUsedClusters: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpIpRangesDhcpOnly: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpIpRangesDhcpBootp: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpIpRangesBootpOnly: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(7i32);
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_TYPE {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_ELEMENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_SUBNET_ELEMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_SUBNET_ELEMENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_SUBNET_ELEMENT_TYPE_V6(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Dhcpv6IpRanges: DHCP_SUBNET_ELEMENT_TYPE_V6 = DHCP_SUBNET_ELEMENT_TYPE_V6(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Dhcpv6ReservedIps: DHCP_SUBNET_ELEMENT_TYPE_V6 = DHCP_SUBNET_ELEMENT_TYPE_V6(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const Dhcpv6ExcludedIpRanges: DHCP_SUBNET_ELEMENT_TYPE_V6 = DHCP_SUBNET_ELEMENT_TYPE_V6(2i32);
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_TYPE_V6 {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_TYPE_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_TYPE_V6 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_ELEMENT_TYPE_V6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_SUBNET_ELEMENT_TYPE_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_SUBNET_ELEMENT_TYPE_V6").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DHCP_SUBNET_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpSubnetEnabled: DHCP_SUBNET_STATE = DHCP_SUBNET_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpSubnetDisabled: DHCP_SUBNET_STATE = DHCP_SUBNET_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpSubnetEnabledSwitched: DHCP_SUBNET_STATE = DHCP_SUBNET_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpSubnetDisabledSwitched: DHCP_SUBNET_STATE = DHCP_SUBNET_STATE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DhcpSubnetInvalidState: DHCP_SUBNET_STATE = DHCP_SUBNET_STATE(4i32);
impl ::core::marker::Copy for DHCP_SUBNET_STATE {}
impl ::core::clone::Clone for DHCP_SUBNET_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DHCP_SUBNET_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DHCP_SUBNET_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_SUBNET_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FSM_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const NO_STATE: FSM_STATE = FSM_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const INIT: FSM_STATE = FSM_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const STARTUP: FSM_STATE = FSM_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const NORMAL: FSM_STATE = FSM_STATE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const COMMUNICATION_INT: FSM_STATE = FSM_STATE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const PARTNER_DOWN: FSM_STATE = FSM_STATE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const POTENTIAL_CONFLICT: FSM_STATE = FSM_STATE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const CONFLICT_DONE: FSM_STATE = FSM_STATE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const RESOLUTION_INT: FSM_STATE = FSM_STATE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const RECOVER: FSM_STATE = FSM_STATE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const RECOVER_WAIT: FSM_STATE = FSM_STATE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const RECOVER_DONE: FSM_STATE = FSM_STATE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const PAUSED: FSM_STATE = FSM_STATE(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const SHUTDOWN: FSM_STATE = FSM_STATE(13i32);
impl ::core::marker::Copy for FSM_STATE {}
impl ::core::clone::Clone for FSM_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FSM_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FSM_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FSM_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FSM_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QuarantineStatus(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const NOQUARANTINE: QuarantineStatus = QuarantineStatus(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const RESTRICTEDACCESS: QuarantineStatus = QuarantineStatus(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DROPPACKET: QuarantineStatus = QuarantineStatus(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const PROBATION: QuarantineStatus = QuarantineStatus(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const EXEMPT: QuarantineStatus = QuarantineStatus(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const DEFAULTQUARSETTING: QuarantineStatus = QuarantineStatus(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const NOQUARINFO: QuarantineStatus = QuarantineStatus(6i32);
impl ::core::marker::Copy for QuarantineStatus {}
impl ::core::clone::Clone for QuarantineStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QuarantineStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for QuarantineStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for QuarantineStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QuarantineStatus").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StatusCode(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const STATUS_NO_ERROR: StatusCode = StatusCode(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const STATUS_UNSPECIFIED_FAILURE: StatusCode = StatusCode(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const STATUS_NO_BINDING: StatusCode = StatusCode(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub const STATUS_NOPREFIX_AVAIL: StatusCode = StatusCode(6i32);
impl ::core::marker::Copy for StatusCode {}
impl ::core::clone::Clone for StatusCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StatusCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for StatusCode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for StatusCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StatusCode").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DATE_TIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
impl ::core::marker::Copy for DATE_TIME {}
impl ::core::clone::Clone for DATE_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DATE_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DATE_TIME").field("dwLowDateTime", &self.dwLowDateTime).field("dwHighDateTime", &self.dwHighDateTime).finish()
    }
}
impl ::windows::core::TypeKind for DATE_TIME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DATE_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.dwLowDateTime == other.dwLowDateTime && self.dwHighDateTime == other.dwHighDateTime
    }
}
impl ::core::cmp::Eq for DATE_TIME {}
impl ::core::default::Default for DATE_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPAPI_PARAMS {
    pub Flags: u32,
    pub OptionId: u32,
    pub IsVendor: super::super::Foundation::BOOL,
    pub Data: *mut u8,
    pub nBytesData: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPAPI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPAPI_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPAPI_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPAPI_PARAMS").field("Flags", &self.Flags).field("OptionId", &self.OptionId).field("IsVendor", &self.IsVendor).field("Data", &self.Data).field("nBytesData", &self.nBytesData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCPAPI_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPAPI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.OptionId == other.OptionId && self.IsVendor == other.IsVendor && self.Data == other.Data && self.nBytesData == other.nBytesData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPAPI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPAPI_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCPCAPI_CLASSID {
    pub Flags: u32,
    pub Data: *mut u8,
    pub nBytesData: u32,
}
impl ::core::marker::Copy for DHCPCAPI_CLASSID {}
impl ::core::clone::Clone for DHCPCAPI_CLASSID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCPCAPI_CLASSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPCAPI_CLASSID").field("Flags", &self.Flags).field("Data", &self.Data).field("nBytesData", &self.nBytesData).finish()
    }
}
impl ::windows::core::TypeKind for DHCPCAPI_CLASSID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCPCAPI_CLASSID {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Data == other.Data && self.nBytesData == other.nBytesData
    }
}
impl ::core::cmp::Eq for DHCPCAPI_CLASSID {}
impl ::core::default::Default for DHCPCAPI_CLASSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPCAPI_PARAMS_ARRAY {
    pub nParams: u32,
    pub Params: *mut DHCPAPI_PARAMS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPCAPI_PARAMS_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPCAPI_PARAMS_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPCAPI_PARAMS_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPCAPI_PARAMS_ARRAY").field("nParams", &self.nParams).field("Params", &self.Params).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCPCAPI_PARAMS_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPCAPI_PARAMS_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.nParams == other.nParams && self.Params == other.Params
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPCAPI_PARAMS_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPCAPI_PARAMS_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCPDS_SERVER {
    pub Version: u32,
    pub ServerName: ::windows::core::PWSTR,
    pub ServerAddress: u32,
    pub Flags: u32,
    pub State: u32,
    pub DsLocation: ::windows::core::PWSTR,
    pub DsLocType: u32,
}
impl ::core::marker::Copy for DHCPDS_SERVER {}
impl ::core::clone::Clone for DHCPDS_SERVER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCPDS_SERVER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPDS_SERVER").field("Version", &self.Version).field("ServerName", &self.ServerName).field("ServerAddress", &self.ServerAddress).field("Flags", &self.Flags).field("State", &self.State).field("DsLocation", &self.DsLocation).field("DsLocType", &self.DsLocType).finish()
    }
}
impl ::windows::core::TypeKind for DHCPDS_SERVER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCPDS_SERVER {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ServerName == other.ServerName && self.ServerAddress == other.ServerAddress && self.Flags == other.Flags && self.State == other.State && self.DsLocation == other.DsLocation && self.DsLocType == other.DsLocType
    }
}
impl ::core::cmp::Eq for DHCPDS_SERVER {}
impl ::core::default::Default for DHCPDS_SERVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCPDS_SERVERS {
    pub Flags: u32,
    pub NumElements: u32,
    pub Servers: *mut DHCPDS_SERVER,
}
impl ::core::marker::Copy for DHCPDS_SERVERS {}
impl ::core::clone::Clone for DHCPDS_SERVERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCPDS_SERVERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPDS_SERVERS").field("Flags", &self.Flags).field("NumElements", &self.NumElements).field("Servers", &self.Servers).finish()
    }
}
impl ::windows::core::TypeKind for DHCPDS_SERVERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCPDS_SERVERS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.NumElements == other.NumElements && self.Servers == other.Servers
    }
}
impl ::core::cmp::Eq for DHCPDS_SERVERS {}
impl ::core::default::Default for DHCPDS_SERVERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPV4_FAILOVER_CLIENT_INFO {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: ::windows::core::PWSTR,
    pub ClientComment: ::windows::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: super::super::Foundation::BOOL,
    pub SentPotExpTime: u32,
    pub AckPotExpTime: u32,
    pub RecvPotExpTime: u32,
    pub StartTime: u32,
    pub CltLastTransTime: u32,
    pub LastBndUpdTime: u32,
    pub BndMsgStatus: u32,
    pub PolicyName: ::windows::core::PWSTR,
    pub Flags: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPV4_FAILOVER_CLIENT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPV4_FAILOVER_CLIENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPV4_FAILOVER_CLIENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV4_FAILOVER_CLIENT_INFO")
            .field("ClientIpAddress", &self.ClientIpAddress)
            .field("SubnetMask", &self.SubnetMask)
            .field("ClientHardwareAddress", &self.ClientHardwareAddress)
            .field("ClientName", &self.ClientName)
            .field("ClientComment", &self.ClientComment)
            .field("ClientLeaseExpires", &self.ClientLeaseExpires)
            .field("OwnerHost", &self.OwnerHost)
            .field("bClientType", &self.bClientType)
            .field("AddressState", &self.AddressState)
            .field("Status", &self.Status)
            .field("ProbationEnds", &self.ProbationEnds)
            .field("QuarantineCapable", &self.QuarantineCapable)
            .field("SentPotExpTime", &self.SentPotExpTime)
            .field("AckPotExpTime", &self.AckPotExpTime)
            .field("RecvPotExpTime", &self.RecvPotExpTime)
            .field("StartTime", &self.StartTime)
            .field("CltLastTransTime", &self.CltLastTransTime)
            .field("LastBndUpdTime", &self.LastBndUpdTime)
            .field("BndMsgStatus", &self.BndMsgStatus)
            .field("PolicyName", &self.PolicyName)
            .field("Flags", &self.Flags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCPV4_FAILOVER_CLIENT_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV4_FAILOVER_CLIENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress
            && self.SubnetMask == other.SubnetMask
            && self.ClientHardwareAddress == other.ClientHardwareAddress
            && self.ClientName == other.ClientName
            && self.ClientComment == other.ClientComment
            && self.ClientLeaseExpires == other.ClientLeaseExpires
            && self.OwnerHost == other.OwnerHost
            && self.bClientType == other.bClientType
            && self.AddressState == other.AddressState
            && self.Status == other.Status
            && self.ProbationEnds == other.ProbationEnds
            && self.QuarantineCapable == other.QuarantineCapable
            && self.SentPotExpTime == other.SentPotExpTime
            && self.AckPotExpTime == other.AckPotExpTime
            && self.RecvPotExpTime == other.RecvPotExpTime
            && self.StartTime == other.StartTime
            && self.CltLastTransTime == other.CltLastTransTime
            && self.LastBndUpdTime == other.LastBndUpdTime
            && self.BndMsgStatus == other.BndMsgStatus
            && self.PolicyName == other.PolicyName
            && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV4_FAILOVER_CLIENT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV4_FAILOVER_CLIENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCPV4_FAILOVER_CLIENT_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV4_FAILOVER_CLIENT_INFO_ARRAY").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPV4_FAILOVER_CLIENT_INFO_EX {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: ::windows::core::PWSTR,
    pub ClientComment: ::windows::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: super::super::Foundation::BOOL,
    pub SentPotExpTime: u32,
    pub AckPotExpTime: u32,
    pub RecvPotExpTime: u32,
    pub StartTime: u32,
    pub CltLastTransTime: u32,
    pub LastBndUpdTime: u32,
    pub BndMsgStatus: u32,
    pub PolicyName: ::windows::core::PWSTR,
    pub Flags: u8,
    pub AddressStateEx: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPV4_FAILOVER_CLIENT_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPV4_FAILOVER_CLIENT_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPV4_FAILOVER_CLIENT_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV4_FAILOVER_CLIENT_INFO_EX")
            .field("ClientIpAddress", &self.ClientIpAddress)
            .field("SubnetMask", &self.SubnetMask)
            .field("ClientHardwareAddress", &self.ClientHardwareAddress)
            .field("ClientName", &self.ClientName)
            .field("ClientComment", &self.ClientComment)
            .field("ClientLeaseExpires", &self.ClientLeaseExpires)
            .field("OwnerHost", &self.OwnerHost)
            .field("bClientType", &self.bClientType)
            .field("AddressState", &self.AddressState)
            .field("Status", &self.Status)
            .field("ProbationEnds", &self.ProbationEnds)
            .field("QuarantineCapable", &self.QuarantineCapable)
            .field("SentPotExpTime", &self.SentPotExpTime)
            .field("AckPotExpTime", &self.AckPotExpTime)
            .field("RecvPotExpTime", &self.RecvPotExpTime)
            .field("StartTime", &self.StartTime)
            .field("CltLastTransTime", &self.CltLastTransTime)
            .field("LastBndUpdTime", &self.LastBndUpdTime)
            .field("BndMsgStatus", &self.BndMsgStatus)
            .field("PolicyName", &self.PolicyName)
            .field("Flags", &self.Flags)
            .field("AddressStateEx", &self.AddressStateEx)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCPV4_FAILOVER_CLIENT_INFO_EX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV4_FAILOVER_CLIENT_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress
            && self.SubnetMask == other.SubnetMask
            && self.ClientHardwareAddress == other.ClientHardwareAddress
            && self.ClientName == other.ClientName
            && self.ClientComment == other.ClientComment
            && self.ClientLeaseExpires == other.ClientLeaseExpires
            && self.OwnerHost == other.OwnerHost
            && self.bClientType == other.bClientType
            && self.AddressState == other.AddressState
            && self.Status == other.Status
            && self.ProbationEnds == other.ProbationEnds
            && self.QuarantineCapable == other.QuarantineCapable
            && self.SentPotExpTime == other.SentPotExpTime
            && self.AckPotExpTime == other.AckPotExpTime
            && self.RecvPotExpTime == other.RecvPotExpTime
            && self.StartTime == other.StartTime
            && self.CltLastTransTime == other.CltLastTransTime
            && self.LastBndUpdTime == other.LastBndUpdTime
            && self.BndMsgStatus == other.BndMsgStatus
            && self.PolicyName == other.PolicyName
            && self.Flags == other.Flags
            && self.AddressStateEx == other.AddressStateEx
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV4_FAILOVER_CLIENT_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV4_FAILOVER_CLIENT_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCPV6CAPI_CLASSID {
    pub Flags: u32,
    pub Data: *mut u8,
    pub nBytesData: u32,
}
impl ::core::marker::Copy for DHCPV6CAPI_CLASSID {}
impl ::core::clone::Clone for DHCPV6CAPI_CLASSID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCPV6CAPI_CLASSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6CAPI_CLASSID").field("Flags", &self.Flags).field("Data", &self.Data).field("nBytesData", &self.nBytesData).finish()
    }
}
impl ::windows::core::TypeKind for DHCPV6CAPI_CLASSID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCPV6CAPI_CLASSID {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Data == other.Data && self.nBytesData == other.nBytesData
    }
}
impl ::core::cmp::Eq for DHCPV6CAPI_CLASSID {}
impl ::core::default::Default for DHCPV6CAPI_CLASSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPV6CAPI_PARAMS {
    pub Flags: u32,
    pub OptionId: u32,
    pub IsVendor: super::super::Foundation::BOOL,
    pub Data: *mut u8,
    pub nBytesData: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPV6CAPI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPV6CAPI_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPV6CAPI_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6CAPI_PARAMS").field("Flags", &self.Flags).field("OptionId", &self.OptionId).field("IsVendor", &self.IsVendor).field("Data", &self.Data).field("nBytesData", &self.nBytesData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCPV6CAPI_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV6CAPI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.OptionId == other.OptionId && self.IsVendor == other.IsVendor && self.Data == other.Data && self.nBytesData == other.nBytesData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV6CAPI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV6CAPI_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPV6CAPI_PARAMS_ARRAY {
    pub nParams: u32,
    pub Params: *mut DHCPV6CAPI_PARAMS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPV6CAPI_PARAMS_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPV6CAPI_PARAMS_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPV6CAPI_PARAMS_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6CAPI_PARAMS_ARRAY").field("nParams", &self.nParams).field("Params", &self.Params).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCPV6CAPI_PARAMS_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV6CAPI_PARAMS_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.nParams == other.nParams && self.Params == other.Params
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV6CAPI_PARAMS_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV6CAPI_PARAMS_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCPV6Prefix {
    pub prefix: [u8; 16],
    pub prefixLength: u32,
    pub preferredLifeTime: u32,
    pub validLifeTime: u32,
    pub status: StatusCode,
}
impl ::core::marker::Copy for DHCPV6Prefix {}
impl ::core::clone::Clone for DHCPV6Prefix {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCPV6Prefix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6Prefix").field("prefix", &self.prefix).field("prefixLength", &self.prefixLength).field("preferredLifeTime", &self.preferredLifeTime).field("validLifeTime", &self.validLifeTime).field("status", &self.status).finish()
    }
}
impl ::windows::core::TypeKind for DHCPV6Prefix {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCPV6Prefix {
    fn eq(&self, other: &Self) -> bool {
        self.prefix == other.prefix && self.prefixLength == other.prefixLength && self.preferredLifeTime == other.preferredLifeTime && self.validLifeTime == other.validLifeTime && self.status == other.status
    }
}
impl ::core::cmp::Eq for DHCPV6Prefix {}
impl ::core::default::Default for DHCPV6Prefix {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCPV6PrefixLeaseInformation {
    pub nPrefixes: u32,
    pub prefixArray: *mut DHCPV6Prefix,
    pub iaid: u32,
    pub T1: i64,
    pub T2: i64,
    pub MaxLeaseExpirationTime: i64,
    pub LastRenewalTime: i64,
    pub status: StatusCode,
    pub ServerId: *mut u8,
    pub ServerIdLen: u32,
}
impl ::core::marker::Copy for DHCPV6PrefixLeaseInformation {}
impl ::core::clone::Clone for DHCPV6PrefixLeaseInformation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCPV6PrefixLeaseInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6PrefixLeaseInformation").field("nPrefixes", &self.nPrefixes).field("prefixArray", &self.prefixArray).field("iaid", &self.iaid).field("T1", &self.T1).field("T2", &self.T2).field("MaxLeaseExpirationTime", &self.MaxLeaseExpirationTime).field("LastRenewalTime", &self.LastRenewalTime).field("status", &self.status).field("ServerId", &self.ServerId).field("ServerIdLen", &self.ServerIdLen).finish()
    }
}
impl ::windows::core::TypeKind for DHCPV6PrefixLeaseInformation {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCPV6PrefixLeaseInformation {
    fn eq(&self, other: &Self) -> bool {
        self.nPrefixes == other.nPrefixes && self.prefixArray == other.prefixArray && self.iaid == other.iaid && self.T1 == other.T1 && self.T2 == other.T2 && self.MaxLeaseExpirationTime == other.MaxLeaseExpirationTime && self.LastRenewalTime == other.LastRenewalTime && self.status == other.status && self.ServerId == other.ServerId && self.ServerIdLen == other.ServerIdLen
    }
}
impl ::core::cmp::Eq for DHCPV6PrefixLeaseInformation {}
impl ::core::default::Default for DHCPV6PrefixLeaseInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPV6_BIND_ELEMENT {
    pub Flags: u32,
    pub fBoundToDHCPServer: super::super::Foundation::BOOL,
    pub AdapterPrimaryAddress: DHCP_IPV6_ADDRESS,
    pub AdapterSubnetAddress: DHCP_IPV6_ADDRESS,
    pub IfDescription: ::windows::core::PWSTR,
    pub IpV6IfIndex: u32,
    pub IfIdSize: u32,
    pub IfId: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPV6_BIND_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPV6_BIND_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPV6_BIND_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6_BIND_ELEMENT").field("Flags", &self.Flags).field("fBoundToDHCPServer", &self.fBoundToDHCPServer).field("AdapterPrimaryAddress", &self.AdapterPrimaryAddress).field("AdapterSubnetAddress", &self.AdapterSubnetAddress).field("IfDescription", &self.IfDescription).field("IpV6IfIndex", &self.IpV6IfIndex).field("IfIdSize", &self.IfIdSize).field("IfId", &self.IfId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCPV6_BIND_ELEMENT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV6_BIND_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.fBoundToDHCPServer == other.fBoundToDHCPServer && self.AdapterPrimaryAddress == other.AdapterPrimaryAddress && self.AdapterSubnetAddress == other.AdapterSubnetAddress && self.IfDescription == other.IfDescription && self.IpV6IfIndex == other.IpV6IfIndex && self.IfIdSize == other.IfIdSize && self.IfId == other.IfId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV6_BIND_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV6_BIND_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPV6_BIND_ELEMENT_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCPV6_BIND_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPV6_BIND_ELEMENT_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPV6_BIND_ELEMENT_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPV6_BIND_ELEMENT_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6_BIND_ELEMENT_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCPV6_BIND_ELEMENT_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV6_BIND_ELEMENT_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV6_BIND_ELEMENT_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV6_BIND_ELEMENT_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCPV6_IP_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_IPV6_ADDRESS,
}
impl ::core::marker::Copy for DHCPV6_IP_ARRAY {}
impl ::core::clone::Clone for DHCPV6_IP_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCPV6_IP_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6_IP_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::windows::core::TypeKind for DHCPV6_IP_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCPV6_IP_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCPV6_IP_ARRAY {}
impl ::core::default::Default for DHCPV6_IP_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCPV6_STATELESS_PARAMS {
    pub Status: super::super::Foundation::BOOL,
    pub PurgeInterval: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCPV6_STATELESS_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCPV6_STATELESS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPV6_STATELESS_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6_STATELESS_PARAMS").field("Status", &self.Status).field("PurgeInterval", &self.PurgeInterval).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCPV6_STATELESS_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV6_STATELESS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.PurgeInterval == other.PurgeInterval
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV6_STATELESS_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV6_STATELESS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCPV6_STATELESS_SCOPE_STATS {
    pub SubnetAddress: DHCP_IPV6_ADDRESS,
    pub NumStatelessClientsAdded: u64,
    pub NumStatelessClientsRemoved: u64,
}
impl ::core::marker::Copy for DHCPV6_STATELESS_SCOPE_STATS {}
impl ::core::clone::Clone for DHCPV6_STATELESS_SCOPE_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCPV6_STATELESS_SCOPE_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6_STATELESS_SCOPE_STATS").field("SubnetAddress", &self.SubnetAddress).field("NumStatelessClientsAdded", &self.NumStatelessClientsAdded).field("NumStatelessClientsRemoved", &self.NumStatelessClientsRemoved).finish()
    }
}
impl ::windows::core::TypeKind for DHCPV6_STATELESS_SCOPE_STATS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCPV6_STATELESS_SCOPE_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.SubnetAddress == other.SubnetAddress && self.NumStatelessClientsAdded == other.NumStatelessClientsAdded && self.NumStatelessClientsRemoved == other.NumStatelessClientsRemoved
    }
}
impl ::core::cmp::Eq for DHCPV6_STATELESS_SCOPE_STATS {}
impl ::core::default::Default for DHCPV6_STATELESS_SCOPE_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCPV6_STATELESS_STATS {
    pub NumScopes: u32,
    pub ScopeStats: *mut DHCPV6_STATELESS_SCOPE_STATS,
}
impl ::core::marker::Copy for DHCPV6_STATELESS_STATS {}
impl ::core::clone::Clone for DHCPV6_STATELESS_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCPV6_STATELESS_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6_STATELESS_STATS").field("NumScopes", &self.NumScopes).field("ScopeStats", &self.ScopeStats).finish()
    }
}
impl ::windows::core::TypeKind for DHCPV6_STATELESS_STATS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCPV6_STATELESS_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.NumScopes == other.NumScopes && self.ScopeStats == other.ScopeStats
    }
}
impl ::core::cmp::Eq for DHCPV6_STATELESS_STATS {}
impl ::core::default::Default for DHCPV6_STATELESS_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_ADDR_PATTERN {
    pub MatchHWType: super::super::Foundation::BOOL,
    pub HWType: u8,
    pub IsWildcard: super::super::Foundation::BOOL,
    pub Length: u8,
    pub Pattern: [u8; 255],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ADDR_PATTERN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ADDR_PATTERN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_ADDR_PATTERN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_ADDR_PATTERN").field("MatchHWType", &self.MatchHWType).field("HWType", &self.HWType).field("IsWildcard", &self.IsWildcard).field("Length", &self.Length).field("Pattern", &self.Pattern).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_ADDR_PATTERN {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ADDR_PATTERN {
    fn eq(&self, other: &Self) -> bool {
        self.MatchHWType == other.MatchHWType && self.HWType == other.HWType && self.IsWildcard == other.IsWildcard && self.Length == other.Length && self.Pattern == other.Pattern
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ADDR_PATTERN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ADDR_PATTERN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_ALL_OPTIONS {
    pub Flags: u32,
    pub NonVendorOptions: *mut DHCP_OPTION_ARRAY,
    pub NumVendorOptions: u32,
    pub VendorOptions: *mut DHCP_ALL_OPTIONS_0,
}
impl ::core::marker::Copy for DHCP_ALL_OPTIONS {}
impl ::core::clone::Clone for DHCP_ALL_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_ALL_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_ALL_OPTIONS").field("Flags", &self.Flags).field("NonVendorOptions", &self.NonVendorOptions).field("NumVendorOptions", &self.NumVendorOptions).field("VendorOptions", &self.VendorOptions).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_ALL_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_ALL_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.NonVendorOptions == other.NonVendorOptions && self.NumVendorOptions == other.NumVendorOptions && self.VendorOptions == other.VendorOptions
    }
}
impl ::core::cmp::Eq for DHCP_ALL_OPTIONS {}
impl ::core::default::Default for DHCP_ALL_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_ALL_OPTIONS_0 {
    pub Option: DHCP_OPTION,
    pub VendorName: ::windows::core::PWSTR,
    pub ClassName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DHCP_ALL_OPTIONS_0 {}
impl ::core::clone::Clone for DHCP_ALL_OPTIONS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_ALL_OPTIONS_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_ALL_OPTIONS_0").field("Option", &self.Option).field("VendorName", &self.VendorName).field("ClassName", &self.ClassName).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_ALL_OPTIONS_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_ALL_OPTIONS_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Option == other.Option && self.VendorName == other.VendorName && self.ClassName == other.ClassName
    }
}
impl ::core::cmp::Eq for DHCP_ALL_OPTIONS_0 {}
impl ::core::default::Default for DHCP_ALL_OPTIONS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_ALL_OPTION_VALUES {
    pub Flags: u32,
    pub NumElements: u32,
    pub Options: *mut DHCP_ALL_OPTION_VALUES_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ALL_OPTION_VALUES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ALL_OPTION_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_ALL_OPTION_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_ALL_OPTION_VALUES").field("Flags", &self.Flags).field("NumElements", &self.NumElements).field("Options", &self.Options).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_ALL_OPTION_VALUES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ALL_OPTION_VALUES {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.NumElements == other.NumElements && self.Options == other.Options
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ALL_OPTION_VALUES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ALL_OPTION_VALUES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_ALL_OPTION_VALUES_0 {
    pub ClassName: ::windows::core::PWSTR,
    pub VendorName: ::windows::core::PWSTR,
    pub IsVendor: super::super::Foundation::BOOL,
    pub OptionsArray: *mut DHCP_OPTION_VALUE_ARRAY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ALL_OPTION_VALUES_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ALL_OPTION_VALUES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_ALL_OPTION_VALUES_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_ALL_OPTION_VALUES_0").field("ClassName", &self.ClassName).field("VendorName", &self.VendorName).field("IsVendor", &self.IsVendor).field("OptionsArray", &self.OptionsArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_ALL_OPTION_VALUES_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ALL_OPTION_VALUES_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ClassName == other.ClassName && self.VendorName == other.VendorName && self.IsVendor == other.IsVendor && self.OptionsArray == other.OptionsArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ALL_OPTION_VALUES_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ALL_OPTION_VALUES_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_ALL_OPTION_VALUES_PB {
    pub Flags: u32,
    pub NumElements: u32,
    pub Options: *mut DHCP_ALL_OPTION_VALUES_PB_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ALL_OPTION_VALUES_PB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ALL_OPTION_VALUES_PB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_ALL_OPTION_VALUES_PB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_ALL_OPTION_VALUES_PB").field("Flags", &self.Flags).field("NumElements", &self.NumElements).field("Options", &self.Options).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_ALL_OPTION_VALUES_PB {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ALL_OPTION_VALUES_PB {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.NumElements == other.NumElements && self.Options == other.Options
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ALL_OPTION_VALUES_PB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ALL_OPTION_VALUES_PB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_ALL_OPTION_VALUES_PB_0 {
    pub PolicyName: ::windows::core::PWSTR,
    pub VendorName: ::windows::core::PWSTR,
    pub IsVendor: super::super::Foundation::BOOL,
    pub OptionsArray: *mut DHCP_OPTION_VALUE_ARRAY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ALL_OPTION_VALUES_PB_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ALL_OPTION_VALUES_PB_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_ALL_OPTION_VALUES_PB_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_ALL_OPTION_VALUES_PB_0").field("PolicyName", &self.PolicyName).field("VendorName", &self.VendorName).field("IsVendor", &self.IsVendor).field("OptionsArray", &self.OptionsArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_ALL_OPTION_VALUES_PB_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ALL_OPTION_VALUES_PB_0 {
    fn eq(&self, other: &Self) -> bool {
        self.PolicyName == other.PolicyName && self.VendorName == other.VendorName && self.IsVendor == other.IsVendor && self.OptionsArray == other.OptionsArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ALL_OPTION_VALUES_PB_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ALL_OPTION_VALUES_PB_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_ATTRIB {
    pub DhcpAttribId: u32,
    pub DhcpAttribType: u32,
    pub Anonymous: DHCP_ATTRIB_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ATTRIB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ATTRIB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_ATTRIB {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ATTRIB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DHCP_ATTRIB_0 {
    pub DhcpAttribBool: super::super::Foundation::BOOL,
    pub DhcpAttribUlong: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ATTRIB_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ATTRIB_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_ATTRIB_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ATTRIB_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_ATTRIB_ARRAY {
    pub NumElements: u32,
    pub DhcpAttribs: *mut DHCP_ATTRIB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_ATTRIB_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_ATTRIB_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_ATTRIB_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_ATTRIB_ARRAY").field("NumElements", &self.NumElements).field("DhcpAttribs", &self.DhcpAttribs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_ATTRIB_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ATTRIB_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.DhcpAttribs == other.DhcpAttribs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ATTRIB_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ATTRIB_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_BINARY_DATA {
    pub DataLength: u32,
    pub Data: *mut u8,
}
impl ::core::marker::Copy for DHCP_BINARY_DATA {}
impl ::core::clone::Clone for DHCP_BINARY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_BINARY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_BINARY_DATA").field("DataLength", &self.DataLength).field("Data", &self.Data).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_BINARY_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_BINARY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DataLength == other.DataLength && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for DHCP_BINARY_DATA {}
impl ::core::default::Default for DHCP_BINARY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_BIND_ELEMENT {
    pub Flags: u32,
    pub fBoundToDHCPServer: super::super::Foundation::BOOL,
    pub AdapterPrimaryAddress: u32,
    pub AdapterSubnetAddress: u32,
    pub IfDescription: ::windows::core::PWSTR,
    pub IfIdSize: u32,
    pub IfId: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_BIND_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_BIND_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_BIND_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_BIND_ELEMENT").field("Flags", &self.Flags).field("fBoundToDHCPServer", &self.fBoundToDHCPServer).field("AdapterPrimaryAddress", &self.AdapterPrimaryAddress).field("AdapterSubnetAddress", &self.AdapterSubnetAddress).field("IfDescription", &self.IfDescription).field("IfIdSize", &self.IfIdSize).field("IfId", &self.IfId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_BIND_ELEMENT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_BIND_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.fBoundToDHCPServer == other.fBoundToDHCPServer && self.AdapterPrimaryAddress == other.AdapterPrimaryAddress && self.AdapterSubnetAddress == other.AdapterSubnetAddress && self.IfDescription == other.IfDescription && self.IfIdSize == other.IfIdSize && self.IfId == other.IfId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_BIND_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_BIND_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_BIND_ELEMENT_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_BIND_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_BIND_ELEMENT_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_BIND_ELEMENT_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_BIND_ELEMENT_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_BIND_ELEMENT_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_BIND_ELEMENT_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_BIND_ELEMENT_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_BIND_ELEMENT_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_BIND_ELEMENT_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_BOOTP_IP_RANGE {
    pub StartAddress: u32,
    pub EndAddress: u32,
    pub BootpAllocated: u32,
    pub MaxBootpAllowed: u32,
}
impl ::core::marker::Copy for DHCP_BOOTP_IP_RANGE {}
impl ::core::clone::Clone for DHCP_BOOTP_IP_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_BOOTP_IP_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_BOOTP_IP_RANGE").field("StartAddress", &self.StartAddress).field("EndAddress", &self.EndAddress).field("BootpAllocated", &self.BootpAllocated).field("MaxBootpAllowed", &self.MaxBootpAllowed).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_BOOTP_IP_RANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_BOOTP_IP_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartAddress == other.StartAddress && self.EndAddress == other.EndAddress && self.BootpAllocated == other.BootpAllocated && self.MaxBootpAllowed == other.MaxBootpAllowed
    }
}
impl ::core::cmp::Eq for DHCP_BOOTP_IP_RANGE {}
impl ::core::default::Default for DHCP_BOOTP_IP_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CALLOUT_TABLE {
    pub DhcpControlHook: LPDHCP_CONTROL,
    pub DhcpNewPktHook: LPDHCP_NEWPKT,
    pub DhcpPktDropHook: LPDHCP_DROP_SEND,
    pub DhcpPktSendHook: LPDHCP_DROP_SEND,
    pub DhcpAddressDelHook: LPDHCP_PROB,
    pub DhcpAddressOfferHook: LPDHCP_GIVE_ADDRESS,
    pub DhcpHandleOptionsHook: LPDHCP_HANDLE_OPTIONS,
    pub DhcpDeleteClientHook: LPDHCP_DELETE_CLIENT,
    pub DhcpExtensionHook: *mut ::core::ffi::c_void,
    pub DhcpReservedHook: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CALLOUT_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CALLOUT_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CALLOUT_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CALLOUT_TABLE").field("DhcpExtensionHook", &self.DhcpExtensionHook).field("DhcpReservedHook", &self.DhcpReservedHook).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_CALLOUT_TABLE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CALLOUT_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLASS_INFO {
    pub ClassName: ::windows::core::PWSTR,
    pub ClassComment: ::windows::core::PWSTR,
    pub ClassDataLength: u32,
    pub IsVendor: super::super::Foundation::BOOL,
    pub Flags: u32,
    pub ClassData: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLASS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLASS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLASS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLASS_INFO").field("ClassName", &self.ClassName).field("ClassComment", &self.ClassComment).field("ClassDataLength", &self.ClassDataLength).field("IsVendor", &self.IsVendor).field("Flags", &self.Flags).field("ClassData", &self.ClassData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_CLASS_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLASS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ClassName == other.ClassName && self.ClassComment == other.ClassComment && self.ClassDataLength == other.ClassDataLength && self.IsVendor == other.IsVendor && self.Flags == other.Flags && self.ClassData == other.ClassData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLASS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLASS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLASS_INFO_ARRAY {
    pub NumElements: u32,
    pub Classes: *mut DHCP_CLASS_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLASS_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLASS_INFO_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLASS_INFO_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLASS_INFO_ARRAY").field("NumElements", &self.NumElements).field("Classes", &self.Classes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_CLASS_INFO_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLASS_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Classes == other.Classes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLASS_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLASS_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLASS_INFO_ARRAY_V6 {
    pub NumElements: u32,
    pub Classes: *mut DHCP_CLASS_INFO_V6,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLASS_INFO_ARRAY_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLASS_INFO_ARRAY_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLASS_INFO_ARRAY_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLASS_INFO_ARRAY_V6").field("NumElements", &self.NumElements).field("Classes", &self.Classes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_CLASS_INFO_ARRAY_V6 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLASS_INFO_ARRAY_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Classes == other.Classes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLASS_INFO_ARRAY_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLASS_INFO_ARRAY_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLASS_INFO_V6 {
    pub ClassName: ::windows::core::PWSTR,
    pub ClassComment: ::windows::core::PWSTR,
    pub ClassDataLength: u32,
    pub IsVendor: super::super::Foundation::BOOL,
    pub EnterpriseNumber: u32,
    pub Flags: u32,
    pub ClassData: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLASS_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLASS_INFO_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLASS_INFO_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLASS_INFO_V6").field("ClassName", &self.ClassName).field("ClassComment", &self.ClassComment).field("ClassDataLength", &self.ClassDataLength).field("IsVendor", &self.IsVendor).field("EnterpriseNumber", &self.EnterpriseNumber).field("Flags", &self.Flags).field("ClassData", &self.ClassData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_CLASS_INFO_V6 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLASS_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.ClassName == other.ClassName && self.ClassComment == other.ClassComment && self.ClassDataLength == other.ClassDataLength && self.IsVendor == other.IsVendor && self.EnterpriseNumber == other.EnterpriseNumber && self.Flags == other.Flags && self.ClassData == other.ClassData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLASS_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLASS_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_FILTER_STATUS_INFO {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: ::windows::core::PWSTR,
    pub ClientComment: ::windows::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: super::super::Foundation::BOOL,
    pub FilterStatus: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_FILTER_STATUS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_FILTER_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLIENT_FILTER_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_FILTER_STATUS_INFO")
            .field("ClientIpAddress", &self.ClientIpAddress)
            .field("SubnetMask", &self.SubnetMask)
            .field("ClientHardwareAddress", &self.ClientHardwareAddress)
            .field("ClientName", &self.ClientName)
            .field("ClientComment", &self.ClientComment)
            .field("ClientLeaseExpires", &self.ClientLeaseExpires)
            .field("OwnerHost", &self.OwnerHost)
            .field("bClientType", &self.bClientType)
            .field("AddressState", &self.AddressState)
            .field("Status", &self.Status)
            .field("ProbationEnds", &self.ProbationEnds)
            .field("QuarantineCapable", &self.QuarantineCapable)
            .field("FilterStatus", &self.FilterStatus)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_CLIENT_FILTER_STATUS_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_FILTER_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress && self.SubnetMask == other.SubnetMask && self.ClientHardwareAddress == other.ClientHardwareAddress && self.ClientName == other.ClientName && self.ClientComment == other.ClientComment && self.ClientLeaseExpires == other.ClientLeaseExpires && self.OwnerHost == other.OwnerHost && self.bClientType == other.bClientType && self.AddressState == other.AddressState && self.Status == other.Status && self.ProbationEnds == other.ProbationEnds && self.QuarantineCapable == other.QuarantineCapable && self.FilterStatus == other.FilterStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_FILTER_STATUS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_FILTER_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_FILTER_STATUS_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_CLIENT_INFO {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: ::windows::core::PWSTR,
    pub ClientComment: ::windows::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
}
impl ::core::marker::Copy for DHCP_CLIENT_INFO {}
impl ::core::clone::Clone for DHCP_CLIENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_CLIENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO").field("ClientIpAddress", &self.ClientIpAddress).field("SubnetMask", &self.SubnetMask).field("ClientHardwareAddress", &self.ClientHardwareAddress).field("ClientName", &self.ClientName).field("ClientComment", &self.ClientComment).field("ClientLeaseExpires", &self.ClientLeaseExpires).field("OwnerHost", &self.OwnerHost).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_CLIENT_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress && self.SubnetMask == other.SubnetMask && self.ClientHardwareAddress == other.ClientHardwareAddress && self.ClientName == other.ClientName && self.ClientComment == other.ClientComment && self.ClientLeaseExpires == other.ClientLeaseExpires && self.OwnerHost == other.OwnerHost
    }
}
impl ::core::cmp::Eq for DHCP_CLIENT_INFO {}
impl ::core::default::Default for DHCP_CLIENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_CLIENT_INFO_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO,
}
impl ::core::marker::Copy for DHCP_CLIENT_INFO_ARRAY {}
impl ::core::clone::Clone for DHCP_CLIENT_INFO_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_ARRAY").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_CLIENT_INFO_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_ARRAY {}
impl ::core::default::Default for DHCP_CLIENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_CLIENT_INFO_ARRAY_V4 {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_V4,
}
impl ::core::marker::Copy for DHCP_CLIENT_INFO_ARRAY_V4 {}
impl ::core::clone::Clone for DHCP_CLIENT_INFO_ARRAY_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_ARRAY_V4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_ARRAY_V4").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_CLIENT_INFO_ARRAY_V4 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_ARRAY_V4 {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_ARRAY_V4 {}
impl ::core::default::Default for DHCP_CLIENT_INFO_ARRAY_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_CLIENT_INFO_ARRAY_V5 {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_V5,
}
impl ::core::marker::Copy for DHCP_CLIENT_INFO_ARRAY_V5 {}
impl ::core::clone::Clone for DHCP_CLIENT_INFO_ARRAY_V5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_ARRAY_V5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_ARRAY_V5").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_CLIENT_INFO_ARRAY_V5 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_ARRAY_V5 {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_ARRAY_V5 {}
impl ::core::default::Default for DHCP_CLIENT_INFO_ARRAY_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_CLIENT_INFO_ARRAY_V6 {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_V6,
}
impl ::core::marker::Copy for DHCP_CLIENT_INFO_ARRAY_V6 {}
impl ::core::clone::Clone for DHCP_CLIENT_INFO_ARRAY_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_ARRAY_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_ARRAY_V6").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_CLIENT_INFO_ARRAY_V6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_ARRAY_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_ARRAY_V6 {}
impl ::core::default::Default for DHCP_CLIENT_INFO_ARRAY_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_ARRAY_VQ {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_VQ,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_ARRAY_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_ARRAY_VQ {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_ARRAY_VQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_ARRAY_VQ").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_CLIENT_INFO_ARRAY_VQ {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_ARRAY_VQ {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_ARRAY_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_ARRAY_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_EX {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: ::windows::core::PWSTR,
    pub ClientComment: ::windows::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: super::super::Foundation::BOOL,
    pub FilterStatus: u32,
    pub PolicyName: ::windows::core::PWSTR,
    pub Properties: *mut DHCP_PROPERTY_ARRAY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_EX")
            .field("ClientIpAddress", &self.ClientIpAddress)
            .field("SubnetMask", &self.SubnetMask)
            .field("ClientHardwareAddress", &self.ClientHardwareAddress)
            .field("ClientName", &self.ClientName)
            .field("ClientComment", &self.ClientComment)
            .field("ClientLeaseExpires", &self.ClientLeaseExpires)
            .field("OwnerHost", &self.OwnerHost)
            .field("bClientType", &self.bClientType)
            .field("AddressState", &self.AddressState)
            .field("Status", &self.Status)
            .field("ProbationEnds", &self.ProbationEnds)
            .field("QuarantineCapable", &self.QuarantineCapable)
            .field("FilterStatus", &self.FilterStatus)
            .field("PolicyName", &self.PolicyName)
            .field("Properties", &self.Properties)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_CLIENT_INFO_EX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress && self.SubnetMask == other.SubnetMask && self.ClientHardwareAddress == other.ClientHardwareAddress && self.ClientName == other.ClientName && self.ClientComment == other.ClientComment && self.ClientLeaseExpires == other.ClientLeaseExpires && self.OwnerHost == other.OwnerHost && self.bClientType == other.bClientType && self.AddressState == other.AddressState && self.Status == other.Status && self.ProbationEnds == other.ProbationEnds && self.QuarantineCapable == other.QuarantineCapable && self.FilterStatus == other.FilterStatus && self.PolicyName == other.PolicyName && self.Properties == other.Properties
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_EX_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_EX,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_EX_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_EX_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_EX_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_EX_ARRAY").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_CLIENT_INFO_EX_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_EX_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_EX_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_EX_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_PB {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: ::windows::core::PWSTR,
    pub ClientComment: ::windows::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: super::super::Foundation::BOOL,
    pub FilterStatus: u32,
    pub PolicyName: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_PB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_PB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_PB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_PB")
            .field("ClientIpAddress", &self.ClientIpAddress)
            .field("SubnetMask", &self.SubnetMask)
            .field("ClientHardwareAddress", &self.ClientHardwareAddress)
            .field("ClientName", &self.ClientName)
            .field("ClientComment", &self.ClientComment)
            .field("ClientLeaseExpires", &self.ClientLeaseExpires)
            .field("OwnerHost", &self.OwnerHost)
            .field("bClientType", &self.bClientType)
            .field("AddressState", &self.AddressState)
            .field("Status", &self.Status)
            .field("ProbationEnds", &self.ProbationEnds)
            .field("QuarantineCapable", &self.QuarantineCapable)
            .field("FilterStatus", &self.FilterStatus)
            .field("PolicyName", &self.PolicyName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_CLIENT_INFO_PB {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_PB {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress && self.SubnetMask == other.SubnetMask && self.ClientHardwareAddress == other.ClientHardwareAddress && self.ClientName == other.ClientName && self.ClientComment == other.ClientComment && self.ClientLeaseExpires == other.ClientLeaseExpires && self.OwnerHost == other.OwnerHost && self.bClientType == other.bClientType && self.AddressState == other.AddressState && self.Status == other.Status && self.ProbationEnds == other.ProbationEnds && self.QuarantineCapable == other.QuarantineCapable && self.FilterStatus == other.FilterStatus && self.PolicyName == other.PolicyName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_PB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_PB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_PB_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_PB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_PB_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_PB_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_PB_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_PB_ARRAY").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_CLIENT_INFO_PB_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_PB_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_PB_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_PB_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_CLIENT_INFO_V4 {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: ::windows::core::PWSTR,
    pub ClientComment: ::windows::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
}
impl ::core::marker::Copy for DHCP_CLIENT_INFO_V4 {}
impl ::core::clone::Clone for DHCP_CLIENT_INFO_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_V4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_V4").field("ClientIpAddress", &self.ClientIpAddress).field("SubnetMask", &self.SubnetMask).field("ClientHardwareAddress", &self.ClientHardwareAddress).field("ClientName", &self.ClientName).field("ClientComment", &self.ClientComment).field("ClientLeaseExpires", &self.ClientLeaseExpires).field("OwnerHost", &self.OwnerHost).field("bClientType", &self.bClientType).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_CLIENT_INFO_V4 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_V4 {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress && self.SubnetMask == other.SubnetMask && self.ClientHardwareAddress == other.ClientHardwareAddress && self.ClientName == other.ClientName && self.ClientComment == other.ClientComment && self.ClientLeaseExpires == other.ClientLeaseExpires && self.OwnerHost == other.OwnerHost && self.bClientType == other.bClientType
    }
}
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_V4 {}
impl ::core::default::Default for DHCP_CLIENT_INFO_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_CLIENT_INFO_V5 {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: ::windows::core::PWSTR,
    pub ClientComment: ::windows::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
}
impl ::core::marker::Copy for DHCP_CLIENT_INFO_V5 {}
impl ::core::clone::Clone for DHCP_CLIENT_INFO_V5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_V5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_V5").field("ClientIpAddress", &self.ClientIpAddress).field("SubnetMask", &self.SubnetMask).field("ClientHardwareAddress", &self.ClientHardwareAddress).field("ClientName", &self.ClientName).field("ClientComment", &self.ClientComment).field("ClientLeaseExpires", &self.ClientLeaseExpires).field("OwnerHost", &self.OwnerHost).field("bClientType", &self.bClientType).field("AddressState", &self.AddressState).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_CLIENT_INFO_V5 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_V5 {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress && self.SubnetMask == other.SubnetMask && self.ClientHardwareAddress == other.ClientHardwareAddress && self.ClientName == other.ClientName && self.ClientComment == other.ClientComment && self.ClientLeaseExpires == other.ClientLeaseExpires && self.OwnerHost == other.OwnerHost && self.bClientType == other.bClientType && self.AddressState == other.AddressState
    }
}
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_V5 {}
impl ::core::default::Default for DHCP_CLIENT_INFO_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_CLIENT_INFO_V6 {
    pub ClientIpAddress: DHCP_IPV6_ADDRESS,
    pub ClientDUID: DHCP_BINARY_DATA,
    pub AddressType: u32,
    pub IAID: u32,
    pub ClientName: ::windows::core::PWSTR,
    pub ClientComment: ::windows::core::PWSTR,
    pub ClientValidLeaseExpires: DATE_TIME,
    pub ClientPrefLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO_V6,
}
impl ::core::marker::Copy for DHCP_CLIENT_INFO_V6 {}
impl ::core::clone::Clone for DHCP_CLIENT_INFO_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_V6").field("ClientIpAddress", &self.ClientIpAddress).field("ClientDUID", &self.ClientDUID).field("AddressType", &self.AddressType).field("IAID", &self.IAID).field("ClientName", &self.ClientName).field("ClientComment", &self.ClientComment).field("ClientValidLeaseExpires", &self.ClientValidLeaseExpires).field("ClientPrefLeaseExpires", &self.ClientPrefLeaseExpires).field("OwnerHost", &self.OwnerHost).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_CLIENT_INFO_V6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress && self.ClientDUID == other.ClientDUID && self.AddressType == other.AddressType && self.IAID == other.IAID && self.ClientName == other.ClientName && self.ClientComment == other.ClientComment && self.ClientValidLeaseExpires == other.ClientValidLeaseExpires && self.ClientPrefLeaseExpires == other.ClientPrefLeaseExpires && self.OwnerHost == other.OwnerHost
    }
}
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_V6 {}
impl ::core::default::Default for DHCP_CLIENT_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_CLIENT_INFO_VQ {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: ::windows::core::PWSTR,
    pub ClientComment: ::windows::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_CLIENT_INFO_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_CLIENT_INFO_VQ {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_VQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_VQ")
            .field("ClientIpAddress", &self.ClientIpAddress)
            .field("SubnetMask", &self.SubnetMask)
            .field("ClientHardwareAddress", &self.ClientHardwareAddress)
            .field("ClientName", &self.ClientName)
            .field("ClientComment", &self.ClientComment)
            .field("ClientLeaseExpires", &self.ClientLeaseExpires)
            .field("OwnerHost", &self.OwnerHost)
            .field("bClientType", &self.bClientType)
            .field("AddressState", &self.AddressState)
            .field("Status", &self.Status)
            .field("ProbationEnds", &self.ProbationEnds)
            .field("QuarantineCapable", &self.QuarantineCapable)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_CLIENT_INFO_VQ {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_VQ {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress && self.SubnetMask == other.SubnetMask && self.ClientHardwareAddress == other.ClientHardwareAddress && self.ClientName == other.ClientName && self.ClientComment == other.ClientComment && self.ClientLeaseExpires == other.ClientLeaseExpires && self.OwnerHost == other.OwnerHost && self.bClientType == other.bClientType && self.AddressState == other.AddressState && self.Status == other.Status && self.ProbationEnds == other.ProbationEnds && self.QuarantineCapable == other.QuarantineCapable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DHCP_CLIENT_SEARCH_UNION(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_FAILOVER_RELATIONSHIP {
    pub PrimaryServer: u32,
    pub SecondaryServer: u32,
    pub Mode: DHCP_FAILOVER_MODE,
    pub ServerType: DHCP_FAILOVER_SERVER,
    pub State: FSM_STATE,
    pub PrevState: FSM_STATE,
    pub Mclt: u32,
    pub SafePeriod: u32,
    pub RelationshipName: ::windows::core::PWSTR,
    pub PrimaryServerName: ::windows::core::PWSTR,
    pub SecondaryServerName: ::windows::core::PWSTR,
    pub pScopes: *mut DHCP_IP_ARRAY,
    pub Percentage: u8,
    pub SharedSecret: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DHCP_FAILOVER_RELATIONSHIP {}
impl ::core::clone::Clone for DHCP_FAILOVER_RELATIONSHIP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_FAILOVER_RELATIONSHIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_FAILOVER_RELATIONSHIP")
            .field("PrimaryServer", &self.PrimaryServer)
            .field("SecondaryServer", &self.SecondaryServer)
            .field("Mode", &self.Mode)
            .field("ServerType", &self.ServerType)
            .field("State", &self.State)
            .field("PrevState", &self.PrevState)
            .field("Mclt", &self.Mclt)
            .field("SafePeriod", &self.SafePeriod)
            .field("RelationshipName", &self.RelationshipName)
            .field("PrimaryServerName", &self.PrimaryServerName)
            .field("SecondaryServerName", &self.SecondaryServerName)
            .field("pScopes", &self.pScopes)
            .field("Percentage", &self.Percentage)
            .field("SharedSecret", &self.SharedSecret)
            .finish()
    }
}
impl ::windows::core::TypeKind for DHCP_FAILOVER_RELATIONSHIP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_FAILOVER_RELATIONSHIP {
    fn eq(&self, other: &Self) -> bool {
        self.PrimaryServer == other.PrimaryServer && self.SecondaryServer == other.SecondaryServer && self.Mode == other.Mode && self.ServerType == other.ServerType && self.State == other.State && self.PrevState == other.PrevState && self.Mclt == other.Mclt && self.SafePeriod == other.SafePeriod && self.RelationshipName == other.RelationshipName && self.PrimaryServerName == other.PrimaryServerName && self.SecondaryServerName == other.SecondaryServerName && self.pScopes == other.pScopes && self.Percentage == other.Percentage && self.SharedSecret == other.SharedSecret
    }
}
impl ::core::cmp::Eq for DHCP_FAILOVER_RELATIONSHIP {}
impl ::core::default::Default for DHCP_FAILOVER_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    pub NumElements: u32,
    pub pRelationships: *mut DHCP_FAILOVER_RELATIONSHIP,
}
impl ::core::marker::Copy for DHCP_FAILOVER_RELATIONSHIP_ARRAY {}
impl ::core::clone::Clone for DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_FAILOVER_RELATIONSHIP_ARRAY").field("NumElements", &self.NumElements).field("pRelationships", &self.pRelationships).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.pRelationships == other.pRelationships
    }
}
impl ::core::cmp::Eq for DHCP_FAILOVER_RELATIONSHIP_ARRAY {}
impl ::core::default::Default for DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_FAILOVER_STATISTICS {
    pub NumAddr: u32,
    pub AddrFree: u32,
    pub AddrInUse: u32,
    pub PartnerAddrFree: u32,
    pub ThisAddrFree: u32,
    pub PartnerAddrInUse: u32,
    pub ThisAddrInUse: u32,
}
impl ::core::marker::Copy for DHCP_FAILOVER_STATISTICS {}
impl ::core::clone::Clone for DHCP_FAILOVER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_FAILOVER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_FAILOVER_STATISTICS").field("NumAddr", &self.NumAddr).field("AddrFree", &self.AddrFree).field("AddrInUse", &self.AddrInUse).field("PartnerAddrFree", &self.PartnerAddrFree).field("ThisAddrFree", &self.ThisAddrFree).field("PartnerAddrInUse", &self.PartnerAddrInUse).field("ThisAddrInUse", &self.ThisAddrInUse).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_FAILOVER_STATISTICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_FAILOVER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.NumAddr == other.NumAddr && self.AddrFree == other.AddrFree && self.AddrInUse == other.AddrInUse && self.PartnerAddrFree == other.PartnerAddrFree && self.ThisAddrFree == other.ThisAddrFree && self.PartnerAddrInUse == other.PartnerAddrInUse && self.ThisAddrInUse == other.ThisAddrInUse
    }
}
impl ::core::cmp::Eq for DHCP_FAILOVER_STATISTICS {}
impl ::core::default::Default for DHCP_FAILOVER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_FILTER_ADD_INFO {
    pub AddrPatt: DHCP_ADDR_PATTERN,
    pub Comment: ::windows::core::PWSTR,
    pub ListType: DHCP_FILTER_LIST_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_FILTER_ADD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_FILTER_ADD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_FILTER_ADD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_FILTER_ADD_INFO").field("AddrPatt", &self.AddrPatt).field("Comment", &self.Comment).field("ListType", &self.ListType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_FILTER_ADD_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_FILTER_ADD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AddrPatt == other.AddrPatt && self.Comment == other.Comment && self.ListType == other.ListType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_FILTER_ADD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_FILTER_ADD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_FILTER_ENUM_INFO {
    pub NumElements: u32,
    pub pEnumRecords: *mut DHCP_FILTER_RECORD,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_FILTER_ENUM_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_FILTER_ENUM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_FILTER_ENUM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_FILTER_ENUM_INFO").field("NumElements", &self.NumElements).field("pEnumRecords", &self.pEnumRecords).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_FILTER_ENUM_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_FILTER_ENUM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.pEnumRecords == other.pEnumRecords
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_FILTER_ENUM_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_FILTER_ENUM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_FILTER_GLOBAL_INFO {
    pub EnforceAllowList: super::super::Foundation::BOOL,
    pub EnforceDenyList: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_FILTER_GLOBAL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_FILTER_GLOBAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_FILTER_GLOBAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_FILTER_GLOBAL_INFO").field("EnforceAllowList", &self.EnforceAllowList).field("EnforceDenyList", &self.EnforceDenyList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_FILTER_GLOBAL_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_FILTER_GLOBAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EnforceAllowList == other.EnforceAllowList && self.EnforceDenyList == other.EnforceDenyList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_FILTER_GLOBAL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_FILTER_GLOBAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_FILTER_RECORD {
    pub AddrPatt: DHCP_ADDR_PATTERN,
    pub Comment: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_FILTER_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_FILTER_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_FILTER_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_FILTER_RECORD").field("AddrPatt", &self.AddrPatt).field("Comment", &self.Comment).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_FILTER_RECORD {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_FILTER_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.AddrPatt == other.AddrPatt && self.Comment == other.Comment
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_FILTER_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_FILTER_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_HOST_INFO {
    pub IpAddress: u32,
    pub NetBiosName: ::windows::core::PWSTR,
    pub HostName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DHCP_HOST_INFO {}
impl ::core::clone::Clone for DHCP_HOST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_HOST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_HOST_INFO").field("IpAddress", &self.IpAddress).field("NetBiosName", &self.NetBiosName).field("HostName", &self.HostName).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_HOST_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_HOST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.IpAddress == other.IpAddress && self.NetBiosName == other.NetBiosName && self.HostName == other.HostName
    }
}
impl ::core::cmp::Eq for DHCP_HOST_INFO {}
impl ::core::default::Default for DHCP_HOST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_HOST_INFO_V6 {
    pub IpAddress: DHCP_IPV6_ADDRESS,
    pub NetBiosName: ::windows::core::PWSTR,
    pub HostName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DHCP_HOST_INFO_V6 {}
impl ::core::clone::Clone for DHCP_HOST_INFO_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_HOST_INFO_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_HOST_INFO_V6").field("IpAddress", &self.IpAddress).field("NetBiosName", &self.NetBiosName).field("HostName", &self.HostName).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_HOST_INFO_V6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_HOST_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.IpAddress == other.IpAddress && self.NetBiosName == other.NetBiosName && self.HostName == other.HostName
    }
}
impl ::core::cmp::Eq for DHCP_HOST_INFO_V6 {}
impl ::core::default::Default for DHCP_HOST_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_IPV6_ADDRESS {
    pub HighOrderBits: u64,
    pub LowOrderBits: u64,
}
impl ::core::marker::Copy for DHCP_IPV6_ADDRESS {}
impl ::core::clone::Clone for DHCP_IPV6_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_IPV6_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IPV6_ADDRESS").field("HighOrderBits", &self.HighOrderBits).field("LowOrderBits", &self.LowOrderBits).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_IPV6_ADDRESS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_IPV6_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.HighOrderBits == other.HighOrderBits && self.LowOrderBits == other.LowOrderBits
    }
}
impl ::core::cmp::Eq for DHCP_IPV6_ADDRESS {}
impl ::core::default::Default for DHCP_IPV6_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_IP_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut u32,
}
impl ::core::marker::Copy for DHCP_IP_ARRAY {}
impl ::core::clone::Clone for DHCP_IP_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_IP_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_IP_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_IP_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_IP_ARRAY {}
impl ::core::default::Default for DHCP_IP_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_IP_CLUSTER {
    pub ClusterAddress: u32,
    pub ClusterMask: u32,
}
impl ::core::marker::Copy for DHCP_IP_CLUSTER {}
impl ::core::clone::Clone for DHCP_IP_CLUSTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_IP_CLUSTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_CLUSTER").field("ClusterAddress", &self.ClusterAddress).field("ClusterMask", &self.ClusterMask).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_IP_CLUSTER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_IP_CLUSTER {
    fn eq(&self, other: &Self) -> bool {
        self.ClusterAddress == other.ClusterAddress && self.ClusterMask == other.ClusterMask
    }
}
impl ::core::cmp::Eq for DHCP_IP_CLUSTER {}
impl ::core::default::Default for DHCP_IP_CLUSTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_IP_RANGE {
    pub StartAddress: u32,
    pub EndAddress: u32,
}
impl ::core::marker::Copy for DHCP_IP_RANGE {}
impl ::core::clone::Clone for DHCP_IP_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_IP_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_RANGE").field("StartAddress", &self.StartAddress).field("EndAddress", &self.EndAddress).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_IP_RANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_IP_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartAddress == other.StartAddress && self.EndAddress == other.EndAddress
    }
}
impl ::core::cmp::Eq for DHCP_IP_RANGE {}
impl ::core::default::Default for DHCP_IP_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_IP_RANGE_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_IP_RANGE,
}
impl ::core::marker::Copy for DHCP_IP_RANGE_ARRAY {}
impl ::core::clone::Clone for DHCP_IP_RANGE_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_IP_RANGE_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_RANGE_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_IP_RANGE_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_IP_RANGE_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_IP_RANGE_ARRAY {}
impl ::core::default::Default for DHCP_IP_RANGE_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_IP_RANGE_V6 {
    pub StartAddress: DHCP_IPV6_ADDRESS,
    pub EndAddress: DHCP_IPV6_ADDRESS,
}
impl ::core::marker::Copy for DHCP_IP_RANGE_V6 {}
impl ::core::clone::Clone for DHCP_IP_RANGE_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_IP_RANGE_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_RANGE_V6").field("StartAddress", &self.StartAddress).field("EndAddress", &self.EndAddress).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_IP_RANGE_V6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_IP_RANGE_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.StartAddress == other.StartAddress && self.EndAddress == other.EndAddress
    }
}
impl ::core::cmp::Eq for DHCP_IP_RANGE_V6 {}
impl ::core::default::Default for DHCP_IP_RANGE_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_IP_RESERVATION {
    pub ReservedIpAddress: u32,
    pub ReservedForClient: *mut DHCP_BINARY_DATA,
}
impl ::core::marker::Copy for DHCP_IP_RESERVATION {}
impl ::core::clone::Clone for DHCP_IP_RESERVATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_IP_RESERVATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_RESERVATION").field("ReservedIpAddress", &self.ReservedIpAddress).field("ReservedForClient", &self.ReservedForClient).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_IP_RESERVATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_IP_RESERVATION {
    fn eq(&self, other: &Self) -> bool {
        self.ReservedIpAddress == other.ReservedIpAddress && self.ReservedForClient == other.ReservedForClient
    }
}
impl ::core::cmp::Eq for DHCP_IP_RESERVATION {}
impl ::core::default::Default for DHCP_IP_RESERVATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_IP_RESERVATION_INFO {
    pub ReservedIpAddress: u32,
    pub ReservedForClient: DHCP_BINARY_DATA,
    pub ReservedClientName: ::windows::core::PWSTR,
    pub ReservedClientDesc: ::windows::core::PWSTR,
    pub bAllowedClientTypes: u8,
    pub fOptionsPresent: u8,
}
impl ::core::marker::Copy for DHCP_IP_RESERVATION_INFO {}
impl ::core::clone::Clone for DHCP_IP_RESERVATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_IP_RESERVATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_RESERVATION_INFO").field("ReservedIpAddress", &self.ReservedIpAddress).field("ReservedForClient", &self.ReservedForClient).field("ReservedClientName", &self.ReservedClientName).field("ReservedClientDesc", &self.ReservedClientDesc).field("bAllowedClientTypes", &self.bAllowedClientTypes).field("fOptionsPresent", &self.fOptionsPresent).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_IP_RESERVATION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_IP_RESERVATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ReservedIpAddress == other.ReservedIpAddress && self.ReservedForClient == other.ReservedForClient && self.ReservedClientName == other.ReservedClientName && self.ReservedClientDesc == other.ReservedClientDesc && self.bAllowedClientTypes == other.bAllowedClientTypes && self.fOptionsPresent == other.fOptionsPresent
    }
}
impl ::core::cmp::Eq for DHCP_IP_RESERVATION_INFO {}
impl ::core::default::Default for DHCP_IP_RESERVATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_IP_RESERVATION_V4 {
    pub ReservedIpAddress: u32,
    pub ReservedForClient: *mut DHCP_BINARY_DATA,
    pub bAllowedClientTypes: u8,
}
impl ::core::marker::Copy for DHCP_IP_RESERVATION_V4 {}
impl ::core::clone::Clone for DHCP_IP_RESERVATION_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_IP_RESERVATION_V4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_RESERVATION_V4").field("ReservedIpAddress", &self.ReservedIpAddress).field("ReservedForClient", &self.ReservedForClient).field("bAllowedClientTypes", &self.bAllowedClientTypes).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_IP_RESERVATION_V4 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_IP_RESERVATION_V4 {
    fn eq(&self, other: &Self) -> bool {
        self.ReservedIpAddress == other.ReservedIpAddress && self.ReservedForClient == other.ReservedForClient && self.bAllowedClientTypes == other.bAllowedClientTypes
    }
}
impl ::core::cmp::Eq for DHCP_IP_RESERVATION_V4 {}
impl ::core::default::Default for DHCP_IP_RESERVATION_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_IP_RESERVATION_V6 {
    pub ReservedIpAddress: DHCP_IPV6_ADDRESS,
    pub ReservedForClient: *mut DHCP_BINARY_DATA,
    pub InterfaceId: u32,
}
impl ::core::marker::Copy for DHCP_IP_RESERVATION_V6 {}
impl ::core::clone::Clone for DHCP_IP_RESERVATION_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_IP_RESERVATION_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_RESERVATION_V6").field("ReservedIpAddress", &self.ReservedIpAddress).field("ReservedForClient", &self.ReservedForClient).field("InterfaceId", &self.InterfaceId).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_IP_RESERVATION_V6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_IP_RESERVATION_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.ReservedIpAddress == other.ReservedIpAddress && self.ReservedForClient == other.ReservedForClient && self.InterfaceId == other.InterfaceId
    }
}
impl ::core::cmp::Eq for DHCP_IP_RESERVATION_V6 {}
impl ::core::default::Default for DHCP_IP_RESERVATION_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_MIB_INFO {
    pub Discovers: u32,
    pub Offers: u32,
    pub Requests: u32,
    pub Acks: u32,
    pub Naks: u32,
    pub Declines: u32,
    pub Releases: u32,
    pub ServerStartTime: DATE_TIME,
    pub Scopes: u32,
    pub ScopeInfo: *mut SCOPE_MIB_INFO,
}
impl ::core::marker::Copy for DHCP_MIB_INFO {}
impl ::core::clone::Clone for DHCP_MIB_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_MIB_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_MIB_INFO").field("Discovers", &self.Discovers).field("Offers", &self.Offers).field("Requests", &self.Requests).field("Acks", &self.Acks).field("Naks", &self.Naks).field("Declines", &self.Declines).field("Releases", &self.Releases).field("ServerStartTime", &self.ServerStartTime).field("Scopes", &self.Scopes).field("ScopeInfo", &self.ScopeInfo).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_MIB_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_MIB_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Discovers == other.Discovers && self.Offers == other.Offers && self.Requests == other.Requests && self.Acks == other.Acks && self.Naks == other.Naks && self.Declines == other.Declines && self.Releases == other.Releases && self.ServerStartTime == other.ServerStartTime && self.Scopes == other.Scopes && self.ScopeInfo == other.ScopeInfo
    }
}
impl ::core::cmp::Eq for DHCP_MIB_INFO {}
impl ::core::default::Default for DHCP_MIB_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_MIB_INFO_V5 {
    pub Discovers: u32,
    pub Offers: u32,
    pub Requests: u32,
    pub Acks: u32,
    pub Naks: u32,
    pub Declines: u32,
    pub Releases: u32,
    pub ServerStartTime: DATE_TIME,
    pub QtnNumLeases: u32,
    pub QtnPctQtnLeases: u32,
    pub QtnProbationLeases: u32,
    pub QtnNonQtnLeases: u32,
    pub QtnExemptLeases: u32,
    pub QtnCapableClients: u32,
    pub QtnIASErrors: u32,
    pub DelayedOffers: u32,
    pub ScopesWithDelayedOffers: u32,
    pub Scopes: u32,
    pub ScopeInfo: *mut SCOPE_MIB_INFO_V5,
}
impl ::core::marker::Copy for DHCP_MIB_INFO_V5 {}
impl ::core::clone::Clone for DHCP_MIB_INFO_V5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_MIB_INFO_V5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_MIB_INFO_V5")
            .field("Discovers", &self.Discovers)
            .field("Offers", &self.Offers)
            .field("Requests", &self.Requests)
            .field("Acks", &self.Acks)
            .field("Naks", &self.Naks)
            .field("Declines", &self.Declines)
            .field("Releases", &self.Releases)
            .field("ServerStartTime", &self.ServerStartTime)
            .field("QtnNumLeases", &self.QtnNumLeases)
            .field("QtnPctQtnLeases", &self.QtnPctQtnLeases)
            .field("QtnProbationLeases", &self.QtnProbationLeases)
            .field("QtnNonQtnLeases", &self.QtnNonQtnLeases)
            .field("QtnExemptLeases", &self.QtnExemptLeases)
            .field("QtnCapableClients", &self.QtnCapableClients)
            .field("QtnIASErrors", &self.QtnIASErrors)
            .field("DelayedOffers", &self.DelayedOffers)
            .field("ScopesWithDelayedOffers", &self.ScopesWithDelayedOffers)
            .field("Scopes", &self.Scopes)
            .field("ScopeInfo", &self.ScopeInfo)
            .finish()
    }
}
impl ::windows::core::TypeKind for DHCP_MIB_INFO_V5 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_MIB_INFO_V5 {
    fn eq(&self, other: &Self) -> bool {
        self.Discovers == other.Discovers && self.Offers == other.Offers && self.Requests == other.Requests && self.Acks == other.Acks && self.Naks == other.Naks && self.Declines == other.Declines && self.Releases == other.Releases && self.ServerStartTime == other.ServerStartTime && self.QtnNumLeases == other.QtnNumLeases && self.QtnPctQtnLeases == other.QtnPctQtnLeases && self.QtnProbationLeases == other.QtnProbationLeases && self.QtnNonQtnLeases == other.QtnNonQtnLeases && self.QtnExemptLeases == other.QtnExemptLeases && self.QtnCapableClients == other.QtnCapableClients && self.QtnIASErrors == other.QtnIASErrors && self.DelayedOffers == other.DelayedOffers && self.ScopesWithDelayedOffers == other.ScopesWithDelayedOffers && self.Scopes == other.Scopes && self.ScopeInfo == other.ScopeInfo
    }
}
impl ::core::cmp::Eq for DHCP_MIB_INFO_V5 {}
impl ::core::default::Default for DHCP_MIB_INFO_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_MIB_INFO_V6 {
    pub Solicits: u32,
    pub Advertises: u32,
    pub Requests: u32,
    pub Renews: u32,
    pub Rebinds: u32,
    pub Replies: u32,
    pub Confirms: u32,
    pub Declines: u32,
    pub Releases: u32,
    pub Informs: u32,
    pub ServerStartTime: DATE_TIME,
    pub Scopes: u32,
    pub ScopeInfo: *mut SCOPE_MIB_INFO_V6,
}
impl ::core::marker::Copy for DHCP_MIB_INFO_V6 {}
impl ::core::clone::Clone for DHCP_MIB_INFO_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_MIB_INFO_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_MIB_INFO_V6")
            .field("Solicits", &self.Solicits)
            .field("Advertises", &self.Advertises)
            .field("Requests", &self.Requests)
            .field("Renews", &self.Renews)
            .field("Rebinds", &self.Rebinds)
            .field("Replies", &self.Replies)
            .field("Confirms", &self.Confirms)
            .field("Declines", &self.Declines)
            .field("Releases", &self.Releases)
            .field("Informs", &self.Informs)
            .field("ServerStartTime", &self.ServerStartTime)
            .field("Scopes", &self.Scopes)
            .field("ScopeInfo", &self.ScopeInfo)
            .finish()
    }
}
impl ::windows::core::TypeKind for DHCP_MIB_INFO_V6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_MIB_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.Solicits == other.Solicits && self.Advertises == other.Advertises && self.Requests == other.Requests && self.Renews == other.Renews && self.Rebinds == other.Rebinds && self.Replies == other.Replies && self.Confirms == other.Confirms && self.Declines == other.Declines && self.Releases == other.Releases && self.Informs == other.Informs && self.ServerStartTime == other.ServerStartTime && self.Scopes == other.Scopes && self.ScopeInfo == other.ScopeInfo
    }
}
impl ::core::cmp::Eq for DHCP_MIB_INFO_V6 {}
impl ::core::default::Default for DHCP_MIB_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_MIB_INFO_VQ {
    pub Discovers: u32,
    pub Offers: u32,
    pub Requests: u32,
    pub Acks: u32,
    pub Naks: u32,
    pub Declines: u32,
    pub Releases: u32,
    pub ServerStartTime: DATE_TIME,
    pub QtnNumLeases: u32,
    pub QtnPctQtnLeases: u32,
    pub QtnProbationLeases: u32,
    pub QtnNonQtnLeases: u32,
    pub QtnExemptLeases: u32,
    pub QtnCapableClients: u32,
    pub QtnIASErrors: u32,
    pub Scopes: u32,
    pub ScopeInfo: *mut SCOPE_MIB_INFO_VQ,
}
impl ::core::marker::Copy for DHCP_MIB_INFO_VQ {}
impl ::core::clone::Clone for DHCP_MIB_INFO_VQ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_MIB_INFO_VQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_MIB_INFO_VQ")
            .field("Discovers", &self.Discovers)
            .field("Offers", &self.Offers)
            .field("Requests", &self.Requests)
            .field("Acks", &self.Acks)
            .field("Naks", &self.Naks)
            .field("Declines", &self.Declines)
            .field("Releases", &self.Releases)
            .field("ServerStartTime", &self.ServerStartTime)
            .field("QtnNumLeases", &self.QtnNumLeases)
            .field("QtnPctQtnLeases", &self.QtnPctQtnLeases)
            .field("QtnProbationLeases", &self.QtnProbationLeases)
            .field("QtnNonQtnLeases", &self.QtnNonQtnLeases)
            .field("QtnExemptLeases", &self.QtnExemptLeases)
            .field("QtnCapableClients", &self.QtnCapableClients)
            .field("QtnIASErrors", &self.QtnIASErrors)
            .field("Scopes", &self.Scopes)
            .field("ScopeInfo", &self.ScopeInfo)
            .finish()
    }
}
impl ::windows::core::TypeKind for DHCP_MIB_INFO_VQ {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_MIB_INFO_VQ {
    fn eq(&self, other: &Self) -> bool {
        self.Discovers == other.Discovers && self.Offers == other.Offers && self.Requests == other.Requests && self.Acks == other.Acks && self.Naks == other.Naks && self.Declines == other.Declines && self.Releases == other.Releases && self.ServerStartTime == other.ServerStartTime && self.QtnNumLeases == other.QtnNumLeases && self.QtnPctQtnLeases == other.QtnPctQtnLeases && self.QtnProbationLeases == other.QtnProbationLeases && self.QtnNonQtnLeases == other.QtnNonQtnLeases && self.QtnExemptLeases == other.QtnExemptLeases && self.QtnCapableClients == other.QtnCapableClients && self.QtnIASErrors == other.QtnIASErrors && self.Scopes == other.Scopes && self.ScopeInfo == other.ScopeInfo
    }
}
impl ::core::cmp::Eq for DHCP_MIB_INFO_VQ {}
impl ::core::default::Default for DHCP_MIB_INFO_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_OPTION {
    pub OptionID: u32,
    pub OptionName: ::windows::core::PWSTR,
    pub OptionComment: ::windows::core::PWSTR,
    pub DefaultValue: DHCP_OPTION_DATA,
    pub OptionType: DHCP_OPTION_TYPE,
}
impl ::core::marker::Copy for DHCP_OPTION {}
impl ::core::clone::Clone for DHCP_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_OPTION").field("OptionID", &self.OptionID).field("OptionName", &self.OptionName).field("OptionComment", &self.OptionComment).field("DefaultValue", &self.DefaultValue).field("OptionType", &self.OptionType).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_OPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_OPTION {
    fn eq(&self, other: &Self) -> bool {
        self.OptionID == other.OptionID && self.OptionName == other.OptionName && self.OptionComment == other.OptionComment && self.DefaultValue == other.DefaultValue && self.OptionType == other.OptionType
    }
}
impl ::core::cmp::Eq for DHCP_OPTION {}
impl ::core::default::Default for DHCP_OPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_OPTION_ARRAY {
    pub NumElements: u32,
    pub Options: *mut DHCP_OPTION,
}
impl ::core::marker::Copy for DHCP_OPTION_ARRAY {}
impl ::core::clone::Clone for DHCP_OPTION_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_OPTION_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_OPTION_ARRAY").field("NumElements", &self.NumElements).field("Options", &self.Options).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_OPTION_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_OPTION_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Options == other.Options
    }
}
impl ::core::cmp::Eq for DHCP_OPTION_ARRAY {}
impl ::core::default::Default for DHCP_OPTION_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_OPTION_DATA {
    pub NumElements: u32,
    pub Elements: *mut DHCP_OPTION_DATA_ELEMENT,
}
impl ::core::marker::Copy for DHCP_OPTION_DATA {}
impl ::core::clone::Clone for DHCP_OPTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_OPTION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_OPTION_DATA").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_OPTION_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_OPTION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_OPTION_DATA {}
impl ::core::default::Default for DHCP_OPTION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_OPTION_DATA_ELEMENT {
    pub OptionType: DHCP_OPTION_DATA_TYPE,
    pub Element: DHCP_OPTION_DATA_ELEMENT_0,
}
impl ::core::marker::Copy for DHCP_OPTION_DATA_ELEMENT {}
impl ::core::clone::Clone for DHCP_OPTION_DATA_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_OPTION_DATA_ELEMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_OPTION_DATA_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub union DHCP_OPTION_DATA_ELEMENT_0 {
    pub ByteOption: u8,
    pub WordOption: u16,
    pub DWordOption: u32,
    pub DWordDWordOption: DWORD_DWORD,
    pub IpAddressOption: u32,
    pub StringDataOption: ::windows::core::PWSTR,
    pub BinaryDataOption: DHCP_BINARY_DATA,
    pub EncapsulatedDataOption: DHCP_BINARY_DATA,
    pub Ipv6AddressDataOption: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DHCP_OPTION_DATA_ELEMENT_0 {}
impl ::core::clone::Clone for DHCP_OPTION_DATA_ELEMENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_OPTION_DATA_ELEMENT_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_OPTION_DATA_ELEMENT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DHCP_OPTION_ELEMENT_UNION(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_OPTION_LIST {
    pub NumOptions: u32,
    pub Options: *mut DHCP_OPTION_VALUE,
}
impl ::core::marker::Copy for DHCP_OPTION_LIST {}
impl ::core::clone::Clone for DHCP_OPTION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_OPTION_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_OPTION_LIST").field("NumOptions", &self.NumOptions).field("Options", &self.Options).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_OPTION_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_OPTION_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumOptions == other.NumOptions && self.Options == other.Options
    }
}
impl ::core::cmp::Eq for DHCP_OPTION_LIST {}
impl ::core::default::Default for DHCP_OPTION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_OPTION_SCOPE_INFO {
    pub ScopeType: DHCP_OPTION_SCOPE_TYPE,
    pub ScopeInfo: DHCP_OPTION_SCOPE_INFO_0,
}
impl ::core::marker::Copy for DHCP_OPTION_SCOPE_INFO {}
impl ::core::clone::Clone for DHCP_OPTION_SCOPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_OPTION_SCOPE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_OPTION_SCOPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub union DHCP_OPTION_SCOPE_INFO_0 {
    pub DefaultScopeInfo: *mut ::core::ffi::c_void,
    pub GlobalScopeInfo: *mut ::core::ffi::c_void,
    pub SubnetScopeInfo: u32,
    pub ReservedScopeInfo: DHCP_RESERVED_SCOPE,
    pub MScopeInfo: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DHCP_OPTION_SCOPE_INFO_0 {}
impl ::core::clone::Clone for DHCP_OPTION_SCOPE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_OPTION_SCOPE_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_OPTION_SCOPE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_OPTION_SCOPE_INFO6 {
    pub ScopeType: DHCP_OPTION_SCOPE_TYPE6,
    pub ScopeInfo: DHCP_OPTION_SCOPE_INFO6_0,
}
impl ::core::marker::Copy for DHCP_OPTION_SCOPE_INFO6 {}
impl ::core::clone::Clone for DHCP_OPTION_SCOPE_INFO6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_OPTION_SCOPE_INFO6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_OPTION_SCOPE_INFO6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub union DHCP_OPTION_SCOPE_INFO6_0 {
    pub DefaultScopeInfo: *mut ::core::ffi::c_void,
    pub SubnetScopeInfo: DHCP_IPV6_ADDRESS,
    pub ReservedScopeInfo: DHCP_RESERVED_SCOPE6,
}
impl ::core::marker::Copy for DHCP_OPTION_SCOPE_INFO6_0 {}
impl ::core::clone::Clone for DHCP_OPTION_SCOPE_INFO6_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_OPTION_SCOPE_INFO6_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_OPTION_SCOPE_INFO6_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DHCP_OPTION_SCOPE_UNION6(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_OPTION_VALUE {
    pub OptionID: u32,
    pub Value: DHCP_OPTION_DATA,
}
impl ::core::marker::Copy for DHCP_OPTION_VALUE {}
impl ::core::clone::Clone for DHCP_OPTION_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_OPTION_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_OPTION_VALUE").field("OptionID", &self.OptionID).field("Value", &self.Value).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_OPTION_VALUE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_OPTION_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.OptionID == other.OptionID && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for DHCP_OPTION_VALUE {}
impl ::core::default::Default for DHCP_OPTION_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_OPTION_VALUE_ARRAY {
    pub NumElements: u32,
    pub Values: *mut DHCP_OPTION_VALUE,
}
impl ::core::marker::Copy for DHCP_OPTION_VALUE_ARRAY {}
impl ::core::clone::Clone for DHCP_OPTION_VALUE_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_OPTION_VALUE_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_OPTION_VALUE_ARRAY").field("NumElements", &self.NumElements).field("Values", &self.Values).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_OPTION_VALUE_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_OPTION_VALUE_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Values == other.Values
    }
}
impl ::core::cmp::Eq for DHCP_OPTION_VALUE_ARRAY {}
impl ::core::default::Default for DHCP_OPTION_VALUE_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_PERF_STATS {
    pub dwNumPacketsReceived: u32,
    pub dwNumPacketsDuplicate: u32,
    pub dwNumPacketsExpired: u32,
    pub dwNumMilliSecondsProcessed: u32,
    pub dwNumPacketsInActiveQueue: u32,
    pub dwNumPacketsInPingQueue: u32,
    pub dwNumDiscoversReceived: u32,
    pub dwNumOffersSent: u32,
    pub dwNumRequestsReceived: u32,
    pub dwNumInformsReceived: u32,
    pub dwNumAcksSent: u32,
    pub dwNumNacksSent: u32,
    pub dwNumDeclinesReceived: u32,
    pub dwNumReleasesReceived: u32,
    pub dwNumDelayedOfferInQueue: u32,
    pub dwNumPacketsProcessed: u32,
    pub dwNumPacketsInQuarWaitingQueue: u32,
    pub dwNumPacketsInQuarReadyQueue: u32,
    pub dwNumPacketsInQuarDecisionQueue: u32,
}
impl ::core::marker::Copy for DHCP_PERF_STATS {}
impl ::core::clone::Clone for DHCP_PERF_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_PERF_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_PERF_STATS")
            .field("dwNumPacketsReceived", &self.dwNumPacketsReceived)
            .field("dwNumPacketsDuplicate", &self.dwNumPacketsDuplicate)
            .field("dwNumPacketsExpired", &self.dwNumPacketsExpired)
            .field("dwNumMilliSecondsProcessed", &self.dwNumMilliSecondsProcessed)
            .field("dwNumPacketsInActiveQueue", &self.dwNumPacketsInActiveQueue)
            .field("dwNumPacketsInPingQueue", &self.dwNumPacketsInPingQueue)
            .field("dwNumDiscoversReceived", &self.dwNumDiscoversReceived)
            .field("dwNumOffersSent", &self.dwNumOffersSent)
            .field("dwNumRequestsReceived", &self.dwNumRequestsReceived)
            .field("dwNumInformsReceived", &self.dwNumInformsReceived)
            .field("dwNumAcksSent", &self.dwNumAcksSent)
            .field("dwNumNacksSent", &self.dwNumNacksSent)
            .field("dwNumDeclinesReceived", &self.dwNumDeclinesReceived)
            .field("dwNumReleasesReceived", &self.dwNumReleasesReceived)
            .field("dwNumDelayedOfferInQueue", &self.dwNumDelayedOfferInQueue)
            .field("dwNumPacketsProcessed", &self.dwNumPacketsProcessed)
            .field("dwNumPacketsInQuarWaitingQueue", &self.dwNumPacketsInQuarWaitingQueue)
            .field("dwNumPacketsInQuarReadyQueue", &self.dwNumPacketsInQuarReadyQueue)
            .field("dwNumPacketsInQuarDecisionQueue", &self.dwNumPacketsInQuarDecisionQueue)
            .finish()
    }
}
impl ::windows::core::TypeKind for DHCP_PERF_STATS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_PERF_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPacketsReceived == other.dwNumPacketsReceived
            && self.dwNumPacketsDuplicate == other.dwNumPacketsDuplicate
            && self.dwNumPacketsExpired == other.dwNumPacketsExpired
            && self.dwNumMilliSecondsProcessed == other.dwNumMilliSecondsProcessed
            && self.dwNumPacketsInActiveQueue == other.dwNumPacketsInActiveQueue
            && self.dwNumPacketsInPingQueue == other.dwNumPacketsInPingQueue
            && self.dwNumDiscoversReceived == other.dwNumDiscoversReceived
            && self.dwNumOffersSent == other.dwNumOffersSent
            && self.dwNumRequestsReceived == other.dwNumRequestsReceived
            && self.dwNumInformsReceived == other.dwNumInformsReceived
            && self.dwNumAcksSent == other.dwNumAcksSent
            && self.dwNumNacksSent == other.dwNumNacksSent
            && self.dwNumDeclinesReceived == other.dwNumDeclinesReceived
            && self.dwNumReleasesReceived == other.dwNumReleasesReceived
            && self.dwNumDelayedOfferInQueue == other.dwNumDelayedOfferInQueue
            && self.dwNumPacketsProcessed == other.dwNumPacketsProcessed
            && self.dwNumPacketsInQuarWaitingQueue == other.dwNumPacketsInQuarWaitingQueue
            && self.dwNumPacketsInQuarReadyQueue == other.dwNumPacketsInQuarReadyQueue
            && self.dwNumPacketsInQuarDecisionQueue == other.dwNumPacketsInQuarDecisionQueue
    }
}
impl ::core::cmp::Eq for DHCP_PERF_STATS {}
impl ::core::default::Default for DHCP_PERF_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_POLICY {
    pub PolicyName: ::windows::core::PWSTR,
    pub IsGlobalPolicy: super::super::Foundation::BOOL,
    pub Subnet: u32,
    pub ProcessingOrder: u32,
    pub Conditions: *mut DHCP_POL_COND_ARRAY,
    pub Expressions: *mut DHCP_POL_EXPR_ARRAY,
    pub Ranges: *mut DHCP_IP_RANGE_ARRAY,
    pub Description: ::windows::core::PWSTR,
    pub Enabled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_POLICY").field("PolicyName", &self.PolicyName).field("IsGlobalPolicy", &self.IsGlobalPolicy).field("Subnet", &self.Subnet).field("ProcessingOrder", &self.ProcessingOrder).field("Conditions", &self.Conditions).field("Expressions", &self.Expressions).field("Ranges", &self.Ranges).field("Description", &self.Description).field("Enabled", &self.Enabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_POLICY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.PolicyName == other.PolicyName && self.IsGlobalPolicy == other.IsGlobalPolicy && self.Subnet == other.Subnet && self.ProcessingOrder == other.ProcessingOrder && self.Conditions == other.Conditions && self.Expressions == other.Expressions && self.Ranges == other.Ranges && self.Description == other.Description && self.Enabled == other.Enabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_POLICY_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_POLICY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_POLICY_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_POLICY_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_POLICY_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_POLICY_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_POLICY_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_POLICY_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_POLICY_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_POLICY_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_POLICY_EX {
    pub PolicyName: ::windows::core::PWSTR,
    pub IsGlobalPolicy: super::super::Foundation::BOOL,
    pub Subnet: u32,
    pub ProcessingOrder: u32,
    pub Conditions: *mut DHCP_POL_COND_ARRAY,
    pub Expressions: *mut DHCP_POL_EXPR_ARRAY,
    pub Ranges: *mut DHCP_IP_RANGE_ARRAY,
    pub Description: ::windows::core::PWSTR,
    pub Enabled: super::super::Foundation::BOOL,
    pub Properties: *mut DHCP_PROPERTY_ARRAY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_POLICY_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_POLICY_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_POLICY_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_POLICY_EX").field("PolicyName", &self.PolicyName).field("IsGlobalPolicy", &self.IsGlobalPolicy).field("Subnet", &self.Subnet).field("ProcessingOrder", &self.ProcessingOrder).field("Conditions", &self.Conditions).field("Expressions", &self.Expressions).field("Ranges", &self.Ranges).field("Description", &self.Description).field("Enabled", &self.Enabled).field("Properties", &self.Properties).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_POLICY_EX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_POLICY_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PolicyName == other.PolicyName && self.IsGlobalPolicy == other.IsGlobalPolicy && self.Subnet == other.Subnet && self.ProcessingOrder == other.ProcessingOrder && self.Conditions == other.Conditions && self.Expressions == other.Expressions && self.Ranges == other.Ranges && self.Description == other.Description && self.Enabled == other.Enabled && self.Properties == other.Properties
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_POLICY_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_POLICY_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_POLICY_EX_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_POLICY_EX,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_POLICY_EX_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_POLICY_EX_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_POLICY_EX_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_POLICY_EX_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_POLICY_EX_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_POLICY_EX_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_POLICY_EX_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_POLICY_EX_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_POL_COND {
    pub ParentExpr: u32,
    pub Type: DHCP_POL_ATTR_TYPE,
    pub OptionID: u32,
    pub SubOptionID: u32,
    pub VendorName: ::windows::core::PWSTR,
    pub Operator: DHCP_POL_COMPARATOR,
    pub Value: *mut u8,
    pub ValueLength: u32,
}
impl ::core::marker::Copy for DHCP_POL_COND {}
impl ::core::clone::Clone for DHCP_POL_COND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_POL_COND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_POL_COND").field("ParentExpr", &self.ParentExpr).field("Type", &self.Type).field("OptionID", &self.OptionID).field("SubOptionID", &self.SubOptionID).field("VendorName", &self.VendorName).field("Operator", &self.Operator).field("Value", &self.Value).field("ValueLength", &self.ValueLength).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_POL_COND {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_POL_COND {
    fn eq(&self, other: &Self) -> bool {
        self.ParentExpr == other.ParentExpr && self.Type == other.Type && self.OptionID == other.OptionID && self.SubOptionID == other.SubOptionID && self.VendorName == other.VendorName && self.Operator == other.Operator && self.Value == other.Value && self.ValueLength == other.ValueLength
    }
}
impl ::core::cmp::Eq for DHCP_POL_COND {}
impl ::core::default::Default for DHCP_POL_COND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_POL_COND_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_POL_COND,
}
impl ::core::marker::Copy for DHCP_POL_COND_ARRAY {}
impl ::core::clone::Clone for DHCP_POL_COND_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_POL_COND_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_POL_COND_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_POL_COND_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_POL_COND_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_POL_COND_ARRAY {}
impl ::core::default::Default for DHCP_POL_COND_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_POL_EXPR {
    pub ParentExpr: u32,
    pub Operator: DHCP_POL_LOGIC_OPER,
}
impl ::core::marker::Copy for DHCP_POL_EXPR {}
impl ::core::clone::Clone for DHCP_POL_EXPR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_POL_EXPR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_POL_EXPR").field("ParentExpr", &self.ParentExpr).field("Operator", &self.Operator).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_POL_EXPR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_POL_EXPR {
    fn eq(&self, other: &Self) -> bool {
        self.ParentExpr == other.ParentExpr && self.Operator == other.Operator
    }
}
impl ::core::cmp::Eq for DHCP_POL_EXPR {}
impl ::core::default::Default for DHCP_POL_EXPR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_POL_EXPR_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_POL_EXPR,
}
impl ::core::marker::Copy for DHCP_POL_EXPR_ARRAY {}
impl ::core::clone::Clone for DHCP_POL_EXPR_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_POL_EXPR_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_POL_EXPR_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_POL_EXPR_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_POL_EXPR_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_POL_EXPR_ARRAY {}
impl ::core::default::Default for DHCP_POL_EXPR_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_PROPERTY {
    pub ID: DHCP_PROPERTY_ID,
    pub Type: DHCP_PROPERTY_TYPE,
    pub Value: DHCP_PROPERTY_0,
}
impl ::core::marker::Copy for DHCP_PROPERTY {}
impl ::core::clone::Clone for DHCP_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub union DHCP_PROPERTY_0 {
    pub ByteValue: u8,
    pub WordValue: u16,
    pub DWordValue: u32,
    pub StringValue: ::windows::core::PWSTR,
    pub BinaryValue: DHCP_BINARY_DATA,
}
impl ::core::marker::Copy for DHCP_PROPERTY_0 {}
impl ::core::clone::Clone for DHCP_PROPERTY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_PROPERTY_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_PROPERTY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_PROPERTY_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_PROPERTY,
}
impl ::core::marker::Copy for DHCP_PROPERTY_ARRAY {}
impl ::core::clone::Clone for DHCP_PROPERTY_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_PROPERTY_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_PROPERTY_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_PROPERTY_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_PROPERTY_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_PROPERTY_ARRAY {}
impl ::core::default::Default for DHCP_PROPERTY_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_RESERVATION_INFO_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut *mut DHCP_IP_RESERVATION_INFO,
}
impl ::core::marker::Copy for DHCP_RESERVATION_INFO_ARRAY {}
impl ::core::clone::Clone for DHCP_RESERVATION_INFO_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_RESERVATION_INFO_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_RESERVATION_INFO_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_RESERVATION_INFO_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_RESERVATION_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_RESERVATION_INFO_ARRAY {}
impl ::core::default::Default for DHCP_RESERVATION_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_RESERVED_SCOPE {
    pub ReservedIpAddress: u32,
    pub ReservedIpSubnetAddress: u32,
}
impl ::core::marker::Copy for DHCP_RESERVED_SCOPE {}
impl ::core::clone::Clone for DHCP_RESERVED_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_RESERVED_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_RESERVED_SCOPE").field("ReservedIpAddress", &self.ReservedIpAddress).field("ReservedIpSubnetAddress", &self.ReservedIpSubnetAddress).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_RESERVED_SCOPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_RESERVED_SCOPE {
    fn eq(&self, other: &Self) -> bool {
        self.ReservedIpAddress == other.ReservedIpAddress && self.ReservedIpSubnetAddress == other.ReservedIpSubnetAddress
    }
}
impl ::core::cmp::Eq for DHCP_RESERVED_SCOPE {}
impl ::core::default::Default for DHCP_RESERVED_SCOPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_RESERVED_SCOPE6 {
    pub ReservedIpAddress: DHCP_IPV6_ADDRESS,
    pub ReservedIpSubnetAddress: DHCP_IPV6_ADDRESS,
}
impl ::core::marker::Copy for DHCP_RESERVED_SCOPE6 {}
impl ::core::clone::Clone for DHCP_RESERVED_SCOPE6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_RESERVED_SCOPE6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_RESERVED_SCOPE6").field("ReservedIpAddress", &self.ReservedIpAddress).field("ReservedIpSubnetAddress", &self.ReservedIpSubnetAddress).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_RESERVED_SCOPE6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_RESERVED_SCOPE6 {
    fn eq(&self, other: &Self) -> bool {
        self.ReservedIpAddress == other.ReservedIpAddress && self.ReservedIpSubnetAddress == other.ReservedIpSubnetAddress
    }
}
impl ::core::cmp::Eq for DHCP_RESERVED_SCOPE6 {}
impl ::core::default::Default for DHCP_RESERVED_SCOPE6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SCAN_ITEM {
    pub IpAddress: u32,
    pub ScanFlag: DHCP_SCAN_FLAG,
}
impl ::core::marker::Copy for DHCP_SCAN_ITEM {}
impl ::core::clone::Clone for DHCP_SCAN_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_SCAN_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SCAN_ITEM").field("IpAddress", &self.IpAddress).field("ScanFlag", &self.ScanFlag).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_SCAN_ITEM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_SCAN_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.IpAddress == other.IpAddress && self.ScanFlag == other.ScanFlag
    }
}
impl ::core::cmp::Eq for DHCP_SCAN_ITEM {}
impl ::core::default::Default for DHCP_SCAN_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SCAN_LIST {
    pub NumScanItems: u32,
    pub ScanItems: *mut DHCP_SCAN_ITEM,
}
impl ::core::marker::Copy for DHCP_SCAN_LIST {}
impl ::core::clone::Clone for DHCP_SCAN_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_SCAN_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SCAN_LIST").field("NumScanItems", &self.NumScanItems).field("ScanItems", &self.ScanItems).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_SCAN_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_SCAN_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumScanItems == other.NumScanItems && self.ScanItems == other.ScanItems
    }
}
impl ::core::cmp::Eq for DHCP_SCAN_LIST {}
impl ::core::default::Default for DHCP_SCAN_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SEARCH_INFO {
    pub SearchType: DHCP_SEARCH_INFO_TYPE,
    pub SearchInfo: DHCP_SEARCH_INFO_0,
}
impl ::core::marker::Copy for DHCP_SEARCH_INFO {}
impl ::core::clone::Clone for DHCP_SEARCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_SEARCH_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_SEARCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub union DHCP_SEARCH_INFO_0 {
    pub ClientIpAddress: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DHCP_SEARCH_INFO_0 {}
impl ::core::clone::Clone for DHCP_SEARCH_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_SEARCH_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_SEARCH_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SEARCH_INFO_V6 {
    pub SearchType: DHCP_SEARCH_INFO_TYPE_V6,
    pub SearchInfo: DHCP_SEARCH_INFO_V6_0,
}
impl ::core::marker::Copy for DHCP_SEARCH_INFO_V6 {}
impl ::core::clone::Clone for DHCP_SEARCH_INFO_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_SEARCH_INFO_V6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_SEARCH_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub union DHCP_SEARCH_INFO_V6_0 {
    pub ClientIpAddress: DHCP_IPV6_ADDRESS,
    pub ClientDUID: DHCP_BINARY_DATA,
    pub ClientName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DHCP_SEARCH_INFO_V6_0 {}
impl ::core::clone::Clone for DHCP_SEARCH_INFO_V6_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_SEARCH_INFO_V6_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_SEARCH_INFO_V6_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SERVER_CONFIG_INFO {
    pub APIProtocolSupport: u32,
    pub DatabaseName: ::windows::core::PWSTR,
    pub DatabasePath: ::windows::core::PWSTR,
    pub BackupPath: ::windows::core::PWSTR,
    pub BackupInterval: u32,
    pub DatabaseLoggingFlag: u32,
    pub RestoreFlag: u32,
    pub DatabaseCleanupInterval: u32,
    pub DebugFlag: u32,
}
impl ::core::marker::Copy for DHCP_SERVER_CONFIG_INFO {}
impl ::core::clone::Clone for DHCP_SERVER_CONFIG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_SERVER_CONFIG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SERVER_CONFIG_INFO").field("APIProtocolSupport", &self.APIProtocolSupport).field("DatabaseName", &self.DatabaseName).field("DatabasePath", &self.DatabasePath).field("BackupPath", &self.BackupPath).field("BackupInterval", &self.BackupInterval).field("DatabaseLoggingFlag", &self.DatabaseLoggingFlag).field("RestoreFlag", &self.RestoreFlag).field("DatabaseCleanupInterval", &self.DatabaseCleanupInterval).field("DebugFlag", &self.DebugFlag).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_SERVER_CONFIG_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_SERVER_CONFIG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.APIProtocolSupport == other.APIProtocolSupport && self.DatabaseName == other.DatabaseName && self.DatabasePath == other.DatabasePath && self.BackupPath == other.BackupPath && self.BackupInterval == other.BackupInterval && self.DatabaseLoggingFlag == other.DatabaseLoggingFlag && self.RestoreFlag == other.RestoreFlag && self.DatabaseCleanupInterval == other.DatabaseCleanupInterval && self.DebugFlag == other.DebugFlag
    }
}
impl ::core::cmp::Eq for DHCP_SERVER_CONFIG_INFO {}
impl ::core::default::Default for DHCP_SERVER_CONFIG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SERVER_CONFIG_INFO_V4 {
    pub APIProtocolSupport: u32,
    pub DatabaseName: ::windows::core::PWSTR,
    pub DatabasePath: ::windows::core::PWSTR,
    pub BackupPath: ::windows::core::PWSTR,
    pub BackupInterval: u32,
    pub DatabaseLoggingFlag: u32,
    pub RestoreFlag: u32,
    pub DatabaseCleanupInterval: u32,
    pub DebugFlag: u32,
    pub dwPingRetries: u32,
    pub cbBootTableString: u32,
    pub wszBootTableString: ::windows::core::PWSTR,
    pub fAuditLog: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SERVER_CONFIG_INFO_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SERVER_CONFIG_INFO_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_SERVER_CONFIG_INFO_V4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SERVER_CONFIG_INFO_V4")
            .field("APIProtocolSupport", &self.APIProtocolSupport)
            .field("DatabaseName", &self.DatabaseName)
            .field("DatabasePath", &self.DatabasePath)
            .field("BackupPath", &self.BackupPath)
            .field("BackupInterval", &self.BackupInterval)
            .field("DatabaseLoggingFlag", &self.DatabaseLoggingFlag)
            .field("RestoreFlag", &self.RestoreFlag)
            .field("DatabaseCleanupInterval", &self.DatabaseCleanupInterval)
            .field("DebugFlag", &self.DebugFlag)
            .field("dwPingRetries", &self.dwPingRetries)
            .field("cbBootTableString", &self.cbBootTableString)
            .field("wszBootTableString", &self.wszBootTableString)
            .field("fAuditLog", &self.fAuditLog)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_SERVER_CONFIG_INFO_V4 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SERVER_CONFIG_INFO_V4 {
    fn eq(&self, other: &Self) -> bool {
        self.APIProtocolSupport == other.APIProtocolSupport && self.DatabaseName == other.DatabaseName && self.DatabasePath == other.DatabasePath && self.BackupPath == other.BackupPath && self.BackupInterval == other.BackupInterval && self.DatabaseLoggingFlag == other.DatabaseLoggingFlag && self.RestoreFlag == other.RestoreFlag && self.DatabaseCleanupInterval == other.DatabaseCleanupInterval && self.DebugFlag == other.DebugFlag && self.dwPingRetries == other.dwPingRetries && self.cbBootTableString == other.cbBootTableString && self.wszBootTableString == other.wszBootTableString && self.fAuditLog == other.fAuditLog
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SERVER_CONFIG_INFO_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SERVER_CONFIG_INFO_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SERVER_CONFIG_INFO_V6 {
    pub UnicastFlag: super::super::Foundation::BOOL,
    pub RapidCommitFlag: super::super::Foundation::BOOL,
    pub PreferredLifetime: u32,
    pub ValidLifetime: u32,
    pub T1: u32,
    pub T2: u32,
    pub PreferredLifetimeIATA: u32,
    pub ValidLifetimeIATA: u32,
    pub fAuditLog: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SERVER_CONFIG_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SERVER_CONFIG_INFO_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_SERVER_CONFIG_INFO_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SERVER_CONFIG_INFO_V6").field("UnicastFlag", &self.UnicastFlag).field("RapidCommitFlag", &self.RapidCommitFlag).field("PreferredLifetime", &self.PreferredLifetime).field("ValidLifetime", &self.ValidLifetime).field("T1", &self.T1).field("T2", &self.T2).field("PreferredLifetimeIATA", &self.PreferredLifetimeIATA).field("ValidLifetimeIATA", &self.ValidLifetimeIATA).field("fAuditLog", &self.fAuditLog).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_SERVER_CONFIG_INFO_V6 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SERVER_CONFIG_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.UnicastFlag == other.UnicastFlag && self.RapidCommitFlag == other.RapidCommitFlag && self.PreferredLifetime == other.PreferredLifetime && self.ValidLifetime == other.ValidLifetime && self.T1 == other.T1 && self.T2 == other.T2 && self.PreferredLifetimeIATA == other.PreferredLifetimeIATA && self.ValidLifetimeIATA == other.ValidLifetimeIATA && self.fAuditLog == other.fAuditLog
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SERVER_CONFIG_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SERVER_CONFIG_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SERVER_CONFIG_INFO_VQ {
    pub APIProtocolSupport: u32,
    pub DatabaseName: ::windows::core::PWSTR,
    pub DatabasePath: ::windows::core::PWSTR,
    pub BackupPath: ::windows::core::PWSTR,
    pub BackupInterval: u32,
    pub DatabaseLoggingFlag: u32,
    pub RestoreFlag: u32,
    pub DatabaseCleanupInterval: u32,
    pub DebugFlag: u32,
    pub dwPingRetries: u32,
    pub cbBootTableString: u32,
    pub wszBootTableString: ::windows::core::PWSTR,
    pub fAuditLog: super::super::Foundation::BOOL,
    pub QuarantineOn: super::super::Foundation::BOOL,
    pub QuarDefFail: u32,
    pub QuarRuntimeStatus: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SERVER_CONFIG_INFO_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SERVER_CONFIG_INFO_VQ {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_SERVER_CONFIG_INFO_VQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SERVER_CONFIG_INFO_VQ")
            .field("APIProtocolSupport", &self.APIProtocolSupport)
            .field("DatabaseName", &self.DatabaseName)
            .field("DatabasePath", &self.DatabasePath)
            .field("BackupPath", &self.BackupPath)
            .field("BackupInterval", &self.BackupInterval)
            .field("DatabaseLoggingFlag", &self.DatabaseLoggingFlag)
            .field("RestoreFlag", &self.RestoreFlag)
            .field("DatabaseCleanupInterval", &self.DatabaseCleanupInterval)
            .field("DebugFlag", &self.DebugFlag)
            .field("dwPingRetries", &self.dwPingRetries)
            .field("cbBootTableString", &self.cbBootTableString)
            .field("wszBootTableString", &self.wszBootTableString)
            .field("fAuditLog", &self.fAuditLog)
            .field("QuarantineOn", &self.QuarantineOn)
            .field("QuarDefFail", &self.QuarDefFail)
            .field("QuarRuntimeStatus", &self.QuarRuntimeStatus)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_SERVER_CONFIG_INFO_VQ {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SERVER_CONFIG_INFO_VQ {
    fn eq(&self, other: &Self) -> bool {
        self.APIProtocolSupport == other.APIProtocolSupport && self.DatabaseName == other.DatabaseName && self.DatabasePath == other.DatabasePath && self.BackupPath == other.BackupPath && self.BackupInterval == other.BackupInterval && self.DatabaseLoggingFlag == other.DatabaseLoggingFlag && self.RestoreFlag == other.RestoreFlag && self.DatabaseCleanupInterval == other.DatabaseCleanupInterval && self.DebugFlag == other.DebugFlag && self.dwPingRetries == other.dwPingRetries && self.cbBootTableString == other.cbBootTableString && self.wszBootTableString == other.wszBootTableString && self.fAuditLog == other.fAuditLog && self.QuarantineOn == other.QuarantineOn && self.QuarDefFail == other.QuarDefFail && self.QuarRuntimeStatus == other.QuarRuntimeStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SERVER_CONFIG_INFO_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SERVER_CONFIG_INFO_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DHCP_SERVER_OPTIONS {
    pub MessageType: *mut u8,
    pub SubnetMask: *mut u32,
    pub RequestedAddress: *mut u32,
    pub RequestLeaseTime: *mut u32,
    pub OverlayFields: *mut u8,
    pub RouterAddress: *mut u32,
    pub Server: *mut u32,
    pub ParameterRequestList: *mut u8,
    pub ParameterRequestListLength: u32,
    pub MachineName: ::windows::core::PSTR,
    pub MachineNameLength: u32,
    pub ClientHardwareAddressType: u8,
    pub ClientHardwareAddressLength: u8,
    pub ClientHardwareAddress: *mut u8,
    pub ClassIdentifier: ::windows::core::PSTR,
    pub ClassIdentifierLength: u32,
    pub VendorClass: *mut u8,
    pub VendorClassLength: u32,
    pub DNSFlags: u32,
    pub DNSNameLength: u32,
    pub DNSName: *mut u8,
    pub DSDomainNameRequested: super::super::Foundation::BOOLEAN,
    pub DSDomainName: ::windows::core::PSTR,
    pub DSDomainNameLen: u32,
    pub ScopeId: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DHCP_SERVER_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DHCP_SERVER_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_SERVER_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SERVER_OPTIONS")
            .field("MessageType", &self.MessageType)
            .field("SubnetMask", &self.SubnetMask)
            .field("RequestedAddress", &self.RequestedAddress)
            .field("RequestLeaseTime", &self.RequestLeaseTime)
            .field("OverlayFields", &self.OverlayFields)
            .field("RouterAddress", &self.RouterAddress)
            .field("Server", &self.Server)
            .field("ParameterRequestList", &self.ParameterRequestList)
            .field("ParameterRequestListLength", &self.ParameterRequestListLength)
            .field("MachineName", &self.MachineName)
            .field("MachineNameLength", &self.MachineNameLength)
            .field("ClientHardwareAddressType", &self.ClientHardwareAddressType)
            .field("ClientHardwareAddressLength", &self.ClientHardwareAddressLength)
            .field("ClientHardwareAddress", &self.ClientHardwareAddress)
            .field("ClassIdentifier", &self.ClassIdentifier)
            .field("ClassIdentifierLength", &self.ClassIdentifierLength)
            .field("VendorClass", &self.VendorClass)
            .field("VendorClassLength", &self.VendorClassLength)
            .field("DNSFlags", &self.DNSFlags)
            .field("DNSNameLength", &self.DNSNameLength)
            .field("DNSName", &self.DNSName)
            .field("DSDomainNameRequested", &self.DSDomainNameRequested)
            .field("DSDomainName", &self.DSDomainName)
            .field("DSDomainNameLen", &self.DSDomainNameLen)
            .field("ScopeId", &self.ScopeId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DHCP_SERVER_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SERVER_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType
            && self.SubnetMask == other.SubnetMask
            && self.RequestedAddress == other.RequestedAddress
            && self.RequestLeaseTime == other.RequestLeaseTime
            && self.OverlayFields == other.OverlayFields
            && self.RouterAddress == other.RouterAddress
            && self.Server == other.Server
            && self.ParameterRequestList == other.ParameterRequestList
            && self.ParameterRequestListLength == other.ParameterRequestListLength
            && self.MachineName == other.MachineName
            && self.MachineNameLength == other.MachineNameLength
            && self.ClientHardwareAddressType == other.ClientHardwareAddressType
            && self.ClientHardwareAddressLength == other.ClientHardwareAddressLength
            && self.ClientHardwareAddress == other.ClientHardwareAddress
            && self.ClassIdentifier == other.ClassIdentifier
            && self.ClassIdentifierLength == other.ClassIdentifierLength
            && self.VendorClass == other.VendorClass
            && self.VendorClassLength == other.VendorClassLength
            && self.DNSFlags == other.DNSFlags
            && self.DNSNameLength == other.DNSNameLength
            && self.DNSName == other.DNSName
            && self.DSDomainNameRequested == other.DSDomainNameRequested
            && self.DSDomainName == other.DSDomainName
            && self.DSDomainNameLen == other.DSDomainNameLen
            && self.ScopeId == other.ScopeId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SERVER_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SERVER_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SERVER_SPECIFIC_STRINGS {
    pub DefaultVendorClassName: ::windows::core::PWSTR,
    pub DefaultUserClassName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DHCP_SERVER_SPECIFIC_STRINGS {}
impl ::core::clone::Clone for DHCP_SERVER_SPECIFIC_STRINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_SERVER_SPECIFIC_STRINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SERVER_SPECIFIC_STRINGS").field("DefaultVendorClassName", &self.DefaultVendorClassName).field("DefaultUserClassName", &self.DefaultUserClassName).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_SERVER_SPECIFIC_STRINGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_SERVER_SPECIFIC_STRINGS {
    fn eq(&self, other: &Self) -> bool {
        self.DefaultVendorClassName == other.DefaultVendorClassName && self.DefaultUserClassName == other.DefaultUserClassName
    }
}
impl ::core::cmp::Eq for DHCP_SERVER_SPECIFIC_STRINGS {}
impl ::core::default::Default for DHCP_SERVER_SPECIFIC_STRINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SUBNET_ELEMENT_DATA {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_0,
}
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_DATA {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_ELEMENT_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub union DHCP_SUBNET_ELEMENT_DATA_0 {
    pub IpRange: *mut DHCP_IP_RANGE,
    pub SecondaryHost: *mut DHCP_HOST_INFO,
    pub ReservedIp: *mut DHCP_IP_RESERVATION,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE,
    pub IpUsedCluster: *mut DHCP_IP_CLUSTER,
}
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_DATA_0 {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_ELEMENT_DATA_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SUBNET_ELEMENT_DATA_V4 {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_V4_0,
}
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_DATA_V4 {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_DATA_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_ELEMENT_DATA_V4 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub union DHCP_SUBNET_ELEMENT_DATA_V4_0 {
    pub IpRange: *mut DHCP_IP_RANGE,
    pub SecondaryHost: *mut DHCP_HOST_INFO,
    pub ReservedIp: *mut DHCP_IP_RESERVATION_V4,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE,
    pub IpUsedCluster: *mut DHCP_IP_CLUSTER,
}
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_DATA_V4_0 {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_DATA_V4_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_ELEMENT_DATA_V4_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_V4_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SUBNET_ELEMENT_DATA_V5 {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_V5_0,
}
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_DATA_V5 {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_DATA_V5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_ELEMENT_DATA_V5 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub union DHCP_SUBNET_ELEMENT_DATA_V5_0 {
    pub IpRange: *mut DHCP_BOOTP_IP_RANGE,
    pub SecondaryHost: *mut DHCP_HOST_INFO,
    pub ReservedIp: *mut DHCP_IP_RESERVATION_V4,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE,
    pub IpUsedCluster: *mut DHCP_IP_CLUSTER,
}
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_DATA_V5_0 {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_DATA_V5_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_ELEMENT_DATA_V5_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_V5_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SUBNET_ELEMENT_DATA_V6 {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE_V6,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_V6_0,
}
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_DATA_V6 {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_DATA_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_ELEMENT_DATA_V6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub union DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    pub IpRange: *mut DHCP_IP_RANGE_V6,
    pub ReservedIp: *mut DHCP_IP_RESERVATION_V6,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE_V6,
}
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_DATA_V6_0 {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_SUBNET_ELEMENT_DATA,
}
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_INFO_ARRAY {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUBNET_ELEMENT_INFO_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_INFO_ARRAY {}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    pub NumElements: u32,
    pub Elements: *mut DHCP_SUBNET_ELEMENT_DATA_V4,
}
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    pub NumElements: u32,
    pub Elements: *mut DHCP_SUBNET_ELEMENT_DATA_V5,
}
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    pub NumElements: u32,
    pub Elements: *mut DHCP_SUBNET_ELEMENT_DATA_V6,
}
impl ::core::marker::Copy for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {}
impl ::core::clone::Clone for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_UNION(pub u8);
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_UNION_V4(pub u8);
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_UNION_V6(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SUBNET_INFO {
    pub SubnetAddress: u32,
    pub SubnetMask: u32,
    pub SubnetName: ::windows::core::PWSTR,
    pub SubnetComment: ::windows::core::PWSTR,
    pub PrimaryHost: DHCP_HOST_INFO,
    pub SubnetState: DHCP_SUBNET_STATE,
}
impl ::core::marker::Copy for DHCP_SUBNET_INFO {}
impl ::core::clone::Clone for DHCP_SUBNET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_SUBNET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUBNET_INFO").field("SubnetAddress", &self.SubnetAddress).field("SubnetMask", &self.SubnetMask).field("SubnetName", &self.SubnetName).field("SubnetComment", &self.SubnetComment).field("PrimaryHost", &self.PrimaryHost).field("SubnetState", &self.SubnetState).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SubnetAddress == other.SubnetAddress && self.SubnetMask == other.SubnetMask && self.SubnetName == other.SubnetName && self.SubnetComment == other.SubnetComment && self.PrimaryHost == other.PrimaryHost && self.SubnetState == other.SubnetState
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_INFO {}
impl ::core::default::Default for DHCP_SUBNET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SUBNET_INFO_V6 {
    pub SubnetAddress: DHCP_IPV6_ADDRESS,
    pub Prefix: u32,
    pub Preference: u16,
    pub SubnetName: ::windows::core::PWSTR,
    pub SubnetComment: ::windows::core::PWSTR,
    pub State: u32,
    pub ScopeId: u32,
}
impl ::core::marker::Copy for DHCP_SUBNET_INFO_V6 {}
impl ::core::clone::Clone for DHCP_SUBNET_INFO_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_SUBNET_INFO_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUBNET_INFO_V6").field("SubnetAddress", &self.SubnetAddress).field("Prefix", &self.Prefix).field("Preference", &self.Preference).field("SubnetName", &self.SubnetName).field("SubnetComment", &self.SubnetComment).field("State", &self.State).field("ScopeId", &self.ScopeId).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_INFO_V6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.SubnetAddress == other.SubnetAddress && self.Prefix == other.Prefix && self.Preference == other.Preference && self.SubnetName == other.SubnetName && self.SubnetComment == other.SubnetComment && self.State == other.State && self.ScopeId == other.ScopeId
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_INFO_V6 {}
impl ::core::default::Default for DHCP_SUBNET_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SUBNET_INFO_VQ {
    pub SubnetAddress: u32,
    pub SubnetMask: u32,
    pub SubnetName: ::windows::core::PWSTR,
    pub SubnetComment: ::windows::core::PWSTR,
    pub PrimaryHost: DHCP_HOST_INFO,
    pub SubnetState: DHCP_SUBNET_STATE,
    pub QuarantineOn: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: i64,
    pub Reserved4: i64,
}
impl ::core::marker::Copy for DHCP_SUBNET_INFO_VQ {}
impl ::core::clone::Clone for DHCP_SUBNET_INFO_VQ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_SUBNET_INFO_VQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUBNET_INFO_VQ").field("SubnetAddress", &self.SubnetAddress).field("SubnetMask", &self.SubnetMask).field("SubnetName", &self.SubnetName).field("SubnetComment", &self.SubnetComment).field("PrimaryHost", &self.PrimaryHost).field("SubnetState", &self.SubnetState).field("QuarantineOn", &self.QuarantineOn).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).field("Reserved4", &self.Reserved4).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_SUBNET_INFO_VQ {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_INFO_VQ {
    fn eq(&self, other: &Self) -> bool {
        self.SubnetAddress == other.SubnetAddress && self.SubnetMask == other.SubnetMask && self.SubnetName == other.SubnetName && self.SubnetComment == other.SubnetComment && self.PrimaryHost == other.PrimaryHost && self.SubnetState == other.SubnetState && self.QuarantineOn == other.QuarantineOn && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.Reserved3 == other.Reserved3 && self.Reserved4 == other.Reserved4
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_INFO_VQ {}
impl ::core::default::Default for DHCP_SUBNET_INFO_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SUPER_SCOPE_TABLE {
    pub cEntries: u32,
    pub pEntries: *mut DHCP_SUPER_SCOPE_TABLE_ENTRY,
}
impl ::core::marker::Copy for DHCP_SUPER_SCOPE_TABLE {}
impl ::core::clone::Clone for DHCP_SUPER_SCOPE_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_SUPER_SCOPE_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUPER_SCOPE_TABLE").field("cEntries", &self.cEntries).field("pEntries", &self.pEntries).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_SUPER_SCOPE_TABLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_SUPER_SCOPE_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pEntries == other.pEntries
    }
}
impl ::core::cmp::Eq for DHCP_SUPER_SCOPE_TABLE {}
impl ::core::default::Default for DHCP_SUPER_SCOPE_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DHCP_SUPER_SCOPE_TABLE_ENTRY {
    pub SubnetAddress: u32,
    pub SuperScopeNumber: u32,
    pub NextInSuperScope: u32,
    pub SuperScopeName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DHCP_SUPER_SCOPE_TABLE_ENTRY {}
impl ::core::clone::Clone for DHCP_SUPER_SCOPE_TABLE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DHCP_SUPER_SCOPE_TABLE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUPER_SCOPE_TABLE_ENTRY").field("SubnetAddress", &self.SubnetAddress).field("SuperScopeNumber", &self.SuperScopeNumber).field("NextInSuperScope", &self.NextInSuperScope).field("SuperScopeName", &self.SuperScopeName).finish()
    }
}
impl ::windows::core::TypeKind for DHCP_SUPER_SCOPE_TABLE_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DHCP_SUPER_SCOPE_TABLE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.SubnetAddress == other.SubnetAddress && self.SuperScopeNumber == other.SuperScopeNumber && self.NextInSuperScope == other.NextInSuperScope && self.SuperScopeName == other.SuperScopeName
    }
}
impl ::core::cmp::Eq for DHCP_SUPER_SCOPE_TABLE_ENTRY {}
impl ::core::default::Default for DHCP_SUPER_SCOPE_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct DWORD_DWORD {
    pub DWord1: u32,
    pub DWord2: u32,
}
impl ::core::marker::Copy for DWORD_DWORD {}
impl ::core::clone::Clone for DWORD_DWORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWORD_DWORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWORD_DWORD").field("DWord1", &self.DWord1).field("DWord2", &self.DWord2).finish()
    }
}
impl ::windows::core::TypeKind for DWORD_DWORD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DWORD_DWORD {
    fn eq(&self, other: &Self) -> bool {
        self.DWord1 == other.DWord1 && self.DWord2 == other.DWord2
    }
}
impl ::core::cmp::Eq for DWORD_DWORD {}
impl ::core::default::Default for DWORD_DWORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct SCOPE_MIB_INFO {
    pub Subnet: u32,
    pub NumAddressesInuse: u32,
    pub NumAddressesFree: u32,
    pub NumPendingOffers: u32,
}
impl ::core::marker::Copy for SCOPE_MIB_INFO {}
impl ::core::clone::Clone for SCOPE_MIB_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCOPE_MIB_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_MIB_INFO").field("Subnet", &self.Subnet).field("NumAddressesInuse", &self.NumAddressesInuse).field("NumAddressesFree", &self.NumAddressesFree).field("NumPendingOffers", &self.NumPendingOffers).finish()
    }
}
impl ::windows::core::TypeKind for SCOPE_MIB_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCOPE_MIB_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Subnet == other.Subnet && self.NumAddressesInuse == other.NumAddressesInuse && self.NumAddressesFree == other.NumAddressesFree && self.NumPendingOffers == other.NumPendingOffers
    }
}
impl ::core::cmp::Eq for SCOPE_MIB_INFO {}
impl ::core::default::Default for SCOPE_MIB_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct SCOPE_MIB_INFO_V5 {
    pub Subnet: u32,
    pub NumAddressesInuse: u32,
    pub NumAddressesFree: u32,
    pub NumPendingOffers: u32,
}
impl ::core::marker::Copy for SCOPE_MIB_INFO_V5 {}
impl ::core::clone::Clone for SCOPE_MIB_INFO_V5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCOPE_MIB_INFO_V5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_MIB_INFO_V5").field("Subnet", &self.Subnet).field("NumAddressesInuse", &self.NumAddressesInuse).field("NumAddressesFree", &self.NumAddressesFree).field("NumPendingOffers", &self.NumPendingOffers).finish()
    }
}
impl ::windows::core::TypeKind for SCOPE_MIB_INFO_V5 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCOPE_MIB_INFO_V5 {
    fn eq(&self, other: &Self) -> bool {
        self.Subnet == other.Subnet && self.NumAddressesInuse == other.NumAddressesInuse && self.NumAddressesFree == other.NumAddressesFree && self.NumPendingOffers == other.NumPendingOffers
    }
}
impl ::core::cmp::Eq for SCOPE_MIB_INFO_V5 {}
impl ::core::default::Default for SCOPE_MIB_INFO_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct SCOPE_MIB_INFO_V6 {
    pub Subnet: DHCP_IPV6_ADDRESS,
    pub NumAddressesInuse: u64,
    pub NumAddressesFree: u64,
    pub NumPendingAdvertises: u64,
}
impl ::core::marker::Copy for SCOPE_MIB_INFO_V6 {}
impl ::core::clone::Clone for SCOPE_MIB_INFO_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCOPE_MIB_INFO_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_MIB_INFO_V6").field("Subnet", &self.Subnet).field("NumAddressesInuse", &self.NumAddressesInuse).field("NumAddressesFree", &self.NumAddressesFree).field("NumPendingAdvertises", &self.NumPendingAdvertises).finish()
    }
}
impl ::windows::core::TypeKind for SCOPE_MIB_INFO_V6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCOPE_MIB_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.Subnet == other.Subnet && self.NumAddressesInuse == other.NumAddressesInuse && self.NumAddressesFree == other.NumAddressesFree && self.NumPendingAdvertises == other.NumPendingAdvertises
    }
}
impl ::core::cmp::Eq for SCOPE_MIB_INFO_V6 {}
impl ::core::default::Default for SCOPE_MIB_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub struct SCOPE_MIB_INFO_VQ {
    pub Subnet: u32,
    pub NumAddressesInuse: u32,
    pub NumAddressesFree: u32,
    pub NumPendingOffers: u32,
    pub QtnNumLeases: u32,
    pub QtnPctQtnLeases: u32,
    pub QtnProbationLeases: u32,
    pub QtnNonQtnLeases: u32,
    pub QtnExemptLeases: u32,
    pub QtnCapableClients: u32,
}
impl ::core::marker::Copy for SCOPE_MIB_INFO_VQ {}
impl ::core::clone::Clone for SCOPE_MIB_INFO_VQ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCOPE_MIB_INFO_VQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_MIB_INFO_VQ")
            .field("Subnet", &self.Subnet)
            .field("NumAddressesInuse", &self.NumAddressesInuse)
            .field("NumAddressesFree", &self.NumAddressesFree)
            .field("NumPendingOffers", &self.NumPendingOffers)
            .field("QtnNumLeases", &self.QtnNumLeases)
            .field("QtnPctQtnLeases", &self.QtnPctQtnLeases)
            .field("QtnProbationLeases", &self.QtnProbationLeases)
            .field("QtnNonQtnLeases", &self.QtnNonQtnLeases)
            .field("QtnExemptLeases", &self.QtnExemptLeases)
            .field("QtnCapableClients", &self.QtnCapableClients)
            .finish()
    }
}
impl ::windows::core::TypeKind for SCOPE_MIB_INFO_VQ {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCOPE_MIB_INFO_VQ {
    fn eq(&self, other: &Self) -> bool {
        self.Subnet == other.Subnet && self.NumAddressesInuse == other.NumAddressesInuse && self.NumAddressesFree == other.NumAddressesFree && self.NumPendingOffers == other.NumPendingOffers && self.QtnNumLeases == other.QtnNumLeases && self.QtnPctQtnLeases == other.QtnPctQtnLeases && self.QtnProbationLeases == other.QtnProbationLeases && self.QtnNonQtnLeases == other.QtnNonQtnLeases && self.QtnExemptLeases == other.QtnExemptLeases && self.QtnCapableClients == other.QtnCapableClients
    }
}
impl ::core::cmp::Eq for SCOPE_MIB_INFO_VQ {}
impl ::core::default::Default for SCOPE_MIB_INFO_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub type LPDHCP_CONTROL = ::core::option::Option<unsafe extern "system" fn(dwcontrolcode: u32, lpreserved: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub type LPDHCP_DELETE_CLIENT = ::core::option::Option<unsafe extern "system" fn(ipaddress: u32, hwaddress: *mut u8, hwaddresslength: u32, reserved: u32, clienttype: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub type LPDHCP_DROP_SEND = ::core::option::Option<unsafe extern "system" fn(packet: *mut *mut u8, packetsize: *mut u32, controlcode: u32, ipaddress: u32, reserved: *mut ::core::ffi::c_void, pktcontext: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDHCP_ENTRY_POINT_FUNC = ::core::option::Option<unsafe extern "system" fn(chaindlls: ::windows::core::PCWSTR, calloutversion: u32, callouttbl: *mut DHCP_CALLOUT_TABLE) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub type LPDHCP_GIVE_ADDRESS = ::core::option::Option<unsafe extern "system" fn(packet: *mut u8, packetsize: u32, controlcode: u32, ipaddress: u32, altaddress: u32, addrtype: u32, leasetime: u32, reserved: *mut ::core::ffi::c_void, pktcontext: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDHCP_HANDLE_OPTIONS = ::core::option::Option<unsafe extern "system" fn(packet: *mut u8, packetsize: u32, reserved: *mut ::core::ffi::c_void, pktcontext: *mut ::core::ffi::c_void, serveroptions: *mut DHCP_SERVER_OPTIONS) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub type LPDHCP_NEWPKT = ::core::option::Option<unsafe extern "system" fn(packet: *mut *mut u8, packetsize: *mut u32, ipaddress: u32, reserved: *mut ::core::ffi::c_void, pktcontext: *mut *mut ::core::ffi::c_void, processit: *mut i32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dhcp\"`*"]
pub type LPDHCP_PROB = ::core::option::Option<unsafe extern "system" fn(packet: *mut u8, packetsize: u32, controlcode: u32, ipaddress: u32, altaddress: u32, reserved: *mut ::core::ffi::c_void, pktcontext: *mut ::core::ffi::c_void) -> u32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
