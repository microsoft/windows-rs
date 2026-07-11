#[inline]
pub unsafe fn DhcpAddFilterV4<P0>(serveripaddress: P0, addfilterinfo: *const DHCP_FILTER_ADD_INFO, forceflag: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpAddFilterV4(serveripaddress : windows_core::PCWSTR, addfilterinfo : *const DHCP_FILTER_ADD_INFO, forceflag : windows_core::BOOL) -> u32);
    unsafe { DhcpAddFilterV4(serveripaddress.param().abi(), addfilterinfo, forceflag.into()) }
}
#[inline]
pub unsafe fn DhcpAddSecurityGroup<P0>(pserver: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpAddSecurityGroup(pserver : windows_core::PCWSTR) -> u32);
    unsafe { DhcpAddSecurityGroup(pserver.param().abi()) }
}
#[inline]
pub unsafe fn DhcpAddServer(flags: u32, idinfo: *mut core::ffi::c_void, newserver: LPDHCP_SERVER_INFO, callbackfn: *mut core::ffi::c_void, callbackdata: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpAddServer(flags : u32, idinfo : *mut core::ffi::c_void, newserver : LPDHCP_SERVER_INFO, callbackfn : *mut core::ffi::c_void, callbackdata : *mut core::ffi::c_void) -> u32);
    unsafe { DhcpAddServer(flags, idinfo as _, newserver, callbackfn as _, callbackdata as _) }
}
#[inline]
pub unsafe fn DhcpAddSubnetElement<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpAddSubnetElement(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, addelementinfo : *const DHCP_SUBNET_ELEMENT_DATA) -> u32);
    unsafe { DhcpAddSubnetElement(serveripaddress.param().abi(), subnetaddress, addelementinfo) }
}
#[inline]
pub unsafe fn DhcpAddSubnetElementV4<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpAddSubnetElementV4(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, addelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V4) -> u32);
    unsafe { DhcpAddSubnetElementV4(serveripaddress.param().abi(), subnetaddress, addelementinfo) }
}
#[inline]
pub unsafe fn DhcpAddSubnetElementV5<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V5) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpAddSubnetElementV5(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, addelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V5) -> u32);
    unsafe { DhcpAddSubnetElementV5(serveripaddress.param().abi(), subnetaddress, addelementinfo) }
}
#[inline]
pub unsafe fn DhcpAddSubnetElementV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, addelementinfo: *mut DHCP_SUBNET_ELEMENT_DATA_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpAddSubnetElementV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, addelementinfo : *mut DHCP_SUBNET_ELEMENT_DATA_V6) -> u32);
    unsafe { DhcpAddSubnetElementV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), addelementinfo as _) }
}
#[inline]
pub unsafe fn DhcpAuditLogGetParams<P0>(serveripaddress: P0, flags: u32, auditlogdir: *mut windows_core::PWSTR, diskcheckinterval: *mut u32, maxlogfilessize: *mut u32, minspaceondisk: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpAuditLogGetParams(serveripaddress : windows_core::PCWSTR, flags : u32, auditlogdir : *mut windows_core::PWSTR, diskcheckinterval : *mut u32, maxlogfilessize : *mut u32, minspaceondisk : *mut u32) -> u32);
    unsafe { DhcpAuditLogGetParams(serveripaddress.param().abi(), flags, auditlogdir as _, diskcheckinterval as _, maxlogfilessize as _, minspaceondisk as _) }
}
#[inline]
pub unsafe fn DhcpAuditLogSetParams<P0, P2>(serveripaddress: P0, flags: u32, auditlogdir: P2, diskcheckinterval: u32, maxlogfilessize: u32, minspaceondisk: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpAuditLogSetParams(serveripaddress : windows_core::PCWSTR, flags : u32, auditlogdir : windows_core::PCWSTR, diskcheckinterval : u32, maxlogfilessize : u32, minspaceondisk : u32) -> u32);
    unsafe { DhcpAuditLogSetParams(serveripaddress.param().abi(), flags, auditlogdir.param().abi(), diskcheckinterval, maxlogfilessize, minspaceondisk) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpCreateClass<P0>(serveripaddress: P0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpCreateClass(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classinfo : *mut DHCP_CLASS_INFO) -> u32);
    unsafe { DhcpCreateClass(serveripaddress.param().abi(), reservedmustbezero, classinfo as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpCreateClassV6<P0>(serveripaddress: P0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateClassV6(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classinfo : *mut DHCP_CLASS_INFO_V6) -> u32);
    unsafe { DhcpCreateClassV6(serveripaddress.param().abi(), reservedmustbezero, classinfo as _) }
}
#[inline]
pub unsafe fn DhcpCreateClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateClientInfo(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO) -> u32);
    unsafe { DhcpCreateClientInfo(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpCreateClientInfoV4<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateClientInfoV4(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_V4) -> u32);
    unsafe { DhcpCreateClientInfoV4(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpCreateClientInfoVQ<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateClientInfoVQ(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_VQ) -> u32);
    unsafe { DhcpCreateClientInfoVQ(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpCreateOption<P0>(serveripaddress: P0, optionid: DHCP_OPTION_ID, optioninfo: *const DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateOption(serveripaddress : windows_core::PCWSTR, optionid : DHCP_OPTION_ID, optioninfo : *const DHCP_OPTION) -> u32);
    unsafe { DhcpCreateOption(serveripaddress.param().abi(), optionid, optioninfo) }
}
#[inline]
pub unsafe fn DhcpCreateOptionV5<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, classname: P3, vendorname: P4, optioninfo: *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpCreateOptionV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut DHCP_OPTION) -> u32);
    unsafe { DhcpCreateOptionV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo as _) }
}
#[inline]
pub unsafe fn DhcpCreateOptionV6<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, classname: P3, vendorname: P4, optioninfo: *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateOptionV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut DHCP_OPTION) -> u32);
    unsafe { DhcpCreateOptionV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo as _) }
}
#[inline]
pub unsafe fn DhcpCreateSubnet<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, subnetinfo: *const DHCP_SUBNET_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateSubnet(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, subnetinfo : *const DHCP_SUBNET_INFO) -> u32);
    unsafe { DhcpCreateSubnet(serveripaddress.param().abi(), subnetaddress, subnetinfo) }
}
#[inline]
pub unsafe fn DhcpCreateSubnetV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut DHCP_SUBNET_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateSubnetV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, subnetinfo : *mut DHCP_SUBNET_INFO_V6) -> u32);
    unsafe { DhcpCreateSubnetV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), subnetinfo as _) }
}
#[inline]
pub unsafe fn DhcpCreateSubnetVQ<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, subnetinfo: *const DHCP_SUBNET_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateSubnetVQ(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, subnetinfo : *const DHCP_SUBNET_INFO_VQ) -> u32);
    unsafe { DhcpCreateSubnetVQ(serveripaddress.param().abi(), subnetaddress, subnetinfo) }
}
#[inline]
pub unsafe fn DhcpDeleteClass<P0, P2>(serveripaddress: P0, reservedmustbezero: u32, classname: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpDeleteClass(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classname : windows_core::PCWSTR) -> u32);
    unsafe { DhcpDeleteClass(serveripaddress.param().abi(), reservedmustbezero, classname.param().abi()) }
}
#[inline]
pub unsafe fn DhcpDeleteClassV6<P0, P2>(serveripaddress: P0, reservedmustbezero: u32, classname: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDeleteClassV6(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classname : windows_core::PCWSTR) -> u32);
    unsafe { DhcpDeleteClassV6(serveripaddress.param().abi(), reservedmustbezero, classname.param().abi()) }
}
#[inline]
pub unsafe fn DhcpDeleteClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_SEARCH_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDeleteClientInfo(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_SEARCH_INFO) -> u32);
    unsafe { DhcpDeleteClientInfo(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpDeleteClientInfoV6<P0>(serveripaddress: P0, clientinfo: *const DHCP_SEARCH_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDeleteClientInfoV6(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_SEARCH_INFO_V6) -> u32);
    unsafe { DhcpDeleteClientInfoV6(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpDeleteFilterV4<P0>(serveripaddress: P0, deletefilterinfo: *const DHCP_ADDR_PATTERN) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDeleteFilterV4(serveripaddress : windows_core::PCWSTR, deletefilterinfo : *const DHCP_ADDR_PATTERN) -> u32);
    unsafe { DhcpDeleteFilterV4(serveripaddress.param().abi(), deletefilterinfo) }
}
#[inline]
pub unsafe fn DhcpDeleteServer(flags: u32, idinfo: *mut core::ffi::c_void, newserver: LPDHCP_SERVER_INFO, callbackfn: *mut core::ffi::c_void, callbackdata: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpDeleteServer(flags : u32, idinfo : *mut core::ffi::c_void, newserver : LPDHCP_SERVER_INFO, callbackfn : *mut core::ffi::c_void, callbackdata : *mut core::ffi::c_void) -> u32);
    unsafe { DhcpDeleteServer(flags, idinfo as _, newserver, callbackfn as _, callbackdata as _) }
}
#[inline]
pub unsafe fn DhcpDeleteSubnet<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDeleteSubnet(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, forceflag : DHCP_FORCE_FLAG) -> u32);
    unsafe { DhcpDeleteSubnet(serveripaddress.param().abi(), subnetaddress, forceflag) }
}
#[inline]
pub unsafe fn DhcpDeleteSubnetV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDeleteSubnetV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, forceflag : DHCP_FORCE_FLAG) -> u32);
    unsafe { DhcpDeleteSubnetV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), forceflag) }
}
#[inline]
pub unsafe fn DhcpDeleteSuperScopeV4<P0, P1>(serveripaddress: P0, superscopename: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpDeleteSuperScopeV4(serveripaddress : windows_core::PCWSTR, superscopename : windows_core::PCWSTR) -> u32);
    unsafe { DhcpDeleteSuperScopeV4(serveripaddress.param().abi(), superscopename.param().abi()) }
}
#[inline]
pub unsafe fn DhcpDsCleanup() {
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpDsCleanup());
    unsafe { DhcpDsCleanup() }
}
#[inline]
pub unsafe fn DhcpDsInit() -> u32 {
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpDsInit() -> u32);
    unsafe { DhcpDsInit() }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpEnumClasses<P0>(serveripaddress: P0, reservedmustbezero: u32, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, classinfoarray: *mut LPDHCP_CLASS_INFO_ARRAY, nread: *mut u32, ntotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpEnumClasses(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, classinfoarray : *mut LPDHCP_CLASS_INFO_ARRAY, nread : *mut u32, ntotal : *mut u32) -> u32);
    unsafe { DhcpEnumClasses(serveripaddress.param().abi(), reservedmustbezero, resumehandle as _, preferredmaximum, classinfoarray as _, nread as _, ntotal as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpEnumClassesV6<P0>(serveripaddress: P0, reservedmustbezero: u32, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, classinfoarray: *mut LPDHCP_CLASS_INFO_ARRAY_V6, nread: *mut u32, ntotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumClassesV6(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, classinfoarray : *mut LPDHCP_CLASS_INFO_ARRAY_V6, nread : *mut u32, ntotal : *mut u32) -> u32);
    unsafe { DhcpEnumClassesV6(serveripaddress.param().abi(), reservedmustbezero, resumehandle as _, preferredmaximum, classinfoarray as _, nread as _, ntotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumFilterV4<P0>(serveripaddress: P0, resumehandle: *mut DHCP_ADDR_PATTERN, preferredmaximum: u32, listtype: DHCP_FILTER_LIST_TYPE, enumfilterinfo: *mut LPDHCP_FILTER_ENUM_INFO, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumFilterV4(serveripaddress : windows_core::PCWSTR, resumehandle : *mut DHCP_ADDR_PATTERN, preferredmaximum : u32, listtype : DHCP_FILTER_LIST_TYPE, enumfilterinfo : *mut LPDHCP_FILTER_ENUM_INFO, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpEnumFilterV4(serveripaddress.param().abi(), resumehandle as _, preferredmaximum, listtype, enumfilterinfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumOptionValues<P0>(serveripaddress: P0, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, optionvalues: *mut LPDHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumOptionValues(serveripaddress : windows_core::PCWSTR, scopeinfo : *const DHCP_OPTION_SCOPE_INFO, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, optionvalues : *mut LPDHCP_OPTION_VALUE_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    unsafe { DhcpEnumOptionValues(serveripaddress.param().abi(), scopeinfo, resumehandle as _, preferredmaximum, optionvalues as _, optionsread as _, optionstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumOptionValuesV5<P0, P2, P3>(serveripaddress: P0, flags: u32, classname: P2, vendorname: P3, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, optionvalues: *mut LPDHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpEnumOptionValuesV5(serveripaddress : windows_core::PCWSTR, flags : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, optionvalues : *mut LPDHCP_OPTION_VALUE_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    unsafe { DhcpEnumOptionValuesV5(serveripaddress.param().abi(), flags, classname.param().abi(), vendorname.param().abi(), scopeinfo as _, resumehandle as _, preferredmaximum, optionvalues as _, optionsread as _, optionstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumOptionValuesV6<P0, P2, P3>(serveripaddress: P0, flags: u32, classname: P2, vendorname: P3, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, optionvalues: *mut LPDHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumOptionValuesV6(serveripaddress : windows_core::PCWSTR, flags : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, optionvalues : *mut LPDHCP_OPTION_VALUE_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    unsafe { DhcpEnumOptionValuesV6(serveripaddress.param().abi(), flags, classname.param().abi(), vendorname.param().abi(), scopeinfo as _, resumehandle as _, preferredmaximum, optionvalues as _, optionsread as _, optionstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumOptions<P0>(serveripaddress: P0, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, options: *mut LPDHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumOptions(serveripaddress : windows_core::PCWSTR, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, options : *mut LPDHCP_OPTION_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    unsafe { DhcpEnumOptions(serveripaddress.param().abi(), resumehandle as _, preferredmaximum, options as _, optionsread as _, optionstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumOptionsV5<P0, P2, P3>(serveripaddress: P0, flags: u32, classname: P2, vendorname: P3, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, options: *mut LPDHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpEnumOptionsV5(serveripaddress : windows_core::PCWSTR, flags : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, options : *mut LPDHCP_OPTION_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    unsafe { DhcpEnumOptionsV5(serveripaddress.param().abi(), flags, classname.param().abi(), vendorname.param().abi(), resumehandle as _, preferredmaximum, options as _, optionsread as _, optionstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumOptionsV6<P0, P2, P3>(serveripaddress: P0, flags: u32, classname: P2, vendorname: P3, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, options: *mut LPDHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumOptionsV6(serveripaddress : windows_core::PCWSTR, flags : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, options : *mut LPDHCP_OPTION_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    unsafe { DhcpEnumOptionsV6(serveripaddress.param().abi(), flags, classname.param().abi(), vendorname.param().abi(), resumehandle as _, preferredmaximum, options as _, optionsread as _, optionstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumServers(flags: u32, idinfo: *mut core::ffi::c_void, servers: *mut LPDHCP_SERVER_INFO_ARRAY, callbackfn: *mut core::ffi::c_void, callbackdata: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpEnumServers(flags : u32, idinfo : *mut core::ffi::c_void, servers : *mut LPDHCP_SERVER_INFO_ARRAY, callbackfn : *mut core::ffi::c_void, callbackdata : *mut core::ffi::c_void) -> u32);
    unsafe { DhcpEnumServers(flags, idinfo as _, servers as _, callbackfn as _, callbackdata as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetClients<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, clientinfo: *mut LPDHCP_CLIENT_INFO_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClients(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, clientinfo : *mut LPDHCP_CLIENT_INFO_ARRAY, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetClients(serveripaddress.param().abi(), subnetaddress, resumehandle as _, preferredmaximum, clientinfo as _, clientsread as _, clientstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetClientsFilterStatusInfo<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, clientinfo: *mut LPDHCP_CLIENT_FILTER_STATUS_INFO_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsFilterStatusInfo(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, clientinfo : *mut LPDHCP_CLIENT_FILTER_STATUS_INFO_ARRAY, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetClientsFilterStatusInfo(serveripaddress.param().abi(), subnetaddress, resumehandle as _, preferredmaximum, clientinfo as _, clientsread as _, clientstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetClientsV4<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, clientinfo: *mut LPDHCP_CLIENT_INFO_ARRAY_V4, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsV4(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, clientinfo : *mut LPDHCP_CLIENT_INFO_ARRAY_V4, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetClientsV4(serveripaddress.param().abi(), subnetaddress, resumehandle as _, preferredmaximum, clientinfo as _, clientsread as _, clientstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetClientsV5<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, clientinfo: *mut LPDHCP_CLIENT_INFO_ARRAY_V5, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsV5(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, clientinfo : *mut LPDHCP_CLIENT_INFO_ARRAY_V5, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetClientsV5(serveripaddress.param().abi(), subnetaddress, resumehandle as _, preferredmaximum, clientinfo as _, clientsread as _, clientstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetClientsV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, resumehandle: *mut DHCP_RESUME_IPV6_HANDLE, preferredmaximum: u32, clientinfo: *mut LPDHCP_CLIENT_INFO_ARRAY_V6, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, resumehandle : *mut DHCP_RESUME_IPV6_HANDLE, preferredmaximum : u32, clientinfo : *mut LPDHCP_CLIENT_INFO_ARRAY_V6, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetClientsV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), resumehandle as _, preferredmaximum, clientinfo as _, clientsread as _, clientstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetClientsVQ<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, clientinfo: *mut LPDHCP_CLIENT_INFO_ARRAY_VQ, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsVQ(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, clientinfo : *mut LPDHCP_CLIENT_INFO_ARRAY_VQ, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetClientsVQ(serveripaddress.param().abi(), subnetaddress, resumehandle as _, preferredmaximum, clientinfo as _, clientsread as _, clientstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetElements<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, enumelementinfo: *mut LPDHCP_SUBNET_ELEMENT_INFO_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetElements(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, enumelementtype : DHCP_SUBNET_ELEMENT_TYPE, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, enumelementinfo : *mut LPDHCP_SUBNET_ELEMENT_INFO_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetElements(serveripaddress.param().abi(), subnetaddress, enumelementtype, resumehandle as _, preferredmaximum, enumelementinfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetElementsV4<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, enumelementinfo: *mut LPDHCP_SUBNET_ELEMENT_INFO_ARRAY_V4, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetElementsV4(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, enumelementtype : DHCP_SUBNET_ELEMENT_TYPE, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, enumelementinfo : *mut LPDHCP_SUBNET_ELEMENT_INFO_ARRAY_V4, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetElementsV4(serveripaddress.param().abi(), subnetaddress, enumelementtype, resumehandle as _, preferredmaximum, enumelementinfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetElementsV5<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, enumelementinfo: *mut LPDHCP_SUBNET_ELEMENT_INFO_ARRAY_V5, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetElementsV5(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, enumelementtype : DHCP_SUBNET_ELEMENT_TYPE, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, enumelementinfo : *mut LPDHCP_SUBNET_ELEMENT_INFO_ARRAY_V5, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetElementsV5(serveripaddress.param().abi(), subnetaddress, enumelementtype, resumehandle as _, preferredmaximum, enumelementinfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetElementsV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE_V6, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, enumelementinfo: *mut LPDHCP_SUBNET_ELEMENT_INFO_ARRAY_V6, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetElementsV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, enumelementtype : DHCP_SUBNET_ELEMENT_TYPE_V6, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, enumelementinfo : *mut LPDHCP_SUBNET_ELEMENT_INFO_ARRAY_V6, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetElementsV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), enumelementtype, resumehandle as _, preferredmaximum, enumelementinfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnets<P0>(serveripaddress: P0, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, enuminfo: *mut LPDHCP_IP_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnets(serveripaddress : windows_core::PCWSTR, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, enuminfo : *mut LPDHCP_IP_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnets(serveripaddress.param().abi(), resumehandle as _, preferredmaximum, enuminfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetsV6<P0>(serveripaddress: P0, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, enuminfo: *mut LPDHCPV6_IP_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetsV6(serveripaddress : windows_core::PCWSTR, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, enuminfo : *mut LPDHCPV6_IP_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetsV6(serveripaddress.param().abi(), resumehandle as _, preferredmaximum, enuminfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpGetAllOptionValues<P0>(serveripaddress: P0, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, values: *mut LPDHCP_ALL_OPTION_VALUES) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpGetAllOptionValues(serveripaddress : windows_core::PCWSTR, flags : u32, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, values : *mut LPDHCP_ALL_OPTION_VALUES) -> u32);
    unsafe { DhcpGetAllOptionValues(serveripaddress.param().abi(), flags, scopeinfo as _, values as _) }
}
#[inline]
pub unsafe fn DhcpGetAllOptionValuesV6<P0>(serveripaddress: P0, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, values: *mut LPDHCP_ALL_OPTION_VALUES) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetAllOptionValuesV6(serveripaddress : windows_core::PCWSTR, flags : u32, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, values : *mut LPDHCP_ALL_OPTION_VALUES) -> u32);
    unsafe { DhcpGetAllOptionValuesV6(serveripaddress.param().abi(), flags, scopeinfo as _, values as _) }
}
#[inline]
pub unsafe fn DhcpGetAllOptions<P0>(serveripaddress: P0, flags: u32, optionstruct: *mut LPDHCP_ALL_OPTIONS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpGetAllOptions(serveripaddress : windows_core::PCWSTR, flags : u32, optionstruct : *mut LPDHCP_ALL_OPTIONS) -> u32);
    unsafe { DhcpGetAllOptions(serveripaddress.param().abi(), flags, optionstruct as _) }
}
#[inline]
pub unsafe fn DhcpGetAllOptionsV6<P0>(serveripaddress: P0, flags: u32, optionstruct: *mut LPDHCP_ALL_OPTIONS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetAllOptionsV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionstruct : *mut LPDHCP_ALL_OPTIONS) -> u32);
    unsafe { DhcpGetAllOptionsV6(serveripaddress.param().abi(), flags, optionstruct as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpGetClassInfo<P0>(serveripaddress: P0, reservedmustbezero: u32, partialclassinfo: *mut DHCP_CLASS_INFO, filledclassinfo: *mut LPDHCP_CLASS_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpGetClassInfo(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, partialclassinfo : *mut DHCP_CLASS_INFO, filledclassinfo : *mut LPDHCP_CLASS_INFO) -> u32);
    unsafe { DhcpGetClassInfo(serveripaddress.param().abi(), reservedmustbezero, partialclassinfo as _, filledclassinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetClientInfo<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut LPDHCP_CLIENT_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetClientInfo(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut LPDHCP_CLIENT_INFO) -> u32);
    unsafe { DhcpGetClientInfo(serveripaddress.param().abi(), searchinfo, clientinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetClientInfoV4<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut LPDHCP_CLIENT_INFO_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetClientInfoV4(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut LPDHCP_CLIENT_INFO_V4) -> u32);
    unsafe { DhcpGetClientInfoV4(serveripaddress.param().abi(), searchinfo, clientinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetClientInfoV6<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO_V6, clientinfo: *mut LPDHCP_CLIENT_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetClientInfoV6(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO_V6, clientinfo : *mut LPDHCP_CLIENT_INFO_V6) -> u32);
    unsafe { DhcpGetClientInfoV6(serveripaddress.param().abi(), searchinfo, clientinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetClientInfoVQ<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut LPDHCP_CLIENT_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetClientInfoVQ(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut LPDHCP_CLIENT_INFO_VQ) -> u32);
    unsafe { DhcpGetClientInfoVQ(serveripaddress.param().abi(), searchinfo, clientinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetClientOptions<P0>(serveripaddress: P0, clientipaddress: DHCP_IP_ADDRESS, clientsubnetmask: DHCP_IP_MASK, clientoptions: *mut LPDHCP_OPTION_LIST) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetClientOptions(serveripaddress : windows_core::PCWSTR, clientipaddress : DHCP_IP_ADDRESS, clientsubnetmask : DHCP_IP_MASK, clientoptions : *mut LPDHCP_OPTION_LIST) -> u32);
    unsafe { DhcpGetClientOptions(serveripaddress.param().abi(), clientipaddress, clientsubnetmask, clientoptions as _) }
}
#[inline]
pub unsafe fn DhcpGetFilterV4<P0>(serveripaddress: P0, globalfilterinfo: *mut DHCP_FILTER_GLOBAL_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetFilterV4(serveripaddress : windows_core::PCWSTR, globalfilterinfo : *mut DHCP_FILTER_GLOBAL_INFO) -> u32);
    unsafe { DhcpGetFilterV4(serveripaddress.param().abi(), globalfilterinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetMibInfo<P0>(serveripaddress: P0, mibinfo: *mut LPDHCP_MIB_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetMibInfo(serveripaddress : windows_core::PCWSTR, mibinfo : *mut LPDHCP_MIB_INFO) -> u32);
    unsafe { DhcpGetMibInfo(serveripaddress.param().abi(), mibinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetMibInfoV5<P0>(serveripaddress: P0, mibinfo: *mut LPDHCP_MIB_INFO_V5) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetMibInfoV5(serveripaddress : windows_core::PCWSTR, mibinfo : *mut LPDHCP_MIB_INFO_V5) -> u32);
    unsafe { DhcpGetMibInfoV5(serveripaddress.param().abi(), mibinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetMibInfoV6<P0>(serveripaddress: P0, mibinfo: *mut LPDHCP_MIB_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpGetMibInfoV6(serveripaddress : windows_core::PCWSTR, mibinfo : *mut LPDHCP_MIB_INFO_V6) -> u32);
    unsafe { DhcpGetMibInfoV6(serveripaddress.param().abi(), mibinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetOptionInfo<P0>(serveripaddress: P0, optionid: DHCP_OPTION_ID, optioninfo: *mut LPDHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetOptionInfo(serveripaddress : windows_core::PCWSTR, optionid : DHCP_OPTION_ID, optioninfo : *mut LPDHCP_OPTION) -> u32);
    unsafe { DhcpGetOptionInfo(serveripaddress.param().abi(), optionid, optioninfo as _) }
}
#[inline]
pub unsafe fn DhcpGetOptionInfoV5<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, classname: P3, vendorname: P4, optioninfo: *mut LPDHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpGetOptionInfoV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut LPDHCP_OPTION) -> u32);
    unsafe { DhcpGetOptionInfoV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo as _) }
}
#[inline]
pub unsafe fn DhcpGetOptionInfoV6<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, classname: P3, vendorname: P4, optioninfo: *mut LPDHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetOptionInfoV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut LPDHCP_OPTION) -> u32);
    unsafe { DhcpGetOptionInfoV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo as _) }
}
#[inline]
pub unsafe fn DhcpGetOptionValue<P0>(serveripaddress: P0, optionid: DHCP_OPTION_ID, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalue: *mut LPDHCP_OPTION_VALUE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetOptionValue(serveripaddress : windows_core::PCWSTR, optionid : DHCP_OPTION_ID, scopeinfo : *const DHCP_OPTION_SCOPE_INFO, optionvalue : *mut LPDHCP_OPTION_VALUE) -> u32);
    unsafe { DhcpGetOptionValue(serveripaddress.param().abi(), optionid, scopeinfo, optionvalue as _) }
}
#[inline]
pub unsafe fn DhcpGetOptionValueV5<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, classname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut LPDHCP_OPTION_VALUE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpGetOptionValueV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalue : *mut LPDHCP_OPTION_VALUE) -> u32);
    unsafe { DhcpGetOptionValueV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo as _, optionvalue as _) }
}
#[inline]
pub unsafe fn DhcpGetOptionValueV6<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, classname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, optionvalue: *mut LPDHCP_OPTION_VALUE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpGetOptionValueV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, optionvalue : *mut LPDHCP_OPTION_VALUE) -> u32);
    unsafe { DhcpGetOptionValueV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo as _, optionvalue as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpGetServerBindingInfo<P0>(serveripaddress: P0, flags: u32, bindelementsinfo: *mut LPDHCP_BIND_ELEMENT_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetServerBindingInfo(serveripaddress : windows_core::PCWSTR, flags : u32, bindelementsinfo : *mut LPDHCP_BIND_ELEMENT_ARRAY) -> u32);
    unsafe { DhcpGetServerBindingInfo(serveripaddress.param().abi(), flags, bindelementsinfo as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpGetServerBindingInfoV6<P0>(serveripaddress: P0, flags: u32, bindelementsinfo: *mut LPDHCPV6_BIND_ELEMENT_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetServerBindingInfoV6(serveripaddress : windows_core::PCWSTR, flags : u32, bindelementsinfo : *mut LPDHCPV6_BIND_ELEMENT_ARRAY) -> u32);
    unsafe { DhcpGetServerBindingInfoV6(serveripaddress.param().abi(), flags, bindelementsinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetServerSpecificStrings<P0>(serveripaddress: P0, serverspecificstrings: *mut LPDHCP_SERVER_SPECIFIC_STRINGS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetServerSpecificStrings(serveripaddress : windows_core::PCWSTR, serverspecificstrings : *mut LPDHCP_SERVER_SPECIFIC_STRINGS) -> u32);
    unsafe { DhcpGetServerSpecificStrings(serveripaddress.param().abi(), serverspecificstrings as _) }
}
#[inline]
pub unsafe fn DhcpGetSubnetDelayOffer<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, timedelayinmilliseconds: *mut u16) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetSubnetDelayOffer(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, timedelayinmilliseconds : *mut u16) -> u32);
    unsafe { DhcpGetSubnetDelayOffer(serveripaddress.param().abi(), subnetaddress, timedelayinmilliseconds as _) }
}
#[inline]
pub unsafe fn DhcpGetSubnetInfo<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, subnetinfo: *mut LPDHCP_SUBNET_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetSubnetInfo(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, subnetinfo : *mut LPDHCP_SUBNET_INFO) -> u32);
    unsafe { DhcpGetSubnetInfo(serveripaddress.param().abi(), subnetaddress, subnetinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetSubnetInfoV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut LPDHCP_SUBNET_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetSubnetInfoV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, subnetinfo : *mut LPDHCP_SUBNET_INFO_V6) -> u32);
    unsafe { DhcpGetSubnetInfoV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), subnetinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetSubnetInfoVQ<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, subnetinfo: *mut LPDHCP_SUBNET_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetSubnetInfoVQ(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, subnetinfo : *mut LPDHCP_SUBNET_INFO_VQ) -> u32);
    unsafe { DhcpGetSubnetInfoVQ(serveripaddress.param().abi(), subnetaddress, subnetinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetSuperScopeInfoV4<P0>(serveripaddress: P0, superscopetable: *mut LPDHCP_SUPER_SCOPE_TABLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpGetSuperScopeInfoV4(serveripaddress : windows_core::PCWSTR, superscopetable : *mut LPDHCP_SUPER_SCOPE_TABLE) -> u32);
    unsafe { DhcpGetSuperScopeInfoV4(serveripaddress.param().abi(), superscopetable as _) }
}
#[inline]
pub unsafe fn DhcpGetThreadOptions(pflags: *mut u32, reserved: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpGetThreadOptions(pflags : *mut u32, reserved : *mut core::ffi::c_void) -> u32);
    unsafe { DhcpGetThreadOptions(pflags as _, reserved as _) }
}
#[inline]
pub unsafe fn DhcpGetVersion<P0>(serveripaddress: P0, majorversion: *mut u32, minorversion: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetVersion(serveripaddress : windows_core::PCWSTR, majorversion : *mut u32, minorversion : *mut u32) -> u32);
    unsafe { DhcpGetVersion(serveripaddress.param().abi(), majorversion as _, minorversion as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpHlprAddV4PolicyCondition<P5>(policy: *mut DHCP_POLICY, parentexpr: u32, r#type: DHCP_POL_ATTR_TYPE, optionid: u32, suboptionid: u32, vendorname: P5, operator: DHCP_POL_COMPARATOR, value: &[u8], conditionindex: *mut u32) -> u32
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprAddV4PolicyCondition(policy : *mut DHCP_POLICY, parentexpr : u32, r#type : DHCP_POL_ATTR_TYPE, optionid : u32, suboptionid : u32, vendorname : windows_core::PCWSTR, operator : DHCP_POL_COMPARATOR, value : *const u8, valuelength : u32, conditionindex : *mut u32) -> u32);
    unsafe { DhcpHlprAddV4PolicyCondition(policy as _, parentexpr, r#type, optionid, suboptionid, vendorname.param().abi(), operator, core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap(), conditionindex as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpHlprAddV4PolicyExpr(policy: *mut DHCP_POLICY, parentexpr: u32, operator: DHCP_POL_LOGIC_OPER, exprindex: *mut u32) -> u32 {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprAddV4PolicyExpr(policy : *mut DHCP_POLICY, parentexpr : u32, operator : DHCP_POL_LOGIC_OPER, exprindex : *mut u32) -> u32);
    unsafe { DhcpHlprAddV4PolicyExpr(policy as _, parentexpr, operator, exprindex as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpHlprAddV4PolicyRange(policy: *mut DHCP_POLICY, range: *const DHCP_IP_RANGE) -> u32 {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprAddV4PolicyRange(policy : *mut DHCP_POLICY, range : *const DHCP_IP_RANGE) -> u32);
    unsafe { DhcpHlprAddV4PolicyRange(policy as _, range) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpHlprCreateV4Policy<P0, P5>(policyname: P0, fglobalpolicy: bool, subnet: DHCP_IP_ADDRESS, processingorder: u32, rootoperator: DHCP_POL_LOGIC_OPER, description: P5, enabled: bool, policy: *mut LPDHCP_POLICY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprCreateV4Policy(policyname : windows_core::PCWSTR, fglobalpolicy : windows_core::BOOL, subnet : DHCP_IP_ADDRESS, processingorder : u32, rootoperator : DHCP_POL_LOGIC_OPER, description : windows_core::PCWSTR, enabled : windows_core::BOOL, policy : *mut LPDHCP_POLICY) -> u32);
    unsafe { DhcpHlprCreateV4Policy(policyname.param().abi(), fglobalpolicy.into(), subnet, processingorder, rootoperator, description.param().abi(), enabled.into(), policy as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpHlprCreateV4PolicyEx<P0, P5>(policyname: P0, fglobalpolicy: bool, subnet: DHCP_IP_ADDRESS, processingorder: u32, rootoperator: DHCP_POL_LOGIC_OPER, description: P5, enabled: bool, policy: *mut LPDHCP_POLICY_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprCreateV4PolicyEx(policyname : windows_core::PCWSTR, fglobalpolicy : windows_core::BOOL, subnet : DHCP_IP_ADDRESS, processingorder : u32, rootoperator : DHCP_POL_LOGIC_OPER, description : windows_core::PCWSTR, enabled : windows_core::BOOL, policy : *mut LPDHCP_POLICY_EX) -> u32);
    unsafe { DhcpHlprCreateV4PolicyEx(policyname.param().abi(), fglobalpolicy.into(), subnet, processingorder, rootoperator, description.param().abi(), enabled.into(), policy as _) }
}
#[inline]
pub unsafe fn DhcpHlprFindV4DhcpProperty(propertyarray: *const DHCP_PROPERTY_ARRAY, id: DHCP_PROPERTY_ID, r#type: DHCP_PROPERTY_TYPE) -> LPDHCP_PROPERTY {
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpHlprFindV4DhcpProperty(propertyarray : *const DHCP_PROPERTY_ARRAY, id : DHCP_PROPERTY_ID, r#type : DHCP_PROPERTY_TYPE) -> LPDHCP_PROPERTY);
    unsafe { DhcpHlprFindV4DhcpProperty(propertyarray, id, r#type) }
}
#[inline]
pub unsafe fn DhcpHlprFreeV4DhcpProperty(property: *mut DHCP_PROPERTY) {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4DhcpProperty(property : *mut DHCP_PROPERTY));
    unsafe { DhcpHlprFreeV4DhcpProperty(property as _) }
}
#[inline]
pub unsafe fn DhcpHlprFreeV4DhcpPropertyArray(propertyarray: *mut DHCP_PROPERTY_ARRAY) {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4DhcpPropertyArray(propertyarray : *mut DHCP_PROPERTY_ARRAY));
    unsafe { DhcpHlprFreeV4DhcpPropertyArray(propertyarray as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpHlprFreeV4Policy(policy: *mut DHCP_POLICY) {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4Policy(policy : *mut DHCP_POLICY));
    unsafe { DhcpHlprFreeV4Policy(policy as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpHlprFreeV4PolicyArray(policyarray: *mut DHCP_POLICY_ARRAY) {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4PolicyArray(policyarray : *mut DHCP_POLICY_ARRAY));
    unsafe { DhcpHlprFreeV4PolicyArray(policyarray as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpHlprFreeV4PolicyEx(policyex: *mut DHCP_POLICY_EX) {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4PolicyEx(policyex : *mut DHCP_POLICY_EX));
    unsafe { DhcpHlprFreeV4PolicyEx(policyex as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpHlprFreeV4PolicyExArray(policyexarray: *mut DHCP_POLICY_EX_ARRAY) {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4PolicyExArray(policyexarray : *mut DHCP_POLICY_EX_ARRAY));
    unsafe { DhcpHlprFreeV4PolicyExArray(policyexarray as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpHlprIsV4PolicySingleUC(policy: *const DHCP_POLICY) -> windows_core::BOOL {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprIsV4PolicySingleUC(policy : *const DHCP_POLICY) -> windows_core::BOOL);
    unsafe { DhcpHlprIsV4PolicySingleUC(policy) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpHlprIsV4PolicyValid(ppolicy: *const DHCP_POLICY) -> u32 {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprIsV4PolicyValid(ppolicy : *const DHCP_POLICY) -> u32);
    unsafe { DhcpHlprIsV4PolicyValid(ppolicy) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpHlprIsV4PolicyWellFormed(ppolicy: *const DHCP_POLICY) -> windows_core::BOOL {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprIsV4PolicyWellFormed(ppolicy : *const DHCP_POLICY) -> windows_core::BOOL);
    unsafe { DhcpHlprIsV4PolicyWellFormed(ppolicy) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpHlprModifyV4PolicyExpr(policy: *mut DHCP_POLICY, operator: DHCP_POL_LOGIC_OPER) -> u32 {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprModifyV4PolicyExpr(policy : *mut DHCP_POLICY, operator : DHCP_POL_LOGIC_OPER) -> u32);
    unsafe { DhcpHlprModifyV4PolicyExpr(policy as _, operator) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpHlprResetV4PolicyExpr(policy: *mut DHCP_POLICY) -> u32 {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprResetV4PolicyExpr(policy : *mut DHCP_POLICY) -> u32);
    unsafe { DhcpHlprResetV4PolicyExpr(policy as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpModifyClass<P0>(serveripaddress: P0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpModifyClass(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classinfo : *mut DHCP_CLASS_INFO) -> u32);
    unsafe { DhcpModifyClass(serveripaddress.param().abi(), reservedmustbezero, classinfo as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpModifyClassV6<P0>(serveripaddress: P0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpModifyClassV6(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classinfo : *mut DHCP_CLASS_INFO_V6) -> u32);
    unsafe { DhcpModifyClassV6(serveripaddress.param().abi(), reservedmustbezero, classinfo as _) }
}
#[inline]
pub unsafe fn DhcpRemoveOption<P0>(serveripaddress: P0, optionid: DHCP_OPTION_ID) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveOption(serveripaddress : windows_core::PCWSTR, optionid : DHCP_OPTION_ID) -> u32);
    unsafe { DhcpRemoveOption(serveripaddress.param().abi(), optionid) }
}
#[inline]
pub unsafe fn DhcpRemoveOptionV5<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, classname: P3, vendorname: P4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpRemoveOptionV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR) -> u32);
    unsafe { DhcpRemoveOptionV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi()) }
}
#[inline]
pub unsafe fn DhcpRemoveOptionV6<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, classname: P3, vendorname: P4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveOptionV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR) -> u32);
    unsafe { DhcpRemoveOptionV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi()) }
}
#[inline]
pub unsafe fn DhcpRemoveOptionValue<P0>(serveripaddress: P0, optionid: DHCP_OPTION_ID, scopeinfo: *const DHCP_OPTION_SCOPE_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveOptionValue(serveripaddress : windows_core::PCWSTR, optionid : DHCP_OPTION_ID, scopeinfo : *const DHCP_OPTION_SCOPE_INFO) -> u32);
    unsafe { DhcpRemoveOptionValue(serveripaddress.param().abi(), optionid, scopeinfo) }
}
#[inline]
pub unsafe fn DhcpRemoveOptionValueV5<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, classname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpRemoveOptionValueV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO) -> u32);
    unsafe { DhcpRemoveOptionValueV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo as _) }
}
#[inline]
pub unsafe fn DhcpRemoveOptionValueV6<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, classname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveOptionValueV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6) -> u32);
    unsafe { DhcpRemoveOptionValueV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo as _) }
}
#[inline]
pub unsafe fn DhcpRemoveSubnetElement<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveSubnetElement(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, removeelementinfo : *const DHCP_SUBNET_ELEMENT_DATA, forceflag : DHCP_FORCE_FLAG) -> u32);
    unsafe { DhcpRemoveSubnetElement(serveripaddress.param().abi(), subnetaddress, removeelementinfo, forceflag) }
}
#[inline]
pub unsafe fn DhcpRemoveSubnetElementV4<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V4, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveSubnetElementV4(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, removeelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V4, forceflag : DHCP_FORCE_FLAG) -> u32);
    unsafe { DhcpRemoveSubnetElementV4(serveripaddress.param().abi(), subnetaddress, removeelementinfo, forceflag) }
}
#[inline]
pub unsafe fn DhcpRemoveSubnetElementV5<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V5, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveSubnetElementV5(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, removeelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V5, forceflag : DHCP_FORCE_FLAG) -> u32);
    unsafe { DhcpRemoveSubnetElementV5(serveripaddress.param().abi(), subnetaddress, removeelementinfo, forceflag) }
}
#[inline]
pub unsafe fn DhcpRemoveSubnetElementV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, removeelementinfo: *mut DHCP_SUBNET_ELEMENT_DATA_V6, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveSubnetElementV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, removeelementinfo : *mut DHCP_SUBNET_ELEMENT_DATA_V6, forceflag : DHCP_FORCE_FLAG) -> u32);
    unsafe { DhcpRemoveSubnetElementV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), removeelementinfo as _, forceflag) }
}
#[inline]
pub unsafe fn DhcpRpcFreeMemory(bufferpointer: *mut core::ffi::c_void) {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRpcFreeMemory(bufferpointer : *mut core::ffi::c_void));
    unsafe { DhcpRpcFreeMemory(bufferpointer as _) }
}
#[inline]
pub unsafe fn DhcpScanDatabase<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, fixflag: u32, scanlist: *mut LPDHCP_SCAN_LIST) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpScanDatabase(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, fixflag : u32, scanlist : *mut LPDHCP_SCAN_LIST) -> u32);
    unsafe { DhcpScanDatabase(serveripaddress.param().abi(), subnetaddress, fixflag, scanlist as _) }
}
#[inline]
pub unsafe fn DhcpServerAuditlogParamsFree(configinfo: *mut DHCP_SERVER_CONFIG_INFO_VQ) {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerAuditlogParamsFree(configinfo : *mut DHCP_SERVER_CONFIG_INFO_VQ));
    unsafe { DhcpServerAuditlogParamsFree(configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerBackupDatabase<P0, P1>(serveripaddress: P0, path: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpServerBackupDatabase(serveripaddress : windows_core::PCWSTR, path : windows_core::PCWSTR) -> u32);
    unsafe { DhcpServerBackupDatabase(serveripaddress.param().abi(), path.param().abi()) }
}
#[inline]
pub unsafe fn DhcpServerGetConfig<P0>(serveripaddress: P0, configinfo: *mut LPDHCP_SERVER_CONFIG_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerGetConfig(serveripaddress : windows_core::PCWSTR, configinfo : *mut LPDHCP_SERVER_CONFIG_INFO) -> u32);
    unsafe { DhcpServerGetConfig(serveripaddress.param().abi(), configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerGetConfigV4<P0>(serveripaddress: P0, configinfo: *mut LPDHCP_SERVER_CONFIG_INFO_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerGetConfigV4(serveripaddress : windows_core::PCWSTR, configinfo : *mut LPDHCP_SERVER_CONFIG_INFO_V4) -> u32);
    unsafe { DhcpServerGetConfigV4(serveripaddress.param().abi(), configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerGetConfigV6<P0>(serveripaddress: P0, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, configinfo: *mut LPDHCP_SERVER_CONFIG_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerGetConfigV6(serveripaddress : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, configinfo : *mut LPDHCP_SERVER_CONFIG_INFO_V6) -> u32);
    unsafe { DhcpServerGetConfigV6(serveripaddress.param().abi(), scopeinfo as _, configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerGetConfigVQ<P0>(serveripaddress: P0, configinfo: *mut LPDHCP_SERVER_CONFIG_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerGetConfigVQ(serveripaddress : windows_core::PCWSTR, configinfo : *mut LPDHCP_SERVER_CONFIG_INFO_VQ) -> u32);
    unsafe { DhcpServerGetConfigVQ(serveripaddress.param().abi(), configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerQueryAttribute<P0>(serveripaddr: P0, dwreserved: u32, dhcpattribid: DHCP_ATTRIB_ID, pdhcpattrib: *mut LPDHCP_ATTRIB) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpServerQueryAttribute(serveripaddr : windows_core::PCWSTR, dwreserved : u32, dhcpattribid : DHCP_ATTRIB_ID, pdhcpattrib : *mut LPDHCP_ATTRIB) -> u32);
    unsafe { DhcpServerQueryAttribute(serveripaddr.param().abi(), dwreserved, dhcpattribid, pdhcpattrib as _) }
}
#[inline]
pub unsafe fn DhcpServerQueryAttributes<P0>(serveripaddr: P0, dwreserved: u32, dwattribcount: u32, pdhcpattribs: *mut DHCP_ATTRIB_ID, pdhcpattribarr: *mut LPDHCP_ATTRIB_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpServerQueryAttributes(serveripaddr : windows_core::PCWSTR, dwreserved : u32, dwattribcount : u32, pdhcpattribs : *mut DHCP_ATTRIB_ID, pdhcpattribarr : *mut LPDHCP_ATTRIB_ARRAY) -> u32);
    unsafe { DhcpServerQueryAttributes(serveripaddr.param().abi(), dwreserved, dwattribcount, pdhcpattribs as _, pdhcpattribarr as _) }
}
#[inline]
pub unsafe fn DhcpServerQueryDnsRegCredentials<P0>(serveripaddress: P0, uname: &mut [u16], domain: &mut [u16]) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpServerQueryDnsRegCredentials(serveripaddress : windows_core::PCWSTR, unamesize : u32, uname : windows_core::PWSTR, domainsize : u32, domain : windows_core::PWSTR) -> u32);
    unsafe { DhcpServerQueryDnsRegCredentials(serveripaddress.param().abi(), uname.len().try_into().unwrap(), core::mem::transmute(uname.as_ptr()), domain.len().try_into().unwrap(), core::mem::transmute(domain.as_ptr())) }
}
#[inline]
pub unsafe fn DhcpServerRedoAuthorization<P0>(serveripaddr: P0, dwreserved: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpServerRedoAuthorization(serveripaddr : windows_core::PCWSTR, dwreserved : u32) -> u32);
    unsafe { DhcpServerRedoAuthorization(serveripaddr.param().abi(), dwreserved) }
}
#[inline]
pub unsafe fn DhcpServerRestoreDatabase<P0, P1>(serveripaddress: P0, path: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpServerRestoreDatabase(serveripaddress : windows_core::PCWSTR, path : windows_core::PCWSTR) -> u32);
    unsafe { DhcpServerRestoreDatabase(serveripaddress.param().abi(), path.param().abi()) }
}
#[inline]
pub unsafe fn DhcpServerSetConfig<P0>(serveripaddress: P0, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerSetConfig(serveripaddress : windows_core::PCWSTR, fieldstoset : u32, configinfo : *mut DHCP_SERVER_CONFIG_INFO) -> u32);
    unsafe { DhcpServerSetConfig(serveripaddress.param().abi(), fieldstoset, configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerSetConfigV4<P0>(serveripaddress: P0, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerSetConfigV4(serveripaddress : windows_core::PCWSTR, fieldstoset : u32, configinfo : *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32);
    unsafe { DhcpServerSetConfigV4(serveripaddress.param().abi(), fieldstoset, configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerSetConfigV6<P0>(serveripaddress: P0, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerSetConfigV6(serveripaddress : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, fieldstoset : u32, configinfo : *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32);
    unsafe { DhcpServerSetConfigV6(serveripaddress.param().abi(), scopeinfo as _, fieldstoset, configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerSetConfigVQ<P0>(serveripaddress: P0, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerSetConfigVQ(serveripaddress : windows_core::PCWSTR, fieldstoset : u32, configinfo : *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32);
    unsafe { DhcpServerSetConfigVQ(serveripaddress.param().abi(), fieldstoset, configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerSetDnsRegCredentials<P0, P1, P2, P3>(serveripaddress: P0, uname: P1, domain: P2, passwd: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpServerSetDnsRegCredentials(serveripaddress : windows_core::PCWSTR, uname : windows_core::PCWSTR, domain : windows_core::PCWSTR, passwd : windows_core::PCWSTR) -> u32);
    unsafe { DhcpServerSetDnsRegCredentials(serveripaddress.param().abi(), uname.param().abi(), domain.param().abi(), passwd.param().abi()) }
}
#[inline]
pub unsafe fn DhcpServerSetDnsRegCredentialsV5<P0, P1, P2, P3>(serveripaddress: P0, uname: P1, domain: P2, passwd: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpServerSetDnsRegCredentialsV5(serveripaddress : windows_core::PCWSTR, uname : windows_core::PCWSTR, domain : windows_core::PCWSTR, passwd : windows_core::PCWSTR) -> u32);
    unsafe { DhcpServerSetDnsRegCredentialsV5(serveripaddress.param().abi(), uname.param().abi(), domain.param().abi(), passwd.param().abi()) }
}
#[inline]
pub unsafe fn DhcpSetClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetClientInfo(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO) -> u32);
    unsafe { DhcpSetClientInfo(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpSetClientInfoV4<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetClientInfoV4(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_V4) -> u32);
    unsafe { DhcpSetClientInfoV4(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpSetClientInfoV6<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetClientInfoV6(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_V6) -> u32);
    unsafe { DhcpSetClientInfoV6(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpSetClientInfoVQ<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetClientInfoVQ(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_VQ) -> u32);
    unsafe { DhcpSetClientInfoVQ(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpSetFilterV4<P0>(serveripaddress: P0, globalfilterinfo: *const DHCP_FILTER_GLOBAL_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetFilterV4(serveripaddress : windows_core::PCWSTR, globalfilterinfo : *const DHCP_FILTER_GLOBAL_INFO) -> u32);
    unsafe { DhcpSetFilterV4(serveripaddress.param().abi(), globalfilterinfo) }
}
#[inline]
pub unsafe fn DhcpSetOptionInfo<P0>(serveripaddress: P0, optionid: DHCP_OPTION_ID, optioninfo: *const DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetOptionInfo(serveripaddress : windows_core::PCWSTR, optionid : DHCP_OPTION_ID, optioninfo : *const DHCP_OPTION) -> u32);
    unsafe { DhcpSetOptionInfo(serveripaddress.param().abi(), optionid, optioninfo) }
}
#[inline]
pub unsafe fn DhcpSetOptionInfoV5<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, classname: P3, vendorname: P4, optioninfo: *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpSetOptionInfoV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut DHCP_OPTION) -> u32);
    unsafe { DhcpSetOptionInfoV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo as _) }
}
#[inline]
pub unsafe fn DhcpSetOptionInfoV6<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, classname: P3, vendorname: P4, optioninfo: *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetOptionInfoV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut DHCP_OPTION) -> u32);
    unsafe { DhcpSetOptionInfoV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo as _) }
}
#[inline]
pub unsafe fn DhcpSetOptionValue<P0>(serveripaddress: P0, optionid: DHCP_OPTION_ID, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalue: *const DHCP_OPTION_DATA) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetOptionValue(serveripaddress : windows_core::PCWSTR, optionid : DHCP_OPTION_ID, scopeinfo : *const DHCP_OPTION_SCOPE_INFO, optionvalue : *const DHCP_OPTION_DATA) -> u32);
    unsafe { DhcpSetOptionValue(serveripaddress.param().abi(), optionid, scopeinfo, optionvalue) }
}
#[inline]
pub unsafe fn DhcpSetOptionValueV5<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, classname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut DHCP_OPTION_DATA) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpSetOptionValueV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalue : *mut DHCP_OPTION_DATA) -> u32);
    unsafe { DhcpSetOptionValueV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo as _, optionvalue as _) }
}
#[inline]
pub unsafe fn DhcpSetOptionValueV6<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, classname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, optionvalue: *mut DHCP_OPTION_DATA) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetOptionValueV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, optionvalue : *mut DHCP_OPTION_DATA) -> u32);
    unsafe { DhcpSetOptionValueV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo as _, optionvalue as _) }
}
#[inline]
pub unsafe fn DhcpSetOptionValues<P0>(serveripaddress: P0, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalues: *const DHCP_OPTION_VALUE_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetOptionValues(serveripaddress : windows_core::PCWSTR, scopeinfo : *const DHCP_OPTION_SCOPE_INFO, optionvalues : *const DHCP_OPTION_VALUE_ARRAY) -> u32);
    unsafe { DhcpSetOptionValues(serveripaddress.param().abi(), scopeinfo, optionvalues) }
}
#[inline]
pub unsafe fn DhcpSetOptionValuesV5<P0, P2, P3>(serveripaddress: P0, flags: u32, classname: P2, vendorname: P3, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalues: *mut DHCP_OPTION_VALUE_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpSetOptionValuesV5(serveripaddress : windows_core::PCWSTR, flags : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalues : *mut DHCP_OPTION_VALUE_ARRAY) -> u32);
    unsafe { DhcpSetOptionValuesV5(serveripaddress.param().abi(), flags, classname.param().abi(), vendorname.param().abi(), scopeinfo as _, optionvalues as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpSetServerBindingInfo<P0>(serveripaddress: P0, flags: u32, bindelementinfo: *mut DHCP_BIND_ELEMENT_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetServerBindingInfo(serveripaddress : windows_core::PCWSTR, flags : u32, bindelementinfo : *mut DHCP_BIND_ELEMENT_ARRAY) -> u32);
    unsafe { DhcpSetServerBindingInfo(serveripaddress.param().abi(), flags, bindelementinfo as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpSetServerBindingInfoV6<P0>(serveripaddress: P0, flags: u32, bindelementinfo: *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetServerBindingInfoV6(serveripaddress : windows_core::PCWSTR, flags : u32, bindelementinfo : *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32);
    unsafe { DhcpSetServerBindingInfoV6(serveripaddress.param().abi(), flags, bindelementinfo as _) }
}
#[inline]
pub unsafe fn DhcpSetSubnetDelayOffer<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, timedelayinmilliseconds: u16) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetSubnetDelayOffer(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, timedelayinmilliseconds : u16) -> u32);
    unsafe { DhcpSetSubnetDelayOffer(serveripaddress.param().abi(), subnetaddress, timedelayinmilliseconds) }
}
#[inline]
pub unsafe fn DhcpSetSubnetInfo<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, subnetinfo: *const DHCP_SUBNET_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetSubnetInfo(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, subnetinfo : *const DHCP_SUBNET_INFO) -> u32);
    unsafe { DhcpSetSubnetInfo(serveripaddress.param().abi(), subnetaddress, subnetinfo) }
}
#[inline]
pub unsafe fn DhcpSetSubnetInfoV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut DHCP_SUBNET_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpSetSubnetInfoV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, subnetinfo : *mut DHCP_SUBNET_INFO_V6) -> u32);
    unsafe { DhcpSetSubnetInfoV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), subnetinfo as _) }
}
#[inline]
pub unsafe fn DhcpSetSubnetInfoVQ<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, subnetinfo: *const DHCP_SUBNET_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetSubnetInfoVQ(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, subnetinfo : *const DHCP_SUBNET_INFO_VQ) -> u32);
    unsafe { DhcpSetSubnetInfoVQ(serveripaddress.param().abi(), subnetaddress, subnetinfo) }
}
#[inline]
pub unsafe fn DhcpSetSuperScopeV4<P0, P2>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, superscopename: P2, changeexisting: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpSetSuperScopeV4(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, superscopename : windows_core::PCWSTR, changeexisting : windows_core::BOOL) -> u32);
    unsafe { DhcpSetSuperScopeV4(serveripaddress.param().abi(), subnetaddress, superscopename.param().abi(), changeexisting.into()) }
}
#[inline]
pub unsafe fn DhcpSetThreadOptions(flags: u32, reserved: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("dhcpsapi.dll" "C" fn DhcpSetThreadOptions(flags : u32, reserved : *mut core::ffi::c_void) -> u32);
    unsafe { DhcpSetThreadOptions(flags, reserved as _) }
}
#[inline]
pub unsafe fn DhcpV4AddPolicyRange<P0, P2>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, policyname: P2, range: *const DHCP_IP_RANGE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4AddPolicyRange(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, policyname : windows_core::PCWSTR, range : *const DHCP_IP_RANGE) -> u32);
    unsafe { DhcpV4AddPolicyRange(serveripaddress.param().abi(), subnetaddress, policyname.param().abi(), range) }
}
#[inline]
pub unsafe fn DhcpV4CreateClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_PB) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4CreateClientInfo(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_PB) -> u32);
    unsafe { DhcpV4CreateClientInfo(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpV4CreateClientInfoEx<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4CreateClientInfoEx(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_EX) -> u32);
    unsafe { DhcpV4CreateClientInfoEx(serveripaddress.param().abi(), clientinfo) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpV4CreatePolicy<P0>(serveripaddress: P0, ppolicy: *const DHCP_POLICY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4CreatePolicy(serveripaddress : windows_core::PCWSTR, ppolicy : *const DHCP_POLICY) -> u32);
    unsafe { DhcpV4CreatePolicy(serveripaddress.param().abi(), ppolicy) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpV4CreatePolicyEx<P0>(serveripaddress: P0, policyex: *const DHCP_POLICY_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4CreatePolicyEx(serveripaddress : windows_core::PCWSTR, policyex : *const DHCP_POLICY_EX) -> u32);
    unsafe { DhcpV4CreatePolicyEx(serveripaddress.param().abi(), policyex) }
}
#[inline]
pub unsafe fn DhcpV4DeletePolicy<P0, P3>(serveripaddress: P0, fglobalpolicy: bool, subnetaddress: DHCP_IP_ADDRESS, policyname: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4DeletePolicy(serveripaddress : windows_core::PCWSTR, fglobalpolicy : windows_core::BOOL, subnetaddress : DHCP_IP_ADDRESS, policyname : windows_core::PCWSTR) -> u32);
    unsafe { DhcpV4DeletePolicy(serveripaddress.param().abi(), fglobalpolicy.into(), subnetaddress, policyname.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpV4EnumPolicies<P0>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, fglobalpolicy: bool, subnetaddress: DHCP_IP_ADDRESS, enuminfo: *mut LPDHCP_POLICY_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4EnumPolicies(serveripaddress : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, fglobalpolicy : windows_core::BOOL, subnetaddress : DHCP_IP_ADDRESS, enuminfo : *mut LPDHCP_POLICY_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpV4EnumPolicies(serveripaddress.param().abi(), resumehandle as _, preferredmaximum, fglobalpolicy.into(), subnetaddress, enuminfo as _, elementsread as _, elementstotal as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpV4EnumPoliciesEx<P0>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, globalpolicy: bool, subnetaddress: DHCP_IP_ADDRESS, enuminfo: *mut LPDHCP_POLICY_EX_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4EnumPoliciesEx(serveripaddress : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, globalpolicy : windows_core::BOOL, subnetaddress : DHCP_IP_ADDRESS, enuminfo : *mut LPDHCP_POLICY_EX_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpV4EnumPoliciesEx(serveripaddress.param().abi(), resumehandle as _, preferredmaximum, globalpolicy.into(), subnetaddress, enuminfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpV4EnumSubnetClients<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, clientinfo: *mut LPDHCP_CLIENT_INFO_PB_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4EnumSubnetClients(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, clientinfo : *mut LPDHCP_CLIENT_INFO_PB_ARRAY, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    unsafe { DhcpV4EnumSubnetClients(serveripaddress.param().abi(), subnetaddress, resumehandle as _, preferredmaximum, clientinfo as _, clientsread as _, clientstotal as _) }
}
#[inline]
pub unsafe fn DhcpV4EnumSubnetClientsEx<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, clientinfo: *mut LPDHCP_CLIENT_INFO_EX_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4EnumSubnetClientsEx(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, clientinfo : *mut LPDHCP_CLIENT_INFO_EX_ARRAY, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    unsafe { DhcpV4EnumSubnetClientsEx(serveripaddress.param().abi(), subnetaddress, resumehandle as _, preferredmaximum, clientinfo as _, clientsread as _, clientstotal as _) }
}
#[inline]
pub unsafe fn DhcpV4EnumSubnetReservations<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, enumelementinfo: *mut LPDHCP_RESERVATION_INFO_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4EnumSubnetReservations(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, enumelementinfo : *mut LPDHCP_RESERVATION_INFO_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpV4EnumSubnetReservations(serveripaddress.param().abi(), subnetaddress, resumehandle as _, preferredmaximum, enumelementinfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpV4FailoverAddScopeToRelationship<P0>(serveripaddress: P0, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverAddScopeToRelationship(serveripaddress : windows_core::PCWSTR, prelationship : *const DHCP_FAILOVER_RELATIONSHIP) -> u32);
    unsafe { DhcpV4FailoverAddScopeToRelationship(serveripaddress.param().abi(), prelationship) }
}
#[inline]
pub unsafe fn DhcpV4FailoverCreateRelationship<P0>(serveripaddress: P0, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverCreateRelationship(serveripaddress : windows_core::PCWSTR, prelationship : *const DHCP_FAILOVER_RELATIONSHIP) -> u32);
    unsafe { DhcpV4FailoverCreateRelationship(serveripaddress.param().abi(), prelationship) }
}
#[inline]
pub unsafe fn DhcpV4FailoverDeleteRelationship<P0, P1>(serveripaddress: P0, prelationshipname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverDeleteRelationship(serveripaddress : windows_core::PCWSTR, prelationshipname : windows_core::PCWSTR) -> u32);
    unsafe { DhcpV4FailoverDeleteRelationship(serveripaddress.param().abi(), prelationshipname.param().abi()) }
}
#[inline]
pub unsafe fn DhcpV4FailoverDeleteScopeFromRelationship<P0>(serveripaddress: P0, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverDeleteScopeFromRelationship(serveripaddress : windows_core::PCWSTR, prelationship : *const DHCP_FAILOVER_RELATIONSHIP) -> u32);
    unsafe { DhcpV4FailoverDeleteScopeFromRelationship(serveripaddress.param().abi(), prelationship) }
}
#[inline]
pub unsafe fn DhcpV4FailoverEnumRelationship<P0>(serveripaddress: P0, resumehandle: *mut DHCP_RESUME_HANDLE, preferredmaximum: u32, prelationship: *mut LPDHCP_FAILOVER_RELATIONSHIP_ARRAY, relationshipread: *mut u32, relationshiptotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverEnumRelationship(serveripaddress : windows_core::PCWSTR, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : u32, prelationship : *mut LPDHCP_FAILOVER_RELATIONSHIP_ARRAY, relationshipread : *mut u32, relationshiptotal : *mut u32) -> u32);
    unsafe { DhcpV4FailoverEnumRelationship(serveripaddress.param().abi(), resumehandle as _, preferredmaximum, prelationship as _, relationshipread as _, relationshiptotal as _) }
}
#[inline]
pub unsafe fn DhcpV4FailoverGetAddressStatus<P0>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, pstatus: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetAddressStatus(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, pstatus : *mut u32) -> u32);
    unsafe { DhcpV4FailoverGetAddressStatus(serveripaddress.param().abi(), subnetaddress, pstatus as _) }
}
#[inline]
pub unsafe fn DhcpV4FailoverGetClientInfo<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut LPDHCPV4_FAILOVER_CLIENT_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetClientInfo(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut LPDHCPV4_FAILOVER_CLIENT_INFO) -> u32);
    unsafe { DhcpV4FailoverGetClientInfo(serveripaddress.param().abi(), searchinfo, clientinfo as _) }
}
#[inline]
pub unsafe fn DhcpV4FailoverGetRelationship<P0, P1>(serveripaddress: P0, prelationshipname: P1, prelationship: *mut LPDHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetRelationship(serveripaddress : windows_core::PCWSTR, prelationshipname : windows_core::PCWSTR, prelationship : *mut LPDHCP_FAILOVER_RELATIONSHIP) -> u32);
    unsafe { DhcpV4FailoverGetRelationship(serveripaddress.param().abi(), prelationshipname.param().abi(), prelationship as _) }
}
#[inline]
pub unsafe fn DhcpV4FailoverGetScopeRelationship<P0>(serveripaddress: P0, scopeid: DHCP_IP_ADDRESS, prelationship: *mut LPDHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetScopeRelationship(serveripaddress : windows_core::PCWSTR, scopeid : DHCP_IP_ADDRESS, prelationship : *mut LPDHCP_FAILOVER_RELATIONSHIP) -> u32);
    unsafe { DhcpV4FailoverGetScopeRelationship(serveripaddress.param().abi(), scopeid, prelationship as _) }
}
#[inline]
pub unsafe fn DhcpV4FailoverGetScopeStatistics<P0>(serveripaddress: P0, scopeid: DHCP_IP_ADDRESS, pstats: *mut LPDHCP_FAILOVER_STATISTICS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetScopeStatistics(serveripaddress : windows_core::PCWSTR, scopeid : DHCP_IP_ADDRESS, pstats : *mut LPDHCP_FAILOVER_STATISTICS) -> u32);
    unsafe { DhcpV4FailoverGetScopeStatistics(serveripaddress.param().abi(), scopeid, pstats as _) }
}
#[inline]
pub unsafe fn DhcpV4FailoverGetSystemTime<P0>(serveripaddress: P0, ptime: *mut u32, pmaxalloweddeltatime: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetSystemTime(serveripaddress : windows_core::PCWSTR, ptime : *mut u32, pmaxalloweddeltatime : *mut u32) -> u32);
    unsafe { DhcpV4FailoverGetSystemTime(serveripaddress.param().abi(), ptime as _, pmaxalloweddeltatime as _) }
}
#[inline]
pub unsafe fn DhcpV4FailoverSetRelationship<P0>(serveripaddress: P0, flags: u32, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverSetRelationship(serveripaddress : windows_core::PCWSTR, flags : u32, prelationship : *const DHCP_FAILOVER_RELATIONSHIP) -> u32);
    unsafe { DhcpV4FailoverSetRelationship(serveripaddress.param().abi(), flags, prelationship) }
}
#[inline]
pub unsafe fn DhcpV4FailoverTriggerAddrAllocation<P0, P1>(serveripaddress: P0, pfailrelname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverTriggerAddrAllocation(serveripaddress : windows_core::PCWSTR, pfailrelname : windows_core::PCWSTR) -> u32);
    unsafe { DhcpV4FailoverTriggerAddrAllocation(serveripaddress.param().abi(), pfailrelname.param().abi()) }
}
#[inline]
pub unsafe fn DhcpV4GetAllOptionValues<P0>(serveripaddress: P0, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, values: *mut LPDHCP_ALL_OPTION_VALUES_PB) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4GetAllOptionValues(serveripaddress : windows_core::PCWSTR, flags : u32, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, values : *mut LPDHCP_ALL_OPTION_VALUES_PB) -> u32);
    unsafe { DhcpV4GetAllOptionValues(serveripaddress.param().abi(), flags, scopeinfo as _, values as _) }
}
#[inline]
pub unsafe fn DhcpV4GetClientInfo<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut LPDHCP_CLIENT_INFO_PB) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4GetClientInfo(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut LPDHCP_CLIENT_INFO_PB) -> u32);
    unsafe { DhcpV4GetClientInfo(serveripaddress.param().abi(), searchinfo, clientinfo as _) }
}
#[inline]
pub unsafe fn DhcpV4GetClientInfoEx<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut LPDHCP_CLIENT_INFO_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4GetClientInfoEx(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut LPDHCP_CLIENT_INFO_EX) -> u32);
    unsafe { DhcpV4GetClientInfoEx(serveripaddress.param().abi(), searchinfo, clientinfo as _) }
}
#[inline]
pub unsafe fn DhcpV4GetFreeIPAddress<P0>(serveripaddress: P0, scopeid: DHCP_IP_ADDRESS, startip: DHCP_IP_ADDRESS, endip: DHCP_IP_ADDRESS, numfreeaddrreq: u32, ipaddrlist: *mut LPDHCP_IP_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4GetFreeIPAddress(serveripaddress : windows_core::PCWSTR, scopeid : DHCP_IP_ADDRESS, startip : DHCP_IP_ADDRESS, endip : DHCP_IP_ADDRESS, numfreeaddrreq : u32, ipaddrlist : *mut LPDHCP_IP_ARRAY) -> u32);
    unsafe { DhcpV4GetFreeIPAddress(serveripaddress.param().abi(), scopeid, startip, endip, numfreeaddrreq, ipaddrlist as _) }
}
#[inline]
pub unsafe fn DhcpV4GetOptionValue<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, policyname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut LPDHCP_OPTION_VALUE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4GetOptionValue(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, policyname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalue : *mut LPDHCP_OPTION_VALUE) -> u32);
    unsafe { DhcpV4GetOptionValue(serveripaddress.param().abi(), flags, optionid, policyname.param().abi(), vendorname.param().abi(), scopeinfo as _, optionvalue as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpV4GetPolicy<P0, P3>(serveripaddress: P0, fglobalpolicy: bool, subnetaddress: DHCP_IP_ADDRESS, policyname: P3, policy: *mut LPDHCP_POLICY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4GetPolicy(serveripaddress : windows_core::PCWSTR, fglobalpolicy : windows_core::BOOL, subnetaddress : DHCP_IP_ADDRESS, policyname : windows_core::PCWSTR, policy : *mut LPDHCP_POLICY) -> u32);
    unsafe { DhcpV4GetPolicy(serveripaddress.param().abi(), fglobalpolicy.into(), subnetaddress, policyname.param().abi(), policy as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpV4GetPolicyEx<P0, P3>(serveripaddress: P0, globalpolicy: bool, subnetaddress: DHCP_IP_ADDRESS, policyname: P3, policy: *mut LPDHCP_POLICY_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4GetPolicyEx(serveripaddress : windows_core::PCWSTR, globalpolicy : windows_core::BOOL, subnetaddress : DHCP_IP_ADDRESS, policyname : windows_core::PCWSTR, policy : *mut LPDHCP_POLICY_EX) -> u32);
    unsafe { DhcpV4GetPolicyEx(serveripaddress.param().abi(), globalpolicy.into(), subnetaddress, policyname.param().abi(), policy as _) }
}
#[inline]
pub unsafe fn DhcpV4QueryPolicyEnforcement<P0>(serveripaddress: P0, fglobalpolicy: bool, subnetaddress: DHCP_IP_ADDRESS, enabled: *mut windows_core::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4QueryPolicyEnforcement(serveripaddress : windows_core::PCWSTR, fglobalpolicy : windows_core::BOOL, subnetaddress : DHCP_IP_ADDRESS, enabled : *mut windows_core::BOOL) -> u32);
    unsafe { DhcpV4QueryPolicyEnforcement(serveripaddress.param().abi(), fglobalpolicy.into(), subnetaddress, enabled as _) }
}
#[inline]
pub unsafe fn DhcpV4RemoveOptionValue<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, policyname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4RemoveOptionValue(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, policyname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO) -> u32);
    unsafe { DhcpV4RemoveOptionValue(serveripaddress.param().abi(), flags, optionid, policyname.param().abi(), vendorname.param().abi(), scopeinfo as _) }
}
#[inline]
pub unsafe fn DhcpV4RemovePolicyRange<P0, P2>(serveripaddress: P0, subnetaddress: DHCP_IP_ADDRESS, policyname: P2, range: *const DHCP_IP_RANGE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4RemovePolicyRange(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, policyname : windows_core::PCWSTR, range : *const DHCP_IP_RANGE) -> u32);
    unsafe { DhcpV4RemovePolicyRange(serveripaddress.param().abi(), subnetaddress, policyname.param().abi(), range) }
}
#[inline]
pub unsafe fn DhcpV4SetOptionValue<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: DHCP_OPTION_ID, policyname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut DHCP_OPTION_DATA) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4SetOptionValue(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : DHCP_OPTION_ID, policyname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalue : *mut DHCP_OPTION_DATA) -> u32);
    unsafe { DhcpV4SetOptionValue(serveripaddress.param().abi(), flags, optionid, policyname.param().abi(), vendorname.param().abi(), scopeinfo as _, optionvalue as _) }
}
#[inline]
pub unsafe fn DhcpV4SetOptionValues<P0, P2, P3>(serveripaddress: P0, flags: u32, policyname: P2, vendorname: P3, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalues: *mut DHCP_OPTION_VALUE_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4SetOptionValues(serveripaddress : windows_core::PCWSTR, flags : u32, policyname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalues : *mut DHCP_OPTION_VALUE_ARRAY) -> u32);
    unsafe { DhcpV4SetOptionValues(serveripaddress.param().abi(), flags, policyname.param().abi(), vendorname.param().abi(), scopeinfo as _, optionvalues as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpV4SetPolicy<P0, P4>(serveripaddress: P0, fieldsmodified: u32, fglobalpolicy: bool, subnetaddress: DHCP_IP_ADDRESS, policyname: P4, policy: *const DHCP_POLICY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4SetPolicy(serveripaddress : windows_core::PCWSTR, fieldsmodified : u32, fglobalpolicy : windows_core::BOOL, subnetaddress : DHCP_IP_ADDRESS, policyname : windows_core::PCWSTR, policy : *const DHCP_POLICY) -> u32);
    unsafe { DhcpV4SetPolicy(serveripaddress.param().abi(), fieldsmodified, fglobalpolicy.into(), subnetaddress, policyname.param().abi(), policy) }
}
#[inline]
pub unsafe fn DhcpV4SetPolicyEnforcement<P0>(serveripaddress: P0, fglobalpolicy: bool, subnetaddress: DHCP_IP_ADDRESS, enable: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4SetPolicyEnforcement(serveripaddress : windows_core::PCWSTR, fglobalpolicy : windows_core::BOOL, subnetaddress : DHCP_IP_ADDRESS, enable : windows_core::BOOL) -> u32);
    unsafe { DhcpV4SetPolicyEnforcement(serveripaddress.param().abi(), fglobalpolicy.into(), subnetaddress, enable.into()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DhcpV4SetPolicyEx<P0, P4>(serveripaddress: P0, fieldsmodified: u32, globalpolicy: bool, subnetaddress: DHCP_IP_ADDRESS, policyname: P4, policy: *const DHCP_POLICY_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4SetPolicyEx(serveripaddress : windows_core::PCWSTR, fieldsmodified : u32, globalpolicy : windows_core::BOOL, subnetaddress : DHCP_IP_ADDRESS, policyname : windows_core::PCWSTR, policy : *const DHCP_POLICY_EX) -> u32);
    unsafe { DhcpV4SetPolicyEx(serveripaddress.param().abi(), fieldsmodified, globalpolicy.into(), subnetaddress, policyname.param().abi(), policy) }
}
#[inline]
pub unsafe fn DhcpV6CreateClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV6CreateClientInfo(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_V6) -> u32);
    unsafe { DhcpV6CreateClientInfo(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpV6GetFreeIPAddress<P0>(serveripaddress: P0, scopeid: DHCP_IPV6_ADDRESS, startip: DHCP_IPV6_ADDRESS, endip: DHCP_IPV6_ADDRESS, numfreeaddrreq: u32, ipaddrlist: *mut LPDHCPV6_IP_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV6GetFreeIPAddress(serveripaddress : windows_core::PCWSTR, scopeid : DHCP_IPV6_ADDRESS, startip : DHCP_IPV6_ADDRESS, endip : DHCP_IPV6_ADDRESS, numfreeaddrreq : u32, ipaddrlist : *mut LPDHCPV6_IP_ARRAY) -> u32);
    unsafe { DhcpV6GetFreeIPAddress(serveripaddress.param().abi(), core::mem::transmute(scopeid), core::mem::transmute(startip), core::mem::transmute(endip), numfreeaddrreq, ipaddrlist as _) }
}
#[inline]
pub unsafe fn DhcpV6GetStatelessStatistics<P0>(serveripaddress: P0, statelessstats: *mut LPDHCPV6_STATELESS_STATS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV6GetStatelessStatistics(serveripaddress : windows_core::PCWSTR, statelessstats : *mut LPDHCPV6_STATELESS_STATS) -> u32);
    unsafe { DhcpV6GetStatelessStatistics(serveripaddress.param().abi(), statelessstats as _) }
}
#[inline]
pub unsafe fn DhcpV6GetStatelessStoreParams<P0>(serveripaddress: P0, fserverlevel: bool, subnetaddress: DHCP_IPV6_ADDRESS, params: *mut LPDHCPV6_STATELESS_PARAMS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV6GetStatelessStoreParams(serveripaddress : windows_core::PCWSTR, fserverlevel : windows_core::BOOL, subnetaddress : DHCP_IPV6_ADDRESS, params : *mut LPDHCPV6_STATELESS_PARAMS) -> u32);
    unsafe { DhcpV6GetStatelessStoreParams(serveripaddress.param().abi(), fserverlevel.into(), core::mem::transmute(subnetaddress), params as _) }
}
#[inline]
pub unsafe fn DhcpV6SetStatelessStoreParams<P0>(serveripaddress: P0, fserverlevel: bool, subnetaddress: DHCP_IPV6_ADDRESS, fieldmodified: u32, params: *const DHCPV6_STATELESS_PARAMS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV6SetStatelessStoreParams(serveripaddress : windows_core::PCWSTR, fserverlevel : windows_core::BOOL, subnetaddress : DHCP_IPV6_ADDRESS, fieldmodified : u32, params : *const DHCPV6_STATELESS_PARAMS) -> u32);
    unsafe { DhcpV6SetStatelessStoreParams(serveripaddress.param().abi(), fserverlevel.into(), core::mem::transmute(subnetaddress), fieldmodified, params) }
}
pub const ADDRESS_TYPE_IANA: u32 = 0;
pub const ADDRESS_TYPE_IATA: u32 = 1;
pub const Allow: DHCP_FILTER_LIST_TYPE = 1;
pub const CHANGESTATE: u32 = 4;
pub const CLIENT_TYPE_BOOTP: u32 = 2;
pub const CLIENT_TYPE_BOTH: u32 = 3;
pub const CLIENT_TYPE_DHCP: u32 = 1;
pub const CLIENT_TYPE_NONE: u32 = 100;
pub const CLIENT_TYPE_RESERVATION_FLAG: u32 = 4;
pub const CLIENT_TYPE_UNSPECIFIED: u32 = 0;
pub const COMMUNICATION_INT: FSM_STATE = 4;
pub const CONFLICT_DONE: FSM_STATE = 7;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DATE_TIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
pub const DEFAULTQUARSETTING: QuarantineStatus = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCPDS_SERVER {
    pub Version: u32,
    pub ServerName: windows_core::PWSTR,
    pub ServerAddress: u32,
    pub Flags: u32,
    pub State: u32,
    pub DsLocation: windows_core::PWSTR,
    pub DsLocType: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCPDS_SERVERS {
    pub Flags: u32,
    pub NumElements: u32,
    pub Servers: LPDHCPDS_SERVER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCPV4_FAILOVER_CLIENT_INFO {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_core::BOOL,
    pub SentPotExpTime: u32,
    pub AckPotExpTime: u32,
    pub RecvPotExpTime: u32,
    pub StartTime: u32,
    pub CltLastTransTime: u32,
    pub LastBndUpdTime: u32,
    pub BndMsgStatus: u32,
    pub PolicyName: windows_core::PWSTR,
    pub Flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut LPDHCPV4_FAILOVER_CLIENT_INFO,
}
impl Default for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCPV4_FAILOVER_CLIENT_INFO_EX {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_core::BOOL,
    pub SentPotExpTime: u32,
    pub AckPotExpTime: u32,
    pub RecvPotExpTime: u32,
    pub StartTime: u32,
    pub CltLastTransTime: u32,
    pub LastBndUpdTime: u32,
    pub BndMsgStatus: u32,
    pub PolicyName: windows_core::PWSTR,
    pub Flags: u8,
    pub AddressStateEx: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCPV6_BIND_ELEMENT {
    pub Flags: u32,
    pub fBoundToDHCPServer: windows_core::BOOL,
    pub AdapterPrimaryAddress: DHCP_IPV6_ADDRESS,
    pub AdapterSubnetAddress: DHCP_IPV6_ADDRESS,
    pub IfDescription: windows_core::PWSTR,
    pub IpV6IfIndex: u32,
    pub IfIdSize: u32,
    pub IfId: super::minwindef::LPBYTE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCPV6_BIND_ELEMENT_ARRAY {
    pub NumElements: u32,
    pub Elements: LPDHCPV6_BIND_ELEMENT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCPV6_IP_ARRAY {
    pub NumElements: u32,
    pub Elements: LPDHCP_IPV6_ADDRESS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCPV6_STATELESS_PARAMS {
    pub Status: windows_core::BOOL,
    pub PurgeInterval: u32,
}
pub type DHCPV6_STATELESS_PARAM_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCPV6_STATELESS_SCOPE_STATS {
    pub SubnetAddress: DHCP_IPV6_ADDRESS,
    pub NumStatelessClientsAdded: u64,
    pub NumStatelessClientsRemoved: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCPV6_STATELESS_STATS {
    pub NumScopes: u32,
    pub ScopeStats: LPDHCPV6_STATELESS_SCOPE_STATS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_ADDR_PATTERN {
    pub MatchHWType: windows_core::BOOL,
    pub HWType: u8,
    pub IsWildcard: windows_core::BOOL,
    pub Length: u8,
    pub Pattern: [u8; 255],
}
impl Default for DHCP_ADDR_PATTERN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_ALL_OPTIONS {
    pub Flags: u32,
    pub NonVendorOptions: LPDHCP_OPTION_ARRAY,
    pub NumVendorOptions: u32,
    pub VendorOptions: *mut DHCP_ALL_OPTIONS_0,
}
impl Default for DHCP_ALL_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_ALL_OPTIONS_0 {
    pub Option: DHCP_OPTION,
    pub VendorName: windows_core::PWSTR,
    pub ClassName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_ALL_OPTION_VALUES {
    pub Flags: u32,
    pub NumElements: u32,
    pub Options: *mut DHCP_ALL_OPTION_VALUES_0,
}
impl Default for DHCP_ALL_OPTION_VALUES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_ALL_OPTION_VALUES_0 {
    pub ClassName: windows_core::PWSTR,
    pub VendorName: windows_core::PWSTR,
    pub IsVendor: windows_core::BOOL,
    pub OptionsArray: LPDHCP_OPTION_VALUE_ARRAY,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_ALL_OPTION_VALUES_PB {
    pub Flags: u32,
    pub NumElements: u32,
    pub Options: *mut DHCP_ALL_OPTION_VALUES_PB_0,
}
impl Default for DHCP_ALL_OPTION_VALUES_PB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_ALL_OPTION_VALUES_PB_0 {
    pub PolicyName: windows_core::PWSTR,
    pub VendorName: windows_core::PWSTR,
    pub IsVendor: windows_core::BOOL,
    pub OptionsArray: LPDHCP_OPTION_VALUE_ARRAY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_ATTRIB {
    pub DhcpAttribId: DHCP_ATTRIB_ID,
    pub DhcpAttribType: u32,
    pub Anonymous: DHCP_ATTRIB_0,
}
impl Default for DHCP_ATTRIB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_ATTRIB_0 {
    pub DhcpAttribBool: windows_core::BOOL,
    pub DhcpAttribUlong: u32,
}
impl Default for DHCP_ATTRIB_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_ATTRIB_ARRAY {
    pub NumElements: u32,
    pub DhcpAttribs: LPDHCP_ATTRIB,
}
pub const DHCP_ATTRIB_BOOL_IS_ADMIN: u32 = 5;
pub const DHCP_ATTRIB_BOOL_IS_BINDING_AWARE: u32 = 4;
pub const DHCP_ATTRIB_BOOL_IS_DYNBOOTP: u32 = 2;
pub const DHCP_ATTRIB_BOOL_IS_PART_OF_DSDC: u32 = 3;
pub const DHCP_ATTRIB_BOOL_IS_ROGUE: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DHCP_ATTRIB_ID(pub u32);
pub const DHCP_ATTRIB_TYPE_BOOL: u32 = 1;
pub const DHCP_ATTRIB_TYPE_ULONG: u32 = 2;
pub const DHCP_ATTRIB_ULONG_RESTORE_STATUS: u32 = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_BINARY_DATA {
    pub DataLength: u32,
    pub Data: *mut u8,
}
impl Default for DHCP_BINARY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_BIND_ELEMENT {
    pub Flags: u32,
    pub fBoundToDHCPServer: windows_core::BOOL,
    pub AdapterPrimaryAddress: DHCP_IP_ADDRESS,
    pub AdapterSubnetAddress: DHCP_IP_ADDRESS,
    pub IfDescription: windows_core::PWSTR,
    pub IfIdSize: u32,
    pub IfId: super::minwindef::LPBYTE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_BIND_ELEMENT_ARRAY {
    pub NumElements: u32,
    pub Elements: LPDHCP_BIND_ELEMENT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_BOOTP_IP_RANGE {
    pub StartAddress: DHCP_IP_ADDRESS,
    pub EndAddress: DHCP_IP_ADDRESS,
    pub BootpAllocated: u32,
    pub MaxBootpAllowed: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLASS_INFO {
    pub ClassName: windows_core::PWSTR,
    pub ClassComment: windows_core::PWSTR,
    pub ClassDataLength: u32,
    pub IsVendor: windows_core::BOOL,
    pub Flags: u32,
    pub ClassData: super::minwindef::LPBYTE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLASS_INFO_ARRAY {
    pub NumElements: u32,
    pub Classes: LPDHCP_CLASS_INFO,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLASS_INFO_ARRAY_V6 {
    pub NumElements: u32,
    pub Classes: LPDHCP_CLASS_INFO_V6,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLASS_INFO_V6 {
    pub ClassName: windows_core::PWSTR,
    pub ClassComment: windows_core::PWSTR,
    pub ClassDataLength: u32,
    pub IsVendor: windows_core::BOOL,
    pub EnterpriseNumber: u32,
    pub Flags: u32,
    pub ClassData: super::minwindef::LPBYTE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLIENT_FILTER_STATUS_INFO {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_core::BOOL,
    pub FilterStatus: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut LPDHCP_CLIENT_FILTER_STATUS_INFO,
}
impl Default for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLIENT_INFO {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_INFO_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut LPDHCP_CLIENT_INFO,
}
impl Default for DHCP_CLIENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_INFO_ARRAY_V4 {
    pub NumElements: u32,
    pub Clients: *mut LPDHCP_CLIENT_INFO_V4,
}
impl Default for DHCP_CLIENT_INFO_ARRAY_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_INFO_ARRAY_V5 {
    pub NumElements: u32,
    pub Clients: *mut LPDHCP_CLIENT_INFO_V5,
}
impl Default for DHCP_CLIENT_INFO_ARRAY_V5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_INFO_ARRAY_V6 {
    pub NumElements: u32,
    pub Clients: *mut LPDHCP_CLIENT_INFO_V6,
}
impl Default for DHCP_CLIENT_INFO_ARRAY_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_INFO_ARRAY_VQ {
    pub NumElements: u32,
    pub Clients: *mut LPDHCP_CLIENT_INFO_VQ,
}
impl Default for DHCP_CLIENT_INFO_ARRAY_VQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLIENT_INFO_EX {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_core::BOOL,
    pub FilterStatus: u32,
    pub PolicyName: windows_core::PWSTR,
    pub Properties: LPDHCP_PROPERTY_ARRAY,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_INFO_EX_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut LPDHCP_CLIENT_INFO_EX,
}
impl Default for DHCP_CLIENT_INFO_EX_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLIENT_INFO_PB {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_core::BOOL,
    pub FilterStatus: u32,
    pub PolicyName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_INFO_PB_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut LPDHCP_CLIENT_INFO_PB,
}
impl Default for DHCP_CLIENT_INFO_PB_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLIENT_INFO_V4 {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLIENT_INFO_V5 {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLIENT_INFO_V6 {
    pub ClientIpAddress: DHCP_IPV6_ADDRESS,
    pub ClientDUID: DHCP_CLIENT_UID,
    pub AddressType: u32,
    pub IAID: u32,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientValidLeaseExpires: DATE_TIME,
    pub ClientPrefLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO_V6,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLIENT_INFO_VQ {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_core::BOOL,
}
pub type DHCP_CLIENT_SEARCH_UNION = DHCP_SEARCH_INFO_0;
pub type DHCP_CLIENT_UID = DHCP_BINARY_DATA;
pub const DHCP_ENDPOINT_FLAG_CANT_MODIFY: u32 = 1;
pub const DHCP_FAILOVER_DELETE_SCOPES: u32 = 1;
pub const DHCP_FAILOVER_MAX_NUM_ADD_SCOPES: u32 = 400;
pub const DHCP_FAILOVER_MAX_NUM_REL: u32 = 31;
pub type DHCP_FAILOVER_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_FAILOVER_RELATIONSHIP {
    pub PrimaryServer: DHCP_IP_ADDRESS,
    pub SecondaryServer: DHCP_IP_ADDRESS,
    pub Mode: DHCP_FAILOVER_MODE,
    pub ServerType: DHCP_FAILOVER_SERVER,
    pub State: FSM_STATE,
    pub PrevState: FSM_STATE,
    pub Mclt: u32,
    pub SafePeriod: u32,
    pub RelationshipName: windows_core::PWSTR,
    pub PrimaryServerName: windows_core::PWSTR,
    pub SecondaryServerName: windows_core::PWSTR,
    pub pScopes: LPDHCP_IP_ARRAY,
    pub Percentage: u8,
    pub SharedSecret: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    pub NumElements: u32,
    pub pRelationships: LPDHCP_FAILOVER_RELATIONSHIP,
}
pub type DHCP_FAILOVER_SERVER = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_FAILOVER_STATISTICS {
    pub NumAddr: u32,
    pub AddrFree: u32,
    pub AddrInUse: u32,
    pub PartnerAddrFree: u32,
    pub ThisAddrFree: u32,
    pub PartnerAddrInUse: u32,
    pub ThisAddrInUse: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_FILTER_ADD_INFO {
    pub AddrPatt: DHCP_ADDR_PATTERN,
    pub Comment: windows_core::PWSTR,
    pub ListType: DHCP_FILTER_LIST_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_FILTER_ENUM_INFO {
    pub NumElements: u32,
    pub pEnumRecords: LPDHCP_FILTER_RECORD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_FILTER_GLOBAL_INFO {
    pub EnforceAllowList: windows_core::BOOL,
    pub EnforceDenyList: windows_core::BOOL,
}
pub type DHCP_FILTER_LIST_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_FILTER_RECORD {
    pub AddrPatt: DHCP_ADDR_PATTERN,
    pub Comment: windows_core::PWSTR,
}
pub const DHCP_FLAGS_DONT_ACCESS_DS: u32 = 1;
pub const DHCP_FLAGS_DONT_DO_RPC: u32 = 2;
pub const DHCP_FLAGS_OPTION_IS_VENDOR: u32 = 3;
pub type DHCP_FORCE_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_HOST_INFO {
    pub IpAddress: DHCP_IP_ADDRESS,
    pub NetBiosName: windows_core::PWSTR,
    pub HostName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_HOST_INFO_V6 {
    pub IpAddress: DHCP_IPV6_ADDRESS,
    pub NetBiosName: windows_core::PWSTR,
    pub HostName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_IPV6_ADDRESS {
    pub HighOrderBits: u64,
    pub LowOrderBits: u64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DHCP_IP_ADDRESS(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_IP_ARRAY {
    pub NumElements: u32,
    pub Elements: LPDHCP_IP_ADDRESS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_IP_CLUSTER {
    pub ClusterAddress: DHCP_IP_ADDRESS,
    pub ClusterMask: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DHCP_IP_MASK(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_IP_RANGE {
    pub StartAddress: DHCP_IP_ADDRESS,
    pub EndAddress: DHCP_IP_ADDRESS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_IP_RANGE_ARRAY {
    pub NumElements: u32,
    pub Elements: LPDHCP_IP_RANGE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_IP_RANGE_V6 {
    pub StartAddress: DHCP_IPV6_ADDRESS,
    pub EndAddress: DHCP_IPV6_ADDRESS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_IP_RESERVATION {
    pub ReservedIpAddress: DHCP_IP_ADDRESS,
    pub ReservedForClient: *mut DHCP_CLIENT_UID,
}
impl Default for DHCP_IP_RESERVATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_IP_RESERVATION_INFO {
    pub ReservedIpAddress: DHCP_IP_ADDRESS,
    pub ReservedForClient: DHCP_CLIENT_UID,
    pub ReservedClientName: windows_core::PWSTR,
    pub ReservedClientDesc: windows_core::PWSTR,
    pub bAllowedClientTypes: u8,
    pub fOptionsPresent: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_IP_RESERVATION_V4 {
    pub ReservedIpAddress: DHCP_IP_ADDRESS,
    pub ReservedForClient: *mut DHCP_CLIENT_UID,
    pub bAllowedClientTypes: u8,
}
impl Default for DHCP_IP_RESERVATION_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_IP_RESERVATION_V6 {
    pub ReservedIpAddress: DHCP_IPV6_ADDRESS,
    pub ReservedForClient: *mut DHCP_CLIENT_UID,
    pub InterfaceId: u32,
}
impl Default for DHCP_IP_RESERVATION_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DHCP_MAX_DELAY: u32 = 1000;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub ScopeInfo: LPSCOPE_MIB_INFO,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub ScopeInfo: LPSCOPE_MIB_INFO_V5,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub ScopeInfo: LPSCOPE_MIB_INFO_V6,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub ScopeInfo: LPSCOPE_MIB_INFO_VQ,
}
pub const DHCP_MIN_DELAY: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_OPTION {
    pub OptionID: DHCP_OPTION_ID,
    pub OptionName: windows_core::PWSTR,
    pub OptionComment: windows_core::PWSTR,
    pub DefaultValue: DHCP_OPTION_DATA,
    pub OptionType: DHCP_OPTION_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_OPTION_ARRAY {
    pub NumElements: u32,
    pub Options: LPDHCP_OPTION,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_OPTION_DATA {
    pub NumElements: u32,
    pub Elements: LPDHCP_OPTION_DATA_ELEMENT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_OPTION_DATA_ELEMENT {
    pub OptionType: DHCP_OPTION_DATA_TYPE,
    pub Element: DHCP_OPTION_DATA_ELEMENT_0,
}
impl Default for DHCP_OPTION_DATA_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_OPTION_DATA_ELEMENT_0 {
    pub ByteOption: u8,
    pub WordOption: u16,
    pub DWordOption: u32,
    pub DWordDWordOption: DWORD_DWORD,
    pub IpAddressOption: DHCP_IP_ADDRESS,
    pub StringDataOption: windows_core::PWSTR,
    pub BinaryDataOption: DHCP_BINARY_DATA,
    pub EncapsulatedDataOption: DHCP_BINARY_DATA,
    pub Ipv6AddressDataOption: windows_core::PWSTR,
}
impl Default for DHCP_OPTION_DATA_ELEMENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DHCP_OPTION_DATA_TYPE = i32;
pub type DHCP_OPTION_ELEMENT_UNION = DHCP_OPTION_DATA_ELEMENT_0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DHCP_OPTION_ID(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_OPTION_LIST {
    pub NumOptions: u32,
    pub Options: *mut DHCP_OPTION_VALUE,
}
impl Default for DHCP_OPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_OPTION_SCOPE_INFO {
    pub ScopeType: DHCP_OPTION_SCOPE_TYPE,
    pub ScopeInfo: DHCP_OPTION_SCOPE_INFO_0,
}
impl Default for DHCP_OPTION_SCOPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_OPTION_SCOPE_INFO6 {
    pub ScopeType: DHCP_OPTION_SCOPE_TYPE6,
    pub ScopeInfo: DHCP_OPTION_SCOPE_INFO6_0,
}
impl Default for DHCP_OPTION_SCOPE_INFO6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_OPTION_SCOPE_INFO6_0 {
    pub DefaultScopeInfo: *mut core::ffi::c_void,
    pub SubnetScopeInfo: DHCP_IPV6_ADDRESS,
    pub ReservedScopeInfo: DHCP_RESERVED_SCOPE6,
}
impl Default for DHCP_OPTION_SCOPE_INFO6_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_OPTION_SCOPE_INFO_0 {
    pub DefaultScopeInfo: *mut core::ffi::c_void,
    pub GlobalScopeInfo: *mut core::ffi::c_void,
    pub SubnetScopeInfo: DHCP_IP_ADDRESS,
    pub ReservedScopeInfo: DHCP_RESERVED_SCOPE,
    pub MScopeInfo: windows_core::PWSTR,
}
impl Default for DHCP_OPTION_SCOPE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DHCP_OPTION_SCOPE_TYPE = i32;
pub type DHCP_OPTION_SCOPE_TYPE6 = i32;
pub type DHCP_OPTION_SCOPE_UNION6 = DHCP_OPTION_SCOPE_INFO6_0;
pub type DHCP_OPTION_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_OPTION_VALUE {
    pub OptionID: DHCP_OPTION_ID,
    pub Value: DHCP_OPTION_DATA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_OPTION_VALUE_ARRAY {
    pub NumElements: u32,
    pub Values: LPDHCP_OPTION_VALUE,
}
pub const DHCP_OPT_ENUM_IGNORE_VENDOR: u32 = 1;
pub const DHCP_OPT_ENUM_USE_CLASSNAME: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_POLICY {
    pub PolicyName: windows_core::PWSTR,
    pub IsGlobalPolicy: windows_core::BOOL,
    pub Subnet: DHCP_IP_ADDRESS,
    pub ProcessingOrder: u32,
    pub Conditions: LPDHCP_POL_COND_ARRAY,
    pub Expressions: LPDHCP_POL_EXPR_ARRAY,
    pub Ranges: LPDHCP_IP_RANGE_ARRAY,
    pub Description: windows_core::PWSTR,
    pub Enabled: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_POLICY_ARRAY {
    pub NumElements: u32,
    pub Elements: LPDHCP_POLICY,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_POLICY_EX {
    pub PolicyName: windows_core::PWSTR,
    pub IsGlobalPolicy: windows_core::BOOL,
    pub Subnet: DHCP_IP_ADDRESS,
    pub ProcessingOrder: u32,
    pub Conditions: LPDHCP_POL_COND_ARRAY,
    pub Expressions: LPDHCP_POL_EXPR_ARRAY,
    pub Ranges: LPDHCP_IP_RANGE_ARRAY,
    pub Description: windows_core::PWSTR,
    pub Enabled: windows_core::BOOL,
    pub Properties: LPDHCP_PROPERTY_ARRAY,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_POLICY_EX_ARRAY {
    pub NumElements: u32,
    pub Elements: LPDHCP_POLICY_EX,
}
pub type DHCP_POLICY_FIELDS_TO_UPDATE = i32;
pub type DHCP_POL_ATTR_TYPE = i32;
pub type DHCP_POL_COMPARATOR = i32;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_POL_COND {
    pub ParentExpr: u32,
    pub Type: DHCP_POL_ATTR_TYPE,
    pub OptionID: u32,
    pub SubOptionID: u32,
    pub VendorName: windows_core::PWSTR,
    pub Operator: DHCP_POL_COMPARATOR,
    pub Value: super::minwindef::LPBYTE,
    pub ValueLength: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_POL_COND_ARRAY {
    pub NumElements: u32,
    pub Elements: LPDHCP_POL_COND,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_POL_EXPR {
    pub ParentExpr: u32,
    pub Operator: DHCP_POL_LOGIC_OPER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_POL_EXPR_ARRAY {
    pub NumElements: u32,
    pub Elements: LPDHCP_POL_EXPR,
}
pub type DHCP_POL_LOGIC_OPER = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_PROPERTY {
    pub ID: DHCP_PROPERTY_ID,
    pub Type: DHCP_PROPERTY_TYPE,
    pub Value: DHCP_PROPERTY_0,
}
impl Default for DHCP_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_PROPERTY_0 {
    pub ByteValue: u8,
    pub WordValue: u16,
    pub DWordValue: u32,
    pub StringValue: windows_core::PWSTR,
    pub BinaryValue: DHCP_BINARY_DATA,
}
impl Default for DHCP_PROPERTY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_PROPERTY_ARRAY {
    pub NumElements: u32,
    pub Elements: LPDHCP_PROPERTY,
}
pub type DHCP_PROPERTY_ID = i32;
pub type DHCP_PROPERTY_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_RESERVATION_INFO_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut LPDHCP_IP_RESERVATION_INFO,
}
impl Default for DHCP_RESERVATION_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_RESERVED_SCOPE {
    pub ReservedIpAddress: DHCP_IP_ADDRESS,
    pub ReservedIpSubnetAddress: DHCP_IP_ADDRESS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_RESERVED_SCOPE6 {
    pub ReservedIpAddress: DHCP_IPV6_ADDRESS,
    pub ReservedIpSubnetAddress: DHCP_IPV6_ADDRESS,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DHCP_RESUME_HANDLE(pub u32);
pub type DHCP_RESUME_IPV6_HANDLE = DHCP_IPV6_ADDRESS;
pub type DHCP_SCAN_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SCAN_ITEM {
    pub IpAddress: DHCP_IP_ADDRESS,
    pub ScanFlag: DHCP_SCAN_FLAG,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_SCAN_LIST {
    pub NumScanItems: u32,
    pub ScanItems: *mut DHCP_SCAN_ITEM,
}
impl Default for DHCP_SCAN_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_SEARCH_INFO {
    pub SearchType: DHCP_SEARCH_INFO_TYPE,
    pub SearchInfo: DHCP_SEARCH_INFO_0,
}
impl Default for DHCP_SEARCH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_SEARCH_INFO_0 {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_core::PWSTR,
}
impl Default for DHCP_SEARCH_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DHCP_SEARCH_INFO_TYPE = i32;
pub type DHCP_SEARCH_INFO_TYPE_V6 = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_SEARCH_INFO_V6 {
    pub SearchType: DHCP_SEARCH_INFO_TYPE_V6,
    pub SearchInfo: DHCP_SEARCH_INFO_V6_0,
}
impl Default for DHCP_SEARCH_INFO_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_SEARCH_INFO_V6_0 {
    pub ClientIpAddress: DHCP_IPV6_ADDRESS,
    pub ClientDUID: DHCP_CLIENT_UID,
    pub ClientName: windows_core::PWSTR,
}
impl Default for DHCP_SEARCH_INFO_V6_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SERVER_CONFIG_INFO {
    pub APIProtocolSupport: u32,
    pub DatabaseName: windows_core::PWSTR,
    pub DatabasePath: windows_core::PWSTR,
    pub BackupPath: windows_core::PWSTR,
    pub BackupInterval: u32,
    pub DatabaseLoggingFlag: u32,
    pub RestoreFlag: u32,
    pub DatabaseCleanupInterval: u32,
    pub DebugFlag: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_SERVER_CONFIG_INFO_V4 {
    pub APIProtocolSupport: u32,
    pub DatabaseName: windows_core::PWSTR,
    pub DatabasePath: windows_core::PWSTR,
    pub BackupPath: windows_core::PWSTR,
    pub BackupInterval: u32,
    pub DatabaseLoggingFlag: u32,
    pub RestoreFlag: u32,
    pub DatabaseCleanupInterval: u32,
    pub DebugFlag: u32,
    pub dwPingRetries: u32,
    pub cbBootTableString: u32,
    pub wszBootTableString: *mut u16,
    pub fAuditLog: windows_core::BOOL,
}
impl Default for DHCP_SERVER_CONFIG_INFO_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SERVER_CONFIG_INFO_V6 {
    pub UnicastFlag: windows_core::BOOL,
    pub RapidCommitFlag: windows_core::BOOL,
    pub PreferredLifetime: u32,
    pub ValidLifetime: u32,
    pub T1: u32,
    pub T2: u32,
    pub PreferredLifetimeIATA: u32,
    pub ValidLifetimeIATA: u32,
    pub fAuditLog: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_SERVER_CONFIG_INFO_VQ {
    pub APIProtocolSupport: u32,
    pub DatabaseName: windows_core::PWSTR,
    pub DatabasePath: windows_core::PWSTR,
    pub BackupPath: windows_core::PWSTR,
    pub BackupInterval: u32,
    pub DatabaseLoggingFlag: u32,
    pub RestoreFlag: u32,
    pub DatabaseCleanupInterval: u32,
    pub DebugFlag: u32,
    pub dwPingRetries: u32,
    pub cbBootTableString: u32,
    pub wszBootTableString: *mut u16,
    pub fAuditLog: windows_core::BOOL,
    pub QuarantineOn: windows_core::BOOL,
    pub QuarDefFail: u32,
    pub QuarRuntimeStatus: windows_core::BOOL,
}
impl Default for DHCP_SERVER_CONFIG_INFO_VQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DHCP_SERVER_INFO = DHCPDS_SERVER;
pub type DHCP_SERVER_INFO_ARRAY = DHCPDS_SERVERS;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SERVER_SPECIFIC_STRINGS {
    pub DefaultVendorClassName: windows_core::PWSTR,
    pub DefaultUserClassName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_SUBNET_ELEMENT_DATA {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_0,
}
impl Default for DHCP_SUBNET_ELEMENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_SUBNET_ELEMENT_DATA_0 {
    pub IpRange: *mut DHCP_IP_RANGE,
    pub SecondaryHost: *mut DHCP_HOST_INFO,
    pub ReservedIp: *mut DHCP_IP_RESERVATION,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE,
    pub IpUsedCluster: *mut DHCP_IP_CLUSTER,
}
impl Default for DHCP_SUBNET_ELEMENT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_SUBNET_ELEMENT_DATA_V4 {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_V4_0,
}
impl Default for DHCP_SUBNET_ELEMENT_DATA_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_SUBNET_ELEMENT_DATA_V4_0 {
    pub IpRange: *mut DHCP_IP_RANGE,
    pub SecondaryHost: *mut DHCP_HOST_INFO,
    pub ReservedIp: *mut DHCP_IP_RESERVATION_V4,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE,
    pub IpUsedCluster: *mut DHCP_IP_CLUSTER,
}
impl Default for DHCP_SUBNET_ELEMENT_DATA_V4_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_SUBNET_ELEMENT_DATA_V5 {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_V5_0,
}
impl Default for DHCP_SUBNET_ELEMENT_DATA_V5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_SUBNET_ELEMENT_DATA_V5_0 {
    pub IpRange: *mut DHCP_BOOTP_IP_RANGE,
    pub SecondaryHost: *mut DHCP_HOST_INFO,
    pub ReservedIp: *mut DHCP_IP_RESERVATION_V4,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE,
    pub IpUsedCluster: *mut DHCP_IP_CLUSTER,
}
impl Default for DHCP_SUBNET_ELEMENT_DATA_V5_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_SUBNET_ELEMENT_DATA_V6 {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE_V6,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_V6_0,
}
impl Default for DHCP_SUBNET_ELEMENT_DATA_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    pub IpRange: *mut DHCP_IP_RANGE_V6,
    pub ReservedIp: *mut DHCP_IP_RESERVATION_V6,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE_V6,
}
impl Default for DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    pub NumElements: u32,
    pub Elements: LPDHCP_SUBNET_ELEMENT_DATA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    pub NumElements: u32,
    pub Elements: LPDHCP_SUBNET_ELEMENT_DATA_V4,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    pub NumElements: u32,
    pub Elements: LPDHCP_SUBNET_ELEMENT_DATA_V5,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    pub NumElements: u32,
    pub Elements: LPDHCP_SUBNET_ELEMENT_DATA_V6,
}
pub type DHCP_SUBNET_ELEMENT_TYPE = i32;
pub type DHCP_SUBNET_ELEMENT_TYPE_V6 = i32;
pub type DHCP_SUBNET_ELEMENT_UNION = DHCP_SUBNET_ELEMENT_DATA_0;
pub type DHCP_SUBNET_ELEMENT_UNION_V4 = DHCP_SUBNET_ELEMENT_DATA_V4_0;
pub type DHCP_SUBNET_ELEMENT_UNION_V6 = DHCP_SUBNET_ELEMENT_DATA_V6_0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SUBNET_INFO {
    pub SubnetAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub SubnetName: windows_core::PWSTR,
    pub SubnetComment: windows_core::PWSTR,
    pub PrimaryHost: DHCP_HOST_INFO,
    pub SubnetState: DHCP_SUBNET_STATE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SUBNET_INFO_V6 {
    pub SubnetAddress: DHCP_IPV6_ADDRESS,
    pub Prefix: u32,
    pub Preference: u16,
    pub SubnetName: windows_core::PWSTR,
    pub SubnetComment: windows_core::PWSTR,
    pub State: u32,
    pub ScopeId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SUBNET_INFO_VQ {
    pub SubnetAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub SubnetName: windows_core::PWSTR,
    pub SubnetComment: windows_core::PWSTR,
    pub PrimaryHost: DHCP_HOST_INFO,
    pub SubnetState: DHCP_SUBNET_STATE,
    pub QuarantineOn: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: i64,
    pub Reserved4: i64,
}
pub const DHCP_SUBNET_INFO_VQ_FLAG_QUARANTINE: u32 = 1;
pub type DHCP_SUBNET_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_SUPER_SCOPE_TABLE {
    pub cEntries: u32,
    pub pEntries: *mut DHCP_SUPER_SCOPE_TABLE_ENTRY,
}
impl Default for DHCP_SUPER_SCOPE_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SUPER_SCOPE_TABLE_ENTRY {
    pub SubnetAddress: DHCP_IP_ADDRESS,
    pub SuperScopeNumber: u32,
    pub NextInSuperScope: u32,
    pub SuperScopeName: windows_core::PWSTR,
}
pub const DNS_FLAG_CLEANUP_EXPIRED: u32 = 4;
pub const DNS_FLAG_DISABLE_PTR_UPDATE: u32 = 64;
pub const DNS_FLAG_ENABLED: u32 = 1;
pub const DNS_FLAG_HAS_DNS_SUFFIX: u32 = 128;
pub const DNS_FLAG_UPDATE_BOTH_ALWAYS: u32 = 16;
pub const DNS_FLAG_UPDATE_DHCID: u32 = 32;
pub const DNS_FLAG_UPDATE_DOWNLEVEL: u32 = 2;
pub const DROPPACKET: QuarantineStatus = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWORD_DWORD {
    pub DWord1: u32,
    pub DWord2: u32,
}
pub const Deny: DHCP_FILTER_LIST_TYPE = 0;
pub const DhcpArrayTypeOption: DHCP_OPTION_TYPE = 1;
pub const DhcpAttrFqdn: DHCP_POL_ATTR_TYPE = 3;
pub const DhcpAttrFqdnSingleLabel: DHCP_POL_ATTR_TYPE = 4;
pub const DhcpAttrHWAddr: DHCP_POL_ATTR_TYPE = 0;
pub const DhcpAttrOption: DHCP_POL_ATTR_TYPE = 1;
pub const DhcpAttrSubOption: DHCP_POL_ATTR_TYPE = 2;
pub const DhcpBinaryDataOption: DHCP_OPTION_DATA_TYPE = 6;
pub const DhcpByteOption: DHCP_OPTION_DATA_TYPE = 0;
pub const DhcpClientHardwareAddress: DHCP_SEARCH_INFO_TYPE = 1;
pub const DhcpClientIpAddress: DHCP_SEARCH_INFO_TYPE = 0;
pub const DhcpClientName: DHCP_SEARCH_INFO_TYPE = 2;
pub const DhcpCompBeginsWith: DHCP_POL_COMPARATOR = 2;
pub const DhcpCompEndsWith: DHCP_POL_COMPARATOR = 4;
pub const DhcpCompEqual: DHCP_POL_COMPARATOR = 0;
pub const DhcpCompNotBeginWith: DHCP_POL_COMPARATOR = 3;
pub const DhcpCompNotEndWith: DHCP_POL_COMPARATOR = 5;
pub const DhcpCompNotEqual: DHCP_POL_COMPARATOR = 1;
pub const DhcpDWordDWordOption: DHCP_OPTION_DATA_TYPE = 3;
pub const DhcpDWordOption: DHCP_OPTION_DATA_TYPE = 2;
pub const DhcpDatabaseFix: DHCP_SCAN_FLAG = 1;
pub const DhcpDefaultOptions: DHCP_OPTION_SCOPE_TYPE = 0;
pub const DhcpDefaultOptions6: DHCP_OPTION_SCOPE_TYPE6 = 0;
pub const DhcpEncapsulatedDataOption: DHCP_OPTION_DATA_TYPE = 7;
pub const DhcpExcludedIpRanges: DHCP_SUBNET_ELEMENT_TYPE = 3;
pub const DhcpFailoverForce: DHCP_FORCE_FLAG = 2;
pub const DhcpFullForce: DHCP_FORCE_FLAG = 0;
pub const DhcpGlobalOptions: DHCP_OPTION_SCOPE_TYPE = 1;
pub const DhcpGlobalOptions6: DHCP_OPTION_SCOPE_TYPE6 = 3;
pub const DhcpIpAddressOption: DHCP_OPTION_DATA_TYPE = 4;
pub const DhcpIpRanges: DHCP_SUBNET_ELEMENT_TYPE = 0;
pub const DhcpIpRangesBootpOnly: DHCP_SUBNET_ELEMENT_TYPE = 7;
pub const DhcpIpRangesDhcpBootp: DHCP_SUBNET_ELEMENT_TYPE = 6;
pub const DhcpIpRangesDhcpOnly: DHCP_SUBNET_ELEMENT_TYPE = 5;
pub const DhcpIpUsedClusters: DHCP_SUBNET_ELEMENT_TYPE = 4;
pub const DhcpIpv6AddressOption: DHCP_OPTION_DATA_TYPE = 8;
pub const DhcpLogicalAnd: DHCP_POL_LOGIC_OPER = 1;
pub const DhcpLogicalOr: DHCP_POL_LOGIC_OPER = 0;
pub const DhcpMScopeOptions: DHCP_OPTION_SCOPE_TYPE = 4;
pub const DhcpNoForce: DHCP_FORCE_FLAG = 1;
pub const DhcpPropIdClientAddressStateEx: DHCP_PROPERTY_ID = 1;
pub const DhcpPropIdPolicyDnsSuffix: DHCP_PROPERTY_ID = 0;
pub const DhcpPropTypeBinary: DHCP_PROPERTY_TYPE = 4;
pub const DhcpPropTypeByte: DHCP_PROPERTY_TYPE = 0;
pub const DhcpPropTypeDword: DHCP_PROPERTY_TYPE = 2;
pub const DhcpPropTypeString: DHCP_PROPERTY_TYPE = 3;
pub const DhcpPropTypeWord: DHCP_PROPERTY_TYPE = 1;
pub const DhcpRegistryFix: DHCP_SCAN_FLAG = 0;
pub const DhcpReservedIps: DHCP_SUBNET_ELEMENT_TYPE = 2;
pub const DhcpReservedOptions: DHCP_OPTION_SCOPE_TYPE = 3;
pub const DhcpReservedOptions6: DHCP_OPTION_SCOPE_TYPE6 = 2;
pub const DhcpScopeOptions6: DHCP_OPTION_SCOPE_TYPE6 = 1;
pub const DhcpSecondaryHosts: DHCP_SUBNET_ELEMENT_TYPE = 1;
pub const DhcpStatelessPurgeInterval: DHCPV6_STATELESS_PARAM_TYPE = 1;
pub const DhcpStatelessStatus: DHCPV6_STATELESS_PARAM_TYPE = 2;
pub const DhcpStringDataOption: DHCP_OPTION_DATA_TYPE = 5;
pub const DhcpSubnetDisabled: DHCP_SUBNET_STATE = 1;
pub const DhcpSubnetDisabledSwitched: DHCP_SUBNET_STATE = 3;
pub const DhcpSubnetEnabled: DHCP_SUBNET_STATE = 0;
pub const DhcpSubnetEnabledSwitched: DHCP_SUBNET_STATE = 2;
pub const DhcpSubnetInvalidState: DHCP_SUBNET_STATE = 4;
pub const DhcpSubnetOptions: DHCP_OPTION_SCOPE_TYPE = 2;
pub const DhcpUnaryElementTypeOption: DHCP_OPTION_TYPE = 0;
pub const DhcpUpdatePolicyDescr: DHCP_POLICY_FIELDS_TO_UPDATE = 16;
pub const DhcpUpdatePolicyDnsSuffix: DHCP_POLICY_FIELDS_TO_UPDATE = 64;
pub const DhcpUpdatePolicyExpr: DHCP_POLICY_FIELDS_TO_UPDATE = 4;
pub const DhcpUpdatePolicyName: DHCP_POLICY_FIELDS_TO_UPDATE = 1;
pub const DhcpUpdatePolicyOrder: DHCP_POLICY_FIELDS_TO_UPDATE = 2;
pub const DhcpUpdatePolicyRanges: DHCP_POLICY_FIELDS_TO_UPDATE = 8;
pub const DhcpUpdatePolicyStatus: DHCP_POLICY_FIELDS_TO_UPDATE = 32;
pub const DhcpWordOption: DHCP_OPTION_DATA_TYPE = 1;
pub const Dhcpv6ClientDUID: DHCP_SEARCH_INFO_TYPE_V6 = 1;
pub const Dhcpv6ClientIpAddress: DHCP_SEARCH_INFO_TYPE_V6 = 0;
pub const Dhcpv6ClientName: DHCP_SEARCH_INFO_TYPE_V6 = 2;
pub const Dhcpv6ExcludedIpRanges: DHCP_SUBNET_ELEMENT_TYPE_V6 = 2;
pub const Dhcpv6IpRanges: DHCP_SUBNET_ELEMENT_TYPE_V6 = 0;
pub const Dhcpv6ReservedIps: DHCP_SUBNET_ELEMENT_TYPE_V6 = 1;
pub const ERROR_DDS_CLASS_DOES_NOT_EXIST: u32 = 20078;
pub const ERROR_DDS_CLASS_EXISTS: u32 = 20077;
pub const ERROR_DDS_DHCP_SERVER_NOT_FOUND: u32 = 20074;
pub const ERROR_DDS_NO_DHCP_ROOT: u32 = 20071;
pub const ERROR_DDS_NO_DS_AVAILABLE: u32 = 20070;
pub const ERROR_DDS_OPTION_ALREADY_EXISTS: u32 = 20075;
pub const ERROR_DDS_OPTION_DOES_NOT_EXIST: u32 = 20076;
pub const ERROR_DDS_POSSIBLE_RANGE_CONFLICT: u32 = 20087;
pub const ERROR_DDS_RANGE_DOES_NOT_EXIST: u32 = 20088;
pub const ERROR_DDS_RESERVATION_CONFLICT: u32 = 20086;
pub const ERROR_DDS_RESERVATION_NOT_PRESENT: u32 = 20085;
pub const ERROR_DDS_SERVER_ADDRESS_MISMATCH: u32 = 20081;
pub const ERROR_DDS_SERVER_ALREADY_EXISTS: u32 = 20079;
pub const ERROR_DDS_SERVER_DOES_NOT_EXIST: u32 = 20080;
pub const ERROR_DDS_SUBNET_EXISTS: u32 = 20082;
pub const ERROR_DDS_SUBNET_HAS_DIFF_SSCOPE: u32 = 20083;
pub const ERROR_DDS_SUBNET_NOT_PRESENT: u32 = 20084;
pub const ERROR_DDS_TOO_MANY_ERRORS: u32 = 20073;
pub const ERROR_DDS_UNEXPECTED_ERROR: u32 = 20072;
pub const ERROR_DHCP_ADDRESS_NOT_AVAILABLE: u32 = 20011;
pub const ERROR_DHCP_CANNOT_MODIFY_BINDINGS: u32 = 20051;
pub const ERROR_DHCP_CANT_CHANGE_ATTRIBUTE: u32 = 20048;
pub const ERROR_DHCP_CLASS_ALREADY_EXISTS: u32 = 20045;
pub const ERROR_DHCP_CLASS_NOT_FOUND: u32 = 20044;
pub const ERROR_DHCP_CLIENT_EXISTS: u32 = 20014;
pub const ERROR_DHCP_DATABASE_INIT_FAILED: u32 = 20001;
pub const ERROR_DHCP_DEFAULT_SCOPE_EXITS: u32 = 20047;
pub const ERROR_DHCP_DELETE_BUILTIN_CLASS: u32 = 20089;
pub const ERROR_DHCP_ELEMENT_CANT_REMOVE: u32 = 20007;
pub const ERROR_DHCP_EXEMPTION_EXISTS: u32 = 20055;
pub const ERROR_DHCP_EXEMPTION_NOT_PRESENT: u32 = 20056;
pub const ERROR_DHCP_FO_ADDSCOPE_LEASES_NOT_SYNCED: u32 = 20127;
pub const ERROR_DHCP_FO_BOOT_NOT_SUPPORTED: u32 = 20131;
pub const ERROR_DHCP_FO_FEATURE_NOT_SUPPORTED: u32 = 20134;
pub const ERROR_DHCP_FO_IPRANGE_TYPE_CONV_ILLEGAL: u32 = 20129;
pub const ERROR_DHCP_FO_MAX_ADD_SCOPES: u32 = 20130;
pub const ERROR_DHCP_FO_MAX_RELATIONSHIPS: u32 = 20128;
pub const ERROR_DHCP_FO_NOT_SUPPORTED: u32 = 20118;
pub const ERROR_DHCP_FO_RANGE_PART_OF_REL: u32 = 20132;
pub const ERROR_DHCP_FO_RELATIONSHIP_DOES_NOT_EXIST: u32 = 20115;
pub const ERROR_DHCP_FO_RELATIONSHIP_EXISTS: u32 = 20114;
pub const ERROR_DHCP_FO_RELATIONSHIP_NAME_TOO_LONG: u32 = 20125;
pub const ERROR_DHCP_FO_RELATION_IS_SECONDARY: u32 = 20117;
pub const ERROR_DHCP_FO_SCOPE_ALREADY_IN_RELATIONSHIP: u32 = 20113;
pub const ERROR_DHCP_FO_SCOPE_NOT_IN_RELATIONSHIP: u32 = 20116;
pub const ERROR_DHCP_FO_SCOPE_SYNC_IN_PROGRESS: u32 = 20133;
pub const ERROR_DHCP_FO_STATE_NOT_NORMAL: u32 = 20120;
pub const ERROR_DHCP_FO_TIME_OUT_OF_SYNC: u32 = 20119;
pub const ERROR_DHCP_HARDWARE_ADDRESS_TYPE_ALREADY_EXEMPT: u32 = 20101;
pub const ERROR_DHCP_INVALID_DELAY: u32 = 20092;
pub const ERROR_DHCP_INVALID_DHCP_CLIENT: u32 = 20016;
pub const ERROR_DHCP_INVALID_DHCP_MESSAGE: u32 = 20015;
pub const ERROR_DHCP_INVALID_PARAMETER_OPTION32: u32 = 20057;
pub const ERROR_DHCP_INVALID_POLICY_EXPRESSION: u32 = 20109;
pub const ERROR_DHCP_INVALID_PROCESSING_ORDER: u32 = 20110;
pub const ERROR_DHCP_INVALID_RANGE: u32 = 20023;
pub const ERROR_DHCP_INVALID_SUBNET_PREFIX: u32 = 20091;
pub const ERROR_DHCP_IPRANGE_CONV_ILLEGAL: u32 = 20049;
pub const ERROR_DHCP_IPRANGE_EXITS: u32 = 20021;
pub const ERROR_DHCP_IP_ADDRESS_IN_USE: u32 = 20032;
pub const ERROR_DHCP_JET97_CONV_REQUIRED: u32 = 20036;
pub const ERROR_DHCP_JET_CONV_REQUIRED: u32 = 20027;
pub const ERROR_DHCP_JET_ERROR: u32 = 20013;
pub const ERROR_DHCP_LINKLAYER_ADDRESS_DOES_NOT_EXIST: u32 = 20095;
pub const ERROR_DHCP_LINKLAYER_ADDRESS_EXISTS: u32 = 20093;
pub const ERROR_DHCP_LINKLAYER_ADDRESS_RESERVATION_EXISTS: u32 = 20094;
pub const ERROR_DHCP_LOG_FILE_PATH_TOO_LONG: u32 = 20033;
pub const ERROR_DHCP_MSCOPE_EXISTS: u32 = 20053;
pub const ERROR_DHCP_NAP_NOT_SUPPORTED: u32 = 20138;
pub const ERROR_DHCP_NETWORK_CHANGED: u32 = 20050;
pub const ERROR_DHCP_NETWORK_INIT_FAILED: u32 = 20003;
pub const ERROR_DHCP_NOT_RESERVED_CLIENT: u32 = 20018;
pub const ERROR_DHCP_NO_ADMIN_PERMISSION: u32 = 20121;
pub const ERROR_DHCP_OPTION_EXITS: u32 = 20009;
pub const ERROR_DHCP_OPTION_NOT_PRESENT: u32 = 20010;
pub const ERROR_DHCP_OPTION_TYPE_MISMATCH: u32 = 20103;
pub const ERROR_DHCP_POLICY_BAD_PARENT_EXPR: u32 = 20104;
pub const ERROR_DHCP_POLICY_EDIT_FQDN_UNSUPPORTED: u32 = 20137;
pub const ERROR_DHCP_POLICY_EXISTS: u32 = 20105;
pub const ERROR_DHCP_POLICY_FQDN_OPTION_UNSUPPORTED: u32 = 20136;
pub const ERROR_DHCP_POLICY_FQDN_RANGE_UNSUPPORTED: u32 = 20135;
pub const ERROR_DHCP_POLICY_NOT_FOUND: u32 = 20111;
pub const ERROR_DHCP_POLICY_RANGE_BAD: u32 = 20107;
pub const ERROR_DHCP_POLICY_RANGE_EXISTS: u32 = 20106;
pub const ERROR_DHCP_PRIMARY_NOT_FOUND: u32 = 20006;
pub const ERROR_DHCP_RANGE_EXTENDED: u32 = 20024;
pub const ERROR_DHCP_RANGE_FULL: u32 = 20012;
pub const ERROR_DHCP_RANGE_INVALID_IN_SERVER_POLICY: u32 = 20108;
pub const ERROR_DHCP_RANGE_TOO_SMALL: u32 = 20020;
pub const ERROR_DHCP_REACHED_END_OF_SELECTION: u32 = 20126;
pub const ERROR_DHCP_REGISTRY_INIT_FAILED: u32 = 20000;
pub const ERROR_DHCP_RESERVEDIP_EXITS: u32 = 20022;
pub const ERROR_DHCP_RESERVED_CLIENT: u32 = 20019;
pub const ERROR_DHCP_ROGUE_DS_CONFLICT: u32 = 20041;
pub const ERROR_DHCP_ROGUE_DS_UNREACHABLE: u32 = 20040;
pub const ERROR_DHCP_ROGUE_INIT_FAILED: u32 = 20037;
pub const ERROR_DHCP_ROGUE_NOT_AUTHORIZED: u32 = 20039;
pub const ERROR_DHCP_ROGUE_NOT_OUR_ENTERPRISE: u32 = 20042;
pub const ERROR_DHCP_ROGUE_SAMSHUTDOWN: u32 = 20038;
pub const ERROR_DHCP_ROGUE_STANDALONE_IN_DS: u32 = 20043;
pub const ERROR_DHCP_RPC_INIT_FAILED: u32 = 20002;
pub const ERROR_DHCP_SCOPE_NAME_TOO_LONG: u32 = 20046;
pub const ERROR_DHCP_SERVER_NAME_NOT_RESOLVED: u32 = 20124;
pub const ERROR_DHCP_SERVER_NOT_REACHABLE: u32 = 20122;
pub const ERROR_DHCP_SERVER_NOT_RUNNING: u32 = 20123;
pub const ERROR_DHCP_SERVICE_PAUSED: u32 = 20017;
pub const ERROR_DHCP_SUBNET_EXISTS: u32 = 20052;
pub const ERROR_DHCP_SUBNET_EXITS: u32 = 20004;
pub const ERROR_DHCP_SUBNET_NOT_PRESENT: u32 = 20005;
pub const ERROR_DHCP_SUPER_SCOPE_NAME_TOO_LONG: u32 = 20030;
pub const ERROR_DHCP_UNDEFINED_HARDWARE_ADDRESS_TYPE: u32 = 20102;
pub const ERROR_DHCP_UNSUPPORTED_CLIENT: u32 = 20034;
pub const ERROR_EXTEND_TOO_SMALL: u32 = 20025;
pub const ERROR_LAST_DHCP_SERVER_ERROR: u32 = 20139;
pub const ERROR_MSCOPE_RANGE_TOO_SMALL: u32 = 20054;
pub const ERROR_SCOPE_RANGE_POLICY_RANGE_CONFLICT: u32 = 20112;
pub const ERROR_SERVER_INVALID_BOOT_FILE_TABLE: u32 = 20028;
pub const ERROR_SERVER_UNKNOWN_BOOT_FILE_NAME: u32 = 20029;
pub const EXEMPT: QuarantineStatus = 4;
pub const FILTER_STATUS_FULL_MATCH_IN_ALLOW_LIST: u32 = 2;
pub const FILTER_STATUS_FULL_MATCH_IN_DENY_LIST: u32 = 4;
pub const FILTER_STATUS_NONE: u32 = 1;
pub const FILTER_STATUS_WILDCARD_MATCH_IN_ALLOW_LIST: u32 = 8;
pub const FILTER_STATUS_WILDCARD_MATCH_IN_DENY_LIST: u32 = 16;
pub type FSM_STATE = i32;
pub const HWTYPE_ETHERNET_10MB: u32 = 1;
pub const HotStandby: DHCP_FAILOVER_MODE = 1;
pub const INIT: FSM_STATE = 1;
pub type LPDATE_TIME = *mut DATE_TIME;
pub type LPDHCPDS_SERVER = *mut DHCPDS_SERVER;
pub type LPDHCPDS_SERVERS = *mut DHCPDS_SERVERS;
pub type LPDHCPV4_FAILOVER_CLIENT_INFO = *mut DHCPV4_FAILOVER_CLIENT_INFO;
pub type LPDHCPV4_FAILOVER_CLIENT_INFO_ARRAY = *mut DHCPV4_FAILOVER_CLIENT_INFO_ARRAY;
pub type LPDHCPV4_FAILOVER_CLIENT_INFO_EX = *mut DHCPV4_FAILOVER_CLIENT_INFO_EX;
#[cfg(feature = "minwindef")]
pub type LPDHCPV6_BIND_ELEMENT = *mut DHCPV6_BIND_ELEMENT;
#[cfg(feature = "minwindef")]
pub type LPDHCPV6_BIND_ELEMENT_ARRAY = *mut DHCPV6_BIND_ELEMENT_ARRAY;
pub type LPDHCPV6_IP_ARRAY = *mut DHCPV6_IP_ARRAY;
pub type LPDHCPV6_STATELESS_PARAMS = *mut DHCPV6_STATELESS_PARAMS;
pub type LPDHCPV6_STATELESS_SCOPE_STATS = *mut DHCPV6_STATELESS_SCOPE_STATS;
pub type LPDHCPV6_STATELESS_STATS = *mut DHCPV6_STATELESS_STATS;
pub type LPDHCP_ADDR_PATTERN = *mut DHCP_ADDR_PATTERN;
pub type LPDHCP_ALL_OPTIONS = *mut DHCP_ALL_OPTIONS;
pub type LPDHCP_ALL_OPTION_VALUES = *mut DHCP_ALL_OPTION_VALUES;
pub type LPDHCP_ALL_OPTION_VALUES_PB = *mut DHCP_ALL_OPTION_VALUES_PB;
pub type LPDHCP_ATTRIB = *mut DHCP_ATTRIB;
pub type LPDHCP_ATTRIB_ARRAY = *mut DHCP_ATTRIB_ARRAY;
pub type LPDHCP_ATTRIB_ID = *mut u32;
pub type LPDHCP_BINARY_DATA = *mut DHCP_BINARY_DATA;
#[cfg(feature = "minwindef")]
pub type LPDHCP_BIND_ELEMENT = *mut DHCP_BIND_ELEMENT;
#[cfg(feature = "minwindef")]
pub type LPDHCP_BIND_ELEMENT_ARRAY = *mut DHCP_BIND_ELEMENT_ARRAY;
pub type LPDHCP_BOOT_IP_RANGE = *mut DHCP_BOOTP_IP_RANGE;
#[cfg(feature = "minwindef")]
pub type LPDHCP_CLASS_INFO = *mut DHCP_CLASS_INFO;
#[cfg(feature = "minwindef")]
pub type LPDHCP_CLASS_INFO_ARRAY = *mut DHCP_CLASS_INFO_ARRAY;
#[cfg(feature = "minwindef")]
pub type LPDHCP_CLASS_INFO_ARRAY_V6 = *mut DHCP_CLASS_INFO_ARRAY_V6;
#[cfg(feature = "minwindef")]
pub type LPDHCP_CLASS_INFO_V6 = *mut DHCP_CLASS_INFO_V6;
pub type LPDHCP_CLIENT_FILTER_STATUS_INFO = *mut DHCP_CLIENT_FILTER_STATUS_INFO;
pub type LPDHCP_CLIENT_FILTER_STATUS_INFO_ARRAY = *mut DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY;
pub type LPDHCP_CLIENT_INFO = *mut DHCP_CLIENT_INFO;
pub type LPDHCP_CLIENT_INFO_ARRAY = *mut DHCP_CLIENT_INFO_ARRAY;
pub type LPDHCP_CLIENT_INFO_ARRAY_V4 = *mut DHCP_CLIENT_INFO_ARRAY_V4;
pub type LPDHCP_CLIENT_INFO_ARRAY_V5 = *mut DHCP_CLIENT_INFO_ARRAY_V5;
pub type LPDHCP_CLIENT_INFO_ARRAY_V6 = *mut DHCP_CLIENT_INFO_ARRAY_V6;
pub type LPDHCP_CLIENT_INFO_ARRAY_VQ = *mut DHCP_CLIENT_INFO_ARRAY_VQ;
pub type LPDHCP_CLIENT_INFO_EX = *mut DHCP_CLIENT_INFO_EX;
pub type LPDHCP_CLIENT_INFO_EX_ARRAY = *mut DHCP_CLIENT_INFO_EX_ARRAY;
pub type LPDHCP_CLIENT_INFO_PB = *mut DHCP_CLIENT_INFO_PB;
pub type LPDHCP_CLIENT_INFO_PB_ARRAY = *mut DHCP_CLIENT_INFO_PB_ARRAY;
pub type LPDHCP_CLIENT_INFO_V4 = *mut DHCP_CLIENT_INFO_V4;
pub type LPDHCP_CLIENT_INFO_V5 = *mut DHCP_CLIENT_INFO_V5;
pub type LPDHCP_CLIENT_INFO_V6 = *mut DHCP_CLIENT_INFO_V6;
pub type LPDHCP_CLIENT_INFO_VQ = *mut DHCP_CLIENT_INFO_VQ;
pub type LPDHCP_CLIENT_SEARCH_UNION = *mut DHCP_SEARCH_INFO_0;
pub type LPDHCP_FAILOVER_MODE = *mut DHCP_FAILOVER_MODE;
pub type LPDHCP_FAILOVER_RELATIONSHIP = *mut DHCP_FAILOVER_RELATIONSHIP;
pub type LPDHCP_FAILOVER_RELATIONSHIP_ARRAY = *mut DHCP_FAILOVER_RELATIONSHIP_ARRAY;
pub type LPDHCP_FAILOVER_SERVER = *mut DHCP_FAILOVER_SERVER;
pub type LPDHCP_FAILOVER_STATISTICS = *mut DHCP_FAILOVER_STATISTICS;
pub type LPDHCP_FILTER_ADD_INFO = *mut DHCP_FILTER_ADD_INFO;
pub type LPDHCP_FILTER_ENUM_INFO = *mut DHCP_FILTER_ENUM_INFO;
pub type LPDHCP_FILTER_GLOBAL_INFO = *mut DHCP_FILTER_GLOBAL_INFO;
pub type LPDHCP_FILTER_LIST_TYPE = *mut DHCP_FILTER_LIST_TYPE;
pub type LPDHCP_FILTER_RECORD = *mut DHCP_FILTER_RECORD;
pub type LPDHCP_FORCE_FLAG = *mut DHCP_FORCE_FLAG;
pub type LPDHCP_HOST_INFO = *mut DHCP_HOST_INFO;
pub type LPDHCP_HOST_INFO_V6 = *mut DHCP_HOST_INFO_V6;
pub type LPDHCP_IPV6_ADDRESS = *mut DHCP_IPV6_ADDRESS;
pub type LPDHCP_IP_ADDRESS = *mut u32;
pub type LPDHCP_IP_ARRAY = *mut DHCP_IP_ARRAY;
pub type LPDHCP_IP_CLUSTER = *mut DHCP_IP_CLUSTER;
pub type LPDHCP_IP_RANGE = *mut DHCP_IP_RANGE;
pub type LPDHCP_IP_RANGE_ARRAY = *mut DHCP_IP_RANGE_ARRAY;
pub type LPDHCP_IP_RANGE_V6 = *mut DHCP_IP_RANGE_V6;
pub type LPDHCP_IP_RESERVATION = *mut DHCP_IP_RESERVATION;
pub type LPDHCP_IP_RESERVATION_INFO = *mut DHCP_IP_RESERVATION_INFO;
pub type LPDHCP_IP_RESERVATION_V4 = *mut DHCP_IP_RESERVATION_V4;
pub type LPDHCP_IP_RESERVATION_V6 = *mut DHCP_IP_RESERVATION_V6;
pub type LPDHCP_MIB_INFO = *mut DHCP_MIB_INFO;
pub type LPDHCP_MIB_INFO_V5 = *mut DHCP_MIB_INFO_V5;
pub type LPDHCP_MIB_INFO_V6 = *mut DHCP_MIB_INFO_V6;
pub type LPDHCP_MIB_INFO_VQ = *mut DHCP_MIB_INFO_VQ;
pub type LPDHCP_OPTION = *mut DHCP_OPTION;
pub type LPDHCP_OPTION_ARRAY = *mut DHCP_OPTION_ARRAY;
pub type LPDHCP_OPTION_DATA = *mut DHCP_OPTION_DATA;
pub type LPDHCP_OPTION_DATA_ELEMENT = *mut DHCP_OPTION_DATA_ELEMENT;
pub type LPDHCP_OPTION_DATA_TYPE = *mut DHCP_OPTION_DATA_TYPE;
pub type LPDHCP_OPTION_ELEMENT_UNION = *mut DHCP_OPTION_DATA_ELEMENT_0;
pub type LPDHCP_OPTION_LIST = *mut DHCP_OPTION_LIST;
pub type LPDHCP_OPTION_SCOPE_INFO = *mut DHCP_OPTION_SCOPE_INFO;
pub type LPDHCP_OPTION_SCOPE_INFO6 = *mut DHCP_OPTION_SCOPE_INFO6;
pub type LPDHCP_OPTION_SCOPE_TYPE = *mut DHCP_OPTION_SCOPE_TYPE;
pub type LPDHCP_OPTION_SCOPE_TYPE6 = *mut DHCP_OPTION_SCOPE_TYPE6;
pub type LPDHCP_OPTION_SCOPE_UNION6 = *mut DHCP_OPTION_SCOPE_INFO6_0;
pub type LPDHCP_OPTION_TYPE = *mut DHCP_OPTION_TYPE;
pub type LPDHCP_OPTION_VALUE = *mut DHCP_OPTION_VALUE;
pub type LPDHCP_OPTION_VALUE_ARRAY = *mut DHCP_OPTION_VALUE_ARRAY;
pub type LPDHCP_PERF_STATS = *mut DHCP_PERF_STATS;
#[cfg(feature = "minwindef")]
pub type LPDHCP_POLICY = *mut DHCP_POLICY;
#[cfg(feature = "minwindef")]
pub type LPDHCP_POLICY_ARRAY = *mut DHCP_POLICY_ARRAY;
#[cfg(feature = "minwindef")]
pub type LPDHCP_POLICY_EX = *mut DHCP_POLICY_EX;
#[cfg(feature = "minwindef")]
pub type LPDHCP_POLICY_EX_ARRAY = *mut DHCP_POLICY_EX_ARRAY;
#[cfg(feature = "minwindef")]
pub type LPDHCP_POL_COND = *mut DHCP_POL_COND;
#[cfg(feature = "minwindef")]
pub type LPDHCP_POL_COND_ARRAY = *mut DHCP_POL_COND_ARRAY;
pub type LPDHCP_POL_EXPR = *mut DHCP_POL_EXPR;
pub type LPDHCP_POL_EXPR_ARRAY = *mut DHCP_POL_EXPR_ARRAY;
pub type LPDHCP_PROPERTY = *mut DHCP_PROPERTY;
pub type LPDHCP_PROPERTY_ARRAY = *mut DHCP_PROPERTY_ARRAY;
pub type LPDHCP_RESERVATION_INFO_ARRAY = *mut DHCP_RESERVATION_INFO_ARRAY;
pub type LPDHCP_RESERVED_SCOPE = *mut DHCP_RESERVED_SCOPE;
pub type LPDHCP_RESERVED_SCOPE6 = *mut DHCP_RESERVED_SCOPE6;
pub type LPDHCP_SCAN_FLAG = *mut DHCP_SCAN_FLAG;
pub type LPDHCP_SCAN_ITEM = *mut DHCP_SCAN_ITEM;
pub type LPDHCP_SCAN_LIST = *mut DHCP_SCAN_LIST;
pub type LPDHCP_SEARCH_INFO = *mut DHCP_SEARCH_INFO;
pub type LPDHCP_SEARCH_INFO_TYPE = *mut DHCP_SEARCH_INFO_TYPE;
pub type LPDHCP_SEARCH_INFO_TYPE_V6 = *mut DHCP_SEARCH_INFO_TYPE_V6;
pub type LPDHCP_SEARCH_INFO_V6 = *mut DHCP_SEARCH_INFO_V6;
pub type LPDHCP_SERVER_CONFIG_INFO = *mut DHCP_SERVER_CONFIG_INFO;
pub type LPDHCP_SERVER_CONFIG_INFO_V4 = *mut DHCP_SERVER_CONFIG_INFO_V4;
pub type LPDHCP_SERVER_CONFIG_INFO_V6 = *mut DHCP_SERVER_CONFIG_INFO_V6;
pub type LPDHCP_SERVER_CONFIG_INFO_VQ = *mut DHCP_SERVER_CONFIG_INFO_VQ;
pub type LPDHCP_SERVER_INFO = LPDHCPDS_SERVER;
pub type LPDHCP_SERVER_INFO_ARRAY = LPDHCPDS_SERVERS;
pub type LPDHCP_SERVER_SPECIFIC_STRINGS = *mut DHCP_SERVER_SPECIFIC_STRINGS;
pub type LPDHCP_SUBNET_ELEMENT_DATA = *mut DHCP_SUBNET_ELEMENT_DATA;
pub type LPDHCP_SUBNET_ELEMENT_DATA_V4 = *mut DHCP_SUBNET_ELEMENT_DATA_V4;
pub type LPDHCP_SUBNET_ELEMENT_DATA_V5 = *mut DHCP_SUBNET_ELEMENT_DATA_V5;
pub type LPDHCP_SUBNET_ELEMENT_DATA_V6 = *mut DHCP_SUBNET_ELEMENT_DATA_V6;
pub type LPDHCP_SUBNET_ELEMENT_INFO_ARRAY = *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY;
pub type LPDHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 = *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4;
pub type LPDHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 = *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5;
pub type LPDHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 = *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6;
pub type LPDHCP_SUBNET_ELEMENT_TYPE = *mut DHCP_SUBNET_ELEMENT_TYPE;
pub type LPDHCP_SUBNET_ELEMENT_TYPE_V6 = *mut DHCP_SUBNET_ELEMENT_TYPE_V6;
pub type LPDHCP_SUBNET_ELEMENT_UNION = *mut DHCP_SUBNET_ELEMENT_DATA_0;
pub type LPDHCP_SUBNET_ELEMENT_UNION_V4 = *mut DHCP_SUBNET_ELEMENT_DATA_V4_0;
pub type LPDHCP_SUBNET_ELEMENT_UNION_V6 = *mut DHCP_SUBNET_ELEMENT_DATA_V6_0;
pub type LPDHCP_SUBNET_INFO = *mut DHCP_SUBNET_INFO;
pub type LPDHCP_SUBNET_INFO_V6 = *mut DHCP_SUBNET_INFO_V6;
pub type LPDHCP_SUBNET_INFO_VQ = *mut DHCP_SUBNET_INFO_VQ;
pub type LPDHCP_SUBNET_STATE = *mut DHCP_SUBNET_STATE;
pub type LPDHCP_SUPER_SCOPE_TABLE = *mut DHCP_SUPER_SCOPE_TABLE;
pub type LPDHCP_SUPER_SCOPE_TABLE_ENTRY = *mut DHCP_SUPER_SCOPE_TABLE_ENTRY;
pub type LPDWORD_DWORD = *mut DWORD_DWORD;
pub type LPSCOPE_MIB_INFO = *mut SCOPE_MIB_INFO;
pub type LPSCOPE_MIB_INFO_V5 = *mut SCOPE_MIB_INFO_V5;
pub type LPSCOPE_MIB_INFO_V6 = *mut SCOPE_MIB_INFO_V6;
pub type LPSCOPE_MIB_INFO_VQ = *mut SCOPE_MIB_INFO_VQ;
pub const LoadBalance: DHCP_FAILOVER_MODE = 0;
pub const MAC_ADDRESS_LENGTH: u32 = 6;
pub const MAX_PATTERN_LENGTH: u32 = 255;
pub const MCLT: u32 = 1;
pub const MODE: u32 = 16;
pub const NOQUARANTINE: QuarantineStatus = 0;
pub const NOQUARINFO: QuarantineStatus = 6;
pub const NORMAL: FSM_STATE = 3;
pub const NO_STATE: FSM_STATE = 0;
pub const PARTNER_DOWN: FSM_STATE = 5;
pub const PAUSED: FSM_STATE = 12;
pub type PDATE_TIME = *mut DATE_TIME;
pub type PDHCPDS_SERVER = *mut DHCPDS_SERVER;
pub type PDHCPDS_SERVERS = *mut DHCPDS_SERVERS;
pub type PDHCPV6_STATELESS_PARAMS = *mut DHCPV6_STATELESS_PARAMS;
pub type PDHCPV6_STATELESS_SCOPE_STATS = *mut DHCPV6_STATELESS_SCOPE_STATS;
pub type PDHCPV6_STATELESS_STATS = *mut DHCPV6_STATELESS_STATS;
pub type PDHCP_ATTRIB = *mut DHCP_ATTRIB;
pub type PDHCP_ATTRIB_ARRAY = *mut DHCP_ATTRIB_ARRAY;
pub type PDHCP_ATTRIB_ID = *mut u32;
pub type PDHCP_IPV6_ADDRESS = *mut DHCP_IPV6_ADDRESS;
pub type PDHCP_IP_ADDRESS = *mut u32;
pub type PDHCP_IP_RANGE_ARRAY = *mut DHCP_IP_RANGE_ARRAY;
#[cfg(feature = "minwindef")]
pub type PDHCP_POLICY = *mut DHCP_POLICY;
#[cfg(feature = "minwindef")]
pub type PDHCP_POLICY_ARRAY = *mut DHCP_POLICY_ARRAY;
#[cfg(feature = "minwindef")]
pub type PDHCP_POLICY_EX = *mut DHCP_POLICY_EX;
#[cfg(feature = "minwindef")]
pub type PDHCP_POLICY_EX_ARRAY = *mut DHCP_POLICY_EX_ARRAY;
#[cfg(feature = "minwindef")]
pub type PDHCP_POL_COND = *mut DHCP_POL_COND;
#[cfg(feature = "minwindef")]
pub type PDHCP_POL_COND_ARRAY = *mut DHCP_POL_COND_ARRAY;
pub type PDHCP_POL_EXPR = *mut DHCP_POL_EXPR;
pub type PDHCP_POL_EXPR_ARRAY = *mut DHCP_POL_EXPR_ARRAY;
pub type PDHCP_PROPERTY = *mut DHCP_PROPERTY;
pub type PDHCP_PROPERTY_ARRAY = *mut DHCP_PROPERTY_ARRAY;
pub type PDHCP_SERVER_INFO = PDHCPDS_SERVER;
pub type PDHCP_SERVER_INFO_ARRAY = PDHCPDS_SERVERS;
pub type PDHCP_SUBNET_INFO_V6 = *mut DHCP_SUBNET_INFO_V6;
pub const PERCENTAGE: u32 = 8;
pub const POTENTIAL_CONFLICT: FSM_STATE = 6;
pub const PREVSTATE: u32 = 32;
pub const PROBATION: QuarantineStatus = 3;
pub const PrimaryServer: DHCP_FAILOVER_SERVER = 0;
pub const QUARANTINE_CONFIG_OPTION: u32 = 43222;
pub const QUARANTINE_SCOPE_QUARPROFILE_OPTION: u32 = 43221;
pub const QUARANTIN_OPTION_BASE: u32 = 43220;
pub type QuarantineStatus = i32;
pub const RECOVER: FSM_STATE = 9;
pub const RECOVER_DONE: FSM_STATE = 11;
pub const RECOVER_WAIT: FSM_STATE = 10;
pub const RESOLUTION_INT: FSM_STATE = 8;
pub const RESTRICTEDACCESS: QuarantineStatus = 1;
pub const SAFEPERIOD: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCOPE_MIB_INFO {
    pub Subnet: DHCP_IP_ADDRESS,
    pub NumAddressesInuse: u32,
    pub NumAddressesFree: u32,
    pub NumPendingOffers: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCOPE_MIB_INFO_V5 {
    pub Subnet: DHCP_IP_ADDRESS,
    pub NumAddressesInuse: u32,
    pub NumAddressesFree: u32,
    pub NumPendingOffers: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCOPE_MIB_INFO_V6 {
    pub Subnet: DHCP_IPV6_ADDRESS,
    pub NumAddressesInuse: u64,
    pub NumAddressesFree: u64,
    pub NumPendingAdvertises: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCOPE_MIB_INFO_VQ {
    pub Subnet: DHCP_IP_ADDRESS,
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
pub const SCOPE_STATE_DISABLED: u32 = 1;
pub const SCOPE_STATE_ENABLED: u32 = 0;
pub const SHAREDSECRET: u32 = 64;
pub const SHUTDOWN: FSM_STATE = 13;
pub const STARTUP: FSM_STATE = 2;
pub const SecondaryServer: DHCP_FAILOVER_SERVER = 1;
pub const Set_APIProtocolSupport: u32 = 1;
pub const Set_AuditLogState: u32 = 2048;
pub const Set_BackupInterval: u32 = 16;
pub const Set_BackupPath: u32 = 8;
pub const Set_BootFileTable: u32 = 1024;
pub const Set_DatabaseCleanupInterval: u32 = 128;
pub const Set_DatabaseLoggingFlag: u32 = 32;
pub const Set_DatabaseName: u32 = 2;
pub const Set_DatabasePath: u32 = 4;
pub const Set_DebugFlag: u32 = 256;
pub const Set_PingRetries: u32 = 512;
pub const Set_PreferredLifetime: u32 = 4;
pub const Set_PreferredLifetimeIATA: u32 = 64;
pub const Set_QuarantineDefFail: u32 = 8192;
pub const Set_QuarantineON: u32 = 4096;
pub const Set_RapidCommitFlag: u32 = 2;
pub const Set_RestoreFlag: u32 = 64;
pub const Set_T1: u32 = 16;
pub const Set_T2: u32 = 32;
pub const Set_UnicastFlag: u32 = 1;
pub const Set_ValidLifetime: u32 = 8;
pub const Set_ValidLifetimeIATA: u32 = 128;
pub const V5_ADDRESS_BIT_BOTH_REC: u32 = 32;
pub const V5_ADDRESS_BIT_DELETED: u32 = 128;
pub const V5_ADDRESS_BIT_UNREGISTERED: u32 = 64;
pub const V5_ADDRESS_EX_BIT_DISABLE_PTR_RR: u32 = 1;
pub const V5_ADDRESS_STATE_ACTIVE: u32 = 1;
pub const V5_ADDRESS_STATE_DECLINED: u32 = 2;
pub const V5_ADDRESS_STATE_DOOM: u32 = 3;
pub const V5_ADDRESS_STATE_OFFERED: u32 = 0;
pub const WARNING_EXTENDED_LESS: u32 = 20026;
