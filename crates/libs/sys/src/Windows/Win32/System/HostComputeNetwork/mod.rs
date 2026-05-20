pub type HcnCloseEndpoint = unsafe extern "system" fn(endpoint: *const core::ffi::c_void) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnCloseEndpoint(endpoint : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type HcnCloseGuestNetworkService = unsafe extern "system" fn(guestnetworkservice: *const core::ffi::c_void) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnCloseGuestNetworkService(guestnetworkservice : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type HcnCloseLoadBalancer = unsafe extern "system" fn(loadbalancer: *const core::ffi::c_void) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnCloseLoadBalancer(loadbalancer : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type HcnCloseNamespace = unsafe extern "system" fn(namespace: *const core::ffi::c_void) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnCloseNamespace(namespace : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type HcnCloseNetwork = unsafe extern "system" fn(network: *const core::ffi::c_void) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnCloseNetwork(network : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type HcnCreateEndpoint = unsafe extern "system" fn(network: *const core::ffi::c_void, id: *const windows_sys::core::GUID, settings: windows_sys::core::PCWSTR, endpoint: *mut *mut core::ffi::c_void, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnCreateEndpoint(network : *const core::ffi::c_void, id : *const windows_sys::core::GUID, settings : windows_sys::core::PCWSTR, endpoint : *mut *mut core::ffi::c_void, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnCreateGuestNetworkService = unsafe extern "system" fn(id: *const windows_sys::core::GUID, settings: windows_sys::core::PCWSTR, guestnetworkservice: *mut *mut core::ffi::c_void, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnCreateGuestNetworkService(id : *const windows_sys::core::GUID, settings : windows_sys::core::PCWSTR, guestnetworkservice : *mut *mut core::ffi::c_void, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnCreateLoadBalancer = unsafe extern "system" fn(id: *const windows_sys::core::GUID, settings: windows_sys::core::PCWSTR, loadbalancer: *mut *mut core::ffi::c_void, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnCreateLoadBalancer(id : *const windows_sys::core::GUID, settings : windows_sys::core::PCWSTR, loadbalancer : *mut *mut core::ffi::c_void, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnCreateNamespace = unsafe extern "system" fn(id: *const windows_sys::core::GUID, settings: windows_sys::core::PCWSTR, namespace: *mut *mut core::ffi::c_void, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnCreateNamespace(id : *const windows_sys::core::GUID, settings : windows_sys::core::PCWSTR, namespace : *mut *mut core::ffi::c_void, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnCreateNetwork = unsafe extern "system" fn(id: *const windows_sys::core::GUID, settings: windows_sys::core::PCWSTR, network: *mut *mut core::ffi::c_void, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnCreateNetwork(id : *const windows_sys::core::GUID, settings : windows_sys::core::PCWSTR, network : *mut *mut core::ffi::c_void, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnDeleteEndpoint = unsafe extern "system" fn(id: *const windows_sys::core::GUID, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnDeleteEndpoint(id : *const windows_sys::core::GUID, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnDeleteGuestNetworkService = unsafe extern "system" fn(id: *const windows_sys::core::GUID, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnDeleteGuestNetworkService(id : *const windows_sys::core::GUID, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnDeleteLoadBalancer = unsafe extern "system" fn(id: *const windows_sys::core::GUID, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnDeleteLoadBalancer(id : *const windows_sys::core::GUID, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnDeleteNamespace = unsafe extern "system" fn(id: *const windows_sys::core::GUID, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnDeleteNamespace(id : *const windows_sys::core::GUID, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnDeleteNetwork = unsafe extern "system" fn(id: *const windows_sys::core::GUID, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnDeleteNetwork(id : *const windows_sys::core::GUID, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnEnumerateEndpoints = unsafe extern "system" fn(query: windows_sys::core::PCWSTR, endpoints: *mut windows_sys::core::PWSTR, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnEnumerateEndpoints(query : windows_sys::core::PCWSTR, endpoints : *mut windows_sys::core::PWSTR, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnEnumerateGuestNetworkPortReservations = unsafe extern "system" fn(returncount: *mut u32, portentries: *mut *mut HCN_PORT_RANGE_ENTRY) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnEnumerateGuestNetworkPortReservations(returncount : *mut u32, portentries : *mut *mut HCN_PORT_RANGE_ENTRY) -> windows_sys::core::HRESULT);
pub type HcnEnumerateLoadBalancers = unsafe extern "system" fn(query: windows_sys::core::PCWSTR, loadbalancer: *mut windows_sys::core::PWSTR, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnEnumerateLoadBalancers(query : windows_sys::core::PCWSTR, loadbalancer : *mut windows_sys::core::PWSTR, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnEnumerateNamespaces = unsafe extern "system" fn(query: windows_sys::core::PCWSTR, namespaces: *mut windows_sys::core::PWSTR, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnEnumerateNamespaces(query : windows_sys::core::PCWSTR, namespaces : *mut windows_sys::core::PWSTR, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnEnumerateNetworks = unsafe extern "system" fn(query: windows_sys::core::PCWSTR, networks: *mut windows_sys::core::PWSTR, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnEnumerateNetworks(query : windows_sys::core::PCWSTR, networks : *mut windows_sys::core::PWSTR, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnFreeGuestNetworkPortReservations = unsafe extern "system" fn(portentries: *mut HCN_PORT_RANGE_ENTRY);
windows_link::link!("computenetwork.dll" "system" fn HcnFreeGuestNetworkPortReservations(portentries : *mut HCN_PORT_RANGE_ENTRY));
pub type HcnModifyEndpoint = unsafe extern "system" fn(endpoint: *const core::ffi::c_void, settings: windows_sys::core::PCWSTR, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnModifyEndpoint(endpoint : *const core::ffi::c_void, settings : windows_sys::core::PCWSTR, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnModifyGuestNetworkService = unsafe extern "system" fn(guestnetworkservice: *const core::ffi::c_void, settings: windows_sys::core::PCWSTR, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnModifyGuestNetworkService(guestnetworkservice : *const core::ffi::c_void, settings : windows_sys::core::PCWSTR, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnModifyLoadBalancer = unsafe extern "system" fn(loadbalancer: *const core::ffi::c_void, settings: windows_sys::core::PCWSTR, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnModifyLoadBalancer(loadbalancer : *const core::ffi::c_void, settings : windows_sys::core::PCWSTR, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnModifyNamespace = unsafe extern "system" fn(namespace: *const core::ffi::c_void, settings: windows_sys::core::PCWSTR, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnModifyNamespace(namespace : *const core::ffi::c_void, settings : windows_sys::core::PCWSTR, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnModifyNetwork = unsafe extern "system" fn(network: *const core::ffi::c_void, settings: windows_sys::core::PCWSTR, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnModifyNetwork(network : *const core::ffi::c_void, settings : windows_sys::core::PCWSTR, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnOpenEndpoint = unsafe extern "system" fn(id: *const windows_sys::core::GUID, endpoint: *mut *mut core::ffi::c_void, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnOpenEndpoint(id : *const windows_sys::core::GUID, endpoint : *mut *mut core::ffi::c_void, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnOpenLoadBalancer = unsafe extern "system" fn(id: *const windows_sys::core::GUID, loadbalancer: *mut *mut core::ffi::c_void, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnOpenLoadBalancer(id : *const windows_sys::core::GUID, loadbalancer : *mut *mut core::ffi::c_void, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnOpenNamespace = unsafe extern "system" fn(id: *const windows_sys::core::GUID, namespace: *mut *mut core::ffi::c_void, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnOpenNamespace(id : *const windows_sys::core::GUID, namespace : *mut *mut core::ffi::c_void, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnOpenNetwork = unsafe extern "system" fn(id: *const windows_sys::core::GUID, network: *mut *mut core::ffi::c_void, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnOpenNetwork(id : *const windows_sys::core::GUID, network : *mut *mut core::ffi::c_void, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnQueryEndpointAddresses = unsafe extern "system" fn(endpoint: *const core::ffi::c_void, query: windows_sys::core::PCWSTR, addresses: *mut windows_sys::core::PWSTR, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnQueryEndpointAddresses(endpoint : *const core::ffi::c_void, query : windows_sys::core::PCWSTR, addresses : *mut windows_sys::core::PWSTR, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnQueryEndpointProperties = unsafe extern "system" fn(endpoint: *const core::ffi::c_void, query: windows_sys::core::PCWSTR, properties: *mut windows_sys::core::PWSTR, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnQueryEndpointProperties(endpoint : *const core::ffi::c_void, query : windows_sys::core::PCWSTR, properties : *mut windows_sys::core::PWSTR, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnQueryEndpointStats = unsafe extern "system" fn(endpoint: *const core::ffi::c_void, query: windows_sys::core::PCWSTR, stats: *mut windows_sys::core::PWSTR, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnQueryEndpointStats(endpoint : *const core::ffi::c_void, query : windows_sys::core::PCWSTR, stats : *mut windows_sys::core::PWSTR, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnQueryLoadBalancerProperties = unsafe extern "system" fn(loadbalancer: *const core::ffi::c_void, query: windows_sys::core::PCWSTR, properties: *mut windows_sys::core::PWSTR, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnQueryLoadBalancerProperties(loadbalancer : *const core::ffi::c_void, query : windows_sys::core::PCWSTR, properties : *mut windows_sys::core::PWSTR, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnQueryNamespaceProperties = unsafe extern "system" fn(namespace: *const core::ffi::c_void, query: windows_sys::core::PCWSTR, properties: *mut windows_sys::core::PWSTR, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnQueryNamespaceProperties(namespace : *const core::ffi::c_void, query : windows_sys::core::PCWSTR, properties : *mut windows_sys::core::PWSTR, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnQueryNetworkProperties = unsafe extern "system" fn(network: *const core::ffi::c_void, query: windows_sys::core::PCWSTR, properties: *mut windows_sys::core::PWSTR, errorrecord: *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnQueryNetworkProperties(network : *const core::ffi::c_void, query : windows_sys::core::PCWSTR, properties : *mut windows_sys::core::PWSTR, errorrecord : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
pub type HcnRegisterGuestNetworkServiceCallback = unsafe extern "system" fn(guestnetworkservice: *const core::ffi::c_void, callback: HCN_NOTIFICATION_CALLBACK, context: *const core::ffi::c_void, callbackhandle: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnRegisterGuestNetworkServiceCallback(guestnetworkservice : *const core::ffi::c_void, callback : HCN_NOTIFICATION_CALLBACK, context : *const core::ffi::c_void, callbackhandle : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type HcnRegisterServiceCallback = unsafe extern "system" fn(callback: HCN_NOTIFICATION_CALLBACK, context: *const core::ffi::c_void, callbackhandle: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnRegisterServiceCallback(callback : HCN_NOTIFICATION_CALLBACK, context : *const core::ffi::c_void, callbackhandle : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type HcnReleaseGuestNetworkServicePortReservationHandle = unsafe extern "system" fn(portreservationhandle: super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnReleaseGuestNetworkServicePortReservationHandle(portreservationhandle : super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT);
pub type HcnReserveGuestNetworkServicePort = unsafe extern "system" fn(guestnetworkservice: *const core::ffi::c_void, protocol: HCN_PORT_PROTOCOL, access: HCN_PORT_ACCESS, port: u16, portreservationhandle: *mut super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnReserveGuestNetworkServicePort(guestnetworkservice : *const core::ffi::c_void, protocol : HCN_PORT_PROTOCOL, access : HCN_PORT_ACCESS, port : u16, portreservationhandle : *mut super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT);
pub type HcnReserveGuestNetworkServicePortRange = unsafe extern "system" fn(guestnetworkservice: *const core::ffi::c_void, portcount: u16, portrangereservation: *mut HCN_PORT_RANGE_RESERVATION, portreservationhandle: *mut super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnReserveGuestNetworkServicePortRange(guestnetworkservice : *const core::ffi::c_void, portcount : u16, portrangereservation : *mut HCN_PORT_RANGE_RESERVATION, portreservationhandle : *mut super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT);
pub type HcnUnregisterGuestNetworkServiceCallback = unsafe extern "system" fn(callbackhandle: *const core::ffi::c_void) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnUnregisterGuestNetworkServiceCallback(callbackhandle : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type HcnUnregisterServiceCallback = unsafe extern "system" fn(callbackhandle: *const core::ffi::c_void) -> windows_sys::core::HRESULT;
windows_link::link!("computenetwork.dll" "system" fn HcnUnregisterServiceCallback(callbackhandle : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type HCN_NOTIFICATIONS = i32;
pub type HCN_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(notificationtype: u32, context: *const core::ffi::c_void, notificationstatus: windows_sys::core::HRESULT, notificationdata: windows_sys::core::PCWSTR)>;
pub type HCN_PORT_ACCESS = i32;
pub const HCN_PORT_ACCESS_EXCLUSIVE: HCN_PORT_ACCESS = 1i32;
pub const HCN_PORT_ACCESS_SHARED: HCN_PORT_ACCESS = 2i32;
pub type HCN_PORT_PROTOCOL = i32;
pub const HCN_PORT_PROTOCOL_BOTH: HCN_PORT_PROTOCOL = 3i32;
pub const HCN_PORT_PROTOCOL_TCP: HCN_PORT_PROTOCOL = 1i32;
pub const HCN_PORT_PROTOCOL_UDP: HCN_PORT_PROTOCOL = 2i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HCN_PORT_RANGE_ENTRY {
    pub OwningPartitionId: windows_sys::core::GUID,
    pub TargetPartitionId: windows_sys::core::GUID,
    pub Protocol: HCN_PORT_PROTOCOL,
    pub Priority: u64,
    pub ReservationType: u32,
    pub SharingFlags: u32,
    pub DeliveryMode: u32,
    pub StartingPort: u16,
    pub EndingPort: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HCN_PORT_RANGE_RESERVATION {
    pub startingPort: u16,
    pub endingPort: u16,
}
pub const HcnNotificationFlagsReserved: HCN_NOTIFICATIONS = -268435456i32;
pub const HcnNotificationGuestNetworkServiceCreate: HCN_NOTIFICATIONS = 7i32;
pub const HcnNotificationGuestNetworkServiceDelete: HCN_NOTIFICATIONS = 8i32;
pub const HcnNotificationGuestNetworkServiceInterfaceStateChanged: HCN_NOTIFICATIONS = 18i32;
pub const HcnNotificationGuestNetworkServiceStateChanged: HCN_NOTIFICATIONS = 17i32;
pub const HcnNotificationInvalid: HCN_NOTIFICATIONS = 0i32;
pub const HcnNotificationNamespaceCreate: HCN_NOTIFICATIONS = 5i32;
pub const HcnNotificationNamespaceDelete: HCN_NOTIFICATIONS = 6i32;
pub const HcnNotificationNetworkCreate: HCN_NOTIFICATIONS = 2i32;
pub const HcnNotificationNetworkDelete: HCN_NOTIFICATIONS = 4i32;
pub const HcnNotificationNetworkEndpointAttached: HCN_NOTIFICATIONS = 9i32;
pub const HcnNotificationNetworkEndpointDetached: HCN_NOTIFICATIONS = 16i32;
pub const HcnNotificationNetworkPreCreate: HCN_NOTIFICATIONS = 1i32;
pub const HcnNotificationNetworkPreDelete: HCN_NOTIFICATIONS = 3i32;
pub const HcnNotificationServiceDisconnect: HCN_NOTIFICATIONS = 16777216i32;
