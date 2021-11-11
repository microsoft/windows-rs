#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmCalloutAdd0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutCreateEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutDeleteById0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutDeleteByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutDestroyEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutEnum0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutGetById0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutGetByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmCalloutGetSecurityInfoByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmCalloutSetSecurityInfoByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutSubscribeChanges0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutSubscriptionsGet0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmCalloutUnsubscribeChanges0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmConnectionCreateEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmConnectionDestroyEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmConnectionEnum0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmConnectionGetById0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmConnectionGetSecurityInfo0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmConnectionSetSecurityInfo0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmConnectionSubscribe0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmConnectionUnsubscribe0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmDynamicKeywordSubscribe0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmDynamicKeywordUnsubscribe0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmEngineClose0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmEngineGetOption0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmEngineGetSecurityInfo0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Rpc`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Rpc"))]
    pub fn FwpmEngineOpen0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmEngineSetOption0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmEngineSetSecurityInfo0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterAdd0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterCreateEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmFilterDeleteById0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmFilterDeleteByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmFilterDestroyEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterEnum0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterGetById0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterGetByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterGetSecurityInfoByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterSetSecurityInfoByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterSubscribeChanges0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmFilterSubscriptionsGet0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmFilterUnsubscribeChanges0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`*"]
    pub fn FwpmFreeMemory0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmGetAppIdFromFileName0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmIPsecTunnelAdd0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmIPsecTunnelAdd1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmIPsecTunnelAdd2();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmIPsecTunnelAdd3();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmIPsecTunnelDeleteByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmLayerCreateEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmLayerDestroyEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmLayerEnum0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmLayerGetById0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmLayerGetByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmLayerGetSecurityInfoByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmLayerSetSecurityInfoByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventCreateEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmNetEventDestroyEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventEnum0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventEnum1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventEnum2();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventEnum3();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventEnum4();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventEnum5();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventSubscribe0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventSubscribe1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventSubscribe2();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventSubscribe3();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventSubscribe4();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventSubscriptionsGet0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmNetEventUnsubscribe0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventsGetSecurityInfo0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmNetEventsSetSecurityInfo0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderAdd0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextAdd0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextAdd1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextAdd2();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextAdd3();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderContextCreateEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderContextDeleteById0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderContextDeleteByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderContextDestroyEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextEnum0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextEnum1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextEnum2();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextEnum3();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetById0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetById1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetById2();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetById3();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetByKey1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetByKey2();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetByKey3();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextGetSecurityInfoByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderContextSetSecurityInfoByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderContextSubscribeChanges0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderContextSubscriptionsGet0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderContextUnsubscribeChanges0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderCreateEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderDeleteByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderDestroyEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderEnum0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderGetByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderGetSecurityInfoByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmProviderSetSecurityInfoByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderSubscribeChanges0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderSubscriptionsGet0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmProviderUnsubscribeChanges0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSessionCreateEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSessionDestroyEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmSessionEnum0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmSubLayerAdd0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSubLayerCreateEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSubLayerDeleteByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSubLayerDestroyEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSubLayerEnum0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSubLayerGetByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmSubLayerGetSecurityInfoByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmSubLayerSetSecurityInfoByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSubLayerSubscribeChanges0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSubLayerSubscriptionsGet0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSubLayerUnsubscribeChanges0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSystemPortsGet0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSystemPortsSubscribe0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmSystemPortsUnsubscribe0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmTransactionAbort0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmTransactionBegin0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmTransactionCommit0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmvSwitchEventSubscribe0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FwpmvSwitchEventUnsubscribe0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmvSwitchEventsGetSecurityInfo0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FwpmvSwitchEventsSetSecurityInfo0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecDospGetSecurityInfo0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecDospGetStatistics0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecDospSetSecurityInfo0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecDospStateCreateEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecDospStateDestroyEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecDospStateEnum0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecGetStatistics0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecGetStatistics1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecKeyManagerAddAndRegister0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecKeyManagerGetSecurityInfoByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecKeyManagerSetSecurityInfoByKey0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecKeyManagerUnregisterAndDelete0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecKeyManagersGet0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextAddInbound0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextAddInbound1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextAddOutbound0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextAddOutbound1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextCreate0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextCreate1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaContextCreateEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextDeleteById0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextDestroyEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaContextEnum0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaContextEnum1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextExpire0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaContextGetById0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaContextGetById1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextGetSpi0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextGetSpi1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextSetSpi0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaContextSubscribe0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaContextSubscriptionsGet0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaContextUnsubscribe0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaContextUpdate0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaCreateEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaDbGetSecurityInfo0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaDbSetSecurityInfo0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IPsecSaDestroyEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaEnum0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IPsecSaEnum1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextGetStatistics0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextGetStatistics1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IkeextSaCreateEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IkeextSaDbGetSecurityInfo0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn IkeextSaDbSetSecurityInfo0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextSaDeleteById0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextSaDestroyEnumHandle0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextSaEnum0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextSaEnum1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextSaEnum2();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextSaGetById0();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextSaGetById1();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFilteringPlatform`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IkeextSaGetById2();
}
