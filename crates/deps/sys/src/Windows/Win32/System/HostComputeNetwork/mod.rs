#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnCloseEndpoint();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnCloseGuestNetworkService();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnCloseLoadBalancer();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnCloseNamespace();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnCloseNetwork();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnCreateEndpoint();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnCreateGuestNetworkService();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnCreateLoadBalancer();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnCreateNamespace();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnCreateNetwork();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnDeleteEndpoint();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnDeleteGuestNetworkService();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnDeleteLoadBalancer();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnDeleteNamespace();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnDeleteNetwork();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnEnumerateEndpoints();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnEnumerateGuestNetworkPortReservations();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnEnumerateLoadBalancers();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnEnumerateNamespaces();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnEnumerateNetworks();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnFreeGuestNetworkPortReservations();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnModifyEndpoint();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnModifyGuestNetworkService();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnModifyLoadBalancer();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnModifyNamespace();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnModifyNetwork();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnOpenEndpoint();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnOpenLoadBalancer();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnOpenNamespace();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnOpenNetwork();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnQueryEndpointProperties();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnQueryLoadBalancerProperties();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnQueryNamespaceProperties();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnQueryNetworkProperties();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnRegisterGuestNetworkServiceCallback();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnRegisterServiceCallback();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnReleaseGuestNetworkServicePortReservationHandle();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnReserveGuestNetworkServicePort();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcnReserveGuestNetworkServicePortRange();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnUnregisterGuestNetworkServiceCallback();
    #[doc = "*Required features: `Win32_System_HostComputeNetwork`*"]
    pub fn HcnUnregisterServiceCallback();
}
