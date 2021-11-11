#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn AddIPAddress();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CancelIPChangeNotify();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelMibChangeNotify2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn CaptureInterfaceHardwareCrossTimestamp();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertCompartmentGuidToId();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertCompartmentIdToGuid();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceAliasToLuid();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceGuidToLuid();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceIndexToLuid();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToAlias();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToGuid();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToIndex();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToNameA();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToNameW();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceNameToLuidA();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceNameToLuidW();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertIpv4MaskToLength();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertLengthToIpv4Mask();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn CreateAnycastIpAddressEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn CreateIpForwardEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn CreateIpForwardEntry2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn CreateIpNetEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn CreateIpNetEntry2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn CreatePersistentTcpPortReservation();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn CreatePersistentUdpPortReservation();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn CreateProxyArpEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn CreateSortedAddressPairs();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn CreateUnicastIpAddressEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DeleteAnycastIpAddressEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn DeleteIPAddress();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn DeleteIpForwardEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DeleteIpForwardEntry2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn DeleteIpNetEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DeleteIpNetEntry2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn DeletePersistentTcpPortReservation();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn DeletePersistentUdpPortReservation();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn DeleteProxyArpEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DeleteUnicastIpAddressEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn DisableMediaSense();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn EnableRouter();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn FlushIpNetTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushIpNetTable2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushIpPathTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeDnsSettings();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeInterfaceDnsSettings();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn FreeMibTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAdapterIndex();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetAdapterOrderMap();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetAdaptersAddresses();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAdaptersInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetAnycastIpAddressEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetAnycastIpAddressTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetBestInterface();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetBestInterfaceEx();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn GetBestRoute();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetBestRoute2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetCurrentThreadCompartmentId();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetCurrentThreadCompartmentScope();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetDefaultCompartmentId();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDnsSettings();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExtendedTcpTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExtendedUdpTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetFriendlyIfIndex();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetIcmpStatistics();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetIcmpStatisticsEx();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetIfEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_NetworkManagement_Ndis`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn GetIfEntry2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_NetworkManagement_Ndis`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn GetIfEntry2Ex();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIfStackTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIfTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_NetworkManagement_Ndis`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn GetIfTable2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_NetworkManagement_Ndis`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn GetIfTable2Ex();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInterfaceActiveTimestampCapabilities();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInterfaceDnsSettings();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetInterfaceInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInterfaceSupportedTimestampCapabilities();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInvertedIfStackTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIpAddrTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIpErrorString();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpForwardEntry2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpForwardTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpForwardTable2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpInterfaceEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpInterfaceTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpNetEntry2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIpNetTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpNetTable2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpNetworkConnectionBandwidthEstimates();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpPathEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpPathTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetIpStatistics();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetIpStatisticsEx();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetJobCompartmentId();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetMulticastIpAddressEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetMulticastIpAddressTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetNetworkConnectivityHint();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetNetworkConnectivityHintForInterface();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNetworkInformation();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNetworkParams();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetNumberOfInterfaces();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetOwnerModuleFromPidAndInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetOwnerModuleFromTcp6Entry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetOwnerModuleFromTcpEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetOwnerModuleFromUdp6Entry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetOwnerModuleFromUdpEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPerAdapterInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn GetPerTcp6ConnectionEStats();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetPerTcpConnectionEStats();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRTTAndHopCount();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetSessionCompartmentId();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetTcp6Table();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetTcp6Table2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetTcpStatistics();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetTcpStatisticsEx();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetTcpStatisticsEx2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTcpTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTcpTable2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTeredoPort();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetUdp6Table();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetUdpStatistics();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetUdpStatisticsEx();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetUdpStatisticsEx2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUdpTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetUniDirectionalAdapterInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetUnicastIpAddressEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetUnicastIpAddressTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn Icmp6CreateFile();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn Icmp6ParseReplies();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_WindowsProgramming"))]
    pub fn Icmp6SendEcho2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IcmpCloseHandle();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn IcmpCreateFile();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn IcmpParseReplies();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn IcmpSendEcho();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn IcmpSendEcho2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn IcmpSendEcho2Ex();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn InitializeIpForwardEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn InitializeIpInterfaceEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn InitializeUnicastIpAddressEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn IpReleaseAddress();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn IpRenewAddress();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn LookupPersistentTcpPortReservation();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn LookupPersistentUdpPortReservation();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NhpAllocateAndGetInterfaceInfoFromStack();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn NotifyAddrChange();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyIpInterfaceChange();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyNetworkConnectivityHintChange();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn NotifyRouteChange();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyRouteChange2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyStableUnicastIpAddressTable();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NotifyTeredoPortChange();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyUnicastIpAddressChange();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfAddFiltersToInterface();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfAddGlobalFilterToInterface();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfBindInterfaceToIPAddress();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfBindInterfaceToIndex();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PfCreateInterface();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfDeleteInterface();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfDeleteLog();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PfGetInterfaceStatistics();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PfMakeLog();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfRebindFilters();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfRemoveFilterHandles();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfRemoveFiltersFromInterface();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfRemoveGlobalFilterFromInterface();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfSetLogBuffer();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfTestPacket();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfUnBindInterface();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn RegisterInterfaceTimestampConfigChange();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn ResolveIpNetEntry2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn ResolveNeighbor();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RestoreMediaSense();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn SendARP();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCurrentThreadCompartmentId();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCurrentThreadCompartmentScope();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDnsSettings();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn SetIfEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetInterfaceDnsSettings();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn SetIpForwardEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn SetIpForwardEntry2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn SetIpInterfaceEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn SetIpNetEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn SetIpNetEntry2();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn SetIpStatistics();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn SetIpStatisticsEx();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn SetIpTTL();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetJobCompartmentId();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetNetworkInformation();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn SetPerTcp6ConnectionEStats();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn SetPerTcpConnectionEStats();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSessionCompartmentId();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn SetTcpEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn SetUnicastIpAddressEntry();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn UnenableRouter();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn UnregisterInterfaceTimestampConfigChange();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn if_indextoname();
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn if_nametoindex();
}
