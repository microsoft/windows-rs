#[cfg(all(feature = "minwindef", feature = "ntddndis"))]
#[inline]
pub unsafe fn AddIPAddress(address: super::IPAddr, ipmask: super::IPMask, ifindex: u32, ntecontext: super::PULONG, nteinstance: super::PULONG) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn AddIPAddress(address : super::IPAddr, ipmask : super::IPMask, ifindex : u32, ntecontext : super::PULONG, nteinstance : super::PULONG) -> u32);
    unsafe { AddIPAddress(address, ipmask, ifindex, ntecontext as _, nteinstance as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CancelIPChangeNotify(notifyoverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("iphlpapi.dll" "system" fn CancelIPChangeNotify(notifyoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { CancelIPChangeNotify(notifyoverlapped) }
}
#[inline]
pub unsafe fn CancelIfTimestampConfigChange(notificationhandle: HIFTIMESTAMPCHANGE) {
    windows_core::link!("iphlpapi.dll" "system" fn CancelIfTimestampConfigChange(notificationhandle : HIFTIMESTAMPCHANGE));
    unsafe { CancelIfTimestampConfigChange(notificationhandle) }
}
#[cfg(feature = "ifdef")]
#[inline]
pub unsafe fn CaptureInterfaceHardwareCrossTimestamp(interfaceluid: *const super::NET_LUID, crosstimestamp: PINTERFACE_HARDWARE_CROSSTIMESTAMP) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn CaptureInterfaceHardwareCrossTimestamp(interfaceluid : *const super::NET_LUID, crosstimestamp : PINTERFACE_HARDWARE_CROSSTIMESTAMP) -> u32);
    unsafe { CaptureInterfaceHardwareCrossTimestamp(interfaceluid, crosstimestamp as _) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
#[inline]
pub unsafe fn CreateIpForwardEntry(proute: super::PMIB_IPFORWARDROW) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn CreateIpForwardEntry(proute : super::PMIB_IPFORWARDROW) -> u32);
    unsafe { CreateIpForwardEntry(proute) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib"))]
#[inline]
pub unsafe fn CreateIpNetEntry(parpentry: super::PMIB_IPNETROW) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn CreateIpNetEntry(parpentry : super::PMIB_IPNETROW) -> u32);
    unsafe { CreateIpNetEntry(parpentry) }
}
#[cfg(feature = "basetsd")]
#[inline]
pub unsafe fn CreatePersistentTcpPortReservation(startport: u16, numberofports: u16, token: super::PULONG64) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn CreatePersistentTcpPortReservation(startport : u16, numberofports : u16, token : super::PULONG64) -> u32);
    unsafe { CreatePersistentTcpPortReservation(startport, numberofports, token as _) }
}
#[cfg(feature = "basetsd")]
#[inline]
pub unsafe fn CreatePersistentUdpPortReservation(startport: u16, numberofports: u16, token: super::PULONG64) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn CreatePersistentUdpPortReservation(startport : u16, numberofports : u16, token : super::PULONG64) -> u32);
    unsafe { CreatePersistentUdpPortReservation(startport, numberofports, token as _) }
}
#[inline]
pub unsafe fn CreateProxyArpEntry(dwaddress: u32, dwmask: u32, dwifindex: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn CreateProxyArpEntry(dwaddress : u32, dwmask : u32, dwifindex : u32) -> u32);
    unsafe { CreateProxyArpEntry(dwaddress, dwmask, dwifindex) }
}
#[inline]
pub unsafe fn DeleteIPAddress(ntecontext: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn DeleteIPAddress(ntecontext : u32) -> u32);
    unsafe { DeleteIPAddress(ntecontext) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
#[inline]
pub unsafe fn DeleteIpForwardEntry(proute: super::PMIB_IPFORWARDROW) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn DeleteIpForwardEntry(proute : super::PMIB_IPFORWARDROW) -> u32);
    unsafe { DeleteIpForwardEntry(proute) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib"))]
#[inline]
pub unsafe fn DeleteIpNetEntry(parpentry: super::PMIB_IPNETROW) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn DeleteIpNetEntry(parpentry : super::PMIB_IPNETROW) -> u32);
    unsafe { DeleteIpNetEntry(parpentry) }
}
#[inline]
pub unsafe fn DeletePersistentTcpPortReservation(startport: u16, numberofports: u16) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn DeletePersistentTcpPortReservation(startport : u16, numberofports : u16) -> u32);
    unsafe { DeletePersistentTcpPortReservation(startport, numberofports) }
}
#[inline]
pub unsafe fn DeletePersistentUdpPortReservation(startport: u16, numberofports: u16) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn DeletePersistentUdpPortReservation(startport : u16, numberofports : u16) -> u32);
    unsafe { DeletePersistentUdpPortReservation(startport, numberofports) }
}
#[inline]
pub unsafe fn DeleteProxyArpEntry(dwaddress: u32, dwmask: u32, dwifindex: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn DeleteProxyArpEntry(dwaddress : u32, dwmask : u32, dwifindex : u32) -> u32);
    unsafe { DeleteProxyArpEntry(dwaddress, dwmask, dwifindex) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn DisableMediaSense(phandle: *mut super::HANDLE, poverlapped: *const super::OVERLAPPED) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn DisableMediaSense(phandle : *mut super::HANDLE, poverlapped : *const super::OVERLAPPED) -> u32);
    unsafe { DisableMediaSense(phandle as _, poverlapped) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn EnableRouter(phandle: *mut super::HANDLE, poverlapped: *mut super::OVERLAPPED) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn EnableRouter(phandle : *mut super::HANDLE, poverlapped : *mut super::OVERLAPPED) -> u32);
    unsafe { EnableRouter(phandle as _, poverlapped as _) }
}
#[inline]
pub unsafe fn FlushIpNetTable(dwifindex: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn FlushIpNetTable(dwifindex : u32) -> u32);
    unsafe { FlushIpNetTable(dwifindex) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetAdapterIndex<P0>(adaptername: P0, ifindex: super::PULONG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("iphlpapi.dll" "system" fn GetAdapterIndex(adaptername : windows_core::PCWSTR, ifindex : super::PULONG) -> u32);
    unsafe { GetAdapterIndex(adaptername.param().abi(), ifindex as _) }
}
#[cfg(feature = "ipexport")]
#[inline]
pub unsafe fn GetAdapterOrderMap() -> super::PIP_ADAPTER_ORDER_MAP {
    windows_core::link!("iphlpapi.dll" "system" fn GetAdapterOrderMap() -> super::PIP_ADAPTER_ORDER_MAP);
    unsafe { GetAdapterOrderMap() }
}
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "iptypes", feature = "minwindef", feature = "nldef", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn GetAdaptersAddresses(family: u32, flags: u32, reserved: Option<*mut core::ffi::c_void>, adapteraddresses: Option<super::PIP_ADAPTER_ADDRESSES>, sizepointer: super::PULONG) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetAdaptersAddresses(family : u32, flags : u32, reserved : *mut core::ffi::c_void, adapteraddresses : super::PIP_ADAPTER_ADDRESSES, sizepointer : super::PULONG) -> u32);
    unsafe { GetAdaptersAddresses(family, flags, reserved.unwrap_or(core::mem::zeroed()) as _, adapteraddresses.unwrap_or(core::mem::zeroed()) as _, sizepointer as _) }
}
#[cfg(all(feature = "corecrt", feature = "iptypes", feature = "minwindef"))]
#[inline]
pub unsafe fn GetAdaptersInfo(adapterinfo: Option<super::PIP_ADAPTER_INFO>, sizepointer: super::PULONG) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetAdaptersInfo(adapterinfo : super::PIP_ADAPTER_INFO, sizepointer : super::PULONG) -> u32);
    unsafe { GetAdaptersInfo(adapterinfo.unwrap_or(core::mem::zeroed()) as _, sizepointer as _) }
}
#[cfg(all(feature = "minwindef", feature = "ntddndis"))]
#[inline]
pub unsafe fn GetBestInterface(dwdestaddr: super::IPAddr, pdwbestifindex: super::PDWORD) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetBestInterface(dwdestaddr : super::IPAddr, pdwbestifindex : super::PDWORD) -> u32);
    unsafe { GetBestInterface(dwdestaddr, pdwbestifindex as _) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
#[inline]
pub unsafe fn GetBestRoute(dwdestaddr: u32, dwsourceaddr: Option<u32>, pbestroute: super::PMIB_IPFORWARDROW) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetBestRoute(dwdestaddr : u32, dwsourceaddr : u32, pbestroute : super::PMIB_IPFORWARDROW) -> u32);
    unsafe { GetBestRoute(dwdestaddr, dwsourceaddr.unwrap_or(core::mem::zeroed()) as _, pbestroute as _) }
}
#[cfg(all(feature = "iprtrmib", feature = "minwindef"))]
#[inline]
pub unsafe fn GetExtendedTcpTable(ptcptable: Option<*mut core::ffi::c_void>, pdwsize: super::PDWORD, border: bool, ulaf: u32, tableclass: super::TCP_TABLE_CLASS, reserved: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetExtendedTcpTable(ptcptable : *mut core::ffi::c_void, pdwsize : super::PDWORD, border : windows_core::BOOL, ulaf : u32, tableclass : super::TCP_TABLE_CLASS, reserved : u32) -> u32);
    unsafe { GetExtendedTcpTable(ptcptable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into(), ulaf, tableclass, reserved) }
}
#[cfg(all(feature = "iprtrmib", feature = "minwindef"))]
#[inline]
pub unsafe fn GetExtendedUdpTable(pudptable: Option<*mut core::ffi::c_void>, pdwsize: super::PDWORD, border: bool, ulaf: u32, tableclass: super::UDP_TABLE_CLASS, reserved: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetExtendedUdpTable(pudptable : *mut core::ffi::c_void, pdwsize : super::PDWORD, border : windows_core::BOOL, ulaf : u32, tableclass : super::UDP_TABLE_CLASS, reserved : u32) -> u32);
    unsafe { GetExtendedUdpTable(pudptable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into(), ulaf, tableclass, reserved) }
}
#[inline]
pub unsafe fn GetFriendlyIfIndex(ifindex: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetFriendlyIfIndex(ifindex : u32) -> u32);
    unsafe { GetFriendlyIfIndex(ifindex) }
}
#[cfg(feature = "ipmib")]
#[inline]
pub unsafe fn GetIcmpStatistics(statistics: super::PMIB_ICMP) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIcmpStatistics(statistics : super::PMIB_ICMP) -> u32);
    unsafe { GetIcmpStatistics(statistics as _) }
}
#[cfg(feature = "ipmib")]
#[inline]
pub unsafe fn GetIcmpStatisticsEx(statistics: super::PMIB_ICMP_EX, family: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIcmpStatisticsEx(statistics : super::PMIB_ICMP_EX, family : u32) -> u32);
    unsafe { GetIcmpStatisticsEx(statistics as _, family) }
}
#[cfg(all(feature = "ifdef", feature = "ifmib", feature = "ipifcons"))]
#[inline]
pub unsafe fn GetIfEntry(pifrow: super::PMIB_IFROW) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIfEntry(pifrow : super::PMIB_IFROW) -> u32);
    unsafe { GetIfEntry(pifrow as _) }
}
#[cfg(all(feature = "ifdef", feature = "ifmib", feature = "ipifcons", feature = "minwindef"))]
#[inline]
pub unsafe fn GetIfTable(piftable: Option<super::PMIB_IFTABLE>, pdwsize: super::PULONG, border: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIfTable(piftable : super::PMIB_IFTABLE, pdwsize : super::PULONG, border : windows_core::BOOL) -> u32);
    unsafe { GetIfTable(piftable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into()) }
}
#[cfg(all(feature = "ifdef", feature = "winnt"))]
#[inline]
pub unsafe fn GetInterfaceActiveTimestampCapabilities(interfaceluid: *const super::NET_LUID, timestampcapabilites: PINTERFACE_TIMESTAMP_CAPABILITIES) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetInterfaceActiveTimestampCapabilities(interfaceluid : *const super::NET_LUID, timestampcapabilites : PINTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
    unsafe { GetInterfaceActiveTimestampCapabilities(interfaceluid, timestampcapabilites as _) }
}
#[cfg(all(feature = "ifdef", feature = "winnt"))]
#[inline]
pub unsafe fn GetInterfaceCurrentTimestampCapabilities(interfaceluid: *const super::NET_LUID, timestampcapabilites: PINTERFACE_TIMESTAMP_CAPABILITIES) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetInterfaceCurrentTimestampCapabilities(interfaceluid : *const super::NET_LUID, timestampcapabilites : PINTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
    unsafe { GetInterfaceCurrentTimestampCapabilities(interfaceluid, timestampcapabilites as _) }
}
#[cfg(all(feature = "ifdef", feature = "winnt"))]
#[inline]
pub unsafe fn GetInterfaceHardwareTimestampCapabilities(interfaceluid: *const super::NET_LUID, timestampcapabilites: PINTERFACE_TIMESTAMP_CAPABILITIES) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetInterfaceHardwareTimestampCapabilities(interfaceluid : *const super::NET_LUID, timestampcapabilites : PINTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
    unsafe { GetInterfaceHardwareTimestampCapabilities(interfaceluid, timestampcapabilites as _) }
}
#[cfg(all(feature = "ipexport", feature = "minwindef"))]
#[inline]
pub unsafe fn GetInterfaceInfo(piftable: Option<super::PIP_INTERFACE_INFO>, dwoutbuflen: super::PULONG) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetInterfaceInfo(piftable : super::PIP_INTERFACE_INFO, dwoutbuflen : super::PULONG) -> u32);
    unsafe { GetInterfaceInfo(piftable.unwrap_or(core::mem::zeroed()) as _, dwoutbuflen as _) }
}
#[cfg(all(feature = "ifdef", feature = "winnt"))]
#[inline]
pub unsafe fn GetInterfaceSupportedTimestampCapabilities(interfaceluid: *const super::NET_LUID, timestampcapabilites: PINTERFACE_TIMESTAMP_CAPABILITIES) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetInterfaceSupportedTimestampCapabilities(interfaceluid : *const super::NET_LUID, timestampcapabilites : PINTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
    unsafe { GetInterfaceSupportedTimestampCapabilities(interfaceluid, timestampcapabilites as _) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "minwindef"))]
#[inline]
pub unsafe fn GetIpAddrTable(pipaddrtable: Option<super::PMIB_IPADDRTABLE>, pdwsize: super::PULONG, border: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpAddrTable(pipaddrtable : super::PMIB_IPADDRTABLE, pdwsize : super::PULONG, border : windows_core::BOOL) -> u32);
    unsafe { GetIpAddrTable(pipaddrtable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into()) }
}
#[cfg(all(feature = "ipexport", feature = "minwindef"))]
#[inline]
pub unsafe fn GetIpErrorString(errorcode: super::IP_STATUS, buffer: Option<windows_core::PWSTR>, size: super::PDWORD) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpErrorString(errorcode : super::IP_STATUS, buffer : windows_core::PWSTR, size : super::PDWORD) -> u32);
    unsafe { GetIpErrorString(errorcode, buffer.unwrap_or(core::mem::zeroed()) as _, size as _) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "minwindef", feature = "nldef"))]
#[inline]
pub unsafe fn GetIpForwardTable(pipforwardtable: Option<super::PMIB_IPFORWARDTABLE>, pdwsize: super::PULONG, border: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpForwardTable(pipforwardtable : super::PMIB_IPFORWARDTABLE, pdwsize : super::PULONG, border : windows_core::BOOL) -> u32);
    unsafe { GetIpForwardTable(pipforwardtable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into()) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "minwindef"))]
#[inline]
pub unsafe fn GetIpNetTable(ipnettable: Option<super::PMIB_IPNETTABLE>, sizepointer: super::PULONG, order: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpNetTable(ipnettable : super::PMIB_IPNETTABLE, sizepointer : super::PULONG, order : windows_core::BOOL) -> u32);
    unsafe { GetIpNetTable(ipnettable.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[cfg(feature = "ipmib")]
#[inline]
pub unsafe fn GetIpStatistics(statistics: super::PMIB_IPSTATS) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpStatistics(statistics : super::PMIB_IPSTATS) -> u32);
    unsafe { GetIpStatistics(statistics as _) }
}
#[cfg(feature = "ipmib")]
#[inline]
pub unsafe fn GetIpStatisticsEx(statistics: super::PMIB_IPSTATS, family: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpStatisticsEx(statistics : super::PMIB_IPSTATS, family : u32) -> u32);
    unsafe { GetIpStatisticsEx(statistics as _, family) }
}
#[cfg(all(feature = "iptypes", feature = "minwindef"))]
#[inline]
pub unsafe fn GetNetworkParams(pfixedinfo: Option<super::PFIXED_INFO>, poutbuflen: super::PULONG) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetNetworkParams(pfixedinfo : super::PFIXED_INFO, poutbuflen : super::PULONG) -> u32);
    unsafe { GetNetworkParams(pfixedinfo.unwrap_or(core::mem::zeroed()) as _, poutbuflen as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetNumberOfInterfaces(pdwnumif: super::PDWORD) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetNumberOfInterfaces(pdwnumif : super::PDWORD) -> u32);
    unsafe { GetNumberOfInterfaces(pdwnumif as _) }
}
#[cfg(all(feature = "iprtrmib", feature = "minwindef"))]
#[inline]
pub unsafe fn GetOwnerModuleFromPidAndInfo(ulpid: u32, pinfo: *const u64, class: super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut core::ffi::c_void, pdwsize: super::PDWORD) -> u32 {
    windows_core::link!("iphlpapi.dll" "C" fn GetOwnerModuleFromPidAndInfo(ulpid : u32, pinfo : *const u64, class : super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : super::PDWORD) -> u32);
    unsafe { GetOwnerModuleFromPidAndInfo(ulpid, pinfo, class, pbuffer as _, pdwsize as _) }
}
#[cfg(all(feature = "iprtrmib", feature = "minwindef", feature = "tcpmib", feature = "winnt"))]
#[inline]
pub unsafe fn GetOwnerModuleFromTcp6Entry(ptcpentry: super::PMIB_TCP6ROW_OWNER_MODULE, class: super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut core::ffi::c_void, pdwsize: super::PDWORD) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromTcp6Entry(ptcpentry : super::PMIB_TCP6ROW_OWNER_MODULE, class : super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : super::PDWORD) -> u32);
    unsafe { GetOwnerModuleFromTcp6Entry(ptcpentry, class, pbuffer as _, pdwsize as _) }
}
#[cfg(all(feature = "iprtrmib", feature = "minwindef", feature = "tcpmib", feature = "winnt"))]
#[inline]
pub unsafe fn GetOwnerModuleFromTcpEntry(ptcpentry: super::PMIB_TCPROW_OWNER_MODULE, class: super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut core::ffi::c_void, pdwsize: super::PDWORD) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromTcpEntry(ptcpentry : super::PMIB_TCPROW_OWNER_MODULE, class : super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : super::PDWORD) -> u32);
    unsafe { GetOwnerModuleFromTcpEntry(ptcpentry, class, pbuffer as _, pdwsize as _) }
}
#[cfg(all(feature = "iprtrmib", feature = "minwindef", feature = "udpmib", feature = "winnt"))]
#[inline]
pub unsafe fn GetOwnerModuleFromUdp6Entry(pudpentry: super::PMIB_UDP6ROW_OWNER_MODULE, class: super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut core::ffi::c_void, pdwsize: super::PDWORD) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromUdp6Entry(pudpentry : super::PMIB_UDP6ROW_OWNER_MODULE, class : super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : super::PDWORD) -> u32);
    unsafe { GetOwnerModuleFromUdp6Entry(pudpentry, class, pbuffer as _, pdwsize as _) }
}
#[cfg(all(feature = "iprtrmib", feature = "minwindef", feature = "udpmib", feature = "winnt"))]
#[inline]
pub unsafe fn GetOwnerModuleFromUdpEntry(pudpentry: super::PMIB_UDPROW_OWNER_MODULE, class: super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut core::ffi::c_void, pdwsize: super::PDWORD) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromUdpEntry(pudpentry : super::PMIB_UDPROW_OWNER_MODULE, class : super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : super::PDWORD) -> u32);
    unsafe { GetOwnerModuleFromUdpEntry(pudpentry, class, pbuffer as _, pdwsize as _) }
}
#[cfg(all(feature = "iptypes", feature = "minwindef"))]
#[inline]
pub unsafe fn GetPerAdapterInfo(ifindex: u32, pperadapterinfo: Option<super::PIP_PER_ADAPTER_INFO>, poutbuflen: super::PULONG) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetPerAdapterInfo(ifindex : u32, pperadapterinfo : super::PIP_PER_ADAPTER_INFO, poutbuflen : super::PULONG) -> u32);
    unsafe { GetPerAdapterInfo(ifindex, pperadapterinfo.unwrap_or(core::mem::zeroed()) as _, poutbuflen as _) }
}
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "tcpestats", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetPerTcp6ConnectionEStats(row: super::PMIB_TCP6ROW, estatstype: super::TCP_ESTATS_TYPE, rw: Option<super::PUCHAR>, rwversion: u32, rwsize: u32, ros: Option<super::PUCHAR>, rosversion: u32, rossize: u32, rod: Option<super::PUCHAR>, rodversion: u32, rodsize: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetPerTcp6ConnectionEStats(row : super::PMIB_TCP6ROW, estatstype : super::TCP_ESTATS_TYPE, rw : super::PUCHAR, rwversion : u32, rwsize : u32, ros : super::PUCHAR, rosversion : u32, rossize : u32, rod : super::PUCHAR, rodversion : u32, rodsize : u32) -> u32);
    unsafe { GetPerTcp6ConnectionEStats(row, estatstype, rw.unwrap_or(core::mem::zeroed()) as _, rwversion, rwsize, ros.unwrap_or(core::mem::zeroed()) as _, rosversion, rossize, rod.unwrap_or(core::mem::zeroed()) as _, rodversion, rodsize) }
}
#[cfg(all(feature = "minwindef", feature = "tcpestats", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetPerTcpConnectionEStats(row: super::PMIB_TCPROW, estatstype: super::TCP_ESTATS_TYPE, rw: Option<super::PUCHAR>, rwversion: u32, rwsize: u32, ros: Option<super::PUCHAR>, rosversion: u32, rossize: u32, rod: Option<super::PUCHAR>, rodversion: u32, rodsize: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetPerTcpConnectionEStats(row : super::PMIB_TCPROW, estatstype : super::TCP_ESTATS_TYPE, rw : super::PUCHAR, rwversion : u32, rwsize : u32, ros : super::PUCHAR, rosversion : u32, rossize : u32, rod : super::PUCHAR, rodversion : u32, rodsize : u32) -> u32);
    unsafe { GetPerTcpConnectionEStats(row, estatstype, rw.unwrap_or(core::mem::zeroed()) as _, rwversion, rwsize, ros.unwrap_or(core::mem::zeroed()) as _, rosversion, rossize, rod.unwrap_or(core::mem::zeroed()) as _, rodversion, rodsize) }
}
#[cfg(all(feature = "minwindef", feature = "ntddndis"))]
#[inline]
pub unsafe fn GetRTTAndHopCount(destipaddress: super::IPAddr, hopcount: super::PULONG, maxhops: u32, rtt: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("iphlpapi.dll" "system" fn GetRTTAndHopCount(destipaddress : super::IPAddr, hopcount : super::PULONG, maxhops : u32, rtt : super::PULONG) -> windows_core::BOOL);
    unsafe { GetRTTAndHopCount(destipaddress, hopcount as _, maxhops, rtt as _) }
}
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetTcp6Table(tcptable: super::PMIB_TCP6TABLE, sizepointer: super::PULONG, order: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcp6Table(tcptable : super::PMIB_TCP6TABLE, sizepointer : super::PULONG, order : windows_core::BOOL) -> u32);
    unsafe { GetTcp6Table(tcptable as _, sizepointer as _, order.into()) }
}
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetTcp6Table2(tcptable: super::PMIB_TCP6TABLE2, sizepointer: super::PULONG, order: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcp6Table2(tcptable : super::PMIB_TCP6TABLE2, sizepointer : super::PULONG, order : windows_core::BOOL) -> u32);
    unsafe { GetTcp6Table2(tcptable as _, sizepointer as _, order.into()) }
}
#[cfg(feature = "tcpmib")]
#[inline]
pub unsafe fn GetTcpStatistics(statistics: super::PMIB_TCPSTATS) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcpStatistics(statistics : super::PMIB_TCPSTATS) -> u32);
    unsafe { GetTcpStatistics(statistics as _) }
}
#[cfg(feature = "tcpmib")]
#[inline]
pub unsafe fn GetTcpStatisticsEx(statistics: super::PMIB_TCPSTATS, family: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcpStatisticsEx(statistics : super::PMIB_TCPSTATS, family : u32) -> u32);
    unsafe { GetTcpStatisticsEx(statistics as _, family) }
}
#[cfg(feature = "tcpmib")]
#[inline]
pub unsafe fn GetTcpStatisticsEx2(statistics: super::PMIB_TCPSTATS2, family: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcpStatisticsEx2(statistics : super::PMIB_TCPSTATS2, family : u32) -> u32);
    unsafe { GetTcpStatisticsEx2(statistics as _, family) }
}
#[cfg(all(feature = "minwindef", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetTcpTable(tcptable: Option<super::PMIB_TCPTABLE>, sizepointer: super::PULONG, order: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcpTable(tcptable : super::PMIB_TCPTABLE, sizepointer : super::PULONG, order : windows_core::BOOL) -> u32);
    unsafe { GetTcpTable(tcptable.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[cfg(all(feature = "minwindef", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetTcpTable2(tcptable: Option<super::PMIB_TCPTABLE2>, sizepointer: super::PULONG, order: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcpTable2(tcptable : super::PMIB_TCPTABLE2, sizepointer : super::PULONG, order : windows_core::BOOL) -> u32);
    unsafe { GetTcpTable2(tcptable.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "udpmib"))]
#[inline]
pub unsafe fn GetUdp6Table(udp6table: Option<super::PMIB_UDP6TABLE>, sizepointer: super::PULONG, order: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetUdp6Table(udp6table : super::PMIB_UDP6TABLE, sizepointer : super::PULONG, order : windows_core::BOOL) -> u32);
    unsafe { GetUdp6Table(udp6table.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[cfg(feature = "udpmib")]
#[inline]
pub unsafe fn GetUdpStatistics(stats: super::PMIB_UDPSTATS) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetUdpStatistics(stats : super::PMIB_UDPSTATS) -> u32);
    unsafe { GetUdpStatistics(stats as _) }
}
#[cfg(feature = "udpmib")]
#[inline]
pub unsafe fn GetUdpStatisticsEx(statistics: super::PMIB_UDPSTATS, family: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetUdpStatisticsEx(statistics : super::PMIB_UDPSTATS, family : u32) -> u32);
    unsafe { GetUdpStatisticsEx(statistics as _, family) }
}
#[cfg(feature = "udpmib")]
#[inline]
pub unsafe fn GetUdpStatisticsEx2(statistics: super::PMIB_UDPSTATS2, family: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetUdpStatisticsEx2(statistics : super::PMIB_UDPSTATS2, family : u32) -> u32);
    unsafe { GetUdpStatisticsEx2(statistics as _, family) }
}
#[cfg(all(feature = "minwindef", feature = "udpmib"))]
#[inline]
pub unsafe fn GetUdpTable(udptable: Option<super::PMIB_UDPTABLE>, sizepointer: super::PULONG, order: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetUdpTable(udptable : super::PMIB_UDPTABLE, sizepointer : super::PULONG, order : windows_core::BOOL) -> u32);
    unsafe { GetUdpTable(udptable.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "ntddndis"))]
#[inline]
pub unsafe fn GetUniDirectionalAdapterInfo(pipifinfo: Option<super::PIP_UNIDIRECTIONAL_ADAPTER_ADDRESS>, dwoutbuflen: super::PULONG) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetUniDirectionalAdapterInfo(pipifinfo : super::PIP_UNIDIRECTIONAL_ADAPTER_ADDRESS, dwoutbuflen : super::PULONG) -> u32);
    unsafe { GetUniDirectionalAdapterInfo(pipifinfo.unwrap_or(core::mem::zeroed()) as _, dwoutbuflen as _) }
}
#[cfg(feature = "ipexport")]
#[inline]
pub unsafe fn IpReleaseAddress(adapterinfo: super::PIP_ADAPTER_INDEX_MAP) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn IpReleaseAddress(adapterinfo : super::PIP_ADAPTER_INDEX_MAP) -> u32);
    unsafe { IpReleaseAddress(adapterinfo) }
}
#[cfg(feature = "ipexport")]
#[inline]
pub unsafe fn IpRenewAddress(adapterinfo: super::PIP_ADAPTER_INDEX_MAP) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn IpRenewAddress(adapterinfo : super::PIP_ADAPTER_INDEX_MAP) -> u32);
    unsafe { IpRenewAddress(adapterinfo) }
}
#[cfg(feature = "basetsd")]
#[inline]
pub unsafe fn LookupPersistentTcpPortReservation(startport: u16, numberofports: u16, token: super::PULONG64) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn LookupPersistentTcpPortReservation(startport : u16, numberofports : u16, token : super::PULONG64) -> u32);
    unsafe { LookupPersistentTcpPortReservation(startport, numberofports, token as _) }
}
#[cfg(feature = "basetsd")]
#[inline]
pub unsafe fn LookupPersistentUdpPortReservation(startport: u16, numberofports: u16, token: super::PULONG64) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn LookupPersistentUdpPortReservation(startport : u16, numberofports : u16, token : super::PULONG64) -> u32);
    unsafe { LookupPersistentUdpPortReservation(startport, numberofports, token as _) }
}
#[cfg(all(feature = "iptypes", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn NhpAllocateAndGetInterfaceInfoFromStack(pptable: *mut *mut super::IP_INTERFACE_NAME_INFO, pdwcount: super::PDWORD, border: bool, hheap: super::HANDLE, dwflags: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn NhpAllocateAndGetInterfaceInfoFromStack(pptable : *mut *mut super::IP_INTERFACE_NAME_INFO, pdwcount : super::PDWORD, border : windows_core::BOOL, hheap : super::HANDLE, dwflags : u32) -> u32);
    unsafe { NhpAllocateAndGetInterfaceInfoFromStack(pptable as _, pdwcount as _, border.into(), hheap, dwflags) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn NotifyAddrChange(handle: super::PHANDLE, overlapped: super::LPOVERLAPPED) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn NotifyAddrChange(handle : super::PHANDLE, overlapped : super::LPOVERLAPPED) -> u32);
    unsafe { NotifyAddrChange(handle as _, overlapped) }
}
#[inline]
pub unsafe fn NotifyIfTimestampConfigChange(callercontext: Option<*const core::ffi::c_void>, callback: PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, notificationhandle: *mut HIFTIMESTAMPCHANGE) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn NotifyIfTimestampConfigChange(callercontext : *const core::ffi::c_void, callback : PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, notificationhandle : *mut HIFTIMESTAMPCHANGE) -> u32);
    unsafe { NotifyIfTimestampConfigChange(callercontext.unwrap_or(core::mem::zeroed()) as _, callback, notificationhandle as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn NotifyRouteChange(handle: super::PHANDLE, overlapped: super::LPOVERLAPPED) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn NotifyRouteChange(handle : super::PHANDLE, overlapped : super::LPOVERLAPPED) -> u32);
    unsafe { NotifyRouteChange(handle as _, overlapped) }
}
#[inline]
pub unsafe fn RegisterInterfaceTimestampConfigChange(callback: PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, callercontext: Option<*const core::ffi::c_void>, notificationhandle: *mut HIFTIMESTAMPCHANGE) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn RegisterInterfaceTimestampConfigChange(callback : PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, notificationhandle : *mut HIFTIMESTAMPCHANGE) -> u32);
    unsafe { RegisterInterfaceTimestampConfigChange(callback, callercontext.unwrap_or(core::mem::zeroed()) as _, notificationhandle as _) }
}
#[cfg(all(feature = "minwindef", feature = "ws2"))]
#[inline]
pub unsafe fn ResolveNeighbor(networkaddress: *const super::SOCKADDR, physicaladdress: *mut core::ffi::c_void, physicaladdresslength: super::PULONG) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn ResolveNeighbor(networkaddress : *const super::SOCKADDR, physicaladdress : *mut core::ffi::c_void, physicaladdresslength : super::PULONG) -> u32);
    unsafe { ResolveNeighbor(networkaddress, physicaladdress as _, physicaladdresslength as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RestoreMediaSense(poverlapped: *const super::OVERLAPPED, lpdwenablecount: Option<super::LPDWORD>) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn RestoreMediaSense(poverlapped : *const super::OVERLAPPED, lpdwenablecount : super::LPDWORD) -> u32);
    unsafe { RestoreMediaSense(poverlapped, lpdwenablecount.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "ntddndis"))]
#[inline]
pub unsafe fn SendARP(destip: super::IPAddr, srcip: super::IPAddr, pmacaddr: *mut core::ffi::c_void, phyaddrlen: super::PULONG) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SendARP(destip : super::IPAddr, srcip : super::IPAddr, pmacaddr : *mut core::ffi::c_void, phyaddrlen : super::PULONG) -> u32);
    unsafe { SendARP(destip, srcip, pmacaddr as _, phyaddrlen as _) }
}
#[cfg(all(feature = "ifdef", feature = "ifmib", feature = "ipifcons"))]
#[inline]
pub unsafe fn SetIfEntry(pifrow: super::PMIB_IFROW) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetIfEntry(pifrow : super::PMIB_IFROW) -> u32);
    unsafe { SetIfEntry(pifrow) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
#[inline]
pub unsafe fn SetIpForwardEntry(proute: super::PMIB_IPFORWARDROW) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpForwardEntry(proute : super::PMIB_IPFORWARDROW) -> u32);
    unsafe { SetIpForwardEntry(proute) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib"))]
#[inline]
pub unsafe fn SetIpNetEntry(parpentry: super::PMIB_IPNETROW) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpNetEntry(parpentry : super::PMIB_IPNETROW) -> u32);
    unsafe { SetIpNetEntry(parpentry) }
}
#[cfg(feature = "ipmib")]
#[inline]
pub unsafe fn SetIpStatistics(pipstats: super::PMIB_IPSTATS) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpStatistics(pipstats : super::PMIB_IPSTATS) -> u32);
    unsafe { SetIpStatistics(pipstats) }
}
#[cfg(feature = "ipmib")]
#[inline]
pub unsafe fn SetIpStatisticsEx(statistics: super::PMIB_IPSTATS, family: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpStatisticsEx(statistics : super::PMIB_IPSTATS, family : u32) -> u32);
    unsafe { SetIpStatisticsEx(statistics, family) }
}
#[inline]
pub unsafe fn SetIpTTL(nttl: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpTTL(nttl : u32) -> u32);
    unsafe { SetIpTTL(nttl) }
}
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "tcpestats", feature = "tcpmib"))]
#[inline]
pub unsafe fn SetPerTcp6ConnectionEStats(row: super::PMIB_TCP6ROW, estatstype: super::TCP_ESTATS_TYPE, rw: &[u8], rwversion: u32, offset: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetPerTcp6ConnectionEStats(row : super::PMIB_TCP6ROW, estatstype : super::TCP_ESTATS_TYPE, rw : super::PUCHAR, rwversion : u32, rwsize : u32, offset : u32) -> u32);
    unsafe { SetPerTcp6ConnectionEStats(row, estatstype, core::mem::transmute(rw.as_ptr()), rwversion, rw.len().try_into().unwrap(), offset) }
}
#[cfg(all(feature = "minwindef", feature = "tcpestats", feature = "tcpmib"))]
#[inline]
pub unsafe fn SetPerTcpConnectionEStats(row: super::PMIB_TCPROW, estatstype: super::TCP_ESTATS_TYPE, rw: &[u8], rwversion: u32, offset: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetPerTcpConnectionEStats(row : super::PMIB_TCPROW, estatstype : super::TCP_ESTATS_TYPE, rw : super::PUCHAR, rwversion : u32, rwsize : u32, offset : u32) -> u32);
    unsafe { SetPerTcpConnectionEStats(row, estatstype, core::mem::transmute(rw.as_ptr()), rwversion, rw.len().try_into().unwrap(), offset) }
}
#[cfg(feature = "tcpmib")]
#[inline]
pub unsafe fn SetTcpEntry(ptcprow: super::PMIB_TCPROW) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetTcpEntry(ptcprow : super::PMIB_TCPROW) -> u32);
    unsafe { SetTcpEntry(ptcprow) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn UnenableRouter(poverlapped: *const super::OVERLAPPED, lpdwenablecount: Option<super::LPDWORD>) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn UnenableRouter(poverlapped : *const super::OVERLAPPED, lpdwenablecount : super::LPDWORD) -> u32);
    unsafe { UnenableRouter(poverlapped, lpdwenablecount.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn UnregisterInterfaceTimestampConfigChange(notificationhandle: HIFTIMESTAMPCHANGE) {
    windows_core::link!("iphlpapi.dll" "system" fn UnregisterInterfaceTimestampConfigChange(notificationhandle : HIFTIMESTAMPCHANGE));
    unsafe { UnregisterInterfaceTimestampConfigChange(notificationhandle) }
}
pub type HIFTIMESTAMPCHANGE = *mut HIFTIMESTAMPCHANGE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HIFTIMESTAMPCHANGE__ {
    pub unused: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERFACE_HARDWARE_CROSSTIMESTAMP {
    pub SystemTimestamp1: u64,
    pub HardwareClockTimestamp: u64,
    pub SystemTimestamp2: u64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    pub AllReceive: super::BOOLEAN,
    pub AllTransmit: super::BOOLEAN,
    pub TaggedTransmit: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
