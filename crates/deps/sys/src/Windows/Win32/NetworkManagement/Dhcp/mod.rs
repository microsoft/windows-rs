#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddFilterV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddSecurityGroup();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddServer();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddSubnetElement();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddSubnetElementV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddSubnetElementV5();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAddSubnetElementV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAuditLogGetParams();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpAuditLogSetParams();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpCApiCleanup();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpCApiInitialize();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateClass();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateClassV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateClientInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateClientInfoV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateClientInfoVQ();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateOption();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateOptionV5();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateOptionV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateSubnet();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateSubnetV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpCreateSubnetVQ();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpDeRegisterParamChange();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteClass();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteClassV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteClientInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteClientInfoV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteFilterV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteServer();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteSubnet();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteSubnetV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpDeleteSuperScopeV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpDsCleanup();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpDsInit();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumClasses();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumClassesV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumFilterV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptionValues();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptionValuesV5();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptionValuesV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptions();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptionsV5();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumOptionsV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumServers();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClients();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClientsFilterStatusInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClientsV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClientsV5();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClientsV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetClientsVQ();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetElements();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetElementsV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetElementsV5();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetElementsV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnets();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpEnumSubnetsV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetAllOptionValues();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetAllOptionValuesV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetAllOptions();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetAllOptionsV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClassInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClientInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClientInfoV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClientInfoV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClientInfoVQ();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetClientOptions();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetFilterV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetMibInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetMibInfoV5();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetMibInfoV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionInfoV5();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionInfoV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionValue();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionValueV5();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOptionValueV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetOriginalSubnetMask();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetServerBindingInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetServerBindingInfoV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetServerSpecificStrings();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetSubnetDelayOffer();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetSubnetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetSubnetInfoV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetSubnetInfoVQ();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetSuperScopeInfoV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpGetThreadOptions();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpGetVersion();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprAddV4PolicyCondition();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprAddV4PolicyExpr();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprAddV4PolicyRange();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprCreateV4Policy();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprCreateV4PolicyEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFindV4DhcpProperty();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4DhcpProperty();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4DhcpPropertyArray();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4Policy();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4PolicyArray();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4PolicyEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprFreeV4PolicyExArray();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprIsV4PolicySingleUC();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprIsV4PolicyValid();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprIsV4PolicyWellFormed();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprModifyV4PolicyExpr();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpHlprResetV4PolicyExpr();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpModifyClass();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpModifyClassV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRegisterParamChange();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpRemoveDNSRegistrations();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOption();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOptionV5();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOptionV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOptionValue();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOptionValueV5();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveOptionValueV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveSubnetElement();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveSubnetElementV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveSubnetElementV5();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRemoveSubnetElementV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpRequestParams();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpRpcFreeMemory();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpScanDatabase();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerAuditlogParamsFree();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerBackupDatabase();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerGetConfig();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerGetConfigV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerGetConfigV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerGetConfigVQ();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerQueryAttribute();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerQueryAttributes();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerQueryDnsRegCredentials();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerRedoAuthorization();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerRestoreDatabase();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetConfig();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetConfigV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetConfigV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetConfigVQ();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetDnsRegCredentials();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpServerSetDnsRegCredentialsV5();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetClientInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetClientInfoV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetClientInfoV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetClientInfoVQ();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetFilterV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionInfoV5();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionInfoV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionValue();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionValueV5();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionValueV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionValues();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetOptionValuesV5();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetServerBindingInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetServerBindingInfoV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetSubnetDelayOffer();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetSubnetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetSubnetInfoV6();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetSubnetInfoVQ();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpSetSuperScopeV4();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn DhcpSetThreadOptions();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpUndoRequestParams();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4AddPolicyRange();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4CreateClientInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4CreateClientInfoEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4CreatePolicy();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4CreatePolicyEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4DeletePolicy();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4EnumPolicies();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4EnumPoliciesEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4EnumSubnetClients();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4EnumSubnetClientsEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4EnumSubnetReservations();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverAddScopeToRelationship();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverCreateRelationship();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverDeleteRelationship();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverDeleteScopeFromRelationship();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverEnumRelationship();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetAddressStatus();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetClientInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetRelationship();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetScopeRelationship();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetScopeStatistics();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverGetSystemTime();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverSetRelationship();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4FailoverTriggerAddrAllocation();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetAllOptionValues();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetClientInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetClientInfoEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetFreeIPAddress();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetOptionValue();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetPolicy();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4GetPolicyEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4QueryPolicyEnforcement();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4RemoveOptionValue();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4RemovePolicyRange();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4SetOptionValue();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4SetOptionValues();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4SetPolicy();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4SetPolicyEnforcement();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV4SetPolicyEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV6CreateClientInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV6GetFreeIPAddress();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV6GetStatelessStatistics();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV6GetStatelessStoreParams();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DhcpV6SetStatelessStoreParams();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn Dhcpv6CApiCleanup();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`*"]
    pub fn Dhcpv6CApiInitialize();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Dhcpv6ReleasePrefix();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Dhcpv6RenewPrefix();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Dhcpv6RequestParams();
    #[doc = "*Required features: `Win32_NetworkManagement_Dhcp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Dhcpv6RequestPrefix();
}
