#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnCloseEndpoint(endpoint: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnCloseGuestNetworkService(guestnetworkservice: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnCloseLoadBalancer(loadbalancer: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnCloseNamespace(namespace: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnCloseNetwork(network: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnCreateEndpoint(network: *const ::core::ffi::c_void, id: *const ::windows_sys::core::GUID, settings: super::super::Foundation::PWSTR, endpoint: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnCreateGuestNetworkService(id: *const ::windows_sys::core::GUID, settings: super::super::Foundation::PWSTR, guestnetworkservice: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnCreateLoadBalancer(id: *const ::windows_sys::core::GUID, settings: super::super::Foundation::PWSTR, loadbalancer: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnCreateNamespace(id: *const ::windows_sys::core::GUID, settings: super::super::Foundation::PWSTR, namespace: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnCreateNetwork(id: *const ::windows_sys::core::GUID, settings: super::super::Foundation::PWSTR, network: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnDeleteEndpoint(id: *const ::windows_sys::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnDeleteGuestNetworkService(id: *const ::windows_sys::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnDeleteLoadBalancer(id: *const ::windows_sys::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnDeleteNamespace(id: *const ::windows_sys::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnDeleteNetwork(id: *const ::windows_sys::core::GUID, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnEnumerateEndpoints(query: super::super::Foundation::PWSTR, endpoints: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnEnumerateGuestNetworkPortReservations(returncount: *mut u32, portentries: *mut *mut HCN_PORT_RANGE_ENTRY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnEnumerateLoadBalancers(query: super::super::Foundation::PWSTR, loadbalancer: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnEnumerateNamespaces(query: super::super::Foundation::PWSTR, namespaces: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnEnumerateNetworks(query: super::super::Foundation::PWSTR, networks: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnFreeGuestNetworkPortReservations(portentries: *mut HCN_PORT_RANGE_ENTRY);
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnModifyEndpoint(endpoint: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnModifyGuestNetworkService(guestnetworkservice: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnModifyLoadBalancer(loadbalancer: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnModifyNamespace(namespace: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnModifyNetwork(network: *const ::core::ffi::c_void, settings: super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnOpenEndpoint(id: *const ::windows_sys::core::GUID, endpoint: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnOpenLoadBalancer(id: *const ::windows_sys::core::GUID, loadbalancer: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnOpenNamespace(id: *const ::windows_sys::core::GUID, namespace: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnOpenNetwork(id: *const ::windows_sys::core::GUID, network: *mut *mut ::core::ffi::c_void, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnQueryEndpointProperties(endpoint: *const ::core::ffi::c_void, query: super::super::Foundation::PWSTR, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnQueryLoadBalancerProperties(loadbalancer: *const ::core::ffi::c_void, query: super::super::Foundation::PWSTR, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnQueryNamespaceProperties(namespace: *const ::core::ffi::c_void, query: super::super::Foundation::PWSTR, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnQueryNetworkProperties(network: *const ::core::ffi::c_void, query: super::super::Foundation::PWSTR, properties: *mut super::super::Foundation::PWSTR, errorrecord: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnRegisterGuestNetworkServiceCallback(guestnetworkservice: *const ::core::ffi::c_void, callback: HCN_NOTIFICATION_CALLBACK, context: *const ::core::ffi::c_void, callbackhandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnRegisterServiceCallback(callback: HCN_NOTIFICATION_CALLBACK, context: *const ::core::ffi::c_void, callbackhandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnReleaseGuestNetworkServicePortReservationHandle(portreservationhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnReserveGuestNetworkServicePort(guestnetworkservice: *const ::core::ffi::c_void, protocol: HCN_PORT_PROTOCOL, access: HCN_PORT_ACCESS, port: u16, portreservationhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnReserveGuestNetworkServicePortRange(guestnetworkservice: *const ::core::ffi::c_void, portcount: u16, portrangereservation: *mut HCN_PORT_RANGE_RESERVATION, portreservationhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnUnregisterGuestNetworkServiceCallback(callbackhandle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnUnregisterServiceCallback(callbackhandle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
pub struct HCN_NOTIFICATIONS(i32);
pub struct HCN_NOTIFICATION_CALLBACK(i32);
pub struct HCN_PORT_ACCESS(i32);
pub struct HCN_PORT_PROTOCOL(i32);
pub struct HCN_PORT_RANGE_ENTRY(i32);
pub struct HCN_PORT_RANGE_RESERVATION(i32);
