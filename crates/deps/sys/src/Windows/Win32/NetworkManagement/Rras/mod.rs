#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmAddGroupMembershipEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmDeRegisterMProtocol();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmDeleteGroupMembershipEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MgmGetFirstMfe();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MgmGetFirstMfeStats();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_NetworkManagement_IpHelper`*"]
    #[cfg(feature = "Win32_NetworkManagement_IpHelper")]
    pub fn MgmGetMfe();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_NetworkManagement_IpHelper`*"]
    #[cfg(feature = "Win32_NetworkManagement_IpHelper")]
    pub fn MgmGetMfeStats();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_NetworkManagement_IpHelper`*"]
    #[cfg(feature = "Win32_NetworkManagement_IpHelper")]
    pub fn MgmGetNextMfe();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_NetworkManagement_IpHelper`*"]
    #[cfg(feature = "Win32_NetworkManagement_IpHelper")]
    pub fn MgmGetNextMfeStats();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MgmGetProtocolOnInterface();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmGroupEnumerationEnd();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmGroupEnumerationGetNext();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmGroupEnumerationStart();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmRegisterMProtocol();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmReleaseInterfaceOwnership();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmTakeInterfaceOwnership();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminBufferFree();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminConnectionClearStats();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminConnectionEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminConnectionEnumEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminConnectionGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminConnectionGetInfoEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminConnectionRemoveQuarantine();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminDeregisterConnectionNotification();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminDeviceEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminEstablishDomainRasServer();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminGetErrorString();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminGetPDCServer();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceConnect();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceCreate();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceDelete();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceDeviceGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceDeviceSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceDisconnect();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminInterfaceEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceGetCredentials();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceGetCredentialsEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
    pub fn MprAdminInterfaceGetCustomInfoEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceGetHandle();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceQueryUpdateResult();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceSetCredentials();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceSetCredentialsEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
    pub fn MprAdminInterfaceSetCustomInfoEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceTransportAdd();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceTransportGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceTransportRemove();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceTransportSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceUpdatePhonebookInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceUpdateRoutes();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminIsDomainRasServer();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminIsServiceInitialized();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminIsServiceRunning();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminMIBBufferFree();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminMIBEntryCreate();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminMIBEntryDelete();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminMIBEntryGet();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminMIBEntryGetFirst();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminMIBEntryGetNext();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminMIBEntrySet();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminMIBServerConnect();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminMIBServerDisconnect();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminPortClearStats();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminPortDisconnect();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminPortEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminPortGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminPortReset();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminRegisterConnectionNotification();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminSendUserMessage();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminServerConnect();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminServerDisconnect();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminServerGetCredentials();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminServerGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn MprAdminServerGetInfoEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminServerSetCredentials();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminServerSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn MprAdminServerSetInfoEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminTransportCreate();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminTransportGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminTransportSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminUpdateConnection();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminUserGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminUserSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprConfigBufferFree();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigFilterGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigFilterSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigGetFriendlyName();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigGetGuidName();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceCreate();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceDelete();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
    pub fn MprConfigInterfaceGetCustomInfoEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceGetHandle();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
    pub fn MprConfigInterfaceSetCustomInfoEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceTransportAdd();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceTransportEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceTransportGetHandle();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceTransportGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceTransportRemove();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceTransportSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigServerBackup();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigServerConnect();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigServerDisconnect();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigServerGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn MprConfigServerGetInfoEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprConfigServerInstall();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigServerRefresh();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigServerRestore();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprConfigServerSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn MprConfigServerSetInfoEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigTransportCreate();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigTransportDelete();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigTransportEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigTransportGetHandle();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigTransportGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigTransportSetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoBlockAdd();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoBlockFind();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoBlockQuerySize();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoBlockRemove();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoBlockSet();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoCreate();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoDelete();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoDuplicate();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoRemoveAll();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasClearConnectionStatistics();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasClearLinkStatistics();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasConnectionNotificationA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasConnectionNotificationW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasCreatePhonebookEntryA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasCreatePhonebookEntryW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasDeleteEntryA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasDeleteEntryW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasDeleteSubEntryA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasDeleteSubEntryW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasDialA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasDialDlgA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasDialDlgW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasDialW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEditPhonebookEntryA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEditPhonebookEntryW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEntryDlgA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEntryDlgW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEnumAutodialAddressesA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEnumAutodialAddressesW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEnumConnectionsA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEnumConnectionsW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEnumDevicesA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasEnumDevicesW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEnumEntriesA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEnumEntriesW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasFreeEapUserIdentityA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasFreeEapUserIdentityW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetAutodialAddressA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetAutodialAddressW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetAutodialEnableA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetAutodialEnableW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetAutodialParamA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetAutodialParamW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn RasGetConnectStatusA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn RasGetConnectStatusW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetConnectionStatistics();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetCountryInfoA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetCountryInfoW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetCredentialsA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetCredentialsW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetCustomAuthDataA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetCustomAuthDataW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetEapUserDataA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetEapUserDataW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetEapUserIdentityA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetEapUserIdentityW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetEntryDialParamsA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetEntryDialParamsW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn RasGetEntryPropertiesA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn RasGetEntryPropertiesW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetErrorStringA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetErrorStringW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetLinkStatistics();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetPCscf();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetProjectionInfoA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn RasGetProjectionInfoEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetProjectionInfoW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetSubEntryHandleA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetSubEntryHandleW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetSubEntryPropertiesA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetSubEntryPropertiesW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasHangUpA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasHangUpW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasInvokeEapUI();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasPhonebookDlgA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasPhonebookDlgW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasRenameEntryA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasRenameEntryW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetAutodialAddressA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetAutodialAddressW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetAutodialEnableA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetAutodialEnableW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasSetAutodialParamA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasSetAutodialParamW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetCredentialsA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetCredentialsW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetCustomAuthDataA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetCustomAuthDataW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetEapUserDataA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetEapUserDataW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetEntryDialParamsA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetEntryDialParamsW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn RasSetEntryPropertiesA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn RasSetEntryPropertiesW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetSubEntryPropertiesA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetSubEntryPropertiesW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn RasUpdateConnection();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasValidateEntryNameA();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasValidateEntryNameW();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmAddNextHop();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmAddRouteToDest();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmBlockMethods();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn RtmConvertIpv6AddressAndLengthToNetAddress();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn RtmConvertNetAddressToIpv6AddressAndLength();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmCreateDestEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmCreateNextHopEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmCreateRouteEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmCreateRouteList();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmCreateRouteListEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmDeleteEnumHandle();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmDeleteNextHop();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmDeleteRouteList();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmDeleteRouteToDest();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmDeregisterEntity();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmDeregisterFromChangeNotification();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmFindNextHop();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmGetChangeStatus();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmGetChangedDests();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmGetDestInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetEntityInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetEntityMethods();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmGetEnumDests();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetEnumNextHops();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetEnumRoutes();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmGetExactMatchDestination();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetExactMatchRoute();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmGetLessSpecificDestination();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetListEnumRoutes();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmGetMostSpecificDestination();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetNextHopInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetNextHopPointer();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetOpaqueInformationPointer();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetRegisteredEntities();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetRouteInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetRoutePointer();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmHoldDestination();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmIgnoreChangedDests();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmInsertInRouteList();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmInvokeMethod();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmIsBestRoute();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmIsMarkedForChangeNotification();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmLockDestination();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmLockNextHop();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmLockRoute();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmMarkDestForChangeNotification();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmReferenceHandles();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmRegisterEntity();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmRegisterForChangeNotification();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmReleaseChangedDests();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmReleaseDestInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmReleaseDests();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmReleaseEntities();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmReleaseEntityInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmReleaseNextHopInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmReleaseNextHops();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmReleaseRouteInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmReleaseRoutes();
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmUpdateAndUnlockRoute();
}
