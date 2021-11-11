#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_System_Ole`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub fn ADsBuildEnumerator();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ADsBuildVarArrayInt();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ADsBuildVarArrayStr();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsDecodeBinaryData();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsEncodeBinaryData();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ADsEnumerateNext();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_System_Ole`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub fn ADsFreeEnumerator();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsGetLastError();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsGetObject();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsOpenObject();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropCheckIfWritable();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ADsPropCreateNotifyObj();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropGetInitInfo();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropSendErrorMessage();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropSetHwnd();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropSetHwndWithTitle();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropShowErrorDialog();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsSetLastError();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdsFreeAdsValues();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn AdsTypeToPropVariant();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn AllocADsMem();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocADsStr();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn BinarySDToSecurityDescriptor();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsAddSidHistoryA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsAddSidHistoryW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsAddressToSiteNamesA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsAddressToSiteNamesExA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsAddressToSiteNamesExW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsAddressToSiteNamesW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindByInstanceA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindByInstanceW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindToISTGA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindToISTGW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithCredA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithCredW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithSpnA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithSpnExA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithSpnExW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithSpnW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindingSetTimeout();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_UI_Shell`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    pub fn DsBrowseForContainerA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_UI_Shell`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    pub fn DsBrowseForContainerW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsClientMakeSpnForTargetServerA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsClientMakeSpnForTargetServerW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackNamesA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackNamesW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpn2A();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpn2W();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpn3W();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpn4W();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpnA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpnW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackUnquotedMangledRdnA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackUnquotedMangledRdnW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsDeregisterDnsHostRecordsA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsDeregisterDnsHostRecordsW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsEnumerateDomainTrustsA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsEnumerateDomainTrustsW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn DsFreeDomainControllerInfoA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn DsFreeDomainControllerInfoW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeNameResultA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeNameResultW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn DsFreePasswordCredentials();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeSchemaGuidMapA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeSchemaGuidMapW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeSpnArrayA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeSpnArrayW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn DsGetDcCloseW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcNameA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcNameW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsGetDcNextA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsGetDcNextW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcOpenA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcOpenW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcSiteCoverageA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcSiteCoverageW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDomainControllerInfoA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDomainControllerInfoW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Security_Authentication_Identity`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn DsGetForestTrustInformationW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetFriendlyClassName();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn DsGetIcon();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetRdnW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetSiteNameA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetSiteNameW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetSpnA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetSpnW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsInheritSecurityIdentityA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsInheritSecurityIdentityW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsIsMangledDnA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsIsMangledDnW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsIsMangledRdnValueA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsIsMangledRdnValueW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListDomainsInSiteA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListDomainsInSiteW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListInfoForServerA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListInfoForServerW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListRolesA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListRolesW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListServersForDomainInSiteA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListServersForDomainInSiteW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListServersInSiteA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListServersInSiteW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListSitesA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListSitesW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMakePasswordCredentialsA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMakePasswordCredentialsW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMakeSpnA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMakeSpnW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMapSchemaGuidsA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMapSchemaGuidsW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Security_Authentication_Identity`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn DsMergeForestTrustInformationW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsQuerySitesByCostA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsQuerySitesByCostW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn DsQuerySitesFree();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsQuoteRdnValueA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsQuoteRdnValueW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsRemoveDsDomainA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsRemoveDsDomainW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsRemoveDsServerA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsRemoveDsServerW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaAddA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaAddW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaConsistencyCheck();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaDelA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaDelW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn DsReplicaFreeInfo();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaGetInfo2W();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaGetInfoW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaModifyA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaModifyW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaSyncA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaSyncAllA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaSyncAllW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaSyncW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaUpdateRefsA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaUpdateRefsW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaVerifyObjectsA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaVerifyObjectsW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn DsRoleFreeMemory();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsRoleGetPrimaryDomainInformation();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsServerRegisterSpnA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsServerRegisterSpnW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsUnBindA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsUnBindW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsUnquoteRdnValueA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsUnquoteRdnValueW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsValidateSubnetNameA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsValidateSubnetNameW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsWriteAccountSpnA();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsWriteAccountSpnW();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeADsMem();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeADsStr();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn PropVariantToAdsType();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn ReallocADsMem();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReallocADsStr();
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn SecurityDescriptorToBinarySD();
}
