#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn AcceptEx();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn EnumProtocolsA();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn EnumProtocolsW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeAddrInfoEx();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeAddrInfoExW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeAddrInfoW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAcceptExSockaddrs();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn GetAddrInfoExA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAddrInfoExCancel();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn GetAddrInfoExOverlappedResult();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn GetAddrInfoExW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAddrInfoW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAddressByNameA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAddressByNameW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetHostNameW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNameByTypeA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNameByTypeW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNameInfoW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetServiceA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetServiceW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTypeByNameA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTypeByNameW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InetNtopW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InetPtonW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ProcessSocketNotifications();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_WindowsFilteringPlatform`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WindowsFilteringPlatform"))]
    pub fn RtlEthernetAddressToStringA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_WindowsFilteringPlatform`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WindowsFilteringPlatform"))]
    pub fn RtlEthernetAddressToStringW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_WindowsFilteringPlatform`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WindowsFilteringPlatform"))]
    pub fn RtlEthernetStringToAddressA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_WindowsFilteringPlatform`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WindowsFilteringPlatform"))]
    pub fn RtlEthernetStringToAddressW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4AddressToStringA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4AddressToStringExA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4AddressToStringExW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4AddressToStringW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4StringToAddressA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4StringToAddressExA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4StringToAddressExW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4StringToAddressW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6AddressToStringA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6AddressToStringExA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6AddressToStringExW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6AddressToStringW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6StringToAddressA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6StringToAddressExA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6StringToAddressExW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6StringToAddressW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
    pub fn SetAddrInfoExA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
    pub fn SetAddrInfoExW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SetServiceA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SetServiceW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSocketMediaStreamingMode();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn TransmitFile();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WPUCompleteOverlappedRequest();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_QoS`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
    pub fn WSAAccept();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAddressToStringA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAddressToStringW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSAAdvertiseProvider();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetHostByAddr();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetHostByName();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetProtoByName();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetProtoByNumber();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetServByName();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetServByPort();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncSelect();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSACancelAsyncRequest();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSACancelBlockingCall();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSACleanup();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSACloseEvent();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_QoS`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
    pub fn WSAConnect();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAConnectByList();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAConnectByNameA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAConnectByNameW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSACreateEvent();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSADeleteSocketPeerTargetName();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSADuplicateSocketA();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSADuplicateSocketW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAEnumNameSpaceProvidersA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSAEnumNameSpaceProvidersExA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSAEnumNameSpaceProvidersExW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAEnumNameSpaceProvidersW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAEnumNetworkEvents();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAEnumProtocolsA();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSAEnumProtocolsW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAEventSelect();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSAGetLastError();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAGetOverlappedResult();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_QoS`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
    pub fn WSAGetQOSByName();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAGetServiceClassInfoA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAGetServiceClassInfoW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAGetServiceClassNameByClassIdA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAGetServiceClassNameByClassIdW();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSAHtonl();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSAHtons();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAImpersonateSocketPeer();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAInstallServiceClassA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAInstallServiceClassW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAIoctl();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAIsBlocking();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_QoS`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
    pub fn WSAJoinLeaf();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSALookupServiceBeginA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSALookupServiceBeginW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSALookupServiceEnd();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSALookupServiceNextA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSALookupServiceNextW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSANSPIoctl();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSANtohl();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSANtohs();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSAPoll();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAProviderCompleteAsyncCall();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAProviderConfigChange();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAQuerySocketSecurity();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSARecv();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSARecvDisconnect();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSARecvEx();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSARecvFrom();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSARemoveServiceClass();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAResetEvent();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSARevertImpersonation();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSASend();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSASendDisconnect();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSASendMsg();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSASendTo();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSASetBlockingHook();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSASetEvent();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSASetLastError();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSASetServiceA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSASetServiceW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSASetSocketPeerTargetName();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSASetSocketSecurity();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSASocketA();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSASocketW();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAStartup();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAStringToAddressA();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAStringToAddressW();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSAUnadvertiseProvider();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSAUnhookBlockingHook();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAWaitForMultipleEvents();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSCDeinstallProvider();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCDeinstallProvider32();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCEnableNSProvider();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCEnableNSProvider32();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCEnumNameSpaceProviders32();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSCEnumNameSpaceProvidersEx32();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSCEnumProtocols();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCEnumProtocols32();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCGetApplicationCategory();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSCGetProviderInfo();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCGetProviderInfo32();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCGetProviderPath();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCGetProviderPath32();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCInstallNameSpace();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCInstallNameSpace32();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSCInstallNameSpaceEx();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSCInstallNameSpaceEx32();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCInstallProvider();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCInstallProvider64_32();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCInstallProviderAndChains64_32();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCSetApplicationCategory();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSCSetProviderInfo();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCSetProviderInfo32();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSCUnInstallNameSpace();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCUnInstallNameSpace32();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCUpdateProvider();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCUpdateProvider32();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSCWriteNameSpaceOrder();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCWriteNameSpaceOrder32();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSCWriteProviderOrder();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCWriteProviderOrder32();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn __WSAFDIsSet();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn accept();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn bind();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn closesocket();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn connect();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn freeaddrinfo();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getaddrinfo();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn gethostbyaddr();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn gethostbyname();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn gethostname();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getnameinfo();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getpeername();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getprotobyname();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getprotobynumber();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getservbyname();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getservbyport();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getsockname();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getsockopt();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn htonl();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn htons();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn inet_addr();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn inet_ntoa();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn inet_ntop();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn inet_pton();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn ioctlsocket();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn listen();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn ntohl();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn ntohs();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn recv();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn recvfrom();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn select();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn send();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sendto();
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn setsockopt();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn shutdown();
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn socket();
}
