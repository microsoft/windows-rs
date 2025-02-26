#[inline]
pub unsafe fn AddIPAddress(address: u32, ipmask: u32, ifindex: u32, ntecontext: *mut u32, nteinstance: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn AddIPAddress(address : u32, ipmask : u32, ifindex : u32, ntecontext : *mut u32, nteinstance : *mut u32) -> u32);
    unsafe { AddIPAddress(address, ipmask, ifindex, ntecontext as _, nteinstance as _) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn CancelIPChangeNotify(notifyoverlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::BOOL {
    windows_link::link!("iphlpapi.dll" "system" fn CancelIPChangeNotify(notifyoverlapped : *const super::super::System::IO:: OVERLAPPED) -> windows_core::BOOL);
    unsafe { CancelIPChangeNotify(notifyoverlapped) }
}
#[inline]
pub unsafe fn CancelIfTimestampConfigChange(notificationhandle: HIFTIMESTAMPCHANGE) {
    windows_link::link!("iphlpapi.dll" "system" fn CancelIfTimestampConfigChange(notificationhandle : HIFTIMESTAMPCHANGE));
    unsafe { CancelIfTimestampConfigChange(notificationhandle) }
}
#[inline]
pub unsafe fn CancelMibChangeNotify2(notificationhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn CancelMibChangeNotify2(notificationhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { CancelMibChangeNotify2(notificationhandle) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn CaptureInterfaceHardwareCrossTimestamp(interfaceluid: *const super::Ndis::NET_LUID_LH, crosstimestamp: *mut INTERFACE_HARDWARE_CROSSTIMESTAMP) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn CaptureInterfaceHardwareCrossTimestamp(interfaceluid : *const super::Ndis:: NET_LUID_LH, crosstimestamp : *mut INTERFACE_HARDWARE_CROSSTIMESTAMP) -> u32);
    unsafe { CaptureInterfaceHardwareCrossTimestamp(interfaceluid, crosstimestamp as _) }
}
#[inline]
pub unsafe fn ConvertCompartmentGuidToId(compartmentguid: *const windows_core::GUID, compartmentid: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn ConvertCompartmentGuidToId(compartmentguid : *const windows_core::GUID, compartmentid : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { ConvertCompartmentGuidToId(compartmentguid, compartmentid as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn ConvertCompartmentIdToGuid(compartmentid: super::Ndis::NET_IF_COMPARTMENT_ID, compartmentguid: *mut windows_core::GUID) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn ConvertCompartmentIdToGuid(compartmentid : super::Ndis:: NET_IF_COMPARTMENT_ID, compartmentguid : *mut windows_core::GUID) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { ConvertCompartmentIdToGuid(compartmentid, compartmentguid as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn ConvertInterfaceAliasToLuid<P0>(interfacealias: P0, interfaceluid: *mut super::Ndis::NET_LUID_LH) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceAliasToLuid(interfacealias : windows_core::PCWSTR, interfaceluid : *mut super::Ndis:: NET_LUID_LH) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { ConvertInterfaceAliasToLuid(interfacealias.param().abi(), interfaceluid as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn ConvertInterfaceGuidToLuid(interfaceguid: *const windows_core::GUID, interfaceluid: *mut super::Ndis::NET_LUID_LH) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceGuidToLuid(interfaceguid : *const windows_core::GUID, interfaceluid : *mut super::Ndis:: NET_LUID_LH) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { ConvertInterfaceGuidToLuid(interfaceguid, interfaceluid as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn ConvertInterfaceIndexToLuid(interfaceindex: u32, interfaceluid: *mut super::Ndis::NET_LUID_LH) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceIndexToLuid(interfaceindex : u32, interfaceluid : *mut super::Ndis:: NET_LUID_LH) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { ConvertInterfaceIndexToLuid(interfaceindex, interfaceluid as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn ConvertInterfaceLuidToAlias(interfaceluid: *const super::Ndis::NET_LUID_LH, interfacealias: &mut [u16]) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToAlias(interfaceluid : *const super::Ndis:: NET_LUID_LH, interfacealias : windows_core::PWSTR, length : usize) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { ConvertInterfaceLuidToAlias(interfaceluid, core::mem::transmute(interfacealias.as_ptr()), interfacealias.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn ConvertInterfaceLuidToGuid(interfaceluid: *const super::Ndis::NET_LUID_LH, interfaceguid: *mut windows_core::GUID) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToGuid(interfaceluid : *const super::Ndis:: NET_LUID_LH, interfaceguid : *mut windows_core::GUID) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { ConvertInterfaceLuidToGuid(interfaceluid, interfaceguid as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn ConvertInterfaceLuidToIndex(interfaceluid: *const super::Ndis::NET_LUID_LH, interfaceindex: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToIndex(interfaceluid : *const super::Ndis:: NET_LUID_LH, interfaceindex : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { ConvertInterfaceLuidToIndex(interfaceluid, interfaceindex as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn ConvertInterfaceLuidToNameA(interfaceluid: *const super::Ndis::NET_LUID_LH, interfacename: &mut [u8]) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToNameA(interfaceluid : *const super::Ndis:: NET_LUID_LH, interfacename : windows_core::PSTR, length : usize) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { ConvertInterfaceLuidToNameA(interfaceluid, core::mem::transmute(interfacename.as_ptr()), interfacename.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn ConvertInterfaceLuidToNameW(interfaceluid: *const super::Ndis::NET_LUID_LH, interfacename: &mut [u16]) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToNameW(interfaceluid : *const super::Ndis:: NET_LUID_LH, interfacename : windows_core::PWSTR, length : usize) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { ConvertInterfaceLuidToNameW(interfaceluid, core::mem::transmute(interfacename.as_ptr()), interfacename.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn ConvertInterfaceNameToLuidA<P0>(interfacename: P0, interfaceluid: *mut super::Ndis::NET_LUID_LH) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceNameToLuidA(interfacename : windows_core::PCSTR, interfaceluid : *mut super::Ndis:: NET_LUID_LH) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { ConvertInterfaceNameToLuidA(interfacename.param().abi(), interfaceluid as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn ConvertInterfaceNameToLuidW<P0>(interfacename: P0, interfaceluid: *mut super::Ndis::NET_LUID_LH) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceNameToLuidW(interfacename : windows_core::PCWSTR, interfaceluid : *mut super::Ndis:: NET_LUID_LH) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { ConvertInterfaceNameToLuidW(interfacename.param().abi(), interfaceluid as _) }
}
#[inline]
pub unsafe fn ConvertIpv4MaskToLength(mask: u32, masklength: *mut u8) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn ConvertIpv4MaskToLength(mask : u32, masklength : *mut u8) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { ConvertIpv4MaskToLength(mask, masklength as _) }
}
#[inline]
pub unsafe fn ConvertLengthToIpv4Mask(masklength: u32, mask: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn ConvertLengthToIpv4Mask(masklength : u32, mask : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { ConvertLengthToIpv4Mask(masklength, mask as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn CreateAnycastIpAddressEntry(row: *const MIB_ANYCASTIPADDRESS_ROW) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn CreateAnycastIpAddressEntry(row : *const MIB_ANYCASTIPADDRESS_ROW) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { CreateAnycastIpAddressEntry(row) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn CreateIpForwardEntry(proute: *const MIB_IPFORWARDROW) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn CreateIpForwardEntry(proute : *const MIB_IPFORWARDROW) -> u32);
    unsafe { CreateIpForwardEntry(proute) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn CreateIpForwardEntry2(row: *const MIB_IPFORWARD_ROW2) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn CreateIpForwardEntry2(row : *const MIB_IPFORWARD_ROW2) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { CreateIpForwardEntry2(row) }
}
#[inline]
pub unsafe fn CreateIpNetEntry(parpentry: *const MIB_IPNETROW_LH) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn CreateIpNetEntry(parpentry : *const MIB_IPNETROW_LH) -> u32);
    unsafe { CreateIpNetEntry(parpentry) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn CreateIpNetEntry2(row: *const MIB_IPNET_ROW2) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn CreateIpNetEntry2(row : *const MIB_IPNET_ROW2) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { CreateIpNetEntry2(row) }
}
#[inline]
pub unsafe fn CreatePersistentTcpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn CreatePersistentTcpPortReservation(startport : u16, numberofports : u16, token : *mut u64) -> u32);
    unsafe { CreatePersistentTcpPortReservation(startport, numberofports, token as _) }
}
#[inline]
pub unsafe fn CreatePersistentUdpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn CreatePersistentUdpPortReservation(startport : u16, numberofports : u16, token : *mut u64) -> u32);
    unsafe { CreatePersistentUdpPortReservation(startport, numberofports, token as _) }
}
#[inline]
pub unsafe fn CreateProxyArpEntry(dwaddress: u32, dwmask: u32, dwifindex: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn CreateProxyArpEntry(dwaddress : u32, dwmask : u32, dwifindex : u32) -> u32);
    unsafe { CreateProxyArpEntry(dwaddress, dwmask, dwifindex) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn CreateSortedAddressPairs(sourceaddresslist: Option<*const super::super::Networking::WinSock::SOCKADDR_IN6>, sourceaddresscount: u32, destinationaddresslist: *const super::super::Networking::WinSock::SOCKADDR_IN6, destinationaddresscount: u32, addresssortoptions: u32, sortedaddresspairlist: *mut *mut super::super::Networking::WinSock::SOCKADDR_IN6_PAIR, sortedaddresspaircount: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn CreateSortedAddressPairs(sourceaddresslist : *const super::super::Networking::WinSock:: SOCKADDR_IN6, sourceaddresscount : u32, destinationaddresslist : *const super::super::Networking::WinSock:: SOCKADDR_IN6, destinationaddresscount : u32, addresssortoptions : u32, sortedaddresspairlist : *mut *mut super::super::Networking::WinSock:: SOCKADDR_IN6_PAIR, sortedaddresspaircount : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { CreateSortedAddressPairs(sourceaddresslist.unwrap_or(core::mem::zeroed()) as _, sourceaddresscount, destinationaddresslist, destinationaddresscount, addresssortoptions, sortedaddresspairlist as _, sortedaddresspaircount as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn CreateUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn CreateUnicastIpAddressEntry(row : *const MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { CreateUnicastIpAddressEntry(row) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DeleteAnycastIpAddressEntry(row: *const MIB_ANYCASTIPADDRESS_ROW) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn DeleteAnycastIpAddressEntry(row : *const MIB_ANYCASTIPADDRESS_ROW) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { DeleteAnycastIpAddressEntry(row) }
}
#[inline]
pub unsafe fn DeleteIPAddress(ntecontext: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn DeleteIPAddress(ntecontext : u32) -> u32);
    unsafe { DeleteIPAddress(ntecontext) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DeleteIpForwardEntry(proute: *const MIB_IPFORWARDROW) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn DeleteIpForwardEntry(proute : *const MIB_IPFORWARDROW) -> u32);
    unsafe { DeleteIpForwardEntry(proute) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DeleteIpForwardEntry2(row: *const MIB_IPFORWARD_ROW2) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn DeleteIpForwardEntry2(row : *const MIB_IPFORWARD_ROW2) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { DeleteIpForwardEntry2(row) }
}
#[inline]
pub unsafe fn DeleteIpNetEntry(parpentry: *const MIB_IPNETROW_LH) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn DeleteIpNetEntry(parpentry : *const MIB_IPNETROW_LH) -> u32);
    unsafe { DeleteIpNetEntry(parpentry) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DeleteIpNetEntry2(row: *const MIB_IPNET_ROW2) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn DeleteIpNetEntry2(row : *const MIB_IPNET_ROW2) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { DeleteIpNetEntry2(row) }
}
#[inline]
pub unsafe fn DeletePersistentTcpPortReservation(startport: u16, numberofports: u16) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn DeletePersistentTcpPortReservation(startport : u16, numberofports : u16) -> u32);
    unsafe { DeletePersistentTcpPortReservation(startport, numberofports) }
}
#[inline]
pub unsafe fn DeletePersistentUdpPortReservation(startport: u16, numberofports: u16) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn DeletePersistentUdpPortReservation(startport : u16, numberofports : u16) -> u32);
    unsafe { DeletePersistentUdpPortReservation(startport, numberofports) }
}
#[inline]
pub unsafe fn DeleteProxyArpEntry(dwaddress: u32, dwmask: u32, dwifindex: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn DeleteProxyArpEntry(dwaddress : u32, dwmask : u32, dwifindex : u32) -> u32);
    unsafe { DeleteProxyArpEntry(dwaddress, dwmask, dwifindex) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DeleteUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn DeleteUnicastIpAddressEntry(row : *const MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { DeleteUnicastIpAddressEntry(row) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn DisableMediaSense(phandle: *mut super::super::Foundation::HANDLE, poverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn DisableMediaSense(phandle : *mut super::super::Foundation:: HANDLE, poverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    unsafe { DisableMediaSense(phandle as _, poverlapped) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn EnableRouter(phandle: *mut super::super::Foundation::HANDLE, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn EnableRouter(phandle : *mut super::super::Foundation:: HANDLE, poverlapped : *mut super::super::System::IO:: OVERLAPPED) -> u32);
    unsafe { EnableRouter(phandle as _, poverlapped as _) }
}
#[inline]
pub unsafe fn FlushIpNetTable(dwifindex: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn FlushIpNetTable(dwifindex : u32) -> u32);
    unsafe { FlushIpNetTable(dwifindex) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn FlushIpNetTable2(family: super::super::Networking::WinSock::ADDRESS_FAMILY, interfaceindex: u32) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn FlushIpNetTable2(family : super::super::Networking::WinSock:: ADDRESS_FAMILY, interfaceindex : u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { FlushIpNetTable2(family, interfaceindex) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn FlushIpPathTable(family: super::super::Networking::WinSock::ADDRESS_FAMILY) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn FlushIpPathTable(family : super::super::Networking::WinSock:: ADDRESS_FAMILY) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { FlushIpPathTable(family) }
}
#[inline]
pub unsafe fn FreeDnsSettings(settings: *mut DNS_SETTINGS) {
    windows_link::link!("iphlpapi.dll" "system" fn FreeDnsSettings(settings : *mut DNS_SETTINGS));
    unsafe { FreeDnsSettings(settings as _) }
}
#[inline]
pub unsafe fn FreeInterfaceDnsSettings(settings: *mut DNS_INTERFACE_SETTINGS) {
    windows_link::link!("iphlpapi.dll" "system" fn FreeInterfaceDnsSettings(settings : *mut DNS_INTERFACE_SETTINGS));
    unsafe { FreeInterfaceDnsSettings(settings as _) }
}
#[inline]
pub unsafe fn FreeMibTable(memory: *const core::ffi::c_void) {
    windows_link::link!("iphlpapi.dll" "system" fn FreeMibTable(memory : *const core::ffi::c_void));
    unsafe { FreeMibTable(memory) }
}
#[inline]
pub unsafe fn GetAdapterIndex<P0>(adaptername: P0, ifindex: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("iphlpapi.dll" "system" fn GetAdapterIndex(adaptername : windows_core::PCWSTR, ifindex : *mut u32) -> u32);
    unsafe { GetAdapterIndex(adaptername.param().abi(), ifindex as _) }
}
#[inline]
pub unsafe fn GetAdapterOrderMap() -> *mut IP_ADAPTER_ORDER_MAP {
    windows_link::link!("iphlpapi.dll" "system" fn GetAdapterOrderMap() -> *mut IP_ADAPTER_ORDER_MAP);
    unsafe { GetAdapterOrderMap() }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetAdaptersAddresses(family: u32, flags: GET_ADAPTERS_ADDRESSES_FLAGS, reserved: Option<*const core::ffi::c_void>, adapteraddresses: Option<*mut IP_ADAPTER_ADDRESSES_LH>, sizepointer: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetAdaptersAddresses(family : u32, flags : GET_ADAPTERS_ADDRESSES_FLAGS, reserved : *const core::ffi::c_void, adapteraddresses : *mut IP_ADAPTER_ADDRESSES_LH, sizepointer : *mut u32) -> u32);
    unsafe { GetAdaptersAddresses(family, flags, reserved.unwrap_or(core::mem::zeroed()) as _, adapteraddresses.unwrap_or(core::mem::zeroed()) as _, sizepointer as _) }
}
#[inline]
pub unsafe fn GetAdaptersInfo(adapterinfo: Option<*mut IP_ADAPTER_INFO>, sizepointer: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetAdaptersInfo(adapterinfo : *mut IP_ADAPTER_INFO, sizepointer : *mut u32) -> u32);
    unsafe { GetAdaptersInfo(adapterinfo.unwrap_or(core::mem::zeroed()) as _, sizepointer as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetAnycastIpAddressEntry(row: *mut MIB_ANYCASTIPADDRESS_ROW) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetAnycastIpAddressEntry(row : *mut MIB_ANYCASTIPADDRESS_ROW) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetAnycastIpAddressEntry(row as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetAnycastIpAddressTable(family: super::super::Networking::WinSock::ADDRESS_FAMILY, table: *mut *mut MIB_ANYCASTIPADDRESS_TABLE) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetAnycastIpAddressTable(family : super::super::Networking::WinSock:: ADDRESS_FAMILY, table : *mut *mut MIB_ANYCASTIPADDRESS_TABLE) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetAnycastIpAddressTable(family, table as _) }
}
#[inline]
pub unsafe fn GetBestInterface(dwdestaddr: u32, pdwbestifindex: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetBestInterface(dwdestaddr : u32, pdwbestifindex : *mut u32) -> u32);
    unsafe { GetBestInterface(dwdestaddr, pdwbestifindex as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn GetBestInterfaceEx(pdestaddr: *const super::super::Networking::WinSock::SOCKADDR, pdwbestifindex: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetBestInterfaceEx(pdestaddr : *const super::super::Networking::WinSock:: SOCKADDR, pdwbestifindex : *mut u32) -> u32);
    unsafe { GetBestInterfaceEx(pdestaddr, pdwbestifindex as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn GetBestRoute(dwdestaddr: u32, dwsourceaddr: Option<u32>, pbestroute: *mut MIB_IPFORWARDROW) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetBestRoute(dwdestaddr : u32, dwsourceaddr : u32, pbestroute : *mut MIB_IPFORWARDROW) -> u32);
    unsafe { GetBestRoute(dwdestaddr, dwsourceaddr.unwrap_or(core::mem::zeroed()) as _, pbestroute as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetBestRoute2(interfaceluid: Option<*const super::Ndis::NET_LUID_LH>, interfaceindex: u32, sourceaddress: Option<*const super::super::Networking::WinSock::SOCKADDR_INET>, destinationaddress: *const super::super::Networking::WinSock::SOCKADDR_INET, addresssortoptions: u32, bestroute: *mut MIB_IPFORWARD_ROW2, bestsourceaddress: *mut super::super::Networking::WinSock::SOCKADDR_INET) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetBestRoute2(interfaceluid : *const super::Ndis:: NET_LUID_LH, interfaceindex : u32, sourceaddress : *const super::super::Networking::WinSock:: SOCKADDR_INET, destinationaddress : *const super::super::Networking::WinSock:: SOCKADDR_INET, addresssortoptions : u32, bestroute : *mut MIB_IPFORWARD_ROW2, bestsourceaddress : *mut super::super::Networking::WinSock:: SOCKADDR_INET) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetBestRoute2(interfaceluid.unwrap_or(core::mem::zeroed()) as _, interfaceindex, sourceaddress.unwrap_or(core::mem::zeroed()) as _, destinationaddress, addresssortoptions, bestroute as _, bestsourceaddress as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn GetCurrentThreadCompartmentId() -> super::Ndis::NET_IF_COMPARTMENT_ID {
    windows_link::link!("iphlpapi.dll" "system" fn GetCurrentThreadCompartmentId() -> super::Ndis:: NET_IF_COMPARTMENT_ID);
    unsafe { GetCurrentThreadCompartmentId() }
}
#[inline]
pub unsafe fn GetCurrentThreadCompartmentScope(compartmentscope: *mut u32, compartmentid: *mut u32) {
    windows_link::link!("iphlpapi.dll" "system" fn GetCurrentThreadCompartmentScope(compartmentscope : *mut u32, compartmentid : *mut u32));
    unsafe { GetCurrentThreadCompartmentScope(compartmentscope as _, compartmentid as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn GetDefaultCompartmentId() -> super::Ndis::NET_IF_COMPARTMENT_ID {
    windows_link::link!("iphlpapi.dll" "system" fn GetDefaultCompartmentId() -> super::Ndis:: NET_IF_COMPARTMENT_ID);
    unsafe { GetDefaultCompartmentId() }
}
#[inline]
pub unsafe fn GetDnsSettings(settings: *mut DNS_SETTINGS) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetDnsSettings(settings : *mut DNS_SETTINGS) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetDnsSettings(settings as _) }
}
#[inline]
pub unsafe fn GetExtendedTcpTable(ptcptable: Option<*mut core::ffi::c_void>, pdwsize: *mut u32, border: bool, ulaf: u32, tableclass: TCP_TABLE_CLASS, reserved: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetExtendedTcpTable(ptcptable : *mut core::ffi::c_void, pdwsize : *mut u32, border : windows_core::BOOL, ulaf : u32, tableclass : TCP_TABLE_CLASS, reserved : u32) -> u32);
    unsafe { GetExtendedTcpTable(ptcptable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into(), ulaf, tableclass, reserved) }
}
#[inline]
pub unsafe fn GetExtendedUdpTable(pudptable: Option<*mut core::ffi::c_void>, pdwsize: *mut u32, border: bool, ulaf: u32, tableclass: UDP_TABLE_CLASS, reserved: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetExtendedUdpTable(pudptable : *mut core::ffi::c_void, pdwsize : *mut u32, border : windows_core::BOOL, ulaf : u32, tableclass : UDP_TABLE_CLASS, reserved : u32) -> u32);
    unsafe { GetExtendedUdpTable(pudptable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into(), ulaf, tableclass, reserved) }
}
#[inline]
pub unsafe fn GetFriendlyIfIndex(ifindex: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetFriendlyIfIndex(ifindex : u32) -> u32);
    unsafe { GetFriendlyIfIndex(ifindex) }
}
#[inline]
pub unsafe fn GetIcmpStatistics(statistics: *mut MIB_ICMP) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetIcmpStatistics(statistics : *mut MIB_ICMP) -> u32);
    unsafe { GetIcmpStatistics(statistics as _) }
}
#[inline]
pub unsafe fn GetIcmpStatisticsEx(statistics: *mut MIB_ICMP_EX_XPSP1, family: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetIcmpStatisticsEx(statistics : *mut MIB_ICMP_EX_XPSP1, family : u32) -> u32);
    unsafe { GetIcmpStatisticsEx(statistics as _, family) }
}
#[inline]
pub unsafe fn GetIfEntry(pifrow: *mut MIB_IFROW) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetIfEntry(pifrow : *mut MIB_IFROW) -> u32);
    unsafe { GetIfEntry(pifrow as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn GetIfEntry2(row: *mut MIB_IF_ROW2) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetIfEntry2(row : *mut MIB_IF_ROW2) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetIfEntry2(row as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn GetIfEntry2Ex(level: MIB_IF_ENTRY_LEVEL, row: *mut MIB_IF_ROW2) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetIfEntry2Ex(level : MIB_IF_ENTRY_LEVEL, row : *mut MIB_IF_ROW2) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetIfEntry2Ex(level, row as _) }
}
#[inline]
pub unsafe fn GetIfStackTable(table: *mut *mut MIB_IFSTACK_TABLE) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetIfStackTable(table : *mut *mut MIB_IFSTACK_TABLE) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetIfStackTable(table as _) }
}
#[inline]
pub unsafe fn GetIfTable(piftable: Option<*mut MIB_IFTABLE>, pdwsize: *mut u32, border: bool) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetIfTable(piftable : *mut MIB_IFTABLE, pdwsize : *mut u32, border : windows_core::BOOL) -> u32);
    unsafe { GetIfTable(piftable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into()) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn GetIfTable2(table: *mut *mut MIB_IF_TABLE2) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetIfTable2(table : *mut *mut MIB_IF_TABLE2) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetIfTable2(table as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn GetIfTable2Ex(level: MIB_IF_TABLE_LEVEL, table: *mut *mut MIB_IF_TABLE2) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetIfTable2Ex(level : MIB_IF_TABLE_LEVEL, table : *mut *mut MIB_IF_TABLE2) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetIfTable2Ex(level, table as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn GetInterfaceActiveTimestampCapabilities(interfaceluid: *const super::Ndis::NET_LUID_LH, timestampcapabilites: *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceActiveTimestampCapabilities(interfaceluid : *const super::Ndis:: NET_LUID_LH, timestampcapabilites : *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
    unsafe { GetInterfaceActiveTimestampCapabilities(interfaceluid, timestampcapabilites as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn GetInterfaceCurrentTimestampCapabilities(interfaceluid: *const super::Ndis::NET_LUID_LH, timestampcapabilites: *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceCurrentTimestampCapabilities(interfaceluid : *const super::Ndis:: NET_LUID_LH, timestampcapabilites : *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
    unsafe { GetInterfaceCurrentTimestampCapabilities(interfaceluid, timestampcapabilites as _) }
}
#[inline]
pub unsafe fn GetInterfaceDnsSettings(interface: windows_core::GUID, settings: *mut DNS_INTERFACE_SETTINGS) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceDnsSettings(interface : windows_core::GUID, settings : *mut DNS_INTERFACE_SETTINGS) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetInterfaceDnsSettings(core::mem::transmute(interface), settings as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn GetInterfaceHardwareTimestampCapabilities(interfaceluid: *const super::Ndis::NET_LUID_LH, timestampcapabilites: *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceHardwareTimestampCapabilities(interfaceluid : *const super::Ndis:: NET_LUID_LH, timestampcapabilites : *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
    unsafe { GetInterfaceHardwareTimestampCapabilities(interfaceluid, timestampcapabilites as _) }
}
#[inline]
pub unsafe fn GetInterfaceInfo(piftable: Option<*mut IP_INTERFACE_INFO>, dwoutbuflen: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceInfo(piftable : *mut IP_INTERFACE_INFO, dwoutbuflen : *mut u32) -> u32);
    unsafe { GetInterfaceInfo(piftable.unwrap_or(core::mem::zeroed()) as _, dwoutbuflen as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn GetInterfaceSupportedTimestampCapabilities(interfaceluid: *const super::Ndis::NET_LUID_LH, timestampcapabilites: *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceSupportedTimestampCapabilities(interfaceluid : *const super::Ndis:: NET_LUID_LH, timestampcapabilites : *mut INTERFACE_TIMESTAMP_CAPABILITIES) -> u32);
    unsafe { GetInterfaceSupportedTimestampCapabilities(interfaceluid, timestampcapabilites as _) }
}
#[inline]
pub unsafe fn GetInvertedIfStackTable(table: *mut *mut MIB_INVERTEDIFSTACK_TABLE) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetInvertedIfStackTable(table : *mut *mut MIB_INVERTEDIFSTACK_TABLE) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetInvertedIfStackTable(table as _) }
}
#[inline]
pub unsafe fn GetIpAddrTable(pipaddrtable: Option<*mut MIB_IPADDRTABLE>, pdwsize: *mut u32, border: bool) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetIpAddrTable(pipaddrtable : *mut MIB_IPADDRTABLE, pdwsize : *mut u32, border : windows_core::BOOL) -> u32);
    unsafe { GetIpAddrTable(pipaddrtable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into()) }
}
#[inline]
pub unsafe fn GetIpErrorString(errorcode: u32, buffer: Option<windows_core::PWSTR>, size: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetIpErrorString(errorcode : u32, buffer : windows_core::PWSTR, size : *mut u32) -> u32);
    unsafe { GetIpErrorString(errorcode, buffer.unwrap_or(core::mem::zeroed()) as _, size as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpForwardEntry2(row: *mut MIB_IPFORWARD_ROW2) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetIpForwardEntry2(row : *mut MIB_IPFORWARD_ROW2) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetIpForwardEntry2(row as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn GetIpForwardTable(pipforwardtable: Option<*mut MIB_IPFORWARDTABLE>, pdwsize: *mut u32, border: bool) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetIpForwardTable(pipforwardtable : *mut MIB_IPFORWARDTABLE, pdwsize : *mut u32, border : windows_core::BOOL) -> u32);
    unsafe { GetIpForwardTable(pipforwardtable.unwrap_or(core::mem::zeroed()) as _, pdwsize as _, border.into()) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpForwardTable2(family: super::super::Networking::WinSock::ADDRESS_FAMILY, table: *mut *mut MIB_IPFORWARD_TABLE2) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetIpForwardTable2(family : super::super::Networking::WinSock:: ADDRESS_FAMILY, table : *mut *mut MIB_IPFORWARD_TABLE2) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetIpForwardTable2(family, table as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetIpInterfaceEntry(row : *mut MIB_IPINTERFACE_ROW) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetIpInterfaceEntry(row as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpInterfaceTable(family: super::super::Networking::WinSock::ADDRESS_FAMILY, table: *mut *mut MIB_IPINTERFACE_TABLE) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetIpInterfaceTable(family : super::super::Networking::WinSock:: ADDRESS_FAMILY, table : *mut *mut MIB_IPINTERFACE_TABLE) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetIpInterfaceTable(family, table as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpNetEntry2(row: *mut MIB_IPNET_ROW2) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetIpNetEntry2(row : *mut MIB_IPNET_ROW2) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetIpNetEntry2(row as _) }
}
#[inline]
pub unsafe fn GetIpNetTable(ipnettable: Option<*mut MIB_IPNETTABLE>, sizepointer: *mut u32, order: bool) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetIpNetTable(ipnettable : *mut MIB_IPNETTABLE, sizepointer : *mut u32, order : windows_core::BOOL) -> u32);
    unsafe { GetIpNetTable(ipnettable.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpNetTable2(family: super::super::Networking::WinSock::ADDRESS_FAMILY, table: *mut *mut MIB_IPNET_TABLE2) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetIpNetTable2(family : super::super::Networking::WinSock:: ADDRESS_FAMILY, table : *mut *mut MIB_IPNET_TABLE2) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetIpNetTable2(family, table as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn GetIpNetworkConnectionBandwidthEstimates(interfaceindex: u32, addressfamily: super::super::Networking::WinSock::ADDRESS_FAMILY, bandwidthestimates: *mut MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetIpNetworkConnectionBandwidthEstimates(interfaceindex : u32, addressfamily : super::super::Networking::WinSock:: ADDRESS_FAMILY, bandwidthestimates : *mut MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetIpNetworkConnectionBandwidthEstimates(interfaceindex, addressfamily, bandwidthestimates as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpPathEntry(row: *mut MIB_IPPATH_ROW) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetIpPathEntry(row : *mut MIB_IPPATH_ROW) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetIpPathEntry(row as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetIpPathTable(family: super::super::Networking::WinSock::ADDRESS_FAMILY, table: *mut *mut MIB_IPPATH_TABLE) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetIpPathTable(family : super::super::Networking::WinSock:: ADDRESS_FAMILY, table : *mut *mut MIB_IPPATH_TABLE) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetIpPathTable(family, table as _) }
}
#[inline]
pub unsafe fn GetIpStatistics(statistics: *mut MIB_IPSTATS_LH) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetIpStatistics(statistics : *mut MIB_IPSTATS_LH) -> u32);
    unsafe { GetIpStatistics(statistics as _) }
}
#[inline]
pub unsafe fn GetIpStatisticsEx(statistics: *mut MIB_IPSTATS_LH, family: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetIpStatisticsEx(statistics : *mut MIB_IPSTATS_LH, family : u32) -> u32);
    unsafe { GetIpStatisticsEx(statistics as _, family) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn GetJobCompartmentId(jobhandle: super::super::Foundation::HANDLE) -> super::Ndis::NET_IF_COMPARTMENT_ID {
    windows_link::link!("iphlpapi.dll" "system" fn GetJobCompartmentId(jobhandle : super::super::Foundation:: HANDLE) -> super::Ndis:: NET_IF_COMPARTMENT_ID);
    unsafe { GetJobCompartmentId(jobhandle) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetMulticastIpAddressEntry(row: *mut MIB_MULTICASTIPADDRESS_ROW) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetMulticastIpAddressEntry(row : *mut MIB_MULTICASTIPADDRESS_ROW) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetMulticastIpAddressEntry(row as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetMulticastIpAddressTable(family: super::super::Networking::WinSock::ADDRESS_FAMILY, table: *mut *mut MIB_MULTICASTIPADDRESS_TABLE) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetMulticastIpAddressTable(family : super::super::Networking::WinSock:: ADDRESS_FAMILY, table : *mut *mut MIB_MULTICASTIPADDRESS_TABLE) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetMulticastIpAddressTable(family, table as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn GetNetworkConnectivityHint(connectivityhint: *mut super::super::Networking::WinSock::NL_NETWORK_CONNECTIVITY_HINT) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetNetworkConnectivityHint(connectivityhint : *mut super::super::Networking::WinSock:: NL_NETWORK_CONNECTIVITY_HINT) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetNetworkConnectivityHint(connectivityhint as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn GetNetworkConnectivityHintForInterface(interfaceindex: u32, connectivityhint: *mut super::super::Networking::WinSock::NL_NETWORK_CONNECTIVITY_HINT) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetNetworkConnectivityHintForInterface(interfaceindex : u32, connectivityhint : *mut super::super::Networking::WinSock:: NL_NETWORK_CONNECTIVITY_HINT) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetNetworkConnectivityHintForInterface(interfaceindex, connectivityhint as _) }
}
#[inline]
pub unsafe fn GetNetworkInformation(networkguid: *const windows_core::GUID, compartmentid: *mut u32, siteid: *mut u32, networkname: &mut [u16]) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetNetworkInformation(networkguid : *const windows_core::GUID, compartmentid : *mut u32, siteid : *mut u32, networkname : windows_core::PWSTR, length : u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetNetworkInformation(networkguid, compartmentid as _, siteid as _, core::mem::transmute(networkname.as_ptr()), networkname.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetNetworkParams(pfixedinfo: Option<*mut FIXED_INFO_W2KSP1>, poutbuflen: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetNetworkParams(pfixedinfo : *mut FIXED_INFO_W2KSP1, poutbuflen : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetNetworkParams(pfixedinfo.unwrap_or(core::mem::zeroed()) as _, poutbuflen as _) }
}
#[inline]
pub unsafe fn GetNumberOfInterfaces(pdwnumif: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetNumberOfInterfaces(pdwnumif : *mut u32) -> u32);
    unsafe { GetNumberOfInterfaces(pdwnumif as _) }
}
#[inline]
pub unsafe fn GetOwnerModuleFromPidAndInfo(ulpid: u32, pinfo: *const u64, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut core::ffi::c_void, pdwsize: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromPidAndInfo(ulpid : u32, pinfo : *const u64, class : TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : *mut u32) -> u32);
    unsafe { GetOwnerModuleFromPidAndInfo(ulpid, pinfo, class, pbuffer as _, pdwsize as _) }
}
#[inline]
pub unsafe fn GetOwnerModuleFromTcp6Entry(ptcpentry: *const MIB_TCP6ROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut core::ffi::c_void, pdwsize: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromTcp6Entry(ptcpentry : *const MIB_TCP6ROW_OWNER_MODULE, class : TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : *mut u32) -> u32);
    unsafe { GetOwnerModuleFromTcp6Entry(ptcpentry, class, pbuffer as _, pdwsize as _) }
}
#[inline]
pub unsafe fn GetOwnerModuleFromTcpEntry(ptcpentry: *const MIB_TCPROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut core::ffi::c_void, pdwsize: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromTcpEntry(ptcpentry : *const MIB_TCPROW_OWNER_MODULE, class : TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : *mut u32) -> u32);
    unsafe { GetOwnerModuleFromTcpEntry(ptcpentry, class, pbuffer as _, pdwsize as _) }
}
#[inline]
pub unsafe fn GetOwnerModuleFromUdp6Entry(pudpentry: *const MIB_UDP6ROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut core::ffi::c_void, pdwsize: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromUdp6Entry(pudpentry : *const MIB_UDP6ROW_OWNER_MODULE, class : TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : *mut u32) -> u32);
    unsafe { GetOwnerModuleFromUdp6Entry(pudpentry, class, pbuffer as _, pdwsize as _) }
}
#[inline]
pub unsafe fn GetOwnerModuleFromUdpEntry(pudpentry: *const MIB_UDPROW_OWNER_MODULE, class: TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer: *mut core::ffi::c_void, pdwsize: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetOwnerModuleFromUdpEntry(pudpentry : *const MIB_UDPROW_OWNER_MODULE, class : TCPIP_OWNER_MODULE_INFO_CLASS, pbuffer : *mut core::ffi::c_void, pdwsize : *mut u32) -> u32);
    unsafe { GetOwnerModuleFromUdpEntry(pudpentry, class, pbuffer as _, pdwsize as _) }
}
#[inline]
pub unsafe fn GetPerAdapterInfo(ifindex: u32, pperadapterinfo: Option<*mut IP_PER_ADAPTER_INFO_W2KSP1>, poutbuflen: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetPerAdapterInfo(ifindex : u32, pperadapterinfo : *mut IP_PER_ADAPTER_INFO_W2KSP1, poutbuflen : *mut u32) -> u32);
    unsafe { GetPerAdapterInfo(ifindex, pperadapterinfo.unwrap_or(core::mem::zeroed()) as _, poutbuflen as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn GetPerTcp6ConnectionEStats(row: *const MIB_TCP6ROW, estatstype: TCP_ESTATS_TYPE, rw: Option<&mut [u8]>, rwversion: u32, ros: Option<&mut [u8]>, rosversion: u32, rod: Option<&mut [u8]>, rodversion: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetPerTcp6ConnectionEStats(row : *const MIB_TCP6ROW, estatstype : TCP_ESTATS_TYPE, rw : *mut u8, rwversion : u32, rwsize : u32, ros : *mut u8, rosversion : u32, rossize : u32, rod : *mut u8, rodversion : u32, rodsize : u32) -> u32);
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
#[inline]
pub unsafe fn GetPerTcpConnectionEStats(row: *const MIB_TCPROW_LH, estatstype: TCP_ESTATS_TYPE, rw: Option<&mut [u8]>, rwversion: u32, ros: Option<&mut [u8]>, rosversion: u32, rod: Option<&mut [u8]>, rodversion: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetPerTcpConnectionEStats(row : *const MIB_TCPROW_LH, estatstype : TCP_ESTATS_TYPE, rw : *mut u8, rwversion : u32, rwsize : u32, ros : *mut u8, rosversion : u32, rossize : u32, rod : *mut u8, rodversion : u32, rodsize : u32) -> u32);
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
#[inline]
pub unsafe fn GetRTTAndHopCount(destipaddress: u32, hopcount: *mut u32, maxhops: u32, rtt: *mut u32) -> windows_core::Result<()> {
    windows_link::link!("iphlpapi.dll" "system" fn GetRTTAndHopCount(destipaddress : u32, hopcount : *mut u32, maxhops : u32, rtt : *mut u32) -> windows_core::BOOL);
    unsafe { GetRTTAndHopCount(destipaddress, hopcount as _, maxhops, rtt as _).ok() }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn GetSessionCompartmentId(sessionid: u32) -> super::Ndis::NET_IF_COMPARTMENT_ID {
    windows_link::link!("iphlpapi.dll" "system" fn GetSessionCompartmentId(sessionid : u32) -> super::Ndis:: NET_IF_COMPARTMENT_ID);
    unsafe { GetSessionCompartmentId(sessionid) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn GetTcp6Table(tcptable: *mut MIB_TCP6TABLE, sizepointer: *mut u32, order: bool) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetTcp6Table(tcptable : *mut MIB_TCP6TABLE, sizepointer : *mut u32, order : windows_core::BOOL) -> u32);
    unsafe { GetTcp6Table(tcptable as _, sizepointer as _, order.into()) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn GetTcp6Table2(tcptable: *mut MIB_TCP6TABLE2, sizepointer: *mut u32, order: bool) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetTcp6Table2(tcptable : *mut MIB_TCP6TABLE2, sizepointer : *mut u32, order : windows_core::BOOL) -> u32);
    unsafe { GetTcp6Table2(tcptable as _, sizepointer as _, order.into()) }
}
#[inline]
pub unsafe fn GetTcpStatistics(statistics: *mut MIB_TCPSTATS_LH) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetTcpStatistics(statistics : *mut MIB_TCPSTATS_LH) -> u32);
    unsafe { GetTcpStatistics(statistics as _) }
}
#[inline]
pub unsafe fn GetTcpStatisticsEx(statistics: *mut MIB_TCPSTATS_LH, family: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetTcpStatisticsEx(statistics : *mut MIB_TCPSTATS_LH, family : u32) -> u32);
    unsafe { GetTcpStatisticsEx(statistics as _, family) }
}
#[inline]
pub unsafe fn GetTcpStatisticsEx2(statistics: *mut MIB_TCPSTATS2, family: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetTcpStatisticsEx2(statistics : *mut MIB_TCPSTATS2, family : u32) -> u32);
    unsafe { GetTcpStatisticsEx2(statistics as _, family) }
}
#[inline]
pub unsafe fn GetTcpTable(tcptable: Option<*mut MIB_TCPTABLE>, sizepointer: *mut u32, order: bool) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetTcpTable(tcptable : *mut MIB_TCPTABLE, sizepointer : *mut u32, order : windows_core::BOOL) -> u32);
    unsafe { GetTcpTable(tcptable.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[inline]
pub unsafe fn GetTcpTable2(tcptable: Option<*mut MIB_TCPTABLE2>, sizepointer: *mut u32, order: bool) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetTcpTable2(tcptable : *mut MIB_TCPTABLE2, sizepointer : *mut u32, order : windows_core::BOOL) -> u32);
    unsafe { GetTcpTable2(tcptable.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[inline]
pub unsafe fn GetTeredoPort(port: *mut u16) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetTeredoPort(port : *mut u16) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetTeredoPort(port as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn GetUdp6Table(udp6table: Option<*mut MIB_UDP6TABLE>, sizepointer: *mut u32, order: bool) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetUdp6Table(udp6table : *mut MIB_UDP6TABLE, sizepointer : *mut u32, order : windows_core::BOOL) -> u32);
    unsafe { GetUdp6Table(udp6table.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[inline]
pub unsafe fn GetUdpStatistics(stats: *mut MIB_UDPSTATS) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetUdpStatistics(stats : *mut MIB_UDPSTATS) -> u32);
    unsafe { GetUdpStatistics(stats as _) }
}
#[inline]
pub unsafe fn GetUdpStatisticsEx(statistics: *mut MIB_UDPSTATS, family: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetUdpStatisticsEx(statistics : *mut MIB_UDPSTATS, family : u32) -> u32);
    unsafe { GetUdpStatisticsEx(statistics as _, family) }
}
#[inline]
pub unsafe fn GetUdpStatisticsEx2(statistics: *mut MIB_UDPSTATS2, family: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetUdpStatisticsEx2(statistics : *mut MIB_UDPSTATS2, family : u32) -> u32);
    unsafe { GetUdpStatisticsEx2(statistics as _, family) }
}
#[inline]
pub unsafe fn GetUdpTable(udptable: Option<*mut MIB_UDPTABLE>, sizepointer: *mut u32, order: bool) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetUdpTable(udptable : *mut MIB_UDPTABLE, sizepointer : *mut u32, order : windows_core::BOOL) -> u32);
    unsafe { GetUdpTable(udptable.unwrap_or(core::mem::zeroed()) as _, sizepointer as _, order.into()) }
}
#[inline]
pub unsafe fn GetUniDirectionalAdapterInfo(pipifinfo: Option<*mut IP_UNIDIRECTIONAL_ADAPTER_ADDRESS>, dwoutbuflen: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn GetUniDirectionalAdapterInfo(pipifinfo : *mut IP_UNIDIRECTIONAL_ADAPTER_ADDRESS, dwoutbuflen : *mut u32) -> u32);
    unsafe { GetUniDirectionalAdapterInfo(pipifinfo.unwrap_or(core::mem::zeroed()) as _, dwoutbuflen as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetUnicastIpAddressEntry(row: *mut MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetUnicastIpAddressEntry(row : *mut MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetUnicastIpAddressEntry(row as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn GetUnicastIpAddressTable(family: super::super::Networking::WinSock::ADDRESS_FAMILY, table: *mut *mut MIB_UNICASTIPADDRESS_TABLE) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn GetUnicastIpAddressTable(family : super::super::Networking::WinSock:: ADDRESS_FAMILY, table : *mut *mut MIB_UNICASTIPADDRESS_TABLE) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetUnicastIpAddressTable(family, table as _) }
}
#[inline]
pub unsafe fn Icmp6CreateFile() -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_link::link!("iphlpapi.dll" "system" fn Icmp6CreateFile() -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { Icmp6CreateFile() };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn Icmp6ParseReplies(replybuffer: *mut core::ffi::c_void, replysize: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn Icmp6ParseReplies(replybuffer : *mut core::ffi::c_void, replysize : u32) -> u32);
    unsafe { Icmp6ParseReplies(replybuffer as _, replysize) }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn Icmp6SendEcho2(icmphandle: super::super::Foundation::HANDLE, event: Option<super::super::Foundation::HANDLE>, apcroutine: super::super::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, sourceaddress: *const super::super::Networking::WinSock::SOCKADDR_IN6, destinationaddress: *const super::super::Networking::WinSock::SOCKADDR_IN6, requestdata: *const core::ffi::c_void, requestsize: u16, requestoptions: Option<*const IP_OPTION_INFORMATION>, replybuffer: *mut core::ffi::c_void, replysize: u32, timeout: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn Icmp6SendEcho2(icmphandle : super::super::Foundation:: HANDLE, event : super::super::Foundation:: HANDLE, apcroutine : super::super::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, sourceaddress : *const super::super::Networking::WinSock:: SOCKADDR_IN6, destinationaddress : *const super::super::Networking::WinSock:: SOCKADDR_IN6, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
    unsafe { Icmp6SendEcho2(icmphandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, sourceaddress, destinationaddress, requestdata, requestsize, requestoptions.unwrap_or(core::mem::zeroed()) as _, replybuffer as _, replysize, timeout) }
}
#[inline]
pub unsafe fn IcmpCloseHandle(icmphandle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_link::link!("iphlpapi.dll" "system" fn IcmpCloseHandle(icmphandle : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { IcmpCloseHandle(icmphandle).ok() }
}
#[inline]
pub unsafe fn IcmpCreateFile() -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_link::link!("iphlpapi.dll" "system" fn IcmpCreateFile() -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { IcmpCreateFile() };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn IcmpParseReplies(replybuffer: *mut core::ffi::c_void, replysize: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn IcmpParseReplies(replybuffer : *mut core::ffi::c_void, replysize : u32) -> u32);
    unsafe { IcmpParseReplies(replybuffer as _, replysize) }
}
#[inline]
pub unsafe fn IcmpSendEcho(icmphandle: super::super::Foundation::HANDLE, destinationaddress: u32, requestdata: *const core::ffi::c_void, requestsize: u16, requestoptions: Option<*const IP_OPTION_INFORMATION>, replybuffer: *mut core::ffi::c_void, replysize: u32, timeout: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn IcmpSendEcho(icmphandle : super::super::Foundation:: HANDLE, destinationaddress : u32, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
    unsafe { IcmpSendEcho(icmphandle, destinationaddress, requestdata, requestsize, requestoptions.unwrap_or(core::mem::zeroed()) as _, replybuffer as _, replysize, timeout) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn IcmpSendEcho2(icmphandle: super::super::Foundation::HANDLE, event: Option<super::super::Foundation::HANDLE>, apcroutine: super::super::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, destinationaddress: u32, requestdata: *const core::ffi::c_void, requestsize: u16, requestoptions: Option<*const IP_OPTION_INFORMATION>, replybuffer: *mut core::ffi::c_void, replysize: u32, timeout: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn IcmpSendEcho2(icmphandle : super::super::Foundation:: HANDLE, event : super::super::Foundation:: HANDLE, apcroutine : super::super::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, destinationaddress : u32, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
    unsafe { IcmpSendEcho2(icmphandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, destinationaddress, requestdata, requestsize, requestoptions.unwrap_or(core::mem::zeroed()) as _, replybuffer as _, replysize, timeout) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn IcmpSendEcho2Ex(icmphandle: super::super::Foundation::HANDLE, event: Option<super::super::Foundation::HANDLE>, apcroutine: super::super::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, sourceaddress: u32, destinationaddress: u32, requestdata: *const core::ffi::c_void, requestsize: u16, requestoptions: Option<*const IP_OPTION_INFORMATION>, replybuffer: *mut core::ffi::c_void, replysize: u32, timeout: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn IcmpSendEcho2Ex(icmphandle : super::super::Foundation:: HANDLE, event : super::super::Foundation:: HANDLE, apcroutine : super::super::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, sourceaddress : u32, destinationaddress : u32, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
    unsafe { IcmpSendEcho2Ex(icmphandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, sourceaddress, destinationaddress, requestdata, requestsize, requestoptions.unwrap_or(core::mem::zeroed()) as _, replybuffer as _, replysize, timeout) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn InitializeIpForwardEntry(row: *mut MIB_IPFORWARD_ROW2) {
    windows_link::link!("iphlpapi.dll" "system" fn InitializeIpForwardEntry(row : *mut MIB_IPFORWARD_ROW2));
    unsafe { InitializeIpForwardEntry(row as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn InitializeIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW) {
    windows_link::link!("iphlpapi.dll" "system" fn InitializeIpInterfaceEntry(row : *mut MIB_IPINTERFACE_ROW));
    unsafe { InitializeIpInterfaceEntry(row as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn InitializeUnicastIpAddressEntry(row: *mut MIB_UNICASTIPADDRESS_ROW) {
    windows_link::link!("iphlpapi.dll" "system" fn InitializeUnicastIpAddressEntry(row : *mut MIB_UNICASTIPADDRESS_ROW));
    unsafe { InitializeUnicastIpAddressEntry(row as _) }
}
#[inline]
pub unsafe fn IpReleaseAddress(adapterinfo: *const IP_ADAPTER_INDEX_MAP) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn IpReleaseAddress(adapterinfo : *const IP_ADAPTER_INDEX_MAP) -> u32);
    unsafe { IpReleaseAddress(adapterinfo) }
}
#[inline]
pub unsafe fn IpRenewAddress(adapterinfo: *const IP_ADAPTER_INDEX_MAP) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn IpRenewAddress(adapterinfo : *const IP_ADAPTER_INDEX_MAP) -> u32);
    unsafe { IpRenewAddress(adapterinfo) }
}
#[inline]
pub unsafe fn LookupPersistentTcpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn LookupPersistentTcpPortReservation(startport : u16, numberofports : u16, token : *mut u64) -> u32);
    unsafe { LookupPersistentTcpPortReservation(startport, numberofports, token as _) }
}
#[inline]
pub unsafe fn LookupPersistentUdpPortReservation(startport: u16, numberofports: u16, token: *mut u64) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn LookupPersistentUdpPortReservation(startport : u16, numberofports : u16, token : *mut u64) -> u32);
    unsafe { LookupPersistentUdpPortReservation(startport, numberofports, token as _) }
}
#[inline]
pub unsafe fn NhpAllocateAndGetInterfaceInfoFromStack(pptable: *mut *mut IP_INTERFACE_NAME_INFO_W2KSP1, pdwcount: *mut u32, border: bool, hheap: super::super::Foundation::HANDLE, dwflags: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn NhpAllocateAndGetInterfaceInfoFromStack(pptable : *mut *mut IP_INTERFACE_NAME_INFO_W2KSP1, pdwcount : *mut u32, border : windows_core::BOOL, hheap : super::super::Foundation:: HANDLE, dwflags : u32) -> u32);
    unsafe { NhpAllocateAndGetInterfaceInfoFromStack(pptable as _, pdwcount as _, border.into(), hheap, dwflags) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NotifyAddrChange(handle: *mut super::super::Foundation::HANDLE, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn NotifyAddrChange(handle : *mut super::super::Foundation:: HANDLE, overlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    unsafe { NotifyAddrChange(handle as _, overlapped) }
}
#[inline]
pub unsafe fn NotifyIfTimestampConfigChange(callercontext: Option<*const core::ffi::c_void>, callback: PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, notificationhandle: *mut HIFTIMESTAMPCHANGE) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn NotifyIfTimestampConfigChange(callercontext : *const core::ffi::c_void, callback : PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, notificationhandle : *mut HIFTIMESTAMPCHANGE) -> u32);
    unsafe { NotifyIfTimestampConfigChange(callercontext.unwrap_or(core::mem::zeroed()) as _, callback, notificationhandle as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn NotifyIpInterfaceChange(family: super::super::Networking::WinSock::ADDRESS_FAMILY, callback: PIPINTERFACE_CHANGE_CALLBACK, callercontext: Option<*const core::ffi::c_void>, initialnotification: bool, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn NotifyIpInterfaceChange(family : super::super::Networking::WinSock:: ADDRESS_FAMILY, callback : PIPINTERFACE_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, initialnotification : bool, notificationhandle : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { NotifyIpInterfaceChange(family, callback, callercontext.unwrap_or(core::mem::zeroed()) as _, initialnotification, notificationhandle as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn NotifyNetworkConnectivityHintChange(callback: PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK, callercontext: Option<*const core::ffi::c_void>, initialnotification: bool, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn NotifyNetworkConnectivityHintChange(callback : PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, initialnotification : bool, notificationhandle : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { NotifyNetworkConnectivityHintChange(callback, callercontext.unwrap_or(core::mem::zeroed()) as _, initialnotification, notificationhandle as _) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NotifyRouteChange(handle: *mut super::super::Foundation::HANDLE, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn NotifyRouteChange(handle : *mut super::super::Foundation:: HANDLE, overlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    unsafe { NotifyRouteChange(handle as _, overlapped) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn NotifyRouteChange2(addressfamily: super::super::Networking::WinSock::ADDRESS_FAMILY, callback: PIPFORWARD_CHANGE_CALLBACK, callercontext: *const core::ffi::c_void, initialnotification: bool, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn NotifyRouteChange2(addressfamily : super::super::Networking::WinSock:: ADDRESS_FAMILY, callback : PIPFORWARD_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, initialnotification : bool, notificationhandle : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { NotifyRouteChange2(addressfamily, callback, callercontext, initialnotification, notificationhandle as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn NotifyStableUnicastIpAddressTable(family: super::super::Networking::WinSock::ADDRESS_FAMILY, table: *mut *mut MIB_UNICASTIPADDRESS_TABLE, callercallback: PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK, callercontext: *const core::ffi::c_void, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn NotifyStableUnicastIpAddressTable(family : super::super::Networking::WinSock:: ADDRESS_FAMILY, table : *mut *mut MIB_UNICASTIPADDRESS_TABLE, callercallback : PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK, callercontext : *const core::ffi::c_void, notificationhandle : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { NotifyStableUnicastIpAddressTable(family, table as _, callercallback, callercontext, notificationhandle as _) }
}
#[inline]
pub unsafe fn NotifyTeredoPortChange(callback: PTEREDO_PORT_CHANGE_CALLBACK, callercontext: *const core::ffi::c_void, initialnotification: bool, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn NotifyTeredoPortChange(callback : PTEREDO_PORT_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, initialnotification : bool, notificationhandle : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { NotifyTeredoPortChange(callback, callercontext, initialnotification, notificationhandle as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn NotifyUnicastIpAddressChange(family: super::super::Networking::WinSock::ADDRESS_FAMILY, callback: PUNICAST_IPADDRESS_CHANGE_CALLBACK, callercontext: Option<*const core::ffi::c_void>, initialnotification: bool, notificationhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn NotifyUnicastIpAddressChange(family : super::super::Networking::WinSock:: ADDRESS_FAMILY, callback : PUNICAST_IPADDRESS_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, initialnotification : bool, notificationhandle : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { NotifyUnicastIpAddressChange(family, callback, callercontext.unwrap_or(core::mem::zeroed()) as _, initialnotification, notificationhandle as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn ParseNetworkString<P0>(networkstring: P0, types: u32, addressinfo: Option<*mut NET_ADDRESS_INFO>, portnumber: Option<*mut u16>, prefixlength: Option<*mut u8>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("iphlpapi.dll" "system" fn ParseNetworkString(networkstring : windows_core::PCWSTR, types : u32, addressinfo : *mut NET_ADDRESS_INFO, portnumber : *mut u16, prefixlength : *mut u8) -> u32);
    unsafe { ParseNetworkString(networkstring.param().abi(), types, addressinfo.unwrap_or(core::mem::zeroed()) as _, portnumber.unwrap_or(core::mem::zeroed()) as _, prefixlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PfAddFiltersToInterface(ih: *mut core::ffi::c_void, cinfilters: u32, pfiltin: *mut PF_FILTER_DESCRIPTOR, coutfilters: u32, pfiltout: *mut PF_FILTER_DESCRIPTOR, pfhandle: *mut *mut core::ffi::c_void) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn PfAddFiltersToInterface(ih : *mut core::ffi::c_void, cinfilters : u32, pfiltin : *mut PF_FILTER_DESCRIPTOR, coutfilters : u32, pfiltout : *mut PF_FILTER_DESCRIPTOR, pfhandle : *mut *mut core::ffi::c_void) -> u32);
    unsafe { PfAddFiltersToInterface(ih as _, cinfilters, pfiltin as _, coutfilters, pfiltout as _, pfhandle as _) }
}
#[inline]
pub unsafe fn PfAddGlobalFilterToInterface(pinterface: *mut core::ffi::c_void, gffilter: GLOBAL_FILTER) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn PfAddGlobalFilterToInterface(pinterface : *mut core::ffi::c_void, gffilter : GLOBAL_FILTER) -> u32);
    unsafe { PfAddGlobalFilterToInterface(pinterface as _, gffilter) }
}
#[inline]
pub unsafe fn PfBindInterfaceToIPAddress(pinterface: *mut core::ffi::c_void, pfattype: PFADDRESSTYPE, ipaddress: *mut u8) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn PfBindInterfaceToIPAddress(pinterface : *mut core::ffi::c_void, pfattype : PFADDRESSTYPE, ipaddress : *mut u8) -> u32);
    unsafe { PfBindInterfaceToIPAddress(pinterface as _, pfattype, ipaddress as _) }
}
#[inline]
pub unsafe fn PfBindInterfaceToIndex(pinterface: *mut core::ffi::c_void, dwindex: u32, pfatlinktype: PFADDRESSTYPE, linkipaddress: *mut u8) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn PfBindInterfaceToIndex(pinterface : *mut core::ffi::c_void, dwindex : u32, pfatlinktype : PFADDRESSTYPE, linkipaddress : *mut u8) -> u32);
    unsafe { PfBindInterfaceToIndex(pinterface as _, dwindex, pfatlinktype, linkipaddress as _) }
}
#[inline]
pub unsafe fn PfCreateInterface(dwname: u32, inaction: PFFORWARD_ACTION, outaction: PFFORWARD_ACTION, buselog: bool, bmustbeunique: bool, ppinterface: *mut *mut core::ffi::c_void) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn PfCreateInterface(dwname : u32, inaction : PFFORWARD_ACTION, outaction : PFFORWARD_ACTION, buselog : windows_core::BOOL, bmustbeunique : windows_core::BOOL, ppinterface : *mut *mut core::ffi::c_void) -> u32);
    unsafe { PfCreateInterface(dwname, inaction, outaction, buselog.into(), bmustbeunique.into(), ppinterface as _) }
}
#[inline]
pub unsafe fn PfDeleteInterface(pinterface: *mut core::ffi::c_void) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn PfDeleteInterface(pinterface : *mut core::ffi::c_void) -> u32);
    unsafe { PfDeleteInterface(pinterface as _) }
}
#[inline]
pub unsafe fn PfDeleteLog() -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn PfDeleteLog() -> u32);
    unsafe { PfDeleteLog() }
}
#[inline]
pub unsafe fn PfGetInterfaceStatistics(pinterface: *mut core::ffi::c_void, ppfstats: *mut PF_INTERFACE_STATS, pdwbuffersize: *mut u32, fresetcounters: bool) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn PfGetInterfaceStatistics(pinterface : *mut core::ffi::c_void, ppfstats : *mut PF_INTERFACE_STATS, pdwbuffersize : *mut u32, fresetcounters : windows_core::BOOL) -> u32);
    unsafe { PfGetInterfaceStatistics(pinterface as _, ppfstats as _, pdwbuffersize as _, fresetcounters.into()) }
}
#[inline]
pub unsafe fn PfMakeLog(hevent: super::super::Foundation::HANDLE) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn PfMakeLog(hevent : super::super::Foundation:: HANDLE) -> u32);
    unsafe { PfMakeLog(hevent) }
}
#[inline]
pub unsafe fn PfRebindFilters(pinterface: *mut core::ffi::c_void, platebindinfo: *mut PF_LATEBIND_INFO) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn PfRebindFilters(pinterface : *mut core::ffi::c_void, platebindinfo : *mut PF_LATEBIND_INFO) -> u32);
    unsafe { PfRebindFilters(pinterface as _, platebindinfo as _) }
}
#[inline]
pub unsafe fn PfRemoveFilterHandles(pinterface: *mut core::ffi::c_void, cfilters: u32, pvhandles: *mut *mut core::ffi::c_void) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn PfRemoveFilterHandles(pinterface : *mut core::ffi::c_void, cfilters : u32, pvhandles : *mut *mut core::ffi::c_void) -> u32);
    unsafe { PfRemoveFilterHandles(pinterface as _, cfilters, pvhandles as _) }
}
#[inline]
pub unsafe fn PfRemoveFiltersFromInterface(ih: *mut core::ffi::c_void, cinfilters: u32, pfiltin: *mut PF_FILTER_DESCRIPTOR, coutfilters: u32, pfiltout: *mut PF_FILTER_DESCRIPTOR) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn PfRemoveFiltersFromInterface(ih : *mut core::ffi::c_void, cinfilters : u32, pfiltin : *mut PF_FILTER_DESCRIPTOR, coutfilters : u32, pfiltout : *mut PF_FILTER_DESCRIPTOR) -> u32);
    unsafe { PfRemoveFiltersFromInterface(ih as _, cinfilters, pfiltin as _, coutfilters, pfiltout as _) }
}
#[inline]
pub unsafe fn PfRemoveGlobalFilterFromInterface(pinterface: *mut core::ffi::c_void, gffilter: GLOBAL_FILTER) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn PfRemoveGlobalFilterFromInterface(pinterface : *mut core::ffi::c_void, gffilter : GLOBAL_FILTER) -> u32);
    unsafe { PfRemoveGlobalFilterFromInterface(pinterface as _, gffilter) }
}
#[inline]
pub unsafe fn PfSetLogBuffer(pbbuffer: *mut u8, dwsize: u32, dwthreshold: u32, dwentries: u32, pdwloggedentries: *mut u32, pdwlostentries: *mut u32, pdwsizeused: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn PfSetLogBuffer(pbbuffer : *mut u8, dwsize : u32, dwthreshold : u32, dwentries : u32, pdwloggedentries : *mut u32, pdwlostentries : *mut u32, pdwsizeused : *mut u32) -> u32);
    unsafe { PfSetLogBuffer(pbbuffer as _, dwsize, dwthreshold, dwentries, pdwloggedentries as _, pdwlostentries as _, pdwsizeused as _) }
}
#[inline]
pub unsafe fn PfTestPacket(pininterface: *mut core::ffi::c_void, poutinterface: *mut core::ffi::c_void, cbytes: u32, pbpacket: *mut u8, ppaction: *mut PFFORWARD_ACTION) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn PfTestPacket(pininterface : *mut core::ffi::c_void, poutinterface : *mut core::ffi::c_void, cbytes : u32, pbpacket : *mut u8, ppaction : *mut PFFORWARD_ACTION) -> u32);
    unsafe { PfTestPacket(pininterface as _, poutinterface as _, cbytes, pbpacket as _, ppaction as _) }
}
#[inline]
pub unsafe fn PfUnBindInterface(pinterface: *mut core::ffi::c_void) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn PfUnBindInterface(pinterface : *mut core::ffi::c_void) -> u32);
    unsafe { PfUnBindInterface(pinterface as _) }
}
#[inline]
pub unsafe fn RegisterInterfaceTimestampConfigChange(callback: PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, callercontext: Option<*const core::ffi::c_void>, notificationhandle: *mut HIFTIMESTAMPCHANGE) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn RegisterInterfaceTimestampConfigChange(callback : PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, notificationhandle : *mut HIFTIMESTAMPCHANGE) -> u32);
    unsafe { RegisterInterfaceTimestampConfigChange(callback, callercontext.unwrap_or(core::mem::zeroed()) as _, notificationhandle as _) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn ResolveIpNetEntry2(row: *mut MIB_IPNET_ROW2, sourceaddress: Option<*const super::super::Networking::WinSock::SOCKADDR_INET>) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn ResolveIpNetEntry2(row : *mut MIB_IPNET_ROW2, sourceaddress : *const super::super::Networking::WinSock:: SOCKADDR_INET) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { ResolveIpNetEntry2(row as _, sourceaddress.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn ResolveNeighbor(networkaddress: *const super::super::Networking::WinSock::SOCKADDR, physicaladdress: *mut core::ffi::c_void, physicaladdresslength: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn ResolveNeighbor(networkaddress : *const super::super::Networking::WinSock:: SOCKADDR, physicaladdress : *mut core::ffi::c_void, physicaladdresslength : *mut u32) -> u32);
    unsafe { ResolveNeighbor(networkaddress, physicaladdress as _, physicaladdresslength as _) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn RestoreMediaSense(poverlapped: *const super::super::System::IO::OVERLAPPED, lpdwenablecount: Option<*mut u32>) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn RestoreMediaSense(poverlapped : *const super::super::System::IO:: OVERLAPPED, lpdwenablecount : *mut u32) -> u32);
    unsafe { RestoreMediaSense(poverlapped, lpdwenablecount.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SendARP(destip: u32, srcip: u32, pmacaddr: *mut core::ffi::c_void, phyaddrlen: *mut u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn SendARP(destip : u32, srcip : u32, pmacaddr : *mut core::ffi::c_void, phyaddrlen : *mut u32) -> u32);
    unsafe { SendARP(destip, srcip, pmacaddr as _, phyaddrlen as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn SetCurrentThreadCompartmentId(compartmentid: super::Ndis::NET_IF_COMPARTMENT_ID) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn SetCurrentThreadCompartmentId(compartmentid : super::Ndis:: NET_IF_COMPARTMENT_ID) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { SetCurrentThreadCompartmentId(compartmentid) }
}
#[inline]
pub unsafe fn SetCurrentThreadCompartmentScope(compartmentscope: u32) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn SetCurrentThreadCompartmentScope(compartmentscope : u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { SetCurrentThreadCompartmentScope(compartmentscope) }
}
#[inline]
pub unsafe fn SetDnsSettings(settings: *const DNS_SETTINGS) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn SetDnsSettings(settings : *const DNS_SETTINGS) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { SetDnsSettings(settings) }
}
#[inline]
pub unsafe fn SetIfEntry(pifrow: *const MIB_IFROW) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn SetIfEntry(pifrow : *const MIB_IFROW) -> u32);
    unsafe { SetIfEntry(pifrow) }
}
#[inline]
pub unsafe fn SetInterfaceDnsSettings(interface: windows_core::GUID, settings: *const DNS_INTERFACE_SETTINGS) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn SetInterfaceDnsSettings(interface : windows_core::GUID, settings : *const DNS_INTERFACE_SETTINGS) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { SetInterfaceDnsSettings(core::mem::transmute(interface), settings) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn SetIpForwardEntry(proute: *const MIB_IPFORWARDROW) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn SetIpForwardEntry(proute : *const MIB_IPFORWARDROW) -> u32);
    unsafe { SetIpForwardEntry(proute) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn SetIpForwardEntry2(route: *const MIB_IPFORWARD_ROW2) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn SetIpForwardEntry2(route : *const MIB_IPFORWARD_ROW2) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { SetIpForwardEntry2(route) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn SetIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn SetIpInterfaceEntry(row : *mut MIB_IPINTERFACE_ROW) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { SetIpInterfaceEntry(row as _) }
}
#[inline]
pub unsafe fn SetIpNetEntry(parpentry: *const MIB_IPNETROW_LH) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn SetIpNetEntry(parpentry : *const MIB_IPNETROW_LH) -> u32);
    unsafe { SetIpNetEntry(parpentry) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn SetIpNetEntry2(row: *const MIB_IPNET_ROW2) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn SetIpNetEntry2(row : *const MIB_IPNET_ROW2) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { SetIpNetEntry2(row) }
}
#[inline]
pub unsafe fn SetIpStatistics(pipstats: *const MIB_IPSTATS_LH) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn SetIpStatistics(pipstats : *const MIB_IPSTATS_LH) -> u32);
    unsafe { SetIpStatistics(pipstats) }
}
#[inline]
pub unsafe fn SetIpStatisticsEx(statistics: *const MIB_IPSTATS_LH, family: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn SetIpStatisticsEx(statistics : *const MIB_IPSTATS_LH, family : u32) -> u32);
    unsafe { SetIpStatisticsEx(statistics, family) }
}
#[inline]
pub unsafe fn SetIpTTL(nttl: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn SetIpTTL(nttl : u32) -> u32);
    unsafe { SetIpTTL(nttl) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn SetJobCompartmentId(jobhandle: super::super::Foundation::HANDLE, compartmentid: super::Ndis::NET_IF_COMPARTMENT_ID) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn SetJobCompartmentId(jobhandle : super::super::Foundation:: HANDLE, compartmentid : super::Ndis:: NET_IF_COMPARTMENT_ID) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { SetJobCompartmentId(jobhandle, compartmentid) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn SetNetworkInformation<P2>(networkguid: *const windows_core::GUID, compartmentid: super::Ndis::NET_IF_COMPARTMENT_ID, networkname: P2) -> super::super::Foundation::WIN32_ERROR
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("iphlpapi.dll" "system" fn SetNetworkInformation(networkguid : *const windows_core::GUID, compartmentid : super::Ndis:: NET_IF_COMPARTMENT_ID, networkname : windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { SetNetworkInformation(networkguid, compartmentid, networkname.param().abi()) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn SetPerTcp6ConnectionEStats(row: *const MIB_TCP6ROW, estatstype: TCP_ESTATS_TYPE, rw: &[u8], rwversion: u32, offset: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn SetPerTcp6ConnectionEStats(row : *const MIB_TCP6ROW, estatstype : TCP_ESTATS_TYPE, rw : *const u8, rwversion : u32, rwsize : u32, offset : u32) -> u32);
    unsafe { SetPerTcp6ConnectionEStats(row, estatstype, core::mem::transmute(rw.as_ptr()), rwversion, rw.len().try_into().unwrap(), offset) }
}
#[inline]
pub unsafe fn SetPerTcpConnectionEStats(row: *const MIB_TCPROW_LH, estatstype: TCP_ESTATS_TYPE, rw: &[u8], rwversion: u32, offset: u32) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn SetPerTcpConnectionEStats(row : *const MIB_TCPROW_LH, estatstype : TCP_ESTATS_TYPE, rw : *const u8, rwversion : u32, rwsize : u32, offset : u32) -> u32);
    unsafe { SetPerTcpConnectionEStats(row, estatstype, core::mem::transmute(rw.as_ptr()), rwversion, rw.len().try_into().unwrap(), offset) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn SetSessionCompartmentId(sessionid: u32, compartmentid: super::Ndis::NET_IF_COMPARTMENT_ID) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn SetSessionCompartmentId(sessionid : u32, compartmentid : super::Ndis:: NET_IF_COMPARTMENT_ID) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { SetSessionCompartmentId(sessionid, compartmentid) }
}
#[inline]
pub unsafe fn SetTcpEntry(ptcprow: *const MIB_TCPROW_LH) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn SetTcpEntry(ptcprow : *const MIB_TCPROW_LH) -> u32);
    unsafe { SetTcpEntry(ptcprow) }
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn SetUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("iphlpapi.dll" "system" fn SetUnicastIpAddressEntry(row : *const MIB_UNICASTIPADDRESS_ROW) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { SetUnicastIpAddressEntry(row) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn UnenableRouter(poverlapped: *const super::super::System::IO::OVERLAPPED, lpdwenablecount: Option<*mut u32>) -> u32 {
    windows_link::link!("iphlpapi.dll" "system" fn UnenableRouter(poverlapped : *const super::super::System::IO:: OVERLAPPED, lpdwenablecount : *mut u32) -> u32);
    unsafe { UnenableRouter(poverlapped, lpdwenablecount.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn UnregisterInterfaceTimestampConfigChange(notificationhandle: HIFTIMESTAMPCHANGE) {
    windows_link::link!("iphlpapi.dll" "system" fn UnregisterInterfaceTimestampConfigChange(notificationhandle : HIFTIMESTAMPCHANGE));
    unsafe { UnregisterInterfaceTimestampConfigChange(notificationhandle) }
}
#[inline]
pub unsafe fn if_indextoname(interfaceindex: u32, interfacename: &mut [u8; 256]) -> windows_core::PSTR {
    windows_link::link!("iphlpapi.dll" "system" fn if_indextoname(interfaceindex : u32, interfacename : windows_core::PSTR) -> windows_core::PSTR);
    unsafe { if_indextoname(interfaceindex, core::mem::transmute(interfacename.as_ptr())) }
}
#[inline]
pub unsafe fn if_nametoindex<P0>(interfacename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("iphlpapi.dll" "system" fn if_nametoindex(interfacename : windows_core::PCSTR) -> u32);
    unsafe { if_nametoindex(interfacename.param().abi()) }
}
pub const ANY_SIZE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ARP_SEND_REPLY {
    pub DestAddress: u32,
    pub SrcAddress: u32,
}
pub const BEST_IF: u32 = 20u32;
pub const BEST_ROUTE: u32 = 21u32;
pub const BROADCAST_NODETYPE: u32 = 1u32;
pub const DEFAULT_MINIMUM_ENTITIES: u32 = 32u32;
pub const DEST_LONGER: u32 = 29u32;
pub const DEST_MATCHING: u32 = 28u32;
pub const DEST_SHORTER: u32 = 30u32;
pub const DNS_DDR_ADAPTER_ENABLE_DOH: u32 = 1u32;
pub const DNS_DDR_ADAPTER_ENABLE_UDP_FALLBACK: u32 = 2u32;
pub const DNS_DOH_AUTO_UPGRADE_SERVER: u32 = 8u32;
pub const DNS_DOH_POLICY_AUTO: u32 = 16u32;
pub const DNS_DOH_POLICY_DISABLE: u32 = 8u32;
pub const DNS_DOH_POLICY_NOT_CONFIGURED: u32 = 4u32;
pub const DNS_DOH_POLICY_REQUIRED: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_DOH_SERVER_SETTINGS {
    pub Template: windows_core::PWSTR,
    pub Flags: u64,
}
pub const DNS_DOH_SERVER_SETTINGS_ENABLE: u32 = 2u32;
pub const DNS_DOH_SERVER_SETTINGS_ENABLE_AUTO: u32 = 1u32;
pub const DNS_DOH_SERVER_SETTINGS_ENABLE_DDR: u32 = 16u32;
pub const DNS_DOH_SERVER_SETTINGS_FALLBACK_TO_UDP: u32 = 4u32;
pub const DNS_ENABLE_DDR: u32 = 64u32;
pub const DNS_ENABLE_DOH: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_INTERFACE_SETTINGS {
    pub Version: u32,
    pub Flags: u64,
    pub Domain: windows_core::PWSTR,
    pub NameServer: windows_core::PWSTR,
    pub SearchList: windows_core::PWSTR,
    pub RegistrationEnabled: u32,
    pub RegisterAdapterName: u32,
    pub EnableLLMNR: u32,
    pub QueryAdapterName: u32,
    pub ProfileNameServer: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_INTERFACE_SETTINGS3 {
    pub Version: u32,
    pub Flags: u64,
    pub Domain: windows_core::PWSTR,
    pub NameServer: windows_core::PWSTR,
    pub SearchList: windows_core::PWSTR,
    pub RegistrationEnabled: u32,
    pub RegisterAdapterName: u32,
    pub EnableLLMNR: u32,
    pub QueryAdapterName: u32,
    pub ProfileNameServer: windows_core::PWSTR,
    pub DisableUnconstrainedQueries: u32,
    pub SupplementalSearchList: windows_core::PWSTR,
    pub cServerProperties: u32,
    pub ServerProperties: *mut DNS_SERVER_PROPERTY,
    pub cProfileServerProperties: u32,
    pub ProfileServerProperties: *mut DNS_SERVER_PROPERTY,
}
impl Default for DNS_INTERFACE_SETTINGS3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_INTERFACE_SETTINGS4 {
    pub Version: u32,
    pub Flags: u64,
    pub Domain: windows_core::PWSTR,
    pub NameServer: windows_core::PWSTR,
    pub SearchList: windows_core::PWSTR,
    pub RegistrationEnabled: u32,
    pub RegisterAdapterName: u32,
    pub EnableLLMNR: u32,
    pub QueryAdapterName: u32,
    pub ProfileNameServer: windows_core::PWSTR,
    pub DisableUnconstrainedQueries: u32,
    pub SupplementalSearchList: windows_core::PWSTR,
    pub cServerProperties: u32,
    pub ServerProperties: *mut DNS_SERVER_PROPERTY,
    pub cProfileServerProperties: u32,
    pub ProfileServerProperties: *mut DNS_SERVER_PROPERTY,
    pub EncryptedDnsAdapterFlags: u32,
}
impl Default for DNS_INTERFACE_SETTINGS4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_INTERFACE_SETTINGS_EX {
    pub SettingsV1: DNS_INTERFACE_SETTINGS,
    pub DisableUnconstrainedQueries: u32,
    pub SupplementalSearchList: windows_core::PWSTR,
}
pub const DNS_INTERFACE_SETTINGS_VERSION1: u32 = 1u32;
pub const DNS_INTERFACE_SETTINGS_VERSION2: u32 = 2u32;
pub const DNS_INTERFACE_SETTINGS_VERSION3: u32 = 3u32;
pub const DNS_INTERFACE_SETTINGS_VERSION4: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_SERVER_PROPERTY {
    pub Version: u32,
    pub ServerIndex: u32,
    pub Type: DNS_SERVER_PROPERTY_TYPE,
    pub Property: DNS_SERVER_PROPERTY_TYPES,
}
impl Default for DNS_SERVER_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_SERVER_PROPERTY_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub union DNS_SERVER_PROPERTY_TYPES {
    pub DohSettings: *mut DNS_DOH_SERVER_SETTINGS,
}
impl Default for DNS_SERVER_PROPERTY_TYPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_SERVER_PROPERTY_VERSION1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_SETTINGS {
    pub Version: u32,
    pub Flags: u64,
    pub Hostname: windows_core::PWSTR,
    pub Domain: windows_core::PWSTR,
    pub SearchList: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_SETTINGS2 {
    pub Version: u32,
    pub Flags: u64,
    pub Hostname: windows_core::PWSTR,
    pub Domain: windows_core::PWSTR,
    pub SearchList: windows_core::PWSTR,
    pub SettingFlags: u64,
}
pub const DNS_SETTINGS_ENABLE_LLMNR: u32 = 128u32;
pub const DNS_SETTINGS_QUERY_ADAPTER_NAME: u32 = 256u32;
pub const DNS_SETTINGS_VERSION1: u32 = 1u32;
pub const DNS_SETTINGS_VERSION2: u32 = 2u32;
pub const DNS_SETTING_DDR: u32 = 32768u32;
pub const DNS_SETTING_DISABLE_UNCONSTRAINED_QUERIES: u32 = 1024u32;
pub const DNS_SETTING_DOH: u32 = 4096u32;
pub const DNS_SETTING_DOH_PROFILE: u32 = 8192u32;
pub const DNS_SETTING_DOMAIN: u32 = 32u32;
pub const DNS_SETTING_ENCRYPTED_DNS_ADAPTER_FLAGS: u32 = 16384u32;
pub const DNS_SETTING_HOSTNAME: u32 = 64u32;
pub const DNS_SETTING_IPV6: u32 = 1u32;
pub const DNS_SETTING_NAMESERVER: u32 = 2u32;
pub const DNS_SETTING_PROFILE_NAMESERVER: u32 = 512u32;
pub const DNS_SETTING_REGISTER_ADAPTER_NAME: u32 = 16u32;
pub const DNS_SETTING_REGISTRATION_ENABLED: u32 = 8u32;
pub const DNS_SETTING_SEARCHLIST: u32 = 4u32;
pub const DNS_SETTING_SUPPLEMENTAL_SEARCH_LIST: u32 = 2048u32;
pub const DnsServerDohProperty: DNS_SERVER_PROPERTY_TYPE = DNS_SERVER_PROPERTY_TYPE(1i32);
pub const DnsServerInvalidProperty: DNS_SERVER_PROPERTY_TYPE = DNS_SERVER_PROPERTY_TYPE(0i32);
pub const ERROR_BASE: u32 = 23000u32;
pub const ERROR_IPV6_NOT_IMPLEMENTED: u32 = 23003u32;
pub const FD_FLAGS_ALLFLAGS: u32 = 1u32;
pub const FD_FLAGS_NOSYN: u32 = 1u32;
pub const FILTER_ICMP_CODE_ANY: u8 = 255u8;
pub const FILTER_ICMP_TYPE_ANY: u8 = 255u8;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FIXED_INFO_W2KSP1 {
    pub HostName: [i8; 132],
    pub DomainName: [i8; 132],
    pub CurrentDnsServer: *mut IP_ADDR_STRING,
    pub DnsServerList: IP_ADDR_STRING,
    pub NodeType: u32,
    pub ScopeId: [i8; 260],
    pub EnableRouting: u32,
    pub EnableProxy: u32,
    pub EnableDns: u32,
}
impl Default for FIXED_INFO_W2KSP1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GAA_FLAG_INCLUDE_ALL_COMPARTMENTS: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(512u32);
pub const GAA_FLAG_INCLUDE_ALL_INTERFACES: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(256u32);
pub const GAA_FLAG_INCLUDE_GATEWAYS: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(128u32);
pub const GAA_FLAG_INCLUDE_PREFIX: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(16u32);
pub const GAA_FLAG_INCLUDE_TUNNEL_BINDINGORDER: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(1024u32);
pub const GAA_FLAG_INCLUDE_WINS_INFO: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(64u32);
pub const GAA_FLAG_SKIP_ANYCAST: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(2u32);
pub const GAA_FLAG_SKIP_DNS_INFO: u32 = 2048u32;
pub const GAA_FLAG_SKIP_DNS_SERVER: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(8u32);
pub const GAA_FLAG_SKIP_FRIENDLY_NAME: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(32u32);
pub const GAA_FLAG_SKIP_MULTICAST: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(4u32);
pub const GAA_FLAG_SKIP_UNICAST: GET_ADAPTERS_ADDRESSES_FLAGS = GET_ADAPTERS_ADDRESSES_FLAGS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_ADAPTERS_ADDRESSES_FLAGS(pub u32);
impl GET_ADAPTERS_ADDRESSES_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GET_ADAPTERS_ADDRESSES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GET_ADAPTERS_ADDRESSES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GET_ADAPTERS_ADDRESSES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GET_ADAPTERS_ADDRESSES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GET_ADAPTERS_ADDRESSES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GF_FRAGCACHE: GLOBAL_FILTER = GLOBAL_FILTER(9i32);
pub const GF_FRAGMENTS: GLOBAL_FILTER = GLOBAL_FILTER(2i32);
pub const GF_STRONGHOST: GLOBAL_FILTER = GLOBAL_FILTER(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GLOBAL_FILTER(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HIFTIMESTAMPCHANGE(pub *mut core::ffi::c_void);
impl HIFTIMESTAMPCHANGE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HIFTIMESTAMPCHANGE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_link::link!("iphlpapi.dll" "system" fn UnregisterInterfaceTimestampConfigChange(notificationhandle : *mut core::ffi::c_void));
            unsafe {
                UnregisterInterfaceTimestampConfigChange(self.0);
            }
        }
    }
}
impl Default for HIFTIMESTAMPCHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HYBRID_NODETYPE: u32 = 8u32;
pub const ICMP4_DST_UNREACH: ICMP4_TYPE = ICMP4_TYPE(3i32);
pub const ICMP4_ECHO_REPLY: ICMP4_TYPE = ICMP4_TYPE(0i32);
pub const ICMP4_ECHO_REQUEST: ICMP4_TYPE = ICMP4_TYPE(8i32);
pub const ICMP4_MASK_REPLY: ICMP4_TYPE = ICMP4_TYPE(18i32);
pub const ICMP4_MASK_REQUEST: ICMP4_TYPE = ICMP4_TYPE(17i32);
pub const ICMP4_PARAM_PROB: ICMP4_TYPE = ICMP4_TYPE(12i32);
pub const ICMP4_REDIRECT: ICMP4_TYPE = ICMP4_TYPE(5i32);
pub const ICMP4_ROUTER_ADVERT: ICMP4_TYPE = ICMP4_TYPE(9i32);
pub const ICMP4_ROUTER_SOLICIT: ICMP4_TYPE = ICMP4_TYPE(10i32);
pub const ICMP4_SOURCE_QUENCH: ICMP4_TYPE = ICMP4_TYPE(4i32);
pub const ICMP4_TIMESTAMP_REPLY: ICMP4_TYPE = ICMP4_TYPE(14i32);
pub const ICMP4_TIMESTAMP_REQUEST: ICMP4_TYPE = ICMP4_TYPE(13i32);
pub const ICMP4_TIME_EXCEEDED: ICMP4_TYPE = ICMP4_TYPE(11i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ICMP4_TYPE(pub i32);
pub const ICMP6_DST_UNREACH: ICMP6_TYPE = ICMP6_TYPE(1i32);
pub const ICMP6_ECHO_REPLY: ICMP6_TYPE = ICMP6_TYPE(129i32);
pub const ICMP6_ECHO_REQUEST: ICMP6_TYPE = ICMP6_TYPE(128i32);
pub const ICMP6_INFOMSG_MASK: u32 = 128u32;
pub const ICMP6_MEMBERSHIP_QUERY: ICMP6_TYPE = ICMP6_TYPE(130i32);
pub const ICMP6_MEMBERSHIP_REDUCTION: ICMP6_TYPE = ICMP6_TYPE(132i32);
pub const ICMP6_MEMBERSHIP_REPORT: ICMP6_TYPE = ICMP6_TYPE(131i32);
pub const ICMP6_PACKET_TOO_BIG: ICMP6_TYPE = ICMP6_TYPE(2i32);
pub const ICMP6_PARAM_PROB: ICMP6_TYPE = ICMP6_TYPE(4i32);
pub const ICMP6_TIME_EXCEEDED: ICMP6_TYPE = ICMP6_TYPE(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ICMP6_TYPE(pub i32);
pub const ICMP6_V2_MEMBERSHIP_REPORT: ICMP6_TYPE = ICMP6_TYPE(143i32);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ICMPV6_ECHO_REPLY_LH {
    pub Address: IPV6_ADDRESS_EX,
    pub Status: u32,
    pub RoundTripTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ICMP_ECHO_REPLY {
    pub Address: u32,
    pub Status: u32,
    pub RoundTripTime: u32,
    pub DataSize: u16,
    pub Reserved: u16,
    pub Data: *mut core::ffi::c_void,
    pub Options: IP_OPTION_INFORMATION,
}
impl Default for ICMP_ECHO_REPLY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ICMP_ECHO_REPLY32 {
    pub Address: u32,
    pub Status: u32,
    pub RoundTripTime: u32,
    pub DataSize: u16,
    pub Reserved: u16,
    pub Data: *mut core::ffi::c_void,
    pub Options: IP_OPTION_INFORMATION32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for ICMP_ECHO_REPLY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ICMP_STATS: u32 = 11u32;
pub const IF_ACCESS_BROADCAST: IF_ACCESS_TYPE = IF_ACCESS_TYPE(2i32);
pub const IF_ACCESS_LOOPBACK: IF_ACCESS_TYPE = IF_ACCESS_TYPE(1i32);
pub const IF_ACCESS_POINTTOMULTIPOINT: IF_ACCESS_TYPE = IF_ACCESS_TYPE(4i32);
pub const IF_ACCESS_POINTTOPOINT: IF_ACCESS_TYPE = IF_ACCESS_TYPE(3i32);
pub const IF_ACCESS_POINT_TO_MULTI_POINT: IF_ACCESS_TYPE = IF_ACCESS_TYPE(4i32);
pub const IF_ACCESS_POINT_TO_POINT: IF_ACCESS_TYPE = IF_ACCESS_TYPE(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IF_ACCESS_TYPE(pub i32);
pub const IF_ADMIN_STATUS_DOWN: u32 = 2u32;
pub const IF_ADMIN_STATUS_TESTING: u32 = 3u32;
pub const IF_ADMIN_STATUS_UP: u32 = 1u32;
pub const IF_CHECK_MCAST: u32 = 1u32;
pub const IF_CHECK_NONE: u32 = 0u32;
pub const IF_CHECK_SEND: u32 = 2u32;
pub const IF_CONNECTION_DEDICATED: u32 = 1u32;
pub const IF_CONNECTION_DEMAND: u32 = 3u32;
pub const IF_CONNECTION_PASSIVE: u32 = 2u32;
pub const IF_NUMBER: u32 = 0u32;
pub const IF_OPER_STATUS_CONNECTED: INTERNAL_IF_OPER_STATUS = INTERNAL_IF_OPER_STATUS(4i32);
pub const IF_OPER_STATUS_CONNECTING: INTERNAL_IF_OPER_STATUS = INTERNAL_IF_OPER_STATUS(3i32);
pub const IF_OPER_STATUS_DISCONNECTED: INTERNAL_IF_OPER_STATUS = INTERNAL_IF_OPER_STATUS(2i32);
pub const IF_OPER_STATUS_NON_OPERATIONAL: INTERNAL_IF_OPER_STATUS = INTERNAL_IF_OPER_STATUS(0i32);
pub const IF_OPER_STATUS_OPERATIONAL: INTERNAL_IF_OPER_STATUS = INTERNAL_IF_OPER_STATUS(5i32);
pub const IF_OPER_STATUS_UNREACHABLE: INTERNAL_IF_OPER_STATUS = INTERNAL_IF_OPER_STATUS(1i32);
pub const IF_ROW: u32 = 2u32;
pub const IF_STATUS: u32 = 25u32;
pub const IF_TABLE: u32 = 1u32;
pub const IF_TYPE_A12MPPSWITCH: u32 = 130u32;
pub const IF_TYPE_AAL2: u32 = 187u32;
pub const IF_TYPE_AAL5: u32 = 49u32;
pub const IF_TYPE_ADSL: u32 = 94u32;
pub const IF_TYPE_AFLANE_8023: u32 = 59u32;
pub const IF_TYPE_AFLANE_8025: u32 = 60u32;
pub const IF_TYPE_ARAP: u32 = 88u32;
pub const IF_TYPE_ARCNET: u32 = 35u32;
pub const IF_TYPE_ARCNET_PLUS: u32 = 36u32;
pub const IF_TYPE_ASYNC: u32 = 84u32;
pub const IF_TYPE_ATM: u32 = 37u32;
pub const IF_TYPE_ATM_DXI: u32 = 105u32;
pub const IF_TYPE_ATM_FUNI: u32 = 106u32;
pub const IF_TYPE_ATM_IMA: u32 = 107u32;
pub const IF_TYPE_ATM_LOGICAL: u32 = 80u32;
pub const IF_TYPE_ATM_RADIO: u32 = 189u32;
pub const IF_TYPE_ATM_SUBINTERFACE: u32 = 134u32;
pub const IF_TYPE_ATM_VCI_ENDPT: u32 = 194u32;
pub const IF_TYPE_ATM_VIRTUAL: u32 = 149u32;
pub const IF_TYPE_BASIC_ISDN: u32 = 20u32;
pub const IF_TYPE_BGP_POLICY_ACCOUNTING: u32 = 162u32;
pub const IF_TYPE_BSC: u32 = 83u32;
pub const IF_TYPE_CCTEMUL: u32 = 61u32;
pub const IF_TYPE_CES: u32 = 133u32;
pub const IF_TYPE_CHANNEL: u32 = 70u32;
pub const IF_TYPE_CNR: u32 = 85u32;
pub const IF_TYPE_COFFEE: u32 = 132u32;
pub const IF_TYPE_COMPOSITELINK: u32 = 155u32;
pub const IF_TYPE_DCN: u32 = 141u32;
pub const IF_TYPE_DDN_X25: u32 = 4u32;
pub const IF_TYPE_DIGITALPOWERLINE: u32 = 138u32;
pub const IF_TYPE_DIGITAL_WRAPPER_OVERHEAD_CHANNEL: u32 = 186u32;
pub const IF_TYPE_DLSW: u32 = 74u32;
pub const IF_TYPE_DOCSCABLE_DOWNSTREAM: u32 = 128u32;
pub const IF_TYPE_DOCSCABLE_MACLAYER: u32 = 127u32;
pub const IF_TYPE_DOCSCABLE_UPSTREAM: u32 = 129u32;
pub const IF_TYPE_DS0: u32 = 81u32;
pub const IF_TYPE_DS0_BUNDLE: u32 = 82u32;
pub const IF_TYPE_DS1: u32 = 18u32;
pub const IF_TYPE_DS1_FDL: u32 = 170u32;
pub const IF_TYPE_DS3: u32 = 30u32;
pub const IF_TYPE_DTM: u32 = 140u32;
pub const IF_TYPE_DVBRCC_DOWNSTREAM: u32 = 147u32;
pub const IF_TYPE_DVBRCC_MACLAYER: u32 = 146u32;
pub const IF_TYPE_DVBRCC_UPSTREAM: u32 = 148u32;
pub const IF_TYPE_DVB_ASI_IN: u32 = 172u32;
pub const IF_TYPE_DVB_ASI_OUT: u32 = 173u32;
pub const IF_TYPE_E1: u32 = 19u32;
pub const IF_TYPE_EON: u32 = 25u32;
pub const IF_TYPE_EPLRS: u32 = 87u32;
pub const IF_TYPE_ESCON: u32 = 73u32;
pub const IF_TYPE_ETHERNET_3MBIT: u32 = 26u32;
pub const IF_TYPE_ETHERNET_CSMACD: u32 = 6u32;
pub const IF_TYPE_FAST: u32 = 125u32;
pub const IF_TYPE_FASTETHER: u32 = 62u32;
pub const IF_TYPE_FASTETHER_FX: u32 = 69u32;
pub const IF_TYPE_FDDI: u32 = 15u32;
pub const IF_TYPE_FIBRECHANNEL: u32 = 56u32;
pub const IF_TYPE_FRAMERELAY: u32 = 32u32;
pub const IF_TYPE_FRAMERELAY_INTERCONNECT: u32 = 58u32;
pub const IF_TYPE_FRAMERELAY_MPI: u32 = 92u32;
pub const IF_TYPE_FRAMERELAY_SERVICE: u32 = 44u32;
pub const IF_TYPE_FRF16_MFR_BUNDLE: u32 = 163u32;
pub const IF_TYPE_FR_DLCI_ENDPT: u32 = 193u32;
pub const IF_TYPE_FR_FORWARD: u32 = 158u32;
pub const IF_TYPE_G703_2MB: u32 = 67u32;
pub const IF_TYPE_G703_64K: u32 = 66u32;
pub const IF_TYPE_GIGABITETHERNET: u32 = 117u32;
pub const IF_TYPE_GR303_IDT: u32 = 178u32;
pub const IF_TYPE_GR303_RDT: u32 = 177u32;
pub const IF_TYPE_H323_GATEKEEPER: u32 = 164u32;
pub const IF_TYPE_H323_PROXY: u32 = 165u32;
pub const IF_TYPE_HDH_1822: u32 = 3u32;
pub const IF_TYPE_HDLC: u32 = 118u32;
pub const IF_TYPE_HDSL2: u32 = 168u32;
pub const IF_TYPE_HIPERLAN2: u32 = 183u32;
pub const IF_TYPE_HIPPI: u32 = 47u32;
pub const IF_TYPE_HIPPIINTERFACE: u32 = 57u32;
pub const IF_TYPE_HOSTPAD: u32 = 90u32;
pub const IF_TYPE_HSSI: u32 = 46u32;
pub const IF_TYPE_HYPERCHANNEL: u32 = 14u32;
pub const IF_TYPE_IBM370PARCHAN: u32 = 72u32;
pub const IF_TYPE_IDSL: u32 = 154u32;
pub const IF_TYPE_IEEE1394: u32 = 144u32;
pub const IF_TYPE_IEEE80211: u32 = 71u32;
pub const IF_TYPE_IEEE80212: u32 = 55u32;
pub const IF_TYPE_IEEE802154: u32 = 259u32;
pub const IF_TYPE_IEEE80216_WMAN: u32 = 237u32;
pub const IF_TYPE_IEEE8023AD_LAG: u32 = 161u32;
pub const IF_TYPE_IF_GSN: u32 = 145u32;
pub const IF_TYPE_IMT: u32 = 190u32;
pub const IF_TYPE_INTERLEAVE: u32 = 124u32;
pub const IF_TYPE_IP: u32 = 126u32;
pub const IF_TYPE_IPFORWARD: u32 = 142u32;
pub const IF_TYPE_IPOVER_ATM: u32 = 114u32;
pub const IF_TYPE_IPOVER_CDLC: u32 = 109u32;
pub const IF_TYPE_IPOVER_CLAW: u32 = 110u32;
pub const IF_TYPE_IPSWITCH: u32 = 78u32;
pub const IF_TYPE_IS088023_CSMACD: u32 = 7u32;
pub const IF_TYPE_ISDN: u32 = 63u32;
pub const IF_TYPE_ISDN_S: u32 = 75u32;
pub const IF_TYPE_ISDN_U: u32 = 76u32;
pub const IF_TYPE_ISO88022_LLC: u32 = 41u32;
pub const IF_TYPE_ISO88024_TOKENBUS: u32 = 8u32;
pub const IF_TYPE_ISO88025R_DTR: u32 = 86u32;
pub const IF_TYPE_ISO88025_CRFPRINT: u32 = 98u32;
pub const IF_TYPE_ISO88025_FIBER: u32 = 115u32;
pub const IF_TYPE_ISO88025_TOKENRING: u32 = 9u32;
pub const IF_TYPE_ISO88026_MAN: u32 = 10u32;
pub const IF_TYPE_ISUP: u32 = 179u32;
pub const IF_TYPE_L2_VLAN: u32 = 135u32;
pub const IF_TYPE_L3_IPVLAN: u32 = 136u32;
pub const IF_TYPE_L3_IPXVLAN: u32 = 137u32;
pub const IF_TYPE_LAP_B: u32 = 16u32;
pub const IF_TYPE_LAP_D: u32 = 77u32;
pub const IF_TYPE_LAP_F: u32 = 119u32;
pub const IF_TYPE_LOCALTALK: u32 = 42u32;
pub const IF_TYPE_MEDIAMAILOVERIP: u32 = 139u32;
pub const IF_TYPE_MF_SIGLINK: u32 = 167u32;
pub const IF_TYPE_MIO_X25: u32 = 38u32;
pub const IF_TYPE_MODEM: u32 = 48u32;
pub const IF_TYPE_MPC: u32 = 113u32;
pub const IF_TYPE_MPLS: u32 = 166u32;
pub const IF_TYPE_MPLS_TUNNEL: u32 = 150u32;
pub const IF_TYPE_MSDSL: u32 = 143u32;
pub const IF_TYPE_MVL: u32 = 191u32;
pub const IF_TYPE_MYRINET: u32 = 99u32;
pub const IF_TYPE_NFAS: u32 = 175u32;
pub const IF_TYPE_NSIP: u32 = 27u32;
pub const IF_TYPE_OPTICAL_CHANNEL: u32 = 195u32;
pub const IF_TYPE_OPTICAL_TRANSPORT: u32 = 196u32;
pub const IF_TYPE_OTHER: u32 = 1u32;
pub const IF_TYPE_PARA: u32 = 34u32;
pub const IF_TYPE_PLC: u32 = 174u32;
pub const IF_TYPE_POS: u32 = 171u32;
pub const IF_TYPE_PPP: u32 = 23u32;
pub const IF_TYPE_PPPMULTILINKBUNDLE: u32 = 108u32;
pub const IF_TYPE_PRIMARY_ISDN: u32 = 21u32;
pub const IF_TYPE_PROP_BWA_P2MP: u32 = 184u32;
pub const IF_TYPE_PROP_CNLS: u32 = 89u32;
pub const IF_TYPE_PROP_DOCS_WIRELESS_DOWNSTREAM: u32 = 181u32;
pub const IF_TYPE_PROP_DOCS_WIRELESS_MACLAYER: u32 = 180u32;
pub const IF_TYPE_PROP_DOCS_WIRELESS_UPSTREAM: u32 = 182u32;
pub const IF_TYPE_PROP_MULTIPLEXOR: u32 = 54u32;
pub const IF_TYPE_PROP_POINT2POINT_SERIAL: u32 = 22u32;
pub const IF_TYPE_PROP_VIRTUAL: u32 = 53u32;
pub const IF_TYPE_PROP_WIRELESS_P2P: u32 = 157u32;
pub const IF_TYPE_PROTEON_10MBIT: u32 = 12u32;
pub const IF_TYPE_PROTEON_80MBIT: u32 = 13u32;
pub const IF_TYPE_QLLC: u32 = 68u32;
pub const IF_TYPE_RADIO_MAC: u32 = 188u32;
pub const IF_TYPE_RADSL: u32 = 95u32;
pub const IF_TYPE_REACH_DSL: u32 = 192u32;
pub const IF_TYPE_REGULAR_1822: u32 = 2u32;
pub const IF_TYPE_RFC1483: u32 = 159u32;
pub const IF_TYPE_RFC877_X25: u32 = 5u32;
pub const IF_TYPE_RS232: u32 = 33u32;
pub const IF_TYPE_RSRB: u32 = 79u32;
pub const IF_TYPE_SDLC: u32 = 17u32;
pub const IF_TYPE_SDSL: u32 = 96u32;
pub const IF_TYPE_SHDSL: u32 = 169u32;
pub const IF_TYPE_SIP: u32 = 31u32;
pub const IF_TYPE_SLIP: u32 = 28u32;
pub const IF_TYPE_SMDS_DXI: u32 = 43u32;
pub const IF_TYPE_SMDS_ICIP: u32 = 52u32;
pub const IF_TYPE_SOFTWARE_LOOPBACK: u32 = 24u32;
pub const IF_TYPE_SONET: u32 = 39u32;
pub const IF_TYPE_SONET_OVERHEAD_CHANNEL: u32 = 185u32;
pub const IF_TYPE_SONET_PATH: u32 = 50u32;
pub const IF_TYPE_SONET_VT: u32 = 51u32;
pub const IF_TYPE_SRP: u32 = 151u32;
pub const IF_TYPE_SS7_SIGLINK: u32 = 156u32;
pub const IF_TYPE_STACKTOSTACK: u32 = 111u32;
pub const IF_TYPE_STARLAN: u32 = 11u32;
pub const IF_TYPE_TDLC: u32 = 116u32;
pub const IF_TYPE_TERMPAD: u32 = 91u32;
pub const IF_TYPE_TR008: u32 = 176u32;
pub const IF_TYPE_TRANSPHDLC: u32 = 123u32;
pub const IF_TYPE_TUNNEL: u32 = 131u32;
pub const IF_TYPE_ULTRA: u32 = 29u32;
pub const IF_TYPE_USB: u32 = 160u32;
pub const IF_TYPE_V11: u32 = 64u32;
pub const IF_TYPE_V35: u32 = 45u32;
pub const IF_TYPE_V36: u32 = 65u32;
pub const IF_TYPE_V37: u32 = 120u32;
pub const IF_TYPE_VDSL: u32 = 97u32;
pub const IF_TYPE_VIRTUALIPADDRESS: u32 = 112u32;
pub const IF_TYPE_VOICEOVERATM: u32 = 152u32;
pub const IF_TYPE_VOICEOVERFRAMERELAY: u32 = 153u32;
pub const IF_TYPE_VOICE_EM: u32 = 100u32;
pub const IF_TYPE_VOICE_ENCAP: u32 = 103u32;
pub const IF_TYPE_VOICE_FXO: u32 = 101u32;
pub const IF_TYPE_VOICE_FXS: u32 = 102u32;
pub const IF_TYPE_VOICE_OVERIP: u32 = 104u32;
pub const IF_TYPE_WWANPP: u32 = 243u32;
pub const IF_TYPE_WWANPP2: u32 = 244u32;
pub const IF_TYPE_X213: u32 = 93u32;
pub const IF_TYPE_X25_HUNTGROUP: u32 = 122u32;
pub const IF_TYPE_X25_MLP: u32 = 121u32;
pub const IF_TYPE_X25_PLE: u32 = 40u32;
pub const IF_TYPE_XBOX_WIRELESS: u32 = 281u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERFACE_HARDWARE_CROSSTIMESTAMP {
    pub SystemTimestamp1: u64,
    pub HardwareClockTimestamp: u64,
    pub SystemTimestamp2: u64,
}
pub const INTERFACE_HARDWARE_CROSSTIMESTAMP_VERSION_1: u32 = 1u32;
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
pub const INTERFACE_TIMESTAMP_CAPABILITIES_VERSION_1: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERNAL_IF_OPER_STATUS(pub i32);
pub const IOCTL_ARP_SEND_REQUEST: u32 = 103u32;
pub const IOCTL_IP_ADDCHANGE_NOTIFY_REQUEST: u32 = 102u32;
pub const IOCTL_IP_GET_BEST_INTERFACE: u32 = 105u32;
pub const IOCTL_IP_INTERFACE_INFO: u32 = 104u32;
pub const IOCTL_IP_RTCHANGE_NOTIFY_REQUEST: u32 = 101u32;
pub const IOCTL_IP_UNIDIRECTIONAL_ADAPTER_ADDRESS: u32 = 106u32;
pub const IP6_STATS: u32 = 36u32;
pub const IPRTRMGR_PID: u32 = 10000u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IPV6_ADDRESS_EX {
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u16; 8],
    pub sin6_scope_id: u32,
}
impl Default for IPV6_ADDRESS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IPV6_GLOBAL_INFO: u32 = 4294901775u32;
pub const IPV6_ROUTE_INFO: u32 = 4294901776u32;
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_ADDRESSES_LH {
    pub Anonymous1: IP_ADAPTER_ADDRESSES_LH_0,
    pub Next: *mut IP_ADAPTER_ADDRESSES_LH,
    pub AdapterName: windows_core::PSTR,
    pub FirstUnicastAddress: *mut IP_ADAPTER_UNICAST_ADDRESS_LH,
    pub FirstAnycastAddress: *mut IP_ADAPTER_ANYCAST_ADDRESS_XP,
    pub FirstMulticastAddress: *mut IP_ADAPTER_MULTICAST_ADDRESS_XP,
    pub FirstDnsServerAddress: *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    pub DnsSuffix: windows_core::PWSTR,
    pub Description: windows_core::PWSTR,
    pub FriendlyName: windows_core::PWSTR,
    pub PhysicalAddress: [u8; 8],
    pub PhysicalAddressLength: u32,
    pub Anonymous2: IP_ADAPTER_ADDRESSES_LH_1,
    pub Mtu: u32,
    pub IfType: u32,
    pub OperStatus: super::Ndis::IF_OPER_STATUS,
    pub Ipv6IfIndex: u32,
    pub ZoneIndices: [u32; 16],
    pub FirstPrefix: *mut IP_ADAPTER_PREFIX_XP,
    pub TransmitLinkSpeed: u64,
    pub ReceiveLinkSpeed: u64,
    pub FirstWinsServerAddress: *mut IP_ADAPTER_WINS_SERVER_ADDRESS_LH,
    pub FirstGatewayAddress: *mut IP_ADAPTER_GATEWAY_ADDRESS_LH,
    pub Ipv4Metric: u32,
    pub Ipv6Metric: u32,
    pub Luid: super::Ndis::NET_LUID_LH,
    pub Dhcpv4Server: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub CompartmentId: super::Ndis::NET_IF_COMPARTMENT_ID,
    pub NetworkGuid: windows_core::GUID,
    pub ConnectionType: super::Ndis::NET_IF_CONNECTION_TYPE,
    pub TunnelType: super::Ndis::TUNNEL_TYPE,
    pub Dhcpv6Server: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub Dhcpv6ClientDuid: [u8; 130],
    pub Dhcpv6ClientDuidLength: u32,
    pub Dhcpv6Iaid: u32,
    pub FirstDnsSuffix: *mut IP_ADAPTER_DNS_SUFFIX,
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for IP_ADAPTER_ADDRESSES_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_ADDRESSES_LH_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_ADDRESSES_LH_0_0,
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for IP_ADAPTER_ADDRESSES_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_ADAPTER_ADDRESSES_LH_0_0 {
    pub Length: u32,
    pub IfIndex: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_ADDRESSES_LH_1 {
    pub Flags: u32,
    pub Anonymous: IP_ADAPTER_ADDRESSES_LH_1_0,
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for IP_ADAPTER_ADDRESSES_LH_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_ADAPTER_ADDRESSES_LH_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_ADDRESSES_XP {
    pub Anonymous: IP_ADAPTER_ADDRESSES_XP_0,
    pub Next: *mut IP_ADAPTER_ADDRESSES_XP,
    pub AdapterName: windows_core::PSTR,
    pub FirstUnicastAddress: *mut IP_ADAPTER_UNICAST_ADDRESS_XP,
    pub FirstAnycastAddress: *mut IP_ADAPTER_ANYCAST_ADDRESS_XP,
    pub FirstMulticastAddress: *mut IP_ADAPTER_MULTICAST_ADDRESS_XP,
    pub FirstDnsServerAddress: *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    pub DnsSuffix: windows_core::PWSTR,
    pub Description: windows_core::PWSTR,
    pub FriendlyName: windows_core::PWSTR,
    pub PhysicalAddress: [u8; 8],
    pub PhysicalAddressLength: u32,
    pub Flags: u32,
    pub Mtu: u32,
    pub IfType: u32,
    pub OperStatus: super::Ndis::IF_OPER_STATUS,
    pub Ipv6IfIndex: u32,
    pub ZoneIndices: [u32; 16],
    pub FirstPrefix: *mut IP_ADAPTER_PREFIX_XP,
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for IP_ADAPTER_ADDRESSES_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_ADDRESSES_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_ADDRESSES_XP_0_0,
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for IP_ADAPTER_ADDRESSES_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_ADAPTER_ADDRESSES_XP_0_0 {
    pub Length: u32,
    pub IfIndex: u32,
}
pub const IP_ADAPTER_ADDRESS_DNS_ELIGIBLE: u32 = 1u32;
pub const IP_ADAPTER_ADDRESS_TRANSIENT: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_ANYCAST_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_ANYCAST_ADDRESS_XP_0,
    pub Next: *mut IP_ADAPTER_ANYCAST_ADDRESS_XP,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADAPTER_ANYCAST_ADDRESS_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
pub const IP_ADAPTER_DDNS_ENABLED: u32 = 1u32;
pub const IP_ADAPTER_DHCP_ENABLED: u32 = 4u32;
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0,
    pub Next: *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {
    pub Length: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IP_ADAPTER_DNS_SUFFIX {
    pub Next: *mut IP_ADAPTER_DNS_SUFFIX,
    pub String: [u16; 256],
}
impl Default for IP_ADAPTER_DNS_SUFFIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_GATEWAY_ADDRESS_LH {
    pub Anonymous: IP_ADAPTER_GATEWAY_ADDRESS_LH_0,
    pub Next: *mut IP_ADAPTER_GATEWAY_ADDRESS_LH,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADAPTER_GATEWAY_ADDRESS_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {
    pub Length: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IP_ADAPTER_INDEX_MAP {
    pub Index: u32,
    pub Name: [u16; 128],
}
impl Default for IP_ADAPTER_INDEX_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IP_ADAPTER_INFO {
    pub Next: *mut IP_ADAPTER_INFO,
    pub ComboIndex: u32,
    pub AdapterName: [i8; 260],
    pub Description: [i8; 132],
    pub AddressLength: u32,
    pub Address: [u8; 8],
    pub Index: u32,
    pub Type: u32,
    pub DhcpEnabled: u32,
    pub CurrentIpAddress: *mut IP_ADDR_STRING,
    pub IpAddressList: IP_ADDR_STRING,
    pub GatewayList: IP_ADDR_STRING,
    pub DhcpServer: IP_ADDR_STRING,
    pub HaveWins: windows_core::BOOL,
    pub PrimaryWinsServer: IP_ADDR_STRING,
    pub SecondaryWinsServer: IP_ADDR_STRING,
    pub LeaseObtained: i64,
    pub LeaseExpires: i64,
}
impl Default for IP_ADAPTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_ADAPTER_IPV4_ENABLED: u32 = 128u32;
pub const IP_ADAPTER_IPV6_ENABLED: u32 = 256u32;
pub const IP_ADAPTER_IPV6_MANAGE_ADDRESS_CONFIG: u32 = 512u32;
pub const IP_ADAPTER_IPV6_OTHER_STATEFUL_CONFIG: u32 = 32u32;
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_MULTICAST_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_MULTICAST_ADDRESS_XP_0,
    pub Next: *mut IP_ADAPTER_MULTICAST_ADDRESS_XP,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADAPTER_MULTICAST_ADDRESS_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
pub const IP_ADAPTER_NETBIOS_OVER_TCPIP_ENABLED: u32 = 64u32;
pub const IP_ADAPTER_NO_MULTICAST: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IP_ADAPTER_ORDER_MAP {
    pub NumAdapters: u32,
    pub AdapterOrder: [u32; 1],
}
impl Default for IP_ADAPTER_ORDER_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_PREFIX_XP {
    pub Anonymous: IP_ADAPTER_PREFIX_XP_0,
    pub Next: *mut IP_ADAPTER_PREFIX_XP,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub PrefixLength: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADAPTER_PREFIX_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_PREFIX_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_PREFIX_XP_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADAPTER_PREFIX_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_ADAPTER_PREFIX_XP_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
pub const IP_ADAPTER_RECEIVE_ONLY: u32 = 8u32;
pub const IP_ADAPTER_REGISTER_ADAPTER_SUFFIX: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
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
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADAPTER_UNICAST_ADDRESS_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_UNICAST_ADDRESS_LH_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_LH_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADAPTER_UNICAST_ADDRESS_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
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
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADAPTER_UNICAST_ADDRESS_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_UNICAST_ADDRESS_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_XP_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADAPTER_UNICAST_ADDRESS_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_ADAPTER_UNICAST_ADDRESS_XP_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    pub Anonymous: IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0,
    pub Next: *mut IP_ADAPTER_WINS_SERVER_ADDRESS_LH,
    pub Address: super::super::Networking::WinSock::SOCKET_ADDRESS,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {
    pub Length: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct IP_ADDRESS_PREFIX {
    pub Prefix: super::super::Networking::WinSock::SOCKADDR_INET,
    pub PrefixLength: u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for IP_ADDRESS_PREFIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IP_ADDRESS_STRING {
    pub String: [i8; 16],
}
impl Default for IP_ADDRESS_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_ADDRROW: u32 = 5u32;
pub const IP_ADDRTABLE: u32 = 4u32;
pub const IP_ADDR_ADDED: u32 = 11023u32;
pub const IP_ADDR_DELETED: u32 = 11019u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IP_ADDR_STRING {
    pub Next: *mut IP_ADDR_STRING,
    pub IpAddress: IP_ADDRESS_STRING,
    pub IpMask: IP_ADDRESS_STRING,
    pub Context: u32,
}
impl Default for IP_ADDR_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_BAD_DESTINATION: u32 = 11018u32;
pub const IP_BAD_HEADER: u32 = 11042u32;
pub const IP_BAD_OPTION: u32 = 11007u32;
pub const IP_BAD_REQ: u32 = 11011u32;
pub const IP_BAD_ROUTE: u32 = 11012u32;
pub const IP_BIND_ADAPTER: u32 = 11026u32;
pub const IP_BUF_TOO_SMALL: u32 = 11001u32;
pub const IP_DEMAND_DIAL_FILTER_INFO: u32 = 4294901769u32;
pub const IP_DEMAND_DIAL_FILTER_INFO_V6: u32 = 4294901779u32;
pub const IP_DEST_ADDR_UNREACHABLE: u32 = 11003u32;
pub const IP_DEST_HOST_UNREACHABLE: u32 = 11003u32;
pub const IP_DEST_NET_UNREACHABLE: u32 = 11002u32;
pub const IP_DEST_NO_ROUTE: u32 = 11002u32;
pub const IP_DEST_PORT_UNREACHABLE: u32 = 11005u32;
pub const IP_DEST_PROHIBITED: u32 = 11004u32;
pub const IP_DEST_PROT_UNREACHABLE: u32 = 11004u32;
pub const IP_DEST_SCOPE_MISMATCH: u32 = 11045u32;
pub const IP_DEST_UNREACHABLE: u32 = 11040u32;
pub const IP_DEVICE_DOES_NOT_EXIST: u32 = 11028u32;
pub const IP_DUPLICATE_ADDRESS: u32 = 11029u32;
pub const IP_DUPLICATE_IPADD: u32 = 11034u32;
pub const IP_EXPORT_INCLUDED: u32 = 1u32;
pub const IP_FILTER_ENABLE_INFO: u32 = 4294901781u32;
pub const IP_FILTER_ENABLE_INFO_V6: u32 = 4294901782u32;
pub const IP_FLAG_DF: u32 = 2u32;
pub const IP_FLAG_REVERSE: u32 = 1u32;
pub const IP_FORWARDNUMBER: u32 = 6u32;
pub const IP_FORWARDROW: u32 = 8u32;
pub const IP_FORWARDTABLE: u32 = 7u32;
pub const IP_GENERAL_FAILURE: u32 = 11050u32;
pub const IP_GENERAL_INFO_BASE: u32 = 4294901760u32;
pub const IP_GLOBAL_INFO: u32 = 4294901763u32;
pub const IP_HOP_LIMIT_EXCEEDED: u32 = 11013u32;
pub const IP_HW_ERROR: u32 = 11008u32;
pub const IP_ICMP_ERROR: u32 = 11044u32;
pub const IP_IFFILTER_INFO: u32 = 4294901773u32;
pub const IP_IFFILTER_INFO_V6: u32 = 4294901780u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IP_INTERFACE_INFO {
    pub NumAdapters: i32,
    pub Adapter: [IP_ADAPTER_INDEX_MAP; 1],
}
impl Default for IP_INTERFACE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_INTERFACE_METRIC_CHANGE: u32 = 11030u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_INTERFACE_NAME_INFO_W2KSP1 {
    pub Index: u32,
    pub MediaType: u32,
    pub ConnectionType: u8,
    pub AccessType: u8,
    pub DeviceGuid: windows_core::GUID,
    pub InterfaceGuid: windows_core::GUID,
}
pub const IP_INTERFACE_STATUS_INFO: u32 = 4294901764u32;
pub const IP_INTERFACE_WOL_CAPABILITY_CHANGE: u32 = 11033u32;
pub const IP_IN_FILTER_INFO: u32 = 4294901761u32;
pub const IP_IN_FILTER_INFO_V6: u32 = 4294901777u32;
pub const IP_IPINIP_CFG_INFO: u32 = 4294901772u32;
pub const IP_MCAST_BOUNDARY_INFO: u32 = 4294901771u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_MCAST_COUNTER_INFO {
    pub InMcastOctets: u64,
    pub OutMcastOctets: u64,
    pub InMcastPkts: u64,
    pub OutMcastPkts: u64,
}
pub const IP_MCAST_HEARBEAT_INFO: u32 = 4294901770u32;
pub const IP_MCAST_LIMIT_INFO: u32 = 4294901774u32;
pub const IP_MEDIA_CONNECT: u32 = 11024u32;
pub const IP_MEDIA_DISCONNECT: u32 = 11025u32;
pub const IP_MTU_CHANGE: u32 = 11021u32;
pub const IP_NEGOTIATING_IPSEC: u32 = 11032u32;
pub const IP_NETROW: u32 = 10u32;
pub const IP_NETTABLE: u32 = 9u32;
pub const IP_NO_RESOURCES: u32 = 11006u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IP_OPTION_INFORMATION {
    pub Ttl: u8,
    pub Tos: u8,
    pub Flags: u8,
    pub OptionsSize: u8,
    pub OptionsData: *mut u8,
}
impl Default for IP_OPTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IP_OPTION_INFORMATION32 {
    pub Ttl: u8,
    pub Tos: u8,
    pub Flags: u8,
    pub OptionsSize: u8,
    pub OptionsData: *mut u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for IP_OPTION_INFORMATION32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_OPTION_TOO_BIG: u32 = 11017u32;
pub const IP_OUT_FILTER_INFO: u32 = 4294901762u32;
pub const IP_OUT_FILTER_INFO_V6: u32 = 4294901778u32;
pub const IP_PACKET_TOO_BIG: u32 = 11009u32;
pub const IP_PARAMETER_PROBLEM: u32 = 11015u32;
pub const IP_PARAM_PROBLEM: u32 = 11015u32;
pub const IP_PENDING: u32 = 11255u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IP_PER_ADAPTER_INFO_W2KSP1 {
    pub AutoconfigEnabled: u32,
    pub AutoconfigActive: u32,
    pub CurrentDnsServer: *mut IP_ADDR_STRING,
    pub DnsServerList: IP_ADDR_STRING,
}
impl Default for IP_PER_ADAPTER_INFO_W2KSP1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_PROT_PRIORITY_INFO: u32 = 4294901766u32;
pub const IP_PROT_PRIORITY_INFO_EX: u32 = 4294901783u32;
pub const IP_REASSEMBLY_TIME_EXCEEDED: u32 = 11014u32;
pub const IP_RECONFIG_SECFLTR: u32 = 11031u32;
pub const IP_REQ_TIMED_OUT: u32 = 11010u32;
pub const IP_ROUTER_DISC_INFO: u32 = 4294901767u32;
pub const IP_ROUTER_MANAGER_VERSION: u32 = 1u32;
pub const IP_ROUTE_INFO: u32 = 4294901765u32;
pub const IP_SOURCE_QUENCH: u32 = 11016u32;
pub const IP_SPEC_MTU_CHANGE: u32 = 11020u32;
pub const IP_STATS: u32 = 3u32;
pub const IP_STATUS_BASE: u32 = 11000u32;
pub const IP_SUCCESS: u32 = 0u32;
pub const IP_TIME_EXCEEDED: u32 = 11041u32;
pub const IP_TTL_EXPIRED_REASSEM: u32 = 11014u32;
pub const IP_TTL_EXPIRED_TRANSIT: u32 = 11013u32;
pub const IP_UNBIND_ADAPTER: u32 = 11027u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    pub NumAdapters: u32,
    pub Address: [u32; 1],
}
impl Default for IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_UNLOAD: u32 = 11022u32;
pub const IP_UNRECOGNIZED_NEXT_HEADER: u32 = 11043u32;
pub const LB_DST_ADDR_USE_DSTADDR_FLAG: u32 = 8u32;
pub const LB_DST_ADDR_USE_SRCADDR_FLAG: u32 = 4u32;
pub const LB_DST_MASK_LATE_FLAG: u32 = 32u32;
pub const LB_SRC_ADDR_USE_DSTADDR_FLAG: u32 = 2u32;
pub const LB_SRC_ADDR_USE_SRCADDR_FLAG: u32 = 1u32;
pub const LB_SRC_MASK_LATE_FLAG: u32 = 16u32;
pub const MAXLEN_IFDESCR: u32 = 256u32;
pub const MAXLEN_PHYSADDR: u32 = 8u32;
pub const MAX_ADAPTER_ADDRESS_LENGTH: u32 = 8u32;
pub const MAX_ADAPTER_DESCRIPTION_LENGTH: u32 = 128u32;
pub const MAX_ADAPTER_NAME: u32 = 128u32;
pub const MAX_ADAPTER_NAME_LENGTH: u32 = 256u32;
pub const MAX_DHCPV6_DUID_LENGTH: u32 = 130u32;
pub const MAX_DNS_SUFFIX_STRING_LENGTH: u32 = 256u32;
pub const MAX_DOMAIN_NAME_LEN: u32 = 128u32;
pub const MAX_HOSTNAME_LEN: u32 = 128u32;
pub const MAX_IF_TYPE: u32 = 281u32;
pub const MAX_INTERFACE_NAME_LEN: u32 = 256u32;
pub const MAX_IP_STATUS: u32 = 11050u32;
pub const MAX_MIB_OFFSET: u32 = 8u32;
pub const MAX_OPT_SIZE: u32 = 40u32;
pub const MAX_SCOPE_ID_LEN: u32 = 256u32;
pub const MAX_SCOPE_NAME_LEN: u32 = 255u32;
pub const MCAST_BOUNDARY: u32 = 26u32;
pub const MCAST_GLOBAL: u32 = 24u32;
pub const MCAST_IF_ENTRY: u32 = 23u32;
pub const MCAST_MFE: u32 = 18u32;
pub const MCAST_MFE_STATS: u32 = 19u32;
pub const MCAST_MFE_STATS_EX: u32 = 35u32;
pub const MCAST_SCOPE: u32 = 27u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIBICMPINFO {
    pub icmpInStats: MIBICMPSTATS,
    pub icmpOutStats: MIBICMPSTATS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIBICMPSTATS_EX_XPSP1 {
    pub dwMsgs: u32,
    pub dwErrors: u32,
    pub rgdwTypeCount: [u32; 256],
}
impl Default for MIBICMPSTATS_EX_XPSP1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub struct MIB_ANYCASTIPADDRESS_ROW {
    pub Address: super::super::Networking::WinSock::SOCKADDR_INET,
    pub InterfaceLuid: super::Ndis::NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub ScopeId: super::super::Networking::WinSock::SCOPE_ID,
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_ANYCASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub struct MIB_ANYCASTIPADDRESS_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_ANYCASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_ANYCASTIPADDRESS_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_BEST_IF {
    pub dwDestAddr: u32,
    pub dwIfIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_BOUNDARYROW {
    pub dwGroupAddress: u32,
    pub dwGroupMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_ICMP {
    pub stats: MIBICMPINFO,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_ICMP_EX_XPSP1 {
    pub icmpInStats: MIBICMPSTATS_EX_XPSP1,
    pub icmpOutStats: MIBICMPSTATS_EX_XPSP1,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_IFNUMBER {
    pub dwValue: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MIB_IFROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_IFSTACK_ROW {
    pub HigherLayerInterfaceIndex: u32,
    pub LowerLayerInterfaceIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_IFSTACK_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_IFSTACK_ROW; 1],
}
impl Default for MIB_IFSTACK_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_IFSTATUS {
    pub dwIfIndex: u32,
    pub dwAdminStatus: u32,
    pub dwOperationalStatus: u32,
    pub bMHbeatActive: windows_core::BOOL,
    pub bMHbeatAlive: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_IFTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IFROW; 1],
}
impl Default for MIB_IFTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIB_IF_ADMIN_STATUS_DOWN: u32 = 2u32;
pub const MIB_IF_ADMIN_STATUS_TESTING: u32 = 3u32;
pub const MIB_IF_ADMIN_STATUS_UP: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_IF_ENTRY_LEVEL(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[derive(Clone, Copy)]
pub struct MIB_IF_ROW2 {
    pub InterfaceLuid: super::Ndis::NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub InterfaceGuid: windows_core::GUID,
    pub Alias: [u16; 257],
    pub Description: [u16; 257],
    pub PhysicalAddressLength: u32,
    pub PhysicalAddress: [u8; 32],
    pub PermanentPhysicalAddress: [u8; 32],
    pub Mtu: u32,
    pub Type: u32,
    pub TunnelType: super::Ndis::TUNNEL_TYPE,
    pub MediaType: super::Ndis::NDIS_MEDIUM,
    pub PhysicalMediumType: super::Ndis::NDIS_PHYSICAL_MEDIUM,
    pub AccessType: super::Ndis::NET_IF_ACCESS_TYPE,
    pub DirectionType: super::Ndis::NET_IF_DIRECTION_TYPE,
    pub InterfaceAndOperStatusFlags: MIB_IF_ROW2_0,
    pub OperStatus: super::Ndis::IF_OPER_STATUS,
    pub AdminStatus: super::Ndis::NET_IF_ADMIN_STATUS,
    pub MediaConnectState: super::Ndis::NET_IF_MEDIA_CONNECT_STATE,
    pub NetworkGuid: windows_core::GUID,
    pub ConnectionType: super::Ndis::NET_IF_CONNECTION_TYPE,
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
impl Default for MIB_IF_ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_IF_ROW2_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[derive(Clone, Copy)]
pub struct MIB_IF_TABLE2 {
    pub NumEntries: u32,
    pub Table: [MIB_IF_ROW2; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl Default for MIB_IF_TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_IF_TABLE_LEVEL(pub i32);
pub const MIB_IF_TYPE_ETHERNET: u32 = 6u32;
pub const MIB_IF_TYPE_FDDI: u32 = 15u32;
pub const MIB_IF_TYPE_LOOPBACK: u32 = 24u32;
pub const MIB_IF_TYPE_OTHER: u32 = 1u32;
pub const MIB_IF_TYPE_PPP: u32 = 23u32;
pub const MIB_IF_TYPE_SLIP: u32 = 28u32;
pub const MIB_IF_TYPE_TOKENRING: u32 = 9u32;
pub const MIB_INVALID_TEREDO_PORT_NUMBER: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_INVERTEDIFSTACK_ROW {
    pub LowerLayerInterfaceIndex: u32,
    pub HigherLayerInterfaceIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_INVERTEDIFSTACK_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_INVERTEDIFSTACK_ROW; 1],
}
impl Default for MIB_INVERTEDIFSTACK_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_IPADDRROW_W2K {
    pub dwAddr: u32,
    pub dwIndex: u32,
    pub dwMask: u32,
    pub dwBCastAddr: u32,
    pub dwReasmSize: u32,
    pub unused1: u16,
    pub unused2: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_IPADDRROW_XP {
    pub dwAddr: u32,
    pub dwIndex: u32,
    pub dwMask: u32,
    pub dwBCastAddr: u32,
    pub dwReasmSize: u32,
    pub unused1: u16,
    pub wType: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_IPADDRTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPADDRROW_XP; 1],
}
impl Default for MIB_IPADDRTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIB_IPADDR_DELETED: u32 = 64u32;
pub const MIB_IPADDR_DISCONNECTED: u32 = 8u32;
pub const MIB_IPADDR_DNS_ELIGIBLE: u32 = 256u32;
pub const MIB_IPADDR_DYNAMIC: u32 = 4u32;
pub const MIB_IPADDR_PRIMARY: u32 = 1u32;
pub const MIB_IPADDR_TRANSIENT: u32 = 128u32;
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct MIB_IPDESTROW {
    pub ForwardRow: MIB_IPFORWARDROW,
    pub dwForwardPreference: u32,
    pub dwForwardViewSet: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for MIB_IPDESTROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct MIB_IPDESTTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPDESTROW; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for MIB_IPDESTTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_IPFORWARDNUMBER {
    pub dwValue: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
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
impl Default for MIB_IPFORWARDROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union MIB_IPFORWARDROW_0 {
    pub dwForwardType: u32,
    pub ForwardType: MIB_IPFORWARD_TYPE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for MIB_IPFORWARDROW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union MIB_IPFORWARDROW_1 {
    pub dwForwardProto: u32,
    pub ForwardProto: super::super::Networking::WinSock::NL_ROUTE_PROTOCOL,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for MIB_IPFORWARDROW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct MIB_IPFORWARDTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPFORWARDROW; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for MIB_IPFORWARDTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub struct MIB_IPFORWARD_ROW2 {
    pub InterfaceLuid: super::Ndis::NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub DestinationPrefix: IP_ADDRESS_PREFIX,
    pub NextHop: super::super::Networking::WinSock::SOCKADDR_INET,
    pub SitePrefixLength: u8,
    pub ValidLifetime: u32,
    pub PreferredLifetime: u32,
    pub Metric: u32,
    pub Protocol: super::super::Networking::WinSock::NL_ROUTE_PROTOCOL,
    pub Loopback: bool,
    pub AutoconfigureAddress: bool,
    pub Publish: bool,
    pub Immortal: bool,
    pub Age: u32,
    pub Origin: super::super::Networking::WinSock::NL_ROUTE_ORIGIN,
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_IPFORWARD_ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub struct MIB_IPFORWARD_TABLE2 {
    pub NumEntries: u32,
    pub Table: [MIB_IPFORWARD_ROW2; 1],
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_IPFORWARD_TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_IPFORWARD_TYPE(pub i32);
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub struct MIB_IPINTERFACE_ROW {
    pub Family: super::super::Networking::WinSock::ADDRESS_FAMILY,
    pub InterfaceLuid: super::Ndis::NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub MaxReassemblySize: u32,
    pub InterfaceIdentifier: u64,
    pub MinRouterAdvertisementInterval: u32,
    pub MaxRouterAdvertisementInterval: u32,
    pub AdvertisingEnabled: bool,
    pub ForwardingEnabled: bool,
    pub WeakHostSend: bool,
    pub WeakHostReceive: bool,
    pub UseAutomaticMetric: bool,
    pub UseNeighborUnreachabilityDetection: bool,
    pub ManagedAddressConfigurationSupported: bool,
    pub OtherStatefulConfigurationSupported: bool,
    pub AdvertiseDefaultRoute: bool,
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
    pub Connected: bool,
    pub SupportsWakeUpPatterns: bool,
    pub SupportsNeighborDiscovery: bool,
    pub SupportsRouterDiscovery: bool,
    pub ReachableTime: u32,
    pub TransmitOffload: super::super::Networking::WinSock::NL_INTERFACE_OFFLOAD_ROD,
    pub ReceiveOffload: super::super::Networking::WinSock::NL_INTERFACE_OFFLOAD_ROD,
    pub DisableDefaultRoutes: bool,
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_IPINTERFACE_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub struct MIB_IPINTERFACE_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_IPINTERFACE_ROW; 1],
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_IPINTERFACE_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_IPMCAST_BOUNDARY {
    pub dwIfIndex: u32,
    pub dwGroupAddress: u32,
    pub dwGroupMask: u32,
    pub dwStatus: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_IPMCAST_BOUNDARY_TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPMCAST_BOUNDARY; 1],
}
impl Default for MIB_IPMCAST_BOUNDARY_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_IPMCAST_GLOBAL {
    pub dwEnable: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_IPMCAST_IF_ENTRY {
    pub dwIfIndex: u32,
    pub dwTtl: u32,
    pub dwProtocol: u32,
    pub dwRateLimit: u32,
    pub ulInMcastOctets: u32,
    pub ulOutMcastOctets: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_IPMCAST_IF_TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPMCAST_IF_ENTRY; 1],
}
impl Default for MIB_IPMCAST_IF_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MIB_IPMCAST_MFE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MIB_IPMCAST_MFE_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MIB_IPMCAST_MFE_STATS_EX_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_IPMCAST_OIF_STATS_LH {
    pub dwOutIfIndex: u32,
    pub dwNextHopAddr: u32,
    pub dwDialContext: u32,
    pub ulTtlTooLow: u32,
    pub ulFragNeeded: u32,
    pub ulOutPackets: u32,
    pub ulOutDiscards: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_IPMCAST_OIF_STATS_W2K {
    pub dwOutIfIndex: u32,
    pub dwNextHopAddr: u32,
    pub pvDialContext: *mut core::ffi::c_void,
    pub ulTtlTooLow: u32,
    pub ulFragNeeded: u32,
    pub ulOutPackets: u32,
    pub ulOutDiscards: u32,
}
impl Default for MIB_IPMCAST_OIF_STATS_W2K {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_IPMCAST_OIF_W2K {
    pub dwOutIfIndex: u32,
    pub dwNextHopAddr: u32,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
}
impl Default for MIB_IPMCAST_OIF_W2K {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_IPMCAST_OIF_XP {
    pub dwOutIfIndex: u32,
    pub dwNextHopAddr: u32,
    pub dwReserved: u32,
    pub dwReserved1: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_IPMCAST_SCOPE {
    pub dwGroupAddress: u32,
    pub dwGroupMask: u32,
    pub snNameBuffer: [u16; 256],
    pub dwStatus: u32,
}
impl Default for MIB_IPMCAST_SCOPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_IPNETROW_LH {
    pub dwIndex: u32,
    pub dwPhysAddrLen: u32,
    pub bPhysAddr: [u8; 8],
    pub dwAddr: u32,
    pub Anonymous: MIB_IPNETROW_LH_0,
}
impl Default for MIB_IPNETROW_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_IPNETROW_LH_0 {
    pub dwType: u32,
    pub Type: MIB_IPNET_TYPE,
}
impl Default for MIB_IPNETROW_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_IPNETROW_W2K {
    pub dwIndex: u32,
    pub dwPhysAddrLen: u32,
    pub bPhysAddr: [u8; 8],
    pub dwAddr: u32,
    pub dwType: u32,
}
impl Default for MIB_IPNETROW_W2K {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_IPNETTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPNETROW_LH; 1],
}
impl Default for MIB_IPNETTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub struct MIB_IPNET_ROW2 {
    pub Address: super::super::Networking::WinSock::SOCKADDR_INET,
    pub InterfaceIndex: u32,
    pub InterfaceLuid: super::Ndis::NET_LUID_LH,
    pub PhysicalAddress: [u8; 32],
    pub PhysicalAddressLength: u32,
    pub State: super::super::Networking::WinSock::NL_NEIGHBOR_STATE,
    pub Anonymous: MIB_IPNET_ROW2_0,
    pub ReachabilityTime: MIB_IPNET_ROW2_1,
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_IPNET_ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub union MIB_IPNET_ROW2_0 {
    pub Anonymous: MIB_IPNET_ROW2_0_0,
    pub Flags: u8,
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_IPNET_ROW2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_IPNET_ROW2_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub union MIB_IPNET_ROW2_1 {
    pub LastReachable: u32,
    pub LastUnreachable: u32,
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_IPNET_ROW2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub struct MIB_IPNET_TABLE2 {
    pub NumEntries: u32,
    pub Table: [MIB_IPNET_ROW2; 1],
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_IPNET_TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_IPNET_TYPE(pub i32);
pub const MIB_IPNET_TYPE_DYNAMIC: MIB_IPNET_TYPE = MIB_IPNET_TYPE(3i32);
pub const MIB_IPNET_TYPE_INVALID: MIB_IPNET_TYPE = MIB_IPNET_TYPE(2i32);
pub const MIB_IPNET_TYPE_OTHER: MIB_IPNET_TYPE = MIB_IPNET_TYPE(1i32);
pub const MIB_IPNET_TYPE_STATIC: MIB_IPNET_TYPE = MIB_IPNET_TYPE(4i32);
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub struct MIB_IPPATH_ROW {
    pub Source: super::super::Networking::WinSock::SOCKADDR_INET,
    pub Destination: super::super::Networking::WinSock::SOCKADDR_INET,
    pub InterfaceLuid: super::Ndis::NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub CurrentNextHop: super::super::Networking::WinSock::SOCKADDR_INET,
    pub PathMtu: u32,
    pub RttMean: u32,
    pub RttDeviation: u32,
    pub Anonymous: MIB_IPPATH_ROW_0,
    pub IsReachable: bool,
    pub LinkTransmitSpeed: u64,
    pub LinkReceiveSpeed: u64,
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_IPPATH_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub union MIB_IPPATH_ROW_0 {
    pub LastReachable: u32,
    pub LastUnreachable: u32,
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_IPPATH_ROW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub struct MIB_IPPATH_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_IPPATH_ROW; 1],
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_IPPATH_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIB_IPROUTE_METRIC_UNUSED: u32 = 4294967295u32;
pub const MIB_IPROUTE_TYPE_DIRECT: MIB_IPFORWARD_TYPE = MIB_IPFORWARD_TYPE(3i32);
pub const MIB_IPROUTE_TYPE_INDIRECT: MIB_IPFORWARD_TYPE = MIB_IPFORWARD_TYPE(4i32);
pub const MIB_IPROUTE_TYPE_INVALID: MIB_IPFORWARD_TYPE = MIB_IPFORWARD_TYPE(2i32);
pub const MIB_IPROUTE_TYPE_OTHER: MIB_IPFORWARD_TYPE = MIB_IPFORWARD_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_IPSTATS_FORWARDING(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for MIB_IPSTATS_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_IPSTATS_LH_0 {
    pub dwForwarding: u32,
    pub Forwarding: MIB_IPSTATS_FORWARDING,
}
impl Default for MIB_IPSTATS_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const MIB_IP_FORWARDING: MIB_IPSTATS_FORWARDING = MIB_IPSTATS_FORWARDING(1i32);
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    pub InboundBandwidthInformation: super::super::Networking::WinSock::NL_BANDWIDTH_INFORMATION,
    pub OutboundBandwidthInformation: super::super::Networking::WinSock::NL_BANDWIDTH_INFORMATION,
}
pub const MIB_IP_NOT_FORWARDING: MIB_IPSTATS_FORWARDING = MIB_IPSTATS_FORWARDING(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_MCAST_LIMIT_ROW {
    pub dwTtl: u32,
    pub dwRateLimit: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_MFE_STATS_TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPMCAST_MFE_STATS; 1],
}
impl Default for MIB_MFE_STATS_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_MFE_STATS_TABLE_EX_XP {
    pub dwNumEntries: u32,
    pub table: [*mut MIB_IPMCAST_MFE_STATS_EX_XP; 1],
}
impl Default for MIB_MFE_STATS_TABLE_EX_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_MFE_TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPMCAST_MFE; 1],
}
impl Default for MIB_MFE_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub struct MIB_MULTICASTIPADDRESS_ROW {
    pub Address: super::super::Networking::WinSock::SOCKADDR_INET,
    pub InterfaceIndex: u32,
    pub InterfaceLuid: super::Ndis::NET_LUID_LH,
    pub ScopeId: super::super::Networking::WinSock::SCOPE_ID,
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_MULTICASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub struct MIB_MULTICASTIPADDRESS_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_MULTICASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_MULTICASTIPADDRESS_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_NOTIFICATION_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_OPAQUE_INFO {
    pub dwId: u32,
    pub Anonymous: MIB_OPAQUE_INFO_0,
}
impl Default for MIB_OPAQUE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_OPAQUE_INFO_0 {
    pub ullAlign: u64,
    pub rgbyData: [u8; 1],
}
impl Default for MIB_OPAQUE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_OPAQUE_QUERY {
    pub dwVarId: u32,
    pub rgdwVarIndex: [u32; 1],
}
impl Default for MIB_OPAQUE_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_PROXYARP {
    pub dwAddress: u32,
    pub dwMask: u32,
    pub dwIfIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_ROUTESTATE {
    pub bRoutesSetToStack: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
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
impl Default for MIB_TCP6ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
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
impl Default for MIB_TCP6ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MIB_TCP6ROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MIB_TCP6ROW_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct MIB_TCP6TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_TCP6ROW; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for MIB_TCP6TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct MIB_TCP6TABLE2 {
    pub dwNumEntries: u32,
    pub table: [MIB_TCP6ROW2; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for MIB_TCP6TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_TCP6TABLE_OWNER_MODULE {
    pub dwNumEntries: u32,
    pub table: [MIB_TCP6ROW_OWNER_MODULE; 1],
}
impl Default for MIB_TCP6TABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_TCP6TABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_TCP6ROW_OWNER_PID; 1],
}
impl Default for MIB_TCP6TABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_TCPROW2 {
    pub dwState: u32,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
    pub dwOwningPid: u32,
    pub dwOffloadState: TCP_CONNECTION_OFFLOAD_STATE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_TCPROW_LH {
    pub Anonymous: MIB_TCPROW_LH_0,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
}
impl Default for MIB_TCPROW_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_TCPROW_LH_0 {
    pub dwState: u32,
    pub State: MIB_TCP_STATE,
}
impl Default for MIB_TCPROW_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for MIB_TCPROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_TCPROW_OWNER_PID {
    pub dwState: u32,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
    pub dwOwningPid: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_TCPROW_W2K {
    pub dwState: u32,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for MIB_TCPSTATS_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_TCPSTATS_LH_0 {
    pub dwRtoAlgorithm: u32,
    pub RtoAlgorithm: TCP_RTO_ALGORITHM,
}
impl Default for MIB_TCPSTATS_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_TCPTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW_LH; 1],
}
impl Default for MIB_TCPTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_TCPTABLE2 {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW2; 1],
}
impl Default for MIB_TCPTABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_TCPTABLE_OWNER_MODULE {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW_OWNER_MODULE; 1],
}
impl Default for MIB_TCPTABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_TCPTABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW_OWNER_PID; 1],
}
impl Default for MIB_TCPTABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIB_TCP_RTO_CONSTANT: TCP_RTO_ALGORITHM = TCP_RTO_ALGORITHM(2i32);
pub const MIB_TCP_RTO_OTHER: TCP_RTO_ALGORITHM = TCP_RTO_ALGORITHM(1i32);
pub const MIB_TCP_RTO_RSRE: TCP_RTO_ALGORITHM = TCP_RTO_ALGORITHM(3i32);
pub const MIB_TCP_RTO_VANJ: TCP_RTO_ALGORITHM = TCP_RTO_ALGORITHM(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_TCP_STATE(pub i32);
pub const MIB_TCP_STATE_CLOSED: MIB_TCP_STATE = MIB_TCP_STATE(1i32);
pub const MIB_TCP_STATE_CLOSE_WAIT: MIB_TCP_STATE = MIB_TCP_STATE(8i32);
pub const MIB_TCP_STATE_CLOSING: MIB_TCP_STATE = MIB_TCP_STATE(9i32);
pub const MIB_TCP_STATE_DELETE_TCB: MIB_TCP_STATE = MIB_TCP_STATE(12i32);
pub const MIB_TCP_STATE_ESTAB: MIB_TCP_STATE = MIB_TCP_STATE(5i32);
pub const MIB_TCP_STATE_FIN_WAIT1: MIB_TCP_STATE = MIB_TCP_STATE(6i32);
pub const MIB_TCP_STATE_FIN_WAIT2: MIB_TCP_STATE = MIB_TCP_STATE(7i32);
pub const MIB_TCP_STATE_LAST_ACK: MIB_TCP_STATE = MIB_TCP_STATE(10i32);
pub const MIB_TCP_STATE_LISTEN: MIB_TCP_STATE = MIB_TCP_STATE(2i32);
pub const MIB_TCP_STATE_RESERVED: MIB_TCP_STATE = MIB_TCP_STATE(100i32);
pub const MIB_TCP_STATE_SYN_RCVD: MIB_TCP_STATE = MIB_TCP_STATE(4i32);
pub const MIB_TCP_STATE_SYN_SENT: MIB_TCP_STATE = MIB_TCP_STATE(3i32);
pub const MIB_TCP_STATE_TIME_WAIT: MIB_TCP_STATE = MIB_TCP_STATE(11i32);
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct MIB_UDP6ROW {
    pub dwLocalAddr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for MIB_UDP6ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for MIB_UDP6ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_UDP6ROW2_0 {
    pub Anonymous: MIB_UDP6ROW2_0_0,
    pub dwFlags: i32,
}
impl Default for MIB_UDP6ROW2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_UDP6ROW2_0_0 {
    pub _bitfield: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_UDP6ROW_OWNER_MODULE {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub Anonymous: MIB_UDP6ROW_OWNER_MODULE_0,
    pub OwningModuleInfo: [u64; 16],
}
impl Default for MIB_UDP6ROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_UDP6ROW_OWNER_MODULE_0 {
    pub Anonymous: MIB_UDP6ROW_OWNER_MODULE_0_0,
    pub dwFlags: i32,
}
impl Default for MIB_UDP6ROW_OWNER_MODULE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_UDP6ROW_OWNER_MODULE_0_0 {
    pub _bitfield: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_UDP6ROW_OWNER_PID {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
}
impl Default for MIB_UDP6ROW_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct MIB_UDP6TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_UDP6ROW; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for MIB_UDP6TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_UDP6TABLE2 {
    pub dwNumEntries: u32,
    pub table: [MIB_UDP6ROW2; 1],
}
impl Default for MIB_UDP6TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_UDP6TABLE_OWNER_MODULE {
    pub dwNumEntries: u32,
    pub table: [MIB_UDP6ROW_OWNER_MODULE; 1],
}
impl Default for MIB_UDP6TABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_UDP6TABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_UDP6ROW_OWNER_PID; 1],
}
impl Default for MIB_UDP6TABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_UDPROW {
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for MIB_UDPROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_UDPROW2_0 {
    pub Anonymous: MIB_UDPROW2_0_0,
    pub dwFlags: i32,
}
impl Default for MIB_UDPROW2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_UDPROW2_0_0 {
    pub _bitfield: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_UDPROW_OWNER_MODULE {
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub Anonymous: MIB_UDPROW_OWNER_MODULE_0,
    pub OwningModuleInfo: [u64; 16],
}
impl Default for MIB_UDPROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_UDPROW_OWNER_MODULE_0 {
    pub Anonymous: MIB_UDPROW_OWNER_MODULE_0_0,
    pub dwFlags: i32,
}
impl Default for MIB_UDPROW_OWNER_MODULE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_UDPROW_OWNER_MODULE_0_0 {
    pub _bitfield: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_UDPROW_OWNER_PID {
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwOwningPid: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_UDPSTATS {
    pub dwInDatagrams: u32,
    pub dwNoPorts: u32,
    pub dwInErrors: u32,
    pub dwOutDatagrams: u32,
    pub dwNumAddrs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIB_UDPSTATS2 {
    pub dw64InDatagrams: u64,
    pub dwNoPorts: u32,
    pub dwInErrors: u32,
    pub dw64OutDatagrams: u64,
    pub dwNumAddrs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_UDPTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW; 1],
}
impl Default for MIB_UDPTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_UDPTABLE2 {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW2; 1],
}
impl Default for MIB_UDPTABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_UDPTABLE_OWNER_MODULE {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW_OWNER_MODULE; 1],
}
impl Default for MIB_UDPTABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIB_UDPTABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_UDPROW_OWNER_PID; 1],
}
impl Default for MIB_UDPTABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub struct MIB_UNICASTIPADDRESS_ROW {
    pub Address: super::super::Networking::WinSock::SOCKADDR_INET,
    pub InterfaceLuid: super::Ndis::NET_LUID_LH,
    pub InterfaceIndex: u32,
    pub PrefixOrigin: super::super::Networking::WinSock::NL_PREFIX_ORIGIN,
    pub SuffixOrigin: super::super::Networking::WinSock::NL_SUFFIX_ORIGIN,
    pub ValidLifetime: u32,
    pub PreferredLifetime: u32,
    pub OnLinkPrefixLength: u8,
    pub SkipAsSource: bool,
    pub DadState: super::super::Networking::WinSock::NL_DAD_STATE,
    pub ScopeId: super::super::Networking::WinSock::SCOPE_ID,
    pub CreationTimeStamp: i64,
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_UNICASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
#[derive(Clone, Copy)]
pub struct MIB_UNICASTIPADDRESS_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_UNICASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl Default for MIB_UNICASTIPADDRESS_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIB_USE_CURRENT_FORWARDING: u32 = 4294967295u32;
pub const MIB_USE_CURRENT_TTL: u32 = 4294967295u32;
pub const MIN_IF_TYPE: u32 = 1u32;
pub const MIXED_NODETYPE: u32 = 4u32;
pub const MibAddInstance: MIB_NOTIFICATION_TYPE = MIB_NOTIFICATION_TYPE(1i32);
pub const MibDeleteInstance: MIB_NOTIFICATION_TYPE = MIB_NOTIFICATION_TYPE(2i32);
pub const MibIfEntryNormal: MIB_IF_ENTRY_LEVEL = MIB_IF_ENTRY_LEVEL(0i32);
pub const MibIfEntryNormalWithoutStatistics: MIB_IF_ENTRY_LEVEL = MIB_IF_ENTRY_LEVEL(2i32);
pub const MibIfTableNormal: MIB_IF_TABLE_LEVEL = MIB_IF_TABLE_LEVEL(0i32);
pub const MibIfTableNormalWithoutStatistics: MIB_IF_TABLE_LEVEL = MIB_IF_TABLE_LEVEL(2i32);
pub const MibIfTableRaw: MIB_IF_TABLE_LEVEL = MIB_IF_TABLE_LEVEL(1i32);
pub const MibInitialNotification: MIB_NOTIFICATION_TYPE = MIB_NOTIFICATION_TYPE(3i32);
pub const MibParameterNotification: MIB_NOTIFICATION_TYPE = MIB_NOTIFICATION_TYPE(0i32);
pub const ND_NEIGHBOR_ADVERT: ICMP6_TYPE = ICMP6_TYPE(136i32);
pub const ND_NEIGHBOR_SOLICIT: ICMP6_TYPE = ICMP6_TYPE(135i32);
pub const ND_REDIRECT: ICMP6_TYPE = ICMP6_TYPE(137i32);
pub const ND_ROUTER_ADVERT: ICMP6_TYPE = ICMP6_TYPE(134i32);
pub const ND_ROUTER_SOLICIT: ICMP6_TYPE = ICMP6_TYPE(133i32);
pub const NET_ADDRESS_DNS_NAME: NET_ADDRESS_FORMAT = NET_ADDRESS_FORMAT(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_ADDRESS_FORMAT(pub i32);
pub const NET_ADDRESS_FORMAT_UNSPECIFIED: NET_ADDRESS_FORMAT = NET_ADDRESS_FORMAT(0i32);
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct NET_ADDRESS_INFO {
    pub Format: NET_ADDRESS_FORMAT,
    pub Anonymous: NET_ADDRESS_INFO_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for NET_ADDRESS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union NET_ADDRESS_INFO_0 {
    pub NamedAddress: NET_ADDRESS_INFO_0_0,
    pub Ipv4Address: super::super::Networking::WinSock::SOCKADDR_IN,
    pub Ipv6Address: super::super::Networking::WinSock::SOCKADDR_IN6,
    pub IpAddress: super::super::Networking::WinSock::SOCKADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for NET_ADDRESS_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NET_ADDRESS_INFO_0_0 {
    pub Address: [u16; 256],
    pub Port: [u16; 6],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for NET_ADDRESS_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NET_ADDRESS_IPV4: NET_ADDRESS_FORMAT = NET_ADDRESS_FORMAT(2i32);
pub const NET_ADDRESS_IPV6: NET_ADDRESS_FORMAT = NET_ADDRESS_FORMAT(3i32);
pub const NET_STRING_IPV4_ADDRESS: u32 = 1u32;
pub const NET_STRING_IPV4_NETWORK: u32 = 4u32;
pub const NET_STRING_IPV4_SERVICE: u32 = 2u32;
pub const NET_STRING_IPV6_ADDRESS: u32 = 8u32;
pub const NET_STRING_IPV6_ADDRESS_NO_SCOPE: u32 = 16u32;
pub const NET_STRING_IPV6_NETWORK: u32 = 128u32;
pub const NET_STRING_IPV6_SERVICE: u32 = 32u32;
pub const NET_STRING_IPV6_SERVICE_NO_SCOPE: u32 = 64u32;
pub const NET_STRING_NAMED_ADDRESS: u32 = 256u32;
pub const NET_STRING_NAMED_SERVICE: u32 = 512u32;
pub const NUMBER_OF_EXPORTED_VARIABLES: u32 = 39u32;
pub const PEER_TO_PEER_NODETYPE: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PFADDRESSTYPE(pub i32);
pub const PFERROR_BUFFER_TOO_SMALL: u32 = 23002u32;
pub const PFERROR_NO_FILTERS_GIVEN: u32 = 23001u32;
pub const PFERROR_NO_PF_INTERFACE: u32 = 23000u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PFFORWARD_ACTION(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PFFRAMETYPE(pub i32);
pub const PFFT_FILTER: PFFRAMETYPE = PFFRAMETYPE(1i32);
pub const PFFT_FRAG: PFFRAMETYPE = PFFRAMETYPE(2i32);
pub const PFFT_SPOOF: PFFRAMETYPE = PFFRAMETYPE(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for PFLOGFRAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PF_ACTION_DROP: PFFORWARD_ACTION = PFFORWARD_ACTION(1i32);
pub const PF_ACTION_FORWARD: PFFORWARD_ACTION = PFFORWARD_ACTION(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for PF_FILTER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PF_FILTER_STATS {
    pub dwNumPacketsFiltered: u32,
    pub info: PF_FILTER_DESCRIPTOR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PF_INTERFACE_STATS {
    pub pvDriverContext: *mut core::ffi::c_void,
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
impl Default for PF_INTERFACE_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PF_IPV4: PFADDRESSTYPE = PFADDRESSTYPE(0i32);
pub const PF_IPV6: PFADDRESSTYPE = PFADDRESSTYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PF_LATEBIND_INFO {
    pub SrcAddr: *mut u8,
    pub DstAddr: *mut u8,
    pub Mask: *mut u8,
}
impl Default for PF_LATEBIND_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void)>;
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
pub type PIPFORWARD_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, row: *const MIB_IPFORWARD_ROW2, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
pub type PIPINTERFACE_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, row: *const MIB_IPINTERFACE_ROW, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[cfg(feature = "Win32_Networking_WinSock")]
pub type PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, connectivityhint: super::super::Networking::WinSock::NL_NETWORK_CONNECTIVITY_HINT)>;
pub const PROXY_ARP: u32 = 22u32;
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
pub type PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, addresstable: *const MIB_UNICASTIPADDRESS_TABLE)>;
pub type PTEREDO_PORT_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, port: u16, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[cfg(all(feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
pub type PUNICAST_IPADDRESS_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, row: *const MIB_UNICASTIPADDRESS_ROW, notificationtype: MIB_NOTIFICATION_TYPE)>;
pub const ROUTE_LONGER: u32 = 32u32;
pub const ROUTE_MATCHING: u32 = 31u32;
pub const ROUTE_SHORTER: u32 = 33u32;
pub const ROUTE_STATE: u32 = 34u32;
pub const TCP6_STATS: u32 = 38u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCPIP_OWNER_MODULE_BASIC_INFO {
    pub pModuleName: windows_core::PWSTR,
    pub pModulePath: windows_core::PWSTR,
}
pub const TCPIP_OWNER_MODULE_INFO_BASIC: TCPIP_OWNER_MODULE_INFO_CLASS = TCPIP_OWNER_MODULE_INFO_CLASS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCPIP_OWNER_MODULE_INFO_CLASS(pub i32);
pub const TCPIP_OWNING_MODULE_SIZE: u32 = 16u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_BOOLEAN_OPTIONAL(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_CONNECTION_OFFLOAD_STATE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCP_ESTATS_BANDWIDTH_ROD_v0 {
    pub OutboundBandwidth: u64,
    pub InboundBandwidth: u64,
    pub OutboundInstability: u64,
    pub InboundInstability: u64,
    pub OutboundBandwidthPeaked: bool,
    pub InboundBandwidthPeaked: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCP_ESTATS_BANDWIDTH_RW_v0 {
    pub EnableCollectionOutbound: TCP_BOOLEAN_OPTIONAL,
    pub EnableCollectionInbound: TCP_BOOLEAN_OPTIONAL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCP_ESTATS_DATA_RW_v0 {
    pub EnableCollection: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCP_ESTATS_FINE_RTT_ROD_v0 {
    pub RttVar: u32,
    pub MaxRtt: u32,
    pub MinRtt: u32,
    pub SumRtt: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCP_ESTATS_FINE_RTT_RW_v0 {
    pub EnableCollection: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCP_ESTATS_OBS_REC_ROD_v0 {
    pub CurRwinRcvd: u32,
    pub MaxRwinRcvd: u32,
    pub MinRwinRcvd: u32,
    pub WinScaleRcvd: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCP_ESTATS_OBS_REC_RW_v0 {
    pub EnableCollection: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCP_ESTATS_PATH_RW_v0 {
    pub EnableCollection: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCP_ESTATS_REC_RW_v0 {
    pub EnableCollection: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCP_ESTATS_SEND_BUFF_ROD_v0 {
    pub CurRetxQueue: usize,
    pub MaxRetxQueue: usize,
    pub CurAppWQueue: usize,
    pub MaxAppWQueue: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCP_ESTATS_SEND_BUFF_RW_v0 {
    pub EnableCollection: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCP_ESTATS_SND_CONG_ROS_v0 {
    pub LimCwnd: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCP_ESTATS_SND_CONG_RW_v0 {
    pub EnableCollection: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCP_ESTATS_SYN_OPTS_ROS_v0 {
    pub ActiveOpen: bool,
    pub MssRcvd: u32,
    pub MssSent: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TCP_RESERVE_PORT_RANGE {
    pub UpperRange: u16,
    pub LowerRange: u16,
}
pub const TCP_ROW: u32 = 14u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_RTO_ALGORITHM(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_SOFT_ERROR(pub i32);
pub const TCP_STATS: u32 = 12u32;
pub const TCP_TABLE: u32 = 13u32;
pub const TCP_TABLE_BASIC_ALL: TCP_TABLE_CLASS = TCP_TABLE_CLASS(2i32);
pub const TCP_TABLE_BASIC_CONNECTIONS: TCP_TABLE_CLASS = TCP_TABLE_CLASS(1i32);
pub const TCP_TABLE_BASIC_LISTENER: TCP_TABLE_CLASS = TCP_TABLE_CLASS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_TABLE_CLASS(pub i32);
pub const TCP_TABLE_OWNER_MODULE_ALL: TCP_TABLE_CLASS = TCP_TABLE_CLASS(8i32);
pub const TCP_TABLE_OWNER_MODULE_CONNECTIONS: TCP_TABLE_CLASS = TCP_TABLE_CLASS(7i32);
pub const TCP_TABLE_OWNER_MODULE_LISTENER: TCP_TABLE_CLASS = TCP_TABLE_CLASS(6i32);
pub const TCP_TABLE_OWNER_PID_ALL: TCP_TABLE_CLASS = TCP_TABLE_CLASS(5i32);
pub const TCP_TABLE_OWNER_PID_CONNECTIONS: TCP_TABLE_CLASS = TCP_TABLE_CLASS(4i32);
pub const TCP_TABLE_OWNER_PID_LISTENER: TCP_TABLE_CLASS = TCP_TABLE_CLASS(3i32);
pub const TcpBoolOptDisabled: TCP_BOOLEAN_OPTIONAL = TCP_BOOLEAN_OPTIONAL(0i32);
pub const TcpBoolOptEnabled: TCP_BOOLEAN_OPTIONAL = TCP_BOOLEAN_OPTIONAL(1i32);
pub const TcpBoolOptUnchanged: TCP_BOOLEAN_OPTIONAL = TCP_BOOLEAN_OPTIONAL(-1i32);
pub const TcpConnectionEstatsBandwidth: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(7i32);
pub const TcpConnectionEstatsData: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(1i32);
pub const TcpConnectionEstatsFineRtt: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(8i32);
pub const TcpConnectionEstatsMaximum: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(9i32);
pub const TcpConnectionEstatsObsRec: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(6i32);
pub const TcpConnectionEstatsPath: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(3i32);
pub const TcpConnectionEstatsRec: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(5i32);
pub const TcpConnectionEstatsSendBuff: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(4i32);
pub const TcpConnectionEstatsSndCong: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(2i32);
pub const TcpConnectionEstatsSynOpts: TCP_ESTATS_TYPE = TCP_ESTATS_TYPE(0i32);
pub const TcpConnectionOffloadStateInHost: TCP_CONNECTION_OFFLOAD_STATE = TCP_CONNECTION_OFFLOAD_STATE(0i32);
pub const TcpConnectionOffloadStateMax: TCP_CONNECTION_OFFLOAD_STATE = TCP_CONNECTION_OFFLOAD_STATE(4i32);
pub const TcpConnectionOffloadStateOffloaded: TCP_CONNECTION_OFFLOAD_STATE = TCP_CONNECTION_OFFLOAD_STATE(2i32);
pub const TcpConnectionOffloadStateOffloading: TCP_CONNECTION_OFFLOAD_STATE = TCP_CONNECTION_OFFLOAD_STATE(1i32);
pub const TcpConnectionOffloadStateUploading: TCP_CONNECTION_OFFLOAD_STATE = TCP_CONNECTION_OFFLOAD_STATE(3i32);
pub const TcpErrorAboveAckWindow: TCP_SOFT_ERROR = TCP_SOFT_ERROR(4i32);
pub const TcpErrorAboveDataWindow: TCP_SOFT_ERROR = TCP_SOFT_ERROR(2i32);
pub const TcpErrorAboveTsWindow: TCP_SOFT_ERROR = TCP_SOFT_ERROR(6i32);
pub const TcpErrorBelowAckWindow: TCP_SOFT_ERROR = TCP_SOFT_ERROR(3i32);
pub const TcpErrorBelowDataWindow: TCP_SOFT_ERROR = TCP_SOFT_ERROR(1i32);
pub const TcpErrorBelowTsWindow: TCP_SOFT_ERROR = TCP_SOFT_ERROR(5i32);
pub const TcpErrorDataChecksumError: TCP_SOFT_ERROR = TCP_SOFT_ERROR(7i32);
pub const TcpErrorDataLengthError: TCP_SOFT_ERROR = TCP_SOFT_ERROR(8i32);
pub const TcpErrorMaxSoftError: TCP_SOFT_ERROR = TCP_SOFT_ERROR(9i32);
pub const TcpErrorNone: TCP_SOFT_ERROR = TCP_SOFT_ERROR(0i32);
pub const TcpRtoAlgorithmConstant: TCP_RTO_ALGORITHM = TCP_RTO_ALGORITHM(2i32);
pub const TcpRtoAlgorithmOther: TCP_RTO_ALGORITHM = TCP_RTO_ALGORITHM(1i32);
pub const TcpRtoAlgorithmRsre: TCP_RTO_ALGORITHM = TCP_RTO_ALGORITHM(3i32);
pub const TcpRtoAlgorithmVanj: TCP_RTO_ALGORITHM = TCP_RTO_ALGORITHM(4i32);
pub const UDP6_STATS: u32 = 37u32;
pub const UDP_ROW: u32 = 17u32;
pub const UDP_STATS: u32 = 15u32;
pub const UDP_TABLE: u32 = 16u32;
pub const UDP_TABLE_BASIC: UDP_TABLE_CLASS = UDP_TABLE_CLASS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UDP_TABLE_CLASS(pub i32);
pub const UDP_TABLE_OWNER_MODULE: UDP_TABLE_CLASS = UDP_TABLE_CLASS(2i32);
pub const UDP_TABLE_OWNER_PID: UDP_TABLE_CLASS = UDP_TABLE_CLASS(1i32);
