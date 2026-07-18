#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CancelMibChangeNotify2(notificationhandle: super::HANDLE) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn CancelMibChangeNotify2(notificationhandle : super::HANDLE) -> super::NTSTATUS);
    unsafe { CancelMibChangeNotify2(notificationhandle) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn ConvertCompartmentGuidToId(compartmentguid: *const windows_core::GUID, compartmentid: *mut u32) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn ConvertCompartmentGuidToId(compartmentguid : *const windows_core::GUID, compartmentid : *mut u32) -> super::NTSTATUS);
    unsafe { ConvertCompartmentGuidToId(compartmentguid, compartmentid as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn ConvertCompartmentIdToGuid(compartmentid: super::NET_IF_COMPARTMENT_ID, compartmentguid: *mut windows_core::GUID) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn ConvertCompartmentIdToGuid(compartmentid : super::NET_IF_COMPARTMENT_ID, compartmentguid : *mut windows_core::GUID) -> super::NTSTATUS);
    unsafe { ConvertCompartmentIdToGuid(compartmentid, compartmentguid as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn ConvertInterfaceAliasToLuid<P0>(interfacealias: P0, interfaceluid: *mut super::NET_LUID) -> super::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("iphlpapi.dll" "system" fn ConvertInterfaceAliasToLuid(interfacealias : windows_core::PCWSTR, interfaceluid : *mut super::NET_LUID) -> super::NTSTATUS);
    unsafe { ConvertInterfaceAliasToLuid(interfacealias.param().abi(), interfaceluid as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn ConvertInterfaceGuidToLuid(interfaceguid: *const windows_core::GUID, interfaceluid: *mut super::NET_LUID) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn ConvertInterfaceGuidToLuid(interfaceguid : *const windows_core::GUID, interfaceluid : *mut super::NET_LUID) -> super::NTSTATUS);
    unsafe { ConvertInterfaceGuidToLuid(interfaceguid, interfaceluid as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn ConvertInterfaceIndexToLuid(interfaceindex: super::NET_IFINDEX, interfaceluid: *mut super::NET_LUID) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn ConvertInterfaceIndexToLuid(interfaceindex : super::NET_IFINDEX, interfaceluid : *mut super::NET_LUID) -> super::NTSTATUS);
    unsafe { ConvertInterfaceIndexToLuid(interfaceindex, interfaceluid as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn ConvertInterfaceLuidToAlias(interfaceluid: *const super::NET_LUID, interfacealias: &mut [u16]) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToAlias(interfaceluid : *const super::NET_LUID, interfacealias : windows_core::PWSTR, length : usize) -> super::NTSTATUS);
    unsafe { ConvertInterfaceLuidToAlias(interfaceluid, core::mem::transmute(interfacealias.as_mut_ptr()), interfacealias.len().try_into().unwrap()) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn ConvertInterfaceLuidToGuid(interfaceluid: *const super::NET_LUID, interfaceguid: *mut windows_core::GUID) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToGuid(interfaceluid : *const super::NET_LUID, interfaceguid : *mut windows_core::GUID) -> super::NTSTATUS);
    unsafe { ConvertInterfaceLuidToGuid(interfaceluid, interfaceguid as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn ConvertInterfaceLuidToIndex(interfaceluid: *const super::NET_LUID, interfaceindex: *mut u32) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToIndex(interfaceluid : *const super::NET_LUID, interfaceindex : *mut u32) -> super::NTSTATUS);
    unsafe { ConvertInterfaceLuidToIndex(interfaceluid, interfaceindex as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn ConvertInterfaceLuidToNameA(interfaceluid: *const super::NET_LUID, interfacename: &mut [u8]) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToNameA(interfaceluid : *const super::NET_LUID, interfacename : windows_core::PSTR, length : usize) -> super::NTSTATUS);
    unsafe { ConvertInterfaceLuidToNameA(interfaceluid, core::mem::transmute(interfacename.as_mut_ptr()), interfacename.len().try_into().unwrap()) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn ConvertInterfaceLuidToNameW(interfaceluid: *const super::NET_LUID, interfacename: &mut [u16]) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToNameW(interfaceluid : *const super::NET_LUID, interfacename : windows_core::PWSTR, length : usize) -> super::NTSTATUS);
    unsafe { ConvertInterfaceLuidToNameW(interfaceluid, core::mem::transmute(interfacename.as_mut_ptr()), interfacename.len().try_into().unwrap()) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn ConvertInterfaceNameToLuidA<P0>(interfacename: P0, interfaceluid: *mut super::NET_LUID) -> super::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("iphlpapi.dll" "system" fn ConvertInterfaceNameToLuidA(interfacename : windows_core::PCSTR, interfaceluid : *mut super::NET_LUID) -> super::NTSTATUS);
    unsafe { ConvertInterfaceNameToLuidA(interfacename.param().abi(), interfaceluid as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn ConvertInterfaceNameToLuidW<P0>(interfacename: P0, interfaceluid: *mut super::NET_LUID) -> super::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("iphlpapi.dll" "system" fn ConvertInterfaceNameToLuidW(interfacename : windows_core::PCWSTR, interfaceluid : *mut super::NET_LUID) -> super::NTSTATUS);
    unsafe { ConvertInterfaceNameToLuidW(interfacename.param().abi(), interfaceluid as _) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn ConvertIpv4MaskToLength(mask: u32, masklength: *mut u8) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn ConvertIpv4MaskToLength(mask : u32, masklength : *mut u8) -> super::NTSTATUS);
    unsafe { ConvertIpv4MaskToLength(mask, masklength as _) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn ConvertLengthToIpv4Mask(masklength: u32, mask: *mut u32) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn ConvertLengthToIpv4Mask(masklength : u32, mask : *mut u32) -> super::NTSTATUS);
    unsafe { ConvertLengthToIpv4Mask(masklength, mask as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[inline]
pub unsafe fn CreateAnycastIpAddressEntry(row: *const MIB_ANYCASTIPADDRESS_ROW) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn CreateAnycastIpAddressEntry(row : *const MIB_ANYCASTIPADDRESS_ROW) -> super::NTSTATUS);
    unsafe { CreateAnycastIpAddressEntry(row) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "ws2"))]
#[inline]
pub unsafe fn CreateFlVirtualInterface(row: *const MIB_FL_VIRTUAL_INTERFACE_ROW) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn CreateFlVirtualInterface(row : *const MIB_FL_VIRTUAL_INTERFACE_ROW) -> super::NTSTATUS);
    unsafe { CreateFlVirtualInterface(row) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn CreateIpForwardEntry2(row: *const MIB_IPFORWARD_ROW2) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn CreateIpForwardEntry2(row : *const MIB_IPFORWARD_ROW2) -> super::NTSTATUS);
    unsafe { CreateIpForwardEntry2(row) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn CreateIpNetEntry2(row: *const MIB_IPNET_ROW2) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn CreateIpNetEntry2(row : *const MIB_IPNET_ROW2) -> super::NTSTATUS);
    unsafe { CreateIpNetEntry2(row) }
}
#[cfg(all(feature = "bcrypt", feature = "in6addr", feature = "ws2"))]
#[inline]
pub unsafe fn CreateSortedAddressPairs(sourceaddresslist: Option<*const super::SOCKADDR_IN6_LH>, sourceaddresscount: u32, destinationaddresslist: *const super::SOCKADDR_IN6_LH, destinationaddresscount: u32, addresssortoptions: u32, sortedaddresspairlist: *mut super::PSOCKADDR_IN6_PAIR, sortedaddresspaircount: *mut u32) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn CreateSortedAddressPairs(sourceaddresslist : *const super::SOCKADDR_IN6_LH, sourceaddresscount : u32, destinationaddresslist : *const super::SOCKADDR_IN6_LH, destinationaddresscount : u32, addresssortoptions : u32, sortedaddresspairlist : *mut super::PSOCKADDR_IN6_PAIR, sortedaddresspaircount : *mut u32) -> super::NTSTATUS);
    unsafe { CreateSortedAddressPairs(sourceaddresslist.unwrap_or(core::mem::zeroed()) as _, sourceaddresscount, destinationaddresslist, destinationaddresscount, addresssortoptions, sortedaddresspairlist as _, sortedaddresspaircount as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn CreateUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn CreateUnicastIpAddressEntry(row : *const MIB_UNICASTIPADDRESS_ROW) -> super::NTSTATUS);
    unsafe { CreateUnicastIpAddressEntry(row) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[inline]
pub unsafe fn DeleteAnycastIpAddressEntry(row: *const MIB_ANYCASTIPADDRESS_ROW) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn DeleteAnycastIpAddressEntry(row : *const MIB_ANYCASTIPADDRESS_ROW) -> super::NTSTATUS);
    unsafe { DeleteAnycastIpAddressEntry(row) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "ws2"))]
#[inline]
pub unsafe fn DeleteFlVirtualInterface(row: *const MIB_FL_VIRTUAL_INTERFACE_ROW) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn DeleteFlVirtualInterface(row : *const MIB_FL_VIRTUAL_INTERFACE_ROW) -> super::NTSTATUS);
    unsafe { DeleteFlVirtualInterface(row) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn DeleteIpForwardEntry2(row: *const MIB_IPFORWARD_ROW2) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn DeleteIpForwardEntry2(row : *const MIB_IPFORWARD_ROW2) -> super::NTSTATUS);
    unsafe { DeleteIpForwardEntry2(row) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn DeleteIpNetEntry2(row: *const MIB_IPNET_ROW2) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn DeleteIpNetEntry2(row : *const MIB_IPNET_ROW2) -> super::NTSTATUS);
    unsafe { DeleteIpNetEntry2(row) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn DeleteUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn DeleteUnicastIpAddressEntry(row : *const MIB_UNICASTIPADDRESS_ROW) -> super::NTSTATUS);
    unsafe { DeleteUnicastIpAddressEntry(row) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "ws2"))]
#[inline]
pub unsafe fn FlushIpNetTable2(family: super::ADDRESS_FAMILY, interfaceindex: super::NET_IFINDEX) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn FlushIpNetTable2(family : super::ADDRESS_FAMILY, interfaceindex : super::NET_IFINDEX) -> super::NTSTATUS);
    unsafe { FlushIpNetTable2(family, interfaceindex) }
}
#[cfg(all(feature = "bcrypt", feature = "ws2"))]
#[inline]
pub unsafe fn FlushIpPathTable(family: super::ADDRESS_FAMILY) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn FlushIpPathTable(family : super::ADDRESS_FAMILY) -> super::NTSTATUS);
    unsafe { FlushIpPathTable(family) }
}
#[inline]
pub unsafe fn FreeDnsSettings(settings: *mut DNS_SETTINGS) {
    windows_core::link!("iphlpapi.dll" "system" fn FreeDnsSettings(settings : *mut DNS_SETTINGS));
    unsafe { FreeDnsSettings(settings as _) }
}
#[inline]
pub unsafe fn FreeInterfaceDnsSettings(settings: *mut DNS_INTERFACE_SETTINGS) {
    windows_core::link!("iphlpapi.dll" "system" fn FreeInterfaceDnsSettings(settings : *mut DNS_INTERFACE_SETTINGS));
    unsafe { FreeInterfaceDnsSettings(settings as _) }
}
#[inline]
pub unsafe fn FreeMibTable(memory: *const core::ffi::c_void) {
    windows_core::link!("iphlpapi.dll" "system" fn FreeMibTable(memory : *const core::ffi::c_void));
    unsafe { FreeMibTable(memory) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[inline]
pub unsafe fn GetAnycastIpAddressEntry(row: *mut MIB_ANYCASTIPADDRESS_ROW) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetAnycastIpAddressEntry(row : *mut MIB_ANYCASTIPADDRESS_ROW) -> super::NTSTATUS);
    unsafe { GetAnycastIpAddressEntry(row as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[inline]
pub unsafe fn GetAnycastIpAddressTable(family: super::ADDRESS_FAMILY, table: *mut PMIB_ANYCASTIPADDRESS_TABLE) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetAnycastIpAddressTable(family : super::ADDRESS_FAMILY, table : *mut PMIB_ANYCASTIPADDRESS_TABLE) -> super::NTSTATUS);
    unsafe { GetAnycastIpAddressTable(family, table as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ws2"))]
#[inline]
pub unsafe fn GetBestInterfaceEx(destinationaddress: *const super::SOCKADDR, bestifindex: *mut u32) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetBestInterfaceEx(destinationaddress : *const super::SOCKADDR, bestifindex : *mut u32) -> super::NTSTATUS);
    unsafe { GetBestInterfaceEx(destinationaddress, bestifindex as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn GetBestRoute2(interfaceluid: Option<*const super::NET_LUID>, interfaceindex: super::NET_IFINDEX, sourceaddress: Option<*const super::SOCKADDR_INET>, destinationaddress: *const super::SOCKADDR_INET, addresssortoptions: u32, bestroute: *mut MIB_IPFORWARD_ROW2, bestsourceaddress: *mut super::SOCKADDR_INET) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetBestRoute2(interfaceluid : *const super::NET_LUID, interfaceindex : super::NET_IFINDEX, sourceaddress : *const super::SOCKADDR_INET, destinationaddress : *const super::SOCKADDR_INET, addresssortoptions : u32, bestroute : *mut MIB_IPFORWARD_ROW2, bestsourceaddress : *mut super::SOCKADDR_INET) -> super::NTSTATUS);
    unsafe { GetBestRoute2(interfaceluid.unwrap_or(core::mem::zeroed()) as _, interfaceindex, sourceaddress.unwrap_or(core::mem::zeroed()) as _, destinationaddress, addresssortoptions, bestroute as _, bestsourceaddress as _) }
}
#[cfg(feature = "ifdef")]
#[inline]
pub unsafe fn GetCurrentThreadCompartmentId() -> super::NET_IF_COMPARTMENT_ID {
    windows_core::link!("iphlpapi.dll" "system" fn GetCurrentThreadCompartmentId() -> super::NET_IF_COMPARTMENT_ID);
    unsafe { GetCurrentThreadCompartmentId() }
}
#[inline]
pub unsafe fn GetCurrentThreadCompartmentScope(compartmentscope: *mut u32, compartmentid: *mut u32) {
    windows_core::link!("iphlpapi.dll" "system" fn GetCurrentThreadCompartmentScope(compartmentscope : *mut u32, compartmentid : *mut u32));
    unsafe { GetCurrentThreadCompartmentScope(compartmentscope as _, compartmentid as _) }
}
#[cfg(feature = "ifdef")]
#[inline]
pub unsafe fn GetDefaultCompartmentId() -> super::NET_IF_COMPARTMENT_ID {
    windows_core::link!("iphlpapi.dll" "system" fn GetDefaultCompartmentId() -> super::NET_IF_COMPARTMENT_ID);
    unsafe { GetDefaultCompartmentId() }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn GetDnsSettings(settings: *mut DNS_SETTINGS) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetDnsSettings(settings : *mut DNS_SETTINGS) -> super::NTSTATUS);
    unsafe { GetDnsSettings(settings as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "ws2"))]
#[inline]
pub unsafe fn GetFlVirtualInterface(row: *mut MIB_FL_VIRTUAL_INTERFACE_ROW) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetFlVirtualInterface(row : *mut MIB_FL_VIRTUAL_INTERFACE_ROW) -> super::NTSTATUS);
    unsafe { GetFlVirtualInterface(row as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "ws2"))]
#[inline]
pub unsafe fn GetFlVirtualInterfaceTable(family: super::ADDRESS_FAMILY, table: *mut PMIB_FL_VIRTUAL_INTERFACE_TABLE) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetFlVirtualInterfaceTable(family : super::ADDRESS_FAMILY, table : *mut PMIB_FL_VIRTUAL_INTERFACE_TABLE) -> super::NTSTATUS);
    unsafe { GetFlVirtualInterfaceTable(family, table as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "ipifcons", feature = "ntddndis"))]
#[inline]
pub unsafe fn GetIfEntry2(row: *mut MIB_IF_ROW2) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetIfEntry2(row : *mut MIB_IF_ROW2) -> super::NTSTATUS);
    unsafe { GetIfEntry2(row as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "ipifcons", feature = "ntddndis"))]
#[inline]
pub unsafe fn GetIfEntry2Ex(level: MIB_IF_ENTRY_LEVEL, row: *mut MIB_IF_ROW2) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetIfEntry2Ex(level : MIB_IF_ENTRY_LEVEL, row : *mut MIB_IF_ROW2) -> super::NTSTATUS);
    unsafe { GetIfEntry2Ex(level, row as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn GetIfStackTable(table: *mut PMIB_IFSTACK_TABLE) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetIfStackTable(table : *mut PMIB_IFSTACK_TABLE) -> super::NTSTATUS);
    unsafe { GetIfStackTable(table as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "ipifcons", feature = "ntddndis"))]
#[inline]
pub unsafe fn GetIfTable2(table: *mut PMIB_IF_TABLE2) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetIfTable2(table : *mut PMIB_IF_TABLE2) -> super::NTSTATUS);
    unsafe { GetIfTable2(table as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "ipifcons", feature = "ntddndis"))]
#[inline]
pub unsafe fn GetIfTable2Ex(level: MIB_IF_TABLE_LEVEL, table: *mut PMIB_IF_TABLE2) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetIfTable2Ex(level : MIB_IF_TABLE_LEVEL, table : *mut PMIB_IF_TABLE2) -> super::NTSTATUS);
    unsafe { GetIfTable2Ex(level, table as _) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn GetInterfaceDnsSettings(interface: windows_core::GUID, settings: *mut DNS_INTERFACE_SETTINGS) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetInterfaceDnsSettings(interface : windows_core::GUID, settings : *mut DNS_INTERFACE_SETTINGS) -> super::NTSTATUS);
    unsafe { GetInterfaceDnsSettings(interface, settings as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn GetInvertedIfStackTable(table: *mut PMIB_INVERTEDIFSTACK_TABLE) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetInvertedIfStackTable(table : *mut PMIB_INVERTEDIFSTACK_TABLE) -> super::NTSTATUS);
    unsafe { GetInvertedIfStackTable(table as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn GetIpForwardEntry2(row: *mut MIB_IPFORWARD_ROW2) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpForwardEntry2(row : *mut MIB_IPFORWARD_ROW2) -> super::NTSTATUS);
    unsafe { GetIpForwardEntry2(row as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn GetIpForwardTable2(family: super::ADDRESS_FAMILY, table: *mut PMIB_IPFORWARD_TABLE2) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpForwardTable2(family : super::ADDRESS_FAMILY, table : *mut PMIB_IPFORWARD_TABLE2) -> super::NTSTATUS);
    unsafe { GetIpForwardTable2(family, table as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn GetIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpInterfaceEntry(row : *mut MIB_IPINTERFACE_ROW) -> super::NTSTATUS);
    unsafe { GetIpInterfaceEntry(row as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn GetIpInterfaceTable(family: super::ADDRESS_FAMILY, table: *mut PMIB_IPINTERFACE_TABLE) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpInterfaceTable(family : super::ADDRESS_FAMILY, table : *mut PMIB_IPINTERFACE_TABLE) -> super::NTSTATUS);
    unsafe { GetIpInterfaceTable(family, table as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn GetIpNetEntry2(row: *mut MIB_IPNET_ROW2) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpNetEntry2(row : *mut MIB_IPNET_ROW2) -> super::NTSTATUS);
    unsafe { GetIpNetEntry2(row as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn GetIpNetTable2(family: super::ADDRESS_FAMILY, table: *mut PMIB_IPNET_TABLE2) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpNetTable2(family : super::ADDRESS_FAMILY, table : *mut PMIB_IPNET_TABLE2) -> super::NTSTATUS);
    unsafe { GetIpNetTable2(family, table as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn GetIpNetworkConnectionBandwidthEstimates(interfaceindex: super::NET_IFINDEX, addressfamily: super::ADDRESS_FAMILY, bandwidthestimates: *mut MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpNetworkConnectionBandwidthEstimates(interfaceindex : super::NET_IFINDEX, addressfamily : super::ADDRESS_FAMILY, bandwidthestimates : *mut MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES) -> super::NTSTATUS);
    unsafe { GetIpNetworkConnectionBandwidthEstimates(interfaceindex, addressfamily, bandwidthestimates as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[inline]
pub unsafe fn GetIpPathEntry(row: *mut MIB_IPPATH_ROW) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpPathEntry(row : *mut MIB_IPPATH_ROW) -> super::NTSTATUS);
    unsafe { GetIpPathEntry(row as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[inline]
pub unsafe fn GetIpPathTable(family: super::ADDRESS_FAMILY, table: *mut PMIB_IPPATH_TABLE) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetIpPathTable(family : super::ADDRESS_FAMILY, table : *mut PMIB_IPPATH_TABLE) -> super::NTSTATUS);
    unsafe { GetIpPathTable(family, table as _) }
}
#[cfg(all(feature = "ifdef", feature = "winnt"))]
#[inline]
pub unsafe fn GetJobCompartmentId(jobhandle: super::HANDLE) -> super::NET_IF_COMPARTMENT_ID {
    windows_core::link!("iphlpapi.dll" "system" fn GetJobCompartmentId(jobhandle : super::HANDLE) -> super::NET_IF_COMPARTMENT_ID);
    unsafe { GetJobCompartmentId(jobhandle) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[inline]
pub unsafe fn GetMulticastIpAddressEntry(row: *mut MIB_MULTICASTIPADDRESS_ROW) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetMulticastIpAddressEntry(row : *mut MIB_MULTICASTIPADDRESS_ROW) -> super::NTSTATUS);
    unsafe { GetMulticastIpAddressEntry(row as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[inline]
pub unsafe fn GetMulticastIpAddressTable(family: super::ADDRESS_FAMILY, table: *mut PMIB_MULTICASTIPADDRESS_TABLE) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetMulticastIpAddressTable(family : super::ADDRESS_FAMILY, table : *mut PMIB_MULTICASTIPADDRESS_TABLE) -> super::NTSTATUS);
    unsafe { GetMulticastIpAddressTable(family, table as _) }
}
#[cfg(all(feature = "bcrypt", feature = "nldef"))]
#[inline]
pub unsafe fn GetNetworkConnectivityHint(connectivityhint: *mut super::NL_NETWORK_CONNECTIVITY_HINT) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetNetworkConnectivityHint(connectivityhint : *mut super::NL_NETWORK_CONNECTIVITY_HINT) -> super::NTSTATUS);
    unsafe { GetNetworkConnectivityHint(connectivityhint as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "nldef"))]
#[inline]
pub unsafe fn GetNetworkConnectivityHintForInterface(interfaceindex: super::NET_IFINDEX, connectivityhint: *mut super::NL_NETWORK_CONNECTIVITY_HINT) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetNetworkConnectivityHintForInterface(interfaceindex : super::NET_IFINDEX, connectivityhint : *mut super::NL_NETWORK_CONNECTIVITY_HINT) -> super::NTSTATUS);
    unsafe { GetNetworkConnectivityHintForInterface(interfaceindex, connectivityhint as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn GetNetworkInformation(networkguid: *const super::NET_IF_NETWORK_GUID, compartmentid: *mut u32, siteid: *mut u32, networkname: &mut [u16]) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetNetworkInformation(networkguid : *const super::NET_IF_NETWORK_GUID, compartmentid : *mut u32, siteid : *mut u32, networkname : *mut u16, length : u32) -> super::NTSTATUS);
    unsafe { GetNetworkInformation(networkguid, compartmentid as _, siteid as _, networkname.as_mut_ptr(), networkname.len().try_into().unwrap()) }
}
#[cfg(feature = "ifdef")]
#[inline]
pub unsafe fn GetSessionCompartmentId(sessionid: u32) -> super::NET_IF_COMPARTMENT_ID {
    windows_core::link!("iphlpapi.dll" "system" fn GetSessionCompartmentId(sessionid : u32) -> super::NET_IF_COMPARTMENT_ID);
    unsafe { GetSessionCompartmentId(sessionid) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn GetTeredoPort(port: *mut u16) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetTeredoPort(port : *mut u16) -> super::NTSTATUS);
    unsafe { GetTeredoPort(port as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn GetUnicastIpAddressEntry(row: *mut MIB_UNICASTIPADDRESS_ROW) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetUnicastIpAddressEntry(row : *mut MIB_UNICASTIPADDRESS_ROW) -> super::NTSTATUS);
    unsafe { GetUnicastIpAddressEntry(row as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn GetUnicastIpAddressTable(family: super::ADDRESS_FAMILY, table: *mut PMIB_UNICASTIPADDRESS_TABLE) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn GetUnicastIpAddressTable(family : super::ADDRESS_FAMILY, table : *mut PMIB_UNICASTIPADDRESS_TABLE) -> super::NTSTATUS);
    unsafe { GetUnicastIpAddressTable(family, table as _) }
}
#[cfg(all(feature = "ifdef", feature = "ws2"))]
#[inline]
pub unsafe fn InitializeFlVirtualInterfaceEntry(row: *mut MIB_FL_VIRTUAL_INTERFACE_ROW) {
    windows_core::link!("iphlpapi.dll" "C" fn InitializeFlVirtualInterfaceEntry(row : *mut MIB_FL_VIRTUAL_INTERFACE_ROW));
    unsafe { InitializeFlVirtualInterfaceEntry(row as _) }
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn InitializeIpForwardEntry(row: *mut MIB_IPFORWARD_ROW2) {
    windows_core::link!("iphlpapi.dll" "system" fn InitializeIpForwardEntry(row : *mut MIB_IPFORWARD_ROW2));
    unsafe { InitializeIpForwardEntry(row as _) }
}
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn InitializeIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW) {
    windows_core::link!("iphlpapi.dll" "system" fn InitializeIpInterfaceEntry(row : *mut MIB_IPINTERFACE_ROW));
    unsafe { InitializeIpInterfaceEntry(row as _) }
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn InitializeUnicastIpAddressEntry(row: *mut MIB_UNICASTIPADDRESS_ROW) {
    windows_core::link!("iphlpapi.dll" "system" fn InitializeUnicastIpAddressEntry(row : *mut MIB_UNICASTIPADDRESS_ROW));
    unsafe { InitializeUnicastIpAddressEntry(row as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "nldef", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn NotifyIpInterfaceChange(family: super::ADDRESS_FAMILY, callback: PIPINTERFACE_CHANGE_CALLBACK, callercontext: Option<*const core::ffi::c_void>, initialnotification: bool, notificationhandle: *mut super::HANDLE) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn NotifyIpInterfaceChange(family : super::ADDRESS_FAMILY, callback : PIPINTERFACE_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, initialnotification : bool, notificationhandle : *mut super::HANDLE) -> super::NTSTATUS);
    unsafe { NotifyIpInterfaceChange(family, callback, callercontext.unwrap_or(core::mem::zeroed()) as _, initialnotification, notificationhandle as _) }
}
#[cfg(all(feature = "bcrypt", feature = "nldef", feature = "winnt"))]
#[inline]
pub unsafe fn NotifyNetworkConnectivityHintChange(callback: PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK, callercontext: Option<*const core::ffi::c_void>, initialnotification: bool, notificationhandle: *mut super::HANDLE) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn NotifyNetworkConnectivityHintChange(callback : PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, initialnotification : bool, notificationhandle : *mut super::HANDLE) -> super::NTSTATUS);
    unsafe { NotifyNetworkConnectivityHintChange(callback, callercontext.unwrap_or(core::mem::zeroed()) as _, initialnotification, notificationhandle as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn NotifyRouteChange2(addressfamily: super::ADDRESS_FAMILY, callback: PIPFORWARD_CHANGE_CALLBACK, callercontext: *const core::ffi::c_void, initialnotification: bool, notificationhandle: *mut super::HANDLE) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn NotifyRouteChange2(addressfamily : super::ADDRESS_FAMILY, callback : PIPFORWARD_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, initialnotification : bool, notificationhandle : *mut super::HANDLE) -> super::NTSTATUS);
    unsafe { NotifyRouteChange2(addressfamily, callback, callercontext, initialnotification, notificationhandle as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn NotifyStableUnicastIpAddressTable(family: super::ADDRESS_FAMILY, table: *mut PMIB_UNICASTIPADDRESS_TABLE, callercallback: PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK, callercontext: *const core::ffi::c_void, notificationhandle: *mut super::HANDLE) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn NotifyStableUnicastIpAddressTable(family : super::ADDRESS_FAMILY, table : *mut PMIB_UNICASTIPADDRESS_TABLE, callercallback : PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK, callercontext : *const core::ffi::c_void, notificationhandle : *mut super::HANDLE) -> super::NTSTATUS);
    unsafe { NotifyStableUnicastIpAddressTable(family, table as _, callercallback, callercontext, notificationhandle as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NotifyTeredoPortChange(callback: PTEREDO_PORT_CHANGE_CALLBACK, callercontext: *const core::ffi::c_void, initialnotification: bool, notificationhandle: *mut super::HANDLE) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn NotifyTeredoPortChange(callback : PTEREDO_PORT_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, initialnotification : bool, notificationhandle : *mut super::HANDLE) -> super::NTSTATUS);
    unsafe { NotifyTeredoPortChange(callback, callercontext, initialnotification, notificationhandle as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn NotifyUnicastIpAddressChange(family: super::ADDRESS_FAMILY, callback: PUNICAST_IPADDRESS_CHANGE_CALLBACK, callercontext: Option<*const core::ffi::c_void>, initialnotification: bool, notificationhandle: *mut super::HANDLE) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn NotifyUnicastIpAddressChange(family : super::ADDRESS_FAMILY, callback : PUNICAST_IPADDRESS_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, initialnotification : bool, notificationhandle : *mut super::HANDLE) -> super::NTSTATUS);
    unsafe { NotifyUnicastIpAddressChange(family, callback, callercontext.unwrap_or(core::mem::zeroed()) as _, initialnotification, notificationhandle as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn ResolveIpNetEntry2(row: *mut MIB_IPNET_ROW2, sourceaddress: Option<*const super::SOCKADDR_INET>) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn ResolveIpNetEntry2(row : *mut MIB_IPNET_ROW2, sourceaddress : *const super::SOCKADDR_INET) -> super::NTSTATUS);
    unsafe { ResolveIpNetEntry2(row as _, sourceaddress.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn SetCurrentThreadCompartmentId(compartmentid: super::NET_IF_COMPARTMENT_ID) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn SetCurrentThreadCompartmentId(compartmentid : super::NET_IF_COMPARTMENT_ID) -> super::NTSTATUS);
    unsafe { SetCurrentThreadCompartmentId(compartmentid) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn SetCurrentThreadCompartmentScope(compartmentscope: super::NET_IF_COMPARTMENT_SCOPE) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn SetCurrentThreadCompartmentScope(compartmentscope : super::NET_IF_COMPARTMENT_SCOPE) -> super::NTSTATUS);
    unsafe { SetCurrentThreadCompartmentScope(compartmentscope) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn SetDnsSettings(settings: *const DNS_SETTINGS) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn SetDnsSettings(settings : *const DNS_SETTINGS) -> super::NTSTATUS);
    unsafe { SetDnsSettings(settings) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "ws2"))]
#[inline]
pub unsafe fn SetFlVirtualInterface(row: *const MIB_FL_VIRTUAL_INTERFACE_ROW) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn SetFlVirtualInterface(row : *const MIB_FL_VIRTUAL_INTERFACE_ROW) -> super::NTSTATUS);
    unsafe { SetFlVirtualInterface(row) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn SetInterfaceDnsSettings(interface: windows_core::GUID, settings: *const DNS_INTERFACE_SETTINGS) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn SetInterfaceDnsSettings(interface : windows_core::GUID, settings : *const DNS_INTERFACE_SETTINGS) -> super::NTSTATUS);
    unsafe { SetInterfaceDnsSettings(interface, settings) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn SetIpForwardEntry2(route: *const MIB_IPFORWARD_ROW2) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpForwardEntry2(route : *const MIB_IPFORWARD_ROW2) -> super::NTSTATUS);
    unsafe { SetIpForwardEntry2(route) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn SetIpInterfaceEntry(row: *mut MIB_IPINTERFACE_ROW) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpInterfaceEntry(row : *mut MIB_IPINTERFACE_ROW) -> super::NTSTATUS);
    unsafe { SetIpInterfaceEntry(row as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn SetIpNetEntry2(row: *const MIB_IPNET_ROW2) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn SetIpNetEntry2(row : *const MIB_IPNET_ROW2) -> super::NTSTATUS);
    unsafe { SetIpNetEntry2(row) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "winnt"))]
#[inline]
pub unsafe fn SetJobCompartmentId(jobhandle: super::HANDLE, compartmentid: super::NET_IF_COMPARTMENT_ID) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn SetJobCompartmentId(jobhandle : super::HANDLE, compartmentid : super::NET_IF_COMPARTMENT_ID) -> super::NTSTATUS);
    unsafe { SetJobCompartmentId(jobhandle, compartmentid) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn SetNetworkInformation<P2>(networkguid: *const super::NET_IF_NETWORK_GUID, compartmentid: super::NET_IF_COMPARTMENT_ID, networkname: P2) -> super::NTSTATUS
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("iphlpapi.dll" "system" fn SetNetworkInformation(networkguid : *const super::NET_IF_NETWORK_GUID, compartmentid : super::NET_IF_COMPARTMENT_ID, networkname : windows_core::PCWSTR) -> super::NTSTATUS);
    unsafe { SetNetworkInformation(networkguid, compartmentid, networkname.param().abi()) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef"))]
#[inline]
pub unsafe fn SetSessionCompartmentId(sessionid: u32, compartmentid: super::NET_IF_COMPARTMENT_ID) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn SetSessionCompartmentId(sessionid : u32, compartmentid : super::NET_IF_COMPARTMENT_ID) -> super::NTSTATUS);
    unsafe { SetSessionCompartmentId(sessionid, compartmentid) }
}
#[cfg(all(feature = "bcrypt", feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[inline]
pub unsafe fn SetUnicastIpAddressEntry(row: *const MIB_UNICASTIPADDRESS_ROW) -> super::NTSTATUS {
    windows_core::link!("iphlpapi.dll" "system" fn SetUnicastIpAddressEntry(row : *const MIB_UNICASTIPADDRESS_ROW) -> super::NTSTATUS);
    unsafe { SetUnicastIpAddressEntry(row) }
}
#[cfg(all(feature = "ifdef", feature = "winnt"))]
#[inline]
pub unsafe fn if_indextoname(interfaceindex: super::NET_IFINDEX, interfacename: *mut i8) -> super::PCHAR {
    windows_core::link!("iphlpapi.dll" "system" fn if_indextoname(interfaceindex : super::NET_IFINDEX, interfacename : *mut i8) -> super::PCHAR);
    unsafe { if_indextoname(interfaceindex, interfacename as _) }
}
#[cfg(feature = "ifdef")]
#[inline]
pub unsafe fn if_nametoindex<P0>(interfacename: P0) -> super::NET_IFINDEX
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("iphlpapi.dll" "system" fn if_nametoindex(interfacename : windows_core::PCSTR) -> super::NET_IFINDEX);
    unsafe { if_nametoindex(interfacename.param().abi()) }
}
pub const ANY_SIZE: u32 = 1;
pub const DNS_DDR_ADAPTER_ENABLE: u32 = 1;
pub const DNS_DDR_ADAPTER_ENABLE_DOH: u32 = 1;
pub const DNS_DDR_ADAPTER_ENABLE_UDP_FALLBACK: u32 = 2;
pub const DNS_DDR_ADAPTER_MAKE_DDR_NON_BLOCKING: u32 = 4;
pub const DNS_DOH_AUTO_UPGRADE_SERVER: u32 = 8;
pub const DNS_DOH_POLICY_AUTO: u32 = 16;
pub const DNS_DOH_POLICY_BLOCK: u32 = 512;
pub const DNS_DOH_POLICY_DISABLE: u32 = 8;
pub const DNS_DOH_POLICY_NOT_CONFIGURED: u32 = 4;
pub const DNS_DOH_POLICY_REQUIRED: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_DOH_SERVER_SETTINGS {
    pub Template: windows_core::PWSTR,
    pub Flags: u64,
}
pub const DNS_DOH_SERVER_SETTINGS_ENABLE: u32 = 2;
pub const DNS_DOH_SERVER_SETTINGS_ENABLE_AUTO: u32 = 1;
pub const DNS_DOH_SERVER_SETTINGS_ENABLE_DDR: u32 = 16;
pub const DNS_DOH_SERVER_SETTINGS_FALLBACK_TO_UDP: u32 = 4;
pub const DNS_DOH_SERVER_SETTINGS_MAKE_DDR_NON_BLOCKING: u32 = 32;
pub const DNS_DOT_AUTO_UPGRADE_SERVER: u32 = 4;
pub const DNS_DOT_POLICY_BLOCK: u32 = 256;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_DOT_SERVER_SETTINGS {
    pub Hostname: windows_core::PWSTR,
    pub Flags: u64,
    pub Port: u16,
}
pub const DNS_DOT_SERVER_SETTINGS_ENABLE: u32 = 1;
pub const DNS_DOT_SERVER_SETTINGS_ENABLE_AUTO: u32 = 8;
pub const DNS_DOT_SERVER_SETTINGS_ENABLE_DDR: u32 = 16;
pub const DNS_DOT_SERVER_SETTINGS_FALLBACK_TO_UDP: u32 = 2;
pub const DNS_DOT_SERVER_SETTINGS_MAKE_DDR_NON_BLOCKING: u32 = 32;
pub const DNS_ENABLE_DDR: u32 = 64;
pub const DNS_ENABLE_DNR: u32 = 1024;
pub const DNS_ENABLE_DOH: u32 = 1;
pub const DNS_ENABLE_DOT: u32 = 128;
pub const DNS_ENCRYPTION_POLICY_AUTO: u32 = 16;
pub const DNS_ENCRYPTION_POLICY_DISABLE: u32 = 8;
pub const DNS_ENCRYPTION_POLICY_NOT_CONFIGURED: u32 = 4;
pub const DNS_ENCRYPTION_POLICY_REQUIRED: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_INTERFACE_SETTINGS_EX {
    pub SettingsV1: DNS_INTERFACE_SETTINGS,
    pub DisableUnconstrainedQueries: u32,
    pub SupplementalSearchList: windows_core::PWSTR,
}
pub const DNS_INTERFACE_SETTINGS_VERSION1: u32 = 1;
pub const DNS_INTERFACE_SETTINGS_VERSION2: u32 = 2;
pub const DNS_INTERFACE_SETTINGS_VERSION3: u32 = 3;
pub const DNS_INTERFACE_SETTINGS_VERSION4: u32 = 4;
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
pub type DNS_SERVER_PROPERTY_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union DNS_SERVER_PROPERTY_TYPES {
    pub DohSettings: *mut DNS_DOH_SERVER_SETTINGS,
    pub DotSettings: *mut DNS_DOT_SERVER_SETTINGS,
}
impl Default for DNS_SERVER_PROPERTY_TYPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_SERVER_PROPERTY_VERSION1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_SETTINGS {
    pub Version: u32,
    pub Flags: u64,
    pub Hostname: windows_core::PWSTR,
    pub Domain: windows_core::PWSTR,
    pub SearchList: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_SETTINGS2 {
    pub Version: u32,
    pub Flags: u64,
    pub Hostname: windows_core::PWSTR,
    pub Domain: windows_core::PWSTR,
    pub SearchList: windows_core::PWSTR,
    pub SettingFlags: u64,
}
pub const DNS_SETTINGS_ENABLE_LLMNR: u32 = 128;
pub const DNS_SETTINGS_QUERY_ADAPTER_NAME: u32 = 256;
pub const DNS_SETTINGS_VERSION1: u32 = 1;
pub const DNS_SETTINGS_VERSION2: u32 = 2;
pub const DNS_SETTING_DDR: u32 = 32768;
pub const DNS_SETTING_DISABLE_UNCONSTRAINED_QUERIES: u32 = 1024;
pub const DNS_SETTING_DOH: u32 = 4096;
pub const DNS_SETTING_DOH_PROFILE: u32 = 8192;
pub const DNS_SETTING_DOMAIN: u32 = 32;
pub const DNS_SETTING_DOT: u32 = 65536;
pub const DNS_SETTING_DOT_PROFILE: u32 = 131072;
pub const DNS_SETTING_ENCRYPTED_DNS_ADAPTER_FLAGS: u32 = 16384;
pub const DNS_SETTING_HOSTNAME: u32 = 64;
pub const DNS_SETTING_IPV6: u32 = 1;
pub const DNS_SETTING_NAMESERVER: u32 = 2;
pub const DNS_SETTING_PROFILE_NAMESERVER: u32 = 512;
pub const DNS_SETTING_REGISTER_ADAPTER_NAME: u32 = 16;
pub const DNS_SETTING_REGISTRATION_ENABLED: u32 = 8;
pub const DNS_SETTING_SEARCHLIST: u32 = 4;
pub const DNS_SETTING_SUPPLEMENTAL_SEARCH_LIST: u32 = 2048;
pub const DnsServerDohProperty: DNS_SERVER_PROPERTY_TYPE = 1;
pub const DnsServerDotProperty: DNS_SERVER_PROPERTY_TYPE = 2;
pub const DnsServerInvalidProperty: DNS_SERVER_PROPERTY_TYPE = 0;
pub const IF_NAMESIZE: u32 = 256;
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct IP_ADDRESS_PREFIX {
    pub Prefix: super::SOCKADDR_INET,
    pub PrefixLength: u8,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "ws2"))]
impl Default for IP_ADDRESS_PREFIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_ANYCASTIPADDRESS_ROW {
    pub Address: super::SOCKADDR_INET,
    pub InterfaceLuid: super::NET_LUID,
    pub InterfaceIndex: super::NET_IFINDEX,
    pub ScopeId: super::SCOPE_ID,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
impl Default for MIB_ANYCASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_ANYCASTIPADDRESS_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_ANYCASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
impl Default for MIB_ANYCASTIPADDRESS_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_FL_VIRTUAL_INTERFACE_ROW {
    pub Family: super::ADDRESS_FAMILY,
    pub IfLuid: super::IF_LUID,
    pub VirtualIfId: u32,
    pub CompartmentGuid: windows_core::GUID,
    pub IsolationMode: NET_FL_ISOLATION_MODE,
    pub Origin: NET_FL_VIRTUAL_INTERFACE_ORIGIN,
    pub VirtualIfLuid: super::IF_LUID,
    pub VirtualIfIndex: super::IF_INDEX,
    pub AllowLocalNd: bool,
    pub AttachedFlsnpiClients: u32,
    pub FlsnpiClientConfigErrors: u32,
    pub FlsnpiClientInjectErrors: u64,
    pub FlsnpiClientCloneErrors: u64,
    pub InFlsnpiIndicatedPackets: u64,
    pub InFlsnpiClientReturnedPackets: u64,
    pub InFlsnpiClientSilentlyDroppedPackets: u64,
    pub InFlsnpiClientDroppedPackets: u64,
    pub InFlsnpiClientInjectedPackets: u64,
    pub InFlsnpiClientClonedPackets: u64,
    pub OutFlsnpiIndicatedPackets: u64,
    pub OutFlsnpiClientReturnedPackets: u64,
    pub OutFlsnpiClientDroppedPackets: u64,
    pub OutFlsnpiClientSilentlyDroppedPackets: u64,
    pub OutFlsnpiClientInjectedPackets: u64,
    pub OutFlsnpiClientClonedPackets: u64,
    pub OutFlsnpiClientClonedPacketsForNbSplit: u64,
}
#[cfg(all(feature = "ifdef", feature = "ws2"))]
impl Default for MIB_FL_VIRTUAL_INTERFACE_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_FL_VIRTUAL_INTERFACE_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_FL_VIRTUAL_INTERFACE_ROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "ws2"))]
impl Default for MIB_FL_VIRTUAL_INTERFACE_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ifdef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_IFSTACK_ROW {
    pub HigherLayerInterfaceIndex: super::NET_IFINDEX,
    pub LowerLayerInterfaceIndex: super::NET_IFINDEX,
}
#[repr(C)]
#[cfg(feature = "ifdef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIB_IFSTACK_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_IFSTACK_ROW; 1],
}
#[cfg(feature = "ifdef")]
impl Default for MIB_IFSTACK_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MIB_IF_ENTRY_LEVEL = i32;
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis"))]
#[derive(Clone, Copy)]
pub struct MIB_IF_ROW2 {
    pub InterfaceLuid: super::NET_LUID,
    pub InterfaceIndex: super::NET_IFINDEX,
    pub InterfaceGuid: windows_core::GUID,
    pub Alias: [u16; 257],
    pub Description: [u16; 257],
    pub PhysicalAddressLength: u32,
    pub PhysicalAddress: [u8; 32],
    pub PermanentPhysicalAddress: [u8; 32],
    pub Mtu: u32,
    pub Type: super::IFTYPE,
    pub TunnelType: super::TUNNEL_TYPE,
    pub MediaType: super::NDIS_MEDIUM,
    pub PhysicalMediumType: super::NDIS_PHYSICAL_MEDIUM,
    pub AccessType: super::NET_IF_ACCESS_TYPE,
    pub DirectionType: super::NET_IF_DIRECTION_TYPE,
    pub InterfaceAndOperStatusFlags: MIB_IF_ROW2_0,
    pub OperStatus: super::IF_OPER_STATUS,
    pub AdminStatus: super::NET_IF_ADMIN_STATUS,
    pub MediaConnectState: super::NET_IF_MEDIA_CONNECT_STATE,
    pub NetworkGuid: super::NET_IF_NETWORK_GUID,
    pub ConnectionType: super::NET_IF_CONNECTION_TYPE,
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
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis"))]
impl Default for MIB_IF_ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_IF_ROW2_0 {
    pub _bitfield: bool,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis"))]
#[derive(Clone, Copy)]
pub struct MIB_IF_TABLE2 {
    pub NumEntries: u32,
    pub Table: [MIB_IF_ROW2; 1],
}
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis"))]
impl Default for MIB_IF_TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MIB_IF_TABLE_LEVEL = i32;
pub const MIB_INVALID_TEREDO_PORT_NUMBER: u32 = 0;
#[repr(C)]
#[cfg(feature = "ifdef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_INVERTEDIFSTACK_ROW {
    pub LowerLayerInterfaceIndex: super::NET_IFINDEX,
    pub HigherLayerInterfaceIndex: super::NET_IFINDEX,
}
#[repr(C)]
#[cfg(feature = "ifdef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIB_INVERTEDIFSTACK_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_INVERTEDIFSTACK_ROW; 1],
}
#[cfg(feature = "ifdef")]
impl Default for MIB_INVERTEDIFSTACK_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_IPFORWARD_ROW2 {
    pub InterfaceLuid: super::NET_LUID,
    pub InterfaceIndex: super::NET_IFINDEX,
    pub DestinationPrefix: IP_ADDRESS_PREFIX,
    pub NextHop: super::SOCKADDR_INET,
    pub SitePrefixLength: u8,
    pub ValidLifetime: u32,
    pub PreferredLifetime: u32,
    pub Metric: u32,
    pub Protocol: super::NL_ROUTE_PROTOCOL,
    pub Loopback: bool,
    pub AutoconfigureAddress: bool,
    pub Publish: bool,
    pub Immortal: bool,
    pub Age: u32,
    pub Origin: super::NL_ROUTE_ORIGIN,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
impl Default for MIB_IPFORWARD_ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_IPFORWARD_TABLE2 {
    pub NumEntries: u32,
    pub Table: [MIB_IPFORWARD_ROW2; 1],
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
impl Default for MIB_IPFORWARD_TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_IPINTERFACE_ROW {
    pub Family: super::ADDRESS_FAMILY,
    pub InterfaceLuid: super::NET_LUID,
    pub InterfaceIndex: super::NET_IFINDEX,
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
    pub RouterDiscoveryBehavior: super::NL_ROUTER_DISCOVERY_BEHAVIOR,
    pub DadTransmits: u32,
    pub BaseReachableTime: u32,
    pub RetransmitTime: u32,
    pub PathMtuDiscoveryTimeout: u32,
    pub LinkLocalAddressBehavior: super::NL_LINK_LOCAL_ADDRESS_BEHAVIOR,
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
    pub TransmitOffload: super::NL_INTERFACE_OFFLOAD_ROD,
    pub ReceiveOffload: super::NL_INTERFACE_OFFLOAD_ROD,
    pub DisableDefaultRoutes: bool,
}
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "ws2"))]
impl Default for MIB_IPINTERFACE_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_IPINTERFACE_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_IPINTERFACE_ROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "ws2"))]
impl Default for MIB_IPINTERFACE_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_IPNET_ROW2 {
    pub Address: super::SOCKADDR_INET,
    pub InterfaceIndex: super::NET_IFINDEX,
    pub InterfaceLuid: super::NET_LUID,
    pub PhysicalAddress: [u8; 32],
    pub PhysicalAddressLength: u32,
    pub State: super::NL_NEIGHBOR_STATE,
    pub Anonymous: MIB_IPNET_ROW2_0,
    pub ReachabilityTime: MIB_IPNET_ROW2_1,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
impl Default for MIB_IPNET_ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union MIB_IPNET_ROW2_0 {
    pub Anonymous: MIB_IPNET_ROW2_0_0,
    pub Flags: u8,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
impl Default for MIB_IPNET_ROW2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_IPNET_ROW2_0_0 {
    pub _bitfield: bool,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union MIB_IPNET_ROW2_1 {
    pub LastReachable: u32,
    pub LastUnreachable: u32,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
impl Default for MIB_IPNET_ROW2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_IPNET_TABLE2 {
    pub NumEntries: u32,
    pub Table: [MIB_IPNET_ROW2; 1],
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
impl Default for MIB_IPNET_TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_IPPATH_ROW {
    pub Source: super::SOCKADDR_INET,
    pub Destination: super::SOCKADDR_INET,
    pub InterfaceLuid: super::NET_LUID,
    pub InterfaceIndex: super::NET_IFINDEX,
    pub CurrentNextHop: super::SOCKADDR_INET,
    pub PathMtu: u32,
    pub RttMean: u32,
    pub RttDeviation: u32,
    pub Anonymous: MIB_IPPATH_ROW_0,
    pub IsReachable: bool,
    pub LinkTransmitSpeed: u64,
    pub LinkReceiveSpeed: u64,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
impl Default for MIB_IPPATH_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union MIB_IPPATH_ROW_0 {
    pub LastReachable: u32,
    pub LastUnreachable: u32,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
impl Default for MIB_IPPATH_ROW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_IPPATH_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_IPPATH_ROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
impl Default for MIB_IPPATH_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "nldef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    pub InboundBandwidthInformation: super::NL_BANDWIDTH_INFORMATION,
    pub OutboundBandwidthInformation: super::NL_BANDWIDTH_INFORMATION,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_MULTICASTIPADDRESS_ROW {
    pub Address: super::SOCKADDR_INET,
    pub InterfaceIndex: super::NET_IFINDEX,
    pub InterfaceLuid: super::NET_LUID,
    pub ScopeId: super::SCOPE_ID,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
impl Default for MIB_MULTICASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_MULTICASTIPADDRESS_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_MULTICASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
impl Default for MIB_MULTICASTIPADDRESS_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MIB_NOTIFICATION_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_UNICASTIPADDRESS_ROW {
    pub Address: super::SOCKADDR_INET,
    pub InterfaceLuid: super::NET_LUID,
    pub InterfaceIndex: super::NET_IFINDEX,
    pub PrefixOrigin: super::NL_PREFIX_ORIGIN,
    pub SuffixOrigin: super::NL_SUFFIX_ORIGIN,
    pub ValidLifetime: u32,
    pub PreferredLifetime: u32,
    pub OnLinkPrefixLength: u8,
    pub SkipAsSource: bool,
    pub DadState: super::NL_DAD_STATE,
    pub ScopeId: super::SCOPE_ID,
    pub CreationTimeStamp: i64,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
impl Default for MIB_UNICASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_UNICASTIPADDRESS_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_UNICASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
impl Default for MIB_UNICASTIPADDRESS_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MibAddInstance: MIB_NOTIFICATION_TYPE = 1;
pub const MibDeleteInstance: MIB_NOTIFICATION_TYPE = 2;
pub const MibIfEntryNormal: MIB_IF_ENTRY_LEVEL = 0;
pub const MibIfEntryNormalWithoutStatistics: MIB_IF_ENTRY_LEVEL = 2;
pub const MibIfTableNormal: MIB_IF_TABLE_LEVEL = 0;
pub const MibIfTableNormalWithoutStatistics: MIB_IF_TABLE_LEVEL = 2;
pub const MibIfTableRaw: MIB_IF_TABLE_LEVEL = 1;
pub const MibInitialNotification: MIB_NOTIFICATION_TYPE = 3;
pub const MibParameterNotification: MIB_NOTIFICATION_TYPE = 0;
pub type NET_FL_ISOLATION_MODE = i32;
pub type NET_FL_VIRTUAL_INTERFACE_ORIGIN = i32;
pub const NetFlIsolationModeNone: NET_FL_ISOLATION_MODE = 0;
pub const NetFlIsolationModeVlan: NET_FL_ISOLATION_MODE = 1;
pub const NetFlIsolationModeVsid: NET_FL_ISOLATION_MODE = 2;
pub const NetFlVirtualInterfaceOriginApi: NET_FL_VIRTUAL_INTERFACE_ORIGIN = 1;
pub const NetFlVirtualInterfaceOriginDefault: NET_FL_VIRTUAL_INTERFACE_ORIGIN = 2;
pub const NetFlVirtualInterfaceOriginOid: NET_FL_VIRTUAL_INTERFACE_ORIGIN = 0;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
pub type PIPFORWARD_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, row: *const MIB_IPFORWARD_ROW2, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "ws2"))]
pub type PIPINTERFACE_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, row: *const MIB_IPINTERFACE_ROW, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "ws2"))]
pub type PIP_ADDRESS_PREFIX = *mut IP_ADDRESS_PREFIX;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
pub type PMIB_ANYCASTIPADDRESS_ROW = *mut MIB_ANYCASTIPADDRESS_ROW;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
pub type PMIB_ANYCASTIPADDRESS_TABLE = *mut MIB_ANYCASTIPADDRESS_TABLE;
#[cfg(all(feature = "ifdef", feature = "ws2"))]
pub type PMIB_FL_VIRTUAL_INTERFACE_ROW = *mut MIB_FL_VIRTUAL_INTERFACE_ROW;
#[cfg(all(feature = "ifdef", feature = "ws2"))]
pub type PMIB_FL_VIRTUAL_INTERFACE_TABLE = *mut MIB_FL_VIRTUAL_INTERFACE_TABLE;
#[cfg(feature = "ifdef")]
pub type PMIB_IFSTACK_ROW = *mut MIB_IFSTACK_ROW;
#[cfg(feature = "ifdef")]
pub type PMIB_IFSTACK_TABLE = *mut MIB_IFSTACK_TABLE;
pub type PMIB_IF_ENTRY_LEVEL = *mut MIB_IF_ENTRY_LEVEL;
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis"))]
pub type PMIB_IF_ROW2 = *mut MIB_IF_ROW2;
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis"))]
pub type PMIB_IF_TABLE2 = *mut MIB_IF_TABLE2;
pub type PMIB_IF_TABLE_LEVEL = *mut MIB_IF_TABLE_LEVEL;
#[cfg(feature = "ifdef")]
pub type PMIB_INVERTEDIFSTACK_ROW = *mut MIB_INVERTEDIFSTACK_ROW;
#[cfg(feature = "ifdef")]
pub type PMIB_INVERTEDIFSTACK_TABLE = *mut MIB_INVERTEDIFSTACK_TABLE;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
pub type PMIB_IPFORWARD_ROW2 = *mut MIB_IPFORWARD_ROW2;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
pub type PMIB_IPFORWARD_TABLE2 = *mut MIB_IPFORWARD_TABLE2;
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "ws2"))]
pub type PMIB_IPINTERFACE_ROW = *mut MIB_IPINTERFACE_ROW;
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "ws2"))]
pub type PMIB_IPINTERFACE_TABLE = *mut MIB_IPINTERFACE_TABLE;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
pub type PMIB_IPNET_ROW2 = *mut MIB_IPNET_ROW2;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
pub type PMIB_IPNET_TABLE2 = *mut MIB_IPNET_TABLE2;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
pub type PMIB_IPPATH_ROW = *mut MIB_IPPATH_ROW;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
pub type PMIB_IPPATH_TABLE = *mut MIB_IPPATH_TABLE;
#[cfg(feature = "nldef")]
pub type PMIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES = *mut MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
pub type PMIB_MULTICASTIPADDRESS_ROW = *mut MIB_MULTICASTIPADDRESS_ROW;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "ws2"))]
pub type PMIB_MULTICASTIPADDRESS_TABLE = *mut MIB_MULTICASTIPADDRESS_TABLE;
pub type PMIB_NOTIFICATION_TYPE = *mut MIB_NOTIFICATION_TYPE;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
pub type PMIB_UNICASTIPADDRESS_ROW = *mut MIB_UNICASTIPADDRESS_ROW;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
pub type PMIB_UNICASTIPADDRESS_TABLE = *mut MIB_UNICASTIPADDRESS_TABLE;
#[cfg(feature = "nldef")]
pub type PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, connectivityhint: super::NL_NETWORK_CONNECTIVITY_HINT)>;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
pub type PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, addresstable: *const MIB_UNICASTIPADDRESS_TABLE)>;
pub type PTEREDO_PORT_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, port: u16, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "ws2"))]
pub type PUNICAST_IPADDRESS_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, row: *const MIB_UNICASTIPADDRESS_ROW, notificationtype: MIB_NOTIFICATION_TYPE)>;
