#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtClose();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtContinueSearch();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn DrtCreateDerivedKey();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn DrtCreateDerivedKeySecurityProvider();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrtCreateDnsBootstrapResolver();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtCreateIpv6UdpTransport();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtCreateNullSecurityProvider();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrtCreatePnrpBootstrapResolver();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtDeleteDerivedKeySecurityProvider();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtDeleteDnsBootstrapResolver();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtDeleteIpv6UdpTransport();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtDeleteNullSecurityProvider();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtDeletePnrpBootstrapResolver();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtEndSearch();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DrtGetEventData();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtGetEventDataSize();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrtGetInstanceName();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtGetInstanceNameSize();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DrtGetSearchPath();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtGetSearchPathSize();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtGetSearchResult();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtGetSearchResultSize();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrtOpen();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtRegisterKey();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrtStartSearch();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtUnregisterKey();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtUpdateKey();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabAddContact();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabAsyncInviteContact();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabAsyncInviteEndpoint();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabCancelInvitation();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabCloseHandle();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabDeleteContact();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabDeleteEndpointData();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabDeleteObject();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabEnumApplicationRegistrationInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabEnumApplications();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabEnumContacts();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabEnumEndpoints();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabEnumObjects();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabEnumPeopleNearMe();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabExportContact();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabGetAppLaunchInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabGetApplicationRegistrationInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabGetContact();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabGetEndpointName();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabGetEventData();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabGetInvitationResponse();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabGetPresenceInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabGetSigninOptions();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabInviteContact();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabInviteEndpoint();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabParseContact();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabQueryContactData();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabRefreshEndpointData();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabRegisterApplication();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabRegisterEvent();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabSetEndpointName();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabSetObject();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabSetPresenceInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabShutdown();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabSignin();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabSignout();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabStartup();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabSubscribeEndpointData();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabUnregisterApplication();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabUnregisterEvent();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabUnsubscribeEndpointData();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabUpdateContact();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCreatePeerName();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientAddContentInformation();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientAddData();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientBlockRead();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientCancelAsyncOperation();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistClientCloseContent();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientCompleteContentInformation();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientFlushContent();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistClientGetInformationByHandle();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerDistClientOpenContent();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientStreamRead();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistGetOverlappedResult();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistGetStatus();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistGetStatusEx();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistRegisterForStatusChangeNotification();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistRegisterForStatusChangeNotificationEx();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistServerCancelAsyncOperation();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistServerCloseContentInformation();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistServerCloseStreamHandle();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerDistServerOpenContentInformation();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerDistServerOpenContentInformationEx();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistServerPublishAddToStream();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistServerPublishCompleteStream();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerDistServerPublishStream();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistServerRetrieveContentInformation();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistServerUnpublish();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistShutdown();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistStartup();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistUnregisterForStatusChangeNotification();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerEndEnumeration();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerEnumGroups();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerEnumIdentities();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerFreeData();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGetItemCount();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGetNextItem();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphAddRecord();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphClose();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphCloseDirectConnection();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerGraphConnect();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphCreate();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphDelete();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphDeleteRecord();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphEndEnumeration();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphEnumConnections();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphEnumNodes();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphEnumRecords();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphExportDatabase();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphFreeData();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphGetEventData();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphGetItemCount();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphGetNextItem();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerGraphGetNodeInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphGetProperties();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphGetRecord();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphGetStatus();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphImportDatabase();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphListen();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphOpen();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerGraphOpenDirectConnection();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphPeerTimeToUniversalTime();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphRegisterEvent();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphSearchRecords();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphSendData();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphSetNodeAttributes();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphSetPresence();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphSetProperties();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphShutdown();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphStartup();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphUniversalTimeToPeerTime();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphUnregisterEvent();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphUpdateRecord();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphValidateDeferredRecords();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupAddRecord();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupClose();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupCloseDirectConnection();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupConnect();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn PeerGroupConnectByAddress();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupCreate();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupCreateInvitation();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupCreatePasswordInvitation();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupDelete();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupDeleteRecord();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupEnumConnections();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupEnumMembers();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupEnumRecords();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupExportConfig();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupExportDatabase();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupGetEventData();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupGetProperties();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupGetRecord();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupGetStatus();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupImportConfig();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupImportDatabase();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn PeerGroupIssueCredentials();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupJoin();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupOpen();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerGroupOpenDirectConnection();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn PeerGroupParseInvitation();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupPasswordJoin();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupPeerTimeToUniversalTime();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupRegisterEvent();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupResumePasswordAuthentication();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupSearchRecords();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupSendData();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupSetProperties();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupShutdown();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupStartup();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupUniversalTimeToPeerTime();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupUnregisterEvent();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupUpdateRecord();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerHostNameToPeerName();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityCreate();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityDelete();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityExport();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityGetCryptKey();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityGetDefault();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityGetFriendlyName();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityGetXML();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityImport();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentitySetFriendlyName();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerNameToPeerHostName();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerPnrpEndResolve();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerPnrpGetCloudInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerPnrpGetEndpoint();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerPnrpRegister();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerPnrpResolve();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerPnrpShutdown();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerPnrpStartResolve();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerPnrpStartup();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerPnrpUnregister();
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerPnrpUpdateRegistration();
}
