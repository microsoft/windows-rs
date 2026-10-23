#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpAddFilterV4(serveripaddress : *const super::WCHAR, addfilterinfo : *const DHCP_FILTER_ADD_INFO, forceflag : windows_sys::core::BOOL) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpAddSecurityGroup(pserver : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpAddServer(flags : super::DWORD, idinfo : super::LPVOID, newserver : LPDHCP_SERVER_INFO, callbackfn : super::LPVOID, callbackdata : super::LPVOID) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpAddSubnetElement(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, addelementinfo : *const DHCP_SUBNET_ELEMENT_DATA) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpAddSubnetElementV4(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, addelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V4) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpAddSubnetElementV5(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, addelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V5) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpAddSubnetElementV6(serveripaddress : windows_sys::core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, addelementinfo : LPDHCP_SUBNET_ELEMENT_DATA_V6) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpAuditLogGetParams(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, auditlogdir : *mut windows_sys::core::PWSTR, diskcheckinterval : *mut super::DWORD, maxlogfilessize : *mut super::DWORD, minspaceondisk : *mut super::DWORD) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpAuditLogSetParams(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, auditlogdir : windows_sys::core::PCWSTR, diskcheckinterval : super::DWORD, maxlogfilessize : super::DWORD, minspaceondisk : super::DWORD) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpCreateClass(serveripaddress : windows_sys::core::PCWSTR, reservedmustbezero : super::DWORD, classinfo : LPDHCP_CLASS_INFO) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpCreateClassV6(serveripaddress : windows_sys::core::PCWSTR, reservedmustbezero : super::DWORD, classinfo : LPDHCP_CLASS_INFO_V6) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpCreateClientInfo(serveripaddress : *const super::WCHAR, clientinfo : *const DHCP_CLIENT_INFO) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpCreateClientInfoV4(serveripaddress : *const super::WCHAR, clientinfo : *const DHCP_CLIENT_INFO_V4) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpCreateClientInfoVQ(serveripaddress : *const super::WCHAR, clientinfo : *const DHCP_CLIENT_INFO_VQ) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpCreateOption(serveripaddress : *const super::WCHAR, optionid : DHCP_OPTION_ID, optioninfo : *const DHCP_OPTION) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpCreateOptionV5(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, optioninfo : LPDHCP_OPTION) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpCreateOptionV6(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, optioninfo : LPDHCP_OPTION) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpCreateSubnet(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, subnetinfo : *const DHCP_SUBNET_INFO) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpCreateSubnetV6(serveripaddress : windows_sys::core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, subnetinfo : LPDHCP_SUBNET_INFO_V6) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpCreateSubnetVQ(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, subnetinfo : *const DHCP_SUBNET_INFO_VQ) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpDeleteClass(serveripaddress : windows_sys::core::PCWSTR, reservedmustbezero : super::DWORD, classname : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpDeleteClassV6(serveripaddress : windows_sys::core::PCWSTR, reservedmustbezero : super::DWORD, classname : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpDeleteClientInfo(serveripaddress : *const super::WCHAR, clientinfo : *const DHCP_SEARCH_INFO) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpDeleteClientInfoV6(serveripaddress : *const super::WCHAR, clientinfo : *const DHCP_SEARCH_INFO_V6) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpDeleteFilterV4(serveripaddress : *const super::WCHAR, deletefilterinfo : *const DHCP_ADDR_PATTERN) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpDeleteServer(flags : super::DWORD, idinfo : super::LPVOID, newserver : LPDHCP_SERVER_INFO, callbackfn : super::LPVOID, callbackdata : super::LPVOID) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpDeleteSubnet(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, forceflag : DHCP_FORCE_FLAG) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpDeleteSubnetV6(serveripaddress : windows_sys::core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, forceflag : DHCP_FORCE_FLAG) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpDeleteSuperScopeV4(serveripaddress : *const super::WCHAR, superscopename : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpDsCleanup());
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpDsInit() -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpEnumClasses(serveripaddress : windows_sys::core::PCWSTR, reservedmustbezero : super::DWORD, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, classinfoarray : *mut LPDHCP_CLASS_INFO_ARRAY, nread : *mut super::DWORD, ntotal : *mut super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumClassesV6(serveripaddress : windows_sys::core::PCWSTR, reservedmustbezero : super::DWORD, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, classinfoarray : *mut LPDHCP_CLASS_INFO_ARRAY_V6, nread : *mut super::DWORD, ntotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumFilterV4(serveripaddress : *const super::WCHAR, resumehandle : LPDHCP_ADDR_PATTERN, preferredmaximum : super::DWORD, listtype : DHCP_FILTER_LIST_TYPE, enumfilterinfo : *mut LPDHCP_FILTER_ENUM_INFO, elementsread : *mut super::DWORD, elementstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumOptionValues(serveripaddress : *const super::WCHAR, scopeinfo : *const DHCP_OPTION_SCOPE_INFO, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, optionvalues : *mut LPDHCP_OPTION_VALUE_ARRAY, optionsread : *mut super::DWORD, optionstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpEnumOptionValuesV5(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, scopeinfo : LPDHCP_OPTION_SCOPE_INFO, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, optionvalues : *mut LPDHCP_OPTION_VALUE_ARRAY, optionsread : *mut super::DWORD, optionstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumOptionValuesV6(serveripaddress : *const super::WCHAR, flags : super::DWORD, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, scopeinfo : LPDHCP_OPTION_SCOPE_INFO6, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, optionvalues : *mut LPDHCP_OPTION_VALUE_ARRAY, optionsread : *mut super::DWORD, optionstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumOptions(serveripaddress : *const super::WCHAR, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, options : *mut LPDHCP_OPTION_ARRAY, optionsread : *mut super::DWORD, optionstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpEnumOptionsV5(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, options : *mut LPDHCP_OPTION_ARRAY, optionsread : *mut super::DWORD, optionstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumOptionsV6(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, options : *mut LPDHCP_OPTION_ARRAY, optionsread : *mut super::DWORD, optionstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpEnumServers(flags : super::DWORD, idinfo : super::LPVOID, servers : *mut LPDHCP_SERVER_INFO_ARRAY, callbackfn : super::LPVOID, callbackdata : super::LPVOID) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClients(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, clientinfo : *mut LPDHCP_CLIENT_INFO_ARRAY, clientsread : *mut super::DWORD, clientstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsFilterStatusInfo(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, clientinfo : *mut LPDHCP_CLIENT_FILTER_STATUS_INFO_ARRAY, clientsread : *mut super::DWORD, clientstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsV4(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, clientinfo : *mut LPDHCP_CLIENT_INFO_ARRAY_V4, clientsread : *mut super::DWORD, clientstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsV5(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, clientinfo : *mut LPDHCP_CLIENT_INFO_ARRAY_V5, clientsread : *mut super::DWORD, clientstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsV6(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IPV6_ADDRESS, resumehandle : *mut DHCP_RESUME_IPV6_HANDLE, preferredmaximum : super::DWORD, clientinfo : *mut LPDHCP_CLIENT_INFO_ARRAY_V6, clientsread : *mut super::DWORD, clientstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsVQ(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, clientinfo : *mut LPDHCP_CLIENT_INFO_ARRAY_VQ, clientsread : *mut super::DWORD, clientstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetElements(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, enumelementtype : DHCP_SUBNET_ELEMENT_TYPE, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, enumelementinfo : *mut LPDHCP_SUBNET_ELEMENT_INFO_ARRAY, elementsread : *mut super::DWORD, elementstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetElementsV4(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, enumelementtype : DHCP_SUBNET_ELEMENT_TYPE, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, enumelementinfo : *mut LPDHCP_SUBNET_ELEMENT_INFO_ARRAY_V4, elementsread : *mut super::DWORD, elementstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetElementsV5(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, enumelementtype : DHCP_SUBNET_ELEMENT_TYPE, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, enumelementinfo : *mut LPDHCP_SUBNET_ELEMENT_INFO_ARRAY_V5, elementsread : *mut super::DWORD, elementstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetElementsV6(serveripaddress : windows_sys::core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, enumelementtype : DHCP_SUBNET_ELEMENT_TYPE_V6, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, enumelementinfo : *mut LPDHCP_SUBNET_ELEMENT_INFO_ARRAY_V6, elementsread : *mut super::DWORD, elementstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnets(serveripaddress : *const super::WCHAR, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, enuminfo : *mut LPDHCP_IP_ARRAY, elementsread : *mut super::DWORD, elementstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetsV6(serveripaddress : *const super::WCHAR, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, enuminfo : *mut LPDHCPV6_IP_ARRAY, elementsread : *mut super::DWORD, elementstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpGetAllOptionValues(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, scopeinfo : LPDHCP_OPTION_SCOPE_INFO, values : *mut LPDHCP_ALL_OPTION_VALUES) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetAllOptionValuesV6(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, scopeinfo : LPDHCP_OPTION_SCOPE_INFO6, values : *mut LPDHCP_ALL_OPTION_VALUES) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpGetAllOptions(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionstruct : *mut LPDHCP_ALL_OPTIONS) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetAllOptionsV6(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionstruct : *mut LPDHCP_ALL_OPTIONS) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpGetClassInfo(serveripaddress : windows_sys::core::PCWSTR, reservedmustbezero : super::DWORD, partialclassinfo : LPDHCP_CLASS_INFO, filledclassinfo : *mut LPDHCP_CLASS_INFO) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetClientInfo(serveripaddress : *const super::WCHAR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut LPDHCP_CLIENT_INFO) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetClientInfoV4(serveripaddress : *const super::WCHAR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut LPDHCP_CLIENT_INFO_V4) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetClientInfoV6(serveripaddress : *const super::WCHAR, searchinfo : *const DHCP_SEARCH_INFO_V6, clientinfo : *mut LPDHCP_CLIENT_INFO_V6) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetClientInfoVQ(serveripaddress : *const super::WCHAR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut LPDHCP_CLIENT_INFO_VQ) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetClientOptions(serveripaddress : *const super::WCHAR, clientipaddress : DHCP_IP_ADDRESS, clientsubnetmask : DHCP_IP_MASK, clientoptions : *mut LPDHCP_OPTION_LIST) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetFilterV4(serveripaddress : *const super::WCHAR, globalfilterinfo : *mut DHCP_FILTER_GLOBAL_INFO) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetMibInfo(serveripaddress : *const super::WCHAR, mibinfo : *mut LPDHCP_MIB_INFO) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetMibInfoV5(serveripaddress : *const super::WCHAR, mibinfo : *mut LPDHCP_MIB_INFO_V5) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpGetMibInfoV6(serveripaddress : *const super::WCHAR, mibinfo : *mut LPDHCP_MIB_INFO_V6) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetOptionInfo(serveripaddress : *const super::WCHAR, optionid : DHCP_OPTION_ID, optioninfo : *mut LPDHCP_OPTION) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpGetOptionInfoV5(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, optioninfo : *mut LPDHCP_OPTION) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetOptionInfoV6(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, optioninfo : *mut LPDHCP_OPTION) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetOptionValue(serveripaddress : *const super::WCHAR, optionid : DHCP_OPTION_ID, scopeinfo : *const DHCP_OPTION_SCOPE_INFO, optionvalue : *mut LPDHCP_OPTION_VALUE) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpGetOptionValueV5(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, scopeinfo : LPDHCP_OPTION_SCOPE_INFO, optionvalue : *mut LPDHCP_OPTION_VALUE) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpGetOptionValueV6(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, scopeinfo : LPDHCP_OPTION_SCOPE_INFO6, optionvalue : *mut LPDHCP_OPTION_VALUE) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetServerBindingInfo(serveripaddress : *const super::WCHAR, flags : super::ULONG, bindelementsinfo : *mut LPDHCP_BIND_ELEMENT_ARRAY) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetServerBindingInfoV6(serveripaddress : *const super::WCHAR, flags : super::ULONG, bindelementsinfo : *mut LPDHCPV6_BIND_ELEMENT_ARRAY) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetServerSpecificStrings(serveripaddress : *const super::WCHAR, serverspecificstrings : *mut LPDHCP_SERVER_SPECIFIC_STRINGS) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetSubnetDelayOffer(serveripaddress : windows_sys::core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, timedelayinmilliseconds : *mut super::USHORT) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetSubnetInfo(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, subnetinfo : *mut LPDHCP_SUBNET_INFO) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetSubnetInfoV6(serveripaddress : windows_sys::core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, subnetinfo : *mut LPDHCP_SUBNET_INFO_V6) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetSubnetInfoVQ(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, subnetinfo : *mut LPDHCP_SUBNET_INFO_VQ) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpGetSuperScopeInfoV4(serveripaddress : *const super::WCHAR, superscopetable : *mut LPDHCP_SUPER_SCOPE_TABLE) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpGetThreadOptions(pflags : super::LPDWORD, reserved : super::LPVOID) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpGetVersion(serveripaddress : windows_sys::core::PCWSTR, majorversion : super::LPDWORD, minorversion : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpHlprAddV4PolicyCondition(policy : LPDHCP_POLICY, parentexpr : super::DWORD, r#type : DHCP_POL_ATTR_TYPE, optionid : super::DWORD, suboptionid : super::DWORD, vendorname : windows_sys::core::PCWSTR, operator : DHCP_POL_COMPARATOR, value : super::LPBYTE, valuelength : super::DWORD, conditionindex : *mut super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpHlprAddV4PolicyExpr(policy : LPDHCP_POLICY, parentexpr : super::DWORD, operator : DHCP_POL_LOGIC_OPER, exprindex : *mut super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpHlprAddV4PolicyRange(policy : LPDHCP_POLICY, range : LPDHCP_IP_RANGE) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpHlprCreateV4Policy(policyname : windows_sys::core::PCWSTR, fglobalpolicy : windows_sys::core::BOOL, subnet : DHCP_IP_ADDRESS, processingorder : super::DWORD, rootoperator : DHCP_POL_LOGIC_OPER, description : windows_sys::core::PCWSTR, enabled : windows_sys::core::BOOL, policy : *mut LPDHCP_POLICY) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpHlprCreateV4PolicyEx(policyname : windows_sys::core::PCWSTR, fglobalpolicy : windows_sys::core::BOOL, subnet : DHCP_IP_ADDRESS, processingorder : super::DWORD, rootoperator : DHCP_POL_LOGIC_OPER, description : windows_sys::core::PCWSTR, enabled : windows_sys::core::BOOL, policy : *mut LPDHCP_POLICY_EX) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpHlprFindV4DhcpProperty(propertyarray : LPDHCP_PROPERTY_ARRAY, id : DHCP_PROPERTY_ID, r#type : DHCP_PROPERTY_TYPE) -> LPDHCP_PROPERTY);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4DhcpProperty(property : LPDHCP_PROPERTY));
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4DhcpPropertyArray(propertyarray : LPDHCP_PROPERTY_ARRAY));
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4Policy(policy : LPDHCP_POLICY));
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4PolicyArray(policyarray : LPDHCP_POLICY_ARRAY));
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4PolicyEx(policyex : LPDHCP_POLICY_EX));
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4PolicyExArray(policyexarray : LPDHCP_POLICY_EX_ARRAY));
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpHlprIsV4PolicySingleUC(policy : LPDHCP_POLICY) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpHlprIsV4PolicyValid(ppolicy : LPDHCP_POLICY) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpHlprIsV4PolicyWellFormed(ppolicy : LPDHCP_POLICY) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpHlprModifyV4PolicyExpr(policy : LPDHCP_POLICY, operator : DHCP_POL_LOGIC_OPER) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpHlprResetV4PolicyExpr(policy : LPDHCP_POLICY) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpModifyClass(serveripaddress : windows_sys::core::PCWSTR, reservedmustbezero : super::DWORD, classinfo : LPDHCP_CLASS_INFO) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpModifyClassV6(serveripaddress : windows_sys::core::PCWSTR, reservedmustbezero : super::DWORD, classinfo : LPDHCP_CLASS_INFO_V6) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpRemoveOption(serveripaddress : *const super::WCHAR, optionid : DHCP_OPTION_ID) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpRemoveOptionV5(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpRemoveOptionV6(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpRemoveOptionValue(serveripaddress : *const super::WCHAR, optionid : DHCP_OPTION_ID, scopeinfo : *const DHCP_OPTION_SCOPE_INFO) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpRemoveOptionValueV5(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, scopeinfo : LPDHCP_OPTION_SCOPE_INFO) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpRemoveOptionValueV6(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, scopeinfo : LPDHCP_OPTION_SCOPE_INFO6) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpRemoveSubnetElement(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, removeelementinfo : *const DHCP_SUBNET_ELEMENT_DATA, forceflag : DHCP_FORCE_FLAG) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpRemoveSubnetElementV4(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, removeelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V4, forceflag : DHCP_FORCE_FLAG) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpRemoveSubnetElementV5(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, removeelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V5, forceflag : DHCP_FORCE_FLAG) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpRemoveSubnetElementV6(serveripaddress : windows_sys::core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, removeelementinfo : LPDHCP_SUBNET_ELEMENT_DATA_V6, forceflag : DHCP_FORCE_FLAG) -> super::DWORD);
#[cfg(feature = "winnt")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpRpcFreeMemory(bufferpointer : super::PVOID));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpScanDatabase(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, fixflag : super::DWORD, scanlist : *mut LPDHCP_SCAN_LIST) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpServerAuditlogParamsFree(configinfo : *mut DHCP_SERVER_CONFIG_INFO_VQ));
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpServerBackupDatabase(serveripaddress : windows_sys::core::PCWSTR, path : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpServerGetConfig(serveripaddress : *const super::WCHAR, configinfo : *mut LPDHCP_SERVER_CONFIG_INFO) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpServerGetConfigV4(serveripaddress : *const super::WCHAR, configinfo : *mut LPDHCP_SERVER_CONFIG_INFO_V4) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpServerGetConfigV6(serveripaddress : *const super::WCHAR, scopeinfo : LPDHCP_OPTION_SCOPE_INFO6, configinfo : *mut LPDHCP_SERVER_CONFIG_INFO_V6) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpServerGetConfigVQ(serveripaddress : *const super::WCHAR, configinfo : *mut LPDHCP_SERVER_CONFIG_INFO_VQ) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpServerQueryAttribute(serveripaddr : windows_sys::core::PCWSTR, dwreserved : super::ULONG, dhcpattribid : DHCP_ATTRIB_ID, pdhcpattrib : *mut LPDHCP_ATTRIB) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpServerQueryAttributes(serveripaddr : windows_sys::core::PCWSTR, dwreserved : super::ULONG, dwattribcount : super::ULONG, pdhcpattribs : *mut DHCP_ATTRIB_ID, pdhcpattribarr : *mut LPDHCP_ATTRIB_ARRAY) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpServerQueryDnsRegCredentials(serveripaddress : windows_sys::core::PCWSTR, unamesize : super::ULONG, uname : windows_sys::core::PWSTR, domainsize : super::ULONG, domain : windows_sys::core::PWSTR) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpServerRedoAuthorization(serveripaddr : windows_sys::core::PCWSTR, dwreserved : super::ULONG) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpServerRestoreDatabase(serveripaddress : windows_sys::core::PCWSTR, path : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpServerSetConfig(serveripaddress : *const super::WCHAR, fieldstoset : super::DWORD, configinfo : LPDHCP_SERVER_CONFIG_INFO) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpServerSetConfigV4(serveripaddress : *const super::WCHAR, fieldstoset : super::DWORD, configinfo : LPDHCP_SERVER_CONFIG_INFO_V4) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpServerSetConfigV6(serveripaddress : *const super::WCHAR, scopeinfo : LPDHCP_OPTION_SCOPE_INFO6, fieldstoset : super::DWORD, configinfo : LPDHCP_SERVER_CONFIG_INFO_V6) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpServerSetConfigVQ(serveripaddress : *const super::WCHAR, fieldstoset : super::DWORD, configinfo : LPDHCP_SERVER_CONFIG_INFO_VQ) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpServerSetDnsRegCredentials(serveripaddress : windows_sys::core::PCWSTR, uname : windows_sys::core::PCWSTR, domain : windows_sys::core::PCWSTR, passwd : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpServerSetDnsRegCredentialsV5(serveripaddress : windows_sys::core::PCWSTR, uname : windows_sys::core::PCWSTR, domain : windows_sys::core::PCWSTR, passwd : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpSetClientInfo(serveripaddress : *const super::WCHAR, clientinfo : *const DHCP_CLIENT_INFO) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpSetClientInfoV4(serveripaddress : *const super::WCHAR, clientinfo : *const DHCP_CLIENT_INFO_V4) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpSetClientInfoV6(serveripaddress : *const super::WCHAR, clientinfo : *const DHCP_CLIENT_INFO_V6) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpSetClientInfoVQ(serveripaddress : *const super::WCHAR, clientinfo : *const DHCP_CLIENT_INFO_VQ) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpSetFilterV4(serveripaddress : *const super::WCHAR, globalfilterinfo : *const DHCP_FILTER_GLOBAL_INFO) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpSetOptionInfo(serveripaddress : *const super::WCHAR, optionid : DHCP_OPTION_ID, optioninfo : *const DHCP_OPTION) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpSetOptionInfoV5(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, optioninfo : LPDHCP_OPTION) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpSetOptionInfoV6(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, optioninfo : LPDHCP_OPTION) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpSetOptionValue(serveripaddress : *const super::WCHAR, optionid : DHCP_OPTION_ID, scopeinfo : *const DHCP_OPTION_SCOPE_INFO, optionvalue : *const DHCP_OPTION_DATA) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpSetOptionValueV5(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, scopeinfo : LPDHCP_OPTION_SCOPE_INFO, optionvalue : LPDHCP_OPTION_DATA) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpSetOptionValueV6(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, scopeinfo : LPDHCP_OPTION_SCOPE_INFO6, optionvalue : LPDHCP_OPTION_DATA) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpSetOptionValues(serveripaddress : *const super::WCHAR, scopeinfo : *const DHCP_OPTION_SCOPE_INFO, optionvalues : *const DHCP_OPTION_VALUE_ARRAY) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpSetOptionValuesV5(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, classname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, scopeinfo : LPDHCP_OPTION_SCOPE_INFO, optionvalues : LPDHCP_OPTION_VALUE_ARRAY) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpSetServerBindingInfo(serveripaddress : *const super::WCHAR, flags : super::ULONG, bindelementinfo : LPDHCP_BIND_ELEMENT_ARRAY) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpSetServerBindingInfoV6(serveripaddress : *const super::WCHAR, flags : super::ULONG, bindelementinfo : LPDHCPV6_BIND_ELEMENT_ARRAY) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpSetSubnetDelayOffer(serveripaddress : windows_sys::core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, timedelayinmilliseconds : super::USHORT) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpSetSubnetInfo(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, subnetinfo : *const DHCP_SUBNET_INFO) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpSetSubnetInfoV6(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IPV6_ADDRESS, subnetinfo : LPDHCP_SUBNET_INFO_V6) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpSetSubnetInfoVQ(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, subnetinfo : *const DHCP_SUBNET_INFO_VQ) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpSetSuperScopeV4(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, superscopename : windows_sys::core::PCWSTR, changeexisting : windows_sys::core::BOOL) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("dhcpsapi.dll" "C" fn DhcpSetThreadOptions(flags : super::DWORD, reserved : super::LPVOID) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4AddPolicyRange(serveripaddress : windows_sys::core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, policyname : windows_sys::core::PCWSTR, range : LPDHCP_IP_RANGE) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4CreateClientInfo(serveripaddress : *const super::WCHAR, clientinfo : LPDHCP_CLIENT_INFO_PB) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4CreateClientInfoEx(serveripaddress : *const super::WCHAR, clientinfo : LPDHCP_CLIENT_INFO_EX) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4CreatePolicy(serveripaddress : windows_sys::core::PCWSTR, ppolicy : LPDHCP_POLICY) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4CreatePolicyEx(serveripaddress : windows_sys::core::PCWSTR, policyex : LPDHCP_POLICY_EX) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4DeletePolicy(serveripaddress : windows_sys::core::PCWSTR, fglobalpolicy : windows_sys::core::BOOL, subnetaddress : DHCP_IP_ADDRESS, policyname : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4EnumPolicies(serveripaddress : windows_sys::core::PCWSTR, resumehandle : super::LPDWORD, preferredmaximum : super::DWORD, fglobalpolicy : windows_sys::core::BOOL, subnetaddress : DHCP_IP_ADDRESS, enuminfo : *mut LPDHCP_POLICY_ARRAY, elementsread : *mut super::DWORD, elementstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4EnumPoliciesEx(serveripaddress : windows_sys::core::PCWSTR, resumehandle : super::LPDWORD, preferredmaximum : super::DWORD, globalpolicy : windows_sys::core::BOOL, subnetaddress : DHCP_IP_ADDRESS, enuminfo : *mut LPDHCP_POLICY_EX_ARRAY, elementsread : *mut super::DWORD, elementstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4EnumSubnetClients(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, clientinfo : *mut LPDHCP_CLIENT_INFO_PB_ARRAY, clientsread : *mut super::DWORD, clientstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4EnumSubnetClientsEx(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, clientinfo : *mut LPDHCP_CLIENT_INFO_EX_ARRAY, clientsread : *mut super::DWORD, clientstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4EnumSubnetReservations(serveripaddress : *const super::WCHAR, subnetaddress : DHCP_IP_ADDRESS, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, enumelementinfo : *mut LPDHCP_RESERVATION_INFO_ARRAY, elementsread : *mut super::DWORD, elementstotal : *mut super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverAddScopeToRelationship(serveripaddress : windows_sys::core::PCWSTR, prelationship : LPDHCP_FAILOVER_RELATIONSHIP) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverCreateRelationship(serveripaddress : windows_sys::core::PCWSTR, prelationship : LPDHCP_FAILOVER_RELATIONSHIP) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverDeleteRelationship(serveripaddress : windows_sys::core::PCWSTR, prelationshipname : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverDeleteScopeFromRelationship(serveripaddress : windows_sys::core::PCWSTR, prelationship : LPDHCP_FAILOVER_RELATIONSHIP) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverEnumRelationship(serveripaddress : windows_sys::core::PCWSTR, resumehandle : *mut DHCP_RESUME_HANDLE, preferredmaximum : super::DWORD, prelationship : *mut LPDHCP_FAILOVER_RELATIONSHIP_ARRAY, relationshipread : super::LPDWORD, relationshiptotal : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetAddressStatus(serveripaddress : windows_sys::core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, pstatus : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetClientInfo(serveripaddress : windows_sys::core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut LPDHCPV4_FAILOVER_CLIENT_INFO) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetRelationship(serveripaddress : windows_sys::core::PCWSTR, prelationshipname : windows_sys::core::PCWSTR, prelationship : *mut LPDHCP_FAILOVER_RELATIONSHIP) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetScopeRelationship(serveripaddress : windows_sys::core::PCWSTR, scopeid : DHCP_IP_ADDRESS, prelationship : *mut LPDHCP_FAILOVER_RELATIONSHIP) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetScopeStatistics(serveripaddress : windows_sys::core::PCWSTR, scopeid : DHCP_IP_ADDRESS, pstats : *mut LPDHCP_FAILOVER_STATISTICS) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetSystemTime(serveripaddress : windows_sys::core::PCWSTR, ptime : super::LPDWORD, pmaxalloweddeltatime : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverSetRelationship(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, prelationship : LPDHCP_FAILOVER_RELATIONSHIP) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverTriggerAddrAllocation(serveripaddress : windows_sys::core::PCWSTR, pfailrelname : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4GetAllOptionValues(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, scopeinfo : LPDHCP_OPTION_SCOPE_INFO, values : *mut LPDHCP_ALL_OPTION_VALUES_PB) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4GetClientInfo(serveripaddress : *const super::WCHAR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut LPDHCP_CLIENT_INFO_PB) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4GetClientInfoEx(serveripaddress : *const super::WCHAR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut LPDHCP_CLIENT_INFO_EX) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4GetFreeIPAddress(serveripaddress : windows_sys::core::PCWSTR, scopeid : DHCP_IP_ADDRESS, startip : DHCP_IP_ADDRESS, endip : DHCP_IP_ADDRESS, numfreeaddrreq : super::DWORD, ipaddrlist : *mut LPDHCP_IP_ARRAY) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4GetOptionValue(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, policyname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, scopeinfo : LPDHCP_OPTION_SCOPE_INFO, optionvalue : *mut LPDHCP_OPTION_VALUE) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4GetPolicy(serveripaddress : windows_sys::core::PCWSTR, fglobalpolicy : windows_sys::core::BOOL, subnetaddress : DHCP_IP_ADDRESS, policyname : windows_sys::core::PCWSTR, policy : *mut LPDHCP_POLICY) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4GetPolicyEx(serveripaddress : windows_sys::core::PCWSTR, globalpolicy : windows_sys::core::BOOL, subnetaddress : DHCP_IP_ADDRESS, policyname : windows_sys::core::PCWSTR, policy : *mut LPDHCP_POLICY_EX) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4QueryPolicyEnforcement(serveripaddress : windows_sys::core::PCWSTR, fglobalpolicy : windows_sys::core::BOOL, subnetaddress : DHCP_IP_ADDRESS, enabled : *mut windows_sys::core::BOOL) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4RemoveOptionValue(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, policyname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, scopeinfo : LPDHCP_OPTION_SCOPE_INFO) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4RemovePolicyRange(serveripaddress : windows_sys::core::PCWSTR, subnetaddress : DHCP_IP_ADDRESS, policyname : windows_sys::core::PCWSTR, range : LPDHCP_IP_RANGE) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4SetOptionValue(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, optionid : DHCP_OPTION_ID, policyname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, scopeinfo : LPDHCP_OPTION_SCOPE_INFO, optionvalue : LPDHCP_OPTION_DATA) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4SetOptionValues(serveripaddress : windows_sys::core::PCWSTR, flags : super::DWORD, policyname : windows_sys::core::PCWSTR, vendorname : windows_sys::core::PCWSTR, scopeinfo : LPDHCP_OPTION_SCOPE_INFO, optionvalues : LPDHCP_OPTION_VALUE_ARRAY) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4SetPolicy(serveripaddress : windows_sys::core::PCWSTR, fieldsmodified : super::DWORD, fglobalpolicy : windows_sys::core::BOOL, subnetaddress : DHCP_IP_ADDRESS, policyname : windows_sys::core::PCWSTR, policy : LPDHCP_POLICY) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4SetPolicyEnforcement(serveripaddress : windows_sys::core::PCWSTR, fglobalpolicy : windows_sys::core::BOOL, subnetaddress : DHCP_IP_ADDRESS, enable : windows_sys::core::BOOL) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV4SetPolicyEx(serveripaddress : windows_sys::core::PCWSTR, fieldsmodified : super::DWORD, globalpolicy : windows_sys::core::BOOL, subnetaddress : DHCP_IP_ADDRESS, policyname : windows_sys::core::PCWSTR, policy : LPDHCP_POLICY_EX) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV6CreateClientInfo(serveripaddress : *const super::WCHAR, clientinfo : LPDHCP_CLIENT_INFO_V6) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV6GetFreeIPAddress(serveripaddress : windows_sys::core::PCWSTR, scopeid : DHCP_IPV6_ADDRESS, startip : DHCP_IPV6_ADDRESS, endip : DHCP_IPV6_ADDRESS, numfreeaddrreq : super::DWORD, ipaddrlist : *mut LPDHCPV6_IP_ARRAY) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV6GetStatelessStatistics(serveripaddress : windows_sys::core::PCWSTR, statelessstats : *mut LPDHCPV6_STATELESS_STATS) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV6GetStatelessStoreParams(serveripaddress : windows_sys::core::PCWSTR, fserverlevel : windows_sys::core::BOOL, subnetaddress : DHCP_IPV6_ADDRESS, params : *mut LPDHCPV6_STATELESS_PARAMS) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("dhcpsapi.dll" "system" fn DhcpV6SetStatelessStoreParams(serveripaddress : windows_sys::core::PCWSTR, fserverlevel : windows_sys::core::BOOL, subnetaddress : DHCP_IPV6_ADDRESS, fieldmodified : super::DWORD, params : LPDHCPV6_STATELESS_PARAMS) -> super::DWORD);
pub const ADDRESS_TYPE_IANA: i32 = 0;
pub const ADDRESS_TYPE_IATA: i32 = 1;
pub const Allow: DHCP_FILTER_LIST_TYPE = 1;
pub const CHANGESTATE: i32 = 4;
pub const CLIENT_TYPE_BOOTP: i32 = 2;
pub const CLIENT_TYPE_BOTH: i32 = 3;
pub const CLIENT_TYPE_DHCP: i32 = 1;
pub const CLIENT_TYPE_NONE: i32 = 100;
pub const CLIENT_TYPE_RESERVATION_FLAG: i32 = 4;
pub const CLIENT_TYPE_UNSPECIFIED: i32 = 0;
pub const COMMUNICATION_INT: FSM_STATE = 4;
pub const CONFLICT_DONE: FSM_STATE = 7;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DATE_TIME {
    pub dwLowDateTime: super::DWORD,
    pub dwHighDateTime: super::DWORD,
}
pub const DEFAULTQUARSETTING: QuarantineStatus = 5;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCPDS_SERVER {
    pub Version: super::DWORD,
    pub ServerName: windows_sys::core::PWSTR,
    pub ServerAddress: super::DWORD,
    pub Flags: super::DWORD,
    pub State: super::DWORD,
    pub DsLocation: windows_sys::core::PWSTR,
    pub DsLocType: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCPDS_SERVERS {
    pub Flags: super::DWORD,
    pub NumElements: super::DWORD,
    pub Servers: LPDHCPDS_SERVER,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCPV4_FAILOVER_CLIENT_INFO {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_sys::core::PWSTR,
    pub ClientComment: windows_sys::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: super::BYTE,
    pub AddressState: super::BYTE,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_sys::core::BOOL,
    pub SentPotExpTime: super::DWORD,
    pub AckPotExpTime: super::DWORD,
    pub RecvPotExpTime: super::DWORD,
    pub StartTime: super::DWORD,
    pub CltLastTransTime: super::DWORD,
    pub LastBndUpdTime: super::DWORD,
    pub BndMsgStatus: super::DWORD,
    pub PolicyName: windows_sys::core::PWSTR,
    pub Flags: super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    pub NumElements: super::DWORD,
    pub Clients: *mut LPDHCPV4_FAILOVER_CLIENT_INFO,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCPV4_FAILOVER_CLIENT_INFO_EX {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_sys::core::PWSTR,
    pub ClientComment: windows_sys::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: super::BYTE,
    pub AddressState: super::BYTE,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_sys::core::BOOL,
    pub SentPotExpTime: super::DWORD,
    pub AckPotExpTime: super::DWORD,
    pub RecvPotExpTime: super::DWORD,
    pub StartTime: super::DWORD,
    pub CltLastTransTime: super::DWORD,
    pub LastBndUpdTime: super::DWORD,
    pub BndMsgStatus: super::DWORD,
    pub PolicyName: windows_sys::core::PWSTR,
    pub Flags: super::BYTE,
    pub AddressStateEx: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCPV6_BIND_ELEMENT {
    pub Flags: super::ULONG,
    pub fBoundToDHCPServer: windows_sys::core::BOOL,
    pub AdapterPrimaryAddress: DHCP_IPV6_ADDRESS,
    pub AdapterSubnetAddress: DHCP_IPV6_ADDRESS,
    pub IfDescription: windows_sys::core::PWSTR,
    pub IpV6IfIndex: super::DWORD,
    pub IfIdSize: super::ULONG,
    pub IfId: super::LPBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCPV6_BIND_ELEMENT_ARRAY {
    pub NumElements: super::DWORD,
    pub Elements: LPDHCPV6_BIND_ELEMENT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCPV6_IP_ARRAY {
    pub NumElements: super::DWORD,
    pub Elements: LPDHCP_IPV6_ADDRESS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCPV6_STATELESS_PARAMS {
    pub Status: windows_sys::core::BOOL,
    pub PurgeInterval: super::DWORD,
}
pub type DHCPV6_STATELESS_PARAM_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCPV6_STATELESS_SCOPE_STATS {
    pub SubnetAddress: DHCP_IPV6_ADDRESS,
    pub NumStatelessClientsAdded: super::ULONGLONG,
    pub NumStatelessClientsRemoved: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCPV6_STATELESS_STATS {
    pub NumScopes: super::DWORD,
    pub ScopeStats: LPDHCPV6_STATELESS_SCOPE_STATS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DHCP_ADDR_PATTERN {
    pub MatchHWType: windows_sys::core::BOOL,
    pub HWType: super::BYTE,
    pub IsWildcard: windows_sys::core::BOOL,
    pub Length: super::BYTE,
    pub Pattern: [super::BYTE; 255],
}
#[cfg(feature = "shared")]
impl Default for DHCP_ADDR_PATTERN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_ALL_OPTIONS {
    pub Flags: super::DWORD,
    pub NonVendorOptions: LPDHCP_OPTION_ARRAY,
    pub NumVendorOptions: super::DWORD,
    pub VendorOptions: *mut DHCP_ALL_OPTIONS_0,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_ALL_OPTIONS_0 {
    pub Option: DHCP_OPTION,
    pub VendorName: windows_sys::core::PWSTR,
    pub ClassName: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_ALL_OPTION_VALUES {
    pub Flags: super::DWORD,
    pub NumElements: super::DWORD,
    pub Options: *mut DHCP_ALL_OPTION_VALUES_0,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_ALL_OPTION_VALUES_0 {
    pub ClassName: windows_sys::core::PWSTR,
    pub VendorName: windows_sys::core::PWSTR,
    pub IsVendor: windows_sys::core::BOOL,
    pub OptionsArray: LPDHCP_OPTION_VALUE_ARRAY,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_ALL_OPTION_VALUES_PB {
    pub Flags: super::DWORD,
    pub NumElements: super::DWORD,
    pub Options: *mut DHCP_ALL_OPTION_VALUES_PB_0,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_ALL_OPTION_VALUES_PB_0 {
    pub PolicyName: windows_sys::core::PWSTR,
    pub VendorName: windows_sys::core::PWSTR,
    pub IsVendor: windows_sys::core::BOOL,
    pub OptionsArray: LPDHCP_OPTION_VALUE_ARRAY,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DHCP_ATTRIB {
    pub DhcpAttribId: DHCP_ATTRIB_ID,
    pub DhcpAttribType: super::ULONG,
    pub Anonymous: DHCP_ATTRIB_0,
}
#[cfg(feature = "shared")]
impl Default for DHCP_ATTRIB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DHCP_ATTRIB_0 {
    pub DhcpAttribBool: windows_sys::core::BOOL,
    pub DhcpAttribUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for DHCP_ATTRIB_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_ATTRIB_ARRAY {
    pub NumElements: super::ULONG,
    pub DhcpAttribs: LPDHCP_ATTRIB,
}
pub const DHCP_ATTRIB_BOOL_IS_ADMIN: i32 = 5;
pub const DHCP_ATTRIB_BOOL_IS_BINDING_AWARE: i32 = 4;
pub const DHCP_ATTRIB_BOOL_IS_DYNBOOTP: i32 = 2;
pub const DHCP_ATTRIB_BOOL_IS_PART_OF_DSDC: i32 = 3;
pub const DHCP_ATTRIB_BOOL_IS_ROGUE: i32 = 1;
pub type DHCP_ATTRIB_ID = u32;
pub const DHCP_ATTRIB_TYPE_BOOL: i32 = 1;
pub const DHCP_ATTRIB_TYPE_ULONG: i32 = 2;
pub const DHCP_ATTRIB_ULONG_RESTORE_STATUS: i32 = 6;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_BINARY_DATA {
    pub DataLength: super::DWORD,
    pub Data: *mut super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_BIND_ELEMENT {
    pub Flags: super::ULONG,
    pub fBoundToDHCPServer: windows_sys::core::BOOL,
    pub AdapterPrimaryAddress: DHCP_IP_ADDRESS,
    pub AdapterSubnetAddress: DHCP_IP_ADDRESS,
    pub IfDescription: windows_sys::core::PWSTR,
    pub IfIdSize: super::ULONG,
    pub IfId: super::LPBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_BIND_ELEMENT_ARRAY {
    pub NumElements: super::DWORD,
    pub Elements: LPDHCP_BIND_ELEMENT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_BOOTP_IP_RANGE {
    pub StartAddress: DHCP_IP_ADDRESS,
    pub EndAddress: DHCP_IP_ADDRESS,
    pub BootpAllocated: super::ULONG,
    pub MaxBootpAllowed: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLASS_INFO {
    pub ClassName: windows_sys::core::PWSTR,
    pub ClassComment: windows_sys::core::PWSTR,
    pub ClassDataLength: super::DWORD,
    pub IsVendor: windows_sys::core::BOOL,
    pub Flags: super::DWORD,
    pub ClassData: super::LPBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLASS_INFO_ARRAY {
    pub NumElements: super::DWORD,
    pub Classes: LPDHCP_CLASS_INFO,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLASS_INFO_ARRAY_V6 {
    pub NumElements: super::DWORD,
    pub Classes: LPDHCP_CLASS_INFO_V6,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLASS_INFO_V6 {
    pub ClassName: windows_sys::core::PWSTR,
    pub ClassComment: windows_sys::core::PWSTR,
    pub ClassDataLength: super::DWORD,
    pub IsVendor: windows_sys::core::BOOL,
    pub EnterpriseNumber: super::DWORD,
    pub Flags: super::DWORD,
    pub ClassData: super::LPBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLIENT_FILTER_STATUS_INFO {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_sys::core::PWSTR,
    pub ClientComment: windows_sys::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: super::BYTE,
    pub AddressState: super::BYTE,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_sys::core::BOOL,
    pub FilterStatus: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    pub NumElements: super::DWORD,
    pub Clients: *mut LPDHCP_CLIENT_FILTER_STATUS_INFO,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLIENT_INFO {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_sys::core::PWSTR,
    pub ClientComment: windows_sys::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLIENT_INFO_ARRAY {
    pub NumElements: super::DWORD,
    pub Clients: *mut LPDHCP_CLIENT_INFO,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLIENT_INFO_ARRAY_V4 {
    pub NumElements: super::DWORD,
    pub Clients: *mut LPDHCP_CLIENT_INFO_V4,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLIENT_INFO_ARRAY_V5 {
    pub NumElements: super::DWORD,
    pub Clients: *mut LPDHCP_CLIENT_INFO_V5,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLIENT_INFO_ARRAY_V6 {
    pub NumElements: super::DWORD,
    pub Clients: *mut LPDHCP_CLIENT_INFO_V6,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLIENT_INFO_ARRAY_VQ {
    pub NumElements: super::DWORD,
    pub Clients: *mut LPDHCP_CLIENT_INFO_VQ,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLIENT_INFO_EX {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_sys::core::PWSTR,
    pub ClientComment: windows_sys::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: super::BYTE,
    pub AddressState: super::BYTE,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_sys::core::BOOL,
    pub FilterStatus: super::DWORD,
    pub PolicyName: windows_sys::core::PWSTR,
    pub Properties: LPDHCP_PROPERTY_ARRAY,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLIENT_INFO_EX_ARRAY {
    pub NumElements: super::DWORD,
    pub Clients: *mut LPDHCP_CLIENT_INFO_EX,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLIENT_INFO_PB {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_sys::core::PWSTR,
    pub ClientComment: windows_sys::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: super::BYTE,
    pub AddressState: super::BYTE,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_sys::core::BOOL,
    pub FilterStatus: super::DWORD,
    pub PolicyName: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLIENT_INFO_PB_ARRAY {
    pub NumElements: super::DWORD,
    pub Clients: *mut LPDHCP_CLIENT_INFO_PB,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLIENT_INFO_V4 {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_sys::core::PWSTR,
    pub ClientComment: windows_sys::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLIENT_INFO_V5 {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_sys::core::PWSTR,
    pub ClientComment: windows_sys::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: super::BYTE,
    pub AddressState: super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLIENT_INFO_V6 {
    pub ClientIpAddress: DHCP_IPV6_ADDRESS,
    pub ClientDUID: DHCP_CLIENT_UID,
    pub AddressType: super::DWORD,
    pub IAID: super::DWORD,
    pub ClientName: windows_sys::core::PWSTR,
    pub ClientComment: windows_sys::core::PWSTR,
    pub ClientValidLeaseExpires: DATE_TIME,
    pub ClientPrefLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO_V6,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_CLIENT_INFO_VQ {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_sys::core::PWSTR,
    pub ClientComment: windows_sys::core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: super::BYTE,
    pub AddressState: super::BYTE,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_sys::core::BOOL,
}
#[cfg(feature = "shared")]
pub type DHCP_CLIENT_UID = DHCP_BINARY_DATA;
pub const DHCP_ENDPOINT_FLAG_CANT_MODIFY: i32 = 1;
pub const DHCP_FAILOVER_DELETE_SCOPES: i32 = 1;
pub const DHCP_FAILOVER_MAX_NUM_ADD_SCOPES: i32 = 400;
pub const DHCP_FAILOVER_MAX_NUM_REL: i32 = 31;
pub type DHCP_FAILOVER_MODE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_FAILOVER_RELATIONSHIP {
    pub PrimaryServer: DHCP_IP_ADDRESS,
    pub SecondaryServer: DHCP_IP_ADDRESS,
    pub Mode: DHCP_FAILOVER_MODE,
    pub ServerType: DHCP_FAILOVER_SERVER,
    pub State: FSM_STATE,
    pub PrevState: FSM_STATE,
    pub Mclt: super::DWORD,
    pub SafePeriod: super::DWORD,
    pub RelationshipName: windows_sys::core::PWSTR,
    pub PrimaryServerName: windows_sys::core::PWSTR,
    pub SecondaryServerName: windows_sys::core::PWSTR,
    pub pScopes: LPDHCP_IP_ARRAY,
    pub Percentage: super::BYTE,
    pub SharedSecret: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    pub NumElements: super::DWORD,
    pub pRelationships: LPDHCP_FAILOVER_RELATIONSHIP,
}
pub type DHCP_FAILOVER_SERVER = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_FAILOVER_STATISTICS {
    pub NumAddr: super::DWORD,
    pub AddrFree: super::DWORD,
    pub AddrInUse: super::DWORD,
    pub PartnerAddrFree: super::DWORD,
    pub ThisAddrFree: super::DWORD,
    pub PartnerAddrInUse: super::DWORD,
    pub ThisAddrInUse: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_FILTER_ADD_INFO {
    pub AddrPatt: DHCP_ADDR_PATTERN,
    pub Comment: windows_sys::core::PWSTR,
    pub ListType: DHCP_FILTER_LIST_TYPE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_FILTER_ENUM_INFO {
    pub NumElements: super::DWORD,
    pub pEnumRecords: LPDHCP_FILTER_RECORD,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DHCP_FILTER_GLOBAL_INFO {
    pub EnforceAllowList: windows_sys::core::BOOL,
    pub EnforceDenyList: windows_sys::core::BOOL,
}
pub type DHCP_FILTER_LIST_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_FILTER_RECORD {
    pub AddrPatt: DHCP_ADDR_PATTERN,
    pub Comment: windows_sys::core::PWSTR,
}
pub const DHCP_FLAGS_DONT_ACCESS_DS: i32 = 1;
pub const DHCP_FLAGS_DONT_DO_RPC: i32 = 2;
pub const DHCP_FLAGS_OPTION_IS_VENDOR: i32 = 3;
pub type DHCP_FORCE_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DHCP_HOST_INFO {
    pub IpAddress: DHCP_IP_ADDRESS,
    pub NetBiosName: windows_sys::core::PWSTR,
    pub HostName: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_HOST_INFO_V6 {
    pub IpAddress: DHCP_IPV6_ADDRESS,
    pub NetBiosName: windows_sys::core::PWSTR,
    pub HostName: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_IPV6_ADDRESS {
    pub HighOrderBits: super::ULONGLONG,
    pub LowOrderBits: super::ULONGLONG,
}
pub type DHCP_IP_ADDRESS = u32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_IP_ARRAY {
    pub NumElements: super::DWORD,
    pub Elements: LPDHCP_IP_ADDRESS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_IP_CLUSTER {
    pub ClusterAddress: DHCP_IP_ADDRESS,
    pub ClusterMask: super::DWORD,
}
pub type DHCP_IP_MASK = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DHCP_IP_RANGE {
    pub StartAddress: DHCP_IP_ADDRESS,
    pub EndAddress: DHCP_IP_ADDRESS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_IP_RANGE_ARRAY {
    pub NumElements: super::DWORD,
    pub Elements: LPDHCP_IP_RANGE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_IP_RANGE_V6 {
    pub StartAddress: DHCP_IPV6_ADDRESS,
    pub EndAddress: DHCP_IPV6_ADDRESS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_IP_RESERVATION {
    pub ReservedIpAddress: DHCP_IP_ADDRESS,
    pub ReservedForClient: *mut DHCP_CLIENT_UID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_IP_RESERVATION_INFO {
    pub ReservedIpAddress: DHCP_IP_ADDRESS,
    pub ReservedForClient: DHCP_CLIENT_UID,
    pub ReservedClientName: windows_sys::core::PWSTR,
    pub ReservedClientDesc: windows_sys::core::PWSTR,
    pub bAllowedClientTypes: super::BYTE,
    pub fOptionsPresent: super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_IP_RESERVATION_V4 {
    pub ReservedIpAddress: DHCP_IP_ADDRESS,
    pub ReservedForClient: *mut DHCP_CLIENT_UID,
    pub bAllowedClientTypes: super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_IP_RESERVATION_V6 {
    pub ReservedIpAddress: DHCP_IPV6_ADDRESS,
    pub ReservedForClient: *mut DHCP_CLIENT_UID,
    pub InterfaceId: super::DWORD,
}
pub const DHCP_MAX_DELAY: i32 = 1000;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_MIB_INFO {
    pub Discovers: super::DWORD,
    pub Offers: super::DWORD,
    pub Requests: super::DWORD,
    pub Acks: super::DWORD,
    pub Naks: super::DWORD,
    pub Declines: super::DWORD,
    pub Releases: super::DWORD,
    pub ServerStartTime: DATE_TIME,
    pub Scopes: super::DWORD,
    pub ScopeInfo: LPSCOPE_MIB_INFO,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_MIB_INFO_V5 {
    pub Discovers: super::DWORD,
    pub Offers: super::DWORD,
    pub Requests: super::DWORD,
    pub Acks: super::DWORD,
    pub Naks: super::DWORD,
    pub Declines: super::DWORD,
    pub Releases: super::DWORD,
    pub ServerStartTime: DATE_TIME,
    pub QtnNumLeases: super::DWORD,
    pub QtnPctQtnLeases: super::DWORD,
    pub QtnProbationLeases: super::DWORD,
    pub QtnNonQtnLeases: super::DWORD,
    pub QtnExemptLeases: super::DWORD,
    pub QtnCapableClients: super::DWORD,
    pub QtnIASErrors: super::DWORD,
    pub DelayedOffers: super::DWORD,
    pub ScopesWithDelayedOffers: super::DWORD,
    pub Scopes: super::DWORD,
    pub ScopeInfo: LPSCOPE_MIB_INFO_V5,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_MIB_INFO_V6 {
    pub Solicits: super::DWORD,
    pub Advertises: super::DWORD,
    pub Requests: super::DWORD,
    pub Renews: super::DWORD,
    pub Rebinds: super::DWORD,
    pub Replies: super::DWORD,
    pub Confirms: super::DWORD,
    pub Declines: super::DWORD,
    pub Releases: super::DWORD,
    pub Informs: super::DWORD,
    pub ServerStartTime: DATE_TIME,
    pub Scopes: super::DWORD,
    pub ScopeInfo: LPSCOPE_MIB_INFO_V6,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_MIB_INFO_VQ {
    pub Discovers: super::DWORD,
    pub Offers: super::DWORD,
    pub Requests: super::DWORD,
    pub Acks: super::DWORD,
    pub Naks: super::DWORD,
    pub Declines: super::DWORD,
    pub Releases: super::DWORD,
    pub ServerStartTime: DATE_TIME,
    pub QtnNumLeases: super::DWORD,
    pub QtnPctQtnLeases: super::DWORD,
    pub QtnProbationLeases: super::DWORD,
    pub QtnNonQtnLeases: super::DWORD,
    pub QtnExemptLeases: super::DWORD,
    pub QtnCapableClients: super::DWORD,
    pub QtnIASErrors: super::DWORD,
    pub Scopes: super::DWORD,
    pub ScopeInfo: LPSCOPE_MIB_INFO_VQ,
}
pub const DHCP_MIN_DELAY: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_OPTION {
    pub OptionID: DHCP_OPTION_ID,
    pub OptionName: windows_sys::core::PWSTR,
    pub OptionComment: windows_sys::core::PWSTR,
    pub DefaultValue: DHCP_OPTION_DATA,
    pub OptionType: DHCP_OPTION_TYPE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_OPTION_ARRAY {
    pub NumElements: super::DWORD,
    pub Options: LPDHCP_OPTION,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_OPTION_DATA {
    pub NumElements: super::DWORD,
    pub Elements: LPDHCP_OPTION_DATA_ELEMENT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DHCP_OPTION_DATA_ELEMENT {
    pub OptionType: DHCP_OPTION_DATA_TYPE,
    pub Element: DHCP_OPTION_DATA_ELEMENT_0,
}
#[cfg(feature = "shared")]
impl Default for DHCP_OPTION_DATA_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DHCP_OPTION_DATA_ELEMENT_0 {
    pub ByteOption: super::BYTE,
    pub WordOption: super::WORD,
    pub DWordOption: super::DWORD,
    pub DWordDWordOption: DWORD_DWORD,
    pub IpAddressOption: DHCP_IP_ADDRESS,
    pub StringDataOption: windows_sys::core::PWSTR,
    pub BinaryDataOption: DHCP_BINARY_DATA,
    pub EncapsulatedDataOption: DHCP_BINARY_DATA,
    pub Ipv6AddressDataOption: windows_sys::core::PWSTR,
}
#[cfg(feature = "shared")]
impl Default for DHCP_OPTION_DATA_ELEMENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DHCP_OPTION_DATA_TYPE = i32;
pub type DHCP_OPTION_ID = u32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_OPTION_LIST {
    pub NumOptions: super::DWORD,
    pub Options: *mut DHCP_OPTION_VALUE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DHCP_OPTION_SCOPE_INFO {
    pub ScopeType: DHCP_OPTION_SCOPE_TYPE,
    pub ScopeInfo: DHCP_OPTION_SCOPE_INFO_0,
}
#[cfg(feature = "winnt")]
impl Default for DHCP_OPTION_SCOPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DHCP_OPTION_SCOPE_INFO_0 {
    pub DefaultScopeInfo: super::PVOID,
    pub GlobalScopeInfo: super::PVOID,
    pub SubnetScopeInfo: DHCP_IP_ADDRESS,
    pub ReservedScopeInfo: DHCP_RESERVED_SCOPE,
    pub MScopeInfo: windows_sys::core::PWSTR,
}
#[cfg(feature = "winnt")]
impl Default for DHCP_OPTION_SCOPE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DHCP_OPTION_SCOPE_INFO6 {
    pub ScopeType: DHCP_OPTION_SCOPE_TYPE6,
    pub ScopeInfo: DHCP_OPTION_SCOPE_INFO6_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DHCP_OPTION_SCOPE_INFO6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DHCP_OPTION_SCOPE_INFO6_0 {
    pub DefaultScopeInfo: super::PVOID,
    pub SubnetScopeInfo: DHCP_IPV6_ADDRESS,
    pub ReservedScopeInfo: DHCP_RESERVED_SCOPE6,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DHCP_OPTION_SCOPE_INFO6_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DHCP_OPTION_SCOPE_TYPE = i32;
pub type DHCP_OPTION_SCOPE_TYPE6 = i32;
pub type DHCP_OPTION_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_OPTION_VALUE {
    pub OptionID: DHCP_OPTION_ID,
    pub Value: DHCP_OPTION_DATA,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_OPTION_VALUE_ARRAY {
    pub NumElements: super::DWORD,
    pub Values: LPDHCP_OPTION_VALUE,
}
pub const DHCP_OPT_ENUM_IGNORE_VENDOR: i32 = 1;
pub const DHCP_OPT_ENUM_USE_CLASSNAME: i32 = 2;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_PERF_STATS {
    pub dwNumPacketsReceived: super::ULONG,
    pub dwNumPacketsDuplicate: super::ULONG,
    pub dwNumPacketsExpired: super::ULONG,
    pub dwNumMilliSecondsProcessed: super::ULONG,
    pub dwNumPacketsInActiveQueue: super::ULONG,
    pub dwNumPacketsInPingQueue: super::ULONG,
    pub dwNumDiscoversReceived: super::ULONG,
    pub dwNumOffersSent: super::ULONG,
    pub dwNumRequestsReceived: super::ULONG,
    pub dwNumInformsReceived: super::ULONG,
    pub dwNumAcksSent: super::ULONG,
    pub dwNumNacksSent: super::ULONG,
    pub dwNumDeclinesReceived: super::ULONG,
    pub dwNumReleasesReceived: super::ULONG,
    pub dwNumDelayedOfferInQueue: super::ULONG,
    pub dwNumPacketsProcessed: super::ULONG,
    pub dwNumPacketsInQuarWaitingQueue: super::ULONG,
    pub dwNumPacketsInQuarReadyQueue: super::ULONG,
    pub dwNumPacketsInQuarDecisionQueue: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_POLICY {
    pub PolicyName: windows_sys::core::PWSTR,
    pub IsGlobalPolicy: windows_sys::core::BOOL,
    pub Subnet: DHCP_IP_ADDRESS,
    pub ProcessingOrder: super::DWORD,
    pub Conditions: LPDHCP_POL_COND_ARRAY,
    pub Expressions: LPDHCP_POL_EXPR_ARRAY,
    pub Ranges: LPDHCP_IP_RANGE_ARRAY,
    pub Description: windows_sys::core::PWSTR,
    pub Enabled: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_POLICY_ARRAY {
    pub NumElements: super::DWORD,
    pub Elements: LPDHCP_POLICY,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_POLICY_EX {
    pub PolicyName: windows_sys::core::PWSTR,
    pub IsGlobalPolicy: windows_sys::core::BOOL,
    pub Subnet: DHCP_IP_ADDRESS,
    pub ProcessingOrder: super::DWORD,
    pub Conditions: LPDHCP_POL_COND_ARRAY,
    pub Expressions: LPDHCP_POL_EXPR_ARRAY,
    pub Ranges: LPDHCP_IP_RANGE_ARRAY,
    pub Description: windows_sys::core::PWSTR,
    pub Enabled: windows_sys::core::BOOL,
    pub Properties: LPDHCP_PROPERTY_ARRAY,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_POLICY_EX_ARRAY {
    pub NumElements: super::DWORD,
    pub Elements: LPDHCP_POLICY_EX,
}
pub type DHCP_POLICY_FIELDS_TO_UPDATE = i32;
pub type DHCP_POL_ATTR_TYPE = i32;
pub type DHCP_POL_COMPARATOR = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_POL_COND {
    pub ParentExpr: super::DWORD,
    pub Type: DHCP_POL_ATTR_TYPE,
    pub OptionID: super::DWORD,
    pub SubOptionID: super::DWORD,
    pub VendorName: windows_sys::core::PWSTR,
    pub Operator: DHCP_POL_COMPARATOR,
    pub Value: super::LPBYTE,
    pub ValueLength: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_POL_COND_ARRAY {
    pub NumElements: super::DWORD,
    pub Elements: LPDHCP_POL_COND,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_POL_EXPR {
    pub ParentExpr: super::DWORD,
    pub Operator: DHCP_POL_LOGIC_OPER,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_POL_EXPR_ARRAY {
    pub NumElements: super::DWORD,
    pub Elements: LPDHCP_POL_EXPR,
}
pub type DHCP_POL_LOGIC_OPER = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DHCP_PROPERTY {
    pub ID: DHCP_PROPERTY_ID,
    pub Type: DHCP_PROPERTY_TYPE,
    pub Value: DHCP_PROPERTY_0,
}
#[cfg(feature = "shared")]
impl Default for DHCP_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DHCP_PROPERTY_0 {
    pub ByteValue: super::BYTE,
    pub WordValue: super::WORD,
    pub DWordValue: super::DWORD,
    pub StringValue: windows_sys::core::PWSTR,
    pub BinaryValue: DHCP_BINARY_DATA,
}
#[cfg(feature = "shared")]
impl Default for DHCP_PROPERTY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_PROPERTY_ARRAY {
    pub NumElements: super::DWORD,
    pub Elements: LPDHCP_PROPERTY,
}
pub type DHCP_PROPERTY_ID = i32;
pub type DHCP_PROPERTY_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_RESERVATION_INFO_ARRAY {
    pub NumElements: super::DWORD,
    pub Elements: *mut LPDHCP_IP_RESERVATION_INFO,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DHCP_RESERVED_SCOPE {
    pub ReservedIpAddress: DHCP_IP_ADDRESS,
    pub ReservedIpSubnetAddress: DHCP_IP_ADDRESS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_RESERVED_SCOPE6 {
    pub ReservedIpAddress: DHCP_IPV6_ADDRESS,
    pub ReservedIpSubnetAddress: DHCP_IPV6_ADDRESS,
}
pub type DHCP_RESUME_HANDLE = u32;
#[cfg(feature = "shared")]
pub type DHCP_RESUME_IPV6_HANDLE = DHCP_IPV6_ADDRESS;
pub type DHCP_SCAN_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DHCP_SCAN_ITEM {
    pub IpAddress: DHCP_IP_ADDRESS,
    pub ScanFlag: DHCP_SCAN_FLAG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_SCAN_LIST {
    pub NumScanItems: super::DWORD,
    pub ScanItems: *mut DHCP_SCAN_ITEM,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DHCP_SEARCH_INFO {
    pub SearchType: DHCP_SEARCH_INFO_TYPE,
    pub SearchInfo: DHCP_SEARCH_INFO_0,
}
#[cfg(feature = "shared")]
impl Default for DHCP_SEARCH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DHCP_SEARCH_INFO_0 {
    pub ClientIpAddress: DHCP_IP_ADDRESS,
    pub ClientHardwareAddress: DHCP_CLIENT_UID,
    pub ClientName: windows_sys::core::PWSTR,
}
#[cfg(feature = "shared")]
impl Default for DHCP_SEARCH_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DHCP_SEARCH_INFO_TYPE = i32;
pub type DHCP_SEARCH_INFO_TYPE_V6 = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DHCP_SEARCH_INFO_V6 {
    pub SearchType: DHCP_SEARCH_INFO_TYPE_V6,
    pub SearchInfo: DHCP_SEARCH_INFO_V6_0,
}
#[cfg(feature = "shared")]
impl Default for DHCP_SEARCH_INFO_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DHCP_SEARCH_INFO_V6_0 {
    pub ClientIpAddress: DHCP_IPV6_ADDRESS,
    pub ClientDUID: DHCP_CLIENT_UID,
    pub ClientName: windows_sys::core::PWSTR,
}
#[cfg(feature = "shared")]
impl Default for DHCP_SEARCH_INFO_V6_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_SERVER_CONFIG_INFO {
    pub APIProtocolSupport: super::DWORD,
    pub DatabaseName: windows_sys::core::PWSTR,
    pub DatabasePath: windows_sys::core::PWSTR,
    pub BackupPath: windows_sys::core::PWSTR,
    pub BackupInterval: super::DWORD,
    pub DatabaseLoggingFlag: super::DWORD,
    pub RestoreFlag: super::DWORD,
    pub DatabaseCleanupInterval: super::DWORD,
    pub DebugFlag: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DHCP_SERVER_CONFIG_INFO_V4 {
    pub APIProtocolSupport: super::DWORD,
    pub DatabaseName: windows_sys::core::PWSTR,
    pub DatabasePath: windows_sys::core::PWSTR,
    pub BackupPath: windows_sys::core::PWSTR,
    pub BackupInterval: super::DWORD,
    pub DatabaseLoggingFlag: super::DWORD,
    pub RestoreFlag: super::DWORD,
    pub DatabaseCleanupInterval: super::DWORD,
    pub DebugFlag: super::DWORD,
    pub dwPingRetries: super::DWORD,
    pub cbBootTableString: super::DWORD,
    pub wszBootTableString: *mut super::WCHAR,
    pub fAuditLog: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_SERVER_CONFIG_INFO_V6 {
    pub UnicastFlag: windows_sys::core::BOOL,
    pub RapidCommitFlag: windows_sys::core::BOOL,
    pub PreferredLifetime: super::DWORD,
    pub ValidLifetime: super::DWORD,
    pub T1: super::DWORD,
    pub T2: super::DWORD,
    pub PreferredLifetimeIATA: super::DWORD,
    pub ValidLifetimeIATA: super::DWORD,
    pub fAuditLog: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DHCP_SERVER_CONFIG_INFO_VQ {
    pub APIProtocolSupport: super::DWORD,
    pub DatabaseName: windows_sys::core::PWSTR,
    pub DatabasePath: windows_sys::core::PWSTR,
    pub BackupPath: windows_sys::core::PWSTR,
    pub BackupInterval: super::DWORD,
    pub DatabaseLoggingFlag: super::DWORD,
    pub RestoreFlag: super::DWORD,
    pub DatabaseCleanupInterval: super::DWORD,
    pub DebugFlag: super::DWORD,
    pub dwPingRetries: super::DWORD,
    pub cbBootTableString: super::DWORD,
    pub wszBootTableString: *mut super::WCHAR,
    pub fAuditLog: windows_sys::core::BOOL,
    pub QuarantineOn: windows_sys::core::BOOL,
    pub QuarDefFail: super::DWORD,
    pub QuarRuntimeStatus: windows_sys::core::BOOL,
}
#[cfg(feature = "shared")]
pub type DHCP_SERVER_INFO = DHCPDS_SERVER;
#[cfg(feature = "shared")]
pub type DHCP_SERVER_INFO_ARRAY = DHCPDS_SERVERS;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DHCP_SERVER_SPECIFIC_STRINGS {
    pub DefaultVendorClassName: windows_sys::core::PWSTR,
    pub DefaultUserClassName: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DHCP_SUBNET_ELEMENT_DATA {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_0,
}
#[cfg(feature = "shared")]
impl Default for DHCP_SUBNET_ELEMENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DHCP_SUBNET_ELEMENT_DATA_0 {
    pub IpRange: *mut DHCP_IP_RANGE,
    pub SecondaryHost: *mut DHCP_HOST_INFO,
    pub ReservedIp: *mut DHCP_IP_RESERVATION,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE,
    pub IpUsedCluster: *mut DHCP_IP_CLUSTER,
}
#[cfg(feature = "shared")]
impl Default for DHCP_SUBNET_ELEMENT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DHCP_SUBNET_ELEMENT_DATA_V4 {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_V4_0,
}
#[cfg(feature = "shared")]
impl Default for DHCP_SUBNET_ELEMENT_DATA_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DHCP_SUBNET_ELEMENT_DATA_V4_0 {
    pub IpRange: *mut DHCP_IP_RANGE,
    pub SecondaryHost: *mut DHCP_HOST_INFO,
    pub ReservedIp: *mut DHCP_IP_RESERVATION_V4,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE,
    pub IpUsedCluster: *mut DHCP_IP_CLUSTER,
}
#[cfg(feature = "shared")]
impl Default for DHCP_SUBNET_ELEMENT_DATA_V4_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DHCP_SUBNET_ELEMENT_DATA_V5 {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_V5_0,
}
#[cfg(feature = "shared")]
impl Default for DHCP_SUBNET_ELEMENT_DATA_V5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DHCP_SUBNET_ELEMENT_DATA_V5_0 {
    pub IpRange: *mut DHCP_BOOTP_IP_RANGE,
    pub SecondaryHost: *mut DHCP_HOST_INFO,
    pub ReservedIp: *mut DHCP_IP_RESERVATION_V4,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE,
    pub IpUsedCluster: *mut DHCP_IP_CLUSTER,
}
#[cfg(feature = "shared")]
impl Default for DHCP_SUBNET_ELEMENT_DATA_V5_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DHCP_SUBNET_ELEMENT_DATA_V6 {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE_V6,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_V6_0,
}
#[cfg(feature = "shared")]
impl Default for DHCP_SUBNET_ELEMENT_DATA_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    pub IpRange: *mut DHCP_IP_RANGE_V6,
    pub ReservedIp: *mut DHCP_IP_RESERVATION_V6,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE_V6,
}
#[cfg(feature = "shared")]
impl Default for DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    pub NumElements: super::DWORD,
    pub Elements: LPDHCP_SUBNET_ELEMENT_DATA,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    pub NumElements: super::DWORD,
    pub Elements: LPDHCP_SUBNET_ELEMENT_DATA_V4,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    pub NumElements: super::DWORD,
    pub Elements: LPDHCP_SUBNET_ELEMENT_DATA_V5,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    pub NumElements: super::DWORD,
    pub Elements: LPDHCP_SUBNET_ELEMENT_DATA_V6,
}
pub type DHCP_SUBNET_ELEMENT_TYPE = i32;
pub type DHCP_SUBNET_ELEMENT_TYPE_V6 = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DHCP_SUBNET_INFO {
    pub SubnetAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub SubnetName: windows_sys::core::PWSTR,
    pub SubnetComment: windows_sys::core::PWSTR,
    pub PrimaryHost: DHCP_HOST_INFO,
    pub SubnetState: DHCP_SUBNET_STATE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_SUBNET_INFO_V6 {
    pub SubnetAddress: DHCP_IPV6_ADDRESS,
    pub Prefix: super::ULONG,
    pub Preference: super::USHORT,
    pub SubnetName: windows_sys::core::PWSTR,
    pub SubnetComment: windows_sys::core::PWSTR,
    pub State: super::DWORD,
    pub ScopeId: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_SUBNET_INFO_VQ {
    pub SubnetAddress: DHCP_IP_ADDRESS,
    pub SubnetMask: DHCP_IP_MASK,
    pub SubnetName: windows_sys::core::PWSTR,
    pub SubnetComment: windows_sys::core::PWSTR,
    pub PrimaryHost: DHCP_HOST_INFO,
    pub SubnetState: DHCP_SUBNET_STATE,
    pub QuarantineOn: super::DWORD,
    pub Reserved1: super::DWORD,
    pub Reserved2: super::DWORD,
    pub Reserved3: super::INT64,
    pub Reserved4: super::INT64,
}
pub const DHCP_SUBNET_INFO_VQ_FLAG_QUARANTINE: i32 = 1;
pub type DHCP_SUBNET_STATE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_SUPER_SCOPE_TABLE {
    pub cEntries: super::DWORD,
    pub pEntries: *mut DHCP_SUPER_SCOPE_TABLE_ENTRY,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DHCP_SUPER_SCOPE_TABLE_ENTRY {
    pub SubnetAddress: DHCP_IP_ADDRESS,
    pub SuperScopeNumber: super::DWORD,
    pub NextInSuperScope: super::DWORD,
    pub SuperScopeName: windows_sys::core::PWSTR,
}
pub const DNS_FLAG_CLEANUP_EXPIRED: i32 = 4;
pub const DNS_FLAG_DISABLE_PTR_UPDATE: i32 = 64;
pub const DNS_FLAG_ENABLED: i32 = 1;
pub const DNS_FLAG_HAS_DNS_SUFFIX: i32 = 128;
pub const DNS_FLAG_UPDATE_BOTH_ALWAYS: i32 = 16;
pub const DNS_FLAG_UPDATE_DHCID: i32 = 32;
pub const DNS_FLAG_UPDATE_DOWNLEVEL: i32 = 2;
pub const DROPPACKET: QuarantineStatus = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DWORD_DWORD {
    pub DWord1: super::DWORD,
    pub DWord2: super::DWORD,
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
pub const ERROR_DDS_CLASS_DOES_NOT_EXIST: i32 = 20078;
pub const ERROR_DDS_CLASS_EXISTS: i32 = 20077;
pub const ERROR_DDS_DHCP_SERVER_NOT_FOUND: i32 = 20074;
pub const ERROR_DDS_NO_DHCP_ROOT: i32 = 20071;
pub const ERROR_DDS_NO_DS_AVAILABLE: i32 = 20070;
pub const ERROR_DDS_OPTION_ALREADY_EXISTS: i32 = 20075;
pub const ERROR_DDS_OPTION_DOES_NOT_EXIST: i32 = 20076;
pub const ERROR_DDS_POSSIBLE_RANGE_CONFLICT: i32 = 20087;
pub const ERROR_DDS_RANGE_DOES_NOT_EXIST: i32 = 20088;
pub const ERROR_DDS_RESERVATION_CONFLICT: i32 = 20086;
pub const ERROR_DDS_RESERVATION_NOT_PRESENT: i32 = 20085;
pub const ERROR_DDS_SERVER_ADDRESS_MISMATCH: i32 = 20081;
pub const ERROR_DDS_SERVER_ALREADY_EXISTS: i32 = 20079;
pub const ERROR_DDS_SERVER_DOES_NOT_EXIST: i32 = 20080;
pub const ERROR_DDS_SUBNET_EXISTS: i32 = 20082;
pub const ERROR_DDS_SUBNET_HAS_DIFF_SSCOPE: i32 = 20083;
pub const ERROR_DDS_SUBNET_NOT_PRESENT: i32 = 20084;
pub const ERROR_DDS_TOO_MANY_ERRORS: i32 = 20073;
pub const ERROR_DDS_UNEXPECTED_ERROR: i32 = 20072;
pub const ERROR_DHCP_ADDRESS_NOT_AVAILABLE: i32 = 20011;
pub const ERROR_DHCP_CANNOT_MODIFY_BINDINGS: i32 = 20051;
pub const ERROR_DHCP_CANT_CHANGE_ATTRIBUTE: i32 = 20048;
pub const ERROR_DHCP_CLASS_ALREADY_EXISTS: i32 = 20045;
pub const ERROR_DHCP_CLASS_NOT_FOUND: i32 = 20044;
pub const ERROR_DHCP_CLIENT_EXISTS: i32 = 20014;
pub const ERROR_DHCP_DATABASE_INIT_FAILED: i32 = 20001;
pub const ERROR_DHCP_DEFAULT_SCOPE_EXITS: i32 = 20047;
pub const ERROR_DHCP_DELETE_BUILTIN_CLASS: i32 = 20089;
pub const ERROR_DHCP_ELEMENT_CANT_REMOVE: i32 = 20007;
pub const ERROR_DHCP_EXEMPTION_EXISTS: i32 = 20055;
pub const ERROR_DHCP_EXEMPTION_NOT_PRESENT: i32 = 20056;
pub const ERROR_DHCP_FO_ADDSCOPE_LEASES_NOT_SYNCED: i32 = 20127;
pub const ERROR_DHCP_FO_BOOT_NOT_SUPPORTED: i32 = 20131;
pub const ERROR_DHCP_FO_FEATURE_NOT_SUPPORTED: i32 = 20134;
pub const ERROR_DHCP_FO_IPRANGE_TYPE_CONV_ILLEGAL: i32 = 20129;
pub const ERROR_DHCP_FO_MAX_ADD_SCOPES: i32 = 20130;
pub const ERROR_DHCP_FO_MAX_RELATIONSHIPS: i32 = 20128;
pub const ERROR_DHCP_FO_NOT_SUPPORTED: i32 = 20118;
pub const ERROR_DHCP_FO_RANGE_PART_OF_REL: i32 = 20132;
pub const ERROR_DHCP_FO_RELATIONSHIP_DOES_NOT_EXIST: i32 = 20115;
pub const ERROR_DHCP_FO_RELATIONSHIP_EXISTS: i32 = 20114;
pub const ERROR_DHCP_FO_RELATIONSHIP_NAME_TOO_LONG: i32 = 20125;
pub const ERROR_DHCP_FO_RELATION_IS_SECONDARY: i32 = 20117;
pub const ERROR_DHCP_FO_SCOPE_ALREADY_IN_RELATIONSHIP: i32 = 20113;
pub const ERROR_DHCP_FO_SCOPE_NOT_IN_RELATIONSHIP: i32 = 20116;
pub const ERROR_DHCP_FO_SCOPE_SYNC_IN_PROGRESS: i32 = 20133;
pub const ERROR_DHCP_FO_STATE_NOT_NORMAL: i32 = 20120;
pub const ERROR_DHCP_FO_TIME_OUT_OF_SYNC: i32 = 20119;
pub const ERROR_DHCP_HARDWARE_ADDRESS_TYPE_ALREADY_EXEMPT: i32 = 20101;
pub const ERROR_DHCP_INVALID_DELAY: i32 = 20092;
pub const ERROR_DHCP_INVALID_DHCP_CLIENT: i32 = 20016;
pub const ERROR_DHCP_INVALID_DHCP_MESSAGE: i32 = 20015;
pub const ERROR_DHCP_INVALID_PARAMETER_OPTION32: i32 = 20057;
pub const ERROR_DHCP_INVALID_POLICY_EXPRESSION: i32 = 20109;
pub const ERROR_DHCP_INVALID_PROCESSING_ORDER: i32 = 20110;
pub const ERROR_DHCP_INVALID_RANGE: i32 = 20023;
pub const ERROR_DHCP_INVALID_SUBNET_PREFIX: i32 = 20091;
pub const ERROR_DHCP_IPRANGE_CONV_ILLEGAL: i32 = 20049;
pub const ERROR_DHCP_IPRANGE_EXITS: i32 = 20021;
pub const ERROR_DHCP_IP_ADDRESS_IN_USE: i32 = 20032;
pub const ERROR_DHCP_JET97_CONV_REQUIRED: i32 = 20036;
pub const ERROR_DHCP_JET_CONV_REQUIRED: i32 = 20027;
pub const ERROR_DHCP_JET_ERROR: i32 = 20013;
pub const ERROR_DHCP_LINKLAYER_ADDRESS_DOES_NOT_EXIST: i32 = 20095;
pub const ERROR_DHCP_LINKLAYER_ADDRESS_EXISTS: i32 = 20093;
pub const ERROR_DHCP_LINKLAYER_ADDRESS_RESERVATION_EXISTS: i32 = 20094;
pub const ERROR_DHCP_LOG_FILE_PATH_TOO_LONG: i32 = 20033;
pub const ERROR_DHCP_MSCOPE_EXISTS: i32 = 20053;
pub const ERROR_DHCP_NAP_NOT_SUPPORTED: i32 = 20138;
pub const ERROR_DHCP_NETWORK_CHANGED: i32 = 20050;
pub const ERROR_DHCP_NETWORK_INIT_FAILED: i32 = 20003;
pub const ERROR_DHCP_NOT_RESERVED_CLIENT: i32 = 20018;
pub const ERROR_DHCP_NO_ADMIN_PERMISSION: i32 = 20121;
pub const ERROR_DHCP_OPTION_EXITS: i32 = 20009;
pub const ERROR_DHCP_OPTION_NOT_PRESENT: i32 = 20010;
pub const ERROR_DHCP_OPTION_TYPE_MISMATCH: i32 = 20103;
pub const ERROR_DHCP_POLICY_BAD_PARENT_EXPR: i32 = 20104;
pub const ERROR_DHCP_POLICY_EDIT_FQDN_UNSUPPORTED: i32 = 20137;
pub const ERROR_DHCP_POLICY_EXISTS: i32 = 20105;
pub const ERROR_DHCP_POLICY_FQDN_OPTION_UNSUPPORTED: i32 = 20136;
pub const ERROR_DHCP_POLICY_FQDN_RANGE_UNSUPPORTED: i32 = 20135;
pub const ERROR_DHCP_POLICY_NOT_FOUND: i32 = 20111;
pub const ERROR_DHCP_POLICY_RANGE_BAD: i32 = 20107;
pub const ERROR_DHCP_POLICY_RANGE_EXISTS: i32 = 20106;
pub const ERROR_DHCP_PRIMARY_NOT_FOUND: i32 = 20006;
pub const ERROR_DHCP_RANGE_EXTENDED: i32 = 20024;
pub const ERROR_DHCP_RANGE_FULL: i32 = 20012;
pub const ERROR_DHCP_RANGE_INVALID_IN_SERVER_POLICY: i32 = 20108;
pub const ERROR_DHCP_RANGE_TOO_SMALL: i32 = 20020;
pub const ERROR_DHCP_REACHED_END_OF_SELECTION: i32 = 20126;
pub const ERROR_DHCP_REGISTRY_INIT_FAILED: i32 = 20000;
pub const ERROR_DHCP_RESERVEDIP_EXITS: i32 = 20022;
pub const ERROR_DHCP_RESERVED_CLIENT: i32 = 20019;
pub const ERROR_DHCP_ROGUE_DS_CONFLICT: i32 = 20041;
pub const ERROR_DHCP_ROGUE_DS_UNREACHABLE: i32 = 20040;
pub const ERROR_DHCP_ROGUE_INIT_FAILED: i32 = 20037;
pub const ERROR_DHCP_ROGUE_NOT_AUTHORIZED: i32 = 20039;
pub const ERROR_DHCP_ROGUE_NOT_OUR_ENTERPRISE: i32 = 20042;
pub const ERROR_DHCP_ROGUE_SAMSHUTDOWN: i32 = 20038;
pub const ERROR_DHCP_ROGUE_STANDALONE_IN_DS: i32 = 20043;
pub const ERROR_DHCP_RPC_INIT_FAILED: i32 = 20002;
pub const ERROR_DHCP_SCOPE_NAME_TOO_LONG: i32 = 20046;
pub const ERROR_DHCP_SERVER_NAME_NOT_RESOLVED: i32 = 20124;
pub const ERROR_DHCP_SERVER_NOT_REACHABLE: i32 = 20122;
pub const ERROR_DHCP_SERVER_NOT_RUNNING: i32 = 20123;
pub const ERROR_DHCP_SERVICE_PAUSED: i32 = 20017;
pub const ERROR_DHCP_SUBNET_EXISTS: i32 = 20052;
pub const ERROR_DHCP_SUBNET_EXITS: i32 = 20004;
pub const ERROR_DHCP_SUBNET_NOT_PRESENT: i32 = 20005;
pub const ERROR_DHCP_SUPER_SCOPE_NAME_TOO_LONG: i32 = 20030;
pub const ERROR_DHCP_UNDEFINED_HARDWARE_ADDRESS_TYPE: i32 = 20102;
pub const ERROR_DHCP_UNSUPPORTED_CLIENT: i32 = 20034;
pub const ERROR_EXTEND_TOO_SMALL: i32 = 20025;
pub const ERROR_LAST_DHCP_SERVER_ERROR: i32 = 20139;
pub const ERROR_MSCOPE_RANGE_TOO_SMALL: i32 = 20054;
pub const ERROR_SCOPE_RANGE_POLICY_RANGE_CONFLICT: i32 = 20112;
pub const ERROR_SERVER_INVALID_BOOT_FILE_TABLE: i32 = 20028;
pub const ERROR_SERVER_UNKNOWN_BOOT_FILE_NAME: i32 = 20029;
pub const EXEMPT: QuarantineStatus = 4;
pub const FILTER_STATUS_FULL_MATCH_IN_ALLOW_LIST: i32 = 2;
pub const FILTER_STATUS_FULL_MATCH_IN_DENY_LIST: i32 = 4;
pub const FILTER_STATUS_NONE: i32 = 1;
pub const FILTER_STATUS_WILDCARD_MATCH_IN_ALLOW_LIST: i32 = 8;
pub const FILTER_STATUS_WILDCARD_MATCH_IN_DENY_LIST: i32 = 16;
pub type FSM_STATE = i32;
pub const HWTYPE_ETHERNET_10MB: i32 = 1;
pub const HotStandby: DHCP_FAILOVER_MODE = 1;
pub const INIT: FSM_STATE = 1;
#[cfg(feature = "shared")]
pub type LPDATE_TIME = *mut DATE_TIME;
#[cfg(feature = "shared")]
pub type LPDHCPDS_SERVER = *mut DHCPDS_SERVER;
#[cfg(feature = "shared")]
pub type LPDHCPDS_SERVERS = *mut DHCPDS_SERVERS;
#[cfg(feature = "shared")]
pub type LPDHCPV4_FAILOVER_CLIENT_INFO = *mut DHCPV4_FAILOVER_CLIENT_INFO;
#[cfg(feature = "shared")]
pub type LPDHCPV4_FAILOVER_CLIENT_INFO_ARRAY = *mut DHCPV4_FAILOVER_CLIENT_INFO_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCPV4_FAILOVER_CLIENT_INFO_EX = *mut DHCPV4_FAILOVER_CLIENT_INFO_EX;
#[cfg(feature = "shared")]
pub type LPDHCPV6_BIND_ELEMENT = *mut DHCPV6_BIND_ELEMENT;
#[cfg(feature = "shared")]
pub type LPDHCPV6_BIND_ELEMENT_ARRAY = *mut DHCPV6_BIND_ELEMENT_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCPV6_IP_ARRAY = *mut DHCPV6_IP_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCPV6_STATELESS_PARAMS = *mut DHCPV6_STATELESS_PARAMS;
#[cfg(feature = "shared")]
pub type LPDHCPV6_STATELESS_SCOPE_STATS = *mut DHCPV6_STATELESS_SCOPE_STATS;
#[cfg(feature = "shared")]
pub type LPDHCPV6_STATELESS_STATS = *mut DHCPV6_STATELESS_STATS;
#[cfg(feature = "shared")]
pub type LPDHCP_ADDR_PATTERN = *mut DHCP_ADDR_PATTERN;
#[cfg(feature = "shared")]
pub type LPDHCP_ALL_OPTIONS = *mut DHCP_ALL_OPTIONS;
#[cfg(feature = "shared")]
pub type LPDHCP_ALL_OPTION_VALUES = *mut DHCP_ALL_OPTION_VALUES;
#[cfg(feature = "shared")]
pub type LPDHCP_ALL_OPTION_VALUES_PB = *mut DHCP_ALL_OPTION_VALUES_PB;
#[cfg(feature = "shared")]
pub type LPDHCP_ATTRIB = *mut DHCP_ATTRIB;
#[cfg(feature = "shared")]
pub type LPDHCP_ATTRIB_ARRAY = *mut DHCP_ATTRIB_ARRAY;
pub type LPDHCP_ATTRIB_ID = *mut u32;
#[cfg(feature = "shared")]
pub type LPDHCP_BINARY_DATA = *mut DHCP_BINARY_DATA;
#[cfg(feature = "shared")]
pub type LPDHCP_BIND_ELEMENT = *mut DHCP_BIND_ELEMENT;
#[cfg(feature = "shared")]
pub type LPDHCP_BIND_ELEMENT_ARRAY = *mut DHCP_BIND_ELEMENT_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCP_BOOT_IP_RANGE = *mut DHCP_BOOTP_IP_RANGE;
#[cfg(feature = "shared")]
pub type LPDHCP_CLASS_INFO = *mut DHCP_CLASS_INFO;
#[cfg(feature = "shared")]
pub type LPDHCP_CLASS_INFO_ARRAY = *mut DHCP_CLASS_INFO_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCP_CLASS_INFO_ARRAY_V6 = *mut DHCP_CLASS_INFO_ARRAY_V6;
#[cfg(feature = "shared")]
pub type LPDHCP_CLASS_INFO_V6 = *mut DHCP_CLASS_INFO_V6;
#[cfg(feature = "shared")]
pub type LPDHCP_CLIENT_FILTER_STATUS_INFO = *mut DHCP_CLIENT_FILTER_STATUS_INFO;
#[cfg(feature = "shared")]
pub type LPDHCP_CLIENT_FILTER_STATUS_INFO_ARRAY = *mut DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCP_CLIENT_INFO = *mut DHCP_CLIENT_INFO;
#[cfg(feature = "shared")]
pub type LPDHCP_CLIENT_INFO_ARRAY = *mut DHCP_CLIENT_INFO_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCP_CLIENT_INFO_ARRAY_V4 = *mut DHCP_CLIENT_INFO_ARRAY_V4;
#[cfg(feature = "shared")]
pub type LPDHCP_CLIENT_INFO_ARRAY_V5 = *mut DHCP_CLIENT_INFO_ARRAY_V5;
#[cfg(feature = "shared")]
pub type LPDHCP_CLIENT_INFO_ARRAY_V6 = *mut DHCP_CLIENT_INFO_ARRAY_V6;
#[cfg(feature = "shared")]
pub type LPDHCP_CLIENT_INFO_ARRAY_VQ = *mut DHCP_CLIENT_INFO_ARRAY_VQ;
#[cfg(feature = "shared")]
pub type LPDHCP_CLIENT_INFO_EX = *mut DHCP_CLIENT_INFO_EX;
#[cfg(feature = "shared")]
pub type LPDHCP_CLIENT_INFO_EX_ARRAY = *mut DHCP_CLIENT_INFO_EX_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCP_CLIENT_INFO_PB = *mut DHCP_CLIENT_INFO_PB;
#[cfg(feature = "shared")]
pub type LPDHCP_CLIENT_INFO_PB_ARRAY = *mut DHCP_CLIENT_INFO_PB_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCP_CLIENT_INFO_V4 = *mut DHCP_CLIENT_INFO_V4;
#[cfg(feature = "shared")]
pub type LPDHCP_CLIENT_INFO_V5 = *mut DHCP_CLIENT_INFO_V5;
#[cfg(feature = "shared")]
pub type LPDHCP_CLIENT_INFO_V6 = *mut DHCP_CLIENT_INFO_V6;
#[cfg(feature = "shared")]
pub type LPDHCP_CLIENT_INFO_VQ = *mut DHCP_CLIENT_INFO_VQ;
pub type LPDHCP_FAILOVER_MODE = *mut DHCP_FAILOVER_MODE;
#[cfg(feature = "shared")]
pub type LPDHCP_FAILOVER_RELATIONSHIP = *mut DHCP_FAILOVER_RELATIONSHIP;
#[cfg(feature = "shared")]
pub type LPDHCP_FAILOVER_RELATIONSHIP_ARRAY = *mut DHCP_FAILOVER_RELATIONSHIP_ARRAY;
pub type LPDHCP_FAILOVER_SERVER = *mut DHCP_FAILOVER_SERVER;
#[cfg(feature = "shared")]
pub type LPDHCP_FAILOVER_STATISTICS = *mut DHCP_FAILOVER_STATISTICS;
#[cfg(feature = "shared")]
pub type LPDHCP_FILTER_ADD_INFO = *mut DHCP_FILTER_ADD_INFO;
#[cfg(feature = "shared")]
pub type LPDHCP_FILTER_ENUM_INFO = *mut DHCP_FILTER_ENUM_INFO;
pub type LPDHCP_FILTER_GLOBAL_INFO = *mut DHCP_FILTER_GLOBAL_INFO;
pub type LPDHCP_FILTER_LIST_TYPE = *mut DHCP_FILTER_LIST_TYPE;
#[cfg(feature = "shared")]
pub type LPDHCP_FILTER_RECORD = *mut DHCP_FILTER_RECORD;
pub type LPDHCP_FORCE_FLAG = *mut DHCP_FORCE_FLAG;
pub type LPDHCP_HOST_INFO = *mut DHCP_HOST_INFO;
#[cfg(feature = "shared")]
pub type LPDHCP_HOST_INFO_V6 = *mut DHCP_HOST_INFO_V6;
#[cfg(feature = "shared")]
pub type LPDHCP_IPV6_ADDRESS = *mut DHCP_IPV6_ADDRESS;
pub type LPDHCP_IP_ADDRESS = *mut u32;
#[cfg(feature = "shared")]
pub type LPDHCP_IP_ARRAY = *mut DHCP_IP_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCP_IP_CLUSTER = *mut DHCP_IP_CLUSTER;
pub type LPDHCP_IP_RANGE = *mut DHCP_IP_RANGE;
#[cfg(feature = "shared")]
pub type LPDHCP_IP_RANGE_ARRAY = *mut DHCP_IP_RANGE_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCP_IP_RANGE_V6 = *mut DHCP_IP_RANGE_V6;
#[cfg(feature = "shared")]
pub type LPDHCP_IP_RESERVATION = *mut DHCP_IP_RESERVATION;
#[cfg(feature = "shared")]
pub type LPDHCP_IP_RESERVATION_INFO = *mut DHCP_IP_RESERVATION_INFO;
#[cfg(feature = "shared")]
pub type LPDHCP_IP_RESERVATION_V4 = *mut DHCP_IP_RESERVATION_V4;
#[cfg(feature = "shared")]
pub type LPDHCP_IP_RESERVATION_V6 = *mut DHCP_IP_RESERVATION_V6;
#[cfg(feature = "shared")]
pub type LPDHCP_MIB_INFO = *mut DHCP_MIB_INFO;
#[cfg(feature = "shared")]
pub type LPDHCP_MIB_INFO_V5 = *mut DHCP_MIB_INFO_V5;
#[cfg(feature = "shared")]
pub type LPDHCP_MIB_INFO_V6 = *mut DHCP_MIB_INFO_V6;
#[cfg(feature = "shared")]
pub type LPDHCP_MIB_INFO_VQ = *mut DHCP_MIB_INFO_VQ;
#[cfg(feature = "shared")]
pub type LPDHCP_OPTION = *mut DHCP_OPTION;
#[cfg(feature = "shared")]
pub type LPDHCP_OPTION_ARRAY = *mut DHCP_OPTION_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCP_OPTION_DATA = *mut DHCP_OPTION_DATA;
#[cfg(feature = "shared")]
pub type LPDHCP_OPTION_DATA_ELEMENT = *mut DHCP_OPTION_DATA_ELEMENT;
pub type LPDHCP_OPTION_DATA_TYPE = *mut DHCP_OPTION_DATA_TYPE;
#[cfg(feature = "shared")]
pub type LPDHCP_OPTION_LIST = *mut DHCP_OPTION_LIST;
#[cfg(feature = "winnt")]
pub type LPDHCP_OPTION_SCOPE_INFO = *mut DHCP_OPTION_SCOPE_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPDHCP_OPTION_SCOPE_INFO6 = *mut DHCP_OPTION_SCOPE_INFO6;
pub type LPDHCP_OPTION_SCOPE_TYPE = *mut DHCP_OPTION_SCOPE_TYPE;
pub type LPDHCP_OPTION_SCOPE_TYPE6 = *mut DHCP_OPTION_SCOPE_TYPE6;
pub type LPDHCP_OPTION_TYPE = *mut DHCP_OPTION_TYPE;
#[cfg(feature = "shared")]
pub type LPDHCP_OPTION_VALUE = *mut DHCP_OPTION_VALUE;
#[cfg(feature = "shared")]
pub type LPDHCP_OPTION_VALUE_ARRAY = *mut DHCP_OPTION_VALUE_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCP_PERF_STATS = *mut DHCP_PERF_STATS;
#[cfg(feature = "shared")]
pub type LPDHCP_POLICY = *mut DHCP_POLICY;
#[cfg(feature = "shared")]
pub type LPDHCP_POLICY_ARRAY = *mut DHCP_POLICY_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCP_POLICY_EX = *mut DHCP_POLICY_EX;
#[cfg(feature = "shared")]
pub type LPDHCP_POLICY_EX_ARRAY = *mut DHCP_POLICY_EX_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCP_POL_COND = *mut DHCP_POL_COND;
#[cfg(feature = "shared")]
pub type LPDHCP_POL_COND_ARRAY = *mut DHCP_POL_COND_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCP_POL_EXPR = *mut DHCP_POL_EXPR;
#[cfg(feature = "shared")]
pub type LPDHCP_POL_EXPR_ARRAY = *mut DHCP_POL_EXPR_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCP_PROPERTY = *mut DHCP_PROPERTY;
#[cfg(feature = "shared")]
pub type LPDHCP_PROPERTY_ARRAY = *mut DHCP_PROPERTY_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCP_RESERVATION_INFO_ARRAY = *mut DHCP_RESERVATION_INFO_ARRAY;
pub type LPDHCP_RESERVED_SCOPE = *mut DHCP_RESERVED_SCOPE;
#[cfg(feature = "shared")]
pub type LPDHCP_RESERVED_SCOPE6 = *mut DHCP_RESERVED_SCOPE6;
pub type LPDHCP_SCAN_FLAG = *mut DHCP_SCAN_FLAG;
pub type LPDHCP_SCAN_ITEM = *mut DHCP_SCAN_ITEM;
#[cfg(feature = "shared")]
pub type LPDHCP_SCAN_LIST = *mut DHCP_SCAN_LIST;
#[cfg(feature = "shared")]
pub type LPDHCP_SEARCH_INFO = *mut DHCP_SEARCH_INFO;
pub type LPDHCP_SEARCH_INFO_TYPE = *mut DHCP_SEARCH_INFO_TYPE;
pub type LPDHCP_SEARCH_INFO_TYPE_V6 = *mut DHCP_SEARCH_INFO_TYPE_V6;
#[cfg(feature = "shared")]
pub type LPDHCP_SEARCH_INFO_V6 = *mut DHCP_SEARCH_INFO_V6;
#[cfg(feature = "shared")]
pub type LPDHCP_SERVER_CONFIG_INFO = *mut DHCP_SERVER_CONFIG_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPDHCP_SERVER_CONFIG_INFO_V4 = *mut DHCP_SERVER_CONFIG_INFO_V4;
#[cfg(feature = "shared")]
pub type LPDHCP_SERVER_CONFIG_INFO_V6 = *mut DHCP_SERVER_CONFIG_INFO_V6;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPDHCP_SERVER_CONFIG_INFO_VQ = *mut DHCP_SERVER_CONFIG_INFO_VQ;
#[cfg(feature = "shared")]
pub type LPDHCP_SERVER_INFO = *mut DHCPDS_SERVER;
#[cfg(feature = "shared")]
pub type LPDHCP_SERVER_INFO_ARRAY = *mut DHCPDS_SERVERS;
pub type LPDHCP_SERVER_SPECIFIC_STRINGS = *mut DHCP_SERVER_SPECIFIC_STRINGS;
#[cfg(feature = "shared")]
pub type LPDHCP_SUBNET_ELEMENT_DATA = *mut DHCP_SUBNET_ELEMENT_DATA;
#[cfg(feature = "shared")]
pub type LPDHCP_SUBNET_ELEMENT_DATA_V4 = *mut DHCP_SUBNET_ELEMENT_DATA_V4;
#[cfg(feature = "shared")]
pub type LPDHCP_SUBNET_ELEMENT_DATA_V5 = *mut DHCP_SUBNET_ELEMENT_DATA_V5;
#[cfg(feature = "shared")]
pub type LPDHCP_SUBNET_ELEMENT_DATA_V6 = *mut DHCP_SUBNET_ELEMENT_DATA_V6;
#[cfg(feature = "shared")]
pub type LPDHCP_SUBNET_ELEMENT_INFO_ARRAY = *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY;
#[cfg(feature = "shared")]
pub type LPDHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 = *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4;
#[cfg(feature = "shared")]
pub type LPDHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 = *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5;
#[cfg(feature = "shared")]
pub type LPDHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 = *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6;
pub type LPDHCP_SUBNET_ELEMENT_TYPE = *mut DHCP_SUBNET_ELEMENT_TYPE;
pub type LPDHCP_SUBNET_ELEMENT_TYPE_V6 = *mut DHCP_SUBNET_ELEMENT_TYPE_V6;
pub type LPDHCP_SUBNET_INFO = *mut DHCP_SUBNET_INFO;
#[cfg(feature = "shared")]
pub type LPDHCP_SUBNET_INFO_V6 = *mut DHCP_SUBNET_INFO_V6;
#[cfg(feature = "shared")]
pub type LPDHCP_SUBNET_INFO_VQ = *mut DHCP_SUBNET_INFO_VQ;
pub type LPDHCP_SUBNET_STATE = *mut DHCP_SUBNET_STATE;
#[cfg(feature = "shared")]
pub type LPDHCP_SUPER_SCOPE_TABLE = *mut DHCP_SUPER_SCOPE_TABLE;
#[cfg(feature = "shared")]
pub type LPDHCP_SUPER_SCOPE_TABLE_ENTRY = *mut DHCP_SUPER_SCOPE_TABLE_ENTRY;
#[cfg(feature = "shared")]
pub type LPDWORD_DWORD = *mut DWORD_DWORD;
#[cfg(feature = "shared")]
pub type LPSCOPE_MIB_INFO = *mut SCOPE_MIB_INFO;
#[cfg(feature = "shared")]
pub type LPSCOPE_MIB_INFO_V5 = *mut SCOPE_MIB_INFO_V5;
#[cfg(feature = "shared")]
pub type LPSCOPE_MIB_INFO_V6 = *mut SCOPE_MIB_INFO_V6;
#[cfg(feature = "shared")]
pub type LPSCOPE_MIB_INFO_VQ = *mut SCOPE_MIB_INFO_VQ;
pub const LoadBalance: DHCP_FAILOVER_MODE = 0;
pub const MAC_ADDRESS_LENGTH: i32 = 6;
pub const MAX_PATTERN_LENGTH: i32 = 255;
pub const MCLT: i32 = 1;
pub const NOQUARANTINE: QuarantineStatus = 0;
pub const NOQUARINFO: QuarantineStatus = 6;
pub const NORMAL: FSM_STATE = 3;
pub const NO_STATE: FSM_STATE = 0;
pub const PARTNER_DOWN: FSM_STATE = 5;
pub const PAUSED: FSM_STATE = 12;
#[cfg(feature = "shared")]
pub type PDATE_TIME = *mut DATE_TIME;
#[cfg(feature = "shared")]
pub type PDHCPDS_SERVER = *mut DHCPDS_SERVER;
#[cfg(feature = "shared")]
pub type PDHCPDS_SERVERS = *mut DHCPDS_SERVERS;
#[cfg(feature = "shared")]
pub type PDHCPV6_STATELESS_PARAMS = *mut DHCPV6_STATELESS_PARAMS;
#[cfg(feature = "shared")]
pub type PDHCPV6_STATELESS_SCOPE_STATS = *mut DHCPV6_STATELESS_SCOPE_STATS;
#[cfg(feature = "shared")]
pub type PDHCPV6_STATELESS_STATS = *mut DHCPV6_STATELESS_STATS;
#[cfg(feature = "shared")]
pub type PDHCP_ATTRIB = *mut DHCP_ATTRIB;
#[cfg(feature = "shared")]
pub type PDHCP_ATTRIB_ARRAY = *mut DHCP_ATTRIB_ARRAY;
pub type PDHCP_ATTRIB_ID = *mut u32;
#[cfg(feature = "shared")]
pub type PDHCP_IPV6_ADDRESS = *mut DHCP_IPV6_ADDRESS;
pub type PDHCP_IP_ADDRESS = *mut u32;
#[cfg(feature = "shared")]
pub type PDHCP_IP_RANGE_ARRAY = *mut DHCP_IP_RANGE_ARRAY;
#[cfg(feature = "shared")]
pub type PDHCP_POLICY = *mut DHCP_POLICY;
#[cfg(feature = "shared")]
pub type PDHCP_POLICY_ARRAY = *mut DHCP_POLICY_ARRAY;
#[cfg(feature = "shared")]
pub type PDHCP_POLICY_EX = *mut DHCP_POLICY_EX;
#[cfg(feature = "shared")]
pub type PDHCP_POLICY_EX_ARRAY = *mut DHCP_POLICY_EX_ARRAY;
#[cfg(feature = "shared")]
pub type PDHCP_POL_COND = *mut DHCP_POL_COND;
#[cfg(feature = "shared")]
pub type PDHCP_POL_COND_ARRAY = *mut DHCP_POL_COND_ARRAY;
#[cfg(feature = "shared")]
pub type PDHCP_POL_EXPR = *mut DHCP_POL_EXPR;
#[cfg(feature = "shared")]
pub type PDHCP_POL_EXPR_ARRAY = *mut DHCP_POL_EXPR_ARRAY;
#[cfg(feature = "shared")]
pub type PDHCP_PROPERTY = *mut DHCP_PROPERTY;
#[cfg(feature = "shared")]
pub type PDHCP_PROPERTY_ARRAY = *mut DHCP_PROPERTY_ARRAY;
#[cfg(feature = "shared")]
pub type PDHCP_SERVER_INFO = *mut DHCPDS_SERVER;
#[cfg(feature = "shared")]
pub type PDHCP_SERVER_INFO_ARRAY = *mut DHCPDS_SERVERS;
#[cfg(feature = "shared")]
pub type PDHCP_SUBNET_INFO_V6 = *mut DHCP_SUBNET_INFO_V6;
pub const PERCENTAGE: i32 = 8;
pub const POTENTIAL_CONFLICT: FSM_STATE = 6;
pub const PREVSTATE: i32 = 32;
pub const PROBATION: QuarantineStatus = 3;
pub const PrimaryServer: DHCP_FAILOVER_SERVER = 0;
pub const QUARANTINE_CONFIG_OPTION: i32 = 43222;
pub const QUARANTINE_SCOPE_QUARPROFILE_OPTION: i32 = 43221;
pub const QUARANTIN_OPTION_BASE: i32 = 43220;
pub type QuarantineStatus = i32;
pub const RECOVER: FSM_STATE = 9;
pub const RECOVER_DONE: FSM_STATE = 11;
pub const RECOVER_WAIT: FSM_STATE = 10;
pub const RESOLUTION_INT: FSM_STATE = 8;
pub const RESTRICTEDACCESS: QuarantineStatus = 1;
pub const SAFEPERIOD: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SCOPE_MIB_INFO {
    pub Subnet: DHCP_IP_ADDRESS,
    pub NumAddressesInuse: super::DWORD,
    pub NumAddressesFree: super::DWORD,
    pub NumPendingOffers: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SCOPE_MIB_INFO_V5 {
    pub Subnet: DHCP_IP_ADDRESS,
    pub NumAddressesInuse: super::DWORD,
    pub NumAddressesFree: super::DWORD,
    pub NumPendingOffers: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SCOPE_MIB_INFO_V6 {
    pub Subnet: DHCP_IPV6_ADDRESS,
    pub NumAddressesInuse: super::ULONGLONG,
    pub NumAddressesFree: super::ULONGLONG,
    pub NumPendingAdvertises: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SCOPE_MIB_INFO_VQ {
    pub Subnet: DHCP_IP_ADDRESS,
    pub NumAddressesInuse: super::DWORD,
    pub NumAddressesFree: super::DWORD,
    pub NumPendingOffers: super::DWORD,
    pub QtnNumLeases: super::DWORD,
    pub QtnPctQtnLeases: super::DWORD,
    pub QtnProbationLeases: super::DWORD,
    pub QtnNonQtnLeases: super::DWORD,
    pub QtnExemptLeases: super::DWORD,
    pub QtnCapableClients: super::DWORD,
}
pub const SCOPE_STATE_DISABLED: DHCP_SUBNET_STATE = 1;
pub const SCOPE_STATE_ENABLED: DHCP_SUBNET_STATE = 0;
pub const SHAREDSECRET: i32 = 64;
pub const SHUTDOWN: FSM_STATE = 13;
pub const STARTUP: FSM_STATE = 2;
pub const SecondaryServer: DHCP_FAILOVER_SERVER = 1;
pub const Set_APIProtocolSupport: i32 = 1;
pub const Set_AuditLogState: i32 = 2048;
pub const Set_BackupInterval: i32 = 16;
pub const Set_BackupPath: i32 = 8;
pub const Set_BootFileTable: i32 = 1024;
pub const Set_DatabaseCleanupInterval: i32 = 128;
pub const Set_DatabaseLoggingFlag: i32 = 32;
pub const Set_DatabaseName: i32 = 2;
pub const Set_DatabasePath: i32 = 4;
pub const Set_DebugFlag: i32 = 256;
pub const Set_PingRetries: i32 = 512;
pub const Set_PreferredLifetime: i32 = 4;
pub const Set_PreferredLifetimeIATA: i32 = 64;
pub const Set_QuarantineDefFail: i32 = 8192;
pub const Set_QuarantineON: i32 = 4096;
pub const Set_RapidCommitFlag: i32 = 2;
pub const Set_RestoreFlag: i32 = 64;
pub const Set_T1: i32 = 16;
pub const Set_T2: i32 = 32;
pub const Set_UnicastFlag: i32 = 1;
pub const Set_ValidLifetime: i32 = 8;
pub const Set_ValidLifetimeIATA: i32 = 128;
pub const V5_ADDRESS_BIT_BOTH_REC: i32 = 32;
pub const V5_ADDRESS_BIT_DELETED: i32 = 128;
pub const V5_ADDRESS_BIT_UNREGISTERED: i32 = 64;
pub const V5_ADDRESS_EX_BIT_DISABLE_PTR_RR: i32 = 1;
pub const V5_ADDRESS_STATE_ACTIVE: i32 = 1;
pub const V5_ADDRESS_STATE_DECLINED: i32 = 2;
pub const V5_ADDRESS_STATE_DOOM: i32 = 3;
pub const V5_ADDRESS_STATE_OFFERED: i32 = 0;
pub const WARNING_EXTENDED_LESS: i32 = 20026;
