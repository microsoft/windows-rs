#[cfg(feature = "winnt")]
windows_link::link!("iphlpapi.dll" "system" fn CancelMibChangeNotify2(notificationhandle : super::HANDLE) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn ConvertCompartmentGuidToId(compartmentguid : *const windows_sys::core::GUID, compartmentid : super::PNET_IF_COMPARTMENT_ID) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn ConvertCompartmentIdToGuid(compartmentid : super::NET_IF_COMPARTMENT_ID, compartmentguid : *mut windows_sys::core::GUID) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceAliasToLuid(interfacealias : *const super::WCHAR, interfaceluid : super::PNET_LUID) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceGuidToLuid(interfaceguid : *const windows_sys::core::GUID, interfaceluid : super::PNET_LUID) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceIndexToLuid(interfaceindex : super::NET_IFINDEX, interfaceluid : super::PNET_LUID) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToAlias(interfaceluid : *const super::NET_LUID, interfacealias : windows_sys::core::PWSTR, length : super::SIZE_T) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToGuid(interfaceluid : *const super::NET_LUID, interfaceguid : *mut windows_sys::core::GUID) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToIndex(interfaceluid : *const super::NET_LUID, interfaceindex : super::PNET_IFINDEX) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToNameA(interfaceluid : *const super::NET_LUID, interfacename : windows_sys::core::PSTR, length : super::SIZE_T) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToNameW(interfaceluid : *const super::NET_LUID, interfacename : windows_sys::core::PWSTR, length : super::SIZE_T) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceNameToLuidA(interfacename : *const super::CHAR, interfaceluid : *mut super::NET_LUID) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceNameToLuidW(interfacename : *const super::WCHAR, interfaceluid : *mut super::NET_LUID) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "basetsd", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertIpv4MaskToLength(mask : super::ULONG, masklength : super::PUINT8) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertLengthToIpv4Mask(masklength : super::ULONG, mask : super::PULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn CreateAnycastIpAddressEntry(row : *const MIB_ANYCASTIPADDRESS_ROW) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn CreateFlVirtualInterface(row : *const MIB_FL_VIRTUAL_INTERFACE_ROW) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn CreateIpForwardEntry2(row : *const MIB_IPFORWARD_ROW2) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn CreateIpNetEntry2(row : *const MIB_IPNET_ROW2) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "in6addr", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn CreateSortedAddressPairs(sourceaddresslist : super::PSOCKADDR_IN6, sourceaddresscount : super::ULONG, destinationaddresslist : super::PSOCKADDR_IN6, destinationaddresscount : super::ULONG, addresssortoptions : super::ULONG, sortedaddresspairlist : *mut super::PSOCKADDR_IN6_PAIR, sortedaddresspaircount : *mut super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn CreateUnicastIpAddressEntry(row : *const MIB_UNICASTIPADDRESS_ROW) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn DeleteAnycastIpAddressEntry(row : *const MIB_ANYCASTIPADDRESS_ROW) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn DeleteFlVirtualInterface(row : *const MIB_FL_VIRTUAL_INTERFACE_ROW) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn DeleteIpForwardEntry2(row : *const MIB_IPFORWARD_ROW2) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn DeleteIpNetEntry2(row : *const MIB_IPNET_ROW2) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn DeleteUnicastIpAddressEntry(row : *const MIB_UNICASTIPADDRESS_ROW) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn FlushIpNetTable2(family : super::ADDRESS_FAMILY, interfaceindex : super::NET_IFINDEX) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "ws2")]
windows_link::link!("iphlpapi.dll" "system" fn FlushIpPathTable(family : super::ADDRESS_FAMILY) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("iphlpapi.dll" "system" fn FreeDnsSettings(settings : *mut DNS_SETTINGS));
#[cfg(feature = "shared")]
windows_link::link!("iphlpapi.dll" "system" fn FreeInterfaceDnsSettings(settings : *mut DNS_INTERFACE_SETTINGS));
#[cfg(feature = "winnt")]
windows_link::link!("iphlpapi.dll" "system" fn FreeMibTable(memory : super::PVOID));
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetAnycastIpAddressEntry(row : PMIB_ANYCASTIPADDRESS_ROW) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetAnycastIpAddressTable(family : super::ADDRESS_FAMILY, table : *mut PMIB_ANYCASTIPADDRESS_TABLE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetBestInterfaceEx(destinationaddress : *const super::SOCKADDR, bestifindex : super::PULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetBestRoute2(interfaceluid : *const super::NET_LUID, interfaceindex : super::NET_IFINDEX, sourceaddress : *const super::SOCKADDR_INET, destinationaddress : *const super::SOCKADDR_INET, addresssortoptions : super::ULONG, bestroute : PMIB_IPFORWARD_ROW2, bestsourceaddress : *mut super::SOCKADDR_INET) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn GetCurrentThreadCompartmentId() -> super::NET_IF_COMPARTMENT_ID);
#[cfg(feature = "ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn GetCurrentThreadCompartmentScope(compartmentscope : super::PNET_IF_COMPARTMENT_SCOPE, compartmentid : super::PNET_IF_COMPARTMENT_ID));
#[cfg(feature = "ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn GetDefaultCompartmentId() -> super::NET_IF_COMPARTMENT_ID);
#[cfg(feature = "shared")]
windows_link::link!("iphlpapi.dll" "system" fn GetDnsSettings(settings : *mut DNS_SETTINGS) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetFlVirtualInterface(row : PMIB_FL_VIRTUAL_INTERFACE_ROW) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetFlVirtualInterfaceTable(family : super::ADDRESS_FAMILY, table : *mut PMIB_FL_VIRTUAL_INTERFACE_TABLE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis", feature = "shared", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIfEntry2(row : PMIB_IF_ROW2) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis", feature = "shared", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIfEntry2Ex(level : MIB_IF_ENTRY_LEVEL, row : PMIB_IF_ROW2) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIfStackTable(table : *mut PMIB_IFSTACK_TABLE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis", feature = "shared", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIfTable2(table : *mut PMIB_IF_TABLE2) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis", feature = "shared", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIfTable2Ex(level : MIB_IF_TABLE_LEVEL, table : *mut PMIB_IF_TABLE2) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceDnsSettings(interface : windows_sys::core::GUID, settings : *mut DNS_INTERFACE_SETTINGS) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn GetInvertedIfStackTable(table : *mut PMIB_INVERTEDIFSTACK_TABLE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpForwardEntry2(row : PMIB_IPFORWARD_ROW2) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpForwardTable2(family : super::ADDRESS_FAMILY, table : *mut PMIB_IPFORWARD_TABLE2) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpInterfaceEntry(row : PMIB_IPINTERFACE_ROW) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpInterfaceTable(family : super::ADDRESS_FAMILY, table : *mut PMIB_IPINTERFACE_TABLE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpNetEntry2(row : PMIB_IPNET_ROW2) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpNetTable2(family : super::ADDRESS_FAMILY, table : *mut PMIB_IPNET_TABLE2) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpNetworkConnectionBandwidthEstimates(interfaceindex : super::NET_IFINDEX, addressfamily : super::ADDRESS_FAMILY, bandwidthestimates : PMIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpPathEntry(row : PMIB_IPPATH_ROW) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpPathTable(family : super::ADDRESS_FAMILY, table : *mut PMIB_IPPATH_TABLE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn GetJobCompartmentId(jobhandle : super::HANDLE) -> super::NET_IF_COMPARTMENT_ID);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetMulticastIpAddressEntry(row : PMIB_MULTICASTIPADDRESS_ROW) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetMulticastIpAddressTable(family : super::ADDRESS_FAMILY, table : *mut PMIB_MULTICASTIPADDRESS_TABLE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "nldef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn GetNetworkConnectivityHint(connectivityhint : *mut super::NL_NETWORK_CONNECTIVITY_HINT) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn GetNetworkConnectivityHintForInterface(interfaceindex : super::NET_IFINDEX, connectivityhint : *mut super::NL_NETWORK_CONNECTIVITY_HINT) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn GetNetworkInformation(networkguid : *const super::NET_IF_NETWORK_GUID, compartmentid : super::PNET_IF_COMPARTMENT_ID, siteid : super::PULONG, networkname : super::PWCHAR, length : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn GetSessionCompartmentId(sessionid : super::ULONG) -> super::NET_IF_COMPARTMENT_ID);
#[cfg(feature = "shared")]
windows_link::link!("iphlpapi.dll" "system" fn GetTeredoPort(port : *mut super::USHORT) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetUnicastIpAddressEntry(row : PMIB_UNICASTIPADDRESS_ROW) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn GetUnicastIpAddressTable(family : super::ADDRESS_FAMILY, table : *mut PMIB_UNICASTIPADDRESS_TABLE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn InitializeFlVirtualInterfaceEntry(row : PMIB_FL_VIRTUAL_INTERFACE_ROW));
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn InitializeIpForwardEntry(row : PMIB_IPFORWARD_ROW2));
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn InitializeIpInterfaceEntry(row : PMIB_IPINTERFACE_ROW));
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn InitializeUnicastIpAddressEntry(row : PMIB_UNICASTIPADDRESS_ROW));
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn NotifyIpInterfaceChange(family : super::ADDRESS_FAMILY, callback : PIPINTERFACE_CHANGE_CALLBACK, callercontext : super::PVOID, initialnotification : super::BOOLEAN, notificationhandle : *mut super::HANDLE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "nldef", feature = "shared", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn NotifyNetworkConnectivityHintChange(callback : PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK, callercontext : super::PVOID, initialnotification : super::BOOLEAN, notificationhandle : super::PHANDLE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn NotifyRouteChange2(addressfamily : super::ADDRESS_FAMILY, callback : PIPFORWARD_CHANGE_CALLBACK, callercontext : super::PVOID, initialnotification : super::BOOLEAN, notificationhandle : *mut super::HANDLE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn NotifyStableUnicastIpAddressTable(family : super::ADDRESS_FAMILY, table : *mut PMIB_UNICASTIPADDRESS_TABLE, callercallback : PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK, callercontext : super::PVOID, notificationhandle : *mut super::HANDLE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn NotifyTeredoPortChange(callback : PTEREDO_PORT_CHANGE_CALLBACK, callercontext : super::PVOID, initialnotification : super::BOOLEAN, notificationhandle : *mut super::HANDLE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn NotifyUnicastIpAddressChange(family : super::ADDRESS_FAMILY, callback : PUNICAST_IPADDRESS_CHANGE_CALLBACK, callercontext : super::PVOID, initialnotification : super::BOOLEAN, notificationhandle : *mut super::HANDLE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn ResolveIpNetEntry2(row : PMIB_IPNET_ROW2, sourceaddress : *const super::SOCKADDR_INET) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn SetCurrentThreadCompartmentId(compartmentid : super::NET_IF_COMPARTMENT_ID) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn SetCurrentThreadCompartmentScope(compartmentscope : super::NET_IF_COMPARTMENT_SCOPE) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("iphlpapi.dll" "system" fn SetDnsSettings(settings : *const DNS_SETTINGS) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn SetFlVirtualInterface(row : *const MIB_FL_VIRTUAL_INTERFACE_ROW) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("iphlpapi.dll" "system" fn SetInterfaceDnsSettings(interface : windows_sys::core::GUID, settings : *const DNS_INTERFACE_SETTINGS) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn SetIpForwardEntry2(route : *const MIB_IPFORWARD_ROW2) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn SetIpInterfaceEntry(row : PMIB_IPINTERFACE_ROW) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn SetIpNetEntry2(row : PMIB_IPNET_ROW2) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn SetJobCompartmentId(jobhandle : super::HANDLE, compartmentid : super::NET_IF_COMPARTMENT_ID) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn SetNetworkInformation(networkguid : *const super::NET_IF_NETWORK_GUID, compartmentid : super::NET_IF_COMPARTMENT_ID, networkname : *const super::WCHAR) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn SetSessionCompartmentId(sessionid : super::ULONG, compartmentid : super::NET_IF_COMPARTMENT_ID) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn SetUnicastIpAddressEntry(row : *const MIB_UNICASTIPADDRESS_ROW) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ifdef", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn if_indextoname(interfaceindex : super::NET_IFINDEX, interfacename : super::PCHAR) -> super::PCHAR);
#[cfg(feature = "ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn if_nametoindex(interfacename : windows_sys::core::PCSTR) -> super::NET_IFINDEX);
pub const ANY_SIZE: i32 = 1;
pub const DNS_DDR_ADAPTER_ENABLE: i32 = 1;
pub const DNS_DDR_ADAPTER_ENABLE_DOH: i32 = 1;
pub const DNS_DDR_ADAPTER_ENABLE_UDP_FALLBACK: i32 = 2;
pub const DNS_DDR_ADAPTER_MAKE_DDR_NON_BLOCKING: i32 = 4;
pub const DNS_DOH_AUTO_UPGRADE_SERVER: i32 = 8;
pub const DNS_DOH_POLICY_AUTO: i32 = 16;
pub const DNS_DOH_POLICY_BLOCK: i32 = 512;
pub const DNS_DOH_POLICY_DISABLE: i32 = 8;
pub const DNS_DOH_POLICY_NOT_CONFIGURED: i32 = 4;
pub const DNS_DOH_POLICY_REQUIRED: i32 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DNS_DOH_SERVER_SETTINGS {
    pub Template: windows_sys::core::PWSTR,
    pub Flags: super::ULONG64,
}
pub const DNS_DOH_SERVER_SETTINGS_ENABLE: i32 = 2;
pub const DNS_DOH_SERVER_SETTINGS_ENABLE_AUTO: i32 = 1;
pub const DNS_DOH_SERVER_SETTINGS_ENABLE_DDR: i32 = 16;
pub const DNS_DOH_SERVER_SETTINGS_FALLBACK_TO_UDP: i32 = 4;
pub const DNS_DOH_SERVER_SETTINGS_MAKE_DDR_NON_BLOCKING: i32 = 32;
pub const DNS_DOT_AUTO_UPGRADE_SERVER: i32 = 4;
pub const DNS_DOT_POLICY_BLOCK: i32 = 256;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DNS_DOT_SERVER_SETTINGS {
    pub Hostname: windows_sys::core::PWSTR,
    pub Flags: super::ULONG64,
    pub Port: super::USHORT,
}
pub const DNS_DOT_SERVER_SETTINGS_ENABLE: i32 = 1;
pub const DNS_DOT_SERVER_SETTINGS_ENABLE_AUTO: i32 = 8;
pub const DNS_DOT_SERVER_SETTINGS_ENABLE_DDR: i32 = 16;
pub const DNS_DOT_SERVER_SETTINGS_FALLBACK_TO_UDP: i32 = 2;
pub const DNS_DOT_SERVER_SETTINGS_MAKE_DDR_NON_BLOCKING: i32 = 32;
pub const DNS_ENABLE_DDR: i32 = 64;
pub const DNS_ENABLE_DNR: i32 = 1024;
pub const DNS_ENABLE_DOH: i32 = 1;
pub const DNS_ENABLE_DOT: i32 = 128;
pub const DNS_ENCRYPTION_POLICY_AUTO: i32 = 16;
pub const DNS_ENCRYPTION_POLICY_DISABLE: i32 = 8;
pub const DNS_ENCRYPTION_POLICY_NOT_CONFIGURED: i32 = 4;
pub const DNS_ENCRYPTION_POLICY_REQUIRED: i32 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DNS_INTERFACE_SETTINGS {
    pub Version: super::ULONG,
    pub Flags: super::ULONG64,
    pub Domain: windows_sys::core::PWSTR,
    pub NameServer: windows_sys::core::PWSTR,
    pub SearchList: windows_sys::core::PWSTR,
    pub RegistrationEnabled: super::ULONG,
    pub RegisterAdapterName: super::ULONG,
    pub EnableLLMNR: super::ULONG,
    pub QueryAdapterName: super::ULONG,
    pub ProfileNameServer: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DNS_INTERFACE_SETTINGS3 {
    pub Version: super::ULONG,
    pub Flags: super::ULONG64,
    pub Domain: windows_sys::core::PWSTR,
    pub NameServer: windows_sys::core::PWSTR,
    pub SearchList: windows_sys::core::PWSTR,
    pub RegistrationEnabled: super::ULONG,
    pub RegisterAdapterName: super::ULONG,
    pub EnableLLMNR: super::ULONG,
    pub QueryAdapterName: super::ULONG,
    pub ProfileNameServer: windows_sys::core::PWSTR,
    pub DisableUnconstrainedQueries: super::ULONG,
    pub SupplementalSearchList: windows_sys::core::PWSTR,
    pub cServerProperties: super::ULONG,
    pub ServerProperties: *mut DNS_SERVER_PROPERTY,
    pub cProfileServerProperties: super::ULONG,
    pub ProfileServerProperties: *mut DNS_SERVER_PROPERTY,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DNS_INTERFACE_SETTINGS4 {
    pub Version: super::ULONG,
    pub Flags: super::ULONG64,
    pub Domain: windows_sys::core::PWSTR,
    pub NameServer: windows_sys::core::PWSTR,
    pub SearchList: windows_sys::core::PWSTR,
    pub RegistrationEnabled: super::ULONG,
    pub RegisterAdapterName: super::ULONG,
    pub EnableLLMNR: super::ULONG,
    pub QueryAdapterName: super::ULONG,
    pub ProfileNameServer: windows_sys::core::PWSTR,
    pub DisableUnconstrainedQueries: super::ULONG,
    pub SupplementalSearchList: windows_sys::core::PWSTR,
    pub cServerProperties: super::ULONG,
    pub ServerProperties: *mut DNS_SERVER_PROPERTY,
    pub cProfileServerProperties: super::ULONG,
    pub ProfileServerProperties: *mut DNS_SERVER_PROPERTY,
    pub EncryptedDnsAdapterFlags: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DNS_INTERFACE_SETTINGS_EX {
    pub SettingsV1: DNS_INTERFACE_SETTINGS,
    pub DisableUnconstrainedQueries: super::ULONG,
    pub SupplementalSearchList: windows_sys::core::PWSTR,
}
pub const DNS_INTERFACE_SETTINGS_VERSION1: i32 = 1;
pub const DNS_INTERFACE_SETTINGS_VERSION2: i32 = 2;
pub const DNS_INTERFACE_SETTINGS_VERSION3: i32 = 3;
pub const DNS_INTERFACE_SETTINGS_VERSION4: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DNS_SERVER_PROPERTY {
    pub Version: super::ULONG,
    pub ServerIndex: super::ULONG,
    pub Type: DNS_SERVER_PROPERTY_TYPE,
    pub Property: DNS_SERVER_PROPERTY_TYPES,
}
#[cfg(feature = "shared")]
impl Default for DNS_SERVER_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DNS_SERVER_PROPERTY_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DNS_SERVER_PROPERTY_TYPES {
    pub DohSettings: *mut DNS_DOH_SERVER_SETTINGS,
    pub DotSettings: *mut DNS_DOT_SERVER_SETTINGS,
}
#[cfg(feature = "shared")]
impl Default for DNS_SERVER_PROPERTY_TYPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_SERVER_PROPERTY_VERSION1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DNS_SETTINGS {
    pub Version: super::ULONG,
    pub Flags: super::ULONG64,
    pub Hostname: windows_sys::core::PWSTR,
    pub Domain: windows_sys::core::PWSTR,
    pub SearchList: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DNS_SETTINGS2 {
    pub Version: super::ULONG,
    pub Flags: super::ULONG64,
    pub Hostname: windows_sys::core::PWSTR,
    pub Domain: windows_sys::core::PWSTR,
    pub SearchList: windows_sys::core::PWSTR,
    pub SettingFlags: super::ULONG64,
}
pub const DNS_SETTINGS_ENABLE_LLMNR: i32 = 128;
pub const DNS_SETTINGS_QUERY_ADAPTER_NAME: i32 = 256;
pub const DNS_SETTINGS_VERSION1: i32 = 1;
pub const DNS_SETTINGS_VERSION2: i32 = 2;
pub const DNS_SETTING_DDR: i32 = 32768;
pub const DNS_SETTING_DISABLE_UNCONSTRAINED_QUERIES: i32 = 1024;
pub const DNS_SETTING_DOH: i32 = 4096;
pub const DNS_SETTING_DOH_PROFILE: i32 = 8192;
pub const DNS_SETTING_DOMAIN: i32 = 32;
pub const DNS_SETTING_DOT: i32 = 65536;
pub const DNS_SETTING_DOT_PROFILE: i32 = 131072;
pub const DNS_SETTING_ENCRYPTED_DNS_ADAPTER_FLAGS: i32 = 16384;
pub const DNS_SETTING_HOSTNAME: i32 = 64;
pub const DNS_SETTING_IPV6: i32 = 1;
pub const DNS_SETTING_NAMESERVER: i32 = 2;
pub const DNS_SETTING_PROFILE_NAMESERVER: i32 = 512;
pub const DNS_SETTING_REGISTER_ADAPTER_NAME: i32 = 16;
pub const DNS_SETTING_REGISTRATION_ENABLED: i32 = 8;
pub const DNS_SETTING_SEARCHLIST: i32 = 4;
pub const DNS_SETTING_SUPPLEMENTAL_SEARCH_LIST: i32 = 2048;
pub const DnsServerDohProperty: DNS_SERVER_PROPERTY_TYPE = 1;
pub const DnsServerDotProperty: DNS_SERVER_PROPERTY_TYPE = 2;
pub const DnsServerInvalidProperty: DNS_SERVER_PROPERTY_TYPE = 0;
pub const IF_NAMESIZE: i32 = 256;
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct IP_ADDRESS_PREFIX {
    pub Prefix: super::SOCKADDR_INET,
    pub PrefixLength: super::UINT8,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
impl Default for IP_ADDRESS_PREFIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_ANYCASTIPADDRESS_ROW {
    pub Address: super::SOCKADDR_INET,
    pub InterfaceLuid: super::NET_LUID,
    pub InterfaceIndex: super::NET_IFINDEX,
    pub ScopeId: super::SCOPE_ID,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
impl Default for MIB_ANYCASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_ANYCASTIPADDRESS_TABLE {
    pub NumEntries: super::ULONG,
    pub Table: [MIB_ANYCASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
impl Default for MIB_ANYCASTIPADDRESS_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_FL_VIRTUAL_INTERFACE_ROW {
    pub Family: super::ADDRESS_FAMILY,
    pub IfLuid: super::IF_LUID,
    pub VirtualIfId: super::ULONG,
    pub CompartmentGuid: windows_sys::core::GUID,
    pub IsolationMode: NET_FL_ISOLATION_MODE,
    pub Origin: NET_FL_VIRTUAL_INTERFACE_ORIGIN,
    pub VirtualIfLuid: super::IF_LUID,
    pub VirtualIfIndex: super::IF_INDEX,
    pub AllowLocalNd: super::BOOLEAN,
    pub AttachedFlsnpiClients: super::ULONG,
    pub FlsnpiClientConfigErrors: super::ULONG,
    pub FlsnpiClientInjectErrors: super::ULONG64,
    pub FlsnpiClientCloneErrors: super::ULONG64,
    pub InFlsnpiIndicatedPackets: super::ULONG64,
    pub InFlsnpiClientReturnedPackets: super::ULONG64,
    pub InFlsnpiClientSilentlyDroppedPackets: super::ULONG64,
    pub InFlsnpiClientDroppedPackets: super::ULONG64,
    pub InFlsnpiClientInjectedPackets: super::ULONG64,
    pub InFlsnpiClientClonedPackets: super::ULONG64,
    pub OutFlsnpiIndicatedPackets: super::ULONG64,
    pub OutFlsnpiClientReturnedPackets: super::ULONG64,
    pub OutFlsnpiClientDroppedPackets: super::ULONG64,
    pub OutFlsnpiClientSilentlyDroppedPackets: super::ULONG64,
    pub OutFlsnpiClientInjectedPackets: super::ULONG64,
    pub OutFlsnpiClientClonedPackets: super::ULONG64,
    pub OutFlsnpiClientClonedPacketsForNbSplit: super::ULONG64,
}
#[cfg(all(feature = "ifdef", feature = "shared", feature = "ws2"))]
impl Default for MIB_FL_VIRTUAL_INTERFACE_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_FL_VIRTUAL_INTERFACE_TABLE {
    pub NumEntries: super::ULONG,
    pub Table: [MIB_FL_VIRTUAL_INTERFACE_ROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "shared", feature = "ws2"))]
impl Default for MIB_FL_VIRTUAL_INTERFACE_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ifdef")]
#[derive(Clone, Copy, Default)]
pub struct MIB_IFSTACK_ROW {
    pub HigherLayerInterfaceIndex: super::NET_IFINDEX,
    pub LowerLayerInterfaceIndex: super::NET_IFINDEX,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIB_IFSTACK_TABLE {
    pub NumEntries: super::ULONG,
    pub Table: [MIB_IFSTACK_ROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "shared"))]
impl Default for MIB_IFSTACK_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MIB_IF_ENTRY_LEVEL = i32;
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIB_IF_ROW2 {
    pub InterfaceLuid: super::NET_LUID,
    pub InterfaceIndex: super::NET_IFINDEX,
    pub InterfaceGuid: windows_sys::core::GUID,
    pub Alias: [super::WCHAR; 257],
    pub Description: [super::WCHAR; 257],
    pub PhysicalAddressLength: super::ULONG,
    pub PhysicalAddress: [super::UCHAR; 32],
    pub PermanentPhysicalAddress: [super::UCHAR; 32],
    pub Mtu: super::ULONG,
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
    pub TransmitLinkSpeed: super::ULONG64,
    pub ReceiveLinkSpeed: super::ULONG64,
    pub InOctets: super::ULONG64,
    pub InUcastPkts: super::ULONG64,
    pub InNUcastPkts: super::ULONG64,
    pub InDiscards: super::ULONG64,
    pub InErrors: super::ULONG64,
    pub InUnknownProtos: super::ULONG64,
    pub InUcastOctets: super::ULONG64,
    pub InMulticastOctets: super::ULONG64,
    pub InBroadcastOctets: super::ULONG64,
    pub OutOctets: super::ULONG64,
    pub OutUcastPkts: super::ULONG64,
    pub OutNUcastPkts: super::ULONG64,
    pub OutDiscards: super::ULONG64,
    pub OutErrors: super::ULONG64,
    pub OutUcastOctets: super::ULONG64,
    pub OutMulticastOctets: super::ULONG64,
    pub OutBroadcastOctets: super::ULONG64,
    pub OutQLen: super::ULONG64,
}
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis", feature = "shared", feature = "winnt"))]
impl Default for MIB_IF_ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MIB_IF_ROW2_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIB_IF_TABLE2 {
    pub NumEntries: super::ULONG,
    pub Table: [MIB_IF_ROW2; 1],
}
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis", feature = "shared", feature = "winnt"))]
impl Default for MIB_IF_TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MIB_IF_TABLE_LEVEL = i32;
pub const MIB_INVALID_TEREDO_PORT_NUMBER: i32 = 0;
#[repr(C)]
#[cfg(feature = "ifdef")]
#[derive(Clone, Copy, Default)]
pub struct MIB_INVERTEDIFSTACK_ROW {
    pub LowerLayerInterfaceIndex: super::NET_IFINDEX,
    pub HigherLayerInterfaceIndex: super::NET_IFINDEX,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIB_INVERTEDIFSTACK_TABLE {
    pub NumEntries: super::ULONG,
    pub Table: [MIB_INVERTEDIFSTACK_ROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "shared"))]
impl Default for MIB_INVERTEDIFSTACK_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_IPFORWARD_ROW2 {
    pub InterfaceLuid: super::NET_LUID,
    pub InterfaceIndex: super::NET_IFINDEX,
    pub DestinationPrefix: IP_ADDRESS_PREFIX,
    pub NextHop: super::SOCKADDR_INET,
    pub SitePrefixLength: super::UCHAR,
    pub ValidLifetime: super::ULONG,
    pub PreferredLifetime: super::ULONG,
    pub Metric: super::ULONG,
    pub Protocol: super::NL_ROUTE_PROTOCOL,
    pub Loopback: super::BOOLEAN,
    pub AutoconfigureAddress: super::BOOLEAN,
    pub Publish: super::BOOLEAN,
    pub Immortal: super::BOOLEAN,
    pub Age: super::ULONG,
    pub Origin: super::NL_ROUTE_ORIGIN,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
impl Default for MIB_IPFORWARD_ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_IPFORWARD_TABLE2 {
    pub NumEntries: super::ULONG,
    pub Table: [MIB_IPFORWARD_ROW2; 1],
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
impl Default for MIB_IPFORWARD_TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_IPINTERFACE_ROW {
    pub Family: super::ADDRESS_FAMILY,
    pub InterfaceLuid: super::NET_LUID,
    pub InterfaceIndex: super::NET_IFINDEX,
    pub MaxReassemblySize: super::ULONG,
    pub InterfaceIdentifier: super::ULONG64,
    pub MinRouterAdvertisementInterval: super::ULONG,
    pub MaxRouterAdvertisementInterval: super::ULONG,
    pub AdvertisingEnabled: super::BOOLEAN,
    pub ForwardingEnabled: super::BOOLEAN,
    pub WeakHostSend: super::BOOLEAN,
    pub WeakHostReceive: super::BOOLEAN,
    pub UseAutomaticMetric: super::BOOLEAN,
    pub UseNeighborUnreachabilityDetection: super::BOOLEAN,
    pub ManagedAddressConfigurationSupported: super::BOOLEAN,
    pub OtherStatefulConfigurationSupported: super::BOOLEAN,
    pub AdvertiseDefaultRoute: super::BOOLEAN,
    pub RouterDiscoveryBehavior: super::NL_ROUTER_DISCOVERY_BEHAVIOR,
    pub DadTransmits: super::ULONG,
    pub BaseReachableTime: super::ULONG,
    pub RetransmitTime: super::ULONG,
    pub PathMtuDiscoveryTimeout: super::ULONG,
    pub LinkLocalAddressBehavior: super::NL_LINK_LOCAL_ADDRESS_BEHAVIOR,
    pub LinkLocalAddressTimeout: super::ULONG,
    pub ZoneIndices: [super::ULONG; 16],
    pub SitePrefixLength: super::ULONG,
    pub Metric: super::ULONG,
    pub NlMtu: super::ULONG,
    pub Connected: super::BOOLEAN,
    pub SupportsWakeUpPatterns: super::BOOLEAN,
    pub SupportsNeighborDiscovery: super::BOOLEAN,
    pub SupportsRouterDiscovery: super::BOOLEAN,
    pub ReachableTime: super::ULONG,
    pub TransmitOffload: super::NL_INTERFACE_OFFLOAD_ROD,
    pub ReceiveOffload: super::NL_INTERFACE_OFFLOAD_ROD,
    pub DisableDefaultRoutes: super::BOOLEAN,
}
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "ws2"))]
impl Default for MIB_IPINTERFACE_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_IPINTERFACE_TABLE {
    pub NumEntries: super::ULONG,
    pub Table: [MIB_IPINTERFACE_ROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "ws2"))]
impl Default for MIB_IPINTERFACE_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_IPNET_ROW2 {
    pub Address: super::SOCKADDR_INET,
    pub InterfaceIndex: super::NET_IFINDEX,
    pub InterfaceLuid: super::NET_LUID,
    pub PhysicalAddress: [super::UCHAR; 32],
    pub PhysicalAddressLength: super::ULONG,
    pub State: super::NL_NEIGHBOR_STATE,
    pub Anonymous: MIB_IPNET_ROW2_0,
    pub ReachabilityTime: MIB_IPNET_ROW2_1,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
impl Default for MIB_IPNET_ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union MIB_IPNET_ROW2_0 {
    pub Anonymous: MIB_IPNET_ROW2_0_0,
    pub Flags: super::UCHAR,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
impl Default for MIB_IPNET_ROW2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPNET_ROW2_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union MIB_IPNET_ROW2_1 {
    pub LastReachable: super::ULONG,
    pub LastUnreachable: super::ULONG,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
impl Default for MIB_IPNET_ROW2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_IPNET_TABLE2 {
    pub NumEntries: super::ULONG,
    pub Table: [MIB_IPNET_ROW2; 1],
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
impl Default for MIB_IPNET_TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_IPPATH_ROW {
    pub Source: super::SOCKADDR_INET,
    pub Destination: super::SOCKADDR_INET,
    pub InterfaceLuid: super::NET_LUID,
    pub InterfaceIndex: super::NET_IFINDEX,
    pub CurrentNextHop: super::SOCKADDR_INET,
    pub PathMtu: super::ULONG,
    pub RttMean: super::ULONG,
    pub RttDeviation: super::ULONG,
    pub Anonymous: MIB_IPPATH_ROW_0,
    pub IsReachable: super::BOOLEAN,
    pub LinkTransmitSpeed: super::ULONG64,
    pub LinkReceiveSpeed: super::ULONG64,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
impl Default for MIB_IPPATH_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union MIB_IPPATH_ROW_0 {
    pub LastReachable: super::ULONG,
    pub LastUnreachable: super::ULONG,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
impl Default for MIB_IPPATH_ROW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_IPPATH_TABLE {
    pub NumEntries: super::ULONG,
    pub Table: [MIB_IPPATH_ROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
impl Default for MIB_IPPATH_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "nldef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    pub InboundBandwidthInformation: super::NL_BANDWIDTH_INFORMATION,
    pub OutboundBandwidthInformation: super::NL_BANDWIDTH_INFORMATION,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_MULTICASTIPADDRESS_ROW {
    pub Address: super::SOCKADDR_INET,
    pub InterfaceIndex: super::NET_IFINDEX,
    pub InterfaceLuid: super::NET_LUID,
    pub ScopeId: super::SCOPE_ID,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
impl Default for MIB_MULTICASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_MULTICASTIPADDRESS_TABLE {
    pub NumEntries: super::ULONG,
    pub Table: [MIB_MULTICASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
impl Default for MIB_MULTICASTIPADDRESS_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MIB_NOTIFICATION_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_UNICASTIPADDRESS_ROW {
    pub Address: super::SOCKADDR_INET,
    pub InterfaceLuid: super::NET_LUID,
    pub InterfaceIndex: super::NET_IFINDEX,
    pub PrefixOrigin: super::NL_PREFIX_ORIGIN,
    pub SuffixOrigin: super::NL_SUFFIX_ORIGIN,
    pub ValidLifetime: super::ULONG,
    pub PreferredLifetime: super::ULONG,
    pub OnLinkPrefixLength: super::UINT8,
    pub SkipAsSource: super::BOOLEAN,
    pub DadState: super::NL_DAD_STATE,
    pub ScopeId: super::SCOPE_ID,
    pub CreationTimeStamp: super::LARGE_INTEGER,
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
impl Default for MIB_UNICASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct MIB_UNICASTIPADDRESS_TABLE {
    pub NumEntries: super::ULONG,
    pub Table: [MIB_UNICASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
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
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PIPFORWARD_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: super::PVOID, row: PMIB_IPFORWARD_ROW2, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PIPINTERFACE_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: super::PVOID, row: PMIB_IPINTERFACE_ROW, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
pub type PIP_ADDRESS_PREFIX = *mut IP_ADDRESS_PREFIX;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
pub type PMIB_ANYCASTIPADDRESS_ROW = *mut MIB_ANYCASTIPADDRESS_ROW;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
pub type PMIB_ANYCASTIPADDRESS_TABLE = *mut MIB_ANYCASTIPADDRESS_TABLE;
#[cfg(all(feature = "ifdef", feature = "shared", feature = "ws2"))]
pub type PMIB_FL_VIRTUAL_INTERFACE_ROW = *mut MIB_FL_VIRTUAL_INTERFACE_ROW;
#[cfg(all(feature = "ifdef", feature = "shared", feature = "ws2"))]
pub type PMIB_FL_VIRTUAL_INTERFACE_TABLE = *mut MIB_FL_VIRTUAL_INTERFACE_TABLE;
#[cfg(feature = "ifdef")]
pub type PMIB_IFSTACK_ROW = *mut MIB_IFSTACK_ROW;
#[cfg(all(feature = "ifdef", feature = "shared"))]
pub type PMIB_IFSTACK_TABLE = *mut MIB_IFSTACK_TABLE;
pub type PMIB_IF_ENTRY_LEVEL = *mut MIB_IF_ENTRY_LEVEL;
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis", feature = "shared", feature = "winnt"))]
pub type PMIB_IF_ROW2 = *mut MIB_IF_ROW2;
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "ntddndis", feature = "shared", feature = "winnt"))]
pub type PMIB_IF_TABLE2 = *mut MIB_IF_TABLE2;
pub type PMIB_IF_TABLE_LEVEL = *mut MIB_IF_TABLE_LEVEL;
#[cfg(feature = "ifdef")]
pub type PMIB_INVERTEDIFSTACK_ROW = *mut MIB_INVERTEDIFSTACK_ROW;
#[cfg(all(feature = "ifdef", feature = "shared"))]
pub type PMIB_INVERTEDIFSTACK_TABLE = *mut MIB_INVERTEDIFSTACK_TABLE;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
pub type PMIB_IPFORWARD_ROW2 = *mut MIB_IPFORWARD_ROW2;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
pub type PMIB_IPFORWARD_TABLE2 = *mut MIB_IPFORWARD_TABLE2;
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "ws2"))]
pub type PMIB_IPINTERFACE_ROW = *mut MIB_IPINTERFACE_ROW;
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "ws2"))]
pub type PMIB_IPINTERFACE_TABLE = *mut MIB_IPINTERFACE_TABLE;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
pub type PMIB_IPNET_ROW2 = *mut MIB_IPNET_ROW2;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "ws2"))]
pub type PMIB_IPNET_TABLE2 = *mut MIB_IPNET_TABLE2;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
pub type PMIB_IPPATH_ROW = *mut MIB_IPPATH_ROW;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
pub type PMIB_IPPATH_TABLE = *mut MIB_IPPATH_TABLE;
#[cfg(all(feature = "nldef", feature = "shared"))]
pub type PMIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES = *mut MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
pub type PMIB_MULTICASTIPADDRESS_ROW = *mut MIB_MULTICASTIPADDRESS_ROW;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "shared", feature = "ws2"))]
pub type PMIB_MULTICASTIPADDRESS_TABLE = *mut MIB_MULTICASTIPADDRESS_TABLE;
pub type PMIB_NOTIFICATION_TYPE = *mut MIB_NOTIFICATION_TYPE;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PMIB_UNICASTIPADDRESS_ROW = *mut MIB_UNICASTIPADDRESS_ROW;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PMIB_UNICASTIPADDRESS_TABLE = *mut MIB_UNICASTIPADDRESS_TABLE;
#[cfg(all(feature = "nldef", feature = "shared", feature = "winnt"))]
pub type PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: super::PVOID, connectivityhint: super::NL_NETWORK_CONNECTIVITY_HINT)>;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK = Option<unsafe extern "system" fn(callercontext: super::PVOID, addresstable: PMIB_UNICASTIPADDRESS_TABLE)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTEREDO_PORT_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: super::PVOID, port: super::USHORT, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[cfg(all(feature = "ifdef", feature = "in6addr", feature = "inaddr", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PUNICAST_IPADDRESS_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: super::PVOID, row: PMIB_UNICASTIPADDRESS_ROW, notificationtype: MIB_NOTIFICATION_TYPE)>;
