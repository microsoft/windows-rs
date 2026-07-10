#[cfg(feature = "ntddndis")]
windows_link::link!("iphlpapi.dll" "system" fn AddIPAddress(address : super::ntddndis::IPAddr, ipmask : super::ntddndis::IPMask, ifindex : u32, ntecontext : *mut u32, nteinstance : *mut u32) -> u32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn CancelIPChangeNotify(notifyoverlapped : *const super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
windows_link::link!("iphlpapi.dll" "system" fn CancelIfTimestampConfigChange(notificationhandle : HIFTIMESTAMPCHANGE));
#[cfg(feature = "ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn CaptureInterfaceHardwareCrossTimestamp(interfaceluid : *const super::ifdef::NET_LUID, crosstimestamp : *mut INTERFACE_HARDWARE_CROSSTIMESTAMP) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
windows_link::link!("iphlpapi.dll" "system" fn CreateIpForwardEntry(proute : *const super::ipmib::MIB_IPFORWARDROW) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib"))]
windows_link::link!("iphlpapi.dll" "system" fn CreateIpNetEntry(parpentry : *const super::ipmib::MIB_IPNETROW_LH) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn CreatePersistentTcpPortReservation(startport : u16, numberofports : u16, token : *mut u64) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn CreatePersistentUdpPortReservation(startport : u16, numberofports : u16, token : *mut u64) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn CreateProxyArpEntry(dwaddress : u32, dwmask : u32, dwifindex : u32) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn DeleteIPAddress(ntecontext : u32) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
windows_link::link!("iphlpapi.dll" "system" fn DeleteIpForwardEntry(proute : *const super::ipmib::MIB_IPFORWARDROW) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib"))]
windows_link::link!("iphlpapi.dll" "system" fn DeleteIpNetEntry(parpentry : *const super::ipmib::MIB_IPNETROW_LH) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn DeletePersistentTcpPortReservation(startport : u16, numberofports : u16) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn DeletePersistentUdpPortReservation(startport : u16, numberofports : u16) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn DeleteProxyArpEntry(dwaddress : u32, dwmask : u32, dwifindex : u32) -> u32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn DisableMediaSense(phandle : *mut super::winnt::HANDLE, poverlapped : *const super::minwinbase::OVERLAPPED) -> u32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn EnableRouter(phandle : *mut super::winnt::HANDLE, poverlapped : *mut super::minwinbase::OVERLAPPED) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn FlushIpNetTable(dwifindex : u32) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn GetAdapterIndex(adaptername : windows_sys::core::PCWSTR, ifindex : *mut u32) -> u32);
#[cfg(feature = "ipexport")]
windows_link::link!("iphlpapi.dll" "system" fn GetAdapterOrderMap() -> super::ipexport::PIP_ADAPTER_ORDER_MAP);
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "iptypes", feature = "nldef", feature = "winnt", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetAdaptersAddresses(family : u32, flags : u32, reserved : *const core::ffi::c_void, adapteraddresses : *mut super::iptypes::IP_ADAPTER_ADDRESSES_LH, sizepointer : *mut u32) -> u32);
#[cfg(all(feature = "corecrt", feature = "iptypes"))]
windows_link::link!("iphlpapi.dll" "system" fn GetAdaptersInfo(adapterinfo : *mut super::iptypes::IP_ADAPTER_INFO, sizepointer : *mut u32) -> u32);
#[cfg(feature = "ntddndis")]
windows_link::link!("iphlpapi.dll" "system" fn GetBestInterface(dwdestaddr : super::ntddndis::IPAddr, pdwbestifindex : *mut u32) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetBestRoute(dwdestaddr : u32, dwsourceaddr : u32, pbestroute : *mut super::ipmib::MIB_IPFORWARDROW) -> u32);
#[cfg(feature = "iprtrmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetExtendedTcpTable(ptcptable : *mut core::ffi::c_void, pdwsize : *mut u32, border : windows_sys::core::BOOL, ulaf : u32, tableclass : super::iprtrmib::TCP_TABLE_CLASS, reserved : u32) -> u32);
#[cfg(feature = "iprtrmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetExtendedUdpTable(pudptable : *mut core::ffi::c_void, pdwsize : *mut u32, border : windows_sys::core::BOOL, ulaf : u32, tableclass : super::iprtrmib::UDP_TABLE_CLASS, reserved : u32) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn GetFriendlyIfIndex(ifindex : u32) -> u32);
#[cfg(feature = "ipmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetIcmpStatistics(statistics : *mut super::ipmib::MIB_ICMP) -> u32);
#[cfg(feature = "ipmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetIcmpStatisticsEx(statistics : *mut super::ipmib::MIB_ICMP_EX_XPSP1, family : u32) -> u32);
#[cfg(all(feature = "ifdef", feature = "ifmib", feature = "ipifcons"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIfEntry(pifrow : *mut super::ifmib::MIB_IFROW) -> u32);
#[cfg(all(feature = "ifdef", feature = "ifmib", feature = "ipifcons"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIfTable(piftable : *mut super::ifmib::MIB_IFTABLE, pdwsize : *mut u32, border : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceActiveTimestampCapabilities(interfaceluid : *const super::ifdef::NET_LUID, timestampcapabilites : *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
#[cfg(feature = "ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceCurrentTimestampCapabilities(interfaceluid : *const super::ifdef::NET_LUID, timestampcapabilites : *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
#[cfg(feature = "ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceHardwareTimestampCapabilities(interfaceluid : *const super::ifdef::NET_LUID, timestampcapabilites : *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
#[cfg(feature = "ipexport")]
windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceInfo(piftable : *mut super::ipexport::IP_INTERFACE_INFO, dwoutbuflen : *mut u32) -> u32);
#[cfg(feature = "ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceSupportedTimestampCapabilities(interfaceluid : *const super::ifdef::NET_LUID, timestampcapabilites : *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpAddrTable(pipaddrtable : *mut super::ipmib::MIB_IPADDRTABLE, pdwsize : *mut u32, border : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "ipexport")]
windows_link::link!("iphlpapi.dll" "system" fn GetIpErrorString(errorcode : super::ipexport::IP_STATUS, buffer : windows_sys::core::PWSTR, size : *mut u32) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpForwardTable(pipforwardtable : *mut super::ipmib::MIB_IPFORWARDTABLE, pdwsize : *mut u32, border : windows_sys::core::BOOL) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpNetTable(ipnettable : *mut super::ipmib::MIB_IPNETTABLE, sizepointer : *mut u32, order : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "ipmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetIpStatistics(statistics : *mut super::ipmib::MIB_IPSTATS_LH) -> u32);
#[cfg(feature = "ipmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetIpStatisticsEx(statistics : *mut super::ipmib::MIB_IPSTATS_LH, family : u32) -> u32);
#[cfg(feature = "iptypes")]
windows_link::link!("iphlpapi.dll" "system" fn GetNetworkParams(pfixedinfo : *mut super::iptypes::FIXED_INFO_W2KSP1, poutbuflen : *mut u32) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn GetNumberOfInterfaces(pdwnumif : *mut u32) -> u32);
#[cfg(feature = "iprtrmib")]
windows_link::link!("iphlpapi.dll" "C" fn GetOwnerModuleFromPidAndInfo(ulpid : u32, pinfo : *const u64, class : super::iprtrmib::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : *mut u32) -> u32);
#[cfg(all(feature = "iprtrmib", feature = "tcpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromTcp6Entry(ptcpentry : *const super::tcpmib::MIB_TCP6ROW_OWNER_MODULE, class : super::iprtrmib::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : *mut u32) -> u32);
#[cfg(all(feature = "iprtrmib", feature = "tcpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromTcpEntry(ptcpentry : *const super::tcpmib::MIB_TCPROW_OWNER_MODULE, class : super::iprtrmib::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : *mut u32) -> u32);
#[cfg(all(feature = "iprtrmib", feature = "udpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromUdp6Entry(pudpentry : *const super::udpmib::MIB_UDP6ROW_OWNER_MODULE, class : super::iprtrmib::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : *mut u32) -> u32);
#[cfg(all(feature = "iprtrmib", feature = "udpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromUdpEntry(pudpentry : *const super::udpmib::MIB_UDPROW_OWNER_MODULE, class : super::iprtrmib::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : *mut u32) -> u32);
#[cfg(feature = "iptypes")]
windows_link::link!("iphlpapi.dll" "system" fn GetPerAdapterInfo(ifindex : u32, pperadapterinfo : *mut super::iptypes::IP_PER_ADAPTER_INFO_W2KSP1, poutbuflen : *mut u32) -> u32);
#[cfg(all(feature = "in6addr", feature = "tcpestats", feature = "tcpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetPerTcp6ConnectionEStats(row : *const super::tcpmib::MIB_TCP6ROW, estatstype : super::tcpestats::TCP_ESTATS_TYPE, rw : *mut u8, rwversion : u32, rwsize : u32, ros : *mut u8, rosversion : u32, rossize : u32, rod : *mut u8, rodversion : u32, rodsize : u32) -> u32);
#[cfg(all(feature = "tcpestats", feature = "tcpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetPerTcpConnectionEStats(row : *const super::tcpmib::MIB_TCPROW_LH, estatstype : super::tcpestats::TCP_ESTATS_TYPE, rw : *mut u8, rwversion : u32, rwsize : u32, ros : *mut u8, rosversion : u32, rossize : u32, rod : *mut u8, rodversion : u32, rodsize : u32) -> u32);
#[cfg(feature = "ntddndis")]
windows_link::link!("iphlpapi.dll" "system" fn GetRTTAndHopCount(destipaddress : super::ntddndis::IPAddr, hopcount : *mut u32, maxhops : u32, rtt : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "in6addr", feature = "tcpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetTcp6Table(tcptable : *mut super::tcpmib::MIB_TCP6TABLE, sizepointer : *mut u32, order : windows_sys::core::BOOL) -> u32);
#[cfg(all(feature = "in6addr", feature = "tcpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetTcp6Table2(tcptable : *mut super::tcpmib::MIB_TCP6TABLE2, sizepointer : *mut u32, order : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "tcpmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetTcpStatistics(statistics : *mut super::tcpmib::MIB_TCPSTATS_LH) -> u32);
#[cfg(feature = "tcpmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetTcpStatisticsEx(statistics : *mut super::tcpmib::MIB_TCPSTATS_LH, family : u32) -> u32);
#[cfg(feature = "tcpmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetTcpStatisticsEx2(statistics : *mut super::tcpmib::MIB_TCPSTATS2, family : u32) -> u32);
#[cfg(feature = "tcpmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetTcpTable(tcptable : *mut super::tcpmib::MIB_TCPTABLE, sizepointer : *mut u32, order : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "tcpmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetTcpTable2(tcptable : *mut super::tcpmib::MIB_TCPTABLE2, sizepointer : *mut u32, order : windows_sys::core::BOOL) -> u32);
#[cfg(all(feature = "in6addr", feature = "udpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetUdp6Table(udp6table : *mut super::udpmib::MIB_UDP6TABLE, sizepointer : *mut u32, order : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "udpmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetUdpStatistics(stats : *mut super::udpmib::MIB_UDPSTATS) -> u32);
#[cfg(feature = "udpmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetUdpStatisticsEx(statistics : *mut super::udpmib::MIB_UDPSTATS, family : u32) -> u32);
#[cfg(feature = "udpmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetUdpStatisticsEx2(statistics : *mut super::udpmib::MIB_UDPSTATS2, family : u32) -> u32);
#[cfg(feature = "udpmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetUdpTable(udptable : *mut super::udpmib::MIB_UDPTABLE, sizepointer : *mut u32, order : windows_sys::core::BOOL) -> u32);
#[cfg(all(feature = "ipexport", feature = "ntddndis"))]
windows_link::link!("iphlpapi.dll" "system" fn GetUniDirectionalAdapterInfo(pipifinfo : *mut super::ipexport::IP_UNIDIRECTIONAL_ADAPTER_ADDRESS, dwoutbuflen : *mut u32) -> u32);
#[cfg(feature = "ipexport")]
windows_link::link!("iphlpapi.dll" "system" fn IpReleaseAddress(adapterinfo : *const super::ipexport::IP_ADAPTER_INDEX_MAP) -> u32);
#[cfg(feature = "ipexport")]
windows_link::link!("iphlpapi.dll" "system" fn IpRenewAddress(adapterinfo : *const super::ipexport::IP_ADAPTER_INDEX_MAP) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn LookupPersistentTcpPortReservation(startport : u16, numberofports : u16, token : *mut u64) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn LookupPersistentUdpPortReservation(startport : u16, numberofports : u16, token : *mut u64) -> u32);
#[cfg(all(feature = "iptypes", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn NhpAllocateAndGetInterfaceInfoFromStack(pptable : *mut *mut super::iptypes::IP_INTERFACE_NAME_INFO, pdwcount : *mut u32, border : windows_sys::core::BOOL, hheap : super::winnt::HANDLE, dwflags : u32) -> u32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn NotifyAddrChange(handle : *mut super::winnt::HANDLE, overlapped : *const super::minwinbase::OVERLAPPED) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn NotifyIfTimestampConfigChange(callercontext : *const core::ffi::c_void, callback : PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, notificationhandle : *mut HIFTIMESTAMPCHANGE) -> u32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn NotifyRouteChange(handle : *mut super::winnt::HANDLE, overlapped : *const super::minwinbase::OVERLAPPED) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn RegisterInterfaceTimestampConfigChange(callback : PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, notificationhandle : *mut HIFTIMESTAMPCHANGE) -> u32);
#[cfg(feature = "ws2")]
windows_link::link!("iphlpapi.dll" "system" fn ResolveNeighbor(networkaddress : *const super::ws2::SOCKADDR, physicaladdress : *mut core::ffi::c_void, physicaladdresslength : *mut u32) -> u32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn RestoreMediaSense(poverlapped : *const super::minwinbase::OVERLAPPED, lpdwenablecount : *mut u32) -> u32);
#[cfg(feature = "ntddndis")]
windows_link::link!("iphlpapi.dll" "system" fn SendARP(destip : super::ntddndis::IPAddr, srcip : super::ntddndis::IPAddr, pmacaddr : *mut core::ffi::c_void, phyaddrlen : *mut u32) -> u32);
#[cfg(all(feature = "ifdef", feature = "ifmib", feature = "ipifcons"))]
windows_link::link!("iphlpapi.dll" "system" fn SetIfEntry(pifrow : *const super::ifmib::MIB_IFROW) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
windows_link::link!("iphlpapi.dll" "system" fn SetIpForwardEntry(proute : *const super::ipmib::MIB_IPFORWARDROW) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib"))]
windows_link::link!("iphlpapi.dll" "system" fn SetIpNetEntry(parpentry : *const super::ipmib::MIB_IPNETROW_LH) -> u32);
#[cfg(feature = "ipmib")]
windows_link::link!("iphlpapi.dll" "system" fn SetIpStatistics(pipstats : *const super::ipmib::MIB_IPSTATS_LH) -> u32);
#[cfg(feature = "ipmib")]
windows_link::link!("iphlpapi.dll" "system" fn SetIpStatisticsEx(statistics : *const super::ipmib::MIB_IPSTATS_LH, family : u32) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn SetIpTTL(nttl : u32) -> u32);
#[cfg(all(feature = "in6addr", feature = "tcpestats", feature = "tcpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn SetPerTcp6ConnectionEStats(row : *const super::tcpmib::MIB_TCP6ROW, estatstype : super::tcpestats::TCP_ESTATS_TYPE, rw : *const u8, rwversion : u32, rwsize : u32, offset : u32) -> u32);
#[cfg(all(feature = "tcpestats", feature = "tcpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn SetPerTcpConnectionEStats(row : *const super::tcpmib::MIB_TCPROW_LH, estatstype : super::tcpestats::TCP_ESTATS_TYPE, rw : *const u8, rwversion : u32, rwsize : u32, offset : u32) -> u32);
#[cfg(feature = "tcpmib")]
windows_link::link!("iphlpapi.dll" "system" fn SetTcpEntry(ptcprow : *const super::tcpmib::MIB_TCPROW_LH) -> u32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn UnenableRouter(poverlapped : *const super::minwinbase::OVERLAPPED, lpdwenablecount : *mut u32) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn UnregisterInterfaceTimestampConfigChange(notificationhandle : HIFTIMESTAMPCHANGE));
pub type HIFTIMESTAMPCHANGE = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INTERFACE_HARDWARE_CROSSTIMESTAMP {
    pub SystemTimestamp1: u64,
    pub HardwareClockTimestamp: u64,
    pub SystemTimestamp2: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {
    pub PtpV2OverUdpIPv4EventMessageReceive: bool,
    pub PtpV2OverUdpIPv4AllMessageReceive: bool,
    pub PtpV2OverUdpIPv4EventMessageTransmit: bool,
    pub PtpV2OverUdpIPv4AllMessageTransmit: bool,
    pub PtpV2OverUdpIPv6EventMessageReceive: bool,
    pub PtpV2OverUdpIPv6AllMessageReceive: bool,
    pub PtpV2OverUdpIPv6EventMessageTransmit: bool,
    pub PtpV2OverUdpIPv6AllMessageTransmit: bool,
    pub AllReceive: bool,
    pub AllTransmit: bool,
    pub TaggedTransmit: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    pub AllReceive: bool,
    pub AllTransmit: bool,
    pub TaggedTransmit: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INTERFACE_TIMESTAMP_CAPABILITIES {
    pub HardwareClockFrequencyHz: u64,
    pub SupportsCrossTimestamp: bool,
    pub HardwareCapabilities: INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES,
    pub SoftwareCapabilities: INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES,
}
pub type INTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void)>;
pub const NET_ADDRESS_DNS_NAME: NET_ADDRESS_FORMAT = 1;
pub type NET_ADDRESS_FORMAT = i32;
pub const NET_ADDRESS_FORMAT_UNSPECIFIED: NET_ADDRESS_FORMAT = 0;
pub const NET_ADDRESS_IPV4: NET_ADDRESS_FORMAT = 2;
pub const NET_ADDRESS_IPV6: NET_ADDRESS_FORMAT = 3;
pub const NET_STRING_ANY_ADDRESS: u32 = 265;
pub const NET_STRING_ANY_ADDRESS_NO_SCOPE: u32 = 273;
pub const NET_STRING_ANY_SERVICE: u32 = 546;
pub const NET_STRING_ANY_SERVICE_NO_SCOPE: u32 = 578;
pub const NET_STRING_IPV4_ADDRESS: u32 = 1;
pub const NET_STRING_IPV4_NETWORK: u32 = 4;
pub const NET_STRING_IPV4_SERVICE: u32 = 2;
pub const NET_STRING_IPV6_ADDRESS: u32 = 8;
pub const NET_STRING_IPV6_ADDRESS_NO_SCOPE: u32 = 16;
pub const NET_STRING_IPV6_NETWORK: u32 = 128;
pub const NET_STRING_IPV6_SERVICE: u32 = 32;
pub const NET_STRING_IPV6_SERVICE_NO_SCOPE: u32 = 64;
pub const NET_STRING_IP_ADDRESS: u32 = 9;
pub const NET_STRING_IP_ADDRESS_NO_SCOPE: u32 = 17;
pub const NET_STRING_IP_NETWORK: u32 = 132;
pub const NET_STRING_IP_SERVICE: u32 = 34;
pub const NET_STRING_IP_SERVICE_NO_SCOPE: u32 = 66;
pub const NET_STRING_NAMED_ADDRESS: u32 = 256;
pub const NET_STRING_NAMED_SERVICE: u32 = 512;
pub type PINTERFACE_HARDWARE_CROSSTIMESTAMP = *mut INTERFACE_HARDWARE_CROSSTIMESTAMP;
pub type PINTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES = *mut INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES;
pub type PINTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES = *mut INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES;
pub type PINTERFACE_TIMESTAMP_CAPABILITIES = *mut INTERFACE_TIMESTAMP_CAPABILITIES;
pub type PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void)>;
