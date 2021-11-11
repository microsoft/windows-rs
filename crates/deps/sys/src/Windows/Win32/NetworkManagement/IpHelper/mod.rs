#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn AddIPAddress(address: u32, ipmask: u32, ifindex: u32, ntecontext: *mut u32, nteinstance: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CancelIPChangeNotify(notifyoverlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelMibChangeNotify2(notificationhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn CaptureInterfaceHardwareCrossTimestamp(interfaceluid: *const NET_LUID_LH, crosstimestamp: *mut INTERFACE_HARDWARE_CROSSTIMESTAMP) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertCompartmentGuidToId(compartmentguid: *const ::windows::runtime::GUID, compartmentid: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertCompartmentIdToGuid(compartmentid: u32, compartmentguid: *mut ::windows::runtime::GUID) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceAliasToLuid(interfacealias: super::super::Foundation::PWSTR, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceGuidToLuid(interfaceguid: *const ::windows::runtime::GUID, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceIndexToLuid(interfaceindex: u32, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToAlias(interfaceluid: *const NET_LUID_LH, interfacealias: super::super::Foundation::PWSTR, length: usize) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToGuid(interfaceluid: *const NET_LUID_LH, interfaceguid: *mut ::windows::runtime::GUID) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToIndex(interfaceluid: *const NET_LUID_LH, interfaceindex: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToNameA(interfaceluid: *const NET_LUID_LH, interfacename: super::super::Foundation::PSTR, length: usize) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToNameW(interfaceluid: *const NET_LUID_LH, interfacename: super::super::Foundation::PWSTR, length: usize) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceNameToLuidA(interfacename: super::super::Foundation::PSTR, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceNameToLuidW(interfacename: super::super::Foundation::PWSTR, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertIpv4MaskToLength(mask: u32, masklength: *mut u8) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertLengthToIpv4Mask(masklength: u32, mask: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn CreateAnycastIpAddressEntry(row: *const MIB_ANYCASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn CreateIpForwardEntry(proute: *const MIB_IPFORWARDROW) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn CreateIpForwardEntry2(row: *const MIB_IPFORWARD_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn CreateIpNetEntry(parpentry: *const MIB_IPNETROW_LH) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn CreateIpNetEntry2(row: *const MIB_IPNET_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn CreatePersistentTcpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn CreatePersistentUdpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn CreateProxyArpEntry(dwaddress: u32, dwmask: u32, dwifindex: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn CreateSortedAddressPairs(sourceaddresslist: *const super::super::Networking::WinSock::SOCKADDR_IN6, sourceaddresscount: u32, destinationaddresslist: *const super::super::Networking::WinSock::SOCKADDR_IN6, destinationaddresscount: u32, addresssortoptions: u32, sortedaddresspairlist: *mut *mut super::super::Networking::WinSock::SOCKADDR_IN6_PAIR, sortedaddresspaircount: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn CreateUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DeleteAnycastIpAddressEntry(row: *const MIB_ANYCASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn DeleteIPAddress(ntecontext: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn DeleteIpForwardEntry(proute: *const MIB_IPFORWARDROW) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DeleteIpForwardEntry2(row: *const MIB_IPFORWARD_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn DeleteIpNetEntry(parpentry: *const MIB_IPNETROW_LH) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DeleteIpNetEntry2(row: *const MIB_IPNET_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn DeletePersistentTcpPortReservation(startport: u16, numberofports: u16) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn DeletePersistentUdpPortReservation(startport: u16, numberofports: u16) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn DeleteProxyArpEntry(dwaddress: u32, dwmask: u32, dwifindex: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DeleteUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn DisableMediaSense(phandle: *mut super::super::Foundation::HANDLE, poverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn EnableRouter(phandle: *mut super::super::Foundation::HANDLE, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn FlushIpNetTable(dwifindex: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushIpNetTable2(family: u16, interfaceindex: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushIpPathTable(family: u16) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeDnsSettings(settings: *mut DNS_SETTINGS);
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeInterfaceDnsSettings(settings: *mut DNS_INTERFACE_SETTINGS);
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn FreeMibTable(memory: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAdapterIndex(adaptername: super::super::Foundation::PWSTR, ifindex: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetAdapterOrderMap() -> *mut IP_ADAPTER_ORDER_MAP;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetAdaptersAddresses(family: ADDRESS_FAMILY, flags: GET_ADAPTERS_ADDRESSES_FLAGS, reserved: *mut ::core::ffi::c_void, adapteraddresses: *mut IP_ADAPTER_ADDRESSES_LH, sizepointer: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAdaptersInfo(adapterinfo: *mut IP_ADAPTER_INFO, sizepointer: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetAnycastIpAddressEntry(row: *mut MIB_ANYCASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetAnycastIpAddressTable(family: u16, table: *mut *mut MIB_ANYCASTIPADDRESS_TABLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetBestInterface(dwdestaddr: u32, pdwbestifindex: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetBestInterfaceEx(pdestaddr: *const super::super::Networking::WinSock::SOCKADDR, pdwbestifindex: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn GetBestRoute(dwdestaddr: u32, dwsourceaddr: u32, pbestroute: *mut MIB_IPFORWARDROW) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetBestRoute2(interfaceluid: *const NET_LUID_LH, interfaceindex: u32, sourceaddress: *const super::super::Networking::WinSock::SOCKADDR_INET, destinationaddress: *const super::super::Networking::WinSock::SOCKADDR_INET, addresssortoptions: u32, bestroute: *mut MIB_IPFORWARD_ROW2, bestsourceaddress: *mut super::super::Networking::WinSock::SOCKADDR_INET) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetCurrentThreadCompartmentId() -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetCurrentThreadCompartmentScope(compartmentscope: *mut u32, compartmentid: *mut u32);
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetDefaultCompartmentId() -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDnsSettings(settings: *mut DNS_SETTINGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExtendedTcpTable(ptcptable: *mut ::core::ffi::c_void, pdwsize: *mut u32, border: super::super::Foundation::BOOL, ulaf: u32, tableclass: TCP_TABLE_CLASS, reserved: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExtendedUdpTable(pudptable: *mut ::core::ffi::c_void, pdwsize: *mut u32, border: super::super::Foundation::BOOL, ulaf: u32, tableclass: UDP_TABLE_CLASS, reserved: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetFriendlyIfIndex(ifindex: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetIcmpStatistics(statistics: *mut MIB_ICMP) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetIcmpStatisticsEx(statistics: *mut MIB_ICMP_EX_XPSP1, family: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetIfEntry(pifrow: *mut MIB_IFROW) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_NetworkManagement_Ndis`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn GetIfEntry2(row: *mut MIB_IF_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_NetworkManagement_Ndis`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn GetIfEntry2Ex(level: MIB_IF_ENTRY_LEVEL, row: *mut MIB_IF_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIfStackTable(table: *mut *mut MIB_IFSTACK_TABLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIfTable(piftable: *mut MIB_IFTABLE, pdwsize: *mut u32, border: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_NetworkManagement_Ndis`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn GetIfTable2(table: *mut *mut MIB_IF_TABLE2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_NetworkManagement_Ndis`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn GetIfTable2Ex(level: MIB_IF_TABLE_LEVEL, table: *mut *mut MIB_IF_TABLE2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInterfaceActiveTimestampCapabilities(interfaceluid: *const NET_LUID_LH, timestampcapabilites: *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInterfaceDnsSettings(interface: ::windows::runtime::GUID, settings: *mut DNS_INTERFACE_SETTINGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetInterfaceInfo(piftable: *mut IP_INTERFACE_INFO, dwoutbuflen: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInterfaceSupportedTimestampCapabilities(interfaceluid: *const NET_LUID_LH, timestampcapabilites: *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInvertedIfStackTable(table: *mut *mut MIB_INVERTEDIFSTACK_TABLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIpAddrTable(pipaddrtable: *mut MIB_IPADDRTABLE, pdwsize: *mut u32, border: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIpErrorString(errorcode: u32, buffer: super::super::Foundation::PWSTR, size: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpForwardEntry2(row: *mut MIB_IPFORWARD_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpForwardTable(pipforwardtable: *mut MIB_IPFORWARDTABLE, pdwsize: *mut u32, border: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpForwardTable2(family: u16, table: *mut *mut MIB_IPFORWARD_TABLE2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpInterfaceTable(family: u16, table: *mut *mut MIB_IPINTERFACE_TABLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpNetEntry2(row: *mut MIB_IPNET_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIpNetTable(ipnettable: *mut MIB_IPNETTABLE, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpNetTable2(family: u16, table: *mut *mut MIB_IPNET_TABLE2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpNetworkConnectionBandwidthEstimates(interfaceindex: u32, addressfamily: u16, bandwidthestimates: *mut MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpPathEntry(row: *mut MIB_IPPATH_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpPathTable(family: u16, table: *mut *mut MIB_IPPATH_TABLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetIpStatistics(statistics: *mut MIB_IPSTATS_LH) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetIpStatisticsEx(statistics: *mut MIB_IPSTATS_LH, family: ADDRESS_FAMILY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetJobCompartmentId(jobhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetMulticastIpAddressEntry(row: *mut MIB_MULTICASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetMulticastIpAddressTable(family: u16, table: *mut *mut MIB_MULTICASTIPADDRESS_TABLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetNetworkConnectivityHint(connectivityhint: *mut super::super::Networking::WinSock::NL_NETWORK_CONNECTIVITY_HINT) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetNetworkConnectivityHintForInterface(interfaceindex: u32, connectivityhint: *mut super::super::Networking::WinSock::NL_NETWORK_CONNECTIVITY_HINT) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNetworkInformation(networkguid: *const ::windows::runtime::GUID, compartmentid: *mut u32, siteid: *mut u32, networkname: super::super::Foundation::PWSTR, length: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNetworkParams(pfixedinfo: *mut FIXED_INFO_W2KSP1, poutbuflen: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetNumberOfInterfaces(pdwnumif: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetOwnerModuleFromPidAndInfo(ulpid: u32, pinfo: *const u64, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetOwnerModuleFromTcp6Entry(ptcpentry: *const MIB_TCP6ROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetOwnerModuleFromTcpEntry(ptcpentry: *const MIB_TCPROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetOwnerModuleFromUdp6Entry(pudpentry: *const MIB_UDP6ROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetOwnerModuleFromUdpEntry(pudpentry: *const MIB_UDPROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPerAdapterInfo(ifindex: u32, pperadapterinfo: *mut IP_PER_ADAPTER_INFO_W2KSP1, poutbuflen: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn GetPerTcp6ConnectionEStats(row: *const MIB_TCP6ROW, estatstype: TCP_ESTATS_TYPE, rw: *mut u8, rwversion: u32, rwsize: u32, ros: *mut u8, rosversion: u32, rossize: u32, rod: *mut u8, rodversion: u32, rodsize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetPerTcpConnectionEStats(row: *const MIB_TCPROW_LH, estatstype: TCP_ESTATS_TYPE, rw: *mut u8, rwversion: u32, rwsize: u32, ros: *mut u8, rosversion: u32, rossize: u32, rod: *mut u8, rodversion: u32, rodsize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRTTAndHopCount(destipaddress: u32, hopcount: *mut u32, maxhops: u32, rtt: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetSessionCompartmentId(sessionid: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetTcp6Table(tcptable: *mut MIB_TCP6TABLE, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetTcp6Table2(tcptable: *mut MIB_TCP6TABLE2, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetTcpStatistics(statistics: *mut MIB_TCPSTATS_LH) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetTcpStatisticsEx(statistics: *mut MIB_TCPSTATS_LH, family: ADDRESS_FAMILY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetTcpStatisticsEx2(statistics: *mut MIB_TCPSTATS2, family: ADDRESS_FAMILY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTcpTable(tcptable: *mut MIB_TCPTABLE, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTcpTable2(tcptable: *mut MIB_TCPTABLE2, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTeredoPort(port: *mut u16) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetUdp6Table(udp6table: *mut MIB_UDP6TABLE, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetUdpStatistics(stats: *mut MIB_UDPSTATS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetUdpStatisticsEx(statistics: *mut MIB_UDPSTATS, family: ADDRESS_FAMILY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetUdpStatisticsEx2(statistics: *mut MIB_UDPSTATS2, family: ADDRESS_FAMILY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUdpTable(udptable: *mut MIB_UDPTABLE, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn GetUniDirectionalAdapterInfo(pipifinfo: *mut IP_UNIDIRECTIONAL_ADAPTER_ADDRESS, dwoutbuflen: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetUnicastIpAddressEntry(row: *mut MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetUnicastIpAddressTable(family: u16, table: *mut *mut MIB_UNICASTIPADDRESS_TABLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn Icmp6CreateFile() -> IcmpHandle;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn Icmp6ParseReplies(replybuffer: *mut ::core::ffi::c_void, replysize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_WindowsProgramming"))]
    pub fn Icmp6SendEcho2(icmphandle: IcmpHandle, event: super::super::Foundation::HANDLE, apcroutine: ::windows::runtime::RawPtr, apccontext: *const ::core::ffi::c_void, sourceaddress: *const super::super::Networking::WinSock::SOCKADDR_IN6, destinationaddress: *const super::super::Networking::WinSock::SOCKADDR_IN6, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IcmpCloseHandle(icmphandle: IcmpHandle) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn IcmpCreateFile() -> IcmpHandle;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn IcmpParseReplies(replybuffer: *mut ::core::ffi::c_void, replysize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn IcmpSendEcho(icmphandle: IcmpHandle, destinationaddress: u32, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn IcmpSendEcho2(icmphandle: IcmpHandle, event: super::super::Foundation::HANDLE, apcroutine: ::windows::runtime::RawPtr, apccontext: *const ::core::ffi::c_void, destinationaddress: u32, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn IcmpSendEcho2Ex(icmphandle: IcmpHandle, event: super::super::Foundation::HANDLE, apcroutine: ::windows::runtime::RawPtr, apccontext: *const ::core::ffi::c_void, sourceaddress: u32, destinationaddress: u32, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn InitializeIpForwardEntry(row: *mut MIB_IPFORWARD_ROW2);
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn InitializeIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW);
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn InitializeUnicastIpAddressEntry(row: *mut MIB_UNICASTIPADDRESS_ROW);
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn IpReleaseAddress(adapterinfo: *const IP_ADAPTER_INDEX_MAP) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn IpRenewAddress(adapterinfo: *const IP_ADAPTER_INDEX_MAP) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn LookupPersistentTcpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn LookupPersistentUdpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NhpAllocateAndGetInterfaceInfoFromStack(pptable: *mut *mut ip_interface_name_info_w2ksp1, pdwcount: *mut u32, border: super::super::Foundation::BOOL, hheap: super::super::Foundation::HANDLE, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn NotifyAddrChange(handle: *mut super::super::Foundation::HANDLE, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyIpInterfaceChange(family: u16, callback: ::windows::runtime::RawPtr, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyNetworkConnectivityHintChange(callback: ::windows::runtime::RawPtr, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn NotifyRouteChange(handle: *mut super::super::Foundation::HANDLE, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyRouteChange2(addressfamily: u16, callback: ::windows::runtime::RawPtr, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyStableUnicastIpAddressTable(family: u16, table: *mut *mut MIB_UNICASTIPADDRESS_TABLE, callercallback: ::windows::runtime::RawPtr, callercontext: *const ::core::ffi::c_void, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NotifyTeredoPortChange(callback: ::windows::runtime::RawPtr, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyUnicastIpAddressChange(family: u16, callback: ::windows::runtime::RawPtr, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfAddFiltersToInterface(ih: *mut ::core::ffi::c_void, cinfilters: u32, pfiltin: *mut PF_FILTER_DESCRIPTOR, coutfilters: u32, pfiltout: *mut PF_FILTER_DESCRIPTOR, pfhandle: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfAddGlobalFilterToInterface(pinterface: *mut ::core::ffi::c_void, gffilter: GLOBAL_FILTER) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfBindInterfaceToIPAddress(pinterface: *mut ::core::ffi::c_void, pfattype: PFADDRESSTYPE, ipaddress: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfBindInterfaceToIndex(pinterface: *mut ::core::ffi::c_void, dwindex: u32, pfatlinktype: PFADDRESSTYPE, linkipaddress: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PfCreateInterface(dwname: u32, inaction: PFFORWARD_ACTION, outaction: PFFORWARD_ACTION, buselog: super::super::Foundation::BOOL, bmustbeunique: super::super::Foundation::BOOL, ppinterface: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfDeleteInterface(pinterface: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfDeleteLog() -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PfGetInterfaceStatistics(pinterface: *mut ::core::ffi::c_void, ppfstats: *mut PF_INTERFACE_STATS, pdwbuffersize: *mut u32, fresetcounters: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PfMakeLog(hevent: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfRebindFilters(pinterface: *mut ::core::ffi::c_void, platebindinfo: *mut PF_LATEBIND_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfRemoveFilterHandles(pinterface: *mut ::core::ffi::c_void, cfilters: u32, pvhandles: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfRemoveFiltersFromInterface(ih: *mut ::core::ffi::c_void, cinfilters: u32, pfiltin: *mut PF_FILTER_DESCRIPTOR, coutfilters: u32, pfiltout: *mut PF_FILTER_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfRemoveGlobalFilterFromInterface(pinterface: *mut ::core::ffi::c_void, gffilter: GLOBAL_FILTER) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfSetLogBuffer(pbbuffer: *mut u8, dwsize: u32, dwthreshold: u32, dwentries: u32, pdwloggedentries: *mut u32, pdwlostentries: *mut u32, pdwsizeused: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfTestPacket(pininterface: *mut ::core::ffi::c_void, poutinterface: *mut ::core::ffi::c_void, cbytes: u32, pbpacket: *mut u8, ppaction: *mut PFFORWARD_ACTION) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn PfUnBindInterface(pinterface: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn RegisterInterfaceTimestampConfigChange(callback: ::windows::runtime::RawPtr, callercontext: *const ::core::ffi::c_void, notificationhandle: *mut HIFTIMESTAMPCHANGE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn ResolveIpNetEntry2(row: *mut MIB_IPNET_ROW2, sourceaddress: *const super::super::Networking::WinSock::SOCKADDR_INET) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn ResolveNeighbor(networkaddress: *const super::super::Networking::WinSock::SOCKADDR, physicaladdress: *mut ::core::ffi::c_void, physicaladdresslength: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RestoreMediaSense(poverlapped: *const super::super::System::IO::OVERLAPPED, lpdwenablecount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn SendARP(destip: u32, srcip: u32, pmacaddr: *mut ::core::ffi::c_void, phyaddrlen: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCurrentThreadCompartmentId(compartmentid: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCurrentThreadCompartmentScope(compartmentscope: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDnsSettings(settings: *const DNS_SETTINGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn SetIfEntry(pifrow: *const MIB_IFROW) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetInterfaceDnsSettings(interface: ::windows::runtime::GUID, settings: *const DNS_INTERFACE_SETTINGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn SetIpForwardEntry(proute: *const MIB_IPFORWARDROW) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn SetIpForwardEntry2(route: *const MIB_IPFORWARD_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn SetIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn SetIpNetEntry(parpentry: *const MIB_IPNETROW_LH) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn SetIpNetEntry2(row: *const MIB_IPNET_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn SetIpStatistics(pipstats: *const MIB_IPSTATS_LH) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn SetIpStatisticsEx(statistics: *const MIB_IPSTATS_LH, family: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn SetIpTTL(nttl: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetJobCompartmentId(jobhandle: super::super::Foundation::HANDLE, compartmentid: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetNetworkInformation(networkguid: *const ::windows::runtime::GUID, compartmentid: u32, networkname: super::super::Foundation::PWSTR) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn SetPerTcp6ConnectionEStats(row: *const MIB_TCP6ROW, estatstype: TCP_ESTATS_TYPE, rw: *const u8, rwversion: u32, rwsize: u32, offset: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn SetPerTcpConnectionEStats(row: *const MIB_TCPROW_LH, estatstype: TCP_ESTATS_TYPE, rw: *const u8, rwversion: u32, rwsize: u32, offset: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSessionCompartmentId(sessionid: u32, compartmentid: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn SetTcpEntry(ptcprow: *const MIB_TCPROW_LH) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn SetUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn UnenableRouter(poverlapped: *const super::super::System::IO::OVERLAPPED, lpdwenablecount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
    pub fn UnregisterInterfaceTimestampConfigChange(notificationhandle: HIFTIMESTAMPCHANGE);
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn if_indextoname(interfaceindex: u32, interfacename: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn if_nametoindex(interfacename: super::super::Foundation::PSTR) -> u32;
}
