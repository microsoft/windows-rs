#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddFilterV4(serveripaddress: super::super::Foundation::PWSTR, addfilterinfo: *const DHCP_FILTER_ADD_INFO, forceflag: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddSecurityGroup(pserver: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddServer(flags: u32, idinfo: *mut ::core::ffi::c_void, newserver: *mut DHCPDS_SERVER, callbackfn: *mut ::core::ffi::c_void, callbackdata: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddSubnetElement(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddSubnetElementV4(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V4) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddSubnetElementV5(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V5) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddSubnetElementV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, addelementinfo: *mut DHCP_SUBNET_ELEMENT_DATA_V6) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAuditLogGetParams(serveripaddress: super::super::Foundation::PWSTR, flags: u32, auditlogdir: *mut super::super::Foundation::PWSTR, diskcheckinterval: *mut u32, maxlogfilessize: *mut u32, minspaceondisk: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAuditLogSetParams(serveripaddress: super::super::Foundation::PWSTR, flags: u32, auditlogdir: super::super::Foundation::PWSTR, diskcheckinterval: u32, maxlogfilessize: u32, minspaceondisk: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpCApiCleanup();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpCApiInitialize(version: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateClass(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateClassV6(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO_V6) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateClientInfo(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateClientInfoV4(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_V4) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateClientInfoVQ(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_VQ) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateOption(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, optioninfo: *const DHCP_OPTION) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateOptionV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut DHCP_OPTION) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateOptionV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut DHCP_OPTION) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateSubnet(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateSubnetV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut DHCP_SUBNET_INFO_V6) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateSubnetVQ(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO_VQ) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpDeRegisterParamChange(flags: u32, reserved: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteClass(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteClassV6(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteClientInfo(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_SEARCH_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteClientInfoV6(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_SEARCH_INFO_V6) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteFilterV4(serveripaddress: super::super::Foundation::PWSTR, deletefilterinfo: *const DHCP_ADDR_PATTERN) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteServer(flags: u32, idinfo: *mut ::core::ffi::c_void, newserver: *mut DHCPDS_SERVER, callbackfn: *mut ::core::ffi::c_void, callbackdata: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteSubnet(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, forceflag: DHCP_FORCE_FLAG) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteSubnetV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, forceflag: DHCP_FORCE_FLAG) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteSuperScopeV4(serveripaddress: super::super::Foundation::PWSTR, superscopename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpDsCleanup();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpDsInit() -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumClasses(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, resumehandle: *mut u32, preferredmaximum: u32, classinfoarray: *mut *mut DHCP_CLASS_INFO_ARRAY, nread: *mut u32, ntotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumClassesV6(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, resumehandle: *mut u32, preferredmaximum: u32, classinfoarray: *mut *mut DHCP_CLASS_INFO_ARRAY_V6, nread: *mut u32, ntotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumFilterV4(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut DHCP_ADDR_PATTERN, preferredmaximum: u32, listtype: DHCP_FILTER_LIST_TYPE, enumfilterinfo: *mut *mut DHCP_FILTER_ENUM_INFO, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptionValues(serveripaddress: super::super::Foundation::PWSTR, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptionValuesV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptionValuesV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptions(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptionsV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptionsV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumServers(flags: u32, idinfo: *mut ::core::ffi::c_void, servers: *mut *mut DHCPDS_SERVERS, callbackfn: *mut ::core::ffi::c_void, callbackdata: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClients(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClientsFilterStatusInfo(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClientsV4(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V4, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClientsV5(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V5, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClientsV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, resumehandle: *mut DHCP_IPV6_ADDRESS, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V6, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClientsVQ(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_VQ, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetElements(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetElementsV4(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetElementsV5(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetElementsV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE_V6, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnets(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, enuminfo: *mut *mut DHCP_IP_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetsV6(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, enuminfo: *mut *mut DHCPV6_IP_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetAllOptionValues(serveripaddress: super::super::Foundation::PWSTR, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, values: *mut *mut DHCP_ALL_OPTION_VALUES) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetAllOptionValuesV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, values: *mut *mut DHCP_ALL_OPTION_VALUES) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetAllOptions(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionstruct: *mut *mut DHCP_ALL_OPTIONS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetAllOptionsV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionstruct: *mut *mut DHCP_ALL_OPTIONS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClassInfo(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, partialclassinfo: *mut DHCP_CLASS_INFO, filledclassinfo: *mut *mut DHCP_CLASS_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClientInfo(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClientInfoV4(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_V4) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClientInfoV6(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO_V6, clientinfo: *mut *mut DHCP_CLIENT_INFO_V6) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClientInfoVQ(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_VQ) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClientOptions(serveripaddress: super::super::Foundation::PWSTR, clientipaddress: u32, clientsubnetmask: u32, clientoptions: *mut *mut DHCP_OPTION_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetFilterV4(serveripaddress: super::super::Foundation::PWSTR, globalfilterinfo: *mut DHCP_FILTER_GLOBAL_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetMibInfo(serveripaddress: super::super::Foundation::PWSTR, mibinfo: *mut *mut DHCP_MIB_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetMibInfoV5(serveripaddress: super::super::Foundation::PWSTR, mibinfo: *mut *mut DHCP_MIB_INFO_V5) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetMibInfoV6(serveripaddress: super::super::Foundation::PWSTR, mibinfo: *mut *mut DHCP_MIB_INFO_V6) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionInfo(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, optioninfo: *mut *mut DHCP_OPTION) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionInfoV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut *mut DHCP_OPTION) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionInfoV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut *mut DHCP_OPTION) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionValue(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionValueV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionValueV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOriginalSubnetMask(sadaptername: super::super::Foundation::PWSTR, dwsubnetmask: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetServerBindingInfo(serveripaddress: super::super::Foundation::PWSTR, flags: u32, bindelementsinfo: *mut *mut DHCP_BIND_ELEMENT_ARRAY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetServerBindingInfoV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, bindelementsinfo: *mut *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetServerSpecificStrings(serveripaddress: super::super::Foundation::PWSTR, serverspecificstrings: *mut *mut DHCP_SERVER_SPECIFIC_STRINGS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetSubnetDelayOffer(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, timedelayinmilliseconds: *mut u16) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetSubnetInfo(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *mut *mut DHCP_SUBNET_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetSubnetInfoV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut *mut DHCP_SUBNET_INFO_V6) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetSubnetInfoVQ(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *mut *mut DHCP_SUBNET_INFO_VQ) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetSuperScopeInfoV4(serveripaddress: super::super::Foundation::PWSTR, superscopetable: *mut *mut DHCP_SUPER_SCOPE_TABLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpGetThreadOptions(pflags: *mut u32, reserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetVersion(serveripaddress: super::super::Foundation::PWSTR, majorversion: *mut u32, minorversion: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprAddV4PolicyCondition(policy: *mut DHCP_POLICY, parentexpr: u32, r#type: DHCP_POL_ATTR_TYPE, optionid: u32, suboptionid: u32, vendorname: super::super::Foundation::PWSTR, operator: DHCP_POL_COMPARATOR, value: *const u8, valuelength: u32, conditionindex: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprAddV4PolicyExpr(policy: *mut DHCP_POLICY, parentexpr: u32, operator: DHCP_POL_LOGIC_OPER, exprindex: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprAddV4PolicyRange(policy: *mut DHCP_POLICY, range: *const DHCP_IP_RANGE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprCreateV4Policy(policyname: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnet: u32, processingorder: u32, rootoperator: DHCP_POL_LOGIC_OPER, description: super::super::Foundation::PWSTR, enabled: super::super::Foundation::BOOL, policy: *mut *mut DHCP_POLICY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprCreateV4PolicyEx(policyname: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnet: u32, processingorder: u32, rootoperator: DHCP_POL_LOGIC_OPER, description: super::super::Foundation::PWSTR, enabled: super::super::Foundation::BOOL, policy: *mut *mut DHCP_POLICY_EX) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFindV4DhcpProperty(propertyarray: *const DHCP_PROPERTY_ARRAY, id: DHCP_PROPERTY_ID, r#type: DHCP_PROPERTY_TYPE) -> *mut DHCP_PROPERTY;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4DhcpProperty(property: *mut DHCP_PROPERTY);
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4DhcpPropertyArray(propertyarray: *mut DHCP_PROPERTY_ARRAY);
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4Policy(policy: *mut DHCP_POLICY);
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4PolicyArray(policyarray: *mut DHCP_POLICY_ARRAY);
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4PolicyEx(policyex: *mut DHCP_POLICY_EX);
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4PolicyExArray(policyexarray: *mut DHCP_POLICY_EX_ARRAY);
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprIsV4PolicySingleUC(policy: *const DHCP_POLICY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprIsV4PolicyValid(ppolicy: *const DHCP_POLICY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprIsV4PolicyWellFormed(ppolicy: *const DHCP_POLICY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprModifyV4PolicyExpr(policy: *mut DHCP_POLICY, operator: DHCP_POL_LOGIC_OPER) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprResetV4PolicyExpr(policy: *mut DHCP_POLICY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpModifyClass(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpModifyClassV6(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO_V6) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRegisterParamChange(flags: u32, reserved: *mut ::core::ffi::c_void, adaptername: super::super::Foundation::PWSTR, classid: *mut DHCPCAPI_CLASSID, params: DHCPCAPI_PARAMS_ARRAY, handle: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpRemoveDNSRegistrations() -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOption(serveripaddress: super::super::Foundation::PWSTR, optionid: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOptionV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOptionV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOptionValue(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOptionValueV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOptionValueV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveSubnetElement(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA, forceflag: DHCP_FORCE_FLAG) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveSubnetElementV4(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V4, forceflag: DHCP_FORCE_FLAG) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveSubnetElementV5(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V5, forceflag: DHCP_FORCE_FLAG) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveSubnetElementV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, removeelementinfo: *mut DHCP_SUBNET_ELEMENT_DATA_V6, forceflag: DHCP_FORCE_FLAG) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRequestParams(flags: u32, reserved: *mut ::core::ffi::c_void, adaptername: super::super::Foundation::PWSTR, classid: *mut DHCPCAPI_CLASSID, sendparams: DHCPCAPI_PARAMS_ARRAY, recdparams: DHCPCAPI_PARAMS_ARRAY, buffer: *mut u8, psize: *mut u32, requestidstr: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpRpcFreeMemory(bufferpointer: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpScanDatabase(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, fixflag: u32, scanlist: *mut *mut DHCP_SCAN_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerAuditlogParamsFree(configinfo: *mut DHCP_SERVER_CONFIG_INFO_VQ);
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerBackupDatabase(serveripaddress: super::super::Foundation::PWSTR, path: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerGetConfig(serveripaddress: super::super::Foundation::PWSTR, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerGetConfigV4(serveripaddress: super::super::Foundation::PWSTR, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerGetConfigV6(serveripaddress: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerGetConfigVQ(serveripaddress: super::super::Foundation::PWSTR, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerQueryAttribute(serveripaddr: super::super::Foundation::PWSTR, dwreserved: u32, dhcpattribid: u32, pdhcpattrib: *mut *mut DHCP_ATTRIB) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerQueryAttributes(serveripaddr: super::super::Foundation::PWSTR, dwreserved: u32, dwattribcount: u32, pdhcpattribs: *mut u32, pdhcpattribarr: *mut *mut DHCP_ATTRIB_ARRAY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerQueryDnsRegCredentials(serveripaddress: super::super::Foundation::PWSTR, unamesize: u32, uname: super::super::Foundation::PWSTR, domainsize: u32, domain: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerRedoAuthorization(serveripaddr: super::super::Foundation::PWSTR, dwreserved: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerRestoreDatabase(serveripaddress: super::super::Foundation::PWSTR, path: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetConfig(serveripaddress: super::super::Foundation::PWSTR, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetConfigV4(serveripaddress: super::super::Foundation::PWSTR, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetConfigV6(serveripaddress: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetConfigVQ(serveripaddress: super::super::Foundation::PWSTR, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetDnsRegCredentials(serveripaddress: super::super::Foundation::PWSTR, uname: super::super::Foundation::PWSTR, domain: super::super::Foundation::PWSTR, passwd: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetDnsRegCredentialsV5(serveripaddress: super::super::Foundation::PWSTR, uname: super::super::Foundation::PWSTR, domain: super::super::Foundation::PWSTR, passwd: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetClientInfo(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetClientInfoV4(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_V4) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetClientInfoV6(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_V6) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetClientInfoVQ(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_VQ) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetFilterV4(serveripaddress: super::super::Foundation::PWSTR, globalfilterinfo: *const DHCP_FILTER_GLOBAL_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionInfo(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, optioninfo: *const DHCP_OPTION) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionInfoV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut DHCP_OPTION) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionInfoV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut DHCP_OPTION) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionValue(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalue: *const DHCP_OPTION_DATA) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionValueV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut DHCP_OPTION_DATA) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionValueV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, optionvalue: *mut DHCP_OPTION_DATA) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionValues(serveripaddress: super::super::Foundation::PWSTR, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalues: *const DHCP_OPTION_VALUE_ARRAY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionValuesV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalues: *mut DHCP_OPTION_VALUE_ARRAY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetServerBindingInfo(serveripaddress: super::super::Foundation::PWSTR, flags: u32, bindelementinfo: *mut DHCP_BIND_ELEMENT_ARRAY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetServerBindingInfoV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, bindelementinfo: *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetSubnetDelayOffer(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, timedelayinmilliseconds: u16) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetSubnetInfo(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetSubnetInfoV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut DHCP_SUBNET_INFO_V6) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetSubnetInfoVQ(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO_VQ) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetSuperScopeV4(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, superscopename: super::super::Foundation::PWSTR, changeexisting: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpSetThreadOptions(flags: u32, reserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpUndoRequestParams(flags: u32, reserved: *mut ::core::ffi::c_void, adaptername: super::super::Foundation::PWSTR, requestidstr: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4AddPolicyRange(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, range: *const DHCP_IP_RANGE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4CreateClientInfo(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_PB) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4CreateClientInfoEx(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_EX) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4CreatePolicy(serveripaddress: super::super::Foundation::PWSTR, ppolicy: *const DHCP_POLICY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4CreatePolicyEx(serveripaddress: super::super::Foundation::PWSTR, policyex: *const DHCP_POLICY_EX) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4DeletePolicy(serveripaddress: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, policyname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4EnumPolicies(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, enuminfo: *mut *mut DHCP_POLICY_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4EnumPoliciesEx(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, globalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, enuminfo: *mut *mut DHCP_POLICY_EX_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4EnumSubnetClients(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_PB_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4EnumSubnetClientsEx(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_EX_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4EnumSubnetReservations(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_RESERVATION_INFO_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverAddScopeToRelationship(serveripaddress: super::super::Foundation::PWSTR, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverCreateRelationship(serveripaddress: super::super::Foundation::PWSTR, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverDeleteRelationship(serveripaddress: super::super::Foundation::PWSTR, prelationshipname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverDeleteScopeFromRelationship(serveripaddress: super::super::Foundation::PWSTR, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverEnumRelationship(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP_ARRAY, relationshipread: *mut u32, relationshiptotal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetAddressStatus(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, pstatus: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetClientInfo(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCPV4_FAILOVER_CLIENT_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetRelationship(serveripaddress: super::super::Foundation::PWSTR, prelationshipname: super::super::Foundation::PWSTR, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetScopeRelationship(serveripaddress: super::super::Foundation::PWSTR, scopeid: u32, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetScopeStatistics(serveripaddress: super::super::Foundation::PWSTR, scopeid: u32, pstats: *mut *mut DHCP_FAILOVER_STATISTICS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetSystemTime(serveripaddress: super::super::Foundation::PWSTR, ptime: *mut u32, pmaxalloweddeltatime: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverSetRelationship(serveripaddress: super::super::Foundation::PWSTR, flags: u32, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverTriggerAddrAllocation(serveripaddress: super::super::Foundation::PWSTR, pfailrelname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetAllOptionValues(serveripaddress: super::super::Foundation::PWSTR, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, values: *mut *mut DHCP_ALL_OPTION_VALUES_PB) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetClientInfo(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_PB) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetClientInfoEx(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_EX) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetFreeIPAddress(serveripaddress: super::super::Foundation::PWSTR, scopeid: u32, startip: u32, endip: u32, numfreeaddrreq: u32, ipaddrlist: *mut *mut DHCP_IP_ARRAY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetOptionValue(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, policyname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetPolicy(serveripaddress: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, policy: *mut *mut DHCP_POLICY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetPolicyEx(serveripaddress: super::super::Foundation::PWSTR, globalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, policy: *mut *mut DHCP_POLICY_EX) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4QueryPolicyEnforcement(serveripaddress: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, enabled: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4RemoveOptionValue(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, policyname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4RemovePolicyRange(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, range: *const DHCP_IP_RANGE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4SetOptionValue(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, policyname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut DHCP_OPTION_DATA) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4SetOptionValues(serveripaddress: super::super::Foundation::PWSTR, flags: u32, policyname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalues: *mut DHCP_OPTION_VALUE_ARRAY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4SetPolicy(serveripaddress: super::super::Foundation::PWSTR, fieldsmodified: u32, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, policy: *const DHCP_POLICY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4SetPolicyEnforcement(serveripaddress: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, enable: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4SetPolicyEx(serveripaddress: super::super::Foundation::PWSTR, fieldsmodified: u32, globalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, policy: *const DHCP_POLICY_EX) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV6CreateClientInfo(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_V6) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV6GetFreeIPAddress(serveripaddress: super::super::Foundation::PWSTR, scopeid: DHCP_IPV6_ADDRESS, startip: DHCP_IPV6_ADDRESS, endip: DHCP_IPV6_ADDRESS, numfreeaddrreq: u32, ipaddrlist: *mut *mut DHCPV6_IP_ARRAY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV6GetStatelessStatistics(serveripaddress: super::super::Foundation::PWSTR, statelessstats: *mut *mut DHCPV6_STATELESS_STATS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV6GetStatelessStoreParams(serveripaddress: super::super::Foundation::PWSTR, fserverlevel: super::super::Foundation::BOOL, subnetaddress: DHCP_IPV6_ADDRESS, params: *mut *mut DHCPV6_STATELESS_PARAMS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV6SetStatelessStoreParams(serveripaddress: super::super::Foundation::PWSTR, fserverlevel: super::super::Foundation::BOOL, subnetaddress: DHCP_IPV6_ADDRESS, fieldmodified: u32, params: *const DHCPV6_STATELESS_PARAMS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn Dhcpv6CApiCleanup();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn Dhcpv6CApiInitialize(version: *mut u32);
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Dhcpv6ReleasePrefix(adaptername: super::super::Foundation::PWSTR, classid: *mut DHCPV6CAPI_CLASSID, leaseinfo: *mut DHCPV6PrefixLeaseInformation) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Dhcpv6RenewPrefix(adaptername: super::super::Foundation::PWSTR, pclassid: *mut DHCPV6CAPI_CLASSID, prefixleaseinfo: *mut DHCPV6PrefixLeaseInformation, pdwtimetowait: *mut u32, bvalidateprefix: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Dhcpv6RequestParams(forcenewinform: super::super::Foundation::BOOL, reserved: *mut ::core::ffi::c_void, adaptername: super::super::Foundation::PWSTR, classid: *mut DHCPV6CAPI_CLASSID, recdparams: DHCPV6CAPI_PARAMS_ARRAY, buffer: *mut u8, psize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Dhcpv6RequestPrefix(adaptername: super::super::Foundation::PWSTR, pclassid: *mut DHCPV6CAPI_CLASSID, prefixleaseinfo: *mut DHCPV6PrefixLeaseInformation, pdwtimetowait: *mut u32) -> u32;
}
