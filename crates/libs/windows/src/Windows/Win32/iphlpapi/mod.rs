#[cfg(feature = "ntddndis")]
#[inline]
pub unsafe fn AddIPAddress(address: super::ntddndis::IPAddr, ipmask: super::ntddndis::IPMask, ifindex: u32, ntecontext: *mut u32, nteinstance: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn AddIPAddress(address : super::ntddndis::IPAddr, ipmask : super::ntddndis::IPMask, ifindex : u32, ntecontext : *mut u32, nteinstance : *mut u32) -> u32);
    unsafe { AddIPAddress(address, ipmask, ifindex, ntecontext as _, nteinstance as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CancelIPChangeNotify(notifyoverlapped: *const super::minwinbase::OVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("iphlpapi.dll" "system" fn CancelIPChangeNotify(notifyoverlapped : *const super::minwinbase::OVERLAPPED) -> windows_core::BOOL);
    unsafe { CancelIPChangeNotify(notifyoverlapped) }
}
#[inline]
pub unsafe fn CancelIfTimestampConfigChange(notificationhandle: HIFTIMESTAMPCHANGE) {
    windows_core::link!("iphlpapi.dll" "system" fn CancelIfTimestampConfigChange(notificationhandle : HIFTIMESTAMPCHANGE));
    unsafe { CancelIfTimestampConfigChange(notificationhandle) }
}
#[cfg(feature = "ifdef")]
#[inline]
pub unsafe fn CaptureInterfaceHardwareCrossTimestamp(interfaceluid: *const super::ifdef::NET_LUID, crosstimestamp: *mut INTERFACE_HARDWARE_CROSSTIMESTAMP) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn CaptureInterfaceHardwareCrossTimestamp(interfaceluid : *const super::ifdef::NET_LUID, crosstimestamp : *mut INTERFACE_HARDWARE_CROSSTIMESTAMP) -> u32);
    unsafe { CaptureInterfaceHardwareCrossTimestamp(interfaceluid, crosstimestamp as _) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
#[inline]
pub unsafe fn CreateIpForwardEntry(proute: *const super::ipmib::MIB_IPFORWARDROW) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn CreateIpForwardEntry(proute : *const super::ipmib::MIB_IPFORWARDROW) -> u32);
    unsafe { CreateIpForwardEntry(proute) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib"))]
#[inline]
pub unsafe fn CreateIpNetEntry(parpentry: *const super::ipmib::MIB_IPNETROW_LH) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn CreateIpNetEntry(parpentry : *const super::ipmib::MIB_IPNETROW_LH) -> u32);
    unsafe { CreateIpNetEntry(parpentry) }
}
#[inline]
pub unsafe fn CreatePersistentTcpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn CreatePersistentTcpPortReservation(startport : u16, numberofports : u16, token : *mut u64) -> u32);
    unsafe { CreatePersistentTcpPortReservation(startport, numberofports, token as _) }
}
#[inline]
pub unsafe fn CreatePersistentUdpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn CreatePersistentUdpPortReservation(startport : u16, numberofports : u16, token : *mut u64) -> u32);
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
pub unsafe fn DeleteIpForwardEntry(proute: *const super::ipmib::MIB_IPFORWARDROW) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn DeleteIpForwardEntry(proute : *const super::ipmib::MIB_IPFORWARDROW) -> u32);
    unsafe { DeleteIpForwardEntry(proute) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib"))]
#[inline]
pub unsafe fn DeleteIpNetEntry(parpentry: *const super::ipmib::MIB_IPNETROW_LH) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn DeleteIpNetEntry(parpentry : *const super::ipmib::MIB_IPNETROW_LH) -> u32);
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
pub unsafe fn DisableMediaSense(phandle: *mut super::winnt::HANDLE, poverlapped: *const super::minwinbase::OVERLAPPED) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn DisableMediaSense(phandle : *mut super::winnt::HANDLE, poverlapped : *const super::minwinbase::OVERLAPPED) -> u32);
    unsafe { DisableMediaSense(phandle as _, poverlapped) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn EnableRouter(phandle: *mut super::winnt::HANDLE, poverlapped: *mut super::minwinbase::OVERLAPPED) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn EnableRouter(phandle : *mut super::winnt::HANDLE, poverlapped : *mut super::minwinbase::OVERLAPPED) -> u32);
    unsafe { EnableRouter(phandle as _, poverlapped as _) }
}
#[inline]
pub unsafe fn FlushIpNetTable(dwifindex: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn FlushIpNetTable(dwifindex : u32) -> u32);
    unsafe { FlushIpNetTable(dwifindex) }
}
#[inline]
pub unsafe fn GetAdapterIndex<P0>(adaptername: P0, ifindex: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("iphlpapi.dll" "system" fn GetAdapterIndex(adaptername : windows_core::PCWSTR, ifindex : *mut u32) -> u32);
    unsafe { GetAdapterIndex(adaptername.param().abi(), ifindex as _) }
}
#[cfg(feature = "ipexport")]
#[inline]
pub unsafe fn GetAdapterOrderMap() -> super::ipexport::PIP_ADAPTER_ORDER_MAP {
    windows_core::link!("iphlpapi.dll" "system" fn GetAdapterOrderMap() -> super::ipexport::PIP_ADAPTER_ORDER_MAP);
    unsafe { GetAdapterOrderMap() }
}
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "iptypes", feature = "nldef", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn GetAdaptersAddresses(family: u32, flags: u32, reserved: Option<*const core::ffi::c_void>, adapteraddresses: Option<*mut super::iptypes::IP_ADAPTER_ADDRESSES_LH>, sizepointer: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetAdaptersAddresses(family : u32, flags : u32, reserved : *const core::ffi::c_void, adapteraddresses : *mut super::iptypes::IP_ADAPTER_ADDRESSES_LH, sizepointer : *mut u32) -> u32);
    unsafe { GetAdaptersAddresses(family, flags, reserved.unwrap_or(core::mem::zeroed()) as _, adapteraddresses.unwrap_or(core::mem::zeroed()) as _, sizepointer as _) }
}
#[cfg(all(feature = "corecrt", feature = "iptypes"))]
#[inline]
pub unsafe fn GetAdaptersInfo(adapterinfo: Option<*mut super::iptypes::IP_ADAPTER_INFO>, sizepointer: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetAdaptersInfo(adapterinfo : *mut super::iptypes::IP_ADAPTER_INFO, sizepointer : *mut u32) -> u32);
    unsafe { GetAdaptersInfo(adapterinfo.unwrap_or(core::mem::zeroed()) as _, sizepointer as _) }
}
#[cfg(feature = "ntddndis")]
#[inline]
pub unsafe fn GetBestInterface(dwdestaddr: super::ntddndis::IPAddr, pdwbestifindex: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetBestInterface(dwdestaddr : super::ntddndis::IPAddr, pdwbestifindex : *mut u32) -> u32);
    unsafe { GetBestInterface(dwdestaddr, pdwbestifindex as _) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
#[inline]
pub unsafe fn GetBestRoute(dwdestaddr: u32, dwsourceaddr: Option<u32>, pbestroute: *mut super::ipmib::MIB_IPFORWARDROW) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetBestRoute(dwdestaddr : u32, dwsourceaddr : u32, pbestroute : *mut super::ipmib::MIB_IPFORWARDROW) -> u32);
    unsafe { GetBestRoute(dwdestaddr, dwsourceaddr.unwrap_or(core::mem::zeroed()) as _, pbestroute as _) }
}
#[cfg(feature = "iprtrmib")]
#[inline]
pub unsafe fn GetExtendedTcpTable(ptcptable: Option<*mut core::ffi::c_void>, pdwsize: *mut u32, border: bool, ulaf: u32, tableclass: super::iprtrmib::TCP_TABLE_CLASS, reserved: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetExtendedTcpTable(ptcptable : *mut core::ffi::c_void, pdwsize : *mut u32, border : windows_core::BOOL, ulaf : u32, tableclass : super::iprtrmib::TCP_TABLE_CLASS, reserved : u32) -> u32);
    unsafe { GetExtendedTcpTable(ptcptable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into(), ulaf, tableclass, reserved) }
}
#[cfg(feature = "iprtrmib")]
#[inline]
pub unsafe fn GetExtendedUdpTable(pudptable: Option<*mut core::ffi::c_void>, pdwsize: *mut u32, border: bool, ulaf: u32, tableclass: super::iprtrmib::UDP_TABLE_CLASS, reserved: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetExtendedUdpTable(pudptable : *mut core::ffi::c_void, pdwsize : *mut u32, border : windows_core::BOOL, ulaf : u32, tableclass : super::iprtrmib::UDP_TABLE_CLASS, reserved : u32) -> u32);
    unsafe { GetExtendedUdpTable(pudptable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into(), ulaf, tableclass, reserved) }
}
#[inline]
pub unsafe fn GetFriendlyIfIndex(ifindex: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetFriendlyIfIndex(ifindex : u32) -> u32);
    unsafe { GetFriendlyIfIndex(ifindex) }
}
#[cfg(feature = "ipmib")]
#[inline]
pub unsafe fn GetIcmpStatistics(statistics: *mut super::ipmib::MIB_ICMP) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIcmpStatistics(statistics : *mut super::ipmib::MIB_ICMP) -> u32);
    unsafe { GetIcmpStatistics(statistics as _) }
}
#[cfg(feature = "ipmib")]
#[inline]
pub unsafe fn GetIcmpStatisticsEx(statistics: *mut super::ipmib::MIB_ICMP_EX_XPSP1, family: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIcmpStatisticsEx(statistics : *mut super::ipmib::MIB_ICMP_EX_XPSP1, family : u32) -> u32);
    unsafe { GetIcmpStatisticsEx(statistics as _, family) }
}
#[cfg(all(feature = "ifdef", feature = "ifmib", feature = "ipifcons"))]
#[inline]
pub unsafe fn GetIfEntry(pifrow: *mut super::ifmib::MIB_IFROW) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIfEntry(pifrow : *mut super::ifmib::MIB_IFROW) -> u32);
    unsafe { GetIfEntry(pifrow as _) }
}
#[cfg(all(feature = "ifdef", feature = "ifmib", feature = "ipifcons"))]
#[inline]
pub unsafe fn GetIfTable(piftable: Option<*mut super::ifmib::MIB_IFTABLE>, pdwsize: *mut u32, border: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIfTable(piftable : *mut super::ifmib::MIB_IFTABLE, pdwsize : *mut u32, border : windows_core::BOOL) -> u32);
    unsafe { GetIfTable(piftable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into()) }
}
#[cfg(feature = "ifdef")]
#[inline]
pub unsafe fn GetInterfaceActiveTimestampCapabilities(interfaceluid: *const super::ifdef::NET_LUID, timestampcapabilites: *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetInterfaceActiveTimestampCapabilities(interfaceluid : *const super::ifdef::NET_LUID, timestampcapabilites : *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
    unsafe { GetInterfaceActiveTimestampCapabilities(interfaceluid, timestampcapabilites as _) }
}
#[cfg(feature = "ifdef")]
#[inline]
pub unsafe fn GetInterfaceCurrentTimestampCapabilities(interfaceluid: *const super::ifdef::NET_LUID, timestampcapabilites: *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetInterfaceCurrentTimestampCapabilities(interfaceluid : *const super::ifdef::NET_LUID, timestampcapabilites : *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
    unsafe { GetInterfaceCurrentTimestampCapabilities(interfaceluid, timestampcapabilites as _) }
}
#[cfg(feature = "ifdef")]
#[inline]
pub unsafe fn GetInterfaceHardwareTimestampCapabilities(interfaceluid: *const super::ifdef::NET_LUID, timestampcapabilites: *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetInterfaceHardwareTimestampCapabilities(interfaceluid : *const super::ifdef::NET_LUID, timestampcapabilites : *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
    unsafe { GetInterfaceHardwareTimestampCapabilities(interfaceluid, timestampcapabilites as _) }
}
#[cfg(feature = "ipexport")]
#[inline]
pub unsafe fn GetInterfaceInfo(piftable: Option<*mut super::ipexport::IP_INTERFACE_INFO>, dwoutbuflen: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetInterfaceInfo(piftable : *mut super::ipexport::IP_INTERFACE_INFO, dwoutbuflen : *mut u32) -> u32);
    unsafe { GetInterfaceInfo(piftable.unwrap_or(core::mem::zeroed()) as _, dwoutbuflen as _) }
}
#[cfg(feature = "ifdef")]
#[inline]
pub unsafe fn GetInterfaceSupportedTimestampCapabilities(interfaceluid: *const super::ifdef::NET_LUID, timestampcapabilites: *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetInterfaceSupportedTimestampCapabilities(interfaceluid : *const super::ifdef::NET_LUID, timestampcapabilites : *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
    unsafe { GetInterfaceSupportedTimestampCapabilities(interfaceluid, timestampcapabilites as _) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib"))]
#[inline]
pub unsafe fn GetIpAddrTable(pipaddrtable: Option<*mut super::ipmib::MIB_IPADDRTABLE>, pdwsize: *mut u32, border: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpAddrTable(pipaddrtable : *mut super::ipmib::MIB_IPADDRTABLE, pdwsize : *mut u32, border : windows_core::BOOL) -> u32);
    unsafe { GetIpAddrTable(pipaddrtable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into()) }
}
#[cfg(feature = "ipexport")]
#[inline]
pub unsafe fn GetIpErrorString(errorcode: super::ipexport::IP_STATUS, buffer: Option<windows_core::PWSTR>, size: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpErrorString(errorcode : super::ipexport::IP_STATUS, buffer : windows_core::PWSTR, size : *mut u32) -> u32);
    unsafe { GetIpErrorString(errorcode, buffer.unwrap_or(core::mem::zeroed()) as _, size as _) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
#[inline]
pub unsafe fn GetIpForwardTable(pipforwardtable: Option<*mut super::ipmib::MIB_IPFORWARDTABLE>, pdwsize: *mut u32, border: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpForwardTable(pipforwardtable : *mut super::ipmib::MIB_IPFORWARDTABLE, pdwsize : *mut u32, border : windows_core::BOOL) -> u32);
    unsafe { GetIpForwardTable(pipforwardtable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into()) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib"))]
#[inline]
pub unsafe fn GetIpNetTable(ipnettable: Option<*mut super::ipmib::MIB_IPNETTABLE>, sizepointer: *mut u32, order: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpNetTable(ipnettable : *mut super::ipmib::MIB_IPNETTABLE, sizepointer : *mut u32, order : windows_core::BOOL) -> u32);
    unsafe { GetIpNetTable(ipnettable.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[cfg(feature = "ipmib")]
#[inline]
pub unsafe fn GetIpStatistics(statistics: *mut super::ipmib::MIB_IPSTATS_LH) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpStatistics(statistics : *mut super::ipmib::MIB_IPSTATS_LH) -> u32);
    unsafe { GetIpStatistics(statistics as _) }
}
#[cfg(feature = "ipmib")]
#[inline]
pub unsafe fn GetIpStatisticsEx(statistics: *mut super::ipmib::MIB_IPSTATS_LH, family: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpStatisticsEx(statistics : *mut super::ipmib::MIB_IPSTATS_LH, family : u32) -> u32);
    unsafe { GetIpStatisticsEx(statistics as _, family) }
}
#[cfg(feature = "iptypes")]
#[inline]
pub unsafe fn GetNetworkParams(pfixedinfo: Option<*mut super::iptypes::FIXED_INFO_W2KSP1>, poutbuflen: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetNetworkParams(pfixedinfo : *mut super::iptypes::FIXED_INFO_W2KSP1, poutbuflen : *mut u32) -> u32);
    unsafe { GetNetworkParams(pfixedinfo.unwrap_or(core::mem::zeroed()) as _, poutbuflen as _) }
}
#[inline]
pub unsafe fn GetNumberOfInterfaces(pdwnumif: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetNumberOfInterfaces(pdwnumif : *mut u32) -> u32);
    unsafe { GetNumberOfInterfaces(pdwnumif as _) }
}
#[cfg(feature = "iprtrmib")]
#[inline]
pub unsafe fn GetOwnerModuleFromPidAndInfo(ulpid: u32, pinfo: *const u64, class: super::iprtrmib::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut core::ffi::c_void, pdwsize: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "C" fn GetOwnerModuleFromPidAndInfo(ulpid : u32, pinfo : *const u64, class : super::iprtrmib::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : *mut u32) -> u32);
    unsafe { GetOwnerModuleFromPidAndInfo(ulpid, pinfo, class, pbuffer as _, pdwsize as _) }
}
#[cfg(all(feature = "iprtrmib", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetOwnerModuleFromTcp6Entry(ptcpentry: *const super::tcpmib::MIB_TCP6ROW_OWNER_MODULE, class: super::iprtrmib::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut core::ffi::c_void, pdwsize: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromTcp6Entry(ptcpentry : *const super::tcpmib::MIB_TCP6ROW_OWNER_MODULE, class : super::iprtrmib::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : *mut u32) -> u32);
    unsafe { GetOwnerModuleFromTcp6Entry(ptcpentry, class, pbuffer as _, pdwsize as _) }
}
#[cfg(all(feature = "iprtrmib", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetOwnerModuleFromTcpEntry(ptcpentry: *const super::tcpmib::MIB_TCPROW_OWNER_MODULE, class: super::iprtrmib::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut core::ffi::c_void, pdwsize: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromTcpEntry(ptcpentry : *const super::tcpmib::MIB_TCPROW_OWNER_MODULE, class : super::iprtrmib::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : *mut u32) -> u32);
    unsafe { GetOwnerModuleFromTcpEntry(ptcpentry, class, pbuffer as _, pdwsize as _) }
}
#[cfg(all(feature = "iprtrmib", feature = "udpmib"))]
#[inline]
pub unsafe fn GetOwnerModuleFromUdp6Entry(pudpentry: *const super::udpmib::MIB_UDP6ROW_OWNER_MODULE, class: super::iprtrmib::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut core::ffi::c_void, pdwsize: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromUdp6Entry(pudpentry : *const super::udpmib::MIB_UDP6ROW_OWNER_MODULE, class : super::iprtrmib::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : *mut u32) -> u32);
    unsafe { GetOwnerModuleFromUdp6Entry(pudpentry, class, pbuffer as _, pdwsize as _) }
}
#[cfg(all(feature = "iprtrmib", feature = "udpmib"))]
#[inline]
pub unsafe fn GetOwnerModuleFromUdpEntry(pudpentry: *const super::udpmib::MIB_UDPROW_OWNER_MODULE, class: super::iprtrmib::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut core::ffi::c_void, pdwsize: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromUdpEntry(pudpentry : *const super::udpmib::MIB_UDPROW_OWNER_MODULE, class : super::iprtrmib::TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : *mut u32) -> u32);
    unsafe { GetOwnerModuleFromUdpEntry(pudpentry, class, pbuffer as _, pdwsize as _) }
}
#[cfg(feature = "iptypes")]
#[inline]
pub unsafe fn GetPerAdapterInfo(ifindex: u32, pperadapterinfo: Option<*mut super::iptypes::IP_PER_ADAPTER_INFO_W2KSP1>, poutbuflen: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetPerAdapterInfo(ifindex : u32, pperadapterinfo : *mut super::iptypes::IP_PER_ADAPTER_INFO_W2KSP1, poutbuflen : *mut u32) -> u32);
    unsafe { GetPerAdapterInfo(ifindex, pperadapterinfo.unwrap_or(core::mem::zeroed()) as _, poutbuflen as _) }
}
#[cfg(all(feature = "in6addr", feature = "tcpestats", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetPerTcp6ConnectionEStats(row: *const super::tcpmib::MIB_TCP6ROW, estatstype: super::tcpestats::TCP_ESTATS_TYPE, rw: Option<&mut [u8]>, rwversion: u32, ros: Option<&mut [u8]>, rosversion: u32, rod: Option<&mut [u8]>, rodversion: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetPerTcp6ConnectionEStats(row : *const super::tcpmib::MIB_TCP6ROW, estatstype : super::tcpestats::TCP_ESTATS_TYPE, rw : *mut u8, rwversion : u32, rwsize : u32, ros : *mut u8, rosversion : u32, rossize : u32, rod : *mut u8, rodversion : u32, rodsize : u32) -> u32);
    unsafe {
        GetPerTcp6ConnectionEStats(
            row,
            estatstype,
            core::mem::transmute(rw.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            rwversion,
            rw.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(ros.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            rosversion,
            ros.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(rod.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            rodversion,
            rod.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        )
    }
}
#[cfg(all(feature = "tcpestats", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetPerTcpConnectionEStats(row: *const super::tcpmib::MIB_TCPROW_LH, estatstype: super::tcpestats::TCP_ESTATS_TYPE, rw: Option<&mut [u8]>, rwversion: u32, ros: Option<&mut [u8]>, rosversion: u32, rod: Option<&mut [u8]>, rodversion: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetPerTcpConnectionEStats(row : *const super::tcpmib::MIB_TCPROW_LH, estatstype : super::tcpestats::TCP_ESTATS_TYPE, rw : *mut u8, rwversion : u32, rwsize : u32, ros : *mut u8, rosversion : u32, rossize : u32, rod : *mut u8, rodversion : u32, rodsize : u32) -> u32);
    unsafe {
        GetPerTcpConnectionEStats(
            row,
            estatstype,
            core::mem::transmute(rw.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            rwversion,
            rw.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(ros.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            rosversion,
            ros.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(rod.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            rodversion,
            rod.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        )
    }
}
#[cfg(feature = "ntddndis")]
#[inline]
pub unsafe fn GetRTTAndHopCount(destipaddress: super::ntddndis::IPAddr, hopcount: *mut u32, maxhops: u32, rtt: *mut u32) -> windows_core::BOOL {
    windows_core::link!("iphlpapi.dll" "system" fn GetRTTAndHopCount(destipaddress : super::ntddndis::IPAddr, hopcount : *mut u32, maxhops : u32, rtt : *mut u32) -> windows_core::BOOL);
    unsafe { GetRTTAndHopCount(destipaddress, hopcount as _, maxhops, rtt as _) }
}
#[cfg(all(feature = "in6addr", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetTcp6Table(tcptable: *mut super::tcpmib::MIB_TCP6TABLE, sizepointer: *mut u32, order: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcp6Table(tcptable : *mut super::tcpmib::MIB_TCP6TABLE, sizepointer : *mut u32, order : windows_core::BOOL) -> u32);
    unsafe { GetTcp6Table(tcptable as _, sizepointer as _, order.into()) }
}
#[cfg(all(feature = "in6addr", feature = "tcpmib"))]
#[inline]
pub unsafe fn GetTcp6Table2(tcptable: *mut super::tcpmib::MIB_TCP6TABLE2, sizepointer: *mut u32, order: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcp6Table2(tcptable : *mut super::tcpmib::MIB_TCP6TABLE2, sizepointer : *mut u32, order : windows_core::BOOL) -> u32);
    unsafe { GetTcp6Table2(tcptable as _, sizepointer as _, order.into()) }
}
#[cfg(feature = "tcpmib")]
#[inline]
pub unsafe fn GetTcpStatistics(statistics: *mut super::tcpmib::MIB_TCPSTATS_LH) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcpStatistics(statistics : *mut super::tcpmib::MIB_TCPSTATS_LH) -> u32);
    unsafe { GetTcpStatistics(statistics as _) }
}
#[cfg(feature = "tcpmib")]
#[inline]
pub unsafe fn GetTcpStatisticsEx(statistics: *mut super::tcpmib::MIB_TCPSTATS_LH, family: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcpStatisticsEx(statistics : *mut super::tcpmib::MIB_TCPSTATS_LH, family : u32) -> u32);
    unsafe { GetTcpStatisticsEx(statistics as _, family) }
}
#[cfg(feature = "tcpmib")]
#[inline]
pub unsafe fn GetTcpStatisticsEx2(statistics: *mut super::tcpmib::MIB_TCPSTATS2, family: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcpStatisticsEx2(statistics : *mut super::tcpmib::MIB_TCPSTATS2, family : u32) -> u32);
    unsafe { GetTcpStatisticsEx2(statistics as _, family) }
}
#[cfg(feature = "tcpmib")]
#[inline]
pub unsafe fn GetTcpTable(tcptable: Option<*mut super::tcpmib::MIB_TCPTABLE>, sizepointer: *mut u32, order: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcpTable(tcptable : *mut super::tcpmib::MIB_TCPTABLE, sizepointer : *mut u32, order : windows_core::BOOL) -> u32);
    unsafe { GetTcpTable(tcptable.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[cfg(feature = "tcpmib")]
#[inline]
pub unsafe fn GetTcpTable2(tcptable: Option<*mut super::tcpmib::MIB_TCPTABLE2>, sizepointer: *mut u32, order: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetTcpTable2(tcptable : *mut super::tcpmib::MIB_TCPTABLE2, sizepointer : *mut u32, order : windows_core::BOOL) -> u32);
    unsafe { GetTcpTable2(tcptable.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[cfg(all(feature = "in6addr", feature = "udpmib"))]
#[inline]
pub unsafe fn GetUdp6Table(udp6table: Option<*mut super::udpmib::MIB_UDP6TABLE>, sizepointer: *mut u32, order: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetUdp6Table(udp6table : *mut super::udpmib::MIB_UDP6TABLE, sizepointer : *mut u32, order : windows_core::BOOL) -> u32);
    unsafe { GetUdp6Table(udp6table.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[cfg(feature = "udpmib")]
#[inline]
pub unsafe fn GetUdpStatistics(stats: *mut super::udpmib::MIB_UDPSTATS) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetUdpStatistics(stats : *mut super::udpmib::MIB_UDPSTATS) -> u32);
    unsafe { GetUdpStatistics(stats as _) }
}
#[cfg(feature = "udpmib")]
#[inline]
pub unsafe fn GetUdpStatisticsEx(statistics: *mut super::udpmib::MIB_UDPSTATS, family: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetUdpStatisticsEx(statistics : *mut super::udpmib::MIB_UDPSTATS, family : u32) -> u32);
    unsafe { GetUdpStatisticsEx(statistics as _, family) }
}
#[cfg(feature = "udpmib")]
#[inline]
pub unsafe fn GetUdpStatisticsEx2(statistics: *mut super::udpmib::MIB_UDPSTATS2, family: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetUdpStatisticsEx2(statistics : *mut super::udpmib::MIB_UDPSTATS2, family : u32) -> u32);
    unsafe { GetUdpStatisticsEx2(statistics as _, family) }
}
#[cfg(feature = "udpmib")]
#[inline]
pub unsafe fn GetUdpTable(udptable: Option<*mut super::udpmib::MIB_UDPTABLE>, sizepointer: *mut u32, order: bool) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetUdpTable(udptable : *mut super::udpmib::MIB_UDPTABLE, sizepointer : *mut u32, order : windows_core::BOOL) -> u32);
    unsafe { GetUdpTable(udptable.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[cfg(all(feature = "ipexport", feature = "ntddndis"))]
#[inline]
pub unsafe fn GetUniDirectionalAdapterInfo(pipifinfo: Option<*mut super::ipexport::IP_UNIDIRECTIONAL_ADAPTER_ADDRESS>, dwoutbuflen: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn GetUniDirectionalAdapterInfo(pipifinfo : *mut super::ipexport::IP_UNIDIRECTIONAL_ADAPTER_ADDRESS, dwoutbuflen : *mut u32) -> u32);
    unsafe { GetUniDirectionalAdapterInfo(pipifinfo.unwrap_or(core::mem::zeroed()) as _, dwoutbuflen as _) }
}
#[cfg(feature = "ipexport")]
#[inline]
pub unsafe fn IpReleaseAddress(adapterinfo: *const super::ipexport::IP_ADAPTER_INDEX_MAP) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn IpReleaseAddress(adapterinfo : *const super::ipexport::IP_ADAPTER_INDEX_MAP) -> u32);
    unsafe { IpReleaseAddress(adapterinfo) }
}
#[cfg(feature = "ipexport")]
#[inline]
pub unsafe fn IpRenewAddress(adapterinfo: *const super::ipexport::IP_ADAPTER_INDEX_MAP) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn IpRenewAddress(adapterinfo : *const super::ipexport::IP_ADAPTER_INDEX_MAP) -> u32);
    unsafe { IpRenewAddress(adapterinfo) }
}
#[inline]
pub unsafe fn LookupPersistentTcpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn LookupPersistentTcpPortReservation(startport : u16, numberofports : u16, token : *mut u64) -> u32);
    unsafe { LookupPersistentTcpPortReservation(startport, numberofports, token as _) }
}
#[inline]
pub unsafe fn LookupPersistentUdpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn LookupPersistentUdpPortReservation(startport : u16, numberofports : u16, token : *mut u64) -> u32);
    unsafe { LookupPersistentUdpPortReservation(startport, numberofports, token as _) }
}
#[cfg(all(feature = "iptypes", feature = "winnt"))]
#[inline]
pub unsafe fn NhpAllocateAndGetInterfaceInfoFromStack(pptable: *mut *mut super::iptypes::IP_INTERFACE_NAME_INFO, pdwcount: *mut u32, border: bool, hheap: super::winnt::HANDLE, dwflags: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn NhpAllocateAndGetInterfaceInfoFromStack(pptable : *mut *mut super::iptypes::IP_INTERFACE_NAME_INFO, pdwcount : *mut u32, border : windows_core::BOOL, hheap : super::winnt::HANDLE, dwflags : u32) -> u32);
    unsafe { NhpAllocateAndGetInterfaceInfoFromStack(pptable as _, pdwcount as _, border.into(), hheap, dwflags) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn NotifyAddrChange(handle: *mut super::winnt::HANDLE, overlapped: *const super::minwinbase::OVERLAPPED) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn NotifyAddrChange(handle : *mut super::winnt::HANDLE, overlapped : *const super::minwinbase::OVERLAPPED) -> u32);
    unsafe { NotifyAddrChange(handle as _, overlapped) }
}
#[inline]
pub unsafe fn NotifyIfTimestampConfigChange(callercontext: Option<*const core::ffi::c_void>, callback: PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, notificationhandle: *mut HIFTIMESTAMPCHANGE) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn NotifyIfTimestampConfigChange(callercontext : *const core::ffi::c_void, callback : PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, notificationhandle : *mut HIFTIMESTAMPCHANGE) -> u32);
    unsafe { NotifyIfTimestampConfigChange(callercontext.unwrap_or(core::mem::zeroed()) as _, callback, notificationhandle as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn NotifyRouteChange(handle: *mut super::winnt::HANDLE, overlapped: *const super::minwinbase::OVERLAPPED) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn NotifyRouteChange(handle : *mut super::winnt::HANDLE, overlapped : *const super::minwinbase::OVERLAPPED) -> u32);
    unsafe { NotifyRouteChange(handle as _, overlapped) }
}
#[inline]
pub unsafe fn RegisterInterfaceTimestampConfigChange(callback: PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, callercontext: Option<*const core::ffi::c_void>, notificationhandle: *mut HIFTIMESTAMPCHANGE) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn RegisterInterfaceTimestampConfigChange(callback : PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, notificationhandle : *mut HIFTIMESTAMPCHANGE) -> u32);
    unsafe { RegisterInterfaceTimestampConfigChange(callback, callercontext.unwrap_or(core::mem::zeroed()) as _, notificationhandle as _) }
}
#[cfg(feature = "ws2")]
#[inline]
pub unsafe fn ResolveNeighbor(networkaddress: *const super::ws2::SOCKADDR, physicaladdress: *mut core::ffi::c_void, physicaladdresslength: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn ResolveNeighbor(networkaddress : *const super::ws2::SOCKADDR, physicaladdress : *mut core::ffi::c_void, physicaladdresslength : *mut u32) -> u32);
    unsafe { ResolveNeighbor(networkaddress, physicaladdress as _, physicaladdresslength as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn RestoreMediaSense(poverlapped: *const super::minwinbase::OVERLAPPED, lpdwenablecount: Option<*mut u32>) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn RestoreMediaSense(poverlapped : *const super::minwinbase::OVERLAPPED, lpdwenablecount : *mut u32) -> u32);
    unsafe { RestoreMediaSense(poverlapped, lpdwenablecount.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "ntddndis")]
#[inline]
pub unsafe fn SendARP(destip: super::ntddndis::IPAddr, srcip: super::ntddndis::IPAddr, pmacaddr: *mut core::ffi::c_void, phyaddrlen: *mut u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SendARP(destip : super::ntddndis::IPAddr, srcip : super::ntddndis::IPAddr, pmacaddr : *mut core::ffi::c_void, phyaddrlen : *mut u32) -> u32);
    unsafe { SendARP(destip, srcip, pmacaddr as _, phyaddrlen as _) }
}
#[cfg(all(feature = "ifdef", feature = "ifmib", feature = "ipifcons"))]
#[inline]
pub unsafe fn SetIfEntry(pifrow: *const super::ifmib::MIB_IFROW) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetIfEntry(pifrow : *const super::ifmib::MIB_IFROW) -> u32);
    unsafe { SetIfEntry(pifrow) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
#[inline]
pub unsafe fn SetIpForwardEntry(proute: *const super::ipmib::MIB_IPFORWARDROW) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpForwardEntry(proute : *const super::ipmib::MIB_IPFORWARDROW) -> u32);
    unsafe { SetIpForwardEntry(proute) }
}
#[cfg(all(feature = "ifdef", feature = "ipmib"))]
#[inline]
pub unsafe fn SetIpNetEntry(parpentry: *const super::ipmib::MIB_IPNETROW_LH) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpNetEntry(parpentry : *const super::ipmib::MIB_IPNETROW_LH) -> u32);
    unsafe { SetIpNetEntry(parpentry) }
}
#[cfg(feature = "ipmib")]
#[inline]
pub unsafe fn SetIpStatistics(pipstats: *const super::ipmib::MIB_IPSTATS_LH) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpStatistics(pipstats : *const super::ipmib::MIB_IPSTATS_LH) -> u32);
    unsafe { SetIpStatistics(pipstats) }
}
#[cfg(feature = "ipmib")]
#[inline]
pub unsafe fn SetIpStatisticsEx(statistics: *const super::ipmib::MIB_IPSTATS_LH, family: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpStatisticsEx(statistics : *const super::ipmib::MIB_IPSTATS_LH, family : u32) -> u32);
    unsafe { SetIpStatisticsEx(statistics, family) }
}
#[inline]
pub unsafe fn SetIpTTL(nttl: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpTTL(nttl : u32) -> u32);
    unsafe { SetIpTTL(nttl) }
}
#[cfg(all(feature = "in6addr", feature = "tcpestats", feature = "tcpmib"))]
#[inline]
pub unsafe fn SetPerTcp6ConnectionEStats(row: *const super::tcpmib::MIB_TCP6ROW, estatstype: super::tcpestats::TCP_ESTATS_TYPE, rw: &[u8], rwversion: u32, offset: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetPerTcp6ConnectionEStats(row : *const super::tcpmib::MIB_TCP6ROW, estatstype : super::tcpestats::TCP_ESTATS_TYPE, rw : *const u8, rwversion : u32, rwsize : u32, offset : u32) -> u32);
    unsafe { SetPerTcp6ConnectionEStats(row, estatstype, core::mem::transmute(rw.as_ptr()), rwversion, rw.len().try_into().unwrap(), offset) }
}
#[cfg(all(feature = "tcpestats", feature = "tcpmib"))]
#[inline]
pub unsafe fn SetPerTcpConnectionEStats(row: *const super::tcpmib::MIB_TCPROW_LH, estatstype: super::tcpestats::TCP_ESTATS_TYPE, rw: &[u8], rwversion: u32, offset: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetPerTcpConnectionEStats(row : *const super::tcpmib::MIB_TCPROW_LH, estatstype : super::tcpestats::TCP_ESTATS_TYPE, rw : *const u8, rwversion : u32, rwsize : u32, offset : u32) -> u32);
    unsafe { SetPerTcpConnectionEStats(row, estatstype, core::mem::transmute(rw.as_ptr()), rwversion, rw.len().try_into().unwrap(), offset) }
}
#[cfg(feature = "tcpmib")]
#[inline]
pub unsafe fn SetTcpEntry(ptcprow: *const super::tcpmib::MIB_TCPROW_LH) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn SetTcpEntry(ptcprow : *const super::tcpmib::MIB_TCPROW_LH) -> u32);
    unsafe { SetTcpEntry(ptcprow) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn UnenableRouter(poverlapped: *const super::minwinbase::OVERLAPPED, lpdwenablecount: Option<*mut u32>) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn UnenableRouter(poverlapped : *const super::minwinbase::OVERLAPPED, lpdwenablecount : *mut u32) -> u32);
    unsafe { UnenableRouter(poverlapped, lpdwenablecount.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn UnregisterInterfaceTimestampConfigChange(notificationhandle: HIFTIMESTAMPCHANGE) {
    windows_core::link!("iphlpapi.dll" "system" fn UnregisterInterfaceTimestampConfigChange(notificationhandle : HIFTIMESTAMPCHANGE));
    unsafe { UnregisterInterfaceTimestampConfigChange(notificationhandle) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HIFTIMESTAMPCHANGE(pub *mut core::ffi::c_void);
impl HIFTIMESTAMPCHANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HIFTIMESTAMPCHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERFACE_HARDWARE_CROSSTIMESTAMP {
    pub SystemTimestamp1: u64,
    pub HardwareClockTimestamp: u64,
    pub SystemTimestamp2: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    pub AllReceive: bool,
    pub AllTransmit: bool,
    pub TaggedTransmit: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PINTERFACE_HARDWARE_CROSSTIMESTAMP(pub *mut INTERFACE_HARDWARE_CROSSTIMESTAMP);
impl PINTERFACE_HARDWARE_CROSSTIMESTAMP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PINTERFACE_HARDWARE_CROSSTIMESTAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PINTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES(pub *mut INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES);
impl PINTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PINTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PINTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES(pub *mut INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES);
impl PINTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PINTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PINTERFACE_TIMESTAMP_CAPABILITIES(pub *mut INTERFACE_TIMESTAMP_CAPABILITIES);
impl PINTERFACE_TIMESTAMP_CAPABILITIES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PINTERFACE_TIMESTAMP_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void)>;
