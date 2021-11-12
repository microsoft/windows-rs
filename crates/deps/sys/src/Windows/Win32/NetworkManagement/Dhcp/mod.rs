#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddFilterV4(serveripaddress: super::super::Foundation::PWSTR, addfilterinfo: *const DHCP_FILTER_ADD_INFO, forceflag: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddSecurityGroup(pserver: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddServer(flags: u32, idinfo: *mut ::core::ffi::c_void, newserver: *mut DHCPDS_SERVER, callbackfn: *mut ::core::ffi::c_void, callbackdata: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddSubnetElement(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddSubnetElementV4(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V4) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddSubnetElementV5(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V5) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddSubnetElementV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, addelementinfo: *mut DHCP_SUBNET_ELEMENT_DATA_V6) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAuditLogGetParams(serveripaddress: super::super::Foundation::PWSTR, flags: u32, auditlogdir: *mut super::super::Foundation::PWSTR, diskcheckinterval: *mut u32, maxlogfilessize: *mut u32, minspaceondisk: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAuditLogSetParams(serveripaddress: super::super::Foundation::PWSTR, flags: u32, auditlogdir: super::super::Foundation::PWSTR, diskcheckinterval: u32, maxlogfilessize: u32, minspaceondisk: u32) -> u32;
    pub fn DhcpCApiCleanup();
    pub fn DhcpCApiInitialize(version: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateClass(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateClassV6(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO_V6) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateClientInfo(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateClientInfoV4(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_V4) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateClientInfoVQ(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_VQ) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateOption(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, optioninfo: *const DHCP_OPTION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateOptionV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut DHCP_OPTION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateOptionV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut DHCP_OPTION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateSubnet(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateSubnetV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut DHCP_SUBNET_INFO_V6) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateSubnetVQ(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO_VQ) -> u32;
    pub fn DhcpDeRegisterParamChange(flags: u32, reserved: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteClass(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteClassV6(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteClientInfo(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_SEARCH_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteClientInfoV6(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_SEARCH_INFO_V6) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteFilterV4(serveripaddress: super::super::Foundation::PWSTR, deletefilterinfo: *const DHCP_ADDR_PATTERN) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteServer(flags: u32, idinfo: *mut ::core::ffi::c_void, newserver: *mut DHCPDS_SERVER, callbackfn: *mut ::core::ffi::c_void, callbackdata: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteSubnet(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, forceflag: DHCP_FORCE_FLAG) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteSubnetV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, forceflag: DHCP_FORCE_FLAG) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteSuperScopeV4(serveripaddress: super::super::Foundation::PWSTR, superscopename: super::super::Foundation::PWSTR) -> u32;
    pub fn DhcpDsCleanup();
    pub fn DhcpDsInit() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumClasses(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, resumehandle: *mut u32, preferredmaximum: u32, classinfoarray: *mut *mut DHCP_CLASS_INFO_ARRAY, nread: *mut u32, ntotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumClassesV6(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, resumehandle: *mut u32, preferredmaximum: u32, classinfoarray: *mut *mut DHCP_CLASS_INFO_ARRAY_V6, nread: *mut u32, ntotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumFilterV4(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut DHCP_ADDR_PATTERN, preferredmaximum: u32, listtype: DHCP_FILTER_LIST_TYPE, enumfilterinfo: *mut *mut DHCP_FILTER_ENUM_INFO, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptionValues(serveripaddress: super::super::Foundation::PWSTR, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptionValuesV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptionValuesV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptions(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptionsV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptionsV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumServers(flags: u32, idinfo: *mut ::core::ffi::c_void, servers: *mut *mut DHCPDS_SERVERS, callbackfn: *mut ::core::ffi::c_void, callbackdata: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClients(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClientsFilterStatusInfo(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClientsV4(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V4, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClientsV5(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V5, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClientsV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, resumehandle: *mut DHCP_IPV6_ADDRESS, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V6, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClientsVQ(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_VQ, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetElements(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetElementsV4(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetElementsV5(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetElementsV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE_V6, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnets(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, enuminfo: *mut *mut DHCP_IP_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetsV6(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, enuminfo: *mut *mut DHCPV6_IP_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetAllOptionValues(serveripaddress: super::super::Foundation::PWSTR, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, values: *mut *mut DHCP_ALL_OPTION_VALUES) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetAllOptionValuesV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, values: *mut *mut DHCP_ALL_OPTION_VALUES) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetAllOptions(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionstruct: *mut *mut DHCP_ALL_OPTIONS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetAllOptionsV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionstruct: *mut *mut DHCP_ALL_OPTIONS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClassInfo(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, partialclassinfo: *mut DHCP_CLASS_INFO, filledclassinfo: *mut *mut DHCP_CLASS_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClientInfo(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClientInfoV4(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_V4) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClientInfoV6(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO_V6, clientinfo: *mut *mut DHCP_CLIENT_INFO_V6) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClientInfoVQ(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_VQ) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClientOptions(serveripaddress: super::super::Foundation::PWSTR, clientipaddress: u32, clientsubnetmask: u32, clientoptions: *mut *mut DHCP_OPTION_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetFilterV4(serveripaddress: super::super::Foundation::PWSTR, globalfilterinfo: *mut DHCP_FILTER_GLOBAL_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetMibInfo(serveripaddress: super::super::Foundation::PWSTR, mibinfo: *mut *mut DHCP_MIB_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetMibInfoV5(serveripaddress: super::super::Foundation::PWSTR, mibinfo: *mut *mut DHCP_MIB_INFO_V5) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetMibInfoV6(serveripaddress: super::super::Foundation::PWSTR, mibinfo: *mut *mut DHCP_MIB_INFO_V6) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionInfo(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, optioninfo: *mut *mut DHCP_OPTION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionInfoV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut *mut DHCP_OPTION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionInfoV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut *mut DHCP_OPTION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionValue(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionValueV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionValueV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOriginalSubnetMask(sadaptername: super::super::Foundation::PWSTR, dwsubnetmask: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetServerBindingInfo(serveripaddress: super::super::Foundation::PWSTR, flags: u32, bindelementsinfo: *mut *mut DHCP_BIND_ELEMENT_ARRAY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetServerBindingInfoV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, bindelementsinfo: *mut *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetServerSpecificStrings(serveripaddress: super::super::Foundation::PWSTR, serverspecificstrings: *mut *mut DHCP_SERVER_SPECIFIC_STRINGS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetSubnetDelayOffer(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, timedelayinmilliseconds: *mut u16) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetSubnetInfo(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *mut *mut DHCP_SUBNET_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetSubnetInfoV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut *mut DHCP_SUBNET_INFO_V6) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetSubnetInfoVQ(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *mut *mut DHCP_SUBNET_INFO_VQ) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetSuperScopeInfoV4(serveripaddress: super::super::Foundation::PWSTR, superscopetable: *mut *mut DHCP_SUPER_SCOPE_TABLE) -> u32;
    pub fn DhcpGetThreadOptions(pflags: *mut u32, reserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetVersion(serveripaddress: super::super::Foundation::PWSTR, majorversion: *mut u32, minorversion: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprAddV4PolicyCondition(policy: *mut DHCP_POLICY, parentexpr: u32, r#type: DHCP_POL_ATTR_TYPE, optionid: u32, suboptionid: u32, vendorname: super::super::Foundation::PWSTR, operator: DHCP_POL_COMPARATOR, value: *const u8, valuelength: u32, conditionindex: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprAddV4PolicyExpr(policy: *mut DHCP_POLICY, parentexpr: u32, operator: DHCP_POL_LOGIC_OPER, exprindex: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprAddV4PolicyRange(policy: *mut DHCP_POLICY, range: *const DHCP_IP_RANGE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprCreateV4Policy(policyname: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnet: u32, processingorder: u32, rootoperator: DHCP_POL_LOGIC_OPER, description: super::super::Foundation::PWSTR, enabled: super::super::Foundation::BOOL, policy: *mut *mut DHCP_POLICY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprCreateV4PolicyEx(policyname: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnet: u32, processingorder: u32, rootoperator: DHCP_POL_LOGIC_OPER, description: super::super::Foundation::PWSTR, enabled: super::super::Foundation::BOOL, policy: *mut *mut DHCP_POLICY_EX) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFindV4DhcpProperty(propertyarray: *const DHCP_PROPERTY_ARRAY, id: DHCP_PROPERTY_ID, r#type: DHCP_PROPERTY_TYPE) -> *mut DHCP_PROPERTY;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4DhcpProperty(property: *mut DHCP_PROPERTY);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4DhcpPropertyArray(propertyarray: *mut DHCP_PROPERTY_ARRAY);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4Policy(policy: *mut DHCP_POLICY);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4PolicyArray(policyarray: *mut DHCP_POLICY_ARRAY);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4PolicyEx(policyex: *mut DHCP_POLICY_EX);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4PolicyExArray(policyexarray: *mut DHCP_POLICY_EX_ARRAY);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprIsV4PolicySingleUC(policy: *const DHCP_POLICY) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprIsV4PolicyValid(ppolicy: *const DHCP_POLICY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprIsV4PolicyWellFormed(ppolicy: *const DHCP_POLICY) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprModifyV4PolicyExpr(policy: *mut DHCP_POLICY, operator: DHCP_POL_LOGIC_OPER) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprResetV4PolicyExpr(policy: *mut DHCP_POLICY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpModifyClass(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpModifyClassV6(serveripaddress: super::super::Foundation::PWSTR, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO_V6) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRegisterParamChange(flags: u32, reserved: *mut ::core::ffi::c_void, adaptername: super::super::Foundation::PWSTR, classid: *mut DHCPCAPI_CLASSID, params: DHCPCAPI_PARAMS_ARRAY, handle: *mut ::core::ffi::c_void) -> u32;
    pub fn DhcpRemoveDNSRegistrations() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOption(serveripaddress: super::super::Foundation::PWSTR, optionid: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOptionV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOptionV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOptionValue(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOptionValueV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOptionValueV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveSubnetElement(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA, forceflag: DHCP_FORCE_FLAG) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveSubnetElementV4(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V4, forceflag: DHCP_FORCE_FLAG) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveSubnetElementV5(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V5, forceflag: DHCP_FORCE_FLAG) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveSubnetElementV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, removeelementinfo: *mut DHCP_SUBNET_ELEMENT_DATA_V6, forceflag: DHCP_FORCE_FLAG) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRequestParams(flags: u32, reserved: *mut ::core::ffi::c_void, adaptername: super::super::Foundation::PWSTR, classid: *mut DHCPCAPI_CLASSID, sendparams: DHCPCAPI_PARAMS_ARRAY, recdparams: DHCPCAPI_PARAMS_ARRAY, buffer: *mut u8, psize: *mut u32, requestidstr: super::super::Foundation::PWSTR) -> u32;
    pub fn DhcpRpcFreeMemory(bufferpointer: *mut ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpScanDatabase(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, fixflag: u32, scanlist: *mut *mut DHCP_SCAN_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerAuditlogParamsFree(configinfo: *mut DHCP_SERVER_CONFIG_INFO_VQ);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerBackupDatabase(serveripaddress: super::super::Foundation::PWSTR, path: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerGetConfig(serveripaddress: super::super::Foundation::PWSTR, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerGetConfigV4(serveripaddress: super::super::Foundation::PWSTR, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerGetConfigV6(serveripaddress: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerGetConfigVQ(serveripaddress: super::super::Foundation::PWSTR, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerQueryAttribute(serveripaddr: super::super::Foundation::PWSTR, dwreserved: u32, dhcpattribid: u32, pdhcpattrib: *mut *mut DHCP_ATTRIB) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerQueryAttributes(serveripaddr: super::super::Foundation::PWSTR, dwreserved: u32, dwattribcount: u32, pdhcpattribs: *mut u32, pdhcpattribarr: *mut *mut DHCP_ATTRIB_ARRAY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerQueryDnsRegCredentials(serveripaddress: super::super::Foundation::PWSTR, unamesize: u32, uname: super::super::Foundation::PWSTR, domainsize: u32, domain: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerRedoAuthorization(serveripaddr: super::super::Foundation::PWSTR, dwreserved: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerRestoreDatabase(serveripaddress: super::super::Foundation::PWSTR, path: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetConfig(serveripaddress: super::super::Foundation::PWSTR, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetConfigV4(serveripaddress: super::super::Foundation::PWSTR, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetConfigV6(serveripaddress: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetConfigVQ(serveripaddress: super::super::Foundation::PWSTR, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetDnsRegCredentials(serveripaddress: super::super::Foundation::PWSTR, uname: super::super::Foundation::PWSTR, domain: super::super::Foundation::PWSTR, passwd: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetDnsRegCredentialsV5(serveripaddress: super::super::Foundation::PWSTR, uname: super::super::Foundation::PWSTR, domain: super::super::Foundation::PWSTR, passwd: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetClientInfo(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetClientInfoV4(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_V4) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetClientInfoV6(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_V6) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetClientInfoVQ(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_VQ) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetFilterV4(serveripaddress: super::super::Foundation::PWSTR, globalfilterinfo: *const DHCP_FILTER_GLOBAL_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionInfo(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, optioninfo: *const DHCP_OPTION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionInfoV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut DHCP_OPTION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionInfoV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, optioninfo: *mut DHCP_OPTION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionValue(serveripaddress: super::super::Foundation::PWSTR, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalue: *const DHCP_OPTION_DATA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionValueV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut DHCP_OPTION_DATA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionValueV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, optionvalue: *mut DHCP_OPTION_DATA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionValues(serveripaddress: super::super::Foundation::PWSTR, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalues: *const DHCP_OPTION_VALUE_ARRAY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionValuesV5(serveripaddress: super::super::Foundation::PWSTR, flags: u32, classname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalues: *mut DHCP_OPTION_VALUE_ARRAY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetServerBindingInfo(serveripaddress: super::super::Foundation::PWSTR, flags: u32, bindelementinfo: *mut DHCP_BIND_ELEMENT_ARRAY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetServerBindingInfoV6(serveripaddress: super::super::Foundation::PWSTR, flags: u32, bindelementinfo: *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetSubnetDelayOffer(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, timedelayinmilliseconds: u16) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetSubnetInfo(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetSubnetInfoV6(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut DHCP_SUBNET_INFO_V6) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetSubnetInfoVQ(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO_VQ) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetSuperScopeV4(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, superscopename: super::super::Foundation::PWSTR, changeexisting: super::super::Foundation::BOOL) -> u32;
    pub fn DhcpSetThreadOptions(flags: u32, reserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpUndoRequestParams(flags: u32, reserved: *mut ::core::ffi::c_void, adaptername: super::super::Foundation::PWSTR, requestidstr: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4AddPolicyRange(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, range: *const DHCP_IP_RANGE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4CreateClientInfo(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_PB) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4CreateClientInfoEx(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_EX) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4CreatePolicy(serveripaddress: super::super::Foundation::PWSTR, ppolicy: *const DHCP_POLICY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4CreatePolicyEx(serveripaddress: super::super::Foundation::PWSTR, policyex: *const DHCP_POLICY_EX) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4DeletePolicy(serveripaddress: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, policyname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4EnumPolicies(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, enuminfo: *mut *mut DHCP_POLICY_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4EnumPoliciesEx(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, globalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, enuminfo: *mut *mut DHCP_POLICY_EX_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4EnumSubnetClients(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_PB_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4EnumSubnetClientsEx(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_EX_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4EnumSubnetReservations(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_RESERVATION_INFO_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverAddScopeToRelationship(serveripaddress: super::super::Foundation::PWSTR, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverCreateRelationship(serveripaddress: super::super::Foundation::PWSTR, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverDeleteRelationship(serveripaddress: super::super::Foundation::PWSTR, prelationshipname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverDeleteScopeFromRelationship(serveripaddress: super::super::Foundation::PWSTR, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverEnumRelationship(serveripaddress: super::super::Foundation::PWSTR, resumehandle: *mut u32, preferredmaximum: u32, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP_ARRAY, relationshipread: *mut u32, relationshiptotal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetAddressStatus(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, pstatus: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetClientInfo(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCPV4_FAILOVER_CLIENT_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetRelationship(serveripaddress: super::super::Foundation::PWSTR, prelationshipname: super::super::Foundation::PWSTR, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetScopeRelationship(serveripaddress: super::super::Foundation::PWSTR, scopeid: u32, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetScopeStatistics(serveripaddress: super::super::Foundation::PWSTR, scopeid: u32, pstats: *mut *mut DHCP_FAILOVER_STATISTICS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetSystemTime(serveripaddress: super::super::Foundation::PWSTR, ptime: *mut u32, pmaxalloweddeltatime: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverSetRelationship(serveripaddress: super::super::Foundation::PWSTR, flags: u32, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverTriggerAddrAllocation(serveripaddress: super::super::Foundation::PWSTR, pfailrelname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetAllOptionValues(serveripaddress: super::super::Foundation::PWSTR, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, values: *mut *mut DHCP_ALL_OPTION_VALUES_PB) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetClientInfo(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_PB) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetClientInfoEx(serveripaddress: super::super::Foundation::PWSTR, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_EX) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetFreeIPAddress(serveripaddress: super::super::Foundation::PWSTR, scopeid: u32, startip: u32, endip: u32, numfreeaddrreq: u32, ipaddrlist: *mut *mut DHCP_IP_ARRAY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetOptionValue(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, policyname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetPolicy(serveripaddress: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, policy: *mut *mut DHCP_POLICY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetPolicyEx(serveripaddress: super::super::Foundation::PWSTR, globalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, policy: *mut *mut DHCP_POLICY_EX) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4QueryPolicyEnforcement(serveripaddress: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, enabled: *mut super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4RemoveOptionValue(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, policyname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4RemovePolicyRange(serveripaddress: super::super::Foundation::PWSTR, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, range: *const DHCP_IP_RANGE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4SetOptionValue(serveripaddress: super::super::Foundation::PWSTR, flags: u32, optionid: u32, policyname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut DHCP_OPTION_DATA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4SetOptionValues(serveripaddress: super::super::Foundation::PWSTR, flags: u32, policyname: super::super::Foundation::PWSTR, vendorname: super::super::Foundation::PWSTR, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalues: *mut DHCP_OPTION_VALUE_ARRAY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4SetPolicy(serveripaddress: super::super::Foundation::PWSTR, fieldsmodified: u32, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, policy: *const DHCP_POLICY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4SetPolicyEnforcement(serveripaddress: super::super::Foundation::PWSTR, fglobalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, enable: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4SetPolicyEx(serveripaddress: super::super::Foundation::PWSTR, fieldsmodified: u32, globalpolicy: super::super::Foundation::BOOL, subnetaddress: u32, policyname: super::super::Foundation::PWSTR, policy: *const DHCP_POLICY_EX) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV6CreateClientInfo(serveripaddress: super::super::Foundation::PWSTR, clientinfo: *const DHCP_CLIENT_INFO_V6) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV6GetFreeIPAddress(serveripaddress: super::super::Foundation::PWSTR, scopeid: DHCP_IPV6_ADDRESS, startip: DHCP_IPV6_ADDRESS, endip: DHCP_IPV6_ADDRESS, numfreeaddrreq: u32, ipaddrlist: *mut *mut DHCPV6_IP_ARRAY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV6GetStatelessStatistics(serveripaddress: super::super::Foundation::PWSTR, statelessstats: *mut *mut DHCPV6_STATELESS_STATS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV6GetStatelessStoreParams(serveripaddress: super::super::Foundation::PWSTR, fserverlevel: super::super::Foundation::BOOL, subnetaddress: DHCP_IPV6_ADDRESS, params: *mut *mut DHCPV6_STATELESS_PARAMS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV6SetStatelessStoreParams(serveripaddress: super::super::Foundation::PWSTR, fserverlevel: super::super::Foundation::BOOL, subnetaddress: DHCP_IPV6_ADDRESS, fieldmodified: u32, params: *const DHCPV6_STATELESS_PARAMS) -> u32;
    pub fn Dhcpv6CApiCleanup();
    pub fn Dhcpv6CApiInitialize(version: *mut u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn Dhcpv6ReleasePrefix(adaptername: super::super::Foundation::PWSTR, classid: *mut DHCPV6CAPI_CLASSID, leaseinfo: *mut DHCPV6PrefixLeaseInformation) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Dhcpv6RenewPrefix(adaptername: super::super::Foundation::PWSTR, pclassid: *mut DHCPV6CAPI_CLASSID, prefixleaseinfo: *mut DHCPV6PrefixLeaseInformation, pdwtimetowait: *mut u32, bvalidateprefix: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Dhcpv6RequestParams(forcenewinform: super::super::Foundation::BOOL, reserved: *mut ::core::ffi::c_void, adaptername: super::super::Foundation::PWSTR, classid: *mut DHCPV6CAPI_CLASSID, recdparams: DHCPV6CAPI_PARAMS_ARRAY, buffer: *mut u8, psize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Dhcpv6RequestPrefix(adaptername: super::super::Foundation::PWSTR, pclassid: *mut DHCPV6CAPI_CLASSID, prefixleaseinfo: *mut DHCPV6PrefixLeaseInformation, pdwtimetowait: *mut u32) -> u32;
}
pub const ADDRESS_TYPE_IANA: u32 = 0u32;
pub const ADDRESS_TYPE_IATA: u32 = 1u32;
pub const CHANGESTATE: u32 = 4u32;
pub const CLIENT_TYPE_BOOTP: u32 = 2u32;
pub const CLIENT_TYPE_DHCP: u32 = 1u32;
pub const CLIENT_TYPE_NONE: u32 = 100u32;
pub const CLIENT_TYPE_RESERVATION_FLAG: u32 = 4u32;
pub const CLIENT_TYPE_UNSPECIFIED: u32 = 0u32;
#[repr(C)]
pub struct DATE_TIME(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCPAPI_PARAMS(i32);
#[repr(C)]
pub struct DHCPCAPI_CLASSID(i32);
pub const DHCPCAPI_DEREGISTER_HANDLE_EVENT: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCPCAPI_PARAMS_ARRAY(i32);
pub const DHCPCAPI_REGISTER_HANDLE_EVENT: u32 = 1u32;
pub const DHCPCAPI_REQUEST_ASYNCHRONOUS: u32 = 4u32;
pub const DHCPCAPI_REQUEST_CANCEL: u32 = 8u32;
pub const DHCPCAPI_REQUEST_MASK: u32 = 15u32;
pub const DHCPCAPI_REQUEST_PERSISTENT: u32 = 1u32;
pub const DHCPCAPI_REQUEST_SYNCHRONOUS: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCPDS_SERVER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCPDS_SERVERS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCPV4_FAILOVER_CLIENT_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCPV4_FAILOVER_CLIENT_INFO_ARRAY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCPV4_FAILOVER_CLIENT_INFO_EX(i32);
#[repr(C)]
pub struct DHCPV6CAPI_CLASSID(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCPV6CAPI_PARAMS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCPV6CAPI_PARAMS_ARRAY(i32);
#[repr(C)]
pub struct DHCPV6Prefix(i32);
#[repr(C)]
pub struct DHCPV6PrefixLeaseInformation(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCPV6_BIND_ELEMENT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCPV6_BIND_ELEMENT_ARRAY(i32);
#[repr(C)]
pub struct DHCPV6_IP_ARRAY(i32);
pub const DHCPV6_OPTION_CLIENTID: u32 = 1u32;
pub const DHCPV6_OPTION_DNS_SERVERS: u32 = 23u32;
pub const DHCPV6_OPTION_DOMAIN_LIST: u32 = 24u32;
pub const DHCPV6_OPTION_IA_NA: u32 = 3u32;
pub const DHCPV6_OPTION_IA_PD: u32 = 25u32;
pub const DHCPV6_OPTION_IA_TA: u32 = 4u32;
pub const DHCPV6_OPTION_NISP_DOMAIN_NAME: u32 = 30u32;
pub const DHCPV6_OPTION_NISP_SERVERS: u32 = 28u32;
pub const DHCPV6_OPTION_NIS_DOMAIN_NAME: u32 = 29u32;
pub const DHCPV6_OPTION_NIS_SERVERS: u32 = 27u32;
pub const DHCPV6_OPTION_ORO: u32 = 6u32;
pub const DHCPV6_OPTION_PREFERENCE: u32 = 7u32;
pub const DHCPV6_OPTION_RAPID_COMMIT: u32 = 14u32;
pub const DHCPV6_OPTION_RECONF_MSG: u32 = 19u32;
pub const DHCPV6_OPTION_SERVERID: u32 = 2u32;
pub const DHCPV6_OPTION_SIP_SERVERS_ADDRS: u32 = 22u32;
pub const DHCPV6_OPTION_SIP_SERVERS_NAMES: u32 = 21u32;
pub const DHCPV6_OPTION_UNICAST: u32 = 12u32;
pub const DHCPV6_OPTION_USER_CLASS: u32 = 15u32;
pub const DHCPV6_OPTION_VENDOR_CLASS: u32 = 16u32;
pub const DHCPV6_OPTION_VENDOR_OPTS: u32 = 17u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCPV6_STATELESS_PARAMS(i32);
#[repr(transparent)]
pub struct DHCPV6_STATELESS_PARAM_TYPE(pub i32);
pub const DhcpStatelessPurgeInterval: DHCPV6_STATELESS_PARAM_TYPE = DHCPV6_STATELESS_PARAM_TYPE(1i32);
pub const DhcpStatelessStatus: DHCPV6_STATELESS_PARAM_TYPE = DHCPV6_STATELESS_PARAM_TYPE(2i32);
#[repr(C)]
pub struct DHCPV6_STATELESS_SCOPE_STATS(i32);
#[repr(C)]
pub struct DHCPV6_STATELESS_STATS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_ADDR_PATTERN(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_ALL_OPTIONS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_ALL_OPTION_VALUES(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_ALL_OPTION_VALUES_PB(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_ATTRIB(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_ATTRIB_ARRAY(i32);
pub const DHCP_ATTRIB_BOOL_IS_ADMIN: u32 = 5u32;
pub const DHCP_ATTRIB_BOOL_IS_BINDING_AWARE: u32 = 4u32;
pub const DHCP_ATTRIB_BOOL_IS_DYNBOOTP: u32 = 2u32;
pub const DHCP_ATTRIB_BOOL_IS_PART_OF_DSDC: u32 = 3u32;
pub const DHCP_ATTRIB_BOOL_IS_ROGUE: u32 = 1u32;
pub const DHCP_ATTRIB_TYPE_BOOL: u32 = 1u32;
pub const DHCP_ATTRIB_TYPE_ULONG: u32 = 2u32;
pub const DHCP_ATTRIB_ULONG_RESTORE_STATUS: u32 = 6u32;
#[repr(C)]
pub struct DHCP_BINARY_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_BIND_ELEMENT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_BIND_ELEMENT_ARRAY(i32);
#[repr(C)]
pub struct DHCP_BOOTP_IP_RANGE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CALLOUT_TABLE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLASS_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLASS_INFO_ARRAY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLASS_INFO_ARRAY_V6(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLASS_INFO_V6(i32);
pub const DHCP_CLIENT_BOOTP: u32 = 805306371u32;
pub const DHCP_CLIENT_DHCP: u32 = 805306372u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLIENT_FILTER_STATUS_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLIENT_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLIENT_INFO_ARRAY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLIENT_INFO_ARRAY_V4(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLIENT_INFO_ARRAY_V5(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLIENT_INFO_ARRAY_V6(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLIENT_INFO_ARRAY_VQ(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLIENT_INFO_EX(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLIENT_INFO_EX_ARRAY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLIENT_INFO_PB(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLIENT_INFO_PB_ARRAY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLIENT_INFO_V4(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLIENT_INFO_V5(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLIENT_INFO_V6(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_CLIENT_INFO_VQ(i32);
#[repr(C)]
pub struct DHCP_CLIENT_SEARCH_UNION(i32);
pub const DHCP_CONTROL_CONTINUE: u32 = 4u32;
pub const DHCP_CONTROL_PAUSE: u32 = 3u32;
pub const DHCP_CONTROL_START: u32 = 1u32;
pub const DHCP_CONTROL_STOP: u32 = 2u32;
pub const DHCP_DROP_DUPLICATE: u32 = 1u32;
pub const DHCP_DROP_GEN_FAILURE: u32 = 256u32;
pub const DHCP_DROP_INTERNAL_ERROR: u32 = 3u32;
pub const DHCP_DROP_INVALID: u32 = 8u32;
pub const DHCP_DROP_NOADDRESS: u32 = 10u32;
pub const DHCP_DROP_NOMEM: u32 = 2u32;
pub const DHCP_DROP_NO_SUBNETS: u32 = 7u32;
pub const DHCP_DROP_PAUSED: u32 = 6u32;
pub const DHCP_DROP_PROCESSED: u32 = 11u32;
pub const DHCP_DROP_TIMEOUT: u32 = 4u32;
pub const DHCP_DROP_UNAUTH: u32 = 5u32;
pub const DHCP_DROP_WRONG_SERVER: u32 = 9u32;
pub const DHCP_ENDPOINT_FLAG_CANT_MODIFY: u32 = 1u32;
pub const DHCP_FAILOVER_DELETE_SCOPES: u32 = 1u32;
pub const DHCP_FAILOVER_MAX_NUM_ADD_SCOPES: u32 = 400u32;
pub const DHCP_FAILOVER_MAX_NUM_REL: u32 = 31u32;
#[repr(transparent)]
pub struct DHCP_FAILOVER_MODE(pub i32);
pub const LoadBalance: DHCP_FAILOVER_MODE = DHCP_FAILOVER_MODE(0i32);
pub const HotStandby: DHCP_FAILOVER_MODE = DHCP_FAILOVER_MODE(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_FAILOVER_RELATIONSHIP(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_FAILOVER_RELATIONSHIP_ARRAY(i32);
#[repr(transparent)]
pub struct DHCP_FAILOVER_SERVER(pub i32);
pub const PrimaryServer: DHCP_FAILOVER_SERVER = DHCP_FAILOVER_SERVER(0i32);
pub const SecondaryServer: DHCP_FAILOVER_SERVER = DHCP_FAILOVER_SERVER(1i32);
#[repr(C)]
pub struct DHCP_FAILOVER_STATISTICS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_FILTER_ADD_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_FILTER_ENUM_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_FILTER_GLOBAL_INFO(i32);
#[repr(transparent)]
pub struct DHCP_FILTER_LIST_TYPE(pub i32);
pub const Deny: DHCP_FILTER_LIST_TYPE = DHCP_FILTER_LIST_TYPE(0i32);
pub const Allow: DHCP_FILTER_LIST_TYPE = DHCP_FILTER_LIST_TYPE(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_FILTER_RECORD(i32);
pub const DHCP_FLAGS_DONT_ACCESS_DS: u32 = 1u32;
pub const DHCP_FLAGS_DONT_DO_RPC: u32 = 2u32;
pub const DHCP_FLAGS_OPTION_IS_VENDOR: u32 = 3u32;
#[repr(transparent)]
pub struct DHCP_FORCE_FLAG(pub i32);
pub const DhcpFullForce: DHCP_FORCE_FLAG = DHCP_FORCE_FLAG(0i32);
pub const DhcpNoForce: DHCP_FORCE_FLAG = DHCP_FORCE_FLAG(1i32);
pub const DhcpFailoverForce: DHCP_FORCE_FLAG = DHCP_FORCE_FLAG(2i32);
pub const DHCP_GIVE_ADDRESS_NEW: u32 = 805306369u32;
pub const DHCP_GIVE_ADDRESS_OLD: u32 = 805306370u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_HOST_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_HOST_INFO_V6(i32);
#[repr(C)]
pub struct DHCP_IPV6_ADDRESS(i32);
#[repr(C)]
pub struct DHCP_IP_ARRAY(i32);
#[repr(C)]
pub struct DHCP_IP_CLUSTER(i32);
#[repr(C)]
pub struct DHCP_IP_RANGE(i32);
#[repr(C)]
pub struct DHCP_IP_RANGE_ARRAY(i32);
#[repr(C)]
pub struct DHCP_IP_RANGE_V6(i32);
#[repr(C)]
pub struct DHCP_IP_RESERVATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_IP_RESERVATION_INFO(i32);
#[repr(C)]
pub struct DHCP_IP_RESERVATION_V4(i32);
#[repr(C)]
pub struct DHCP_IP_RESERVATION_V6(i32);
pub const DHCP_MAX_DELAY: u32 = 1000u32;
#[repr(C)]
pub struct DHCP_MIB_INFO(i32);
#[repr(C)]
pub struct DHCP_MIB_INFO_V5(i32);
#[repr(C)]
pub struct DHCP_MIB_INFO_V6(i32);
#[repr(C)]
pub struct DHCP_MIB_INFO_VQ(i32);
pub const DHCP_MIN_DELAY: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_OPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_OPTION_ARRAY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_OPTION_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_OPTION_DATA_ELEMENT(i32);
#[repr(transparent)]
pub struct DHCP_OPTION_DATA_TYPE(pub i32);
pub const DhcpByteOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(0i32);
pub const DhcpWordOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(1i32);
pub const DhcpDWordOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(2i32);
pub const DhcpDWordDWordOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(3i32);
pub const DhcpIpAddressOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(4i32);
pub const DhcpStringDataOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(5i32);
pub const DhcpBinaryDataOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(6i32);
pub const DhcpEncapsulatedDataOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(7i32);
pub const DhcpIpv6AddressOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(8i32);
#[repr(C)]
pub struct DHCP_OPTION_ELEMENT_UNION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_OPTION_LIST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_OPTION_SCOPE_INFO(i32);
#[repr(C)]
pub struct DHCP_OPTION_SCOPE_INFO6(i32);
#[repr(transparent)]
pub struct DHCP_OPTION_SCOPE_TYPE(pub i32);
pub const DhcpDefaultOptions: DHCP_OPTION_SCOPE_TYPE = DHCP_OPTION_SCOPE_TYPE(0i32);
pub const DhcpGlobalOptions: DHCP_OPTION_SCOPE_TYPE = DHCP_OPTION_SCOPE_TYPE(1i32);
pub const DhcpSubnetOptions: DHCP_OPTION_SCOPE_TYPE = DHCP_OPTION_SCOPE_TYPE(2i32);
pub const DhcpReservedOptions: DHCP_OPTION_SCOPE_TYPE = DHCP_OPTION_SCOPE_TYPE(3i32);
pub const DhcpMScopeOptions: DHCP_OPTION_SCOPE_TYPE = DHCP_OPTION_SCOPE_TYPE(4i32);
#[repr(transparent)]
pub struct DHCP_OPTION_SCOPE_TYPE6(pub i32);
pub const DhcpDefaultOptions6: DHCP_OPTION_SCOPE_TYPE6 = DHCP_OPTION_SCOPE_TYPE6(0i32);
pub const DhcpScopeOptions6: DHCP_OPTION_SCOPE_TYPE6 = DHCP_OPTION_SCOPE_TYPE6(1i32);
pub const DhcpReservedOptions6: DHCP_OPTION_SCOPE_TYPE6 = DHCP_OPTION_SCOPE_TYPE6(2i32);
pub const DhcpGlobalOptions6: DHCP_OPTION_SCOPE_TYPE6 = DHCP_OPTION_SCOPE_TYPE6(3i32);
#[repr(C)]
pub struct DHCP_OPTION_SCOPE_UNION6(i32);
#[repr(transparent)]
pub struct DHCP_OPTION_TYPE(pub i32);
pub const DhcpUnaryElementTypeOption: DHCP_OPTION_TYPE = DHCP_OPTION_TYPE(0i32);
pub const DhcpArrayTypeOption: DHCP_OPTION_TYPE = DHCP_OPTION_TYPE(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_OPTION_VALUE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_OPTION_VALUE_ARRAY(i32);
pub const DHCP_OPT_ENUM_IGNORE_VENDOR: u32 = 1u32;
pub const DHCP_OPT_ENUM_USE_CLASSNAME: u32 = 2u32;
#[repr(C)]
pub struct DHCP_PERF_STATS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_POLICY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_POLICY_ARRAY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_POLICY_EX(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_POLICY_EX_ARRAY(i32);
#[repr(transparent)]
pub struct DHCP_POLICY_FIELDS_TO_UPDATE(pub i32);
pub const DhcpUpdatePolicyName: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(1i32);
pub const DhcpUpdatePolicyOrder: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(2i32);
pub const DhcpUpdatePolicyExpr: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(4i32);
pub const DhcpUpdatePolicyRanges: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(8i32);
pub const DhcpUpdatePolicyDescr: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(16i32);
pub const DhcpUpdatePolicyStatus: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(32i32);
pub const DhcpUpdatePolicyDnsSuffix: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(64i32);
#[repr(transparent)]
pub struct DHCP_POL_ATTR_TYPE(pub i32);
pub const DhcpAttrHWAddr: DHCP_POL_ATTR_TYPE = DHCP_POL_ATTR_TYPE(0i32);
pub const DhcpAttrOption: DHCP_POL_ATTR_TYPE = DHCP_POL_ATTR_TYPE(1i32);
pub const DhcpAttrSubOption: DHCP_POL_ATTR_TYPE = DHCP_POL_ATTR_TYPE(2i32);
pub const DhcpAttrFqdn: DHCP_POL_ATTR_TYPE = DHCP_POL_ATTR_TYPE(3i32);
pub const DhcpAttrFqdnSingleLabel: DHCP_POL_ATTR_TYPE = DHCP_POL_ATTR_TYPE(4i32);
#[repr(transparent)]
pub struct DHCP_POL_COMPARATOR(pub i32);
pub const DhcpCompEqual: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(0i32);
pub const DhcpCompNotEqual: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(1i32);
pub const DhcpCompBeginsWith: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(2i32);
pub const DhcpCompNotBeginWith: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(3i32);
pub const DhcpCompEndsWith: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(4i32);
pub const DhcpCompNotEndWith: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(5i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_POL_COND(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_POL_COND_ARRAY(i32);
#[repr(C)]
pub struct DHCP_POL_EXPR(i32);
#[repr(C)]
pub struct DHCP_POL_EXPR_ARRAY(i32);
#[repr(transparent)]
pub struct DHCP_POL_LOGIC_OPER(pub i32);
pub const DhcpLogicalOr: DHCP_POL_LOGIC_OPER = DHCP_POL_LOGIC_OPER(0i32);
pub const DhcpLogicalAnd: DHCP_POL_LOGIC_OPER = DHCP_POL_LOGIC_OPER(1i32);
pub const DHCP_PROB_CONFLICT: u32 = 536870913u32;
pub const DHCP_PROB_DECLINE: u32 = 536870914u32;
pub const DHCP_PROB_NACKED: u32 = 536870916u32;
pub const DHCP_PROB_RELEASE: u32 = 536870915u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_PROPERTY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_PROPERTY_ARRAY(i32);
#[repr(transparent)]
pub struct DHCP_PROPERTY_ID(pub i32);
pub const DhcpPropIdPolicyDnsSuffix: DHCP_PROPERTY_ID = DHCP_PROPERTY_ID(0i32);
pub const DhcpPropIdClientAddressStateEx: DHCP_PROPERTY_ID = DHCP_PROPERTY_ID(1i32);
#[repr(transparent)]
pub struct DHCP_PROPERTY_TYPE(pub i32);
pub const DhcpPropTypeByte: DHCP_PROPERTY_TYPE = DHCP_PROPERTY_TYPE(0i32);
pub const DhcpPropTypeWord: DHCP_PROPERTY_TYPE = DHCP_PROPERTY_TYPE(1i32);
pub const DhcpPropTypeDword: DHCP_PROPERTY_TYPE = DHCP_PROPERTY_TYPE(2i32);
pub const DhcpPropTypeString: DHCP_PROPERTY_TYPE = DHCP_PROPERTY_TYPE(3i32);
pub const DhcpPropTypeBinary: DHCP_PROPERTY_TYPE = DHCP_PROPERTY_TYPE(4i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_RESERVATION_INFO_ARRAY(i32);
#[repr(C)]
pub struct DHCP_RESERVED_SCOPE(i32);
#[repr(C)]
pub struct DHCP_RESERVED_SCOPE6(i32);
#[repr(transparent)]
pub struct DHCP_SCAN_FLAG(pub i32);
pub const DhcpRegistryFix: DHCP_SCAN_FLAG = DHCP_SCAN_FLAG(0i32);
pub const DhcpDatabaseFix: DHCP_SCAN_FLAG = DHCP_SCAN_FLAG(1i32);
#[repr(C)]
pub struct DHCP_SCAN_ITEM(i32);
#[repr(C)]
pub struct DHCP_SCAN_LIST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SEARCH_INFO(i32);
#[repr(transparent)]
pub struct DHCP_SEARCH_INFO_TYPE(pub i32);
pub const DhcpClientIpAddress: DHCP_SEARCH_INFO_TYPE = DHCP_SEARCH_INFO_TYPE(0i32);
pub const DhcpClientHardwareAddress: DHCP_SEARCH_INFO_TYPE = DHCP_SEARCH_INFO_TYPE(1i32);
pub const DhcpClientName: DHCP_SEARCH_INFO_TYPE = DHCP_SEARCH_INFO_TYPE(2i32);
#[repr(transparent)]
pub struct DHCP_SEARCH_INFO_TYPE_V6(pub i32);
pub const Dhcpv6ClientIpAddress: DHCP_SEARCH_INFO_TYPE_V6 = DHCP_SEARCH_INFO_TYPE_V6(0i32);
pub const Dhcpv6ClientDUID: DHCP_SEARCH_INFO_TYPE_V6 = DHCP_SEARCH_INFO_TYPE_V6(1i32);
pub const Dhcpv6ClientName: DHCP_SEARCH_INFO_TYPE_V6 = DHCP_SEARCH_INFO_TYPE_V6(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SEARCH_INFO_V6(i32);
pub const DHCP_SEND_PACKET: u32 = 268435456u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SERVER_CONFIG_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SERVER_CONFIG_INFO_V4(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SERVER_CONFIG_INFO_V6(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SERVER_CONFIG_INFO_VQ(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SERVER_OPTIONS(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SERVER_OPTIONS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SERVER_SPECIFIC_STRINGS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_DATA_V4(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_DATA_V5(i32);
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_DATA_V6(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5(i32);
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6(i32);
#[repr(transparent)]
pub struct DHCP_SUBNET_ELEMENT_TYPE(pub i32);
pub const DhcpIpRanges: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(0i32);
pub const DhcpSecondaryHosts: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(1i32);
pub const DhcpReservedIps: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(2i32);
pub const DhcpExcludedIpRanges: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(3i32);
pub const DhcpIpUsedClusters: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(4i32);
pub const DhcpIpRangesDhcpOnly: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(5i32);
pub const DhcpIpRangesDhcpBootp: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(6i32);
pub const DhcpIpRangesBootpOnly: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(7i32);
#[repr(transparent)]
pub struct DHCP_SUBNET_ELEMENT_TYPE_V6(pub i32);
pub const Dhcpv6IpRanges: DHCP_SUBNET_ELEMENT_TYPE_V6 = DHCP_SUBNET_ELEMENT_TYPE_V6(0i32);
pub const Dhcpv6ReservedIps: DHCP_SUBNET_ELEMENT_TYPE_V6 = DHCP_SUBNET_ELEMENT_TYPE_V6(1i32);
pub const Dhcpv6ExcludedIpRanges: DHCP_SUBNET_ELEMENT_TYPE_V6 = DHCP_SUBNET_ELEMENT_TYPE_V6(2i32);
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_UNION(i32);
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_UNION_V4(i32);
#[repr(C)]
pub struct DHCP_SUBNET_ELEMENT_UNION_V6(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SUBNET_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SUBNET_INFO_V6(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SUBNET_INFO_VQ(i32);
pub const DHCP_SUBNET_INFO_VQ_FLAG_QUARANTINE: u32 = 1u32;
#[repr(transparent)]
pub struct DHCP_SUBNET_STATE(pub i32);
pub const DhcpSubnetEnabled: DHCP_SUBNET_STATE = DHCP_SUBNET_STATE(0i32);
pub const DhcpSubnetDisabled: DHCP_SUBNET_STATE = DHCP_SUBNET_STATE(1i32);
pub const DhcpSubnetEnabledSwitched: DHCP_SUBNET_STATE = DHCP_SUBNET_STATE(2i32);
pub const DhcpSubnetDisabledSwitched: DHCP_SUBNET_STATE = DHCP_SUBNET_STATE(3i32);
pub const DhcpSubnetInvalidState: DHCP_SUBNET_STATE = DHCP_SUBNET_STATE(4i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SUPER_SCOPE_TABLE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DHCP_SUPER_SCOPE_TABLE_ENTRY(i32);
pub const DNS_FLAG_CLEANUP_EXPIRED: u32 = 4u32;
pub const DNS_FLAG_DISABLE_PTR_UPDATE: u32 = 64u32;
pub const DNS_FLAG_ENABLED: u32 = 1u32;
pub const DNS_FLAG_HAS_DNS_SUFFIX: u32 = 128u32;
pub const DNS_FLAG_UPDATE_BOTH_ALWAYS: u32 = 16u32;
pub const DNS_FLAG_UPDATE_DHCID: u32 = 32u32;
pub const DNS_FLAG_UPDATE_DOWNLEVEL: u32 = 2u32;
#[repr(C)]
pub struct DWORD_DWORD(i32);
pub const ERROR_DDS_CLASS_DOES_NOT_EXIST: u32 = 20078u32;
pub const ERROR_DDS_CLASS_EXISTS: u32 = 20077u32;
pub const ERROR_DDS_DHCP_SERVER_NOT_FOUND: u32 = 20074u32;
pub const ERROR_DDS_NO_DHCP_ROOT: u32 = 20071u32;
pub const ERROR_DDS_NO_DS_AVAILABLE: u32 = 20070u32;
pub const ERROR_DDS_OPTION_ALREADY_EXISTS: u32 = 20075u32;
pub const ERROR_DDS_OPTION_DOES_NOT_EXIST: u32 = 20076u32;
pub const ERROR_DDS_POSSIBLE_RANGE_CONFLICT: u32 = 20087u32;
pub const ERROR_DDS_RANGE_DOES_NOT_EXIST: u32 = 20088u32;
pub const ERROR_DDS_RESERVATION_CONFLICT: u32 = 20086u32;
pub const ERROR_DDS_RESERVATION_NOT_PRESENT: u32 = 20085u32;
pub const ERROR_DDS_SERVER_ADDRESS_MISMATCH: u32 = 20081u32;
pub const ERROR_DDS_SERVER_ALREADY_EXISTS: u32 = 20079u32;
pub const ERROR_DDS_SERVER_DOES_NOT_EXIST: u32 = 20080u32;
pub const ERROR_DDS_SUBNET_EXISTS: u32 = 20082u32;
pub const ERROR_DDS_SUBNET_HAS_DIFF_SSCOPE: u32 = 20083u32;
pub const ERROR_DDS_SUBNET_NOT_PRESENT: u32 = 20084u32;
pub const ERROR_DDS_TOO_MANY_ERRORS: u32 = 20073u32;
pub const ERROR_DDS_UNEXPECTED_ERROR: u32 = 20072u32;
pub const ERROR_DHCP_ADDRESS_NOT_AVAILABLE: u32 = 20011u32;
pub const ERROR_DHCP_CANNOT_MODIFY_BINDINGS: u32 = 20051u32;
pub const ERROR_DHCP_CANT_CHANGE_ATTRIBUTE: u32 = 20048u32;
pub const ERROR_DHCP_CLASS_ALREADY_EXISTS: u32 = 20045u32;
pub const ERROR_DHCP_CLASS_NOT_FOUND: u32 = 20044u32;
pub const ERROR_DHCP_CLIENT_EXISTS: u32 = 20014u32;
pub const ERROR_DHCP_DATABASE_INIT_FAILED: u32 = 20001u32;
pub const ERROR_DHCP_DEFAULT_SCOPE_EXITS: u32 = 20047u32;
pub const ERROR_DHCP_DELETE_BUILTIN_CLASS: u32 = 20089u32;
pub const ERROR_DHCP_ELEMENT_CANT_REMOVE: u32 = 20007u32;
pub const ERROR_DHCP_EXEMPTION_EXISTS: u32 = 20055u32;
pub const ERROR_DHCP_EXEMPTION_NOT_PRESENT: u32 = 20056u32;
pub const ERROR_DHCP_FO_ADDSCOPE_LEASES_NOT_SYNCED: u32 = 20127u32;
pub const ERROR_DHCP_FO_BOOT_NOT_SUPPORTED: u32 = 20131u32;
pub const ERROR_DHCP_FO_FEATURE_NOT_SUPPORTED: u32 = 20134u32;
pub const ERROR_DHCP_FO_IPRANGE_TYPE_CONV_ILLEGAL: u32 = 20129u32;
pub const ERROR_DHCP_FO_MAX_ADD_SCOPES: u32 = 20130u32;
pub const ERROR_DHCP_FO_MAX_RELATIONSHIPS: u32 = 20128u32;
pub const ERROR_DHCP_FO_NOT_SUPPORTED: u32 = 20118u32;
pub const ERROR_DHCP_FO_RANGE_PART_OF_REL: u32 = 20132u32;
pub const ERROR_DHCP_FO_RELATIONSHIP_DOES_NOT_EXIST: u32 = 20115u32;
pub const ERROR_DHCP_FO_RELATIONSHIP_EXISTS: u32 = 20114u32;
pub const ERROR_DHCP_FO_RELATIONSHIP_NAME_TOO_LONG: u32 = 20125u32;
pub const ERROR_DHCP_FO_RELATION_IS_SECONDARY: u32 = 20117u32;
pub const ERROR_DHCP_FO_SCOPE_ALREADY_IN_RELATIONSHIP: u32 = 20113u32;
pub const ERROR_DHCP_FO_SCOPE_NOT_IN_RELATIONSHIP: u32 = 20116u32;
pub const ERROR_DHCP_FO_SCOPE_SYNC_IN_PROGRESS: u32 = 20133u32;
pub const ERROR_DHCP_FO_STATE_NOT_NORMAL: u32 = 20120u32;
pub const ERROR_DHCP_FO_TIME_OUT_OF_SYNC: u32 = 20119u32;
pub const ERROR_DHCP_HARDWARE_ADDRESS_TYPE_ALREADY_EXEMPT: u32 = 20101u32;
pub const ERROR_DHCP_INVALID_DELAY: u32 = 20092u32;
pub const ERROR_DHCP_INVALID_DHCP_CLIENT: u32 = 20016u32;
pub const ERROR_DHCP_INVALID_DHCP_MESSAGE: u32 = 20015u32;
pub const ERROR_DHCP_INVALID_PARAMETER_OPTION32: u32 = 20057u32;
pub const ERROR_DHCP_INVALID_POLICY_EXPRESSION: u32 = 20109u32;
pub const ERROR_DHCP_INVALID_PROCESSING_ORDER: u32 = 20110u32;
pub const ERROR_DHCP_INVALID_RANGE: u32 = 20023u32;
pub const ERROR_DHCP_INVALID_SUBNET_PREFIX: u32 = 20091u32;
pub const ERROR_DHCP_IPRANGE_CONV_ILLEGAL: u32 = 20049u32;
pub const ERROR_DHCP_IPRANGE_EXITS: u32 = 20021u32;
pub const ERROR_DHCP_IP_ADDRESS_IN_USE: u32 = 20032u32;
pub const ERROR_DHCP_JET97_CONV_REQUIRED: u32 = 20036u32;
pub const ERROR_DHCP_JET_CONV_REQUIRED: u32 = 20027u32;
pub const ERROR_DHCP_JET_ERROR: u32 = 20013u32;
pub const ERROR_DHCP_LINKLAYER_ADDRESS_DOES_NOT_EXIST: u32 = 20095u32;
pub const ERROR_DHCP_LINKLAYER_ADDRESS_EXISTS: u32 = 20093u32;
pub const ERROR_DHCP_LINKLAYER_ADDRESS_RESERVATION_EXISTS: u32 = 20094u32;
pub const ERROR_DHCP_LOG_FILE_PATH_TOO_LONG: u32 = 20033u32;
pub const ERROR_DHCP_MSCOPE_EXISTS: u32 = 20053u32;
pub const ERROR_DHCP_NAP_NOT_SUPPORTED: u32 = 20138u32;
pub const ERROR_DHCP_NETWORK_CHANGED: u32 = 20050u32;
pub const ERROR_DHCP_NETWORK_INIT_FAILED: u32 = 20003u32;
pub const ERROR_DHCP_NOT_RESERVED_CLIENT: u32 = 20018u32;
pub const ERROR_DHCP_NO_ADMIN_PERMISSION: u32 = 20121u32;
pub const ERROR_DHCP_OPTION_EXITS: u32 = 20009u32;
pub const ERROR_DHCP_OPTION_NOT_PRESENT: u32 = 20010u32;
pub const ERROR_DHCP_OPTION_TYPE_MISMATCH: u32 = 20103u32;
pub const ERROR_DHCP_POLICY_BAD_PARENT_EXPR: u32 = 20104u32;
pub const ERROR_DHCP_POLICY_EDIT_FQDN_UNSUPPORTED: u32 = 20137u32;
pub const ERROR_DHCP_POLICY_EXISTS: u32 = 20105u32;
pub const ERROR_DHCP_POLICY_FQDN_OPTION_UNSUPPORTED: u32 = 20136u32;
pub const ERROR_DHCP_POLICY_FQDN_RANGE_UNSUPPORTED: u32 = 20135u32;
pub const ERROR_DHCP_POLICY_NOT_FOUND: u32 = 20111u32;
pub const ERROR_DHCP_POLICY_RANGE_BAD: u32 = 20107u32;
pub const ERROR_DHCP_POLICY_RANGE_EXISTS: u32 = 20106u32;
pub const ERROR_DHCP_PRIMARY_NOT_FOUND: u32 = 20006u32;
pub const ERROR_DHCP_RANGE_EXTENDED: u32 = 20024u32;
pub const ERROR_DHCP_RANGE_FULL: u32 = 20012u32;
pub const ERROR_DHCP_RANGE_INVALID_IN_SERVER_POLICY: u32 = 20108u32;
pub const ERROR_DHCP_RANGE_TOO_SMALL: u32 = 20020u32;
pub const ERROR_DHCP_REACHED_END_OF_SELECTION: u32 = 20126u32;
pub const ERROR_DHCP_REGISTRY_INIT_FAILED: u32 = 20000u32;
pub const ERROR_DHCP_RESERVEDIP_EXITS: u32 = 20022u32;
pub const ERROR_DHCP_RESERVED_CLIENT: u32 = 20019u32;
pub const ERROR_DHCP_ROGUE_DS_CONFLICT: u32 = 20041u32;
pub const ERROR_DHCP_ROGUE_DS_UNREACHABLE: u32 = 20040u32;
pub const ERROR_DHCP_ROGUE_INIT_FAILED: u32 = 20037u32;
pub const ERROR_DHCP_ROGUE_NOT_AUTHORIZED: u32 = 20039u32;
pub const ERROR_DHCP_ROGUE_NOT_OUR_ENTERPRISE: u32 = 20042u32;
pub const ERROR_DHCP_ROGUE_SAMSHUTDOWN: u32 = 20038u32;
pub const ERROR_DHCP_ROGUE_STANDALONE_IN_DS: u32 = 20043u32;
pub const ERROR_DHCP_RPC_INIT_FAILED: u32 = 20002u32;
pub const ERROR_DHCP_SCOPE_NAME_TOO_LONG: u32 = 20046u32;
pub const ERROR_DHCP_SERVER_NAME_NOT_RESOLVED: u32 = 20124u32;
pub const ERROR_DHCP_SERVER_NOT_REACHABLE: u32 = 20122u32;
pub const ERROR_DHCP_SERVER_NOT_RUNNING: u32 = 20123u32;
pub const ERROR_DHCP_SERVICE_PAUSED: u32 = 20017u32;
pub const ERROR_DHCP_SUBNET_EXISTS: u32 = 20052u32;
pub const ERROR_DHCP_SUBNET_EXITS: u32 = 20004u32;
pub const ERROR_DHCP_SUBNET_NOT_PRESENT: u32 = 20005u32;
pub const ERROR_DHCP_SUPER_SCOPE_NAME_TOO_LONG: u32 = 20030u32;
pub const ERROR_DHCP_UNDEFINED_HARDWARE_ADDRESS_TYPE: u32 = 20102u32;
pub const ERROR_DHCP_UNSUPPORTED_CLIENT: u32 = 20034u32;
pub const ERROR_EXTEND_TOO_SMALL: u32 = 20025u32;
pub const ERROR_LAST_DHCP_SERVER_ERROR: u32 = 20139u32;
pub const ERROR_MSCOPE_RANGE_TOO_SMALL: u32 = 20054u32;
pub const ERROR_SCOPE_RANGE_POLICY_RANGE_CONFLICT: u32 = 20112u32;
pub const ERROR_SERVER_INVALID_BOOT_FILE_TABLE: u32 = 20028u32;
pub const ERROR_SERVER_UNKNOWN_BOOT_FILE_NAME: u32 = 20029u32;
pub const FILTER_STATUS_FULL_MATCH_IN_ALLOW_LIST: u32 = 2u32;
pub const FILTER_STATUS_FULL_MATCH_IN_DENY_LIST: u32 = 4u32;
pub const FILTER_STATUS_NONE: u32 = 1u32;
pub const FILTER_STATUS_WILDCARD_MATCH_IN_ALLOW_LIST: u32 = 8u32;
pub const FILTER_STATUS_WILDCARD_MATCH_IN_DENY_LIST: u32 = 16u32;
#[repr(transparent)]
pub struct FSM_STATE(pub i32);
pub const NO_STATE: FSM_STATE = FSM_STATE(0i32);
pub const INIT: FSM_STATE = FSM_STATE(1i32);
pub const STARTUP: FSM_STATE = FSM_STATE(2i32);
pub const NORMAL: FSM_STATE = FSM_STATE(3i32);
pub const COMMUNICATION_INT: FSM_STATE = FSM_STATE(4i32);
pub const PARTNER_DOWN: FSM_STATE = FSM_STATE(5i32);
pub const POTENTIAL_CONFLICT: FSM_STATE = FSM_STATE(6i32);
pub const CONFLICT_DONE: FSM_STATE = FSM_STATE(7i32);
pub const RESOLUTION_INT: FSM_STATE = FSM_STATE(8i32);
pub const RECOVER: FSM_STATE = FSM_STATE(9i32);
pub const RECOVER_WAIT: FSM_STATE = FSM_STATE(10i32);
pub const RECOVER_DONE: FSM_STATE = FSM_STATE(11i32);
pub const PAUSED: FSM_STATE = FSM_STATE(12i32);
pub const SHUTDOWN: FSM_STATE = FSM_STATE(13i32);
pub const HWTYPE_ETHERNET_10MB: u32 = 1u32;
pub type LPDHCP_CONTROL = unsafe extern "system" fn(dwcontrolcode: u32, lpreserved: *mut ::core::ffi::c_void) -> u32;
pub type LPDHCP_DELETE_CLIENT = unsafe extern "system" fn(ipaddress: u32, hwaddress: *mut u8, hwaddresslength: u32, reserved: u32, clienttype: u32) -> u32;
pub type LPDHCP_DROP_SEND = unsafe extern "system" fn(packet: *mut *mut u8, packetsize: *mut u32, controlcode: u32, ipaddress: u32, reserved: *mut ::core::ffi::c_void, pktcontext: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPDHCP_ENTRY_POINT_FUNC = unsafe extern "system" fn(chaindlls: super::super::Foundation::PWSTR, calloutversion: u32, callouttbl: *mut DHCP_CALLOUT_TABLE) -> u32;
pub type LPDHCP_GIVE_ADDRESS = unsafe extern "system" fn(packet: *mut u8, packetsize: u32, controlcode: u32, ipaddress: u32, altaddress: u32, addrtype: u32, leasetime: u32, reserved: *mut ::core::ffi::c_void, pktcontext: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPDHCP_HANDLE_OPTIONS = unsafe extern "system" fn(packet: *mut u8, packetsize: u32, reserved: *mut ::core::ffi::c_void, pktcontext: *mut ::core::ffi::c_void, serveroptions: *mut DHCP_SERVER_OPTIONS) -> u32;
pub type LPDHCP_NEWPKT = unsafe extern "system" fn(packet: *mut *mut u8, packetsize: *mut u32, ipaddress: u32, reserved: *mut ::core::ffi::c_void, pktcontext: *mut *mut ::core::ffi::c_void, processit: *mut i32) -> u32;
pub type LPDHCP_PROB = unsafe extern "system" fn(packet: *mut u8, packetsize: u32, controlcode: u32, ipaddress: u32, altaddress: u32, reserved: *mut ::core::ffi::c_void, pktcontext: *mut ::core::ffi::c_void) -> u32;
pub const MAC_ADDRESS_LENGTH: u32 = 6u32;
pub const MAX_PATTERN_LENGTH: u32 = 255u32;
pub const MCLT: u32 = 1u32;
pub const MODE: u32 = 16u32;
pub const OPTION_ALL_SUBNETS_MTU: u32 = 27u32;
pub const OPTION_ARP_CACHE_TIMEOUT: u32 = 35u32;
pub const OPTION_BE_A_MASK_SUPPLIER: u32 = 30u32;
pub const OPTION_BE_A_ROUTER: u32 = 19u32;
pub const OPTION_BOOTFILE_NAME: u32 = 67u32;
pub const OPTION_BOOT_FILE_SIZE: u32 = 13u32;
pub const OPTION_BROADCAST_ADDRESS: u32 = 28u32;
pub const OPTION_CLIENT_CLASS_INFO: u32 = 60u32;
pub const OPTION_CLIENT_ID: u32 = 61u32;
pub const OPTION_COOKIE_SERVERS: u32 = 8u32;
pub const OPTION_DEFAULT_TTL: u32 = 23u32;
pub const OPTION_DOMAIN_NAME: u32 = 15u32;
pub const OPTION_DOMAIN_NAME_SERVERS: u32 = 6u32;
pub const OPTION_END: u32 = 255u32;
pub const OPTION_ETHERNET_ENCAPSULATION: u32 = 36u32;
pub const OPTION_EXTENSIONS_PATH: u32 = 18u32;
pub const OPTION_HOST_NAME: u32 = 12u32;
pub const OPTION_IEN116_NAME_SERVERS: u32 = 5u32;
pub const OPTION_IMPRESS_SERVERS: u32 = 10u32;
pub const OPTION_KEEP_ALIVE_DATA_SIZE: u32 = 39u32;
pub const OPTION_KEEP_ALIVE_INTERVAL: u32 = 38u32;
pub const OPTION_LEASE_TIME: u32 = 51u32;
pub const OPTION_LOG_SERVERS: u32 = 7u32;
pub const OPTION_LPR_SERVERS: u32 = 9u32;
pub const OPTION_MAX_REASSEMBLY_SIZE: u32 = 22u32;
pub const OPTION_MERIT_DUMP_FILE: u32 = 14u32;
pub const OPTION_MESSAGE: u32 = 56u32;
pub const OPTION_MESSAGE_LENGTH: u32 = 57u32;
pub const OPTION_MESSAGE_TYPE: u32 = 53u32;
pub const OPTION_MSFT_IE_PROXY: u32 = 252u32;
pub const OPTION_MTU: u32 = 26u32;
pub const OPTION_NETBIOS_DATAGRAM_SERVER: u32 = 45u32;
pub const OPTION_NETBIOS_NAME_SERVER: u32 = 44u32;
pub const OPTION_NETBIOS_NODE_TYPE: u32 = 46u32;
pub const OPTION_NETBIOS_SCOPE_OPTION: u32 = 47u32;
pub const OPTION_NETWORK_INFO_SERVERS: u32 = 41u32;
pub const OPTION_NETWORK_INFO_SERVICE_DOM: u32 = 40u32;
pub const OPTION_NETWORK_TIME_SERVERS: u32 = 42u32;
pub const OPTION_NON_LOCAL_SOURCE_ROUTING: u32 = 20u32;
pub const OPTION_OK_TO_OVERLAY: u32 = 52u32;
pub const OPTION_PAD: u32 = 0u32;
pub const OPTION_PARAMETER_REQUEST_LIST: u32 = 55u32;
pub const OPTION_PERFORM_MASK_DISCOVERY: u32 = 29u32;
pub const OPTION_PERFORM_ROUTER_DISCOVERY: u32 = 31u32;
pub const OPTION_PMTU_AGING_TIMEOUT: u32 = 24u32;
pub const OPTION_PMTU_PLATEAU_TABLE: u32 = 25u32;
pub const OPTION_POLICY_FILTER_FOR_NLSR: u32 = 21u32;
pub const OPTION_REBIND_TIME: u32 = 59u32;
pub const OPTION_RENEWAL_TIME: u32 = 58u32;
pub const OPTION_REQUESTED_ADDRESS: u32 = 50u32;
pub const OPTION_RLP_SERVERS: u32 = 11u32;
pub const OPTION_ROOT_DISK: u32 = 17u32;
pub const OPTION_ROUTER_ADDRESS: u32 = 3u32;
pub const OPTION_ROUTER_SOLICITATION_ADDR: u32 = 32u32;
pub const OPTION_SERVER_IDENTIFIER: u32 = 54u32;
pub const OPTION_STATIC_ROUTES: u32 = 33u32;
pub const OPTION_SUBNET_MASK: u32 = 1u32;
pub const OPTION_SWAP_SERVER: u32 = 16u32;
pub const OPTION_TFTP_SERVER_NAME: u32 = 66u32;
pub const OPTION_TIME_OFFSET: u32 = 2u32;
pub const OPTION_TIME_SERVERS: u32 = 4u32;
pub const OPTION_TRAILERS: u32 = 34u32;
pub const OPTION_TTL: u32 = 37u32;
pub const OPTION_VENDOR_SPEC_INFO: u32 = 43u32;
pub const OPTION_XWINDOW_DISPLAY_MANAGER: u32 = 49u32;
pub const OPTION_XWINDOW_FONT_SERVER: u32 = 48u32;
pub const PERCENTAGE: u32 = 8u32;
pub const PREVSTATE: u32 = 32u32;
pub const QUARANTINE_CONFIG_OPTION: u32 = 43222u32;
pub const QUARANTINE_SCOPE_QUARPROFILE_OPTION: u32 = 43221u32;
pub const QUARANTIN_OPTION_BASE: u32 = 43220u32;
#[repr(transparent)]
pub struct QuarantineStatus(pub i32);
pub const NOQUARANTINE: QuarantineStatus = QuarantineStatus(0i32);
pub const RESTRICTEDACCESS: QuarantineStatus = QuarantineStatus(1i32);
pub const DROPPACKET: QuarantineStatus = QuarantineStatus(2i32);
pub const PROBATION: QuarantineStatus = QuarantineStatus(3i32);
pub const EXEMPT: QuarantineStatus = QuarantineStatus(4i32);
pub const DEFAULTQUARSETTING: QuarantineStatus = QuarantineStatus(5i32);
pub const NOQUARINFO: QuarantineStatus = QuarantineStatus(6i32);
pub const SAFEPERIOD: u32 = 2u32;
#[repr(C)]
pub struct SCOPE_MIB_INFO(i32);
#[repr(C)]
pub struct SCOPE_MIB_INFO_V5(i32);
#[repr(C)]
pub struct SCOPE_MIB_INFO_V6(i32);
#[repr(C)]
pub struct SCOPE_MIB_INFO_VQ(i32);
pub const SHAREDSECRET: u32 = 64u32;
pub const Set_APIProtocolSupport: u32 = 1u32;
pub const Set_AuditLogState: u32 = 2048u32;
pub const Set_BackupInterval: u32 = 16u32;
pub const Set_BackupPath: u32 = 8u32;
pub const Set_BootFileTable: u32 = 1024u32;
pub const Set_DatabaseCleanupInterval: u32 = 128u32;
pub const Set_DatabaseLoggingFlag: u32 = 32u32;
pub const Set_DatabaseName: u32 = 2u32;
pub const Set_DatabasePath: u32 = 4u32;
pub const Set_DebugFlag: u32 = 256u32;
pub const Set_PingRetries: u32 = 512u32;
pub const Set_PreferredLifetime: u32 = 4u32;
pub const Set_PreferredLifetimeIATA: u32 = 64u32;
pub const Set_QuarantineDefFail: u32 = 8192u32;
pub const Set_QuarantineON: u32 = 4096u32;
pub const Set_RapidCommitFlag: u32 = 2u32;
pub const Set_RestoreFlag: u32 = 64u32;
pub const Set_T1: u32 = 16u32;
pub const Set_T2: u32 = 32u32;
pub const Set_UnicastFlag: u32 = 1u32;
pub const Set_ValidLifetime: u32 = 8u32;
pub const Set_ValidLifetimeIATA: u32 = 128u32;
#[repr(transparent)]
pub struct StatusCode(pub i32);
pub const STATUS_NO_ERROR: StatusCode = StatusCode(0i32);
pub const STATUS_UNSPECIFIED_FAILURE: StatusCode = StatusCode(1i32);
pub const STATUS_NO_BINDING: StatusCode = StatusCode(3i32);
pub const STATUS_NOPREFIX_AVAIL: StatusCode = StatusCode(6i32);
pub const V5_ADDRESS_BIT_BOTH_REC: u32 = 32u32;
pub const V5_ADDRESS_BIT_DELETED: u32 = 128u32;
pub const V5_ADDRESS_BIT_UNREGISTERED: u32 = 64u32;
pub const V5_ADDRESS_EX_BIT_DISABLE_PTR_RR: u32 = 1u32;
pub const V5_ADDRESS_STATE_ACTIVE: u32 = 1u32;
pub const V5_ADDRESS_STATE_DECLINED: u32 = 2u32;
pub const V5_ADDRESS_STATE_DOOM: u32 = 3u32;
pub const V5_ADDRESS_STATE_OFFERED: u32 = 0u32;
pub const WARNING_EXTENDED_LESS: i32 = 20026i32;
