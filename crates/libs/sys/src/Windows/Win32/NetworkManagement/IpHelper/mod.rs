#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn AddIPAddress(address: u32, ipmask: u32, ifindex: u32, ntecontext: *mut u32, nteinstance: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CancelIPChangeNotify(notifyoverlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelMibChangeNotify2(notificationhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn CaptureInterfaceHardwareCrossTimestamp(interfaceluid: *const NET_LUID_LH, crosstimestamp: *mut INTERFACE_HARDWARE_CROSSTIMESTAMP) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertCompartmentGuidToId(compartmentguid: *const ::windows_sys::core::GUID, compartmentid: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertCompartmentIdToGuid(compartmentid: u32, compartmentguid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceAliasToLuid(interfacealias: ::windows_sys::core::PCWSTR, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceGuidToLuid(interfaceguid: *const ::windows_sys::core::GUID, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceIndexToLuid(interfaceindex: u32, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToAlias(interfaceluid: *const NET_LUID_LH, interfacealias: ::windows_sys::core::PWSTR, length: usize) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToGuid(interfaceluid: *const NET_LUID_LH, interfaceguid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToIndex(interfaceluid: *const NET_LUID_LH, interfaceindex: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToNameA(interfaceluid: *const NET_LUID_LH, interfacename: ::windows_sys::core::PSTR, length: usize) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceLuidToNameW(interfaceluid: *const NET_LUID_LH, interfacename: ::windows_sys::core::PWSTR, length: usize) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceNameToLuidA(interfacename: ::windows_sys::core::PCSTR, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertInterfaceNameToLuidW(interfacename: ::windows_sys::core::PCWSTR, interfaceluid: *mut NET_LUID_LH) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertIpv4MaskToLength(mask: u32, masklength: *mut u8) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertLengthToIpv4Mask(masklength: u32, mask: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn CreateAnycastIpAddressEntry(row: *const MIB_ANYCASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn CreateIpForwardEntry(proute: *const MIB_IPFORWARDROW) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn CreateIpForwardEntry2(row: *const MIB_IPFORWARD_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn CreateIpNetEntry(parpentry: *const MIB_IPNETROW_LH) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn CreateIpNetEntry2(row: *const MIB_IPNET_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn CreatePersistentTcpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn CreatePersistentUdpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn CreateProxyArpEntry(dwaddress: u32, dwmask: u32, dwifindex: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn CreateSortedAddressPairs(sourceaddresslist: *const super::super::Networking::WinSock::SOCKADDR_IN6, sourceaddresscount: u32, destinationaddresslist: *const super::super::Networking::WinSock::SOCKADDR_IN6, destinationaddresscount: u32, addresssortoptions: u32, sortedaddresspairlist: *mut *mut super::super::Networking::WinSock::SOCKADDR_IN6_PAIR, sortedaddresspaircount: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn CreateUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DeleteAnycastIpAddressEntry(row: *const MIB_ANYCASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn DeleteIPAddress(ntecontext: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn DeleteIpForwardEntry(proute: *const MIB_IPFORWARDROW) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DeleteIpForwardEntry2(row: *const MIB_IPFORWARD_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn DeleteIpNetEntry(parpentry: *const MIB_IPNETROW_LH) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DeleteIpNetEntry2(row: *const MIB_IPNET_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn DeletePersistentTcpPortReservation(startport: u16, numberofports: u16) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn DeletePersistentUdpPortReservation(startport: u16, numberofports: u16) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn DeleteProxyArpEntry(dwaddress: u32, dwmask: u32, dwifindex: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DeleteUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn DisableMediaSense(phandle: *mut super::super::Foundation::HANDLE, poverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn EnableRouter(phandle: *mut super::super::Foundation::HANDLE, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn FlushIpNetTable(dwifindex: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushIpNetTable2(family: u16, interfaceindex: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushIpPathTable(family: u16) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn FreeDnsSettings(settings: *mut DNS_SETTINGS);
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn FreeInterfaceDnsSettings(settings: *mut DNS_INTERFACE_SETTINGS);
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn FreeMibTable(memory: *const ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetAdapterIndex(adaptername: ::windows_sys::core::PCWSTR, ifindex: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetAdapterOrderMap() -> *mut IP_ADAPTER_ORDER_MAP;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetAdaptersAddresses(family: super::super::Networking::WinSock::ADDRESS_FAMILY, flags: GET_ADAPTERS_ADDRESSES_FLAGS, reserved: *mut ::core::ffi::c_void, adapteraddresses: *mut IP_ADAPTER_ADDRESSES_LH, sizepointer: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAdaptersInfo(adapterinfo: *mut IP_ADAPTER_INFO, sizepointer: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetAnycastIpAddressEntry(row: *mut MIB_ANYCASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetAnycastIpAddressTable(family: u16, table: *mut *mut MIB_ANYCASTIPADDRESS_TABLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetBestInterface(dwdestaddr: u32, pdwbestifindex: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetBestInterfaceEx(pdestaddr: *const super::super::Networking::WinSock::SOCKADDR, pdwbestifindex: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn GetBestRoute(dwdestaddr: u32, dwsourceaddr: u32, pbestroute: *mut MIB_IPFORWARDROW) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetBestRoute2(interfaceluid: *const NET_LUID_LH, interfaceindex: u32, sourceaddress: *const super::super::Networking::WinSock::SOCKADDR_INET, destinationaddress: *const super::super::Networking::WinSock::SOCKADDR_INET, addresssortoptions: u32, bestroute: *mut MIB_IPFORWARD_ROW2, bestsourceaddress: *mut super::super::Networking::WinSock::SOCKADDR_INET) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetCurrentThreadCompartmentId() -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetCurrentThreadCompartmentScope(compartmentscope: *mut u32, compartmentid: *mut u32);
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetDefaultCompartmentId() -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDnsSettings(settings: *mut DNS_SETTINGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExtendedTcpTable(ptcptable: *mut ::core::ffi::c_void, pdwsize: *mut u32, border: super::super::Foundation::BOOL, ulaf: u32, tableclass: TCP_TABLE_CLASS, reserved: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExtendedUdpTable(pudptable: *mut ::core::ffi::c_void, pdwsize: *mut u32, border: super::super::Foundation::BOOL, ulaf: u32, tableclass: UDP_TABLE_CLASS, reserved: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetFriendlyIfIndex(ifindex: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetIcmpStatistics(statistics: *mut MIB_ICMP) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetIcmpStatisticsEx(statistics: *mut MIB_ICMP_EX_XPSP1, family: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetIfEntry(pifrow: *mut MIB_IFROW) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn GetIfEntry2(row: *mut MIB_IF_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn GetIfEntry2Ex(level: MIB_IF_ENTRY_LEVEL, row: *mut MIB_IF_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIfStackTable(table: *mut *mut MIB_IFSTACK_TABLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIfTable(piftable: *mut MIB_IFTABLE, pdwsize: *mut u32, border: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn GetIfTable2(table: *mut *mut MIB_IF_TABLE2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn GetIfTable2Ex(level: MIB_IF_TABLE_LEVEL, table: *mut *mut MIB_IF_TABLE2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInterfaceActiveTimestampCapabilities(interfaceluid: *const NET_LUID_LH, timestampcapabilites: *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInterfaceDnsSettings(interface: ::windows_sys::core::GUID, settings: *mut DNS_INTERFACE_SETTINGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetInterfaceInfo(piftable: *mut IP_INTERFACE_INFO, dwoutbuflen: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInterfaceSupportedTimestampCapabilities(interfaceluid: *const NET_LUID_LH, timestampcapabilites: *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInvertedIfStackTable(table: *mut *mut MIB_INVERTEDIFSTACK_TABLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIpAddrTable(pipaddrtable: *mut MIB_IPADDRTABLE, pdwsize: *mut u32, border: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetIpErrorString(errorcode: u32, buffer: ::windows_sys::core::PWSTR, size: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpForwardEntry2(row: *mut MIB_IPFORWARD_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpForwardTable(pipforwardtable: *mut MIB_IPFORWARDTABLE, pdwsize: *mut u32, border: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpForwardTable2(family: u16, table: *mut *mut MIB_IPFORWARD_TABLE2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpInterfaceTable(family: u16, table: *mut *mut MIB_IPINTERFACE_TABLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpNetEntry2(row: *mut MIB_IPNET_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIpNetTable(ipnettable: *mut MIB_IPNETTABLE, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpNetTable2(family: u16, table: *mut *mut MIB_IPNET_TABLE2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpNetworkConnectionBandwidthEstimates(interfaceindex: u32, addressfamily: u16, bandwidthestimates: *mut MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpPathEntry(row: *mut MIB_IPPATH_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetIpPathTable(family: u16, table: *mut *mut MIB_IPPATH_TABLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetIpStatistics(statistics: *mut MIB_IPSTATS_LH) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn GetIpStatisticsEx(statistics: *mut MIB_IPSTATS_LH, family: super::super::Networking::WinSock::ADDRESS_FAMILY) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetJobCompartmentId(jobhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetMulticastIpAddressEntry(row: *mut MIB_MULTICASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetMulticastIpAddressTable(family: u16, table: *mut *mut MIB_MULTICASTIPADDRESS_TABLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetNetworkConnectivityHint(connectivityhint: *mut super::super::Networking::WinSock::NL_NETWORK_CONNECTIVITY_HINT) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetNetworkConnectivityHintForInterface(interfaceindex: u32, connectivityhint: *mut super::super::Networking::WinSock::NL_NETWORK_CONNECTIVITY_HINT) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNetworkInformation(networkguid: *const ::windows_sys::core::GUID, compartmentid: *mut u32, siteid: *mut u32, networkname: ::windows_sys::core::PWSTR, length: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNetworkParams(pfixedinfo: *mut FIXED_INFO_W2KSP1, poutbuflen: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetNumberOfInterfaces(pdwnumif: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetOwnerModuleFromPidAndInfo(ulpid: u32, pinfo: *const u64, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetOwnerModuleFromTcp6Entry(ptcpentry: *const MIB_TCP6ROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetOwnerModuleFromTcpEntry(ptcpentry: *const MIB_TCPROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetOwnerModuleFromUdp6Entry(pudpentry: *const MIB_UDP6ROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetOwnerModuleFromUdpEntry(pudpentry: *const MIB_UDPROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPerAdapterInfo(ifindex: u32, pperadapterinfo: *mut IP_PER_ADAPTER_INFO_W2KSP1, poutbuflen: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn GetPerTcp6ConnectionEStats(row: *const MIB_TCP6ROW, estatstype: TCP_ESTATS_TYPE, rw: *mut u8, rwversion: u32, rwsize: u32, ros: *mut u8, rosversion: u32, rossize: u32, rod: *mut u8, rodversion: u32, rodsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetPerTcpConnectionEStats(row: *const MIB_TCPROW_LH, estatstype: TCP_ESTATS_TYPE, rw: *mut u8, rwversion: u32, rwsize: u32, ros: *mut u8, rosversion: u32, rossize: u32, rod: *mut u8, rodversion: u32, rodsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRTTAndHopCount(destipaddress: u32, hopcount: *mut u32, maxhops: u32, rtt: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetSessionCompartmentId(sessionid: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetTcp6Table(tcptable: *mut MIB_TCP6TABLE, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetTcp6Table2(tcptable: *mut MIB_TCP6TABLE2, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetTcpStatistics(statistics: *mut MIB_TCPSTATS_LH) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn GetTcpStatisticsEx(statistics: *mut MIB_TCPSTATS_LH, family: super::super::Networking::WinSock::ADDRESS_FAMILY) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn GetTcpStatisticsEx2(statistics: *mut MIB_TCPSTATS2, family: super::super::Networking::WinSock::ADDRESS_FAMILY) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTcpTable(tcptable: *mut MIB_TCPTABLE, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTcpTable2(tcptable: *mut MIB_TCPTABLE2, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTeredoPort(port: *mut u16) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetUdp6Table(udp6table: *mut MIB_UDP6TABLE, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetUdpStatistics(stats: *mut MIB_UDPSTATS) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn GetUdpStatisticsEx(statistics: *mut MIB_UDPSTATS, family: super::super::Networking::WinSock::ADDRESS_FAMILY) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn GetUdpStatisticsEx2(statistics: *mut MIB_UDPSTATS2, family: super::super::Networking::WinSock::ADDRESS_FAMILY) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUdpTable(udptable: *mut MIB_UDPTABLE, sizepointer: *mut u32, order: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn GetUniDirectionalAdapterInfo(pipifinfo: *mut IP_UNIDIRECTIONAL_ADAPTER_ADDRESS, dwoutbuflen: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetUnicastIpAddressEntry(row: *mut MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn GetUnicastIpAddressTable(family: u16, table: *mut *mut MIB_UNICASTIPADDRESS_TABLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn Icmp6CreateFile() -> IcmpHandle;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn Icmp6ParseReplies(replybuffer: *mut ::core::ffi::c_void, replysize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`, `\"Win32_System_WindowsProgramming\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_WindowsProgramming"))]
    pub fn Icmp6SendEcho2(icmphandle: IcmpHandle, event: super::super::Foundation::HANDLE, apcroutine: super::super::System::WindowsProgramming::PIO_APC_ROUTINE, apccontext: *const ::core::ffi::c_void, sourceaddress: *const super::super::Networking::WinSock::SOCKADDR_IN6, destinationaddress: *const super::super::Networking::WinSock::SOCKADDR_IN6, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IcmpCloseHandle(icmphandle: IcmpHandle) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn IcmpCreateFile() -> IcmpHandle;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn IcmpParseReplies(replybuffer: *mut ::core::ffi::c_void, replysize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn IcmpSendEcho(icmphandle: IcmpHandle, destinationaddress: u32, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn IcmpSendEcho2(icmphandle: IcmpHandle, event: super::super::Foundation::HANDLE, apcroutine: super::super::System::WindowsProgramming::PIO_APC_ROUTINE, apccontext: *const ::core::ffi::c_void, destinationaddress: u32, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn IcmpSendEcho2Ex(icmphandle: IcmpHandle, event: super::super::Foundation::HANDLE, apcroutine: super::super::System::WindowsProgramming::PIO_APC_ROUTINE, apccontext: *const ::core::ffi::c_void, sourceaddress: u32, destinationaddress: u32, requestdata: *const ::core::ffi::c_void, requestsize: u16, requestoptions: *const ip_option_information, replybuffer: *mut ::core::ffi::c_void, replysize: u32, timeout: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn InitializeIpForwardEntry(row: *mut MIB_IPFORWARD_ROW2);
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn InitializeIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW);
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn InitializeUnicastIpAddressEntry(row: *mut MIB_UNICASTIPADDRESS_ROW);
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn IpReleaseAddress(adapterinfo: *const IP_ADAPTER_INDEX_MAP) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn IpRenewAddress(adapterinfo: *const IP_ADAPTER_INDEX_MAP) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn LookupPersistentTcpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn LookupPersistentUdpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NhpAllocateAndGetInterfaceInfoFromStack(pptable: *mut *mut ip_interface_name_info_w2ksp1, pdwcount: *mut u32, border: super::super::Foundation::BOOL, hheap: super::super::Foundation::HANDLE, dwflags: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn NotifyAddrChange(handle: *mut super::super::Foundation::HANDLE, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyIpInterfaceChange(family: u16, callback: PIPINTERFACE_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyNetworkConnectivityHintChange(callback: PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn NotifyRouteChange(handle: *mut super::super::Foundation::HANDLE, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyRouteChange2(addressfamily: u16, callback: PIPFORWARD_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyStableUnicastIpAddressTable(family: u16, table: *mut *mut MIB_UNICASTIPADDRESS_TABLE, callercallback: PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK, callercontext: *const ::core::ffi::c_void, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NotifyTeredoPortChange(callback: PTEREDO_PORT_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NotifyUnicastIpAddressChange(family: u16, callback: PUNICAST_IPADDRESS_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, initialnotification: super::super::Foundation::BOOLEAN, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn PfAddFiltersToInterface(ih: *mut ::core::ffi::c_void, cinfilters: u32, pfiltin: *mut PF_FILTER_DESCRIPTOR, coutfilters: u32, pfiltout: *mut PF_FILTER_DESCRIPTOR, pfhandle: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn PfAddGlobalFilterToInterface(pinterface: *mut ::core::ffi::c_void, gffilter: GLOBAL_FILTER) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn PfBindInterfaceToIPAddress(pinterface: *mut ::core::ffi::c_void, pfattype: PFADDRESSTYPE, ipaddress: *mut u8) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn PfBindInterfaceToIndex(pinterface: *mut ::core::ffi::c_void, dwindex: u32, pfatlinktype: PFADDRESSTYPE, linkipaddress: *mut u8) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PfCreateInterface(dwname: u32, inaction: PFFORWARD_ACTION, outaction: PFFORWARD_ACTION, buselog: super::super::Foundation::BOOL, bmustbeunique: super::super::Foundation::BOOL, ppinterface: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn PfDeleteInterface(pinterface: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn PfDeleteLog() -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PfGetInterfaceStatistics(pinterface: *mut ::core::ffi::c_void, ppfstats: *mut PF_INTERFACE_STATS, pdwbuffersize: *mut u32, fresetcounters: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PfMakeLog(hevent: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn PfRebindFilters(pinterface: *mut ::core::ffi::c_void, platebindinfo: *mut PF_LATEBIND_INFO) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn PfRemoveFilterHandles(pinterface: *mut ::core::ffi::c_void, cfilters: u32, pvhandles: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn PfRemoveFiltersFromInterface(ih: *mut ::core::ffi::c_void, cinfilters: u32, pfiltin: *mut PF_FILTER_DESCRIPTOR, coutfilters: u32, pfiltout: *mut PF_FILTER_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn PfRemoveGlobalFilterFromInterface(pinterface: *mut ::core::ffi::c_void, gffilter: GLOBAL_FILTER) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn PfSetLogBuffer(pbbuffer: *mut u8, dwsize: u32, dwthreshold: u32, dwentries: u32, pdwloggedentries: *mut u32, pdwlostentries: *mut u32, pdwsizeused: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn PfTestPacket(pininterface: *mut ::core::ffi::c_void, poutinterface: *mut ::core::ffi::c_void, cbytes: u32, pbpacket: *mut u8, ppaction: *mut PFFORWARD_ACTION) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn PfUnBindInterface(pinterface: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn RegisterInterfaceTimestampConfigChange(callback: PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, callercontext: *const ::core::ffi::c_void, notificationhandle: *mut HIFTIMESTAMPCHANGE) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn ResolveIpNetEntry2(row: *mut MIB_IPNET_ROW2, sourceaddress: *const super::super::Networking::WinSock::SOCKADDR_INET) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn ResolveNeighbor(networkaddress: *const super::super::Networking::WinSock::SOCKADDR, physicaladdress: *mut ::core::ffi::c_void, physicaladdresslength: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RestoreMediaSense(poverlapped: *const super::super::System::IO::OVERLAPPED, lpdwenablecount: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn SendARP(destip: u32, srcip: u32, pmacaddr: *mut ::core::ffi::c_void, phyaddrlen: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCurrentThreadCompartmentId(compartmentid: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCurrentThreadCompartmentScope(compartmentscope: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDnsSettings(settings: *const DNS_SETTINGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn SetIfEntry(pifrow: *const MIB_IFROW) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetInterfaceDnsSettings(interface: ::windows_sys::core::GUID, settings: *const DNS_INTERFACE_SETTINGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn SetIpForwardEntry(proute: *const MIB_IPFORWARDROW) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn SetIpForwardEntry2(route: *const MIB_IPFORWARD_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn SetIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn SetIpNetEntry(parpentry: *const MIB_IPNETROW_LH) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn SetIpNetEntry2(row: *const MIB_IPNET_ROW2) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn SetIpStatistics(pipstats: *const MIB_IPSTATS_LH) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn SetIpStatisticsEx(statistics: *const MIB_IPSTATS_LH, family: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn SetIpTTL(nttl: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetJobCompartmentId(jobhandle: super::super::Foundation::HANDLE, compartmentid: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetNetworkInformation(networkguid: *const ::windows_sys::core::GUID, compartmentid: u32, networkname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn SetPerTcp6ConnectionEStats(row: *const MIB_TCP6ROW, estatstype: TCP_ESTATS_TYPE, rw: *const u8, rwversion: u32, rwsize: u32, offset: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn SetPerTcpConnectionEStats(row: *const MIB_TCPROW_LH, estatstype: TCP_ESTATS_TYPE, rw: *const u8, rwversion: u32, rwsize: u32, offset: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSessionCompartmentId(sessionid: u32, compartmentid: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn SetTcpEntry(ptcprow: *const MIB_TCPROW_LH) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn SetUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn UnenableRouter(poverlapped: *const super::super::System::IO::OVERLAPPED, lpdwenablecount: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn UnregisterInterfaceTimestampConfigChange(notificationhandle: HIFTIMESTAMPCHANGE);
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn if_indextoname(interfaceindex: u32, interfacename: ::windows_sys::core::PSTR) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
    pub fn if_nametoindex(interfacename: ::windows_sys::core::PCSTR) -> u32;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ANY_SIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const BEST_IF: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const BEST_ROUTE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const BROADCAST_NODETYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DEFAULT_MINIMUM_ENTITIES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DEST_LONGER: u32 = 29u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DEST_MATCHING: u32 = 28u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DEST_SHORTER: u32 = 30u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_DOH_AUTO_UPGRADE_SERVER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_DOH_POLICY_AUTO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_DOH_POLICY_DISABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_DOH_POLICY_NOT_CONFIGURED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_DOH_POLICY_REQUIRED: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct DNS_DOH_SERVER_SETTINGS {
    pub Template: ::windows_sys::core::PWSTR,
    pub Flags: u64,
}
impl ::core::marker::Copy for DNS_DOH_SERVER_SETTINGS {}
impl ::core::clone::Clone for DNS_DOH_SERVER_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_DOH_SERVER_SETTINGS_ENABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_DOH_SERVER_SETTINGS_ENABLE_AUTO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_DOH_SERVER_SETTINGS_FALLBACK_TO_UDP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_ENABLE_DOH: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct DNS_INTERFACE_SETTINGS {
    pub Version: u32,
    pub Flags: u64,
    pub Domain: ::windows_sys::core::PWSTR,
    pub NameServer: ::windows_sys::core::PWSTR,
    pub SearchList: ::windows_sys::core::PWSTR,
    pub RegistrationEnabled: u32,
    pub RegisterAdapterName: u32,
    pub EnableLLMNR: u32,
    pub QueryAdapterName: u32,
    pub ProfileNameServer: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for DNS_INTERFACE_SETTINGS {}
impl ::core::clone::Clone for DNS_INTERFACE_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct DNS_INTERFACE_SETTINGS3 {
    pub Version: u32,
    pub Flags: u64,
    pub Domain: ::windows_sys::core::PWSTR,
    pub NameServer: ::windows_sys::core::PWSTR,
    pub SearchList: ::windows_sys::core::PWSTR,
    pub RegistrationEnabled: u32,
    pub RegisterAdapterName: u32,
    pub EnableLLMNR: u32,
    pub QueryAdapterName: u32,
    pub ProfileNameServer: ::windows_sys::core::PWSTR,
    pub DisableUnconstrainedQueries: u32,
    pub SupplementalSearchList: ::windows_sys::core::PWSTR,
    pub cServerProperties: u32,
    pub ServerProperties: *mut DNS_SERVER_PROPERTY,
    pub cProfileServerProperties: u32,
    pub ProfileServerProperties: *mut DNS_SERVER_PROPERTY,
}
impl ::core::marker::Copy for DNS_INTERFACE_SETTINGS3 {}
impl ::core::clone::Clone for DNS_INTERFACE_SETTINGS3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct DNS_INTERFACE_SETTINGS_EX {
    pub SettingsV1: DNS_INTERFACE_SETTINGS,
    pub DisableUnconstrainedQueries: u32,
    pub SupplementalSearchList: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for DNS_INTERFACE_SETTINGS_EX {}
impl ::core::clone::Clone for DNS_INTERFACE_SETTINGS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_INTERFACE_SETTINGS_VERSION1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_INTERFACE_SETTINGS_VERSION2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_INTERFACE_SETTINGS_VERSION3: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct DNS_SERVER_PROPERTY {
    pub Version: u32,
    pub ServerIndex: u32,
    pub Type: DNS_SERVER_PROPERTY_TYPE,
    pub Property: DNS_SERVER_PROPERTY_TYPES,
}
impl ::core::marker::Copy for DNS_SERVER_PROPERTY {}
impl ::core::clone::Clone for DNS_SERVER_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type DNS_SERVER_PROPERTY_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DnsServerInvalidProperty: DNS_SERVER_PROPERTY_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DnsServerDohProperty: DNS_SERVER_PROPERTY_TYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union DNS_SERVER_PROPERTY_TYPES {
    pub DohSettings: *mut DNS_DOH_SERVER_SETTINGS,
}
impl ::core::marker::Copy for DNS_SERVER_PROPERTY_TYPES {}
impl ::core::clone::Clone for DNS_SERVER_PROPERTY_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SERVER_PROPERTY_VERSION1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct DNS_SETTINGS {
    pub Version: u32,
    pub Flags: u64,
    pub Hostname: ::windows_sys::core::PWSTR,
    pub Domain: ::windows_sys::core::PWSTR,
    pub SearchList: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for DNS_SETTINGS {}
impl ::core::clone::Clone for DNS_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct DNS_SETTINGS2 {
    pub Version: u32,
    pub Flags: u64,
    pub Hostname: ::windows_sys::core::PWSTR,
    pub Domain: ::windows_sys::core::PWSTR,
    pub SearchList: ::windows_sys::core::PWSTR,
    pub SettingFlags: u64,
}
impl ::core::marker::Copy for DNS_SETTINGS2 {}
impl ::core::clone::Clone for DNS_SETTINGS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTINGS_ENABLE_LLMNR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTINGS_QUERY_ADAPTER_NAME: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTINGS_VERSION1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTINGS_VERSION2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_DISABLE_UNCONSTRAINED_QUERIES: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_DOH: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_DOH_PROFILE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_DOMAIN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_HOSTNAME: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_IPV6: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_NAMESERVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_PROFILE_NAMESERVER: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_REGISTER_ADAPTER_NAME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_REGISTRATION_ENABLED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_SEARCHLIST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const DNS_SETTING_SUPPLEMENTAL_SEARCH_LIST: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ERROR_BASE: u32 = 23000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ERROR_IPV6_NOT_IMPLEMENTED: u32 = 23003u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const FD_FLAGS_ALLFLAGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const FD_FLAGS_NOSYN: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FIXED_INFO_W2KSP1 {
    pub HostName: [super::super::Foundation::CHAR; 132],
    pub DomainName: [super::super::Foundation::CHAR; 132],
    pub CurrentDnsServer: *mut IP_ADDR_STRING,
    pub DnsServerList: IP_ADDR_STRING,
    pub NodeType: u32,
    pub ScopeId: [super::super::Foundation::CHAR; 260],
    pub EnableRouting: u32,
    pub EnableProxy: u32,
    pub EnableDns: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FIXED_INFO_W2KSP1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FIXED_INFO_W2KSP1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_SKIP_DNS_INFO: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type GET_ADAPTERS_ADDRESSES_FLAGS = u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_SKIP_UNICAST: GET_ADAPTERS_ADDRESSES_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_SKIP_ANYCAST: GET_ADAPTERS_ADDRESSES_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_SKIP_MULTICAST: GET_ADAPTERS_ADDRESSES_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_SKIP_DNS_SERVER: GET_ADAPTERS_ADDRESSES_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_INCLUDE_PREFIX: GET_ADAPTERS_ADDRESSES_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_SKIP_FRIENDLY_NAME: GET_ADAPTERS_ADDRESSES_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_INCLUDE_WINS_INFO: GET_ADAPTERS_ADDRESSES_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_INCLUDE_GATEWAYS: GET_ADAPTERS_ADDRESSES_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_INCLUDE_ALL_INTERFACES: GET_ADAPTERS_ADDRESSES_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_INCLUDE_ALL_COMPARTMENTS: GET_ADAPTERS_ADDRESSES_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GAA_FLAG_INCLUDE_TUNNEL_BINDINGORDER: GET_ADAPTERS_ADDRESSES_FLAGS = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type GLOBAL_FILTER = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GF_FRAGMENTS: GLOBAL_FILTER = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GF_STRONGHOST: GLOBAL_FILTER = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const GF_FRAGCACHE: GLOBAL_FILTER = 9i32;
pub type HIFTIMESTAMPCHANGE = isize;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const HYBRID_NODETYPE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type ICMP4_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_ECHO_REPLY: ICMP4_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_DST_UNREACH: ICMP4_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_SOURCE_QUENCH: ICMP4_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_REDIRECT: ICMP4_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_ECHO_REQUEST: ICMP4_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_ROUTER_ADVERT: ICMP4_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_ROUTER_SOLICIT: ICMP4_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_TIME_EXCEEDED: ICMP4_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_PARAM_PROB: ICMP4_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_TIMESTAMP_REQUEST: ICMP4_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_TIMESTAMP_REPLY: ICMP4_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_MASK_REQUEST: ICMP4_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP4_MASK_REPLY: ICMP4_TYPE = 18i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_INFOMSG_MASK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type ICMP6_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_DST_UNREACH: ICMP6_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_PACKET_TOO_BIG: ICMP6_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_TIME_EXCEEDED: ICMP6_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_PARAM_PROB: ICMP6_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_ECHO_REQUEST: ICMP6_TYPE = 128i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_ECHO_REPLY: ICMP6_TYPE = 129i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_MEMBERSHIP_QUERY: ICMP6_TYPE = 130i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_MEMBERSHIP_REPORT: ICMP6_TYPE = 131i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_MEMBERSHIP_REDUCTION: ICMP6_TYPE = 132i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ND_ROUTER_SOLICIT: ICMP6_TYPE = 133i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ND_ROUTER_ADVERT: ICMP6_TYPE = 134i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ND_NEIGHBOR_SOLICIT: ICMP6_TYPE = 135i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ND_NEIGHBOR_ADVERT: ICMP6_TYPE = 136i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ND_REDIRECT: ICMP6_TYPE = 137i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP6_V2_MEMBERSHIP_REPORT: ICMP6_TYPE = 143i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ICMP_STATS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type IF_ACCESS_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ACCESS_LOOPBACK: IF_ACCESS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ACCESS_BROADCAST: IF_ACCESS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ACCESS_POINT_TO_POINT: IF_ACCESS_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ACCESS_POINTTOPOINT: IF_ACCESS_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ACCESS_POINT_TO_MULTI_POINT: IF_ACCESS_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ACCESS_POINTTOMULTIPOINT: IF_ACCESS_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type IF_ADMINISTRATIVE_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ADMINISTRATIVE_DISABLED: IF_ADMINISTRATIVE_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ADMINISTRATIVE_ENABLED: IF_ADMINISTRATIVE_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ADMINISTRATIVE_DEMANDDIAL: IF_ADMINISTRATIVE_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ADMIN_STATUS_DOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ADMIN_STATUS_TESTING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ADMIN_STATUS_UP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_CHECK_MCAST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_CHECK_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_CHECK_SEND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_CONNECTION_DEDICATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_CONNECTION_DEMAND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_CONNECTION_PASSIVE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IF_COUNTED_STRING_LH {
    pub Length: u16,
    pub String: [u16; 257],
}
impl ::core::marker::Copy for IF_COUNTED_STRING_LH {}
impl ::core::clone::Clone for IF_COUNTED_STRING_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_MAX_PHYS_ADDRESS_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_MAX_STRING_SIZE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_NUMBER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type IF_OPER_STATUS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IfOperStatusUp: IF_OPER_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IfOperStatusDown: IF_OPER_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IfOperStatusTesting: IF_OPER_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IfOperStatusUnknown: IF_OPER_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IfOperStatusDormant: IF_OPER_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IfOperStatusNotPresent: IF_OPER_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IfOperStatusLowerLayerDown: IF_OPER_STATUS = 7i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IF_PHYSICAL_ADDRESS_LH {
    pub Length: u16,
    pub Address: [u8; 32],
}
impl ::core::marker::Copy for IF_PHYSICAL_ADDRESS_LH {}
impl ::core::clone::Clone for IF_PHYSICAL_ADDRESS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_ROW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_STATUS: u32 = 25u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_A12MPPSWITCH: u32 = 130u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_AAL2: u32 = 187u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_AAL5: u32 = 49u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ADSL: u32 = 94u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_AFLANE_8023: u32 = 59u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_AFLANE_8025: u32 = 60u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ARAP: u32 = 88u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ARCNET: u32 = 35u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ARCNET_PLUS: u32 = 36u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ASYNC: u32 = 84u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM: u32 = 37u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM_DXI: u32 = 105u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM_FUNI: u32 = 106u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM_IMA: u32 = 107u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM_LOGICAL: u32 = 80u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM_RADIO: u32 = 189u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM_SUBINTERFACE: u32 = 134u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM_VCI_ENDPT: u32 = 194u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ATM_VIRTUAL: u32 = 149u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_BASIC_ISDN: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_BGP_POLICY_ACCOUNTING: u32 = 162u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_BSC: u32 = 83u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_CCTEMUL: u32 = 61u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_CES: u32 = 133u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_CHANNEL: u32 = 70u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_CNR: u32 = 85u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_COFFEE: u32 = 132u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_COMPOSITELINK: u32 = 155u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DCN: u32 = 141u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DDN_X25: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DIGITALPOWERLINE: u32 = 138u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DIGITAL_WRAPPER_OVERHEAD_CHANNEL: u32 = 186u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DLSW: u32 = 74u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DOCSCABLE_DOWNSTREAM: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DOCSCABLE_MACLAYER: u32 = 127u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DOCSCABLE_UPSTREAM: u32 = 129u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DS0: u32 = 81u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DS0_BUNDLE: u32 = 82u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DS1: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DS1_FDL: u32 = 170u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DS3: u32 = 30u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DTM: u32 = 140u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DVBRCC_DOWNSTREAM: u32 = 147u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DVBRCC_MACLAYER: u32 = 146u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DVBRCC_UPSTREAM: u32 = 148u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DVB_ASI_IN: u32 = 172u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_DVB_ASI_OUT: u32 = 173u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_E1: u32 = 19u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_EON: u32 = 25u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_EPLRS: u32 = 87u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ESCON: u32 = 73u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ETHERNET_3MBIT: u32 = 26u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ETHERNET_CSMACD: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FAST: u32 = 125u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FASTETHER: u32 = 62u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FASTETHER_FX: u32 = 69u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FDDI: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FIBRECHANNEL: u32 = 56u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FRAMERELAY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FRAMERELAY_INTERCONNECT: u32 = 58u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FRAMERELAY_MPI: u32 = 92u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FRAMERELAY_SERVICE: u32 = 44u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FRF16_MFR_BUNDLE: u32 = 163u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FR_DLCI_ENDPT: u32 = 193u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_FR_FORWARD: u32 = 158u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_G703_2MB: u32 = 67u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_G703_64K: u32 = 66u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_GIGABITETHERNET: u32 = 117u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_GR303_IDT: u32 = 178u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_GR303_RDT: u32 = 177u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_H323_GATEKEEPER: u32 = 164u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_H323_PROXY: u32 = 165u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HDH_1822: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HDLC: u32 = 118u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HDSL2: u32 = 168u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HIPERLAN2: u32 = 183u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HIPPI: u32 = 47u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HIPPIINTERFACE: u32 = 57u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HOSTPAD: u32 = 90u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HSSI: u32 = 46u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_HYPERCHANNEL: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IBM370PARCHAN: u32 = 72u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IDSL: u32 = 154u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IEEE1394: u32 = 144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IEEE80211: u32 = 71u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IEEE80212: u32 = 55u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IEEE802154: u32 = 259u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IEEE80216_WMAN: u32 = 237u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IEEE8023AD_LAG: u32 = 161u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IF_GSN: u32 = 145u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IMT: u32 = 190u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_INTERLEAVE: u32 = 124u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IP: u32 = 126u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IPFORWARD: u32 = 142u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IPOVER_ATM: u32 = 114u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IPOVER_CDLC: u32 = 109u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IPOVER_CLAW: u32 = 110u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IPSWITCH: u32 = 78u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_IS088023_CSMACD: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISDN: u32 = 63u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISDN_S: u32 = 75u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISDN_U: u32 = 76u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISO88022_LLC: u32 = 41u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISO88024_TOKENBUS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISO88025R_DTR: u32 = 86u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISO88025_CRFPRINT: u32 = 98u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISO88025_FIBER: u32 = 115u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISO88025_TOKENRING: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISO88026_MAN: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ISUP: u32 = 179u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_L2_VLAN: u32 = 135u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_L3_IPVLAN: u32 = 136u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_L3_IPXVLAN: u32 = 137u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_LAP_B: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_LAP_D: u32 = 77u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_LAP_F: u32 = 119u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_LOCALTALK: u32 = 42u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MEDIAMAILOVERIP: u32 = 139u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MF_SIGLINK: u32 = 167u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MIO_X25: u32 = 38u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MODEM: u32 = 48u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MPC: u32 = 113u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MPLS: u32 = 166u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MPLS_TUNNEL: u32 = 150u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MSDSL: u32 = 143u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MVL: u32 = 191u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_MYRINET: u32 = 99u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_NFAS: u32 = 175u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_NSIP: u32 = 27u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_OPTICAL_CHANNEL: u32 = 195u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_OPTICAL_TRANSPORT: u32 = 196u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_OTHER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PARA: u32 = 34u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PLC: u32 = 174u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_POS: u32 = 171u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PPP: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PPPMULTILINKBUNDLE: u32 = 108u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PRIMARY_ISDN: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_BWA_P2MP: u32 = 184u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_CNLS: u32 = 89u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_DOCS_WIRELESS_DOWNSTREAM: u32 = 181u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_DOCS_WIRELESS_MACLAYER: u32 = 180u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_DOCS_WIRELESS_UPSTREAM: u32 = 182u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_MULTIPLEXOR: u32 = 54u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_POINT2POINT_SERIAL: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_VIRTUAL: u32 = 53u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROP_WIRELESS_P2P: u32 = 157u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROTEON_10MBIT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_PROTEON_80MBIT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_QLLC: u32 = 68u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_RADIO_MAC: u32 = 188u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_RADSL: u32 = 95u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_REACH_DSL: u32 = 192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_REGULAR_1822: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_RFC1483: u32 = 159u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_RFC877_X25: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_RS232: u32 = 33u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_RSRB: u32 = 79u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SDLC: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SDSL: u32 = 96u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SHDSL: u32 = 169u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SIP: u32 = 31u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SLIP: u32 = 28u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SMDS_DXI: u32 = 43u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SMDS_ICIP: u32 = 52u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SOFTWARE_LOOPBACK: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SONET: u32 = 39u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SONET_OVERHEAD_CHANNEL: u32 = 185u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SONET_PATH: u32 = 50u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SONET_VT: u32 = 51u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SRP: u32 = 151u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_SS7_SIGLINK: u32 = 156u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_STACKTOSTACK: u32 = 111u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_STARLAN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_TDLC: u32 = 116u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_TERMPAD: u32 = 91u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_TR008: u32 = 176u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_TRANSPHDLC: u32 = 123u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_TUNNEL: u32 = 131u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_ULTRA: u32 = 29u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_USB: u32 = 160u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_V11: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_V35: u32 = 45u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_V36: u32 = 65u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_V37: u32 = 120u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VDSL: u32 = 97u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VIRTUALIPADDRESS: u32 = 112u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VOICEOVERATM: u32 = 152u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VOICEOVERFRAMERELAY: u32 = 153u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VOICE_EM: u32 = 100u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VOICE_ENCAP: u32 = 103u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VOICE_FXO: u32 = 101u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VOICE_FXS: u32 = 102u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_VOICE_OVERIP: u32 = 104u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_WWANPP: u32 = 243u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_WWANPP2: u32 = 244u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_X213: u32 = 93u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_X25_HUNTGROUP: u32 = 122u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_X25_MLP: u32 = 121u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_X25_PLE: u32 = 40u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_TYPE_XBOX_WIRELESS: u32 = 281u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct INTERFACE_HARDWARE_CROSSTIMESTAMP {
    pub SystemTimestamp1: u64,
    pub HardwareClockTimestamp: u64,
    pub SystemTimestamp2: u64,
}
impl ::core::marker::Copy for INTERFACE_HARDWARE_CROSSTIMESTAMP {}
impl ::core::clone::Clone for INTERFACE_HARDWARE_CROSSTIMESTAMP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {
    pub PtpV2OverUdpIPv4EventMessageReceive: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv4AllMessageReceive: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv4EventMessageTransmit: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv4AllMessageTransmit: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv6EventMessageReceive: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv6AllMessageReceive: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv6EventMessageTransmit: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv6AllMessageTransmit: super::super::Foundation::BOOLEAN,
    pub AllReceive: super::super::Foundation::BOOLEAN,
    pub AllTransmit: super::super::Foundation::BOOLEAN,
    pub TaggedTransmit: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    pub AllReceive: super::super::Foundation::BOOLEAN,
    pub AllTransmit: super::super::Foundation::BOOLEAN,
    pub TaggedTransmit: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERFACE_TIMESTAMP_CAPABILITIES {
    pub HardwareClockFrequencyHz: u64,
    pub SupportsCrossTimestamp: super::super::Foundation::BOOLEAN,
    pub HardwareCapabilities: INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES,
    pub SoftwareCapabilities: INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTERFACE_TIMESTAMP_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTERFACE_TIMESTAMP_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type INTERNAL_IF_OPER_STATUS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_OPER_STATUS_NON_OPERATIONAL: INTERNAL_IF_OPER_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_OPER_STATUS_UNREACHABLE: INTERNAL_IF_OPER_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_OPER_STATUS_DISCONNECTED: INTERNAL_IF_OPER_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_OPER_STATUS_CONNECTING: INTERNAL_IF_OPER_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_OPER_STATUS_CONNECTED: INTERNAL_IF_OPER_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IF_OPER_STATUS_OPERATIONAL: INTERNAL_IF_OPER_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IOCTL_ARP_SEND_REQUEST: u32 = 103u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IOCTL_IP_ADDCHANGE_NOTIFY_REQUEST: u32 = 102u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IOCTL_IP_GET_BEST_INTERFACE: u32 = 105u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IOCTL_IP_INTERFACE_INFO: u32 = 104u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IOCTL_IP_RTCHANGE_NOTIFY_REQUEST: u32 = 101u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IOCTL_IP_UNIDIRECTIONAL_ADAPTER_ADDRESS: u32 = 106u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP6_STATS: u32 = 36u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IPRTRMGR_PID: u32 = 10000u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IPV6_ADDRESS_EX {
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u16; 8],
    pub sin6_scope_id: u32,
}
impl ::core::marker::Copy for IPV6_ADDRESS_EX {}
impl ::core::clone::Clone for IPV6_ADDRESS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IPV6_GLOBAL_INFO: u32 = 4294901775u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IPV6_ROUTE_INFO: u32 = 4294901776u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_ADDRESSES_LH {
    pub Anonymous1: IP_ADAPTER_ADDRESSES_LH_0,
    pub Next: *mut IP_ADAPTER_ADDRESSES_LH,
    pub AdapterName: ::windows_sys::core::PSTR,
    pub FirstUnicastAddress: *mut IP_ADAPTER_UNICAST_ADDRESS_LH,
    pub FirstAnycastAddress: *mut IP_ADAPTER_ANYCAST_ADDRESS_XP,
    pub FirstMulticastAddress: *mut IP_ADAPTER_MULTICAST_ADDRESS_XP,
    pub FirstDnsServerAddress: *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    pub DnsSuffix: ::windows_sys::core::PWSTR,
    pub Description: ::windows_sys::core::PWSTR,
    pub FriendlyName: ::windows_sys::core::PWSTR,
    pub PhysicalAddress: [u8; 8],
    pub PhysicalAddressLength: u32,
    pub Anonymous2: IP_ADAPTER_ADDRESSES_LH_1,
    pub Mtu: u32,
    pub IfType: u32,
    pub OperStatus: IF_OPER_STATUS,
    pub Ipv6IfIndex: u32,
    pub ZoneIndices: [u32; 16],
    pub FirstPrefix: *mut IP_ADAPTER_PREFIX_XP,
    pub TransmitLinkSpeed: u64,
    pub ReceiveLinkSpeed: u64,
    pub FirstWinsServerAddress: *mut IP_ADAPTER_WINS_SERVER_ADDRESS_LH,
    pub FirstGatewayAddress: *mut IP_ADAPTER_GATEWAY_ADDRESS_LH,
    pub Ipv4Metric: u32,
    pub Ipv6Metric: u32,
    pub Luid: NET_LUID_LH,
    pub Dhcpv4Server: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub CompartmentId: u32,
    pub NetworkGuid: ::windows_sys::core::GUID,
    pub ConnectionType: NET_IF_CONNECTION_TYPE,
    pub TunnelType: TUNNEL_TYPE,
    pub Dhcpv6Server: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub Dhcpv6ClientDuid: [u8; 130],
    pub Dhcpv6ClientDuidLength: u32,
    pub Dhcpv6Iaid: u32,
    pub FirstDnsSuffix: *mut IP_ADAPTER_DNS_SUFFIX,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ADDRESSES_LH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ADDRESSES_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_ADDRESSES_LH_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_ADDRESSES_LH_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ADDRESSES_LH_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ADDRESSES_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_ADDRESSES_LH_0_0 {
    pub Length: u32,
    pub IfIndex: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ADDRESSES_LH_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ADDRESSES_LH_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_ADDRESSES_LH_1 {
    pub Flags: u32,
    pub Anonymous: IP_ADAPTER_ADDRESSES_LH_1_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ADDRESSES_LH_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ADDRESSES_LH_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_ADDRESSES_LH_1_0 {
    pub _bitfield: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ADDRESSES_LH_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ADDRESSES_LH_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_ADDRESSES_XP {
    pub Anonymous: IP_ADAPTER_ADDRESSES_XP_0,
    pub Next: *mut IP_ADAPTER_ADDRESSES_XP,
    pub AdapterName: ::windows_sys::core::PSTR,
    pub FirstUnicastAddress: *mut IP_ADAPTER_UNICAST_ADDRESS_XP,
    pub FirstAnycastAddress: *mut IP_ADAPTER_ANYCAST_ADDRESS_XP,
    pub FirstMulticastAddress: *mut IP_ADAPTER_MULTICAST_ADDRESS_XP,
    pub FirstDnsServerAddress: *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    pub DnsSuffix: ::windows_sys::core::PWSTR,
    pub Description: ::windows_sys::core::PWSTR,
    pub FriendlyName: ::windows_sys::core::PWSTR,
    pub PhysicalAddress: [u8; 8],
    pub PhysicalAddressLength: u32,
    pub Flags: u32,
    pub Mtu: u32,
    pub IfType: u32,
    pub OperStatus: IF_OPER_STATUS,
    pub Ipv6IfIndex: u32,
    pub ZoneIndices: [u32; 16],
    pub FirstPrefix: *mut IP_ADAPTER_PREFIX_XP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ADDRESSES_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ADDRESSES_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_ADDRESSES_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_ADDRESSES_XP_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ADDRESSES_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ADDRESSES_XP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_ADDRESSES_XP_0_0 {
    pub Length: u32,
    pub IfIndex: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ADDRESSES_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ADDRESSES_XP_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_ADDRESS_DNS_ELIGIBLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_ADDRESS_TRANSIENT: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_ANYCAST_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_ANYCAST_ADDRESS_XP_0,
    pub Next: *mut IP_ADAPTER_ANYCAST_ADDRESS_XP,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ANYCAST_ADDRESS_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ANYCAST_ADDRESS_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_DDNS_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_DHCP_ENABLED: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0,
    pub Next: *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_DNS_SERVER_ADDRESS_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {
    pub Length: u32,
    pub Reserved: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IP_ADAPTER_DNS_SUFFIX {
    pub Next: *mut IP_ADAPTER_DNS_SUFFIX,
    pub String: [u16; 256],
}
impl ::core::marker::Copy for IP_ADAPTER_DNS_SUFFIX {}
impl ::core::clone::Clone for IP_ADAPTER_DNS_SUFFIX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_GATEWAY_ADDRESS_LH {
    pub Anonymous: IP_ADAPTER_GATEWAY_ADDRESS_LH_0,
    pub Next: *mut IP_ADAPTER_GATEWAY_ADDRESS_LH,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_GATEWAY_ADDRESS_LH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_GATEWAY_ADDRESS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {
    pub Length: u32,
    pub Reserved: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IP_ADAPTER_INDEX_MAP {
    pub Index: u32,
    pub Name: [u16; 128],
}
impl ::core::marker::Copy for IP_ADAPTER_INDEX_MAP {}
impl ::core::clone::Clone for IP_ADAPTER_INDEX_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IP_ADAPTER_INFO {
    pub Next: *mut IP_ADAPTER_INFO,
    pub ComboIndex: u32,
    pub AdapterName: [super::super::Foundation::CHAR; 260],
    pub Description: [super::super::Foundation::CHAR; 132],
    pub AddressLength: u32,
    pub Address: [u8; 8],
    pub Index: u32,
    pub Type: u32,
    pub DhcpEnabled: u32,
    pub CurrentIpAddress: *mut IP_ADDR_STRING,
    pub IpAddressList: IP_ADDR_STRING,
    pub GatewayList: IP_ADDR_STRING,
    pub DhcpServer: IP_ADDR_STRING,
    pub HaveWins: super::super::Foundation::BOOL,
    pub PrimaryWinsServer: IP_ADDR_STRING,
    pub SecondaryWinsServer: IP_ADDR_STRING,
    pub LeaseObtained: i64,
    pub LeaseExpires: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IP_ADAPTER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IP_ADAPTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_IPV4_ENABLED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_IPV6_ENABLED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_IPV6_MANAGE_ADDRESS_CONFIG: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_IPV6_OTHER_STATEFUL_CONFIG: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_MULTICAST_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_MULTICAST_ADDRESS_XP_0,
    pub Next: *mut IP_ADAPTER_MULTICAST_ADDRESS_XP,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_MULTICAST_ADDRESS_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_MULTICAST_ADDRESS_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_NETBIOS_OVER_TCPIP_ENABLED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_NO_MULTICAST: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IP_ADAPTER_ORDER_MAP {
    pub NumAdapters: u32,
    pub AdapterOrder: [u32; 1],
}
impl ::core::marker::Copy for IP_ADAPTER_ORDER_MAP {}
impl ::core::clone::Clone for IP_ADAPTER_ORDER_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_PREFIX_XP {
    pub Anonymous: IP_ADAPTER_PREFIX_XP_0,
    pub Next: *mut IP_ADAPTER_PREFIX_XP,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub PrefixLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_PREFIX_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_PREFIX_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_PREFIX_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_PREFIX_XP_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_PREFIX_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_PREFIX_XP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_PREFIX_XP_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_PREFIX_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_PREFIX_XP_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_RECEIVE_ONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADAPTER_REGISTER_ADAPTER_SUFFIX: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_UNICAST_ADDRESS_LH {
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_LH_0,
    pub Next: *mut IP_ADAPTER_UNICAST_ADDRESS_LH,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub PrefixOrigin: super::super::Networking::WinSock::NL_PREFIX_ORIGIN,
    pub SuffixOrigin: super::super::Networking::WinSock::NL_SUFFIX_ORIGIN,
    pub DadState: super::super::Networking::WinSock::NL_DAD_STATE,
    pub ValidLifetime: u32,
    pub PreferredLifetime: u32,
    pub LeaseLifetime: u32,
    pub OnLinkPrefixLength: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_UNICAST_ADDRESS_LH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_UNICAST_ADDRESS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_UNICAST_ADDRESS_LH_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_LH_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_UNICAST_ADDRESS_LH_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_UNICAST_ADDRESS_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_UNICAST_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_XP_0,
    pub Next: *mut IP_ADAPTER_UNICAST_ADDRESS_XP,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub PrefixOrigin: super::super::Networking::WinSock::NL_PREFIX_ORIGIN,
    pub SuffixOrigin: super::super::Networking::WinSock::NL_SUFFIX_ORIGIN,
    pub DadState: super::super::Networking::WinSock::NL_DAD_STATE,
    pub ValidLifetime: u32,
    pub PreferredLifetime: u32,
    pub LeaseLifetime: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_UNICAST_ADDRESS_XP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_UNICAST_ADDRESS_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_UNICAST_ADDRESS_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_XP_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_UNICAST_ADDRESS_XP_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_UNICAST_ADDRESS_XP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_UNICAST_ADDRESS_XP_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_UNICAST_ADDRESS_XP_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_UNICAST_ADDRESS_XP_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    pub Anonymous: IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0,
    pub Next: *mut IP_ADAPTER_WINS_SERVER_ADDRESS_LH,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_WINS_SERVER_ADDRESS_LH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {
    pub Length: u32,
    pub Reserved: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct IP_ADDRESS_PREFIX {
    pub Prefix: super::super::Networking::WinSock::SOCKADDR_INET,
    pub PrefixLength: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for IP_ADDRESS_PREFIX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for IP_ADDRESS_PREFIX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IP_ADDRESS_STRING {
    pub String: [super::super::Foundation::CHAR; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IP_ADDRESS_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IP_ADDRESS_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADDRROW: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADDRTABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADDR_ADDED: u32 = 11023u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ADDR_DELETED: u32 = 11019u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IP_ADDR_STRING {
    pub Next: *mut IP_ADDR_STRING,
    pub IpAddress: IP_ADDRESS_STRING,
    pub IpMask: IP_ADDRESS_STRING,
    pub Context: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IP_ADDR_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IP_ADDR_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_BAD_DESTINATION: u32 = 11018u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_BAD_HEADER: u32 = 11042u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_BAD_OPTION: u32 = 11007u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_BAD_REQ: u32 = 11011u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_BAD_ROUTE: u32 = 11012u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_BIND_ADAPTER: u32 = 11026u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_BUF_TOO_SMALL: u32 = 11001u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEMAND_DIAL_FILTER_INFO: u32 = 4294901769u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEMAND_DIAL_FILTER_INFO_V6: u32 = 4294901779u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_ADDR_UNREACHABLE: u32 = 11003u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_HOST_UNREACHABLE: u32 = 11003u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_NET_UNREACHABLE: u32 = 11002u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_NO_ROUTE: u32 = 11002u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_PORT_UNREACHABLE: u32 = 11005u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_PROHIBITED: u32 = 11004u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_PROT_UNREACHABLE: u32 = 11004u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_SCOPE_MISMATCH: u32 = 11045u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEST_UNREACHABLE: u32 = 11040u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DEVICE_DOES_NOT_EXIST: u32 = 11028u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DUPLICATE_ADDRESS: u32 = 11029u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_DUPLICATE_IPADD: u32 = 11034u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_EXPORT_INCLUDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_FILTER_ENABLE_INFO: u32 = 4294901781u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_FILTER_ENABLE_INFO_V6: u32 = 4294901782u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_FLAG_DF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_FLAG_REVERSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_FORWARDNUMBER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_FORWARDROW: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_FORWARDTABLE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_GENERAL_FAILURE: u32 = 11050u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_GENERAL_INFO_BASE: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_GLOBAL_INFO: u32 = 4294901763u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_HOP_LIMIT_EXCEEDED: u32 = 11013u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_HW_ERROR: u32 = 11008u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ICMP_ERROR: u32 = 11044u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_IFFILTER_INFO: u32 = 4294901773u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_IFFILTER_INFO_V6: u32 = 4294901780u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IP_INTERFACE_INFO {
    pub NumAdapters: i32,
    pub Adapter: [IP_ADAPTER_INDEX_MAP; 1],
}
impl ::core::marker::Copy for IP_INTERFACE_INFO {}
impl ::core::clone::Clone for IP_INTERFACE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_INTERFACE_METRIC_CHANGE: u32 = 11030u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_INTERFACE_STATUS_INFO: u32 = 4294901764u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_INTERFACE_WOL_CAPABILITY_CHANGE: u32 = 11033u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_IN_FILTER_INFO: u32 = 4294901761u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_IN_FILTER_INFO_V6: u32 = 4294901777u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_IPINIP_CFG_INFO: u32 = 4294901772u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_MCAST_BOUNDARY_INFO: u32 = 4294901771u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IP_MCAST_COUNTER_INFO {
    pub InMcastOctets: u64,
    pub OutMcastOctets: u64,
    pub InMcastPkts: u64,
    pub OutMcastPkts: u64,
}
impl ::core::marker::Copy for IP_MCAST_COUNTER_INFO {}
impl ::core::clone::Clone for IP_MCAST_COUNTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_MCAST_HEARBEAT_INFO: u32 = 4294901770u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_MCAST_LIMIT_INFO: u32 = 4294901774u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_MEDIA_CONNECT: u32 = 11024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_MEDIA_DISCONNECT: u32 = 11025u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_MTU_CHANGE: u32 = 11021u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_NEGOTIATING_IPSEC: u32 = 11032u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_NETROW: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_NETTABLE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_NO_RESOURCES: u32 = 11006u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_OPTION_TOO_BIG: u32 = 11017u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_OUT_FILTER_INFO: u32 = 4294901762u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_OUT_FILTER_INFO_V6: u32 = 4294901778u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_PACKET_TOO_BIG: u32 = 11009u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_PARAMETER_PROBLEM: u32 = 11015u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_PARAM_PROBLEM: u32 = 11015u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_PENDING: u32 = 11255u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IP_PER_ADAPTER_INFO_W2KSP1 {
    pub AutoconfigEnabled: u32,
    pub AutoconfigActive: u32,
    pub CurrentDnsServer: *mut IP_ADDR_STRING,
    pub DnsServerList: IP_ADDR_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IP_PER_ADAPTER_INFO_W2KSP1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IP_PER_ADAPTER_INFO_W2KSP1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_PROT_PRIORITY_INFO: u32 = 4294901766u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_PROT_PRIORITY_INFO_EX: u32 = 4294901783u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_REASSEMBLY_TIME_EXCEEDED: u32 = 11014u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_RECONFIG_SECFLTR: u32 = 11031u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_REQ_TIMED_OUT: u32 = 11010u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ROUTER_DISC_INFO: u32 = 4294901767u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ROUTER_MANAGER_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_ROUTE_INFO: u32 = 4294901765u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_SOURCE_QUENCH: u32 = 11016u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_SPEC_MTU_CHANGE: u32 = 11020u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_STATS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_STATUS_BASE: u32 = 11000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_TIME_EXCEEDED: u32 = 11041u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_TTL_EXPIRED_REASSEM: u32 = 11014u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_TTL_EXPIRED_TRANSIT: u32 = 11013u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_UNBIND_ADAPTER: u32 = 11027u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    pub NumAdapters: u32,
    pub Address: [u32; 1],
}
impl ::core::marker::Copy for IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {}
impl ::core::clone::Clone for IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_UNLOAD: u32 = 11022u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const IP_UNRECOGNIZED_NEXT_HEADER: u32 = 11043u32;
pub type IcmpHandle = isize;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const LB_DST_ADDR_USE_DSTADDR_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const LB_DST_ADDR_USE_SRCADDR_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const LB_DST_MASK_LATE_FLAG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const LB_SRC_ADDR_USE_DSTADDR_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const LB_SRC_ADDR_USE_SRCADDR_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const LB_SRC_MASK_LATE_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAXLEN_IFDESCR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAXLEN_PHYSADDR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_ADAPTER_ADDRESS_LENGTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_ADAPTER_DESCRIPTION_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_ADAPTER_NAME: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_ADAPTER_NAME_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_DHCPV6_DUID_LENGTH: u32 = 130u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_DNS_SUFFIX_STRING_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_DOMAIN_NAME_LEN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_HOSTNAME_LEN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_IF_TYPE: u32 = 281u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_INTERFACE_NAME_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_IP_STATUS: u32 = 11050u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_MIB_OFFSET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_OPT_SIZE: u32 = 40u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_SCOPE_ID_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MAX_SCOPE_NAME_LEN: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MCAST_BOUNDARY: u32 = 26u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MCAST_GLOBAL: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MCAST_IF_ENTRY: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MCAST_MFE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MCAST_MFE_STATS: u32 = 19u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MCAST_MFE_STATS_EX: u32 = 35u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MCAST_SCOPE: u32 = 27u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIBICMPINFO {
    pub icmpInStats: MIBICMPSTATS,
    pub icmpOutStats: MIBICMPSTATS,
}
impl ::core::marker::Copy for MIBICMPINFO {}
impl ::core::clone::Clone for MIBICMPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIBICMPSTATS {
    pub dwMsgs: u32,
    pub dwErrors: u32,
    pub dwDestUnreachs: u32,
    pub dwTimeExcds: u32,
    pub dwParmProbs: u32,
    pub dwSrcQuenchs: u32,
    pub dwRedirects: u32,
    pub dwEchos: u32,
    pub dwEchoReps: u32,
    pub dwTimestamps: u32,
    pub dwTimestampReps: u32,
    pub dwAddrMasks: u32,
    pub dwAddrMaskReps: u32,
}
impl ::core::marker::Copy for MIBICMPSTATS {}
impl ::core::clone::Clone for MIBICMPSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIBICMPSTATS_EX_XPSP1 {
    pub dwMsgs: u32,
    pub dwErrors: u32,
    pub rgdwTypeCount: [u32; 256],
}
impl ::core::marker::Copy for MIBICMPSTATS_EX_XPSP1 {}
impl ::core::clone::Clone for MIBICMPSTATS_EX_XPSP1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_ANYCASTIPADDRESS_ROW {
    pub Address: super::super::Networking::WinSock::SOCKADDR_INET,
    pub InterfaceLuid: NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub ScopeId: super::super::Networking::WinSock::SCOPE_ID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_ANYCASTIPADDRESS_ROW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_ANYCASTIPADDRESS_ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_ANYCASTIPADDRESS_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_ANYCASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_ANYCASTIPADDRESS_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_ANYCASTIPADDRESS_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_BEST_IF {
    pub dwDestAddr: u32,
    pub dwIfIndex: u32,
}
impl ::core::marker::Copy for MIB_BEST_IF {}
impl ::core::clone::Clone for MIB_BEST_IF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_BOUNDARYROW {
    pub dwGroupAddress: u32,
    pub dwGroupMask: u32,
}
impl ::core::marker::Copy for MIB_BOUNDARYROW {}
impl ::core::clone::Clone for MIB_BOUNDARYROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_ICMP {
    pub stats: MIBICMPINFO,
}
impl ::core::marker::Copy for MIB_ICMP {}
impl ::core::clone::Clone for MIB_ICMP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_ICMP_EX_XPSP1 {
    pub icmpInStats: MIBICMPSTATS_EX_XPSP1,
    pub icmpOutStats: MIBICMPSTATS_EX_XPSP1,
}
impl ::core::marker::Copy for MIB_ICMP_EX_XPSP1 {}
impl ::core::clone::Clone for MIB_ICMP_EX_XPSP1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IFNUMBER {
    pub dwValue: u32,
}
impl ::core::marker::Copy for MIB_IFNUMBER {}
impl ::core::clone::Clone for MIB_IFNUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IFROW {
    pub wszName: [u16; 256],
    pub dwIndex: u32,
    pub dwType: u32,
    pub dwMtu: u32,
    pub dwSpeed: u32,
    pub dwPhysAddrLen: u32,
    pub bPhysAddr: [u8; 8],
    pub dwAdminStatus: u32,
    pub dwOperStatus: INTERNAL_IF_OPER_STATUS,
    pub dwLastChange: u32,
    pub dwInOctets: u32,
    pub dwInUcastPkts: u32,
    pub dwInNUcastPkts: u32,
    pub dwInDiscards: u32,
    pub dwInErrors: u32,
    pub dwInUnknownProtos: u32,
    pub dwOutOctets: u32,
    pub dwOutUcastPkts: u32,
    pub dwOutNUcastPkts: u32,
    pub dwOutDiscards: u32,
    pub dwOutErrors: u32,
    pub dwOutQLen: u32,
    pub dwDescrLen: u32,
    pub bDescr: [u8; 256],
}
impl ::core::marker::Copy for MIB_IFROW {}
impl ::core::clone::Clone for MIB_IFROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IFSTACK_ROW {
    pub HigherLayerInterfaceIndex: u32,
    pub LowerLayerInterfaceIndex: u32,
}
impl ::core::marker::Copy for MIB_IFSTACK_ROW {}
impl ::core::clone::Clone for MIB_IFSTACK_ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IFSTACK_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_IFSTACK_ROW; 1],
}
impl ::core::marker::Copy for MIB_IFSTACK_TABLE {}
impl ::core::clone::Clone for MIB_IFSTACK_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIB_IFSTATUS {
    pub dwIfIndex: u32,
    pub dwAdminStatus: u32,
    pub dwOperationalStatus: u32,
    pub bMHbeatActive: super::super::Foundation::BOOL,
    pub bMHbeatAlive: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIB_IFSTATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIB_IFSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IFTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IFROW; 1],
}
impl ::core::marker::Copy for MIB_IFTABLE {}
impl ::core::clone::Clone for MIB_IFTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_ADMIN_STATUS_DOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_ADMIN_STATUS_TESTING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_ADMIN_STATUS_UP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type MIB_IF_ENTRY_LEVEL = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibIfEntryNormal: MIB_IF_ENTRY_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibIfEntryNormalWithoutStatistics: MIB_IF_ENTRY_LEVEL = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct MIB_IF_ROW2 {
    pub InterfaceLuid: NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub InterfaceGuid: ::windows_sys::core::GUID,
    pub Alias: [u16; 257],
    pub Description: [u16; 257],
    pub PhysicalAddressLength: u32,
    pub PhysicalAddress: [u8; 32],
    pub PermanentPhysicalAddress: [u8; 32],
    pub Mtu: u32,
    pub Type: u32,
    pub TunnelType: TUNNEL_TYPE,
    pub MediaType: super::Ndis::NDIS_MEDIUM,
    pub PhysicalMediumType: super::Ndis::NDIS_PHYSICAL_MEDIUM,
    pub AccessType: NET_IF_ACCESS_TYPE,
    pub DirectionType: NET_IF_DIRECTION_TYPE,
    pub InterfaceAndOperStatusFlags: MIB_IF_ROW2_0,
    pub OperStatus: IF_OPER_STATUS,
    pub AdminStatus: NET_IF_ADMIN_STATUS,
    pub MediaConnectState: NET_IF_MEDIA_CONNECT_STATE,
    pub NetworkGuid: ::windows_sys::core::GUID,
    pub ConnectionType: NET_IF_CONNECTION_TYPE,
    pub TransmitLinkSpeed: u64,
    pub ReceiveLinkSpeed: u64,
    pub InOctets: u64,
    pub InUcastPkts: u64,
    pub InNUcastPkts: u64,
    pub InDiscards: u64,
    pub InErrors: u64,
    pub InUnknownProtos: u64,
    pub InUcastOctets: u64,
    pub InMulticastOctets: u64,
    pub InBroadcastOctets: u64,
    pub OutOctets: u64,
    pub OutUcastPkts: u64,
    pub OutNUcastPkts: u64,
    pub OutDiscards: u64,
    pub OutErrors: u64,
    pub OutUcastOctets: u64,
    pub OutMulticastOctets: u64,
    pub OutBroadcastOctets: u64,
    pub OutQLen: u64,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for MIB_IF_ROW2 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for MIB_IF_ROW2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct MIB_IF_ROW2_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for MIB_IF_ROW2_0 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for MIB_IF_ROW2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct MIB_IF_TABLE2 {
    pub NumEntries: u32,
    pub Table: [MIB_IF_ROW2; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for MIB_IF_TABLE2 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for MIB_IF_TABLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type MIB_IF_TABLE_LEVEL = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibIfTableNormal: MIB_IF_TABLE_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibIfTableRaw: MIB_IF_TABLE_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibIfTableNormalWithoutStatistics: MIB_IF_TABLE_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_TYPE_ETHERNET: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_TYPE_FDDI: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_TYPE_LOOPBACK: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_TYPE_OTHER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_TYPE_PPP: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_TYPE_SLIP: u32 = 28u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IF_TYPE_TOKENRING: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_INVALID_TEREDO_PORT_NUMBER: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_INVERTEDIFSTACK_ROW {
    pub LowerLayerInterfaceIndex: u32,
    pub HigherLayerInterfaceIndex: u32,
}
impl ::core::marker::Copy for MIB_INVERTEDIFSTACK_ROW {}
impl ::core::clone::Clone for MIB_INVERTEDIFSTACK_ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_INVERTEDIFSTACK_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_INVERTEDIFSTACK_ROW; 1],
}
impl ::core::marker::Copy for MIB_INVERTEDIFSTACK_TABLE {}
impl ::core::clone::Clone for MIB_INVERTEDIFSTACK_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPADDRROW_W2K {
    pub dwAddr: u32,
    pub dwIndex: u32,
    pub dwMask: u32,
    pub dwBCastAddr: u32,
    pub dwReasmSize: u32,
    pub unused1: u16,
    pub unused2: u16,
}
impl ::core::marker::Copy for MIB_IPADDRROW_W2K {}
impl ::core::clone::Clone for MIB_IPADDRROW_W2K {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPADDRROW_XP {
    pub dwAddr: u32,
    pub dwIndex: u32,
    pub dwMask: u32,
    pub dwBCastAddr: u32,
    pub dwReasmSize: u32,
    pub unused1: u16,
    pub wType: u16,
}
impl ::core::marker::Copy for MIB_IPADDRROW_XP {}
impl ::core::clone::Clone for MIB_IPADDRROW_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPADDRTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPADDRROW_XP; 1],
}
impl ::core::marker::Copy for MIB_IPADDRTABLE {}
impl ::core::clone::Clone for MIB_IPADDRTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPADDR_DELETED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPADDR_DISCONNECTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPADDR_DNS_ELIGIBLE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPADDR_DYNAMIC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPADDR_PRIMARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPADDR_TRANSIENT: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_IPDESTROW {
    pub ForwardRow: MIB_IPFORWARDROW,
    pub dwForwardPreference: u32,
    pub dwForwardViewSet: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_IPDESTROW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_IPDESTROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_IPDESTTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPDESTROW; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_IPDESTTABLE {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_IPDESTTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPFORWARDNUMBER {
    pub dwValue: u32,
}
impl ::core::marker::Copy for MIB_IPFORWARDNUMBER {}
impl ::core::clone::Clone for MIB_IPFORWARDNUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_IPFORWARDROW {
    pub dwForwardDest: u32,
    pub dwForwardMask: u32,
    pub dwForwardPolicy: u32,
    pub dwForwardNextHop: u32,
    pub dwForwardIfIndex: u32,
    pub Anonymous1: MIB_IPFORWARDROW_0,
    pub Anonymous2: MIB_IPFORWARDROW_1,
    pub dwForwardAge: u32,
    pub dwForwardNextHopAS: u32,
    pub dwForwardMetric1: u32,
    pub dwForwardMetric2: u32,
    pub dwForwardMetric3: u32,
    pub dwForwardMetric4: u32,
    pub dwForwardMetric5: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_IPFORWARDROW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_IPFORWARDROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union MIB_IPFORWARDROW_0 {
    pub dwForwardType: u32,
    pub ForwardType: MIB_IPFORWARD_TYPE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_IPFORWARDROW_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_IPFORWARDROW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union MIB_IPFORWARDROW_1 {
    pub dwForwardProto: u32,
    pub ForwardProto: super::super::Networking::WinSock::NL_ROUTE_PROTOCOL,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_IPFORWARDROW_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_IPFORWARDROW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_IPFORWARDTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPFORWARDROW; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_IPFORWARDTABLE {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_IPFORWARDTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPFORWARD_ROW2 {
    pub InterfaceLuid: NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub DestinationPrefix: IP_ADDRESS_PREFIX,
    pub NextHop: super::super::Networking::WinSock::SOCKADDR_INET,
    pub SitePrefixLength: u8,
    pub ValidLifetime: u32,
    pub PreferredLifetime: u32,
    pub Metric: u32,
    pub Protocol: super::super::Networking::WinSock::NL_ROUTE_PROTOCOL,
    pub Loopback: super::super::Foundation::BOOLEAN,
    pub AutoconfigureAddress: super::super::Foundation::BOOLEAN,
    pub Publish: super::super::Foundation::BOOLEAN,
    pub Immortal: super::super::Foundation::BOOLEAN,
    pub Age: u32,
    pub Origin: super::super::Networking::WinSock::NL_ROUTE_ORIGIN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPFORWARD_ROW2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPFORWARD_ROW2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPFORWARD_TABLE2 {
    pub NumEntries: u32,
    pub Table: [MIB_IPFORWARD_ROW2; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPFORWARD_TABLE2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPFORWARD_TABLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type MIB_IPFORWARD_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPROUTE_TYPE_OTHER: MIB_IPFORWARD_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPROUTE_TYPE_INVALID: MIB_IPFORWARD_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPROUTE_TYPE_DIRECT: MIB_IPFORWARD_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPROUTE_TYPE_INDIRECT: MIB_IPFORWARD_TYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPINTERFACE_ROW {
    pub Family: u16,
    pub InterfaceLuid: NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub MaxReassemblySize: u32,
    pub InterfaceIdentifier: u64,
    pub MinRouterAdvertisementInterval: u32,
    pub MaxRouterAdvertisementInterval: u32,
    pub AdvertisingEnabled: super::super::Foundation::BOOLEAN,
    pub ForwardingEnabled: super::super::Foundation::BOOLEAN,
    pub WeakHostSend: super::super::Foundation::BOOLEAN,
    pub WeakHostReceive: super::super::Foundation::BOOLEAN,
    pub UseAutomaticMetric: super::super::Foundation::BOOLEAN,
    pub UseNeighborUnreachabilityDetection: super::super::Foundation::BOOLEAN,
    pub ManagedAddressConfigurationSupported: super::super::Foundation::BOOLEAN,
    pub OtherStatefulConfigurationSupported: super::super::Foundation::BOOLEAN,
    pub AdvertiseDefaultRoute: super::super::Foundation::BOOLEAN,
    pub RouterDiscoveryBehavior: super::super::Networking::WinSock::NL_ROUTER_DISCOVERY_BEHAVIOR,
    pub DadTransmits: u32,
    pub BaseReachableTime: u32,
    pub RetransmitTime: u32,
    pub PathMtuDiscoveryTimeout: u32,
    pub LinkLocalAddressBehavior: super::super::Networking::WinSock::NL_LINK_LOCAL_ADDRESS_BEHAVIOR,
    pub LinkLocalAddressTimeout: u32,
    pub ZoneIndices: [u32; 16],
    pub SitePrefixLength: u32,
    pub Metric: u32,
    pub NlMtu: u32,
    pub Connected: super::super::Foundation::BOOLEAN,
    pub SupportsWakeUpPatterns: super::super::Foundation::BOOLEAN,
    pub SupportsNeighborDiscovery: super::super::Foundation::BOOLEAN,
    pub SupportsRouterDiscovery: super::super::Foundation::BOOLEAN,
    pub ReachableTime: u32,
    pub TransmitOffload: super::super::Networking::WinSock::NL_INTERFACE_OFFLOAD_ROD,
    pub ReceiveOffload: super::super::Networking::WinSock::NL_INTERFACE_OFFLOAD_ROD,
    pub DisableDefaultRoutes: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPINTERFACE_ROW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPINTERFACE_ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPINTERFACE_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_IPINTERFACE_ROW; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPINTERFACE_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPINTERFACE_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_BOUNDARY {
    pub dwIfIndex: u32,
    pub dwGroupAddress: u32,
    pub dwGroupMask: u32,
    pub dwStatus: u32,
}
impl ::core::marker::Copy for MIB_IPMCAST_BOUNDARY {}
impl ::core::clone::Clone for MIB_IPMCAST_BOUNDARY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_BOUNDARY_TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPMCAST_BOUNDARY; 1],
}
impl ::core::marker::Copy for MIB_IPMCAST_BOUNDARY_TABLE {}
impl ::core::clone::Clone for MIB_IPMCAST_BOUNDARY_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_GLOBAL {
    pub dwEnable: u32,
}
impl ::core::marker::Copy for MIB_IPMCAST_GLOBAL {}
impl ::core::clone::Clone for MIB_IPMCAST_GLOBAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_IF_ENTRY {
    pub dwIfIndex: u32,
    pub dwTtl: u32,
    pub dwProtocol: u32,
    pub dwRateLimit: u32,
    pub ulInMcastOctets: u32,
    pub ulOutMcastOctets: u32,
}
impl ::core::marker::Copy for MIB_IPMCAST_IF_ENTRY {}
impl ::core::clone::Clone for MIB_IPMCAST_IF_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_IF_TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPMCAST_IF_ENTRY; 1],
}
impl ::core::marker::Copy for MIB_IPMCAST_IF_TABLE {}
impl ::core::clone::Clone for MIB_IPMCAST_IF_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_MFE {
    pub dwGroup: u32,
    pub dwSource: u32,
    pub dwSrcMask: u32,
    pub dwUpStrmNgbr: u32,
    pub dwInIfIndex: u32,
    pub dwInIfProtocol: u32,
    pub dwRouteProtocol: u32,
    pub dwRouteNetwork: u32,
    pub dwRouteMask: u32,
    pub ulUpTime: u32,
    pub ulExpiryTime: u32,
    pub ulTimeOut: u32,
    pub ulNumOutIf: u32,
    pub fFlags: u32,
    pub dwReserved: u32,
    pub rgmioOutInfo: [MIB_IPMCAST_OIF_XP; 1],
}
impl ::core::marker::Copy for MIB_IPMCAST_MFE {}
impl ::core::clone::Clone for MIB_IPMCAST_MFE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_MFE_STATS {
    pub dwGroup: u32,
    pub dwSource: u32,
    pub dwSrcMask: u32,
    pub dwUpStrmNgbr: u32,
    pub dwInIfIndex: u32,
    pub dwInIfProtocol: u32,
    pub dwRouteProtocol: u32,
    pub dwRouteNetwork: u32,
    pub dwRouteMask: u32,
    pub ulUpTime: u32,
    pub ulExpiryTime: u32,
    pub ulNumOutIf: u32,
    pub ulInPkts: u32,
    pub ulInOctets: u32,
    pub ulPktsDifferentIf: u32,
    pub ulQueueOverflow: u32,
    pub rgmiosOutStats: [MIB_IPMCAST_OIF_STATS_LH; 1],
}
impl ::core::marker::Copy for MIB_IPMCAST_MFE_STATS {}
impl ::core::clone::Clone for MIB_IPMCAST_MFE_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_MFE_STATS_EX_XP {
    pub dwGroup: u32,
    pub dwSource: u32,
    pub dwSrcMask: u32,
    pub dwUpStrmNgbr: u32,
    pub dwInIfIndex: u32,
    pub dwInIfProtocol: u32,
    pub dwRouteProtocol: u32,
    pub dwRouteNetwork: u32,
    pub dwRouteMask: u32,
    pub ulUpTime: u32,
    pub ulExpiryTime: u32,
    pub ulNumOutIf: u32,
    pub ulInPkts: u32,
    pub ulInOctets: u32,
    pub ulPktsDifferentIf: u32,
    pub ulQueueOverflow: u32,
    pub ulUninitMfe: u32,
    pub ulNegativeMfe: u32,
    pub ulInDiscards: u32,
    pub ulInHdrErrors: u32,
    pub ulTotalOutPackets: u32,
    pub rgmiosOutStats: [MIB_IPMCAST_OIF_STATS_LH; 1],
}
impl ::core::marker::Copy for MIB_IPMCAST_MFE_STATS_EX_XP {}
impl ::core::clone::Clone for MIB_IPMCAST_MFE_STATS_EX_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_OIF_STATS_LH {
    pub dwOutIfIndex: u32,
    pub dwNextHopAddr: u32,
    pub dwDialContext: u32,
    pub ulTtlTooLow: u32,
    pub ulFragNeeded: u32,
    pub ulOutPackets: u32,
    pub ulOutDiscards: u32,
}
impl ::core::marker::Copy for MIB_IPMCAST_OIF_STATS_LH {}
impl ::core::clone::Clone for MIB_IPMCAST_OIF_STATS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_OIF_STATS_W2K {
    pub dwOutIfIndex: u32,
    pub dwNextHopAddr: u32,
    pub pvDialContext: *mut ::core::ffi::c_void,
    pub ulTtlTooLow: u32,
    pub ulFragNeeded: u32,
    pub ulOutPackets: u32,
    pub ulOutDiscards: u32,
}
impl ::core::marker::Copy for MIB_IPMCAST_OIF_STATS_W2K {}
impl ::core::clone::Clone for MIB_IPMCAST_OIF_STATS_W2K {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_OIF_W2K {
    pub dwOutIfIndex: u32,
    pub dwNextHopAddr: u32,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for MIB_IPMCAST_OIF_W2K {}
impl ::core::clone::Clone for MIB_IPMCAST_OIF_W2K {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_OIF_XP {
    pub dwOutIfIndex: u32,
    pub dwNextHopAddr: u32,
    pub dwReserved: u32,
    pub dwReserved1: u32,
}
impl ::core::marker::Copy for MIB_IPMCAST_OIF_XP {}
impl ::core::clone::Clone for MIB_IPMCAST_OIF_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPMCAST_SCOPE {
    pub dwGroupAddress: u32,
    pub dwGroupMask: u32,
    pub snNameBuffer: [u16; 256],
    pub dwStatus: u32,
}
impl ::core::marker::Copy for MIB_IPMCAST_SCOPE {}
impl ::core::clone::Clone for MIB_IPMCAST_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPNETROW_LH {
    pub dwIndex: u32,
    pub dwPhysAddrLen: u32,
    pub bPhysAddr: [u8; 8],
    pub dwAddr: u32,
    pub Anonymous: MIB_IPNETROW_LH_0,
}
impl ::core::marker::Copy for MIB_IPNETROW_LH {}
impl ::core::clone::Clone for MIB_IPNETROW_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_IPNETROW_LH_0 {
    pub dwType: u32,
    pub Type: MIB_IPNET_TYPE,
}
impl ::core::marker::Copy for MIB_IPNETROW_LH_0 {}
impl ::core::clone::Clone for MIB_IPNETROW_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPNETROW_W2K {
    pub dwIndex: u32,
    pub dwPhysAddrLen: u32,
    pub bPhysAddr: [u8; 8],
    pub dwAddr: u32,
    pub dwType: u32,
}
impl ::core::marker::Copy for MIB_IPNETROW_W2K {}
impl ::core::clone::Clone for MIB_IPNETROW_W2K {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPNETTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPNETROW_LH; 1],
}
impl ::core::marker::Copy for MIB_IPNETTABLE {}
impl ::core::clone::Clone for MIB_IPNETTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPNET_ROW2 {
    pub Address: super::super::Networking::WinSock::SOCKADDR_INET,
    pub InterfaceIndex: u32,
    pub InterfaceLuid: NET_LUID_LH,
    pub PhysicalAddress: [u8; 32],
    pub PhysicalAddressLength: u32,
    pub State: super::super::Networking::WinSock::NL_NEIGHBOR_STATE,
    pub Anonymous: MIB_IPNET_ROW2_0,
    pub ReachabilityTime: MIB_IPNET_ROW2_1,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPNET_ROW2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPNET_ROW2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union MIB_IPNET_ROW2_0 {
    pub Anonymous: MIB_IPNET_ROW2_0_0,
    pub Flags: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPNET_ROW2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPNET_ROW2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPNET_ROW2_0_0 {
    pub _bitfield: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPNET_ROW2_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPNET_ROW2_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union MIB_IPNET_ROW2_1 {
    pub LastReachable: u32,
    pub LastUnreachable: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPNET_ROW2_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPNET_ROW2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPNET_TABLE2 {
    pub NumEntries: u32,
    pub Table: [MIB_IPNET_ROW2; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPNET_TABLE2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPNET_TABLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type MIB_IPNET_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPNET_TYPE_OTHER: MIB_IPNET_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPNET_TYPE_INVALID: MIB_IPNET_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPNET_TYPE_DYNAMIC: MIB_IPNET_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPNET_TYPE_STATIC: MIB_IPNET_TYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPPATH_ROW {
    pub Source: super::super::Networking::WinSock::SOCKADDR_INET,
    pub Destination: super::super::Networking::WinSock::SOCKADDR_INET,
    pub InterfaceLuid: NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub CurrentNextHop: super::super::Networking::WinSock::SOCKADDR_INET,
    pub PathMtu: u32,
    pub RttMean: u32,
    pub RttDeviation: u32,
    pub Anonymous: MIB_IPPATH_ROW_0,
    pub IsReachable: super::super::Foundation::BOOLEAN,
    pub LinkTransmitSpeed: u64,
    pub LinkReceiveSpeed: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPPATH_ROW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPPATH_ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union MIB_IPPATH_ROW_0 {
    pub LastReachable: u32,
    pub LastUnreachable: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPPATH_ROW_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPPATH_ROW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IPPATH_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_IPPATH_ROW; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IPPATH_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IPPATH_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IPROUTE_METRIC_UNUSED: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type MIB_IPSTATS_FORWARDING = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IP_FORWARDING: MIB_IPSTATS_FORWARDING = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_IP_NOT_FORWARDING: MIB_IPSTATS_FORWARDING = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPSTATS_LH {
    pub Anonymous: MIB_IPSTATS_LH_0,
    pub dwDefaultTTL: u32,
    pub dwInReceives: u32,
    pub dwInHdrErrors: u32,
    pub dwInAddrErrors: u32,
    pub dwForwDatagrams: u32,
    pub dwInUnknownProtos: u32,
    pub dwInDiscards: u32,
    pub dwInDelivers: u32,
    pub dwOutRequests: u32,
    pub dwRoutingDiscards: u32,
    pub dwOutDiscards: u32,
    pub dwOutNoRoutes: u32,
    pub dwReasmTimeout: u32,
    pub dwReasmReqds: u32,
    pub dwReasmOks: u32,
    pub dwReasmFails: u32,
    pub dwFragOks: u32,
    pub dwFragFails: u32,
    pub dwFragCreates: u32,
    pub dwNumIf: u32,
    pub dwNumAddr: u32,
    pub dwNumRoutes: u32,
}
impl ::core::marker::Copy for MIB_IPSTATS_LH {}
impl ::core::clone::Clone for MIB_IPSTATS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_IPSTATS_LH_0 {
    pub dwForwarding: u32,
    pub Forwarding: MIB_IPSTATS_FORWARDING,
}
impl ::core::marker::Copy for MIB_IPSTATS_LH_0 {}
impl ::core::clone::Clone for MIB_IPSTATS_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_IPSTATS_W2K {
    pub dwForwarding: u32,
    pub dwDefaultTTL: u32,
    pub dwInReceives: u32,
    pub dwInHdrErrors: u32,
    pub dwInAddrErrors: u32,
    pub dwForwDatagrams: u32,
    pub dwInUnknownProtos: u32,
    pub dwInDiscards: u32,
    pub dwInDelivers: u32,
    pub dwOutRequests: u32,
    pub dwRoutingDiscards: u32,
    pub dwOutDiscards: u32,
    pub dwOutNoRoutes: u32,
    pub dwReasmTimeout: u32,
    pub dwReasmReqds: u32,
    pub dwReasmOks: u32,
    pub dwReasmFails: u32,
    pub dwFragOks: u32,
    pub dwFragFails: u32,
    pub dwFragCreates: u32,
    pub dwNumIf: u32,
    pub dwNumAddr: u32,
    pub dwNumRoutes: u32,
}
impl ::core::marker::Copy for MIB_IPSTATS_W2K {}
impl ::core::clone::Clone for MIB_IPSTATS_W2K {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    pub InboundBandwidthInformation: super::super::Networking::WinSock::NL_BANDWIDTH_INFORMATION,
    pub OutboundBandwidthInformation: super::super::Networking::WinSock::NL_BANDWIDTH_INFORMATION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_MCAST_LIMIT_ROW {
    pub dwTtl: u32,
    pub dwRateLimit: u32,
}
impl ::core::marker::Copy for MIB_MCAST_LIMIT_ROW {}
impl ::core::clone::Clone for MIB_MCAST_LIMIT_ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_MFE_STATS_TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPMCAST_MFE_STATS; 1],
}
impl ::core::marker::Copy for MIB_MFE_STATS_TABLE {}
impl ::core::clone::Clone for MIB_MFE_STATS_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_MFE_STATS_TABLE_EX_XP {
    pub dwNumEntries: u32,
    pub table: [*mut MIB_IPMCAST_MFE_STATS_EX_XP; 1],
}
impl ::core::marker::Copy for MIB_MFE_STATS_TABLE_EX_XP {}
impl ::core::clone::Clone for MIB_MFE_STATS_TABLE_EX_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_MFE_TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPMCAST_MFE; 1],
}
impl ::core::marker::Copy for MIB_MFE_TABLE {}
impl ::core::clone::Clone for MIB_MFE_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_MULTICASTIPADDRESS_ROW {
    pub Address: super::super::Networking::WinSock::SOCKADDR_INET,
    pub InterfaceIndex: u32,
    pub InterfaceLuid: NET_LUID_LH,
    pub ScopeId: super::super::Networking::WinSock::SCOPE_ID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_MULTICASTIPADDRESS_ROW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_MULTICASTIPADDRESS_ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_MULTICASTIPADDRESS_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_MULTICASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_MULTICASTIPADDRESS_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_MULTICASTIPADDRESS_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type MIB_NOTIFICATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibParameterNotification: MIB_NOTIFICATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibAddInstance: MIB_NOTIFICATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibDeleteInstance: MIB_NOTIFICATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MibInitialNotification: MIB_NOTIFICATION_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_OPAQUE_INFO {
    pub dwId: u32,
    pub Anonymous: MIB_OPAQUE_INFO_0,
}
impl ::core::marker::Copy for MIB_OPAQUE_INFO {}
impl ::core::clone::Clone for MIB_OPAQUE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_OPAQUE_INFO_0 {
    pub ullAlign: u64,
    pub rgbyData: [u8; 1],
}
impl ::core::marker::Copy for MIB_OPAQUE_INFO_0 {}
impl ::core::clone::Clone for MIB_OPAQUE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_OPAQUE_QUERY {
    pub dwVarId: u32,
    pub rgdwVarIndex: [u32; 1],
}
impl ::core::marker::Copy for MIB_OPAQUE_QUERY {}
impl ::core::clone::Clone for MIB_OPAQUE_QUERY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_PROXYARP {
    pub dwAddress: u32,
    pub dwMask: u32,
    pub dwIfIndex: u32,
}
impl ::core::marker::Copy for MIB_PROXYARP {}
impl ::core::clone::Clone for MIB_PROXYARP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIB_ROUTESTATE {
    pub bRoutesSetToStack: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIB_ROUTESTATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIB_ROUTESTATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_TCP6ROW {
    pub State: MIB_TCP_STATE,
    pub LocalAddr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub RemoteAddr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwRemoteScopeId: u32,
    pub dwRemotePort: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_TCP6ROW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_TCP6ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_TCP6ROW2 {
    pub LocalAddr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub RemoteAddr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwRemoteScopeId: u32,
    pub dwRemotePort: u32,
    pub State: MIB_TCP_STATE,
    pub dwOwningPid: u32,
    pub dwOffloadState: TCP_CONNECTION_OFFLOAD_STATE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_TCP6ROW2 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_TCP6ROW2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCP6ROW_OWNER_MODULE {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub ucRemoteAddr: [u8; 16],
    pub dwRemoteScopeId: u32,
    pub dwRemotePort: u32,
    pub dwState: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub OwningModuleInfo: [u64; 16],
}
impl ::core::marker::Copy for MIB_TCP6ROW_OWNER_MODULE {}
impl ::core::clone::Clone for MIB_TCP6ROW_OWNER_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCP6ROW_OWNER_PID {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub ucRemoteAddr: [u8; 16],
    pub dwRemoteScopeId: u32,
    pub dwRemotePort: u32,
    pub dwState: u32,
    pub dwOwningPid: u32,
}
impl ::core::marker::Copy for MIB_TCP6ROW_OWNER_PID {}
impl ::core::clone::Clone for MIB_TCP6ROW_OWNER_PID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_TCP6TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_TCP6ROW; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_TCP6TABLE {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_TCP6TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_TCP6TABLE2 {
    pub dwNumEntries: u32,
    pub table: [MIB_TCP6ROW2; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_TCP6TABLE2 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_TCP6TABLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCP6TABLE_OWNER_MODULE {
    pub dwNumEntries: u32,
    pub table: [MIB_TCP6ROW_OWNER_MODULE; 1],
}
impl ::core::marker::Copy for MIB_TCP6TABLE_OWNER_MODULE {}
impl ::core::clone::Clone for MIB_TCP6TABLE_OWNER_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCP6TABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_TCP6ROW_OWNER_PID; 1],
}
impl ::core::marker::Copy for MIB_TCP6TABLE_OWNER_PID {}
impl ::core::clone::Clone for MIB_TCP6TABLE_OWNER_PID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPROW2 {
    pub dwState: u32,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
    pub dwOwningPid: u32,
    pub dwOffloadState: TCP_CONNECTION_OFFLOAD_STATE,
}
impl ::core::marker::Copy for MIB_TCPROW2 {}
impl ::core::clone::Clone for MIB_TCPROW2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPROW_LH {
    pub Anonymous: MIB_TCPROW_LH_0,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
}
impl ::core::marker::Copy for MIB_TCPROW_LH {}
impl ::core::clone::Clone for MIB_TCPROW_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_TCPROW_LH_0 {
    pub dwState: u32,
    pub State: MIB_TCP_STATE,
}
impl ::core::marker::Copy for MIB_TCPROW_LH_0 {}
impl ::core::clone::Clone for MIB_TCPROW_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPROW_OWNER_MODULE {
    pub dwState: u32,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub OwningModuleInfo: [u64; 16],
}
impl ::core::marker::Copy for MIB_TCPROW_OWNER_MODULE {}
impl ::core::clone::Clone for MIB_TCPROW_OWNER_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPROW_OWNER_PID {
    pub dwState: u32,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
    pub dwOwningPid: u32,
}
impl ::core::marker::Copy for MIB_TCPROW_OWNER_PID {}
impl ::core::clone::Clone for MIB_TCPROW_OWNER_PID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPROW_W2K {
    pub dwState: u32,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
}
impl ::core::marker::Copy for MIB_TCPROW_W2K {}
impl ::core::clone::Clone for MIB_TCPROW_W2K {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPSTATS2 {
    pub RtoAlgorithm: TCP_RTO_ALGORITHM,
    pub dwRtoMin: u32,
    pub dwRtoMax: u32,
    pub dwMaxConn: u32,
    pub dwActiveOpens: u32,
    pub dwPassiveOpens: u32,
    pub dwAttemptFails: u32,
    pub dwEstabResets: u32,
    pub dwCurrEstab: u32,
    pub dw64InSegs: u64,
    pub dw64OutSegs: u64,
    pub dwRetransSegs: u32,
    pub dwInErrs: u32,
    pub dwOutRsts: u32,
    pub dwNumConns: u32,
}
impl ::core::marker::Copy for MIB_TCPSTATS2 {}
impl ::core::clone::Clone for MIB_TCPSTATS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPSTATS_LH {
    pub Anonymous: MIB_TCPSTATS_LH_0,
    pub dwRtoMin: u32,
    pub dwRtoMax: u32,
    pub dwMaxConn: u32,
    pub dwActiveOpens: u32,
    pub dwPassiveOpens: u32,
    pub dwAttemptFails: u32,
    pub dwEstabResets: u32,
    pub dwCurrEstab: u32,
    pub dwInSegs: u32,
    pub dwOutSegs: u32,
    pub dwRetransSegs: u32,
    pub dwInErrs: u32,
    pub dwOutRsts: u32,
    pub dwNumConns: u32,
}
impl ::core::marker::Copy for MIB_TCPSTATS_LH {}
impl ::core::clone::Clone for MIB_TCPSTATS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_TCPSTATS_LH_0 {
    pub dwRtoAlgorithm: u32,
    pub RtoAlgorithm: TCP_RTO_ALGORITHM,
}
impl ::core::marker::Copy for MIB_TCPSTATS_LH_0 {}
impl ::core::clone::Clone for MIB_TCPSTATS_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPSTATS_W2K {
    pub dwRtoAlgorithm: u32,
    pub dwRtoMin: u32,
    pub dwRtoMax: u32,
    pub dwMaxConn: u32,
    pub dwActiveOpens: u32,
    pub dwPassiveOpens: u32,
    pub dwAttemptFails: u32,
    pub dwEstabResets: u32,
    pub dwCurrEstab: u32,
    pub dwInSegs: u32,
    pub dwOutSegs: u32,
    pub dwRetransSegs: u32,
    pub dwInErrs: u32,
    pub dwOutRsts: u32,
    pub dwNumConns: u32,
}
impl ::core::marker::Copy for MIB_TCPSTATS_W2K {}
impl ::core::clone::Clone for MIB_TCPSTATS_W2K {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW_LH; 1],
}
impl ::core::marker::Copy for MIB_TCPTABLE {}
impl ::core::clone::Clone for MIB_TCPTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPTABLE2 {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW2; 1],
}
impl ::core::marker::Copy for MIB_TCPTABLE2 {}
impl ::core::clone::Clone for MIB_TCPTABLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPTABLE_OWNER_MODULE {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW_OWNER_MODULE; 1],
}
impl ::core::marker::Copy for MIB_TCPTABLE_OWNER_MODULE {}
impl ::core::clone::Clone for MIB_TCPTABLE_OWNER_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_TCPTABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW_OWNER_PID; 1],
}
impl ::core::marker::Copy for MIB_TCPTABLE_OWNER_PID {}
impl ::core::clone::Clone for MIB_TCPTABLE_OWNER_PID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type MIB_TCP_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_CLOSED: MIB_TCP_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_LISTEN: MIB_TCP_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_SYN_SENT: MIB_TCP_STATE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_SYN_RCVD: MIB_TCP_STATE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_ESTAB: MIB_TCP_STATE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_FIN_WAIT1: MIB_TCP_STATE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_FIN_WAIT2: MIB_TCP_STATE = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_CLOSE_WAIT: MIB_TCP_STATE = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_CLOSING: MIB_TCP_STATE = 9i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_LAST_ACK: MIB_TCP_STATE = 10i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_TIME_WAIT: MIB_TCP_STATE = 11i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_DELETE_TCB: MIB_TCP_STATE = 12i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_STATE_RESERVED: MIB_TCP_STATE = 100i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_UDP6ROW {
    pub dwLocalAddr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_UDP6ROW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_UDP6ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDP6ROW2 {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub Anonymous: MIB_UDP6ROW2_0,
    pub OwningModuleInfo: [u64; 16],
    pub ucRemoteAddr: [u8; 16],
    pub dwRemoteScopeId: u32,
    pub dwRemotePort: u32,
}
impl ::core::marker::Copy for MIB_UDP6ROW2 {}
impl ::core::clone::Clone for MIB_UDP6ROW2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_UDP6ROW2_0 {
    pub Anonymous: MIB_UDP6ROW2_0_0,
    pub dwFlags: i32,
}
impl ::core::marker::Copy for MIB_UDP6ROW2_0 {}
impl ::core::clone::Clone for MIB_UDP6ROW2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDP6ROW2_0_0 {
    pub _bitfield: i32,
}
impl ::core::marker::Copy for MIB_UDP6ROW2_0_0 {}
impl ::core::clone::Clone for MIB_UDP6ROW2_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDP6ROW_OWNER_MODULE {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub Anonymous: MIB_UDP6ROW_OWNER_MODULE_0,
    pub OwningModuleInfo: [u64; 16],
}
impl ::core::marker::Copy for MIB_UDP6ROW_OWNER_MODULE {}
impl ::core::clone::Clone for MIB_UDP6ROW_OWNER_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_UDP6ROW_OWNER_MODULE_0 {
    pub Anonymous: MIB_UDP6ROW_OWNER_MODULE_0_0,
    pub dwFlags: i32,
}
impl ::core::marker::Copy for MIB_UDP6ROW_OWNER_MODULE_0 {}
impl ::core::clone::Clone for MIB_UDP6ROW_OWNER_MODULE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDP6ROW_OWNER_MODULE_0_0 {
    pub _bitfield: i32,
}
impl ::core::marker::Copy for MIB_UDP6ROW_OWNER_MODULE_0_0 {}
impl ::core::clone::Clone for MIB_UDP6ROW_OWNER_MODULE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDP6ROW_OWNER_PID {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
}
impl ::core::marker::Copy for MIB_UDP6ROW_OWNER_PID {}
impl ::core::clone::Clone for MIB_UDP6ROW_OWNER_PID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MIB_UDP6TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_UDP6ROW; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MIB_UDP6TABLE {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MIB_UDP6TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDP6TABLE2 {
    pub dwNumEntries: u32,
    pub table: [MIB_UDP6ROW2; 1],
}
impl ::core::marker::Copy for MIB_UDP6TABLE2 {}
impl ::core::clone::Clone for MIB_UDP6TABLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDP6TABLE_OWNER_MODULE {
    pub dwNumEntries: u32,
    pub table: [MIB_UDP6ROW_OWNER_MODULE; 1],
}
impl ::core::marker::Copy for MIB_UDP6TABLE_OWNER_MODULE {}
impl ::core::clone::Clone for MIB_UDP6TABLE_OWNER_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDP6TABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_UDP6ROW_OWNER_PID; 1],
}
impl ::core::marker::Copy for MIB_UDP6TABLE_OWNER_PID {}
impl ::core::clone::Clone for MIB_UDP6TABLE_OWNER_PID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPROW {
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
}
impl ::core::marker::Copy for MIB_UDPROW {}
impl ::core::clone::Clone for MIB_UDPROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPROW2 {
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub Anonymous: MIB_UDPROW2_0,
    pub OwningModuleInfo: [u64; 16],
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
}
impl ::core::marker::Copy for MIB_UDPROW2 {}
impl ::core::clone::Clone for MIB_UDPROW2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_UDPROW2_0 {
    pub Anonymous: MIB_UDPROW2_0_0,
    pub dwFlags: i32,
}
impl ::core::marker::Copy for MIB_UDPROW2_0 {}
impl ::core::clone::Clone for MIB_UDPROW2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPROW2_0_0 {
    pub _bitfield: i32,
}
impl ::core::marker::Copy for MIB_UDPROW2_0_0 {}
impl ::core::clone::Clone for MIB_UDPROW2_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPROW_OWNER_MODULE {
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub Anonymous: MIB_UDPROW_OWNER_MODULE_0,
    pub OwningModuleInfo: [u64; 16],
}
impl ::core::marker::Copy for MIB_UDPROW_OWNER_MODULE {}
impl ::core::clone::Clone for MIB_UDPROW_OWNER_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union MIB_UDPROW_OWNER_MODULE_0 {
    pub Anonymous: MIB_UDPROW_OWNER_MODULE_0_0,
    pub dwFlags: i32,
}
impl ::core::marker::Copy for MIB_UDPROW_OWNER_MODULE_0 {}
impl ::core::clone::Clone for MIB_UDPROW_OWNER_MODULE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPROW_OWNER_MODULE_0_0 {
    pub _bitfield: i32,
}
impl ::core::marker::Copy for MIB_UDPROW_OWNER_MODULE_0_0 {}
impl ::core::clone::Clone for MIB_UDPROW_OWNER_MODULE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPROW_OWNER_PID {
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
}
impl ::core::marker::Copy for MIB_UDPROW_OWNER_PID {}
impl ::core::clone::Clone for MIB_UDPROW_OWNER_PID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPSTATS {
    pub dwInDatagrams: u32,
    pub dwNoPorts: u32,
    pub dwInErrors: u32,
    pub dwOutDatagrams: u32,
    pub dwNumAddrs: u32,
}
impl ::core::marker::Copy for MIB_UDPSTATS {}
impl ::core::clone::Clone for MIB_UDPSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPSTATS2 {
    pub dw64InDatagrams: u64,
    pub dwNoPorts: u32,
    pub dwInErrors: u32,
    pub dw64OutDatagrams: u64,
    pub dwNumAddrs: u32,
}
impl ::core::marker::Copy for MIB_UDPSTATS2 {}
impl ::core::clone::Clone for MIB_UDPSTATS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW; 1],
}
impl ::core::marker::Copy for MIB_UDPTABLE {}
impl ::core::clone::Clone for MIB_UDPTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPTABLE2 {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW2; 1],
}
impl ::core::marker::Copy for MIB_UDPTABLE2 {}
impl ::core::clone::Clone for MIB_UDPTABLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPTABLE_OWNER_MODULE {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW_OWNER_MODULE; 1],
}
impl ::core::marker::Copy for MIB_UDPTABLE_OWNER_MODULE {}
impl ::core::clone::Clone for MIB_UDPTABLE_OWNER_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct MIB_UDPTABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW_OWNER_PID; 1],
}
impl ::core::marker::Copy for MIB_UDPTABLE_OWNER_PID {}
impl ::core::clone::Clone for MIB_UDPTABLE_OWNER_PID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_UNICASTIPADDRESS_ROW {
    pub Address: super::super::Networking::WinSock::SOCKADDR_INET,
    pub InterfaceLuid: NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub PrefixOrigin: super::super::Networking::WinSock::NL_PREFIX_ORIGIN,
    pub SuffixOrigin: super::super::Networking::WinSock::NL_SUFFIX_ORIGIN,
    pub ValidLifetime: u32,
    pub PreferredLifetime: u32,
    pub OnLinkPrefixLength: u8,
    pub SkipAsSource: super::super::Foundation::BOOLEAN,
    pub DadState: super::super::Networking::WinSock::NL_DAD_STATE,
    pub ScopeId: super::super::Networking::WinSock::SCOPE_ID,
    pub CreationTimeStamp: i64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_UNICASTIPADDRESS_ROW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_UNICASTIPADDRESS_ROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MIB_UNICASTIPADDRESS_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_UNICASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MIB_UNICASTIPADDRESS_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MIB_UNICASTIPADDRESS_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_USE_CURRENT_FORWARDING: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_USE_CURRENT_TTL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIN_IF_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIXED_NODETYPE: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NDIS_INTERFACE_INFORMATION {
    pub ifOperStatus: NET_IF_OPER_STATUS,
    pub ifOperStatusFlags: u32,
    pub MediaConnectState: NET_IF_MEDIA_CONNECT_STATE,
    pub MediaDuplexState: NET_IF_MEDIA_DUPLEX_STATE,
    pub ifMtu: u32,
    pub ifPromiscuousMode: super::super::Foundation::BOOLEAN,
    pub ifDeviceWakeUpEnable: super::super::Foundation::BOOLEAN,
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
    pub ifLastChange: u64,
    pub ifCounterDiscontinuityTime: u64,
    pub ifInUnknownProtos: u64,
    pub ifInDiscards: u64,
    pub ifInErrors: u64,
    pub ifHCInOctets: u64,
    pub ifHCInUcastPkts: u64,
    pub ifHCInMulticastPkts: u64,
    pub ifHCInBroadcastPkts: u64,
    pub ifHCOutOctets: u64,
    pub ifHCOutUcastPkts: u64,
    pub ifHCOutMulticastPkts: u64,
    pub ifHCOutBroadcastPkts: u64,
    pub ifOutErrors: u64,
    pub ifOutDiscards: u64,
    pub ifHCInUcastOctets: u64,
    pub ifHCInMulticastOctets: u64,
    pub ifHCInBroadcastOctets: u64,
    pub ifHCOutUcastOctets: u64,
    pub ifHCOutMulticastOctets: u64,
    pub ifHCOutBroadcastOctets: u64,
    pub CompartmentId: u32,
    pub SupportedStatistics: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NDIS_INTERFACE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NDIS_INTERFACE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type NET_ADDRESS_FORMAT = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_ADDRESS_FORMAT_UNSPECIFIED: NET_ADDRESS_FORMAT = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_ADDRESS_DNS_NAME: NET_ADDRESS_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_ADDRESS_IPV4: NET_ADDRESS_FORMAT = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_ADDRESS_IPV6: NET_ADDRESS_FORMAT = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IFLUID_UNSPECIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type NET_IF_ACCESS_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_ACCESS_LOOPBACK: NET_IF_ACCESS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_ACCESS_BROADCAST: NET_IF_ACCESS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_ACCESS_POINT_TO_POINT: NET_IF_ACCESS_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_ACCESS_POINT_TO_MULTI_POINT: NET_IF_ACCESS_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_ACCESS_MAXIMUM: NET_IF_ACCESS_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type NET_IF_ADMIN_STATUS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_ADMIN_STATUS_UP: NET_IF_ADMIN_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_ADMIN_STATUS_DOWN: NET_IF_ADMIN_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_ADMIN_STATUS_TESTING: NET_IF_ADMIN_STATUS = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct NET_IF_ALIAS_LH {
    pub ifAliasLength: u16,
    pub ifAliasOffset: u16,
}
impl ::core::marker::Copy for NET_IF_ALIAS_LH {}
impl ::core::clone::Clone for NET_IF_ALIAS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type NET_IF_CONNECTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_CONNECTION_DEDICATED: NET_IF_CONNECTION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_CONNECTION_PASSIVE: NET_IF_CONNECTION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_CONNECTION_DEMAND: NET_IF_CONNECTION_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_CONNECTION_MAXIMUM: NET_IF_CONNECTION_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type NET_IF_DIRECTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_DIRECTION_SENDRECEIVE: NET_IF_DIRECTION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_DIRECTION_SENDONLY: NET_IF_DIRECTION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_DIRECTION_RECEIVEONLY: NET_IF_DIRECTION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_DIRECTION_MAXIMUM: NET_IF_DIRECTION_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type NET_IF_MEDIA_CONNECT_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MediaConnectStateUnknown: NET_IF_MEDIA_CONNECT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MediaConnectStateConnected: NET_IF_MEDIA_CONNECT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MediaConnectStateDisconnected: NET_IF_MEDIA_CONNECT_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type NET_IF_MEDIA_DUPLEX_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MediaDuplexStateUnknown: NET_IF_MEDIA_DUPLEX_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MediaDuplexStateHalf: NET_IF_MEDIA_DUPLEX_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MediaDuplexStateFull: NET_IF_MEDIA_DUPLEX_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OID_COMPARTMENT_ID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OID_IF_ALIAS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OID_IF_ENTRY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OID_NETWORK_GUID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type NET_IF_OPER_STATUS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_UP: NET_IF_OPER_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_DOWN: NET_IF_OPER_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_TESTING: NET_IF_OPER_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_UNKNOWN: NET_IF_OPER_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_DORMANT: NET_IF_OPER_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_NOT_PRESENT: NET_IF_OPER_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_LOWER_LAYER_DOWN: NET_IF_OPER_STATUS = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_DORMANT_LOW_POWER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_DORMANT_PAUSED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_DOWN_NOT_AUTHENTICATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_OPER_STATUS_DOWN_NOT_MEDIA_CONNECTED: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct NET_IF_RCV_ADDRESS_LH {
    pub ifRcvAddressType: NET_IF_RCV_ADDRESS_TYPE,
    pub ifRcvAddressLength: u16,
    pub ifRcvAddressOffset: u16,
}
impl ::core::marker::Copy for NET_IF_RCV_ADDRESS_LH {}
impl ::core::clone::Clone for NET_IF_RCV_ADDRESS_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type NET_IF_RCV_ADDRESS_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_RCV_ADDRESS_TYPE_OTHER: NET_IF_RCV_ADDRESS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_RCV_ADDRESS_TYPE_VOLATILE: NET_IF_RCV_ADDRESS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_IF_RCV_ADDRESS_TYPE_NON_VOLATILE: NET_IF_RCV_ADDRESS_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub union NET_LUID_LH {
    pub Value: u64,
    pub Info: NET_LUID_LH_0,
}
impl ::core::marker::Copy for NET_LUID_LH {}
impl ::core::clone::Clone for NET_LUID_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct NET_LUID_LH_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for NET_LUID_LH_0 {}
impl ::core::clone::Clone for NET_LUID_LH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct NET_PHYSICAL_LOCATION_LH {
    pub BusNumber: u32,
    pub SlotNumber: u32,
    pub FunctionNumber: u32,
}
impl ::core::marker::Copy for NET_PHYSICAL_LOCATION_LH {}
impl ::core::clone::Clone for NET_PHYSICAL_LOCATION_LH {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_SITEID_MAXSYSTEM: u32 = 268435455u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_SITEID_MAXUSER: u32 = 134217727u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_SITEID_UNSPECIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_IPV4_ADDRESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_IPV4_NETWORK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_IPV4_SERVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_IPV6_ADDRESS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_IPV6_ADDRESS_NO_SCOPE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_IPV6_NETWORK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_IPV6_SERVICE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_IPV6_SERVICE_NO_SCOPE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_NAMED_ADDRESS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NET_STRING_NAMED_SERVICE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_FILTER_INTERFACE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_HARDWARE_INTERFACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_NDIS_ENDPOINT_INTERFACE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_NDIS_ISCSI_INTERFACE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_NDIS_RESERVED1: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_NDIS_RESERVED2: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_NDIS_RESERVED3: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_NDIS_RESERVED4: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NIIF_NDIS_WDM_INTERFACE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const NUMBER_OF_EXPORTED_VARIABLES: u32 = 39u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PEER_TO_PEER_NODETYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type PFADDRESSTYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PF_IPV4: PFADDRESSTYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PF_IPV6: PFADDRESSTYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PFERROR_BUFFER_TOO_SMALL: u32 = 23002u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PFERROR_NO_FILTERS_GIVEN: u32 = 23001u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PFERROR_NO_PF_INTERFACE: u32 = 23000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type PFFORWARD_ACTION = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PF_ACTION_FORWARD: PFFORWARD_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PF_ACTION_DROP: PFFORWARD_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type PFFRAMETYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PFFT_FILTER: PFFRAMETYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PFFT_FRAG: PFFRAMETYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PFFT_SPOOF: PFFRAMETYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct PFLOGFRAME {
    pub Timestamp: i64,
    pub pfeTypeOfFrame: PFFRAMETYPE,
    pub dwTotalSizeUsed: u32,
    pub dwFilterRule: u32,
    pub wSizeOfAdditionalData: u16,
    pub wSizeOfIpHeader: u16,
    pub dwInterfaceName: u32,
    pub dwIPIndex: u32,
    pub bPacketData: [u8; 1],
}
impl ::core::marker::Copy for PFLOGFRAME {}
impl ::core::clone::Clone for PFLOGFRAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct PF_FILTER_DESCRIPTOR {
    pub dwFilterFlags: u32,
    pub dwRule: u32,
    pub pfatType: PFADDRESSTYPE,
    pub SrcAddr: *mut u8,
    pub SrcMask: *mut u8,
    pub DstAddr: *mut u8,
    pub DstMask: *mut u8,
    pub dwProtocol: u32,
    pub fLateBound: u32,
    pub wSrcPort: u16,
    pub wDstPort: u16,
    pub wSrcPortHighRange: u16,
    pub wDstPortHighRange: u16,
}
impl ::core::marker::Copy for PF_FILTER_DESCRIPTOR {}
impl ::core::clone::Clone for PF_FILTER_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct PF_FILTER_STATS {
    pub dwNumPacketsFiltered: u32,
    pub info: PF_FILTER_DESCRIPTOR,
}
impl ::core::marker::Copy for PF_FILTER_STATS {}
impl ::core::clone::Clone for PF_FILTER_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct PF_INTERFACE_STATS {
    pub pvDriverContext: *mut ::core::ffi::c_void,
    pub dwFlags: u32,
    pub dwInDrops: u32,
    pub dwOutDrops: u32,
    pub eaInAction: PFFORWARD_ACTION,
    pub eaOutAction: PFFORWARD_ACTION,
    pub dwNumInFilters: u32,
    pub dwNumOutFilters: u32,
    pub dwFrag: u32,
    pub dwSpoof: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub liSYN: i64,
    pub liTotalLogged: i64,
    pub dwLostLogEntries: u32,
    pub FilterInfo: [PF_FILTER_STATS; 1],
}
impl ::core::marker::Copy for PF_INTERFACE_STATS {}
impl ::core::clone::Clone for PF_INTERFACE_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct PF_LATEBIND_INFO {
    pub SrcAddr: *mut u8,
    pub DstAddr: *mut u8,
    pub Mask: *mut u8,
}
impl ::core::marker::Copy for PF_LATEBIND_INFO {}
impl ::core::clone::Clone for PF_LATEBIND_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callercontext: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type PIPFORWARD_CHANGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callercontext: *const ::core::ffi::c_void, row: *const MIB_IPFORWARD_ROW2, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type PIPINTERFACE_CHANGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callercontext: *const ::core::ffi::c_void, row: *const MIB_IPINTERFACE_ROW, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callercontext: *const ::core::ffi::c_void, connectivityhint: super::super::Networking::WinSock::NL_NETWORK_CONNECTIVITY_HINT)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const PROXY_ARP: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callercontext: *const ::core::ffi::c_void, addresstable: *const MIB_UNICASTIPADDRESS_TABLE)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type PTEREDO_PORT_CHANGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callercontext: *const ::core::ffi::c_void, port: u16, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type PUNICAST_IPADDRESS_CHANGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callercontext: *const ::core::ffi::c_void, row: *const MIB_UNICASTIPADDRESS_ROW, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ROUTE_LONGER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ROUTE_MATCHING: u32 = 31u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ROUTE_SHORTER: u32 = 33u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const ROUTE_STATE: u32 = 34u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP6_STATS: u32 = 38u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCPIP_OWNER_MODULE_BASIC_INFO {
    pub pModuleName: ::windows_sys::core::PWSTR,
    pub pModulePath: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for TCPIP_OWNER_MODULE_BASIC_INFO {}
impl ::core::clone::Clone for TCPIP_OWNER_MODULE_BASIC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type TCPIP_OWNER_MODULE_INFO_CLASS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCPIP_OWNER_MODULE_INFO_BASIC: TCPIP_OWNER_MODULE_INFO_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCPIP_OWNING_MODULE_SIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type TCP_BOOLEAN_OPTIONAL = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpBoolOptDisabled: TCP_BOOLEAN_OPTIONAL = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpBoolOptEnabled: TCP_BOOLEAN_OPTIONAL = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpBoolOptUnchanged: TCP_BOOLEAN_OPTIONAL = -1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type TCP_CONNECTION_OFFLOAD_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionOffloadStateInHost: TCP_CONNECTION_OFFLOAD_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionOffloadStateOffloading: TCP_CONNECTION_OFFLOAD_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionOffloadStateOffloaded: TCP_CONNECTION_OFFLOAD_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionOffloadStateUploading: TCP_CONNECTION_OFFLOAD_STATE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionOffloadStateMax: TCP_CONNECTION_OFFLOAD_STATE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_BANDWIDTH_ROD_v0 {
    pub OutboundBandwidth: u64,
    pub InboundBandwidth: u64,
    pub OutboundInstability: u64,
    pub InboundInstability: u64,
    pub OutboundBandwidthPeaked: super::super::Foundation::BOOLEAN,
    pub InboundBandwidthPeaked: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_BANDWIDTH_ROD_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_BANDWIDTH_ROD_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_BANDWIDTH_RW_v0 {
    pub EnableCollectionOutbound: TCP_BOOLEAN_OPTIONAL,
    pub EnableCollectionInbound: TCP_BOOLEAN_OPTIONAL,
}
impl ::core::marker::Copy for TCP_ESTATS_BANDWIDTH_RW_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_BANDWIDTH_RW_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_DATA_ROD_v0 {
    pub DataBytesOut: u64,
    pub DataSegsOut: u64,
    pub DataBytesIn: u64,
    pub DataSegsIn: u64,
    pub SegsOut: u64,
    pub SegsIn: u64,
    pub SoftErrors: u32,
    pub SoftErrorReason: u32,
    pub SndUna: u32,
    pub SndNxt: u32,
    pub SndMax: u32,
    pub ThruBytesAcked: u64,
    pub RcvNxt: u32,
    pub ThruBytesReceived: u64,
}
impl ::core::marker::Copy for TCP_ESTATS_DATA_ROD_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_DATA_ROD_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_DATA_RW_v0 {
    pub EnableCollection: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_DATA_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_DATA_RW_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_FINE_RTT_ROD_v0 {
    pub RttVar: u32,
    pub MaxRtt: u32,
    pub MinRtt: u32,
    pub SumRtt: u32,
}
impl ::core::marker::Copy for TCP_ESTATS_FINE_RTT_ROD_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_FINE_RTT_ROD_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_FINE_RTT_RW_v0 {
    pub EnableCollection: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_FINE_RTT_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_FINE_RTT_RW_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_OBS_REC_ROD_v0 {
    pub CurRwinRcvd: u32,
    pub MaxRwinRcvd: u32,
    pub MinRwinRcvd: u32,
    pub WinScaleRcvd: u8,
}
impl ::core::marker::Copy for TCP_ESTATS_OBS_REC_ROD_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_OBS_REC_ROD_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_OBS_REC_RW_v0 {
    pub EnableCollection: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_OBS_REC_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_OBS_REC_RW_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_PATH_ROD_v0 {
    pub FastRetran: u32,
    pub Timeouts: u32,
    pub SubsequentTimeouts: u32,
    pub CurTimeoutCount: u32,
    pub AbruptTimeouts: u32,
    pub PktsRetrans: u32,
    pub BytesRetrans: u32,
    pub DupAcksIn: u32,
    pub SacksRcvd: u32,
    pub SackBlocksRcvd: u32,
    pub CongSignals: u32,
    pub PreCongSumCwnd: u32,
    pub PreCongSumRtt: u32,
    pub PostCongSumRtt: u32,
    pub PostCongCountRtt: u32,
    pub EcnSignals: u32,
    pub EceRcvd: u32,
    pub SendStall: u32,
    pub QuenchRcvd: u32,
    pub RetranThresh: u32,
    pub SndDupAckEpisodes: u32,
    pub SumBytesReordered: u32,
    pub NonRecovDa: u32,
    pub NonRecovDaEpisodes: u32,
    pub AckAfterFr: u32,
    pub DsackDups: u32,
    pub SampleRtt: u32,
    pub SmoothedRtt: u32,
    pub RttVar: u32,
    pub MaxRtt: u32,
    pub MinRtt: u32,
    pub SumRtt: u32,
    pub CountRtt: u32,
    pub CurRto: u32,
    pub MaxRto: u32,
    pub MinRto: u32,
    pub CurMss: u32,
    pub MaxMss: u32,
    pub MinMss: u32,
    pub SpuriousRtoDetections: u32,
}
impl ::core::marker::Copy for TCP_ESTATS_PATH_ROD_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_PATH_ROD_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_PATH_RW_v0 {
    pub EnableCollection: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_PATH_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_PATH_RW_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_REC_ROD_v0 {
    pub CurRwinSent: u32,
    pub MaxRwinSent: u32,
    pub MinRwinSent: u32,
    pub LimRwin: u32,
    pub DupAckEpisodes: u32,
    pub DupAcksOut: u32,
    pub CeRcvd: u32,
    pub EcnSent: u32,
    pub EcnNoncesRcvd: u32,
    pub CurReasmQueue: u32,
    pub MaxReasmQueue: u32,
    pub CurAppRQueue: usize,
    pub MaxAppRQueue: usize,
    pub WinScaleSent: u8,
}
impl ::core::marker::Copy for TCP_ESTATS_REC_ROD_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_REC_ROD_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_REC_RW_v0 {
    pub EnableCollection: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_REC_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_REC_RW_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_SEND_BUFF_ROD_v0 {
    pub CurRetxQueue: usize,
    pub MaxRetxQueue: usize,
    pub CurAppWQueue: usize,
    pub MaxAppWQueue: usize,
}
impl ::core::marker::Copy for TCP_ESTATS_SEND_BUFF_ROD_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_SEND_BUFF_ROD_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_SEND_BUFF_RW_v0 {
    pub EnableCollection: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_SEND_BUFF_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_SEND_BUFF_RW_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_SND_CONG_ROD_v0 {
    pub SndLimTransRwin: u32,
    pub SndLimTimeRwin: u32,
    pub SndLimBytesRwin: usize,
    pub SndLimTransCwnd: u32,
    pub SndLimTimeCwnd: u32,
    pub SndLimBytesCwnd: usize,
    pub SndLimTransSnd: u32,
    pub SndLimTimeSnd: u32,
    pub SndLimBytesSnd: usize,
    pub SlowStart: u32,
    pub CongAvoid: u32,
    pub OtherReductions: u32,
    pub CurCwnd: u32,
    pub MaxSsCwnd: u32,
    pub MaxCaCwnd: u32,
    pub CurSsthresh: u32,
    pub MaxSsthresh: u32,
    pub MinSsthresh: u32,
}
impl ::core::marker::Copy for TCP_ESTATS_SND_CONG_ROD_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_SND_CONG_ROD_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct TCP_ESTATS_SND_CONG_ROS_v0 {
    pub LimCwnd: u32,
}
impl ::core::marker::Copy for TCP_ESTATS_SND_CONG_ROS_v0 {}
impl ::core::clone::Clone for TCP_ESTATS_SND_CONG_ROS_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_SND_CONG_RW_v0 {
    pub EnableCollection: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_SND_CONG_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_SND_CONG_RW_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCP_ESTATS_SYN_OPTS_ROS_v0 {
    pub ActiveOpen: super::super::Foundation::BOOLEAN,
    pub MssRcvd: u32,
    pub MssSent: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCP_ESTATS_SYN_OPTS_ROS_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCP_ESTATS_SYN_OPTS_ROS_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type TCP_ESTATS_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsSynOpts: TCP_ESTATS_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsData: TCP_ESTATS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsSndCong: TCP_ESTATS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsPath: TCP_ESTATS_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsSendBuff: TCP_ESTATS_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsRec: TCP_ESTATS_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsObsRec: TCP_ESTATS_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsBandwidth: TCP_ESTATS_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsFineRtt: TCP_ESTATS_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpConnectionEstatsMaximum: TCP_ESTATS_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_ROW: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type TCP_RTO_ALGORITHM = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpRtoAlgorithmOther: TCP_RTO_ALGORITHM = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpRtoAlgorithmConstant: TCP_RTO_ALGORITHM = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpRtoAlgorithmRsre: TCP_RTO_ALGORITHM = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpRtoAlgorithmVanj: TCP_RTO_ALGORITHM = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_RTO_OTHER: TCP_RTO_ALGORITHM = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_RTO_CONSTANT: TCP_RTO_ALGORITHM = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_RTO_RSRE: TCP_RTO_ALGORITHM = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const MIB_TCP_RTO_VANJ: TCP_RTO_ALGORITHM = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type TCP_SOFT_ERROR = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorNone: TCP_SOFT_ERROR = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorBelowDataWindow: TCP_SOFT_ERROR = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorAboveDataWindow: TCP_SOFT_ERROR = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorBelowAckWindow: TCP_SOFT_ERROR = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorAboveAckWindow: TCP_SOFT_ERROR = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorBelowTsWindow: TCP_SOFT_ERROR = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorAboveTsWindow: TCP_SOFT_ERROR = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorDataChecksumError: TCP_SOFT_ERROR = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorDataLengthError: TCP_SOFT_ERROR = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TcpErrorMaxSoftError: TCP_SOFT_ERROR = 9i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_STATS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type TCP_TABLE_CLASS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_BASIC_LISTENER: TCP_TABLE_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_BASIC_CONNECTIONS: TCP_TABLE_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_BASIC_ALL: TCP_TABLE_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_OWNER_PID_LISTENER: TCP_TABLE_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_OWNER_PID_CONNECTIONS: TCP_TABLE_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_OWNER_PID_ALL: TCP_TABLE_CLASS = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_OWNER_MODULE_LISTENER: TCP_TABLE_CLASS = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_OWNER_MODULE_CONNECTIONS: TCP_TABLE_CLASS = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TCP_TABLE_OWNER_MODULE_ALL: TCP_TABLE_CLASS = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type TUNNEL_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TUNNEL_TYPE_NONE: TUNNEL_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TUNNEL_TYPE_OTHER: TUNNEL_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TUNNEL_TYPE_DIRECT: TUNNEL_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TUNNEL_TYPE_6TO4: TUNNEL_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TUNNEL_TYPE_ISATAP: TUNNEL_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TUNNEL_TYPE_TEREDO: TUNNEL_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const TUNNEL_TYPE_IPHTTPS: TUNNEL_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const UDP6_STATS: u32 = 37u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const UDP_ROW: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const UDP_STATS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const UDP_TABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub type UDP_TABLE_CLASS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const UDP_TABLE_BASIC: UDP_TABLE_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const UDP_TABLE_OWNER_PID: UDP_TABLE_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub const UDP_TABLE_OWNER_MODULE: UDP_TABLE_CLASS = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct arp_send_reply {
    pub DestAddress: u32,
    pub SrcAddress: u32,
}
impl ::core::marker::Copy for arp_send_reply {}
impl ::core::clone::Clone for arp_send_reply {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct icmp_echo_reply {
    pub Address: u32,
    pub Status: u32,
    pub RoundTripTime: u32,
    pub DataSize: u16,
    pub Reserved: u16,
    pub Data: *mut ::core::ffi::c_void,
    pub Options: ip_option_information,
}
impl ::core::marker::Copy for icmp_echo_reply {}
impl ::core::clone::Clone for icmp_echo_reply {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct icmp_echo_reply32 {
    pub Address: u32,
    pub Status: u32,
    pub RoundTripTime: u32,
    pub DataSize: u16,
    pub Reserved: u16,
    pub Data: *mut ::core::ffi::c_void,
    pub Options: ip_option_information32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for icmp_echo_reply32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for icmp_echo_reply32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct icmpv6_echo_reply_lh {
    pub Address: IPV6_ADDRESS_EX,
    pub Status: u32,
    pub RoundTripTime: u32,
}
impl ::core::marker::Copy for icmpv6_echo_reply_lh {}
impl ::core::clone::Clone for icmpv6_echo_reply_lh {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct ip_interface_name_info_w2ksp1 {
    pub Index: u32,
    pub MediaType: u32,
    pub ConnectionType: u8,
    pub AccessType: u8,
    pub DeviceGuid: ::windows_sys::core::GUID,
    pub InterfaceGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for ip_interface_name_info_w2ksp1 {}
impl ::core::clone::Clone for ip_interface_name_info_w2ksp1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct ip_option_information {
    pub Ttl: u8,
    pub Tos: u8,
    pub Flags: u8,
    pub OptionsSize: u8,
    pub OptionsData: *mut u8,
}
impl ::core::marker::Copy for ip_option_information {}
impl ::core::clone::Clone for ip_option_information {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct ip_option_information32 {
    pub Ttl: u8,
    pub Tos: u8,
    pub Flags: u8,
    pub OptionsSize: u8,
    pub OptionsData: *mut u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for ip_option_information32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for ip_option_information32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_IpHelper\"`*"]
pub struct tcp_reserve_port_range {
    pub UpperRange: u16,
    pub LowerRange: u16,
}
impl ::core::marker::Copy for tcp_reserve_port_range {}
impl ::core::clone::Clone for tcp_reserve_port_range {
    fn clone(&self) -> Self {
        *self
    }
}
