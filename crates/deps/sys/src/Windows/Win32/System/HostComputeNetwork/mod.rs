#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn HcnCloseEndpoint(endpoint: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn HcnCloseGuestNetworkService(guestnetworkservice: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn HcnCloseLoadBalancer(loadbalancer: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn HcnCloseNamespace(namespace: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn HcnCloseNetwork(network: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnCreateEndpoint(network: *const ::core::ffi::c_void, id: *const ::windows_sys::core::GUID, settings: super::super::Foundation::PWSTR, endpoint: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnCreateGuestNetworkService(id: *const ::windows_sys::core::GUID, settings: super::super::Foundation::PWSTR, guestnetworkservice: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnCreateLoadBalancer(id: *const ::windows_sys::core::GUID, settings: super::super::Foundation::PWSTR, loadbalancer: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnCreateNamespace(id: *const ::windows_sys::core::GUID, settings: super::super::Foundation::PWSTR, namespace: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnCreateNetwork(id: *const ::windows_sys::core::GUID, settings: super::super::Foundation::PWSTR, network: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnDeleteEndpoint(id: *const ::windows_sys::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnDeleteGuestNetworkService(id: *const ::windows_sys::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnDeleteLoadBalancer(id: *const ::windows_sys::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnDeleteNamespace(id: *const ::windows_sys::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnDeleteNetwork(id: *const ::windows_sys::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnEnumerateEndpoints(query: super::super::Foundation::PWSTR, endpoints: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn HcnEnumerateGuestNetworkPortReservations(returncount: *mut u32, portentries: *mut *mut HCN_PORT_RANGE_ENTRY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnEnumerateLoadBalancers(query: super::super::Foundation::PWSTR, loadbalancer: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnEnumerateNamespaces(query: super::super::Foundation::PWSTR, namespaces: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnEnumerateNetworks(query: super::super::Foundation::PWSTR, networks: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn HcnFreeGuestNetworkPortReservations(portentries: *mut HCN_PORT_RANGE_ENTRY);
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnModifyEndpoint(endpoint: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnModifyGuestNetworkService(guestnetworkservice: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnModifyLoadBalancer(loadbalancer: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnModifyNamespace(namespace: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnModifyNetwork(network: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnOpenEndpoint(id: *const ::windows_sys::core::GUID, endpoint: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnOpenLoadBalancer(id: *const ::windows_sys::core::GUID, loadbalancer: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnOpenNamespace(id: *const ::windows_sys::core::GUID, namespace: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnOpenNetwork(id: *const ::windows_sys::core::GUID, network: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnQueryEndpointProperties(endpoint: *const ::core::ffi::c_void, query: super::super::Foundation::PWSTR, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnQueryLoadBalancerProperties(loadbalancer: *const ::core::ffi::c_void, query: super::super::Foundation::PWSTR, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnQueryNamespaceProperties(namespace: *const ::core::ffi::c_void, query: super::super::Foundation::PWSTR, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnQueryNetworkProperties(network: *const ::core::ffi::c_void, query: super::super::Foundation::PWSTR, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnRegisterGuestNetworkServiceCallback(guestnetworkservice: *const ::core::ffi::c_void, callback: ::core::option::Option<HCN_NOTIFICATION_CALLBACK>, context: *const ::core::ffi::c_void, callbackhandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnRegisterServiceCallback(callback: ::core::option::Option<HCN_NOTIFICATION_CALLBACK>, context: *const ::core::ffi::c_void, callbackhandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnReleaseGuestNetworkServicePortReservationHandle(portreservationhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnReserveGuestNetworkServicePort(guestnetworkservice: *const ::core::ffi::c_void, protocol: HCN_PORT_PROTOCOL, access: HCN_PORT_ACCESS, port: u16, portreservationhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnReserveGuestNetworkServicePortRange(guestnetworkservice: *const ::core::ffi::c_void, portcount: u16, portrangereservation: *mut HCN_PORT_RANGE_RESERVATION, portreservationhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn HcnUnregisterGuestNetworkServiceCallback(callbackhandle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn HcnUnregisterServiceCallback(callbackhandle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
pub type HCN_NOTIFICATIONS = i32;
pub const HcnNotificationInvalid: HCN_NOTIFICATIONS = 0i32;
pub const HcnNotificationNetworkPreCreate: HCN_NOTIFICATIONS = 1i32;
pub const HcnNotificationNetworkCreate: HCN_NOTIFICATIONS = 2i32;
pub const HcnNotificationNetworkPreDelete: HCN_NOTIFICATIONS = 3i32;
pub const HcnNotificationNetworkDelete: HCN_NOTIFICATIONS = 4i32;
pub const HcnNotificationNamespaceCreate: HCN_NOTIFICATIONS = 5i32;
pub const HcnNotificationNamespaceDelete: HCN_NOTIFICATIONS = 6i32;
pub const HcnNotificationGuestNetworkServiceCreate: HCN_NOTIFICATIONS = 7i32;
pub const HcnNotificationGuestNetworkServiceDelete: HCN_NOTIFICATIONS = 8i32;
pub const HcnNotificationNetworkEndpointAttached: HCN_NOTIFICATIONS = 9i32;
pub const HcnNotificationNetworkEndpointDetached: HCN_NOTIFICATIONS = 16i32;
pub const HcnNotificationGuestNetworkServiceStateChanged: HCN_NOTIFICATIONS = 17i32;
pub const HcnNotificationGuestNetworkServiceInterfaceStateChanged: HCN_NOTIFICATIONS = 18i32;
pub const HcnNotificationServiceDisconnect: HCN_NOTIFICATIONS = 16777216i32;
pub const HcnNotificationFlagsReserved: HCN_NOTIFICATIONS = -268435456i32;
#[cfg(feature = "Win32_Foundation")]
pub type HCN_NOTIFICATION_CALLBACK = unsafe extern "system" fn(notificationtype: u32, context: *const ::core::ffi::c_void, notificationstatus: ::windows_sys::core::HRESULT, notificationdata: super::super::Foundation::PWSTR);
pub type HCN_PORT_ACCESS = i32;
pub const HCN_PORT_ACCESS_EXCLUSIVE: HCN_PORT_ACCESS = 1i32;
pub const HCN_PORT_ACCESS_SHARED: HCN_PORT_ACCESS = 2i32;
pub type HCN_PORT_PROTOCOL = i32;
pub const HCN_PORT_PROTOCOL_TCP: HCN_PORT_PROTOCOL = 1i32;
pub const HCN_PORT_PROTOCOL_UDP: HCN_PORT_PROTOCOL = 2i32;
pub const HCN_PORT_PROTOCOL_BOTH: HCN_PORT_PROTOCOL = 3i32;
#[repr(C)]
pub struct HCN_PORT_RANGE_ENTRY {
    pub OwningPartitionId: ::windows_sys::core::GUID,
    pub TargetPartitionId: ::windows_sys::core::GUID,
    pub Protocol: HCN_PORT_PROTOCOL,
    pub Priority: u64,
    pub ReservationType: u32,
    pub SharingFlags: u32,
    pub DeliveryMode: u32,
    pub StartingPort: u16,
    pub EndingPort: u16,
}
impl ::core::marker::Copy for HCN_PORT_RANGE_ENTRY {}
impl ::core::clone::Clone for HCN_PORT_RANGE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct HCN_PORT_RANGE_RESERVATION {
    pub startingPort: u16,
    pub endingPort: u16,
}
impl ::core::marker::Copy for HCN_PORT_RANGE_RESERVATION {}
impl ::core::clone::Clone for HCN_PORT_RANGE_RESERVATION {
    fn clone(&self) -> Self {
        *self
    }
}
