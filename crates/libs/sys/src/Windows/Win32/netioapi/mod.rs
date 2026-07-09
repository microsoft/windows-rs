#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn CancelMibChangeNotify2(notificationhandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("iphlpapi.dll" "system" fn ConvertCompartmentGuidToId(compartmentguid : *const windows_sys::core::GUID, compartmentid : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertCompartmentIdToGuid(compartmentid : super::ifdef::NET_IF_COMPARTMENT_ID, compartmentguid : *mut windows_sys::core::GUID) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceAliasToLuid(interfacealias : windows_sys::core::PCWSTR, interfaceluid : *mut super::ifdef::NET_LUID) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceGuidToLuid(interfaceguid : *const windows_sys::core::GUID, interfaceluid : *mut super::ifdef::NET_LUID) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceIndexToLuid(interfaceindex : super::ifdef::NET_IFINDEX, interfaceluid : *mut super::ifdef::NET_LUID) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToAlias(interfaceluid : *const super::ifdef::NET_LUID, interfacealias : windows_sys::core::PWSTR, length : usize) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToGuid(interfaceluid : *const super::ifdef::NET_LUID, interfaceguid : *mut windows_sys::core::GUID) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToIndex(interfaceluid : *const super::ifdef::NET_LUID, interfaceindex : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToNameA(interfaceluid : *const super::ifdef::NET_LUID, interfacename : windows_sys::core::PSTR, length : usize) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceLuidToNameW(interfaceluid : *const super::ifdef::NET_LUID, interfacename : windows_sys::core::PWSTR, length : usize) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceNameToLuidA(interfacename : windows_sys::core::PCSTR, interfaceluid : *mut super::ifdef::NET_LUID) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn ConvertInterfaceNameToLuidW(interfacename : windows_sys::core::PCWSTR, interfaceluid : *mut super::ifdef::NET_LUID) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("iphlpapi.dll" "system" fn ConvertIpv4MaskToLength(mask : u32, masklength : *mut u8) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("iphlpapi.dll" "system" fn ConvertLengthToIpv4Mask(masklength : u32, mask : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn CreateAnycastIpAddressEntry(row : *const MIB_ANYCASTIPADDRESS_ROW) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_ws2def"))]
windows_link::link!("iphlpapi.dll" "system" fn CreateFlVirtualInterface(row : *const MIB_FL_VIRTUAL_INTERFACE_ROW) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn CreateIpForwardEntry2(row : *const MIB_IPFORWARD_ROW2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn CreateIpNetEntry2(row : *const MIB_IPNET_ROW2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_in6addr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn CreateSortedAddressPairs(sourceaddresslist : *const super::ws2ipdef::SOCKADDR_IN6_LH, sourceaddresscount : u32, destinationaddresslist : *const super::ws2ipdef::SOCKADDR_IN6_LH, destinationaddresscount : u32, addresssortoptions : u32, sortedaddresspairlist : *mut super::ws2ipdef::PSOCKADDR_IN6_PAIR, sortedaddresspaircount : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn CreateUnicastIpAddressEntry(row : *const MIB_UNICASTIPADDRESS_ROW) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn DeleteAnycastIpAddressEntry(row : *const MIB_ANYCASTIPADDRESS_ROW) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_ws2def"))]
windows_link::link!("iphlpapi.dll" "system" fn DeleteFlVirtualInterface(row : *const MIB_FL_VIRTUAL_INTERFACE_ROW) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn DeleteIpForwardEntry2(row : *const MIB_IPFORWARD_ROW2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn DeleteIpNetEntry2(row : *const MIB_IPNET_ROW2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn DeleteUnicastIpAddressEntry(row : *const MIB_UNICASTIPADDRESS_ROW) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_ws2def"))]
windows_link::link!("iphlpapi.dll" "system" fn FlushIpNetTable2(family : super::ws2def::ADDRESS_FAMILY, interfaceindex : super::ifdef::NET_IFINDEX) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ws2def"))]
windows_link::link!("iphlpapi.dll" "system" fn FlushIpPathTable(family : super::ws2def::ADDRESS_FAMILY) -> super::bcrypt::NTSTATUS);
windows_link::link!("iphlpapi.dll" "system" fn FreeDnsSettings(settings : *mut DNS_SETTINGS));
windows_link::link!("iphlpapi.dll" "system" fn FreeInterfaceDnsSettings(settings : *mut DNS_INTERFACE_SETTINGS));
windows_link::link!("iphlpapi.dll" "system" fn FreeMibTable(memory : *const core::ffi::c_void));
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetAnycastIpAddressEntry(row : *mut MIB_ANYCASTIPADDRESS_ROW) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetAnycastIpAddressTable(family : super::ws2def::ADDRESS_FAMILY, table : *mut PMIB_ANYCASTIPADDRESS_TABLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ws2def"))]
windows_link::link!("iphlpapi.dll" "system" fn GetBestInterfaceEx(destinationaddress : *const super::ws2def::SOCKADDR, bestifindex : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetBestRoute2(interfaceluid : *const super::ifdef::NET_LUID, interfaceindex : super::ifdef::NET_IFINDEX, sourceaddress : *const super::ws2ipdef::SOCKADDR_INET, destinationaddress : *const super::ws2ipdef::SOCKADDR_INET, addresssortoptions : u32, bestroute : *mut MIB_IPFORWARD_ROW2, bestsourceaddress : *mut super::ws2ipdef::SOCKADDR_INET) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn GetCurrentThreadCompartmentId() -> super::ifdef::NET_IF_COMPARTMENT_ID);
windows_link::link!("iphlpapi.dll" "system" fn GetCurrentThreadCompartmentScope(compartmentscope : *mut u32, compartmentid : *mut u32));
#[cfg(feature = "Win32_ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn GetDefaultCompartmentId() -> super::ifdef::NET_IF_COMPARTMENT_ID);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("iphlpapi.dll" "system" fn GetDnsSettings(settings : *mut DNS_SETTINGS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_ws2def"))]
windows_link::link!("iphlpapi.dll" "system" fn GetFlVirtualInterface(row : *mut MIB_FL_VIRTUAL_INTERFACE_ROW) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_ws2def"))]
windows_link::link!("iphlpapi.dll" "system" fn GetFlVirtualInterfaceTable(family : super::ws2def::ADDRESS_FAMILY, table : *mut PMIB_FL_VIRTUAL_INTERFACE_TABLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_ipifcons", feature = "Win32_ntddndis"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIfEntry2(row : *mut MIB_IF_ROW2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_ipifcons", feature = "Win32_ntddndis"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIfEntry2Ex(level : MIB_IF_ENTRY_LEVEL, row : *mut MIB_IF_ROW2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIfStackTable(table : *mut PMIB_IFSTACK_TABLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_ipifcons", feature = "Win32_ntddndis"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIfTable2(table : *mut PMIB_IF_TABLE2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_ipifcons", feature = "Win32_ntddndis"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIfTable2Ex(level : MIB_IF_TABLE_LEVEL, table : *mut PMIB_IF_TABLE2) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("iphlpapi.dll" "system" fn GetInterfaceDnsSettings(interface : windows_sys::core::GUID, settings : *mut DNS_INTERFACE_SETTINGS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetInvertedIfStackTable(table : *mut PMIB_INVERTEDIFSTACK_TABLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpForwardEntry2(row : *mut MIB_IPFORWARD_ROW2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpForwardTable2(family : super::ws2def::ADDRESS_FAMILY, table : *mut PMIB_IPFORWARD_TABLE2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_nldef", feature = "Win32_ws2def"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpInterfaceEntry(row : *mut MIB_IPINTERFACE_ROW) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_nldef", feature = "Win32_ws2def"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpInterfaceTable(family : super::ws2def::ADDRESS_FAMILY, table : *mut PMIB_IPINTERFACE_TABLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpNetEntry2(row : *mut MIB_IPNET_ROW2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpNetTable2(family : super::ws2def::ADDRESS_FAMILY, table : *mut PMIB_IPNET_TABLE2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_nldef", feature = "Win32_ws2def"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpNetworkConnectionBandwidthEstimates(interfaceindex : super::ifdef::NET_IFINDEX, addressfamily : super::ws2def::ADDRESS_FAMILY, bandwidthestimates : *mut MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpPathEntry(row : *mut MIB_IPPATH_ROW) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetIpPathTable(family : super::ws2def::ADDRESS_FAMILY, table : *mut PMIB_IPPATH_TABLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn GetJobCompartmentId(jobhandle : super::winnt::HANDLE) -> super::ifdef::NET_IF_COMPARTMENT_ID);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetMulticastIpAddressEntry(row : *mut MIB_MULTICASTIPADDRESS_ROW) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetMulticastIpAddressTable(family : super::ws2def::ADDRESS_FAMILY, table : *mut PMIB_MULTICASTIPADDRESS_TABLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_nldef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetNetworkConnectivityHint(connectivityhint : *mut super::nldef::NL_NETWORK_CONNECTIVITY_HINT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_nldef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetNetworkConnectivityHintForInterface(interfaceindex : super::ifdef::NET_IFINDEX, connectivityhint : *mut super::nldef::NL_NETWORK_CONNECTIVITY_HINT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetNetworkInformation(networkguid : *const super::ifdef::NET_IF_NETWORK_GUID, compartmentid : *mut u32, siteid : *mut u32, networkname : *mut u16, length : u32) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn GetSessionCompartmentId(sessionid : u32) -> super::ifdef::NET_IF_COMPARTMENT_ID);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("iphlpapi.dll" "system" fn GetTeredoPort(port : *mut u16) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetUnicastIpAddressEntry(row : *mut MIB_UNICASTIPADDRESS_ROW) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn GetUnicastIpAddressTable(family : super::ws2def::ADDRESS_FAMILY, table : *mut PMIB_UNICASTIPADDRESS_TABLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ws2def"))]
windows_link::link!("iphlpapi.dll" "C" fn InitializeFlVirtualInterfaceEntry(row : *mut MIB_FL_VIRTUAL_INTERFACE_ROW));
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn InitializeIpForwardEntry(row : *mut MIB_IPFORWARD_ROW2));
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_nldef", feature = "Win32_ws2def"))]
windows_link::link!("iphlpapi.dll" "system" fn InitializeIpInterfaceEntry(row : *mut MIB_IPINTERFACE_ROW));
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn InitializeUnicastIpAddressEntry(row : *mut MIB_UNICASTIPADDRESS_ROW));
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_nldef", feature = "Win32_winnt", feature = "Win32_ws2def"))]
windows_link::link!("iphlpapi.dll" "system" fn NotifyIpInterfaceChange(family : super::ws2def::ADDRESS_FAMILY, callback : PIPINTERFACE_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, initialnotification : bool, notificationhandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_nldef", feature = "Win32_winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn NotifyNetworkConnectivityHintChange(callback : PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, initialnotification : bool, notificationhandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_winnt", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn NotifyRouteChange2(addressfamily : super::ws2def::ADDRESS_FAMILY, callback : PIPFORWARD_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, initialnotification : bool, notificationhandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_winnt", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn NotifyStableUnicastIpAddressTable(family : super::ws2def::ADDRESS_FAMILY, table : *mut PMIB_UNICASTIPADDRESS_TABLE, callercallback : PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK, callercontext : *const core::ffi::c_void, notificationhandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn NotifyTeredoPortChange(callback : PTEREDO_PORT_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, initialnotification : bool, notificationhandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_winnt", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn NotifyUnicastIpAddressChange(family : super::ws2def::ADDRESS_FAMILY, callback : PUNICAST_IPADDRESS_CHANGE_CALLBACK, callercontext : *const core::ffi::c_void, initialnotification : bool, notificationhandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn ResolveIpNetEntry2(row : *mut MIB_IPNET_ROW2, sourceaddress : *const super::ws2ipdef::SOCKADDR_INET) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn SetCurrentThreadCompartmentId(compartmentid : super::ifdef::NET_IF_COMPARTMENT_ID) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn SetCurrentThreadCompartmentScope(compartmentscope : super::ifdef::NET_IF_COMPARTMENT_SCOPE) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("iphlpapi.dll" "system" fn SetDnsSettings(settings : *const DNS_SETTINGS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_ws2def"))]
windows_link::link!("iphlpapi.dll" "system" fn SetFlVirtualInterface(row : *const MIB_FL_VIRTUAL_INTERFACE_ROW) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("iphlpapi.dll" "system" fn SetInterfaceDnsSettings(interface : windows_sys::core::GUID, settings : *const DNS_INTERFACE_SETTINGS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn SetIpForwardEntry2(route : *const MIB_IPFORWARD_ROW2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_nldef", feature = "Win32_ws2def"))]
windows_link::link!("iphlpapi.dll" "system" fn SetIpInterfaceEntry(row : *mut MIB_IPINTERFACE_ROW) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn SetIpNetEntry2(row : *const MIB_IPNET_ROW2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn SetJobCompartmentId(jobhandle : super::winnt::HANDLE, compartmentid : super::ifdef::NET_IF_COMPARTMENT_ID) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn SetNetworkInformation(networkguid : *const super::ifdef::NET_IF_NETWORK_GUID, compartmentid : super::ifdef::NET_IF_COMPARTMENT_ID, networkname : windows_sys::core::PCWSTR) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef"))]
windows_link::link!("iphlpapi.dll" "system" fn SetSessionCompartmentId(sessionid : u32, compartmentid : super::ifdef::NET_IF_COMPARTMENT_ID) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("iphlpapi.dll" "system" fn SetUnicastIpAddressEntry(row : *const MIB_UNICASTIPADDRESS_ROW) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn if_indextoname(interfaceindex : super::ifdef::NET_IFINDEX, interfacename : *mut i8) -> super::winnt::PCHAR);
#[cfg(feature = "Win32_ifdef")]
windows_link::link!("iphlpapi.dll" "system" fn if_nametoindex(interfacename : windows_sys::core::PCSTR) -> super::ifdef::NET_IFINDEX);
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
#[derive(Clone, Copy)]
pub struct DNS_DOH_SERVER_SETTINGS {
    pub Template: windows_sys::core::PWSTR,
    pub Flags: u64,
}
impl Default for DNS_DOH_SERVER_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_DOH_SERVER_SETTINGS_ENABLE: u32 = 2;
pub const DNS_DOH_SERVER_SETTINGS_ENABLE_AUTO: u32 = 1;
pub const DNS_DOH_SERVER_SETTINGS_ENABLE_DDR: u32 = 16;
pub const DNS_DOH_SERVER_SETTINGS_FALLBACK_TO_UDP: u32 = 4;
pub const DNS_DOH_SERVER_SETTINGS_MAKE_DDR_NON_BLOCKING: u32 = 32;
pub const DNS_DOT_AUTO_UPGRADE_SERVER: u32 = 4;
pub const DNS_DOT_POLICY_BLOCK: u32 = 256;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_DOT_SERVER_SETTINGS {
    pub Hostname: windows_sys::core::PWSTR,
    pub Flags: u64,
    pub Port: u16,
}
impl Default for DNS_DOT_SERVER_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[derive(Clone, Copy)]
pub struct DNS_INTERFACE_SETTINGS {
    pub Version: u32,
    pub Flags: u64,
    pub Domain: windows_sys::core::PWSTR,
    pub NameServer: windows_sys::core::PWSTR,
    pub SearchList: windows_sys::core::PWSTR,
    pub RegistrationEnabled: u32,
    pub RegisterAdapterName: u32,
    pub EnableLLMNR: u32,
    pub QueryAdapterName: u32,
    pub ProfileNameServer: windows_sys::core::PWSTR,
}
impl Default for DNS_INTERFACE_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_INTERFACE_SETTINGS3 {
    pub Version: u32,
    pub Flags: u64,
    pub Domain: windows_sys::core::PWSTR,
    pub NameServer: windows_sys::core::PWSTR,
    pub SearchList: windows_sys::core::PWSTR,
    pub RegistrationEnabled: u32,
    pub RegisterAdapterName: u32,
    pub EnableLLMNR: u32,
    pub QueryAdapterName: u32,
    pub ProfileNameServer: windows_sys::core::PWSTR,
    pub DisableUnconstrainedQueries: u32,
    pub SupplementalSearchList: windows_sys::core::PWSTR,
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
#[derive(Clone, Copy)]
pub struct DNS_INTERFACE_SETTINGS4 {
    pub Version: u32,
    pub Flags: u64,
    pub Domain: windows_sys::core::PWSTR,
    pub NameServer: windows_sys::core::PWSTR,
    pub SearchList: windows_sys::core::PWSTR,
    pub RegistrationEnabled: u32,
    pub RegisterAdapterName: u32,
    pub EnableLLMNR: u32,
    pub QueryAdapterName: u32,
    pub ProfileNameServer: windows_sys::core::PWSTR,
    pub DisableUnconstrainedQueries: u32,
    pub SupplementalSearchList: windows_sys::core::PWSTR,
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
#[derive(Clone, Copy)]
pub struct DNS_INTERFACE_SETTINGS_EX {
    pub SettingsV1: DNS_INTERFACE_SETTINGS,
    pub DisableUnconstrainedQueries: u32,
    pub SupplementalSearchList: windows_sys::core::PWSTR,
}
impl Default for DNS_INTERFACE_SETTINGS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[derive(Clone, Copy)]
pub struct DNS_SETTINGS {
    pub Version: u32,
    pub Flags: u64,
    pub Hostname: windows_sys::core::PWSTR,
    pub Domain: windows_sys::core::PWSTR,
    pub SearchList: windows_sys::core::PWSTR,
}
impl Default for DNS_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_SETTINGS2 {
    pub Version: u32,
    pub Flags: u64,
    pub Hostname: windows_sys::core::PWSTR,
    pub Domain: windows_sys::core::PWSTR,
    pub SearchList: windows_sys::core::PWSTR,
    pub SettingFlags: u64,
}
impl Default for DNS_SETTINGS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct IP_ADDRESS_PREFIX {
    pub Prefix: super::ws2ipdef::SOCKADDR_INET,
    pub PrefixLength: u8,
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for IP_ADDRESS_PREFIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct MIB_ANYCASTIPADDRESS_ROW {
    pub Address: super::ws2ipdef::SOCKADDR_INET,
    pub InterfaceLuid: super::ifdef::NET_LUID,
    pub InterfaceIndex: super::ifdef::NET_IFINDEX,
    pub ScopeId: super::ws2def::SCOPE_ID,
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for MIB_ANYCASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct MIB_ANYCASTIPADDRESS_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_ANYCASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for MIB_ANYCASTIPADDRESS_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ws2def"))]
#[derive(Clone, Copy)]
pub struct MIB_FL_VIRTUAL_INTERFACE_ROW {
    pub Family: super::ws2def::ADDRESS_FAMILY,
    pub IfLuid: super::ifdef::IF_LUID,
    pub VirtualIfId: u32,
    pub CompartmentGuid: windows_sys::core::GUID,
    pub IsolationMode: NET_FL_ISOLATION_MODE,
    pub Origin: NET_FL_VIRTUAL_INTERFACE_ORIGIN,
    pub VirtualIfLuid: super::ifdef::IF_LUID,
    pub VirtualIfIndex: super::ifdef::IF_INDEX,
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
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ws2def"))]
impl Default for MIB_FL_VIRTUAL_INTERFACE_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ws2def"))]
#[derive(Clone, Copy)]
pub struct MIB_FL_VIRTUAL_INTERFACE_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_FL_VIRTUAL_INTERFACE_ROW; 1],
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ws2def"))]
impl Default for MIB_FL_VIRTUAL_INTERFACE_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ifdef")]
#[derive(Clone, Copy, Default)]
pub struct MIB_IFSTACK_ROW {
    pub HigherLayerInterfaceIndex: super::ifdef::NET_IFINDEX,
    pub LowerLayerInterfaceIndex: super::ifdef::NET_IFINDEX,
}
#[repr(C)]
#[cfg(feature = "Win32_ifdef")]
#[derive(Clone, Copy)]
pub struct MIB_IFSTACK_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_IFSTACK_ROW; 1],
}
#[cfg(feature = "Win32_ifdef")]
impl Default for MIB_IFSTACK_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MIB_IF_ENTRY_LEVEL = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ipifcons", feature = "Win32_ntddndis"))]
#[derive(Clone, Copy)]
pub struct MIB_IF_ROW2 {
    pub InterfaceLuid: super::ifdef::NET_LUID,
    pub InterfaceIndex: super::ifdef::NET_IFINDEX,
    pub InterfaceGuid: windows_sys::core::GUID,
    pub Alias: [u16; 257],
    pub Description: [u16; 257],
    pub PhysicalAddressLength: u32,
    pub PhysicalAddress: [u8; 32],
    pub PermanentPhysicalAddress: [u8; 32],
    pub Mtu: u32,
    pub Type: super::ipifcons::IFTYPE,
    pub TunnelType: super::ifdef::TUNNEL_TYPE,
    pub MediaType: super::ntddndis::NDIS_MEDIUM,
    pub PhysicalMediumType: super::ntddndis::NDIS_PHYSICAL_MEDIUM,
    pub AccessType: super::ifdef::NET_IF_ACCESS_TYPE,
    pub DirectionType: super::ifdef::NET_IF_DIRECTION_TYPE,
    pub InterfaceAndOperStatusFlags: MIB_IF_ROW2_0,
    pub OperStatus: super::ifdef::IF_OPER_STATUS,
    pub AdminStatus: super::ifdef::NET_IF_ADMIN_STATUS,
    pub MediaConnectState: super::ifdef::NET_IF_MEDIA_CONNECT_STATE,
    pub NetworkGuid: super::ifdef::NET_IF_NETWORK_GUID,
    pub ConnectionType: super::ifdef::NET_IF_CONNECTION_TYPE,
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
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ipifcons", feature = "Win32_ntddndis"))]
impl Default for MIB_IF_ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ipifcons", feature = "Win32_ntddndis"))]
#[derive(Clone, Copy, Default)]
pub struct MIB_IF_ROW2_0 {
    pub _bitfield: bool,
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ipifcons", feature = "Win32_ntddndis"))]
#[derive(Clone, Copy)]
pub struct MIB_IF_TABLE2 {
    pub NumEntries: u32,
    pub Table: [MIB_IF_ROW2; 1],
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ipifcons", feature = "Win32_ntddndis"))]
impl Default for MIB_IF_TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MIB_IF_TABLE_LEVEL = i32;
pub const MIB_INVALID_TEREDO_PORT_NUMBER: u32 = 0;
#[repr(C)]
#[cfg(feature = "Win32_ifdef")]
#[derive(Clone, Copy, Default)]
pub struct MIB_INVERTEDIFSTACK_ROW {
    pub LowerLayerInterfaceIndex: super::ifdef::NET_IFINDEX,
    pub HigherLayerInterfaceIndex: super::ifdef::NET_IFINDEX,
}
#[repr(C)]
#[cfg(feature = "Win32_ifdef")]
#[derive(Clone, Copy)]
pub struct MIB_INVERTEDIFSTACK_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_INVERTEDIFSTACK_ROW; 1],
}
#[cfg(feature = "Win32_ifdef")]
impl Default for MIB_INVERTEDIFSTACK_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct MIB_IPFORWARD_ROW2 {
    pub InterfaceLuid: super::ifdef::NET_LUID,
    pub InterfaceIndex: super::ifdef::NET_IFINDEX,
    pub DestinationPrefix: IP_ADDRESS_PREFIX,
    pub NextHop: super::ws2ipdef::SOCKADDR_INET,
    pub SitePrefixLength: u8,
    pub ValidLifetime: u32,
    pub PreferredLifetime: u32,
    pub Metric: u32,
    pub Protocol: super::nldef::NL_ROUTE_PROTOCOL,
    pub Loopback: bool,
    pub AutoconfigureAddress: bool,
    pub Publish: bool,
    pub Immortal: bool,
    pub Age: u32,
    pub Origin: super::nldef::NL_ROUTE_ORIGIN,
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for MIB_IPFORWARD_ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct MIB_IPFORWARD_TABLE2 {
    pub NumEntries: u32,
    pub Table: [MIB_IPFORWARD_ROW2; 1],
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for MIB_IPFORWARD_TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_nldef", feature = "Win32_ws2def"))]
#[derive(Clone, Copy)]
pub struct MIB_IPINTERFACE_ROW {
    pub Family: super::ws2def::ADDRESS_FAMILY,
    pub InterfaceLuid: super::ifdef::NET_LUID,
    pub InterfaceIndex: super::ifdef::NET_IFINDEX,
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
    pub RouterDiscoveryBehavior: super::nldef::NL_ROUTER_DISCOVERY_BEHAVIOR,
    pub DadTransmits: u32,
    pub BaseReachableTime: u32,
    pub RetransmitTime: u32,
    pub PathMtuDiscoveryTimeout: u32,
    pub LinkLocalAddressBehavior: super::nldef::NL_LINK_LOCAL_ADDRESS_BEHAVIOR,
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
    pub TransmitOffload: super::nldef::NL_INTERFACE_OFFLOAD_ROD,
    pub ReceiveOffload: super::nldef::NL_INTERFACE_OFFLOAD_ROD,
    pub DisableDefaultRoutes: bool,
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_nldef", feature = "Win32_ws2def"))]
impl Default for MIB_IPINTERFACE_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_nldef", feature = "Win32_ws2def"))]
#[derive(Clone, Copy)]
pub struct MIB_IPINTERFACE_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_IPINTERFACE_ROW; 1],
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_nldef", feature = "Win32_ws2def"))]
impl Default for MIB_IPINTERFACE_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct MIB_IPNET_ROW2 {
    pub Address: super::ws2ipdef::SOCKADDR_INET,
    pub InterfaceIndex: super::ifdef::NET_IFINDEX,
    pub InterfaceLuid: super::ifdef::NET_LUID,
    pub PhysicalAddress: [u8; 32],
    pub PhysicalAddressLength: u32,
    pub State: super::nldef::NL_NEIGHBOR_STATE,
    pub Anonymous: MIB_IPNET_ROW2_0,
    pub ReachabilityTime: MIB_IPNET_ROW2_1,
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for MIB_IPNET_ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub union MIB_IPNET_ROW2_0 {
    pub Anonymous: MIB_IPNET_ROW2_0_0,
    pub Flags: u8,
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for MIB_IPNET_ROW2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPNET_ROW2_0_0 {
    pub _bitfield: bool,
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub union MIB_IPNET_ROW2_1 {
    pub LastReachable: u32,
    pub LastUnreachable: u32,
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for MIB_IPNET_ROW2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct MIB_IPNET_TABLE2 {
    pub NumEntries: u32,
    pub Table: [MIB_IPNET_ROW2; 1],
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for MIB_IPNET_TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct MIB_IPPATH_ROW {
    pub Source: super::ws2ipdef::SOCKADDR_INET,
    pub Destination: super::ws2ipdef::SOCKADDR_INET,
    pub InterfaceLuid: super::ifdef::NET_LUID,
    pub InterfaceIndex: super::ifdef::NET_IFINDEX,
    pub CurrentNextHop: super::ws2ipdef::SOCKADDR_INET,
    pub PathMtu: u32,
    pub RttMean: u32,
    pub RttDeviation: u32,
    pub Anonymous: MIB_IPPATH_ROW_0,
    pub IsReachable: bool,
    pub LinkTransmitSpeed: u64,
    pub LinkReceiveSpeed: u64,
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for MIB_IPPATH_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub union MIB_IPPATH_ROW_0 {
    pub LastReachable: u32,
    pub LastUnreachable: u32,
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for MIB_IPPATH_ROW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct MIB_IPPATH_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_IPPATH_ROW; 1],
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for MIB_IPPATH_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_nldef")]
#[derive(Clone, Copy, Default)]
pub struct MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    pub InboundBandwidthInformation: super::nldef::NL_BANDWIDTH_INFORMATION,
    pub OutboundBandwidthInformation: super::nldef::NL_BANDWIDTH_INFORMATION,
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct MIB_MULTICASTIPADDRESS_ROW {
    pub Address: super::ws2ipdef::SOCKADDR_INET,
    pub InterfaceIndex: super::ifdef::NET_IFINDEX,
    pub InterfaceLuid: super::ifdef::NET_LUID,
    pub ScopeId: super::ws2def::SCOPE_ID,
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for MIB_MULTICASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct MIB_MULTICASTIPADDRESS_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_MULTICASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for MIB_MULTICASTIPADDRESS_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MIB_NOTIFICATION_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct MIB_UNICASTIPADDRESS_ROW {
    pub Address: super::ws2ipdef::SOCKADDR_INET,
    pub InterfaceLuid: super::ifdef::NET_LUID,
    pub InterfaceIndex: super::ifdef::NET_IFINDEX,
    pub PrefixOrigin: super::nldef::NL_PREFIX_ORIGIN,
    pub SuffixOrigin: super::nldef::NL_SUFFIX_ORIGIN,
    pub ValidLifetime: u32,
    pub PreferredLifetime: u32,
    pub OnLinkPrefixLength: u8,
    pub SkipAsSource: bool,
    pub DadState: super::nldef::NL_DAD_STATE,
    pub ScopeId: super::ws2def::SCOPE_ID,
    pub CreationTimeStamp: i64,
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for MIB_UNICASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct MIB_UNICASTIPADDRESS_TABLE {
    pub NumEntries: u32,
    pub Table: [MIB_UNICASTIPADDRESS_ROW; 1],
}
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
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
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PIPFORWARD_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, row: *const MIB_IPFORWARD_ROW2, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_nldef", feature = "Win32_ws2def"))]
pub type PIPINTERFACE_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, row: *const MIB_IPINTERFACE_ROW, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PIP_ADDRESS_PREFIX = *mut IP_ADDRESS_PREFIX;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PMIB_ANYCASTIPADDRESS_ROW = *mut MIB_ANYCASTIPADDRESS_ROW;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PMIB_ANYCASTIPADDRESS_TABLE = *mut MIB_ANYCASTIPADDRESS_TABLE;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ws2def"))]
pub type PMIB_FL_VIRTUAL_INTERFACE_ROW = *mut MIB_FL_VIRTUAL_INTERFACE_ROW;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ws2def"))]
pub type PMIB_FL_VIRTUAL_INTERFACE_TABLE = *mut MIB_FL_VIRTUAL_INTERFACE_TABLE;
#[cfg(feature = "Win32_ifdef")]
pub type PMIB_IFSTACK_ROW = *mut MIB_IFSTACK_ROW;
#[cfg(feature = "Win32_ifdef")]
pub type PMIB_IFSTACK_TABLE = *mut MIB_IFSTACK_TABLE;
pub type PMIB_IF_ENTRY_LEVEL = *mut MIB_IF_ENTRY_LEVEL;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ipifcons", feature = "Win32_ntddndis"))]
pub type PMIB_IF_ROW2 = *mut MIB_IF_ROW2;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_ipifcons", feature = "Win32_ntddndis"))]
pub type PMIB_IF_TABLE2 = *mut MIB_IF_TABLE2;
pub type PMIB_IF_TABLE_LEVEL = *mut MIB_IF_TABLE_LEVEL;
#[cfg(feature = "Win32_ifdef")]
pub type PMIB_INVERTEDIFSTACK_ROW = *mut MIB_INVERTEDIFSTACK_ROW;
#[cfg(feature = "Win32_ifdef")]
pub type PMIB_INVERTEDIFSTACK_TABLE = *mut MIB_INVERTEDIFSTACK_TABLE;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PMIB_IPFORWARD_ROW2 = *mut MIB_IPFORWARD_ROW2;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PMIB_IPFORWARD_TABLE2 = *mut MIB_IPFORWARD_TABLE2;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_nldef", feature = "Win32_ws2def"))]
pub type PMIB_IPINTERFACE_ROW = *mut MIB_IPINTERFACE_ROW;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_nldef", feature = "Win32_ws2def"))]
pub type PMIB_IPINTERFACE_TABLE = *mut MIB_IPINTERFACE_TABLE;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PMIB_IPNET_ROW2 = *mut MIB_IPNET_ROW2;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PMIB_IPNET_TABLE2 = *mut MIB_IPNET_TABLE2;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PMIB_IPPATH_ROW = *mut MIB_IPPATH_ROW;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PMIB_IPPATH_TABLE = *mut MIB_IPPATH_TABLE;
#[cfg(feature = "Win32_nldef")]
pub type PMIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES = *mut MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PMIB_MULTICASTIPADDRESS_ROW = *mut MIB_MULTICASTIPADDRESS_ROW;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PMIB_MULTICASTIPADDRESS_TABLE = *mut MIB_MULTICASTIPADDRESS_TABLE;
pub type PMIB_NOTIFICATION_TYPE = *mut MIB_NOTIFICATION_TYPE;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PMIB_UNICASTIPADDRESS_ROW = *mut MIB_UNICASTIPADDRESS_ROW;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PMIB_UNICASTIPADDRESS_TABLE = *mut MIB_UNICASTIPADDRESS_TABLE;
#[cfg(feature = "Win32_nldef")]
pub type PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, connectivityhint: super::nldef::NL_NETWORK_CONNECTIVITY_HINT)>;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, addresstable: *const MIB_UNICASTIPADDRESS_TABLE)>;
pub type PTEREDO_PORT_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, port: u16, notificationtype: MIB_NOTIFICATION_TYPE)>;
#[cfg(all(feature = "Win32_ifdef", feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_nldef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PUNICAST_IPADDRESS_CHANGE_CALLBACK = Option<unsafe extern "system" fn(callercontext: *const core::ffi::c_void, row: *const MIB_UNICASTIPADDRESS_ROW, notificationtype: MIB_NOTIFICATION_TYPE)>;
