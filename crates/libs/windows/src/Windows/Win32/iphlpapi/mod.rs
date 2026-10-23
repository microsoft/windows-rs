#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn AddIPAddress(address: super::IPAddr, ipmask: super::IPMask, ifindex: super::DWORD, ntecontext: super::PULONG, nteinstance: super::PULONG) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn AddIPAddress(address : super::IPAddr, ipmask : super::IPMask, ifindex : super::DWORD, ntecontext : super::PULONG, nteinstance : super::PULONG) -> super::DWORD);
    unsafe { AddIPAddress(address, ipmask, ifindex, ntecontext as _, nteinstance as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
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
#[cfg(all(feature = "ifdef", feature = "shared"))]
#[inline]
pub unsafe fn CaptureInterfaceHardwareCrossTimestamp(interfaceluid: *const super::NET_LUID, crosstimestamp: PINTERFACE_HARDWARE_CROSSTIMESTAMP) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn CaptureInterfaceHardwareCrossTimestamp(interfaceluid : *const super::NET_LUID, crosstimestamp : PINTERFACE_HARDWARE_CROSSTIMESTAMP) -> super::DWORD);
    unsafe { CaptureInterfaceHardwareCrossTimestamp(interfaceluid, crosstimestamp as _) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef", feature = "shared"))]
#[inline]
pub unsafe fn CreateIpForwardEntry(proute: super::PMIB_IPFORWARDROW) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn CreateIpForwardEntry(proute : super::PMIB_IPFORWARDROW) -> super::DWORD);
    unsafe { CreateIpForwardEntry(proute) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "shared"))]
#[inline]
pub unsafe fn CreateIpNetEntry(parpentry: super::PMIB_IPNETROW) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn CreateIpNetEntry(parpentry : super::PMIB_IPNETROW) -> super::DWORD);
    unsafe { CreateIpNetEntry(parpentry) }
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[inline]
pub unsafe fn CreatePersistentTcpPortReservation(startport: super::USHORT, numberofports: super::USHORT, token: super::PULONG64) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn CreatePersistentTcpPortReservation(startport : super::USHORT, numberofports : super::USHORT, token : super::PULONG64) -> super::ULONG);
    unsafe { CreatePersistentTcpPortReservation(startport, numberofports, token as _) }
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[inline]
pub unsafe fn CreatePersistentUdpPortReservation(startport: super::USHORT, numberofports: super::USHORT, token: super::PULONG64) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn CreatePersistentUdpPortReservation(startport : super::USHORT, numberofports : super::USHORT, token : super::PULONG64) -> super::ULONG);
    unsafe { CreatePersistentUdpPortReservation(startport, numberofports, token as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CreateProxyArpEntry(dwaddress: super::DWORD, dwmask: super::DWORD, dwifindex: super::DWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn CreateProxyArpEntry(dwaddress : super::DWORD, dwmask : super::DWORD, dwifindex : super::DWORD) -> super::DWORD);
    unsafe { CreateProxyArpEntry(dwaddress, dwmask, dwifindex) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DeleteIPAddress(ntecontext: super::ULONG) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn DeleteIPAddress(ntecontext : super::ULONG) -> super::DWORD);
    unsafe { DeleteIPAddress(ntecontext) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef", feature = "shared"))]
#[inline]
pub unsafe fn DeleteIpForwardEntry(proute: super::PMIB_IPFORWARDROW) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn DeleteIpForwardEntry(proute : super::PMIB_IPFORWARDROW) -> super::DWORD);
    unsafe { DeleteIpForwardEntry(proute) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "shared"))]
#[inline]
pub unsafe fn DeleteIpNetEntry(parpentry: super::PMIB_IPNETROW) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn DeleteIpNetEntry(parpentry : super::PMIB_IPNETROW) -> super::DWORD);
    unsafe { DeleteIpNetEntry(parpentry) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DeletePersistentTcpPortReservation(startport: super::USHORT, numberofports: super::USHORT) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn DeletePersistentTcpPortReservation(startport : super::USHORT, numberofports : super::USHORT) -> super::ULONG);
    unsafe { DeletePersistentTcpPortReservation(startport, numberofports) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DeletePersistentUdpPortReservation(startport: super::USHORT, numberofports: super::USHORT) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn DeletePersistentUdpPortReservation(startport : super::USHORT, numberofports : super::USHORT) -> super::ULONG);
    unsafe { DeletePersistentUdpPortReservation(startport, numberofports) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DeleteProxyArpEntry(dwaddress: super::DWORD, dwmask: super::DWORD, dwifindex: super::DWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn DeleteProxyArpEntry(dwaddress : super::DWORD, dwmask : super::DWORD, dwifindex : super::DWORD) -> super::DWORD);
    unsafe { DeleteProxyArpEntry(dwaddress, dwmask, dwifindex) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn DisableMediaSense(phandle: *mut super::HANDLE, poverlapped: *const super::OVERLAPPED) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn DisableMediaSense(phandle : *mut super::HANDLE, poverlapped : *const super::OVERLAPPED) -> super::DWORD);
    unsafe { DisableMediaSense(phandle as _, poverlapped) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EnableRouter(phandle: *mut super::HANDLE, poverlapped: *mut super::OVERLAPPED) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn EnableRouter(phandle : *mut super::HANDLE, poverlapped : *mut super::OVERLAPPED) -> super::DWORD);
    unsafe { EnableRouter(phandle as _, poverlapped as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn FlushIpNetTable(dwifindex: super::DWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn FlushIpNetTable(dwifindex : super::DWORD) -> super::DWORD);
    unsafe { FlushIpNetTable(dwifindex) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetAdapterIndex<P0>(adaptername: P0, ifindex: super::PULONG) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("iphlpapi.dll" "system" fn GetAdapterIndex(adaptername : windows_core::PCWSTR, ifindex : super::PULONG) -> super::DWORD);
    unsafe { GetAdapterIndex(adaptername.param().abi(), ifindex as _) }
}
#[cfg(all(feature = "ipexport", feature = "shared"))]
#[inline]
pub unsafe fn GetAdapterOrderMap() -> super::PIP_ADAPTER_ORDER_MAP {
    windows_core::link!("iphlpapi.dll" "system" fn GetAdapterOrderMap() -> super::PIP_ADAPTER_ORDER_MAP);
    unsafe { GetAdapterOrderMap() }
}
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "iptypes", feature = "minwindef", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn GetAdaptersAddresses(family: super::ULONG, flags: super::ULONG, reserved: super::PVOID, adapteraddresses: Option<super::PIP_ADAPTER_ADDRESSES>, sizepointer: super::PULONG) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetAdaptersAddresses(family : super::ULONG, flags : super::ULONG, reserved : super::PVOID, adapteraddresses : super::PIP_ADAPTER_ADDRESSES, sizepointer : super::PULONG) -> super::ULONG);
    unsafe { GetAdaptersAddresses(family, flags, reserved as _, adapteraddresses.unwrap_or(core::mem::zeroed()) as _, sizepointer as _) }
}
#[cfg(all(feature = "corecrt", feature = "iptypes", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetAdaptersInfo(adapterinfo: Option<super::PIP_ADAPTER_INFO>, sizepointer: super::PULONG) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetAdaptersInfo(adapterinfo : super::PIP_ADAPTER_INFO, sizepointer : super::PULONG) -> super::ULONG);
    unsafe { GetAdaptersInfo(adapterinfo.unwrap_or(core::mem::zeroed()) as _, sizepointer as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetBestInterface(dwdestaddr: super::IPAddr, pdwbestifindex: super::PDWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetBestInterface(dwdestaddr : super::IPAddr, pdwbestifindex : super::PDWORD) -> super::DWORD);
    unsafe { GetBestInterface(dwdestaddr, pdwbestifindex as _) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef", feature = "shared"))]
#[inline]
pub unsafe fn GetBestRoute(dwdestaddr: super::DWORD, dwsourceaddr: Option<super::DWORD>, pbestroute: super::PMIB_IPFORWARDROW) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetBestRoute(dwdestaddr : super::DWORD, dwsourceaddr : super::DWORD, pbestroute : super::PMIB_IPFORWARDROW) -> super::DWORD);
    unsafe { GetBestRoute(dwdestaddr, dwsourceaddr.unwrap_or(core::mem::zeroed()) as _, pbestroute as _) }
}
#[cfg(all(feature = "iprtrmib", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetExtendedTcpTable(ptcptable: Option<super::PVOID>, pdwsize: super::PDWORD, border: bool, ulaf: super::ULONG, tableclass: super::TCP_TABLE_CLASS, reserved: super::ULONG) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetExtendedTcpTable(ptcptable : super::PVOID, pdwsize : super::PDWORD, border : windows_core::BOOL, ulaf : super::ULONG, tableclass : super::TCP_TABLE_CLASS, reserved : super::ULONG) -> super::DWORD);
    unsafe { GetExtendedTcpTable(ptcptable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into(), ulaf, tableclass, reserved) }
}
#[cfg(all(feature = "iprtrmib", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetExtendedUdpTable(pudptable: Option<super::PVOID>, pdwsize: super::PDWORD, border: bool, ulaf: super::ULONG, tableclass: super::UDP_TABLE_CLASS, reserved: super::ULONG) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetExtendedUdpTable(pudptable : super::PVOID, pdwsize : super::PDWORD, border : windows_core::BOOL, ulaf : super::ULONG, tableclass : super::UDP_TABLE_CLASS, reserved : super::ULONG) -> super::DWORD);
    unsafe { GetExtendedUdpTable(pudptable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into(), ulaf, tableclass, reserved) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetFriendlyIfIndex(ifindex: super::DWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetFriendlyIfIndex(ifindex : super::DWORD) -> super::DWORD);
    unsafe { GetFriendlyIfIndex(ifindex) }
}
#[cfg(all(feature = "ipmib", feature = "shared"))]
#[inline]
pub unsafe fn GetIcmpStatistics(statistics: super::PMIB_ICMP) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetIcmpStatistics(statistics : super::PMIB_ICMP) -> super::ULONG);
    unsafe { GetIcmpStatistics(statistics as _) }
}
#[cfg(all(feature = "ipmib", feature = "shared"))]
#[inline]
pub unsafe fn GetIcmpStatisticsEx(statistics: super::PMIB_ICMP_EX, family: super::ULONG) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetIcmpStatisticsEx(statistics : super::PMIB_ICMP_EX, family : super::ULONG) -> super::ULONG);
    unsafe { GetIcmpStatisticsEx(statistics as _, family) }
}
#[cfg(all(feature = "ifdef", feature = "ifmib", feature = "ipifcons", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetIfEntry(pifrow: super::PMIB_IFROW) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetIfEntry(pifrow : super::PMIB_IFROW) -> super::DWORD);
    unsafe { GetIfEntry(pifrow as _) }
}
#[cfg(all(feature = "ifdef", feature = "ifmib", feature = "ipifcons", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetIfTable(piftable: Option<super::PMIB_IFTABLE>, pdwsize: super::PULONG, border: bool) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetIfTable(piftable : super::PMIB_IFTABLE, pdwsize : super::PULONG, border : windows_core::BOOL) -> super::DWORD);
    unsafe { GetIfTable(piftable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into()) }
}
#[cfg(all(feature = "ifdef", feature = "shared"))]
#[inline]
pub unsafe fn GetInterfaceActiveTimestampCapabilities(interfaceluid: *const super::NET_LUID, timestampcapabilites: PINTERFACE_TIMESTAMP_CAPABILITIES) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetInterfaceActiveTimestampCapabilities(interfaceluid : *const super::NET_LUID, timestampcapabilites : PINTERFACE_TIMESTAMP_CAPABILITIES) -> super::DWORD);
    unsafe { GetInterfaceActiveTimestampCapabilities(interfaceluid, timestampcapabilites as _) }
}
#[cfg(all(feature = "ifdef", feature = "shared"))]
#[inline]
pub unsafe fn GetInterfaceCurrentTimestampCapabilities(interfaceluid: *const super::NET_LUID, timestampcapabilites: PINTERFACE_TIMESTAMP_CAPABILITIES) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetInterfaceCurrentTimestampCapabilities(interfaceluid : *const super::NET_LUID, timestampcapabilites : PINTERFACE_TIMESTAMP_CAPABILITIES) -> super::DWORD);
    unsafe { GetInterfaceCurrentTimestampCapabilities(interfaceluid, timestampcapabilites as _) }
}
#[cfg(all(feature = "ifdef", feature = "shared"))]
#[inline]
pub unsafe fn GetInterfaceHardwareTimestampCapabilities(interfaceluid: *const super::NET_LUID, timestampcapabilites: PINTERFACE_TIMESTAMP_CAPABILITIES) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetInterfaceHardwareTimestampCapabilities(interfaceluid : *const super::NET_LUID, timestampcapabilites : PINTERFACE_TIMESTAMP_CAPABILITIES) -> super::DWORD);
    unsafe { GetInterfaceHardwareTimestampCapabilities(interfaceluid, timestampcapabilites as _) }
}
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetInterfaceInfo(piftable: Option<super::PIP_INTERFACE_INFO>, dwoutbuflen: super::PULONG) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetInterfaceInfo(piftable : super::PIP_INTERFACE_INFO, dwoutbuflen : super::PULONG) -> super::DWORD);
    unsafe { GetInterfaceInfo(piftable.unwrap_or(core::mem::zeroed()) as _, dwoutbuflen as _) }
}
#[cfg(all(feature = "ifdef", feature = "shared"))]
#[inline]
pub unsafe fn GetInterfaceSupportedTimestampCapabilities(interfaceluid: *const super::NET_LUID, timestampcapabilites: PINTERFACE_TIMESTAMP_CAPABILITIES) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetInterfaceSupportedTimestampCapabilities(interfaceluid : *const super::NET_LUID, timestampcapabilites : PINTERFACE_TIMESTAMP_CAPABILITIES) -> super::DWORD);
    unsafe { GetInterfaceSupportedTimestampCapabilities(interfaceluid, timestampcapabilites as _) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetIpAddrTable(pipaddrtable: Option<super::PMIB_IPADDRTABLE>, pdwsize: super::PULONG, border: bool) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpAddrTable(pipaddrtable : super::PMIB_IPADDRTABLE, pdwsize : super::PULONG, border : windows_core::BOOL) -> super::DWORD);
    unsafe { GetIpAddrTable(pipaddrtable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into()) }
}
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetIpErrorString(errorcode: super::IP_STATUS, buffer: Option<windows_core::PWSTR>, size: super::PDWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpErrorString(errorcode : super::IP_STATUS, buffer : windows_core::PWSTR, size : super::PDWORD) -> super::DWORD);
    unsafe { GetIpErrorString(errorcode, buffer.unwrap_or(core::mem::zeroed()) as _, size as _) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "minwindef", feature = "nldef", feature = "shared"))]
#[inline]
pub unsafe fn GetIpForwardTable(pipforwardtable: Option<super::PMIB_IPFORWARDTABLE>, pdwsize: super::PULONG, border: bool) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpForwardTable(pipforwardtable : super::PMIB_IPFORWARDTABLE, pdwsize : super::PULONG, border : windows_core::BOOL) -> super::DWORD);
    unsafe { GetIpForwardTable(pipforwardtable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into()) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetIpNetTable(ipnettable: Option<super::PMIB_IPNETTABLE>, sizepointer: super::PULONG, order: bool) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpNetTable(ipnettable : super::PMIB_IPNETTABLE, sizepointer : super::PULONG, order : windows_core::BOOL) -> super::ULONG);
    unsafe { GetIpNetTable(ipnettable.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[cfg(all(feature = "ipmib", feature = "shared"))]
#[inline]
pub unsafe fn GetIpStatistics(statistics: super::PMIB_IPSTATS) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpStatistics(statistics : super::PMIB_IPSTATS) -> super::ULONG);
    unsafe { GetIpStatistics(statistics as _) }
}
#[cfg(all(feature = "ipmib", feature = "shared"))]
#[inline]
pub unsafe fn GetIpStatisticsEx(statistics: super::PMIB_IPSTATS, family: super::ULONG) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpStatisticsEx(statistics : super::PMIB_IPSTATS, family : super::ULONG) -> super::ULONG);
    unsafe { GetIpStatisticsEx(statistics as _, family) }
}
#[cfg(all(feature = "iptypes", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetNetworkParams(pfixedinfo: Option<super::PFIXED_INFO>, poutbuflen: super::PULONG) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetNetworkParams(pfixedinfo : super::PFIXED_INFO, poutbuflen : super::PULONG) -> super::DWORD);
    unsafe { GetNetworkParams(pfixedinfo.unwrap_or(core::mem::zeroed()) as _, poutbuflen as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetNumberOfInterfaces(pdwnumif: super::PDWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetNumberOfInterfaces(pdwnumif : super::PDWORD) -> super::DWORD);
    unsafe { GetNumberOfInterfaces(pdwnumif as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "iprtrmib", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetOwnerModuleFromPidAndInfo(ulpid: super::ULONG, pinfo: *const super::ULONGLONG, class: super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: super::PVOID, pdwsize: super::PDWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "C" fn GetOwnerModuleFromPidAndInfo(ulpid : super::ULONG, pinfo : *const super::ULONGLONG, class : super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : super::PVOID, pdwsize : super::PDWORD) -> super::DWORD);
    unsafe { GetOwnerModuleFromPidAndInfo(ulpid, pinfo, class, pbuffer as _, pdwsize as _) }
}
#[cfg(all(feature = "iprtrmib", feature = "minwindef", feature = "shared", feature = "tcpmib", feature = "winnt"))]
#[inline]
pub unsafe fn GetOwnerModuleFromTcp6Entry(ptcpentry: super::PMIB_TCP6ROW_OWNER_MODULE, class: super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: super::PVOID, pdwsize: super::PDWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromTcp6Entry(ptcpentry : super::PMIB_TCP6ROW_OWNER_MODULE, class : super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : super::PVOID, pdwsize : super::PDWORD) -> super::DWORD);
    unsafe { GetOwnerModuleFromTcp6Entry(ptcpentry, class, pbuffer as _, pdwsize as _) }
}
#[cfg(all(feature = "iprtrmib", feature = "minwindef", feature = "shared", feature = "tcpmib", feature = "winnt"))]
#[inline]
pub unsafe fn GetOwnerModuleFromTcpEntry(ptcpentry: super::PMIB_TCPROW_OWNER_MODULE, class: super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: super::PVOID, pdwsize: super::PDWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromTcpEntry(ptcpentry : super::PMIB_TCPROW_OWNER_MODULE, class : super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : super::PVOID, pdwsize : super::PDWORD) -> super::DWORD);
    unsafe { GetOwnerModuleFromTcpEntry(ptcpentry, class, pbuffer as _, pdwsize as _) }
}
#[cfg(all(feature = "iprtrmib", feature = "minwindef", feature = "shared", feature = "udpmib", feature = "winnt"))]
#[inline]
pub unsafe fn GetOwnerModuleFromUdp6Entry(pudpentry: super::PMIB_UDP6ROW_OWNER_MODULE, class: super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: super::PVOID, pdwsize: super::PDWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromUdp6Entry(pudpentry : super::PMIB_UDP6ROW_OWNER_MODULE, class : super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : super::PVOID, pdwsize : super::PDWORD) -> super::DWORD);
    unsafe { GetOwnerModuleFromUdp6Entry(pudpentry, class, pbuffer as _, pdwsize as _) }
}
#[cfg(all(feature = "iprtrmib", feature = "minwindef", feature = "shared", feature = "udpmib", feature = "winnt"))]
#[inline]
pub unsafe fn GetOwnerModuleFromUdpEntry(pudpentry: super::PMIB_UDPROW_OWNER_MODULE, class: super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: super::PVOID, pdwsize: super::PDWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromUdpEntry(pudpentry : super::PMIB_UDPROW_OWNER_MODULE, class : super::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : super::PVOID, pdwsize : super::PDWORD) -> super::DWORD);
    unsafe { GetOwnerModuleFromUdpEntry(pudpentry, class, pbuffer as _, pdwsize as _) }
}
#[cfg(all(feature = "iptypes", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetPerAdapterInfo(ifindex: super::ULONG, pperadapterinfo: Option<super::PIP_PER_ADAPTER_INFO>, poutbuflen: super::PULONG) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetPerAdapterInfo(ifindex : super::ULONG, pperadapterinfo : super::PIP_PER_ADAPTER_INFO, poutbuflen : super::PULONG) -> super::DWORD);
    unsafe { GetPerAdapterInfo(ifindex, pperadapterinfo.unwrap_or(core::mem::zeroed()) as _, poutbuflen as _) }
}
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "shared", feature = "tcpestats", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetPerTcp6ConnectionEStats(row: super::PMIB_TCP6ROW, estatstype: super::TCP_ESTATS_TYPE, rw: Option<super::PUCHAR>, rwversion: super::ULONG, rwsize: super::ULONG, ros: Option<super::PUCHAR>, rosversion: super::ULONG, rossize: super::ULONG, rod: Option<super::PUCHAR>, rodversion: super::ULONG, rodsize: super::ULONG) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetPerTcp6ConnectionEStats(row : super::PMIB_TCP6ROW, estatstype : super::TCP_ESTATS_TYPE, rw : super::PUCHAR, rwversion : super::ULONG, rwsize : super::ULONG, ros : super::PUCHAR, rosversion : super::ULONG, rossize : super::ULONG, rod : super::PUCHAR, rodversion : super::ULONG, rodsize : super::ULONG) -> super::ULONG);
    unsafe { GetPerTcp6ConnectionEStats(row, estatstype, rw.unwrap_or(core::mem::zeroed()) as _, rwversion, rwsize, ros.unwrap_or(core::mem::zeroed()) as _, rosversion, rossize, rod.unwrap_or(core::mem::zeroed()) as _, rodversion, rodsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "tcpestats", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetPerTcpConnectionEStats(row: super::PMIB_TCPROW, estatstype: super::TCP_ESTATS_TYPE, rw: Option<super::PUCHAR>, rwversion: super::ULONG, rwsize: super::ULONG, ros: Option<super::PUCHAR>, rosversion: super::ULONG, rossize: super::ULONG, rod: Option<super::PUCHAR>, rodversion: super::ULONG, rodsize: super::ULONG) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetPerTcpConnectionEStats(row : super::PMIB_TCPROW, estatstype : super::TCP_ESTATS_TYPE, rw : super::PUCHAR, rwversion : super::ULONG, rwsize : super::ULONG, ros : super::PUCHAR, rosversion : super::ULONG, rossize : super::ULONG, rod : super::PUCHAR, rodversion : super::ULONG, rodsize : super::ULONG) -> super::ULONG);
    unsafe { GetPerTcpConnectionEStats(row, estatstype, rw.unwrap_or(core::mem::zeroed()) as _, rwversion, rwsize, ros.unwrap_or(core::mem::zeroed()) as _, rosversion, rossize, rod.unwrap_or(core::mem::zeroed()) as _, rodversion, rodsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetRTTAndHopCount(destipaddress: super::IPAddr, hopcount: super::PULONG, maxhops: super::ULONG, rtt: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("iphlpapi.dll" "system" fn GetRTTAndHopCount(destipaddress : super::IPAddr, hopcount : super::PULONG, maxhops : super::ULONG, rtt : super::PULONG) -> windows_core::BOOL);
    unsafe { GetRTTAndHopCount(destipaddress, hopcount as _, maxhops, rtt as _) }
}
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "shared", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetTcp6Table(tcptable: super::PMIB_TCP6TABLE, sizepointer: super::PULONG, order: bool) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcp6Table(tcptable : super::PMIB_TCP6TABLE, sizepointer : super::PULONG, order : windows_core::BOOL) -> super::ULONG);
    unsafe { GetTcp6Table(tcptable as _, sizepointer as _, order.into()) }
}
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "shared", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetTcp6Table2(tcptable: super::PMIB_TCP6TABLE2, sizepointer: super::PULONG, order: bool) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcp6Table2(tcptable : super::PMIB_TCP6TABLE2, sizepointer : super::PULONG, order : windows_core::BOOL) -> super::ULONG);
    unsafe { GetTcp6Table2(tcptable as _, sizepointer as _, order.into()) }
}
#[cfg(all(feature = "shared", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetTcpStatistics(statistics: super::PMIB_TCPSTATS) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcpStatistics(statistics : super::PMIB_TCPSTATS) -> super::ULONG);
    unsafe { GetTcpStatistics(statistics as _) }
}
#[cfg(all(feature = "shared", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetTcpStatisticsEx(statistics: super::PMIB_TCPSTATS, family: super::ULONG) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcpStatisticsEx(statistics : super::PMIB_TCPSTATS, family : super::ULONG) -> super::ULONG);
    unsafe { GetTcpStatisticsEx(statistics as _, family) }
}
#[cfg(all(feature = "shared", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetTcpStatisticsEx2(statistics: super::PMIB_TCPSTATS2, family: super::ULONG) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcpStatisticsEx2(statistics : super::PMIB_TCPSTATS2, family : super::ULONG) -> super::ULONG);
    unsafe { GetTcpStatisticsEx2(statistics as _, family) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetTcpTable(tcptable: Option<super::PMIB_TCPTABLE>, sizepointer: super::PULONG, order: bool) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcpTable(tcptable : super::PMIB_TCPTABLE, sizepointer : super::PULONG, order : windows_core::BOOL) -> super::ULONG);
    unsafe { GetTcpTable(tcptable.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetTcpTable2(tcptable: Option<super::PMIB_TCPTABLE2>, sizepointer: super::PULONG, order: bool) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcpTable2(tcptable : super::PMIB_TCPTABLE2, sizepointer : super::PULONG, order : windows_core::BOOL) -> super::ULONG);
    unsafe { GetTcpTable2(tcptable.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "shared", feature = "udpmib"))]
#[inline]
pub unsafe fn GetUdp6Table(udp6table: Option<super::PMIB_UDP6TABLE>, sizepointer: super::PULONG, order: bool) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetUdp6Table(udp6table : super::PMIB_UDP6TABLE, sizepointer : super::PULONG, order : windows_core::BOOL) -> super::ULONG);
    unsafe { GetUdp6Table(udp6table.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[cfg(all(feature = "shared", feature = "udpmib"))]
#[inline]
pub unsafe fn GetUdpStatistics(stats: super::PMIB_UDPSTATS) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetUdpStatistics(stats : super::PMIB_UDPSTATS) -> super::ULONG);
    unsafe { GetUdpStatistics(stats as _) }
}
#[cfg(all(feature = "shared", feature = "udpmib"))]
#[inline]
pub unsafe fn GetUdpStatisticsEx(statistics: super::PMIB_UDPSTATS, family: super::ULONG) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetUdpStatisticsEx(statistics : super::PMIB_UDPSTATS, family : super::ULONG) -> super::ULONG);
    unsafe { GetUdpStatisticsEx(statistics as _, family) }
}
#[cfg(all(feature = "shared", feature = "udpmib"))]
#[inline]
pub unsafe fn GetUdpStatisticsEx2(statistics: super::PMIB_UDPSTATS2, family: super::ULONG) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetUdpStatisticsEx2(statistics : super::PMIB_UDPSTATS2, family : super::ULONG) -> super::ULONG);
    unsafe { GetUdpStatisticsEx2(statistics as _, family) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "udpmib"))]
#[inline]
pub unsafe fn GetUdpTable(udptable: Option<super::PMIB_UDPTABLE>, sizepointer: super::PULONG, order: bool) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn GetUdpTable(udptable : super::PMIB_UDPTABLE, sizepointer : super::PULONG, order : windows_core::BOOL) -> super::ULONG);
    unsafe { GetUdpTable(udptable.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetUniDirectionalAdapterInfo(pipifinfo: Option<super::PIP_UNIDIRECTIONAL_ADAPTER_ADDRESS>, dwoutbuflen: super::PULONG) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn GetUniDirectionalAdapterInfo(pipifinfo : super::PIP_UNIDIRECTIONAL_ADAPTER_ADDRESS, dwoutbuflen : super::PULONG) -> super::DWORD);
    unsafe { GetUniDirectionalAdapterInfo(pipifinfo.unwrap_or(core::mem::zeroed()) as _, dwoutbuflen as _) }
}
#[cfg(all(feature = "ipexport", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IpReleaseAddress(adapterinfo: super::PIP_ADAPTER_INDEX_MAP) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn IpReleaseAddress(adapterinfo : super::PIP_ADAPTER_INDEX_MAP) -> super::DWORD);
    unsafe { IpReleaseAddress(adapterinfo) }
}
#[cfg(all(feature = "ipexport", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IpRenewAddress(adapterinfo: super::PIP_ADAPTER_INDEX_MAP) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn IpRenewAddress(adapterinfo : super::PIP_ADAPTER_INDEX_MAP) -> super::DWORD);
    unsafe { IpRenewAddress(adapterinfo) }
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[inline]
pub unsafe fn LookupPersistentTcpPortReservation(startport: super::USHORT, numberofports: super::USHORT, token: super::PULONG64) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn LookupPersistentTcpPortReservation(startport : super::USHORT, numberofports : super::USHORT, token : super::PULONG64) -> super::ULONG);
    unsafe { LookupPersistentTcpPortReservation(startport, numberofports, token as _) }
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[inline]
pub unsafe fn LookupPersistentUdpPortReservation(startport: super::USHORT, numberofports: super::USHORT, token: super::PULONG64) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn LookupPersistentUdpPortReservation(startport : super::USHORT, numberofports : super::USHORT, token : super::PULONG64) -> super::ULONG);
    unsafe { LookupPersistentUdpPortReservation(startport, numberofports, token as _) }
}
#[cfg(all(feature = "iptypes", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NhpAllocateAndGetInterfaceInfoFromStack(pptable: *mut *mut super::IP_INTERFACE_NAME_INFO, pdwcount: super::PDWORD, border: bool, hheap: super::HANDLE, dwflags: super::DWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn NhpAllocateAndGetInterfaceInfoFromStack(pptable : *mut *mut super::IP_INTERFACE_NAME_INFO, pdwcount : super::PDWORD, border : windows_core::BOOL, hheap : super::HANDLE, dwflags : super::DWORD) -> super::DWORD);
    unsafe { NhpAllocateAndGetInterfaceInfoFromStack(pptable as _, pdwcount as _, border.into(), hheap, dwflags) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NotifyAddrChange(handle: super::PHANDLE, overlapped: super::LPOVERLAPPED) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn NotifyAddrChange(handle : super::PHANDLE, overlapped : super::LPOVERLAPPED) -> super::DWORD);
    unsafe { NotifyAddrChange(handle as _, overlapped) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NotifyIfTimestampConfigChange(callercontext: Option<super::PVOID>, callback: PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, notificationhandle: *mut HIFTIMESTAMPCHANGE) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn NotifyIfTimestampConfigChange(callercontext : super::PVOID, callback : PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, notificationhandle : *mut HIFTIMESTAMPCHANGE) -> super::DWORD);
    unsafe { NotifyIfTimestampConfigChange(callercontext.unwrap_or(core::mem::zeroed()) as _, callback, notificationhandle as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NotifyRouteChange(handle: super::PHANDLE, overlapped: super::LPOVERLAPPED) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn NotifyRouteChange(handle : super::PHANDLE, overlapped : super::LPOVERLAPPED) -> super::DWORD);
    unsafe { NotifyRouteChange(handle as _, overlapped) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RegisterInterfaceTimestampConfigChange(callback: PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, callercontext: Option<super::PVOID>, notificationhandle: *mut HIFTIMESTAMPCHANGE) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn RegisterInterfaceTimestampConfigChange(callback : PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, callercontext : super::PVOID, notificationhandle : *mut HIFTIMESTAMPCHANGE) -> super::DWORD);
    unsafe { RegisterInterfaceTimestampConfigChange(callback, callercontext.unwrap_or(core::mem::zeroed()) as _, notificationhandle as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn ResolveNeighbor(networkaddress: *const super::SOCKADDR, physicaladdress: super::PVOID, physicaladdresslength: super::PULONG) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn ResolveNeighbor(networkaddress : *const super::SOCKADDR, physicaladdress : super::PVOID, physicaladdresslength : super::PULONG) -> super::ULONG);
    unsafe { ResolveNeighbor(networkaddress, physicaladdress as _, physicaladdresslength as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RestoreMediaSense(poverlapped: *const super::OVERLAPPED, lpdwenablecount: Option<super::LPDWORD>) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn RestoreMediaSense(poverlapped : *const super::OVERLAPPED, lpdwenablecount : super::LPDWORD) -> super::DWORD);
    unsafe { RestoreMediaSense(poverlapped, lpdwenablecount.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SendARP(destip: super::IPAddr, srcip: super::IPAddr, pmacaddr: super::PVOID, phyaddrlen: super::PULONG) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn SendARP(destip : super::IPAddr, srcip : super::IPAddr, pmacaddr : super::PVOID, phyaddrlen : super::PULONG) -> super::DWORD);
    unsafe { SendARP(destip, srcip, pmacaddr as _, phyaddrlen as _) }
}
#[cfg(all(feature = "ifdef", feature = "ifmib", feature = "ipifcons", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetIfEntry(pifrow: super::PMIB_IFROW) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn SetIfEntry(pifrow : super::PMIB_IFROW) -> super::DWORD);
    unsafe { SetIfEntry(pifrow) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef", feature = "shared"))]
#[inline]
pub unsafe fn SetIpForwardEntry(proute: super::PMIB_IPFORWARDROW) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpForwardEntry(proute : super::PMIB_IPFORWARDROW) -> super::DWORD);
    unsafe { SetIpForwardEntry(proute) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "shared"))]
#[inline]
pub unsafe fn SetIpNetEntry(parpentry: super::PMIB_IPNETROW) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpNetEntry(parpentry : super::PMIB_IPNETROW) -> super::DWORD);
    unsafe { SetIpNetEntry(parpentry) }
}
#[cfg(all(feature = "ipmib", feature = "shared"))]
#[inline]
pub unsafe fn SetIpStatistics(pipstats: super::PMIB_IPSTATS) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpStatistics(pipstats : super::PMIB_IPSTATS) -> super::DWORD);
    unsafe { SetIpStatistics(pipstats) }
}
#[cfg(all(feature = "ipmib", feature = "shared"))]
#[inline]
pub unsafe fn SetIpStatisticsEx(statistics: super::PMIB_IPSTATS, family: super::ULONG) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpStatisticsEx(statistics : super::PMIB_IPSTATS, family : super::ULONG) -> super::ULONG);
    unsafe { SetIpStatisticsEx(statistics, family) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetIpTTL(nttl: super::UINT) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpTTL(nttl : super::UINT) -> super::DWORD);
    unsafe { SetIpTTL(nttl) }
}
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "shared", feature = "tcpestats", feature = "tcpmib"))]
#[inline]
pub unsafe fn SetPerTcp6ConnectionEStats(row: super::PMIB_TCP6ROW, estatstype: super::TCP_ESTATS_TYPE, rw: &[u8], rwversion: super::ULONG, offset: super::ULONG) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn SetPerTcp6ConnectionEStats(row : super::PMIB_TCP6ROW, estatstype : super::TCP_ESTATS_TYPE, rw : super::PUCHAR, rwversion : super::ULONG, rwsize : super::ULONG, offset : super::ULONG) -> super::ULONG);
    unsafe { SetPerTcp6ConnectionEStats(row, estatstype, core::mem::transmute(rw.as_ptr()), rwversion, rw.len().try_into().unwrap(), offset) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "tcpestats", feature = "tcpmib"))]
#[inline]
pub unsafe fn SetPerTcpConnectionEStats(row: super::PMIB_TCPROW, estatstype: super::TCP_ESTATS_TYPE, rw: &[u8], rwversion: super::ULONG, offset: super::ULONG) -> super::ULONG {
    windows_core::link!("iphlpapi.dll" "system" fn SetPerTcpConnectionEStats(row : super::PMIB_TCPROW, estatstype : super::TCP_ESTATS_TYPE, rw : super::PUCHAR, rwversion : super::ULONG, rwsize : super::ULONG, offset : super::ULONG) -> super::ULONG);
    unsafe { SetPerTcpConnectionEStats(row, estatstype, core::mem::transmute(rw.as_ptr()), rwversion, rw.len().try_into().unwrap(), offset) }
}
#[cfg(all(feature = "shared", feature = "tcpmib"))]
#[inline]
pub unsafe fn SetTcpEntry(ptcprow: super::PMIB_TCPROW) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn SetTcpEntry(ptcprow : super::PMIB_TCPROW) -> super::DWORD);
    unsafe { SetTcpEntry(ptcprow) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn UnenableRouter(poverlapped: *const super::OVERLAPPED, lpdwenablecount: Option<super::LPDWORD>) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn UnenableRouter(poverlapped : *const super::OVERLAPPED, lpdwenablecount : super::LPDWORD) -> super::DWORD);
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERFACE_HARDWARE_CROSSTIMESTAMP {
    pub SystemTimestamp1: super::ULONG64,
    pub HardwareClockTimestamp: super::ULONG64,
    pub SystemTimestamp2: super::ULONG64,
}
#[repr(C)]
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    pub AllReceive: super::BOOLEAN,
    pub AllTransmit: super::BOOLEAN,
    pub TaggedTransmit: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERFACE_TIMESTAMP_CAPABILITIES {
    pub HardwareClockFrequencyHz: super::ULONG64,
    pub SupportsCrossTimestamp: super::BOOLEAN,
    pub HardwareCapabilities: INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES,
    pub SoftwareCapabilities: INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES,
}
#[cfg(feature = "winnt")]
pub type INTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: super::PVOID)>;
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
#[cfg(feature = "shared")]
pub type PINTERFACE_HARDWARE_CROSSTIMESTAMP = *mut INTERFACE_HARDWARE_CROSSTIMESTAMP;
#[cfg(feature = "shared")]
pub type PINTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES = *mut INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES;
#[cfg(feature = "shared")]
pub type PINTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES = *mut INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES;
#[cfg(feature = "shared")]
pub type PINTERFACE_TIMESTAMP_CAPABILITIES = *mut INTERFACE_TIMESTAMP_CAPABILITIES;
#[cfg(feature = "winnt")]
pub type PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: super::PVOID)>;
