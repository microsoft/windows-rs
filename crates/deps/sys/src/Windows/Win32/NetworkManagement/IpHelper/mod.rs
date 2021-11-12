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
    pub fn ConvertCompartmentGuidToId(compartmentguid: *const ::windows_sys::core::GUID, compartmentid: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertCompartmentIdToGuid(compartmentid: u32, compartmentguid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceAliasToLuid(interfacealias: super::super::Foundation::PWSTR, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceGuidToLuid(interfaceguid: *const ::windows_sys::core::GUID, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceIndexToLuid(interfaceindex: u32, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToAlias(interfaceluid: *const NET_LUID_LH, interfacealias: super::super::Foundation::PWSTR, length: usize) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToGuid(interfaceluid: *const NET_LUID_LH, interfaceguid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::NTSTATUS;
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
    pub fn GetInterfaceDnsSettings(interface: ::windows_sys::core::GUID, settings: *mut DNS_INTERFACE_SETTINGS) -> super::super::Foundation::NTSTATUS;
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
    pub fn GetNetworkInformation(networkguid: *const ::windows_sys::core::GUID, compartmentid: *mut u32, siteid: *mut u32, networkname: super::super::Foundation::PWSTR, length: u32) -> super::super::Foundation::NTSTATUS;
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
    pub fn Icmp6SendEcho2(
        icmphandle: IcmpHandle,
        event: super::super::Foundation::HANDLE,
        apcroutine: super::super::System::WindowsProgramming::PIO_APC_ROUTINE,
        apccontext: *const ::core::ffi::c_void,
        sourceaddress: *const super::super::Networking::WinSock::SOCKADDR_IN6,
        destinationaddress: *const super::super::Networking::WinSock::SOCKADDR_IN6,
        requestdata: *const ::core::ffi::c_void,
        requestsize: u16,
        requestoptions: *const ip_option_information,
        replybuffer: *mut ::core::ffi::c_void,
        replysize: u32,
        timeout: u32,
    ) -> u32;
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
    pub fn IcmpSendEcho2(icmphandle: IcmpHandle, event: super::super::Foundation::HANDLE, apcroutine: super::super::System::WindowsProgramming::PIO_APC_ROUTINE, apccontext: *const ::core::ffi::c_void, destinationaddress: u32, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn IcmpSendEcho2Ex(icmphandle: IcmpHandle, event: super::super::Foundation::HANDLE, apcroutine: super::super::System::WindowsProgramming::PIO_APC_ROUTINE, apccontext: *const ::core::ffi::c_void, sourceaddress: u32, destinationaddress: u32, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32;
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
    pub fn NotifyIpInterfaceChange(family: u16, callback: PIPINTERFACE_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyNetworkConnectivityHintChange(callback: PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn NotifyRouteChange(handle: *mut super::super::Foundation::HANDLE, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyRouteChange2(addressfamily: u16, callback: PIPFORWARD_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyStableUnicastIpAddressTable(family: u16, table: *mut *mut MIB_UNICASTIPADDRESS_TABLE, callercallback: PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK, callercontext: *const ::core::ffi::c_void, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NotifyTeredoPortChange(callback: PTEREDO_PORT_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_NetworkManagement_IpHelper`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyUnicastIpAddressChange(family: u16, callback: PUNICAST_IPADDRESS_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
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
    pub fn RegisterInterfaceTimestampConfigChange(callback: PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, notificationhandle: *mut HIFTIMESTAMPCHANGE) -> u32;
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
    pub fn SetInterfaceDnsSettings(interface: ::windows_sys::core::GUID, settings: *const DNS_INTERFACE_SETTINGS) -> super::super::Foundation::NTSTATUS;
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
    pub fn SetNetworkInformation(networkguid: *const ::windows_sys::core::GUID, compartmentid: u32, networkname: super::super::Foundation::PWSTR) -> super::super::Foundation::NTSTATUS;
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
pub struct ADDRESS_FAMILY(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const ANY_SIZE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const BEST_IF: u32 = 20u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const BEST_ROUTE: u32 = 21u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const BROADCAST_NODETYPE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DEFAULT_MINIMUM_ENTITIES: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DEST_LONGER: u32 = 29u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DEST_MATCHING: u32 = 28u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DEST_SHORTER: u32 = 30u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_DOH_AUTO_UPGRADE_SERVER: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_DOH_POLICY_AUTO: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_DOH_POLICY_DISABLE: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_DOH_POLICY_NOT_CONFIGURED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_DOH_POLICY_REQUIRED: u32 = 32u32;
pub struct DNS_DOH_SERVER_SETTINGS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_DOH_SERVER_SETTINGS_ENABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_DOH_SERVER_SETTINGS_ENABLE_AUTO: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_DOH_SERVER_SETTINGS_FALLBACK_TO_UDP: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_ENABLE_DOH: u32 = 1u32;
pub struct DNS_INTERFACE_SETTINGS(i32);
pub struct DNS_INTERFACE_SETTINGS3(i32);
pub struct DNS_INTERFACE_SETTINGS_EX(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_INTERFACE_SETTINGS_VERSION1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_INTERFACE_SETTINGS_VERSION2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_INTERFACE_SETTINGS_VERSION3: u32 = 3u32;
pub struct DNS_SERVER_PROPERTY(i32);
pub struct DNS_SERVER_PROPERTY_TYPE(i32);
pub struct DNS_SERVER_PROPERTY_TYPES(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SERVER_PROPERTY_VERSION1: u32 = 1u32;
pub struct DNS_SETTINGS(i32);
pub struct DNS_SETTINGS2(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SETTINGS_ENABLE_LLMNR: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SETTINGS_QUERY_ADAPTER_NAME: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SETTINGS_VERSION1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SETTINGS_VERSION2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SETTING_DISABLE_UNCONSTRAINED_QUERIES: u32 = 1024u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SETTING_DOH: u32 = 4096u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SETTING_DOH_PROFILE: u32 = 8192u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SETTING_DOMAIN: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SETTING_HOSTNAME: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SETTING_IPV6: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SETTING_NAMESERVER: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SETTING_PROFILE_NAMESERVER: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SETTING_REGISTER_ADAPTER_NAME: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SETTING_REGISTRATION_ENABLED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SETTING_SEARCHLIST: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const DNS_SETTING_SUPPLEMENTAL_SEARCH_LIST: u32 = 2048u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const ERROR_BASE: u32 = 23000u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const ERROR_IPV6_NOT_IMPLEMENTED: u32 = 23003u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const FD_FLAGS_ALLFLAGS: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const FD_FLAGS_NOSYN: u32 = 1u32;
pub struct FIXED_INFO_W2KSP1(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const GAA_FLAG_SKIP_DNS_INFO: u32 = 2048u32;
pub struct GET_ADAPTERS_ADDRESSES_FLAGS(i32);
pub struct GLOBAL_FILTER(i32);
pub struct HIFTIMESTAMPCHANGE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const HYBRID_NODETYPE: u32 = 8u32;
pub struct ICMP4_TYPE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const ICMP6_INFOMSG_MASK: u32 = 128u32;
pub struct ICMP6_TYPE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const ICMP_STATS: u32 = 11u32;
pub struct IF_ACCESS_TYPE(i32);
pub struct IF_ADMINISTRATIVE_STATE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_ADMIN_STATUS_DOWN: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_ADMIN_STATUS_TESTING: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_ADMIN_STATUS_UP: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_CHECK_MCAST: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_CHECK_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_CHECK_SEND: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_CONNECTION_DEDICATED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_CONNECTION_DEMAND: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_CONNECTION_PASSIVE: u32 = 2u32;
pub struct IF_COUNTED_STRING_LH(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_MAX_PHYS_ADDRESS_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_MAX_STRING_SIZE: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_NUMBER: u32 = 0u32;
pub struct IF_OPER_STATUS(i32);
pub struct IF_PHYSICAL_ADDRESS_LH(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_ROW: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_STATUS: u32 = 25u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_A12MPPSWITCH: u32 = 130u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_AAL2: u32 = 187u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_AAL5: u32 = 49u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ADSL: u32 = 94u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_AFLANE_8023: u32 = 59u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_AFLANE_8025: u32 = 60u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ARAP: u32 = 88u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ARCNET: u32 = 35u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ARCNET_PLUS: u32 = 36u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ASYNC: u32 = 84u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ATM: u32 = 37u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ATM_DXI: u32 = 105u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ATM_FUNI: u32 = 106u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ATM_IMA: u32 = 107u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ATM_LOGICAL: u32 = 80u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ATM_RADIO: u32 = 189u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ATM_SUBINTERFACE: u32 = 134u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ATM_VCI_ENDPT: u32 = 194u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ATM_VIRTUAL: u32 = 149u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_BASIC_ISDN: u32 = 20u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_BGP_POLICY_ACCOUNTING: u32 = 162u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_BSC: u32 = 83u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_CCTEMUL: u32 = 61u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_CES: u32 = 133u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_CHANNEL: u32 = 70u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_CNR: u32 = 85u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_COFFEE: u32 = 132u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_COMPOSITELINK: u32 = 155u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DCN: u32 = 141u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DDN_X25: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DIGITALPOWERLINE: u32 = 138u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DIGITAL_WRAPPER_OVERHEAD_CHANNEL: u32 = 186u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DLSW: u32 = 74u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DOCSCABLE_DOWNSTREAM: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DOCSCABLE_MACLAYER: u32 = 127u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DOCSCABLE_UPSTREAM: u32 = 129u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DS0: u32 = 81u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DS0_BUNDLE: u32 = 82u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DS1: u32 = 18u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DS1_FDL: u32 = 170u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DS3: u32 = 30u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DTM: u32 = 140u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DVBRCC_DOWNSTREAM: u32 = 147u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DVBRCC_MACLAYER: u32 = 146u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DVBRCC_UPSTREAM: u32 = 148u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DVB_ASI_IN: u32 = 172u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_DVB_ASI_OUT: u32 = 173u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_E1: u32 = 19u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_EON: u32 = 25u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_EPLRS: u32 = 87u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ESCON: u32 = 73u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ETHERNET_3MBIT: u32 = 26u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ETHERNET_CSMACD: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_FAST: u32 = 125u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_FASTETHER: u32 = 62u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_FASTETHER_FX: u32 = 69u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_FDDI: u32 = 15u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_FIBRECHANNEL: u32 = 56u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_FRAMERELAY: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_FRAMERELAY_INTERCONNECT: u32 = 58u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_FRAMERELAY_MPI: u32 = 92u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_FRAMERELAY_SERVICE: u32 = 44u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_FRF16_MFR_BUNDLE: u32 = 163u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_FR_DLCI_ENDPT: u32 = 193u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_FR_FORWARD: u32 = 158u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_G703_2MB: u32 = 67u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_G703_64K: u32 = 66u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_GIGABITETHERNET: u32 = 117u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_GR303_IDT: u32 = 178u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_GR303_RDT: u32 = 177u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_H323_GATEKEEPER: u32 = 164u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_H323_PROXY: u32 = 165u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_HDH_1822: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_HDLC: u32 = 118u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_HDSL2: u32 = 168u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_HIPERLAN2: u32 = 183u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_HIPPI: u32 = 47u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_HIPPIINTERFACE: u32 = 57u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_HOSTPAD: u32 = 90u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_HSSI: u32 = 46u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_HYPERCHANNEL: u32 = 14u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IBM370PARCHAN: u32 = 72u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IDSL: u32 = 154u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IEEE1394: u32 = 144u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IEEE80211: u32 = 71u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IEEE80212: u32 = 55u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IEEE802154: u32 = 259u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IEEE80216_WMAN: u32 = 237u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IEEE8023AD_LAG: u32 = 161u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IF_GSN: u32 = 145u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IMT: u32 = 190u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_INTERLEAVE: u32 = 124u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IP: u32 = 126u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IPFORWARD: u32 = 142u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IPOVER_ATM: u32 = 114u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IPOVER_CDLC: u32 = 109u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IPOVER_CLAW: u32 = 110u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IPSWITCH: u32 = 78u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_IS088023_CSMACD: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ISDN: u32 = 63u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ISDN_S: u32 = 75u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ISDN_U: u32 = 76u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ISO88022_LLC: u32 = 41u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ISO88024_TOKENBUS: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ISO88025R_DTR: u32 = 86u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ISO88025_CRFPRINT: u32 = 98u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ISO88025_FIBER: u32 = 115u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ISO88025_TOKENRING: u32 = 9u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ISO88026_MAN: u32 = 10u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ISUP: u32 = 179u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_L2_VLAN: u32 = 135u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_L3_IPVLAN: u32 = 136u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_L3_IPXVLAN: u32 = 137u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_LAP_B: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_LAP_D: u32 = 77u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_LAP_F: u32 = 119u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_LOCALTALK: u32 = 42u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_MEDIAMAILOVERIP: u32 = 139u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_MF_SIGLINK: u32 = 167u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_MIO_X25: u32 = 38u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_MODEM: u32 = 48u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_MPC: u32 = 113u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_MPLS: u32 = 166u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_MPLS_TUNNEL: u32 = 150u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_MSDSL: u32 = 143u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_MVL: u32 = 191u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_MYRINET: u32 = 99u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_NFAS: u32 = 175u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_NSIP: u32 = 27u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_OPTICAL_CHANNEL: u32 = 195u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_OPTICAL_TRANSPORT: u32 = 196u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_OTHER: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_PARA: u32 = 34u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_PLC: u32 = 174u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_POS: u32 = 171u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_PPP: u32 = 23u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_PPPMULTILINKBUNDLE: u32 = 108u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_PRIMARY_ISDN: u32 = 21u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_PROP_BWA_P2MP: u32 = 184u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_PROP_CNLS: u32 = 89u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_PROP_DOCS_WIRELESS_DOWNSTREAM: u32 = 181u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_PROP_DOCS_WIRELESS_MACLAYER: u32 = 180u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_PROP_DOCS_WIRELESS_UPSTREAM: u32 = 182u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_PROP_MULTIPLEXOR: u32 = 54u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_PROP_POINT2POINT_SERIAL: u32 = 22u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_PROP_VIRTUAL: u32 = 53u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_PROP_WIRELESS_P2P: u32 = 157u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_PROTEON_10MBIT: u32 = 12u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_PROTEON_80MBIT: u32 = 13u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_QLLC: u32 = 68u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_RADIO_MAC: u32 = 188u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_RADSL: u32 = 95u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_REACH_DSL: u32 = 192u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_REGULAR_1822: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_RFC1483: u32 = 159u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_RFC877_X25: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_RS232: u32 = 33u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_RSRB: u32 = 79u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_SDLC: u32 = 17u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_SDSL: u32 = 96u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_SHDSL: u32 = 169u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_SIP: u32 = 31u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_SLIP: u32 = 28u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_SMDS_DXI: u32 = 43u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_SMDS_ICIP: u32 = 52u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_SOFTWARE_LOOPBACK: u32 = 24u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_SONET: u32 = 39u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_SONET_OVERHEAD_CHANNEL: u32 = 185u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_SONET_PATH: u32 = 50u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_SONET_VT: u32 = 51u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_SRP: u32 = 151u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_SS7_SIGLINK: u32 = 156u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_STACKTOSTACK: u32 = 111u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_STARLAN: u32 = 11u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_TDLC: u32 = 116u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_TERMPAD: u32 = 91u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_TR008: u32 = 176u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_TRANSPHDLC: u32 = 123u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_TUNNEL: u32 = 131u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_ULTRA: u32 = 29u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_USB: u32 = 160u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_V11: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_V35: u32 = 45u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_V36: u32 = 65u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_V37: u32 = 120u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_VDSL: u32 = 97u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_VIRTUALIPADDRESS: u32 = 112u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_VOICEOVERATM: u32 = 152u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_VOICEOVERFRAMERELAY: u32 = 153u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_VOICE_EM: u32 = 100u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_VOICE_ENCAP: u32 = 103u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_VOICE_FXO: u32 = 101u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_VOICE_FXS: u32 = 102u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_VOICE_OVERIP: u32 = 104u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_WWANPP: u32 = 243u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_WWANPP2: u32 = 244u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_X213: u32 = 93u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_X25_HUNTGROUP: u32 = 122u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_X25_MLP: u32 = 121u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_X25_PLE: u32 = 40u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IF_TYPE_XBOX_WIRELESS: u32 = 281u32;
pub struct INTERFACE_HARDWARE_CROSSTIMESTAMP(i32);
pub struct INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES(i32);
pub struct INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES(i32);
pub struct INTERFACE_TIMESTAMP_CAPABILITIES(i32);
pub struct INTERNAL_IF_OPER_STATUS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IOCTL_ARP_SEND_REQUEST: u32 = 103u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IOCTL_IP_ADDCHANGE_NOTIFY_REQUEST: u32 = 102u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IOCTL_IP_GET_BEST_INTERFACE: u32 = 105u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IOCTL_IP_INTERFACE_INFO: u32 = 104u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IOCTL_IP_RTCHANGE_NOTIFY_REQUEST: u32 = 101u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IOCTL_IP_UNIDIRECTIONAL_ADAPTER_ADDRESS: u32 = 106u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP6_STATS: u32 = 36u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IPRTRMGR_PID: u32 = 10000u32;
pub struct IPV6_ADDRESS_EX(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IPV6_GLOBAL_INFO: u32 = 4294901775u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IPV6_ROUTE_INFO: u32 = 4294901776u32;
pub struct IP_ADAPTER_ADDRESSES_LH(i32);
pub struct IP_ADAPTER_ADDRESSES_XP(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ADAPTER_ADDRESS_DNS_ELIGIBLE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ADAPTER_ADDRESS_TRANSIENT: u32 = 2u32;
pub struct IP_ADAPTER_ANYCAST_ADDRESS_XP(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ADAPTER_DDNS_ENABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ADAPTER_DHCP_ENABLED: u32 = 4u32;
pub struct IP_ADAPTER_DNS_SERVER_ADDRESS_XP(i32);
pub struct IP_ADAPTER_DNS_SUFFIX(i32);
pub struct IP_ADAPTER_GATEWAY_ADDRESS_LH(i32);
pub struct IP_ADAPTER_INDEX_MAP(i32);
pub struct IP_ADAPTER_INFO(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ADAPTER_IPV4_ENABLED: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ADAPTER_IPV6_ENABLED: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ADAPTER_IPV6_MANAGE_ADDRESS_CONFIG: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ADAPTER_IPV6_OTHER_STATEFUL_CONFIG: u32 = 32u32;
pub struct IP_ADAPTER_MULTICAST_ADDRESS_XP(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ADAPTER_NETBIOS_OVER_TCPIP_ENABLED: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ADAPTER_NO_MULTICAST: u32 = 16u32;
pub struct IP_ADAPTER_ORDER_MAP(i32);
pub struct IP_ADAPTER_PREFIX_XP(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ADAPTER_RECEIVE_ONLY: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ADAPTER_REGISTER_ADAPTER_SUFFIX: u32 = 2u32;
pub struct IP_ADAPTER_UNICAST_ADDRESS_LH(i32);
pub struct IP_ADAPTER_UNICAST_ADDRESS_XP(i32);
pub struct IP_ADAPTER_WINS_SERVER_ADDRESS_LH(i32);
pub struct IP_ADDRESS_PREFIX(i32);
pub struct IP_ADDRESS_STRING(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ADDRROW: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ADDRTABLE: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ADDR_ADDED: u32 = 11023u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ADDR_DELETED: u32 = 11019u32;
pub struct IP_ADDR_STRING(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_BAD_DESTINATION: u32 = 11018u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_BAD_HEADER: u32 = 11042u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_BAD_OPTION: u32 = 11007u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_BAD_REQ: u32 = 11011u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_BAD_ROUTE: u32 = 11012u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_BIND_ADAPTER: u32 = 11026u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_BUF_TOO_SMALL: u32 = 11001u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_DEMAND_DIAL_FILTER_INFO: u32 = 4294901769u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_DEMAND_DIAL_FILTER_INFO_V6: u32 = 4294901779u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_DEST_ADDR_UNREACHABLE: u32 = 11003u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_DEST_HOST_UNREACHABLE: u32 = 11003u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_DEST_NET_UNREACHABLE: u32 = 11002u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_DEST_NO_ROUTE: u32 = 11002u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_DEST_PORT_UNREACHABLE: u32 = 11005u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_DEST_PROHIBITED: u32 = 11004u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_DEST_PROT_UNREACHABLE: u32 = 11004u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_DEST_SCOPE_MISMATCH: u32 = 11045u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_DEST_UNREACHABLE: u32 = 11040u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_DEVICE_DOES_NOT_EXIST: u32 = 11028u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_DUPLICATE_ADDRESS: u32 = 11029u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_DUPLICATE_IPADD: u32 = 11034u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_EXPORT_INCLUDED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_FILTER_ENABLE_INFO: u32 = 4294901781u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_FILTER_ENABLE_INFO_V6: u32 = 4294901782u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_FLAG_DF: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_FLAG_REVERSE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_FORWARDNUMBER: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_FORWARDROW: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_FORWARDTABLE: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_GENERAL_FAILURE: u32 = 11050u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_GENERAL_INFO_BASE: u32 = 4294901760u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_GLOBAL_INFO: u32 = 4294901763u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_HOP_LIMIT_EXCEEDED: u32 = 11013u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_HW_ERROR: u32 = 11008u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ICMP_ERROR: u32 = 11044u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_IFFILTER_INFO: u32 = 4294901773u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_IFFILTER_INFO_V6: u32 = 4294901780u32;
pub struct IP_INTERFACE_INFO(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_INTERFACE_METRIC_CHANGE: u32 = 11030u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_INTERFACE_STATUS_INFO: u32 = 4294901764u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_INTERFACE_WOL_CAPABILITY_CHANGE: u32 = 11033u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_IN_FILTER_INFO: u32 = 4294901761u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_IN_FILTER_INFO_V6: u32 = 4294901777u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_IPINIP_CFG_INFO: u32 = 4294901772u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_MCAST_BOUNDARY_INFO: u32 = 4294901771u32;
pub struct IP_MCAST_COUNTER_INFO(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_MCAST_HEARBEAT_INFO: u32 = 4294901770u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_MCAST_LIMIT_INFO: u32 = 4294901774u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_MEDIA_CONNECT: u32 = 11024u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_MEDIA_DISCONNECT: u32 = 11025u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_MTU_CHANGE: u32 = 11021u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_NEGOTIATING_IPSEC: u32 = 11032u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_NETROW: u32 = 10u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_NETTABLE: u32 = 9u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_NO_RESOURCES: u32 = 11006u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_OPTION_TOO_BIG: u32 = 11017u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_OUT_FILTER_INFO: u32 = 4294901762u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_OUT_FILTER_INFO_V6: u32 = 4294901778u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_PACKET_TOO_BIG: u32 = 11009u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_PARAMETER_PROBLEM: u32 = 11015u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_PARAM_PROBLEM: u32 = 11015u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_PENDING: u32 = 11255u32;
pub struct IP_PER_ADAPTER_INFO_W2KSP1(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_PROT_PRIORITY_INFO: u32 = 4294901766u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_PROT_PRIORITY_INFO_EX: u32 = 4294901783u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_REASSEMBLY_TIME_EXCEEDED: u32 = 11014u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_RECONFIG_SECFLTR: u32 = 11031u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_REQ_TIMED_OUT: u32 = 11010u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ROUTER_DISC_INFO: u32 = 4294901767u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ROUTER_MANAGER_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_ROUTE_INFO: u32 = 4294901765u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_SOURCE_QUENCH: u32 = 11016u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_SPEC_MTU_CHANGE: u32 = 11020u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_STATS: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_STATUS_BASE: u32 = 11000u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_TIME_EXCEEDED: u32 = 11041u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_TTL_EXPIRED_REASSEM: u32 = 11014u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_TTL_EXPIRED_TRANSIT: u32 = 11013u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_UNBIND_ADAPTER: u32 = 11027u32;
pub struct IP_UNIDIRECTIONAL_ADAPTER_ADDRESS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_UNLOAD: u32 = 11022u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const IP_UNRECOGNIZED_NEXT_HEADER: u32 = 11043u32;
pub struct IcmpHandle(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const LB_DST_ADDR_USE_DSTADDR_FLAG: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const LB_DST_ADDR_USE_SRCADDR_FLAG: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const LB_DST_MASK_LATE_FLAG: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const LB_SRC_ADDR_USE_DSTADDR_FLAG: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const LB_SRC_ADDR_USE_SRCADDR_FLAG: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const LB_SRC_MASK_LATE_FLAG: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAXLEN_IFDESCR: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAXLEN_PHYSADDR: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAX_ADAPTER_ADDRESS_LENGTH: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAX_ADAPTER_DESCRIPTION_LENGTH: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAX_ADAPTER_NAME: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAX_ADAPTER_NAME_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAX_DHCPV6_DUID_LENGTH: u32 = 130u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAX_DNS_SUFFIX_STRING_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAX_DOMAIN_NAME_LEN: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAX_HOSTNAME_LEN: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAX_IF_TYPE: u32 = 281u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAX_INTERFACE_NAME_LEN: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAX_IP_STATUS: u32 = 11050u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAX_MIB_OFFSET: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAX_OPT_SIZE: u32 = 40u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAX_SCOPE_ID_LEN: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MAX_SCOPE_NAME_LEN: u32 = 255u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MCAST_BOUNDARY: u32 = 26u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MCAST_GLOBAL: u32 = 24u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MCAST_IF_ENTRY: u32 = 23u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MCAST_MFE: u32 = 18u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MCAST_MFE_STATS: u32 = 19u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MCAST_MFE_STATS_EX: u32 = 35u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MCAST_SCOPE: u32 = 27u32;
pub struct MIBICMPINFO(i32);
pub struct MIBICMPSTATS(i32);
pub struct MIBICMPSTATS_EX_XPSP1(i32);
pub struct MIB_ANYCASTIPADDRESS_ROW(i32);
pub struct MIB_ANYCASTIPADDRESS_TABLE(i32);
pub struct MIB_BEST_IF(i32);
pub struct MIB_BOUNDARYROW(i32);
pub struct MIB_ICMP(i32);
pub struct MIB_ICMP_EX_XPSP1(i32);
pub struct MIB_IFNUMBER(i32);
pub struct MIB_IFROW(i32);
pub struct MIB_IFSTACK_ROW(i32);
pub struct MIB_IFSTACK_TABLE(i32);
pub struct MIB_IFSTATUS(i32);
pub struct MIB_IFTABLE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IF_ADMIN_STATUS_DOWN: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IF_ADMIN_STATUS_TESTING: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IF_ADMIN_STATUS_UP: u32 = 1u32;
pub struct MIB_IF_ENTRY_LEVEL(i32);
pub struct MIB_IF_ROW2(i32);
pub struct MIB_IF_TABLE2(i32);
pub struct MIB_IF_TABLE_LEVEL(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IF_TYPE_ETHERNET: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IF_TYPE_FDDI: u32 = 15u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IF_TYPE_LOOPBACK: u32 = 24u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IF_TYPE_OTHER: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IF_TYPE_PPP: u32 = 23u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IF_TYPE_SLIP: u32 = 28u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IF_TYPE_TOKENRING: u32 = 9u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_INVALID_TEREDO_PORT_NUMBER: u32 = 0u32;
pub struct MIB_INVERTEDIFSTACK_ROW(i32);
pub struct MIB_INVERTEDIFSTACK_TABLE(i32);
pub struct MIB_IPADDRROW_W2K(i32);
pub struct MIB_IPADDRROW_XP(i32);
pub struct MIB_IPADDRTABLE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IPADDR_DELETED: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IPADDR_DISCONNECTED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IPADDR_DNS_ELIGIBLE: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IPADDR_DYNAMIC: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IPADDR_PRIMARY: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IPADDR_TRANSIENT: u32 = 128u32;
pub struct MIB_IPDESTROW(i32);
pub struct MIB_IPDESTTABLE(i32);
pub struct MIB_IPFORWARDNUMBER(i32);
pub struct MIB_IPFORWARDROW(i32);
pub struct MIB_IPFORWARDTABLE(i32);
pub struct MIB_IPFORWARD_ROW2(i32);
pub struct MIB_IPFORWARD_TABLE2(i32);
pub struct MIB_IPFORWARD_TYPE(i32);
pub struct MIB_IPINTERFACE_ROW(i32);
pub struct MIB_IPINTERFACE_TABLE(i32);
pub struct MIB_IPMCAST_BOUNDARY(i32);
pub struct MIB_IPMCAST_BOUNDARY_TABLE(i32);
pub struct MIB_IPMCAST_GLOBAL(i32);
pub struct MIB_IPMCAST_IF_ENTRY(i32);
pub struct MIB_IPMCAST_IF_TABLE(i32);
pub struct MIB_IPMCAST_MFE(i32);
pub struct MIB_IPMCAST_MFE_STATS(i32);
pub struct MIB_IPMCAST_MFE_STATS_EX_XP(i32);
pub struct MIB_IPMCAST_OIF_STATS_LH(i32);
pub struct MIB_IPMCAST_OIF_STATS_W2K(i32);
pub struct MIB_IPMCAST_OIF_W2K(i32);
pub struct MIB_IPMCAST_OIF_XP(i32);
pub struct MIB_IPMCAST_SCOPE(i32);
pub struct MIB_IPNETROW_LH(i32);
pub struct MIB_IPNETROW_W2K(i32);
pub struct MIB_IPNETTABLE(i32);
pub struct MIB_IPNET_ROW2(i32);
pub struct MIB_IPNET_TABLE2(i32);
pub struct MIB_IPNET_TYPE(i32);
pub struct MIB_IPPATH_ROW(i32);
pub struct MIB_IPPATH_TABLE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_IPROUTE_METRIC_UNUSED: u32 = 4294967295u32;
pub struct MIB_IPSTATS_FORWARDING(i32);
pub struct MIB_IPSTATS_LH(i32);
pub struct MIB_IPSTATS_W2K(i32);
pub struct MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES(i32);
pub struct MIB_MCAST_LIMIT_ROW(i32);
pub struct MIB_MFE_STATS_TABLE(i32);
pub struct MIB_MFE_STATS_TABLE_EX_XP(i32);
pub struct MIB_MFE_TABLE(i32);
pub struct MIB_MULTICASTIPADDRESS_ROW(i32);
pub struct MIB_MULTICASTIPADDRESS_TABLE(i32);
pub struct MIB_NOTIFICATION_TYPE(i32);
pub struct MIB_OPAQUE_INFO(i32);
pub struct MIB_OPAQUE_QUERY(i32);
pub struct MIB_PROXYARP(i32);
pub struct MIB_ROUTESTATE(i32);
pub struct MIB_TCP6ROW(i32);
pub struct MIB_TCP6ROW2(i32);
pub struct MIB_TCP6ROW_OWNER_MODULE(i32);
pub struct MIB_TCP6ROW_OWNER_PID(i32);
pub struct MIB_TCP6TABLE(i32);
pub struct MIB_TCP6TABLE2(i32);
pub struct MIB_TCP6TABLE_OWNER_MODULE(i32);
pub struct MIB_TCP6TABLE_OWNER_PID(i32);
pub struct MIB_TCPROW2(i32);
pub struct MIB_TCPROW_LH(i32);
pub struct MIB_TCPROW_OWNER_MODULE(i32);
pub struct MIB_TCPROW_OWNER_PID(i32);
pub struct MIB_TCPROW_W2K(i32);
pub struct MIB_TCPSTATS2(i32);
pub struct MIB_TCPSTATS_LH(i32);
pub struct MIB_TCPSTATS_W2K(i32);
pub struct MIB_TCPTABLE(i32);
pub struct MIB_TCPTABLE2(i32);
pub struct MIB_TCPTABLE_OWNER_MODULE(i32);
pub struct MIB_TCPTABLE_OWNER_PID(i32);
pub struct MIB_TCP_STATE(i32);
pub struct MIB_UDP6ROW(i32);
pub struct MIB_UDP6ROW2(i32);
pub struct MIB_UDP6ROW_OWNER_MODULE(i32);
pub struct MIB_UDP6ROW_OWNER_PID(i32);
pub struct MIB_UDP6TABLE(i32);
pub struct MIB_UDP6TABLE2(i32);
pub struct MIB_UDP6TABLE_OWNER_MODULE(i32);
pub struct MIB_UDP6TABLE_OWNER_PID(i32);
pub struct MIB_UDPROW(i32);
pub struct MIB_UDPROW2(i32);
pub struct MIB_UDPROW_OWNER_MODULE(i32);
pub struct MIB_UDPROW_OWNER_PID(i32);
pub struct MIB_UDPSTATS(i32);
pub struct MIB_UDPSTATS2(i32);
pub struct MIB_UDPTABLE(i32);
pub struct MIB_UDPTABLE2(i32);
pub struct MIB_UDPTABLE_OWNER_MODULE(i32);
pub struct MIB_UDPTABLE_OWNER_PID(i32);
pub struct MIB_UNICASTIPADDRESS_ROW(i32);
pub struct MIB_UNICASTIPADDRESS_TABLE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_USE_CURRENT_FORWARDING: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIB_USE_CURRENT_TTL: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIN_IF_TYPE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const MIXED_NODETYPE: u32 = 4u32;
pub struct NDIS_INTERFACE_INFORMATION(i32);
pub struct NET_ADDRESS_FORMAT(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_IFLUID_UNSPECIFIED: u32 = 0u32;
pub struct NET_IF_ACCESS_TYPE(i32);
pub struct NET_IF_ADMIN_STATUS(i32);
pub struct NET_IF_ALIAS_LH(i32);
pub struct NET_IF_CONNECTION_TYPE(i32);
pub struct NET_IF_DIRECTION_TYPE(i32);
pub struct NET_IF_MEDIA_CONNECT_STATE(i32);
pub struct NET_IF_MEDIA_DUPLEX_STATE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_IF_OID_COMPARTMENT_ID: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_IF_OID_IF_ALIAS: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_IF_OID_IF_ENTRY: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_IF_OID_NETWORK_GUID: u32 = 3u32;
pub struct NET_IF_OPER_STATUS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_IF_OPER_STATUS_DORMANT_LOW_POWER: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_IF_OPER_STATUS_DORMANT_PAUSED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_IF_OPER_STATUS_DOWN_NOT_AUTHENTICATED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_IF_OPER_STATUS_DOWN_NOT_MEDIA_CONNECTED: u32 = 2u32;
pub struct NET_IF_RCV_ADDRESS_LH(i32);
pub struct NET_IF_RCV_ADDRESS_TYPE(i32);
pub struct NET_LUID_LH(i32);
pub struct NET_PHYSICAL_LOCATION_LH(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_SITEID_MAXSYSTEM: u32 = 268435455u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_SITEID_MAXUSER: u32 = 134217727u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_SITEID_UNSPECIFIED: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_STRING_IPV4_ADDRESS: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_STRING_IPV4_NETWORK: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_STRING_IPV4_SERVICE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_STRING_IPV6_ADDRESS: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_STRING_IPV6_ADDRESS_NO_SCOPE: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_STRING_IPV6_NETWORK: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_STRING_IPV6_SERVICE: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_STRING_IPV6_SERVICE_NO_SCOPE: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_STRING_NAMED_ADDRESS: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NET_STRING_NAMED_SERVICE: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NIIF_FILTER_INTERFACE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NIIF_HARDWARE_INTERFACE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NIIF_NDIS_ENDPOINT_INTERFACE: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NIIF_NDIS_ISCSI_INTERFACE: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NIIF_NDIS_RESERVED1: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NIIF_NDIS_RESERVED2: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NIIF_NDIS_RESERVED3: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NIIF_NDIS_RESERVED4: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NIIF_NDIS_WDM_INTERFACE: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const NUMBER_OF_EXPORTED_VARIABLES: u32 = 39u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const PEER_TO_PEER_NODETYPE: u32 = 2u32;
pub struct PFADDRESSTYPE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const PFERROR_BUFFER_TOO_SMALL: u32 = 23002u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const PFERROR_NO_FILTERS_GIVEN: u32 = 23001u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const PFERROR_NO_PF_INTERFACE: u32 = 23000u32;
pub struct PFFORWARD_ACTION(i32);
pub struct PFFRAMETYPE(i32);
pub struct PFLOGFRAME(i32);
pub struct PF_FILTER_DESCRIPTOR(i32);
pub struct PF_FILTER_STATS(i32);
pub struct PF_INTERFACE_STATS(i32);
pub struct PF_LATEBIND_INFO(i32);
pub struct PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK(i32);
pub struct PIPFORWARD_CHANGE_CALLBACK(i32);
pub struct PIPINTERFACE_CHANGE_CALLBACK(i32);
pub struct PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const PROXY_ARP: u32 = 22u32;
pub struct PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK(i32);
pub struct PTEREDO_PORT_CHANGE_CALLBACK(i32);
pub struct PUNICAST_IPADDRESS_CHANGE_CALLBACK(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const ROUTE_LONGER: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const ROUTE_MATCHING: u32 = 31u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const ROUTE_SHORTER: u32 = 33u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const ROUTE_STATE: u32 = 34u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const TCP6_STATS: u32 = 38u32;
pub struct TCPIP_OWNER_MODULE_BASIC_INFO(i32);
pub struct TCPIP_OWNER_MODULE_INFO_CLASS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const TCPIP_OWNING_MODULE_SIZE: u32 = 16u32;
pub struct TCP_BOOLEAN_OPTIONAL(i32);
pub struct TCP_CONNECTION_OFFLOAD_STATE(i32);
pub struct TCP_ESTATS_BANDWIDTH_ROD_v0(i32);
pub struct TCP_ESTATS_BANDWIDTH_RW_v0(i32);
pub struct TCP_ESTATS_DATA_ROD_v0(i32);
pub struct TCP_ESTATS_DATA_RW_v0(i32);
pub struct TCP_ESTATS_FINE_RTT_ROD_v0(i32);
pub struct TCP_ESTATS_FINE_RTT_RW_v0(i32);
pub struct TCP_ESTATS_OBS_REC_ROD_v0(i32);
pub struct TCP_ESTATS_OBS_REC_RW_v0(i32);
pub struct TCP_ESTATS_PATH_ROD_v0(i32);
pub struct TCP_ESTATS_PATH_RW_v0(i32);
pub struct TCP_ESTATS_REC_ROD_v0(i32);
pub struct TCP_ESTATS_REC_RW_v0(i32);
pub struct TCP_ESTATS_SEND_BUFF_ROD_v0(i32);
pub struct TCP_ESTATS_SEND_BUFF_RW_v0(i32);
pub struct TCP_ESTATS_SND_CONG_ROD_v0(i32);
pub struct TCP_ESTATS_SND_CONG_ROS_v0(i32);
pub struct TCP_ESTATS_SND_CONG_RW_v0(i32);
pub struct TCP_ESTATS_SYN_OPTS_ROS_v0(i32);
pub struct TCP_ESTATS_TYPE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const TCP_ROW: u32 = 14u32;
pub struct TCP_RTO_ALGORITHM(i32);
pub struct TCP_SOFT_ERROR(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const TCP_STATS: u32 = 12u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const TCP_TABLE: u32 = 13u32;
pub struct TCP_TABLE_CLASS(i32);
pub struct TUNNEL_TYPE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const UDP6_STATS: u32 = 37u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const UDP_ROW: u32 = 17u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const UDP_STATS: u32 = 15u32;
#[doc = "*Required features: `Win32_NetworkManagement_IpHelper`*"]
pub const UDP_TABLE: u32 = 16u32;
pub struct UDP_TABLE_CLASS(i32);
pub struct arp_send_reply(i32);
pub struct icmp_echo_reply(i32);
pub struct icmp_echo_reply32(i32);
pub struct icmpv6_echo_reply_lh(i32);
pub struct ip_interface_name_info_w2ksp1(i32);
pub struct ip_option_information(i32);
pub struct ip_option_information32(i32);
pub struct tcp_reserve_port_range(i32);
