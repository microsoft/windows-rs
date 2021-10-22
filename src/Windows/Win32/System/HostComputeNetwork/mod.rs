#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct HCN_NOTIFICATIONS(pub i32);
pub const HcnNotificationInvalid: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(0i32);
pub const HcnNotificationNetworkPreCreate: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(1i32);
pub const HcnNotificationNetworkCreate: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(2i32);
pub const HcnNotificationNetworkPreDelete: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(3i32);
pub const HcnNotificationNetworkDelete: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(4i32);
pub const HcnNotificationNamespaceCreate: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(5i32);
pub const HcnNotificationNamespaceDelete: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(6i32);
pub const HcnNotificationGuestNetworkServiceCreate: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(7i32);
pub const HcnNotificationGuestNetworkServiceDelete: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(8i32);
pub const HcnNotificationNetworkEndpointAttached: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(9i32);
pub const HcnNotificationNetworkEndpointDetached: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(16i32);
pub const HcnNotificationGuestNetworkServiceStateChanged: HCN_NOTIFICATIONS =
    HCN_NOTIFICATIONS(17i32);
pub const HcnNotificationGuestNetworkServiceInterfaceStateChanged: HCN_NOTIFICATIONS =
    HCN_NOTIFICATIONS(18i32);
pub const HcnNotificationServiceDisconnect: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(16777216i32);
pub const HcnNotificationFlagsReserved: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(-268435456i32);
impl ::std::convert::From<i32> for HCN_NOTIFICATIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HCN_NOTIFICATIONS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type HCN_NOTIFICATION_CALLBACK = unsafe extern "system" fn(
    notificationtype: u32,
    context: *const ::std::ffi::c_void,
    notificationstatus: ::windows::runtime::HRESULT,
    notificationdata: super::super::Foundation::PWSTR,
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct HCN_PORT_ACCESS(pub i32);
pub const HCN_PORT_ACCESS_EXCLUSIVE: HCN_PORT_ACCESS = HCN_PORT_ACCESS(1i32);
pub const HCN_PORT_ACCESS_SHARED: HCN_PORT_ACCESS = HCN_PORT_ACCESS(2i32);
impl ::std::convert::From<i32> for HCN_PORT_ACCESS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HCN_PORT_ACCESS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct HCN_PORT_PROTOCOL(pub i32);
pub const HCN_PORT_PROTOCOL_TCP: HCN_PORT_PROTOCOL = HCN_PORT_PROTOCOL(1i32);
pub const HCN_PORT_PROTOCOL_UDP: HCN_PORT_PROTOCOL = HCN_PORT_PROTOCOL(2i32);
pub const HCN_PORT_PROTOCOL_BOTH: HCN_PORT_PROTOCOL = HCN_PORT_PROTOCOL(3i32);
impl ::std::convert::From<i32> for HCN_PORT_PROTOCOL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HCN_PORT_PROTOCOL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HCN_PORT_RANGE_ENTRY {
    pub OwningPartitionId: ::windows::runtime::GUID,
    pub TargetPartitionId: ::windows::runtime::GUID,
    pub Protocol: HCN_PORT_PROTOCOL,
    pub Priority: u64,
    pub ReservationType: u32,
    pub SharingFlags: u32,
    pub DeliveryMode: u32,
    pub StartingPort: u16,
    pub EndingPort: u16,
}
impl HCN_PORT_RANGE_ENTRY {}
impl ::std::default::Default for HCN_PORT_RANGE_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HCN_PORT_RANGE_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HCN_PORT_RANGE_ENTRY")
            .field("OwningPartitionId", &self.OwningPartitionId)
            .field("TargetPartitionId", &self.TargetPartitionId)
            .field("Protocol", &self.Protocol)
            .field("Priority", &self.Priority)
            .field("ReservationType", &self.ReservationType)
            .field("SharingFlags", &self.SharingFlags)
            .field("DeliveryMode", &self.DeliveryMode)
            .field("StartingPort", &self.StartingPort)
            .field("EndingPort", &self.EndingPort)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HCN_PORT_RANGE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.OwningPartitionId == other.OwningPartitionId
            && self.TargetPartitionId == other.TargetPartitionId
            && self.Protocol == other.Protocol
            && self.Priority == other.Priority
            && self.ReservationType == other.ReservationType
            && self.SharingFlags == other.SharingFlags
            && self.DeliveryMode == other.DeliveryMode
            && self.StartingPort == other.StartingPort
            && self.EndingPort == other.EndingPort
    }
}
impl ::std::cmp::Eq for HCN_PORT_RANGE_ENTRY {}
unsafe impl ::windows::runtime::Abi for HCN_PORT_RANGE_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HCN_PORT_RANGE_RESERVATION {
    pub startingPort: u16,
    pub endingPort: u16,
}
impl HCN_PORT_RANGE_RESERVATION {}
impl ::std::default::Default for HCN_PORT_RANGE_RESERVATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HCN_PORT_RANGE_RESERVATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HCN_PORT_RANGE_RESERVATION")
            .field("startingPort", &self.startingPort)
            .field("endingPort", &self.endingPort)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HCN_PORT_RANGE_RESERVATION {
    fn eq(&self, other: &Self) -> bool {
        self.startingPort == other.startingPort && self.endingPort == other.endingPort
    }
}
impl ::std::cmp::Eq for HCN_PORT_RANGE_RESERVATION {}
unsafe impl ::windows::runtime::Abi for HCN_PORT_RANGE_RESERVATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub unsafe fn HcnCloseEndpoint(
    endpoint: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnCloseEndpoint(endpoint: *const ::std::ffi::c_void)
                -> ::windows::runtime::HRESULT;
        }
        HcnCloseEndpoint(::std::mem::transmute(endpoint)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn HcnCloseGuestNetworkService(
    guestnetworkservice: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnCloseGuestNetworkService(
                guestnetworkservice: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnCloseGuestNetworkService(::std::mem::transmute(guestnetworkservice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn HcnCloseLoadBalancer(
    loadbalancer: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnCloseLoadBalancer(
                loadbalancer: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnCloseLoadBalancer(::std::mem::transmute(loadbalancer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn HcnCloseNamespace(
    namespace: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnCloseNamespace(
                namespace: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnCloseNamespace(::std::mem::transmute(namespace)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn HcnCloseNetwork(
    network: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnCloseNetwork(network: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        HcnCloseNetwork(::std::mem::transmute(network)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnCreateEndpoint<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    network: *const ::std::ffi::c_void,
    id: *const ::windows::runtime::GUID,
    settings: Param2,
    endpoint: *mut *mut ::std::ffi::c_void,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnCreateEndpoint(
                network: *const ::std::ffi::c_void,
                id: *const ::windows::runtime::GUID,
                settings: super::super::Foundation::PWSTR,
                endpoint: *mut *mut ::std::ffi::c_void,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnCreateEndpoint(
            ::std::mem::transmute(network),
            ::std::mem::transmute(id),
            settings.into_param().abi(),
            ::std::mem::transmute(endpoint),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnCreateGuestNetworkService<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    id: *const ::windows::runtime::GUID,
    settings: Param1,
    guestnetworkservice: *mut *mut ::std::ffi::c_void,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnCreateGuestNetworkService(
                id: *const ::windows::runtime::GUID,
                settings: super::super::Foundation::PWSTR,
                guestnetworkservice: *mut *mut ::std::ffi::c_void,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnCreateGuestNetworkService(
            ::std::mem::transmute(id),
            settings.into_param().abi(),
            ::std::mem::transmute(guestnetworkservice),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnCreateLoadBalancer<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    id: *const ::windows::runtime::GUID,
    settings: Param1,
    loadbalancer: *mut *mut ::std::ffi::c_void,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnCreateLoadBalancer(
                id: *const ::windows::runtime::GUID,
                settings: super::super::Foundation::PWSTR,
                loadbalancer: *mut *mut ::std::ffi::c_void,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnCreateLoadBalancer(
            ::std::mem::transmute(id),
            settings.into_param().abi(),
            ::std::mem::transmute(loadbalancer),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnCreateNamespace<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    id: *const ::windows::runtime::GUID,
    settings: Param1,
    namespace: *mut *mut ::std::ffi::c_void,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnCreateNamespace(
                id: *const ::windows::runtime::GUID,
                settings: super::super::Foundation::PWSTR,
                namespace: *mut *mut ::std::ffi::c_void,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnCreateNamespace(
            ::std::mem::transmute(id),
            settings.into_param().abi(),
            ::std::mem::transmute(namespace),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnCreateNetwork<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    id: *const ::windows::runtime::GUID,
    settings: Param1,
    network: *mut *mut ::std::ffi::c_void,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnCreateNetwork(
                id: *const ::windows::runtime::GUID,
                settings: super::super::Foundation::PWSTR,
                network: *mut *mut ::std::ffi::c_void,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnCreateNetwork(
            ::std::mem::transmute(id),
            settings.into_param().abi(),
            ::std::mem::transmute(network),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnDeleteEndpoint(
    id: *const ::windows::runtime::GUID,
) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnDeleteEndpoint(
                id: *const ::windows::runtime::GUID,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        HcnDeleteEndpoint(::std::mem::transmute(id), &mut result__)
            .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnDeleteGuestNetworkService(
    id: *const ::windows::runtime::GUID,
) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnDeleteGuestNetworkService(
                id: *const ::windows::runtime::GUID,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        HcnDeleteGuestNetworkService(::std::mem::transmute(id), &mut result__)
            .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnDeleteLoadBalancer(
    id: *const ::windows::runtime::GUID,
) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnDeleteLoadBalancer(
                id: *const ::windows::runtime::GUID,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        HcnDeleteLoadBalancer(::std::mem::transmute(id), &mut result__)
            .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnDeleteNamespace(
    id: *const ::windows::runtime::GUID,
) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnDeleteNamespace(
                id: *const ::windows::runtime::GUID,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        HcnDeleteNamespace(::std::mem::transmute(id), &mut result__)
            .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnDeleteNetwork(
    id: *const ::windows::runtime::GUID,
) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnDeleteNetwork(
                id: *const ::windows::runtime::GUID,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        HcnDeleteNetwork(::std::mem::transmute(id), &mut result__)
            .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnEnumerateEndpoints<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    query: Param0,
    endpoints: *mut super::super::Foundation::PWSTR,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnEnumerateEndpoints(
                query: super::super::Foundation::PWSTR,
                endpoints: *mut super::super::Foundation::PWSTR,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnEnumerateEndpoints(
            query.into_param().abi(),
            ::std::mem::transmute(endpoints),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn HcnEnumerateGuestNetworkPortReservations(
    returncount: *mut u32,
    portentries: *mut *mut HCN_PORT_RANGE_ENTRY,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnEnumerateGuestNetworkPortReservations(
                returncount: *mut u32,
                portentries: *mut *mut HCN_PORT_RANGE_ENTRY,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnEnumerateGuestNetworkPortReservations(
            ::std::mem::transmute(returncount),
            ::std::mem::transmute(portentries),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnEnumerateLoadBalancers<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    query: Param0,
    loadbalancer: *mut super::super::Foundation::PWSTR,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnEnumerateLoadBalancers(
                query: super::super::Foundation::PWSTR,
                loadbalancer: *mut super::super::Foundation::PWSTR,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnEnumerateLoadBalancers(
            query.into_param().abi(),
            ::std::mem::transmute(loadbalancer),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnEnumerateNamespaces<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    query: Param0,
    namespaces: *mut super::super::Foundation::PWSTR,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnEnumerateNamespaces(
                query: super::super::Foundation::PWSTR,
                namespaces: *mut super::super::Foundation::PWSTR,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnEnumerateNamespaces(
            query.into_param().abi(),
            ::std::mem::transmute(namespaces),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnEnumerateNetworks<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    query: Param0,
    networks: *mut super::super::Foundation::PWSTR,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnEnumerateNetworks(
                query: super::super::Foundation::PWSTR,
                networks: *mut super::super::Foundation::PWSTR,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnEnumerateNetworks(
            query.into_param().abi(),
            ::std::mem::transmute(networks),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn HcnFreeGuestNetworkPortReservations(portentries: *mut HCN_PORT_RANGE_ENTRY) {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnFreeGuestNetworkPortReservations(portentries: *mut HCN_PORT_RANGE_ENTRY);
        }
        ::std::mem::transmute(HcnFreeGuestNetworkPortReservations(::std::mem::transmute(
            portentries,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnModifyEndpoint<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    endpoint: *const ::std::ffi::c_void,
    settings: Param1,
) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnModifyEndpoint(
                endpoint: *const ::std::ffi::c_void,
                settings: super::super::Foundation::PWSTR,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        HcnModifyEndpoint(
            ::std::mem::transmute(endpoint),
            settings.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnModifyGuestNetworkService<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    guestnetworkservice: *const ::std::ffi::c_void,
    settings: Param1,
) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnModifyGuestNetworkService(
                guestnetworkservice: *const ::std::ffi::c_void,
                settings: super::super::Foundation::PWSTR,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        HcnModifyGuestNetworkService(
            ::std::mem::transmute(guestnetworkservice),
            settings.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnModifyLoadBalancer<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    loadbalancer: *const ::std::ffi::c_void,
    settings: Param1,
) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnModifyLoadBalancer(
                loadbalancer: *const ::std::ffi::c_void,
                settings: super::super::Foundation::PWSTR,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        HcnModifyLoadBalancer(
            ::std::mem::transmute(loadbalancer),
            settings.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnModifyNamespace<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    namespace: *const ::std::ffi::c_void,
    settings: Param1,
) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnModifyNamespace(
                namespace: *const ::std::ffi::c_void,
                settings: super::super::Foundation::PWSTR,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        HcnModifyNamespace(
            ::std::mem::transmute(namespace),
            settings.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnModifyNetwork<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    network: *const ::std::ffi::c_void,
    settings: Param1,
) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnModifyNetwork(
                network: *const ::std::ffi::c_void,
                settings: super::super::Foundation::PWSTR,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        HcnModifyNetwork(
            ::std::mem::transmute(network),
            settings.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnOpenEndpoint(
    id: *const ::windows::runtime::GUID,
    endpoint: *mut *mut ::std::ffi::c_void,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnOpenEndpoint(
                id: *const ::windows::runtime::GUID,
                endpoint: *mut *mut ::std::ffi::c_void,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnOpenEndpoint(
            ::std::mem::transmute(id),
            ::std::mem::transmute(endpoint),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnOpenLoadBalancer(
    id: *const ::windows::runtime::GUID,
    loadbalancer: *mut *mut ::std::ffi::c_void,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnOpenLoadBalancer(
                id: *const ::windows::runtime::GUID,
                loadbalancer: *mut *mut ::std::ffi::c_void,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnOpenLoadBalancer(
            ::std::mem::transmute(id),
            ::std::mem::transmute(loadbalancer),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnOpenNamespace(
    id: *const ::windows::runtime::GUID,
    namespace: *mut *mut ::std::ffi::c_void,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnOpenNamespace(
                id: *const ::windows::runtime::GUID,
                namespace: *mut *mut ::std::ffi::c_void,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnOpenNamespace(
            ::std::mem::transmute(id),
            ::std::mem::transmute(namespace),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnOpenNetwork(
    id: *const ::windows::runtime::GUID,
    network: *mut *mut ::std::ffi::c_void,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnOpenNetwork(
                id: *const ::windows::runtime::GUID,
                network: *mut *mut ::std::ffi::c_void,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnOpenNetwork(
            ::std::mem::transmute(id),
            ::std::mem::transmute(network),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnQueryEndpointProperties<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    endpoint: *const ::std::ffi::c_void,
    query: Param1,
    properties: *mut super::super::Foundation::PWSTR,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnQueryEndpointProperties(
                endpoint: *const ::std::ffi::c_void,
                query: super::super::Foundation::PWSTR,
                properties: *mut super::super::Foundation::PWSTR,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnQueryEndpointProperties(
            ::std::mem::transmute(endpoint),
            query.into_param().abi(),
            ::std::mem::transmute(properties),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnQueryLoadBalancerProperties<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    loadbalancer: *const ::std::ffi::c_void,
    query: Param1,
    properties: *mut super::super::Foundation::PWSTR,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnQueryLoadBalancerProperties(
                loadbalancer: *const ::std::ffi::c_void,
                query: super::super::Foundation::PWSTR,
                properties: *mut super::super::Foundation::PWSTR,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnQueryLoadBalancerProperties(
            ::std::mem::transmute(loadbalancer),
            query.into_param().abi(),
            ::std::mem::transmute(properties),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnQueryNamespaceProperties<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    namespace: *const ::std::ffi::c_void,
    query: Param1,
    properties: *mut super::super::Foundation::PWSTR,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnQueryNamespaceProperties(
                namespace: *const ::std::ffi::c_void,
                query: super::super::Foundation::PWSTR,
                properties: *mut super::super::Foundation::PWSTR,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnQueryNamespaceProperties(
            ::std::mem::transmute(namespace),
            query.into_param().abi(),
            ::std::mem::transmute(properties),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnQueryNetworkProperties<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    network: *const ::std::ffi::c_void,
    query: Param1,
    properties: *mut super::super::Foundation::PWSTR,
    errorrecord: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnQueryNetworkProperties(
                network: *const ::std::ffi::c_void,
                query: super::super::Foundation::PWSTR,
                properties: *mut super::super::Foundation::PWSTR,
                errorrecord: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnQueryNetworkProperties(
            ::std::mem::transmute(network),
            query.into_param().abi(),
            ::std::mem::transmute(properties),
            ::std::mem::transmute(errorrecord),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnRegisterGuestNetworkServiceCallback(
    guestnetworkservice: *const ::std::ffi::c_void,
    callback: ::std::option::Option<HCN_NOTIFICATION_CALLBACK>,
    context: *const ::std::ffi::c_void,
    callbackhandle: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnRegisterGuestNetworkServiceCallback(
                guestnetworkservice: *const ::std::ffi::c_void,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                callbackhandle: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnRegisterGuestNetworkServiceCallback(
            ::std::mem::transmute(guestnetworkservice),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(callbackhandle),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnRegisterServiceCallback(
    callback: ::std::option::Option<HCN_NOTIFICATION_CALLBACK>,
    context: *const ::std::ffi::c_void,
    callbackhandle: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnRegisterServiceCallback(
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                callbackhandle: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnRegisterServiceCallback(
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(callbackhandle),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnReleaseGuestNetworkServicePortReservationHandle<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    portreservationhandle: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnReleaseGuestNetworkServicePortReservationHandle(
                portreservationhandle: super::super::Foundation::HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnReleaseGuestNetworkServicePortReservationHandle(portreservationhandle.into_param().abi())
            .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnReserveGuestNetworkServicePort(
    guestnetworkservice: *const ::std::ffi::c_void,
    protocol: HCN_PORT_PROTOCOL,
    access: HCN_PORT_ACCESS,
    port: u16,
) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnReserveGuestNetworkServicePort(
                guestnetworkservice: *const ::std::ffi::c_void,
                protocol: HCN_PORT_PROTOCOL,
                access: HCN_PORT_ACCESS,
                port: u16,
                portreservationhandle: *mut super::super::Foundation::HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        HcnReserveGuestNetworkServicePort(
            ::std::mem::transmute(guestnetworkservice),
            ::std::mem::transmute(protocol),
            ::std::mem::transmute(access),
            ::std::mem::transmute(port),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn HcnReserveGuestNetworkServicePortRange(
    guestnetworkservice: *const ::std::ffi::c_void,
    portcount: u16,
    portrangereservation: *mut HCN_PORT_RANGE_RESERVATION,
    portreservationhandle: *mut super::super::Foundation::HANDLE,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnReserveGuestNetworkServicePortRange(
                guestnetworkservice: *const ::std::ffi::c_void,
                portcount: u16,
                portrangereservation: *mut HCN_PORT_RANGE_RESERVATION,
                portreservationhandle: *mut super::super::Foundation::HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnReserveGuestNetworkServicePortRange(
            ::std::mem::transmute(guestnetworkservice),
            ::std::mem::transmute(portcount),
            ::std::mem::transmute(portrangereservation),
            ::std::mem::transmute(portreservationhandle),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn HcnUnregisterGuestNetworkServiceCallback(
    callbackhandle: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnUnregisterGuestNetworkServiceCallback(
                callbackhandle: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnUnregisterGuestNetworkServiceCallback(::std::mem::transmute(callbackhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn HcnUnregisterServiceCallback(
    callbackhandle: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "computenetwork")]
        extern "system" {
            fn HcnUnregisterServiceCallback(
                callbackhandle: *const ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        HcnUnregisterServiceCallback(::std::mem::transmute(callbackhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
