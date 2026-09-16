#[cfg(all(feature = "minwindef", feature = "ntddndis"))]
windows_link::link!("iphlpapi.dll" "system" fn AddIPAddress(address : super::IPAddr, ipmask : super::IPMask, ifindex : u32, ntecontext : super::PULONG, nteinstance : super::PULONG) -> u32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn CancelIPChangeNotify(notifyoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
windows_link::link!("iphlpapi.dll" "system" fn CancelIfTimestampConfigChange(notificationhandle : HIFTIMESTAMPCHANGE));
#[cfg(feature = "ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn CaptureInterfaceHardwareCrossTimestamp(interfaceluid : *const super::NET_LUID, crosstimestamp : PINTERFACE_HARDWARE_CROSSTIMESTAMP) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
windows_link::link!("iphlpapi.dll" "system" fn CreateIpForwardEntry(proute : super::PMIB_IPFORWARDROW) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib"))]
windows_link::link!("iphlpapi.dll" "system" fn CreateIpNetEntry(parpentry : super::PMIB_IPNETROW) -> u32);
#[cfg(feature = "basetsd")]
windows_link::link!("iphlpapi.dll" "system" fn CreatePersistentTcpPortReservation(startport : u16, numberofports : u16, token : super::PULONG64) -> u32);
#[cfg(feature = "basetsd")]
windows_link::link!("iphlpapi.dll" "system" fn CreatePersistentUdpPortReservation(startport : u16, numberofports : u16, token : super::PULONG64) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn CreateProxyArpEntry(dwaddress : u32, dwmask : u32, dwifindex : u32) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn DeleteIPAddress(ntecontext : u32) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
windows_link::link!("iphlpapi.dll" "system" fn DeleteIpForwardEntry(proute : super::PMIB_IPFORWARDROW) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib"))]
windows_link::link!("iphlpapi.dll" "system" fn DeleteIpNetEntry(parpentry : super::PMIB_IPNETROW) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn DeletePersistentTcpPortReservation(startport : u16, numberofports : u16) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn DeletePersistentUdpPortReservation(startport : u16, numberofports : u16) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn DeleteProxyArpEntry(dwaddress : u32, dwmask : u32, dwifindex : u32) -> u32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn DisableMediaSense(phandle : *mut super::HANDLE, poverlapped : *const super::OVERLAPPED) -> u32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn EnableRouter(phandle : *mut super::HANDLE, poverlapped : *mut super::OVERLAPPED) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn FlushIpNetTable(dwifindex : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("iphlpapi.dll" "system" fn GetAdapterIndex(adaptername : windows_sys::core::PCWSTR, ifindex : super::PULONG) -> u32);
#[cfg(feature = "ipexport")]
windows_link::link!("iphlpapi.dll" "system" fn GetAdapterOrderMap() -> super::PIP_ADAPTER_ORDER_MAP);
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "iptypes", feature = "minwindef", feature = "nldef", feature = "winnt", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetAdaptersAddresses(family : u32, flags : u32, reserved : *mut core::ffi::c_void, adapteraddresses : super::PIP_ADAPTER_ADDRESSES, sizepointer : super::PULONG) -> u32);
#[cfg(all(feature = "corecrt", feature = "iptypes", feature = "minwindef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetAdaptersInfo(adapterinfo : super::PIP_ADAPTER_INFO, sizepointer : super::PULONG) -> u32);
#[cfg(all(feature = "minwindef", feature = "ntddndis"))]
windows_link::link!("iphlpapi.dll" "system" fn GetBestInterface(dwdestaddr : super::IPAddr, pdwbestifindex : super::PDWORD) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetBestRoute(dwdestaddr : u32, dwsourceaddr : u32, pbestroute : super::PMIB_IPFORWARDROW) -> u32);
#[cfg(all(feature = "iprtrmib", feature = "minwindef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetExtendedTcpTable(ptcptable : *mut core::ffi::c_void, pdwsize : super::PDWORD, border : windows_sys::core::BOOL, ulaf : u32, tableclass : super::TCP_TABLE_CLASS, reserved : u32) -> u32);
#[cfg(all(feature = "iprtrmib", feature = "minwindef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetExtendedUdpTable(pudptable : *mut core::ffi::c_void, pdwsize : super::PDWORD, border : windows_sys::core::BOOL, ulaf : u32, tableclass : super::UDP_TABLE_CLASS, reserved : u32) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn GetFriendlyIfIndex(ifindex : u32) -> u32);
#[cfg(feature = "ipmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetIcmpStatistics(statistics : super::PMIB_ICMP) -> u32);
#[cfg(feature = "ipmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetIcmpStatisticsEx(statistics : super::PMIB_ICMP_EX, family : u32) -> u32);
#[cfg(all(feature = "ifdef", feature = "ifmib", feature = "ipifcons"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIfEntry(pifrow : super::PMIB_IFROW) -> u32);
#[cfg(all(feature = "ifdef", feature = "ifmib", feature = "ipifcons", feature = "minwindef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIfTable(piftable : super::PMIB_IFTABLE, pdwsize : super::PULONG, border : windows_sys::core::BOOL) -> u32);
#[cfg(all(feature = "ifdef", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceActiveTimestampCapabilities(interfaceluid : *const super::NET_LUID, timestampcapabilites : PINTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
#[cfg(all(feature = "ifdef", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceCurrentTimestampCapabilities(interfaceluid : *const super::NET_LUID, timestampcapabilites : PINTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
#[cfg(all(feature = "ifdef", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceHardwareTimestampCapabilities(interfaceluid : *const super::NET_LUID, timestampcapabilites : PINTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
#[cfg(all(feature = "ipexport", feature = "minwindef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceInfo(piftable : super::PIP_INTERFACE_INFO, dwoutbuflen : super::PULONG) -> u32);
#[cfg(all(feature = "ifdef", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceSupportedTimestampCapabilities(interfaceluid : *const super::NET_LUID, timestampcapabilites : PINTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "minwindef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpAddrTable(pipaddrtable : super::PMIB_IPADDRTABLE, pdwsize : super::PULONG, border : windows_sys::core::BOOL) -> u32);
#[cfg(all(feature = "ipexport", feature = "minwindef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpErrorString(errorcode : super::IP_STATUS, buffer : windows_sys::core::PWSTR, size : super::PDWORD) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "minwindef", feature = "nldef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpForwardTable(pipforwardtable : super::PMIB_IPFORWARDTABLE, pdwsize : super::PULONG, border : windows_sys::core::BOOL) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "minwindef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpNetTable(ipnettable : super::PMIB_IPNETTABLE, sizepointer : super::PULONG, order : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "ipmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetIpStatistics(statistics : super::PMIB_IPSTATS) -> u32);
#[cfg(feature = "ipmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetIpStatisticsEx(statistics : super::PMIB_IPSTATS, family : u32) -> u32);
#[cfg(all(feature = "iptypes", feature = "minwindef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetNetworkParams(pfixedinfo : super::PFIXED_INFO, poutbuflen : super::PULONG) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("iphlpapi.dll" "system" fn GetNumberOfInterfaces(pdwnumif : super::PDWORD) -> u32);
#[cfg(all(feature = "iprtrmib", feature = "minwindef"))]
windows_link::link!("iphlpapi.dll" "C" fn GetOwnerModuleFromPidAndInfo(ulpid : u32, pinfo : *const u64, class : super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : super::PDWORD) -> u32);
#[cfg(all(feature = "iprtrmib", feature = "minwindef", feature = "tcpmib", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromTcp6Entry(ptcpentry : super::PMIB_TCP6ROW_OWNER_MODULE, class : super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : super::PDWORD) -> u32);
#[cfg(all(feature = "iprtrmib", feature = "minwindef", feature = "tcpmib", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromTcpEntry(ptcpentry : super::PMIB_TCPROW_OWNER_MODULE, class : super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : super::PDWORD) -> u32);
#[cfg(all(feature = "iprtrmib", feature = "minwindef", feature = "udpmib", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromUdp6Entry(pudpentry : super::PMIB_UDP6ROW_OWNER_MODULE, class : super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : super::PDWORD) -> u32);
#[cfg(all(feature = "iprtrmib", feature = "minwindef", feature = "udpmib", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromUdpEntry(pudpentry : super::PMIB_UDPROW_OWNER_MODULE, class : super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : super::PDWORD) -> u32);
#[cfg(all(feature = "iptypes", feature = "minwindef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetPerAdapterInfo(ifindex : u32, pperadapterinfo : super::PIP_PER_ADAPTER_INFO, poutbuflen : super::PULONG) -> u32);
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "tcpestats", feature = "tcpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetPerTcp6ConnectionEStats(row : super::PMIB_TCP6ROW, estatstype : super::TCP_ESTATS_TYPE, rw : super::PUCHAR, rwversion : u32, rwsize : u32, ros : super::PUCHAR, rosversion : u32, rossize : u32, rod : super::PUCHAR, rodversion : u32, rodsize : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "tcpestats", feature = "tcpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetPerTcpConnectionEStats(row : super::PMIB_TCPROW, estatstype : super::TCP_ESTATS_TYPE, rw : super::PUCHAR, rwversion : u32, rwsize : u32, ros : super::PUCHAR, rosversion : u32, rossize : u32, rod : super::PUCHAR, rodversion : u32, rodsize : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "ntddndis"))]
windows_link::link!("iphlpapi.dll" "system" fn GetRTTAndHopCount(destipaddress : super::IPAddr, hopcount : super::PULONG, maxhops : u32, rtt : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "tcpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetTcp6Table(tcptable : super::PMIB_TCP6TABLE, sizepointer : super::PULONG, order : windows_sys::core::BOOL) -> u32);
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "tcpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetTcp6Table2(tcptable : super::PMIB_TCP6TABLE2, sizepointer : super::PULONG, order : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "tcpmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetTcpStatistics(statistics : super::PMIB_TCPSTATS) -> u32);
#[cfg(feature = "tcpmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetTcpStatisticsEx(statistics : super::PMIB_TCPSTATS, family : u32) -> u32);
#[cfg(feature = "tcpmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetTcpStatisticsEx2(statistics : super::PMIB_TCPSTATS2, family : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "tcpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetTcpTable(tcptable : super::PMIB_TCPTABLE, sizepointer : super::PULONG, order : windows_sys::core::BOOL) -> u32);
#[cfg(all(feature = "minwindef", feature = "tcpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetTcpTable2(tcptable : super::PMIB_TCPTABLE2, sizepointer : super::PULONG, order : windows_sys::core::BOOL) -> u32);
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "udpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetUdp6Table(udp6table : super::PMIB_UDP6TABLE, sizepointer : super::PULONG, order : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "udpmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetUdpStatistics(stats : super::PMIB_UDPSTATS) -> u32);
#[cfg(feature = "udpmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetUdpStatisticsEx(statistics : super::PMIB_UDPSTATS, family : u32) -> u32);
#[cfg(feature = "udpmib")]
windows_link::link!("iphlpapi.dll" "system" fn GetUdpStatisticsEx2(statistics : super::PMIB_UDPSTATS2, family : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "udpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn GetUdpTable(udptable : super::PMIB_UDPTABLE, sizepointer : super::PULONG, order : windows_sys::core::BOOL) -> u32);
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "ntddndis"))]
windows_link::link!("iphlpapi.dll" "system" fn GetUniDirectionalAdapterInfo(pipifinfo : super::PIP_UNIDIRECTIONAL_ADAPTER_ADDRESS, dwoutbuflen : super::PULONG) -> u32);
#[cfg(feature = "ipexport")]
windows_link::link!("iphlpapi.dll" "system" fn IpReleaseAddress(adapterinfo : super::PIP_ADAPTER_INDEX_MAP) -> u32);
#[cfg(feature = "ipexport")]
windows_link::link!("iphlpapi.dll" "system" fn IpRenewAddress(adapterinfo : super::PIP_ADAPTER_INDEX_MAP) -> u32);
#[cfg(feature = "basetsd")]
windows_link::link!("iphlpapi.dll" "system" fn LookupPersistentTcpPortReservation(startport : u16, numberofports : u16, token : super::PULONG64) -> u32);
#[cfg(feature = "basetsd")]
windows_link::link!("iphlpapi.dll" "system" fn LookupPersistentUdpPortReservation(startport : u16, numberofports : u16, token : super::PULONG64) -> u32);
#[cfg(all(feature = "iptypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn NhpAllocateAndGetInterfaceInfoFromStack(pptable : *mut *mut super::IP_INTERFACE_NAME_INFO, pdwcount : super::PDWORD, border : windows_sys::core::BOOL, hheap : super::HANDLE, dwflags : u32) -> u32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn NotifyAddrChange(handle : super::PHANDLE, overlapped : super::LPOVERLAPPED) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn NotifyIfTimestampConfigChange(callercontext : *const core::ffi::c_void, callback : PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, notificationhandle : *mut HIFTIMESTAMPCHANGE) -> u32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn NotifyRouteChange(handle : super::PHANDLE, overlapped : super::LPOVERLAPPED) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn RegisterInterfaceTimestampConfigChange(callback : PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, notificationhandle : *mut HIFTIMESTAMPCHANGE) -> u32);
#[cfg(all(feature = "minwindef", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn ResolveNeighbor(networkaddress : *const super::SOCKADDR, physicaladdress : *mut core::ffi::c_void, physicaladdresslength : super::PULONG) -> u32);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn RestoreMediaSense(poverlapped : *const super::OVERLAPPED, lpdwenablecount : super::LPDWORD) -> u32);
#[cfg(all(feature = "minwindef", feature = "ntddndis"))]
windows_link::link!("iphlpapi.dll" "system" fn SendARP(destip : super::IPAddr, srcip : super::IPAddr, pmacaddr : *mut core::ffi::c_void, phyaddrlen : super::PULONG) -> u32);
#[cfg(all(feature = "ifdef", feature = "ifmib", feature = "ipifcons"))]
windows_link::link!("iphlpapi.dll" "system" fn SetIfEntry(pifrow : super::PMIB_IFROW) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
windows_link::link!("iphlpapi.dll" "system" fn SetIpForwardEntry(proute : super::PMIB_IPFORWARDROW) -> u32);
#[cfg(all(feature = "ifdef", feature = "ipmib"))]
windows_link::link!("iphlpapi.dll" "system" fn SetIpNetEntry(parpentry : super::PMIB_IPNETROW) -> u32);
#[cfg(feature = "ipmib")]
windows_link::link!("iphlpapi.dll" "system" fn SetIpStatistics(pipstats : super::PMIB_IPSTATS) -> u32);
#[cfg(feature = "ipmib")]
windows_link::link!("iphlpapi.dll" "system" fn SetIpStatisticsEx(statistics : super::PMIB_IPSTATS, family : u32) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn SetIpTTL(nttl : u32) -> u32);
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "tcpestats", feature = "tcpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn SetPerTcp6ConnectionEStats(row : super::PMIB_TCP6ROW, estatstype : super::TCP_ESTATS_TYPE, rw : super::PUCHAR, rwversion : u32, rwsize : u32, offset : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "tcpestats", feature = "tcpmib"))]
windows_link::link!("iphlpapi.dll" "system" fn SetPerTcpConnectionEStats(row : super::PMIB_TCPROW, estatstype : super::TCP_ESTATS_TYPE, rw : super::PUCHAR, rwversion : u32, rwsize : u32, offset : u32) -> u32);
#[cfg(feature = "tcpmib")]
windows_link::link!("iphlpapi.dll" "system" fn SetTcpEntry(ptcprow : super::PMIB_TCPROW) -> u32);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn UnenableRouter(poverlapped : *const super::OVERLAPPED, lpdwenablecount : super::LPDWORD) -> u32);
windows_link::link!("iphlpapi.dll" "system" fn UnregisterInterfaceTimestampConfigChange(notificationhandle : HIFTIMESTAMPCHANGE));
pub type HIFTIMESTAMPCHANGE = *mut HIFTIMESTAMPCHANGE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HIFTIMESTAMPCHANGE__ {
    pub unused: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INTERFACE_HARDWARE_CROSSTIMESTAMP {
    pub SystemTimestamp1: u64,
    pub HardwareClockTimestamp: u64,
    pub SystemTimestamp2: u64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {
    pub PtpV2OverUdpIPv4EventMessageReceive: super::BOOLEAN,
    pub PtpV2OverUdpIPv4AllMessageReceive: super::BOOLEAN,
    pub PtpV2OverUdpIPv4EventMessageTransmit: super::BOOLEAN,
    pub PtpV2OverUdpIPv4AllMessageTransmit: super::BOOLEAN,
    pub PtpV2OverUdpIPv6EventMessageReceive: super::BOOLEAN,
    pub PtpV2OverUdpIPv6AllMessageReceive: super::BOOLEAN,
    pub PtpV2OverUdpIPv6EventMessageTransmit: super::BOOLEAN,
    pub PtpV2OverUdpIPv6AllMessageTransmit: super::BOOLEAN,
    pub AllReceive: super::BOOLEAN,
    pub AllTransmit: super::BOOLEAN,
    pub TaggedTransmit: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    pub AllReceive: super::BOOLEAN,
    pub AllTransmit: super::BOOLEAN,
    pub TaggedTransmit: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct INTERFACE_TIMESTAMP_CAPABILITIES {
    pub HardwareClockFrequencyHz: u64,
    pub SupportsCrossTimestamp: super::BOOLEAN,
    pub HardwareCapabilities: INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES,
    pub SoftwareCapabilities: INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES,
}
pub type INTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void)>;
pub const NET_ADDRESS_DNS_NAME: NET_ADDRESS_FORMAT = 1;
pub type NET_ADDRESS_FORMAT = i32;
pub const NET_ADDRESS_FORMAT_UNSPECIFIED: NET_ADDRESS_FORMAT = 0;
pub const NET_ADDRESS_IPV4: NET_ADDRESS_FORMAT = 2;
pub const NET_ADDRESS_IPV6: NET_ADDRESS_FORMAT = 3;
pub const NET_STRING_ANY_ADDRESS: i32 = 265;
pub const NET_STRING_ANY_ADDRESS_NO_SCOPE: i32 = 273;
pub const NET_STRING_ANY_SERVICE: i32 = 546;
pub const NET_STRING_ANY_SERVICE_NO_SCOPE: i32 = 578;
pub const NET_STRING_IPV4_ADDRESS: i32 = 1;
pub const NET_STRING_IPV4_NETWORK: i32 = 4;
pub const NET_STRING_IPV4_SERVICE: i32 = 2;
pub const NET_STRING_IPV6_ADDRESS: i32 = 8;
pub const NET_STRING_IPV6_ADDRESS_NO_SCOPE: i32 = 16;
pub const NET_STRING_IPV6_NETWORK: i32 = 128;
pub const NET_STRING_IPV6_SERVICE: i32 = 32;
pub const NET_STRING_IPV6_SERVICE_NO_SCOPE: i32 = 64;
pub const NET_STRING_IP_ADDRESS: i32 = 9;
pub const NET_STRING_IP_ADDRESS_NO_SCOPE: i32 = 17;
pub const NET_STRING_IP_NETWORK: i32 = 132;
pub const NET_STRING_IP_SERVICE: i32 = 34;
pub const NET_STRING_IP_SERVICE_NO_SCOPE: i32 = 66;
pub const NET_STRING_NAMED_ADDRESS: i32 = 256;
pub const NET_STRING_NAMED_SERVICE: i32 = 512;
pub type PINTERFACE_HARDWARE_CROSSTIMESTAMP = *mut INTERFACE_HARDWARE_CROSSTIMESTAMP;
#[cfg(feature = "winnt")]
pub type PINTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES = *mut INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES;
#[cfg(feature = "winnt")]
pub type PINTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES = *mut INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES;
#[cfg(feature = "winnt")]
pub type PINTERFACE_TIMESTAMP_CAPABILITIES = *mut INTERFACE_TIMESTAMP_CAPABILITIES;
pub type PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void)>;
